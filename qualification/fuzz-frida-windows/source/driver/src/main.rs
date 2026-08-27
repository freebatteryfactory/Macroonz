//! Qualification-only LibAFL loop over Frida EventSink block transitions.
//!
//! This driver stays outside the published Macroonz packages.
//! It translates target-relative Frida blocks into a fixed edge map, exposes that map through a safe owned observer, and runs one bounded deterministic mutational campaign over declared compiler-input seeds.

mod classify;
mod compose;
mod evidence;
mod preflight;
mod witnesses;

use core::{
    hash::{Hash, Hasher},
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Range},
};
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    error::Error,
    fs,
    io::{self, Write},
    path::PathBuf,
    rc::Rc,
    time::Duration,
};

use frida_gum::{
    Gum, Process,
    stalker::{Event, EventMask, EventSink, Stalker, Transformer},
};
use libafl::{
    corpus::{Corpus, InMemoryCorpus},
    events::SimpleEventManager,
    executors::{ExitKind, HasObservers, InProcessExecutor},
    feedback_or,
    feedbacks::{CrashFeedback, MaxMapFeedback, TimeoutFeedback},
    fuzzer::{Evaluator, ExecuteInputResult, Fuzzer, StdFuzzer},
    inputs::{BytesInput, HasTargetBytes},
    monitors::SimpleMonitor,
    mutators::{havoc_mutations, scheduled::HavocScheduledMutator},
    observers::{MapObserver, Observer},
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::{HasCorpus, HasSolutions, StdState},
};
use libafl_bolts::{
    Error as BoltsError, HasLen, Named,
    rands::StdRand,
    tuples::{Handle, Handled, MatchNameRef, tuple_list},
};
use macroonz_f0_target::{observe, CaptureOutcome};
use serde::{Deserialize, Serialize};

use macroonz_harness::fuzz::{
    PreflightCapability as HarnessPreflightCapability, PreflightFact,
    PreflightStatus as HarnessPreflightStatus, SelectedBackend, preflight_ready,
};

use crate::{
    classify::classify,
    evidence::HandoffCase,
    preflight::{Capability, PreflightRow, PreflightStatus},
};

const TARGET_MODULE: &str = "macroonz_f0_target.dll";
const EDGE_MAP_SIZE: usize = 16_384;
const EDGE_MAP_MASK: usize = EDGE_MAP_SIZE - 1;
const FIXED_SEED: u64 = 0x4d41_4352_4f4f_4e5a;
const BOUNDED_ITERS: u64 = 64;
const MUTATION_STAGE_ITERS: usize = 4;

const SEED_LAWFUL: &[u8] = b"struct Lawful;";
const SEED_REFUSED: &[u8] = b"struct Refused {";
const SEED_NON_UTF8: &[u8] = &[0xff];
/// Planted executor exit for solutions-corpus crash custody (not a TextCapture abort).
const SEED_CRASH_CUSTODY: &[u8] = b"macroonz-crash-custody";
/// Planted executor exit for solutions-corpus timeout custody (not a wall-clock hang).
const SEED_TIMEOUT_CUSTODY: &[u8] = b"macroonz-timeout-custody";

/// Ordered target-relative block starts for one execution.
#[derive(Debug, Default)]
struct BlockTrace {
    starts: Vec<usize>,
}

struct BlockRecorder {
    target: Range<usize>,
    base: usize,
    blocks: Rc<RefCell<BlockTrace>>,
    borrow_collision: Rc<Cell<bool>>,
}

impl EventSink for BlockRecorder {
    fn query_mask(&mut self) -> EventMask {
        EventMask::Block
    }

    fn start(&mut self) {}

    fn process(&mut self, event: &Event) {
        let Event::Block { start, .. } = event else {
            return;
        };
        let absolute = start.0.addr();
        if !self.target.contains(&absolute) {
            return;
        }
        let relative = absolute.wrapping_sub(self.base);
        match self.blocks.try_borrow_mut() {
            Ok(mut blocks) => blocks.starts.push(relative),
            Err(_) => self.borrow_collision.set(true),
        }
    }

    fn flush(&mut self) {}

    fn stop(&mut self) {}
}

/// Safe owned edge map filled from ordered Frida block transitions after each harness run.
#[derive(Debug, Serialize, Deserialize)]
struct FridaEdgeMapObserver {
    name: Cow<'static, str>,
    map: Vec<u8>,
    initial: u8,
    #[serde(skip, default = "default_blocks")]
    blocks: Rc<RefCell<BlockTrace>>,
    #[serde(skip, default = "default_collision")]
    borrow_collision: Rc<Cell<bool>>,
}

fn default_blocks() -> Rc<RefCell<BlockTrace>> {
    Rc::new(RefCell::new(BlockTrace::default()))
}

fn default_collision() -> Rc<Cell<bool>> {
    Rc::new(Cell::new(false))
}

impl FridaEdgeMapObserver {
    fn new(
        name: &'static str,
        blocks: Rc<RefCell<BlockTrace>>,
        borrow_collision: Rc<Cell<bool>>,
    ) -> Self {
        Self {
            name: Cow::Borrowed(name),
            map: vec![0_u8; EDGE_MAP_SIZE],
            initial: 0,
            blocks,
            borrow_collision,
        }
    }

    fn clear_trace(&self) -> Result<(), io::Error> {
        self.blocks
            .try_borrow_mut()
            .map_err(|_| io::Error::other("block trace was already borrowed"))?
            .starts
            .clear();
        self.borrow_collision.set(false);
        Ok(())
    }

    fn fill_edges_from_trace(&mut self) -> Result<(), BoltsError> {
        if self.borrow_collision.get() {
            return Err(BoltsError::illegal_state(
                "event delivery overlapped a block-trace borrow",
            ));
        }
        let starts = self
            .blocks
            .try_borrow()
            .map_err(|_| BoltsError::illegal_state("block trace remained mutably borrowed"))?
            .starts
            .clone();
        let mut previous = 0_usize;
        for current in starts {
            let edge = (previous.wrapping_shr(1) ^ current) & EDGE_MAP_MASK;
            match self.map.get_mut(edge) {
                Some(slot) => *slot = slot.saturating_add(1),
                None => {
                    return Err(BoltsError::illegal_state(
                        "edge index escaped the fixed map",
                    ));
                }
            }
            previous = current;
        }
        Ok(())
    }

    fn nonempty_edges(&self) -> usize {
        self.map.iter().filter(|value| **value != self.initial).count()
    }
}

impl Named for FridaEdgeMapObserver {
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}

impl HasLen for FridaEdgeMapObserver {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl Hash for FridaEdgeMapObserver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.map.hash(state);
        self.initial.hash(state);
    }
}

impl AsRef<Self> for FridaEdgeMapObserver {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for FridaEdgeMapObserver {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Deref for FridaEdgeMapObserver {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for FridaEdgeMapObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl MapObserver for FridaEdgeMapObserver {
    type Entry = u8;

    fn get(&self, idx: usize) -> Self::Entry {
        match self.map.get(idx) {
            Some(value) => *value,
            None => self.initial,
        }
    }

    fn set(&mut self, idx: usize, val: Self::Entry) {
        if let Some(slot) = self.map.get_mut(idx) {
            *slot = val;
        }
    }

    fn usable_count(&self) -> usize {
        self.map.len()
    }

    fn count_bytes(&self) -> u64 {
        u64::try_from(self.nonempty_edges()).unwrap_or(u64::MAX)
    }

    fn initial(&self) -> Self::Entry {
        self.initial
    }

    fn reset_map(&mut self) -> Result<(), BoltsError> {
        for slot in &mut self.map {
            *slot = self.initial;
        }
        Ok(())
    }

    fn to_vec(&self) -> Vec<Self::Entry> {
        self.map.clone()
    }

    fn how_many_set(&self, indexes: &[usize]) -> usize {
        indexes
            .iter()
            .filter(|idx| match self.map.get(**idx) {
                Some(value) => *value != self.initial,
                None => false,
            })
            .count()
    }
}

impl<I, S> Observer<I, S> for FridaEdgeMapObserver {
    fn pre_exec(&mut self, _state: &mut S, _input: &I) -> Result<(), BoltsError> {
        self.reset_map()?;
        self.clear_trace()
            .map_err(|error| BoltsError::illegal_state(error.to_string()))
    }

    fn post_exec(
        &mut self,
        _state: &mut S,
        _input: &I,
        _exit_kind: &ExitKind,
    ) -> Result<(), BoltsError> {
        self.fill_edges_from_trace()
    }
}

fn target_range(gum: &Gum) -> Result<(Range<usize>, usize), io::Error> {
    let process = Process::obtain(gum);
    process
        .enumerate_modules()
        .into_iter()
        .find(|module| module.name().eq_ignore_ascii_case(TARGET_MODULE))
        .map(|module| {
            let range: Range<usize> = module.range().into();
            let base = range.start;
            (range, base)
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "target module was not loaded"))
}

fn edge_fingerprint(observer: &FridaEdgeMapObserver) -> Vec<u8> {
    observer.map.clone()
}

fn prove_observation_seam(
    coverage_blocks: &Rc<RefCell<BlockTrace>>,
    borrow_collision: &Rc<Cell<bool>>,
) -> Result<(), Box<dyn Error>> {
    let mut probe = FridaEdgeMapObserver::new("probe", Rc::clone(coverage_blocks), Rc::clone(borrow_collision));
    let mut fingerprints = Vec::new();
    for seed in [SEED_LAWFUL, SEED_LAWFUL, SEED_REFUSED, SEED_NON_UTF8] {
        probe.pre_exec(&mut (), &BytesInput::new(seed.to_vec()))?;
        let _outcome = observe(seed);
        probe.post_exec(&mut (), &BytesInput::new(seed.to_vec()), &ExitKind::Ok)?;
        if probe.nonempty_edges() == 0 && seed == SEED_LAWFUL {
            return Err(io::Error::other("lawful input produced an empty edge map").into());
        }
        fingerprints.push(edge_fingerprint(&probe));
    }
    let Some([lawful, lawful_repeat, refused, non_utf8]) = <[_; 4]>::try_from(fingerprints).ok()
    else {
        return Err(io::Error::other("observation seam did not yield four fingerprints").into());
    };
    if lawful != lawful_repeat {
        return Err(io::Error::other("identical input produced unstable edge evidence").into());
    }
    if lawful == refused || lawful == non_utf8 || refused == non_utf8 {
        return Err(io::Error::other("distinct target paths did not produce distinct edge evidence").into());
    }
    Ok(())
}

fn harness_capability(capability: Capability) -> HarnessPreflightCapability {
    match capability {
        Capability::VsWhere => HarnessPreflightCapability::VsWhere,
        Capability::VcVarsAll => HarnessPreflightCapability::VcVarsAll,
        Capability::ComposedMsvcSdkEnv => HarnessPreflightCapability::ComposedMsvcSdkEnv,
        Capability::Rustc198 => HarnessPreflightCapability::RustcMsrv,
        Capability::RustHostTuple => HarnessPreflightCapability::RustcHostTuple,
        Capability::RustSysroot => HarnessPreflightCapability::RustcSysroot,
        Capability::RustTargetLibdir => HarnessPreflightCapability::RustcTargetLibdir,
        Capability::RustStdDll => HarnessPreflightCapability::RustStdDll,
        Capability::LlvmReported => HarnessPreflightCapability::LlvmReported,
        Capability::FridaGumLib => HarnessPreflightCapability::FridaGumLib,
        Capability::FridaGumHeader => HarnessPreflightCapability::FridaGumHeader,
        Capability::FridaDevkitHash => HarnessPreflightCapability::FridaDevkitHash,
    }
}

fn harness_facts(rows: &[PreflightRow]) -> Vec<PreflightFact> {
    rows.iter()
        .map(|row| {
            let status = match row.status {
                PreflightStatus::Available { .. } => HarnessPreflightStatus::Available,
                PreflightStatus::Unavailable { .. } => HarnessPreflightStatus::Unavailable,
            };
            PreflightFact::declared(harness_capability(row.capability), status)
        })
        .collect()
}

fn report_line(writer: &mut impl Write, line: &str) -> Result<(), io::Error> {
    writeln!(writer, "{line}")
}

fn main() -> Result<(), Box<dyn Error>> {
    // Durable tooling source lives under qualification/fuzz-frida-windows/source/driver.
    // Disposable work (devkit, build, evidence) lives under target/qualification/fuzz-frida-windows.
    let work = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("..")
        .join("target")
        .join("qualification")
        .join("fuzz-frida-windows");
    let workspace = &work;
    let devkit = work.join("devkit").join("frida-gum-17.9.5");
    let evidence_root = work.join("evidence");
    fs::create_dir_all(&evidence_root)?;

    let final_exam = evidence_root.join("final-exam");
    fs::create_dir_all(&final_exam)?;
    let (preflight_rows, composed_env) = preflight::probe_cold(&workspace, &devkit);
    {
        let mut preflight_out = fs::File::create(evidence_root.join("preflight.tsv"))?;
        writeln!(preflight_out, "phase\tcapability\tstatus\tfact")?;
        preflight::write_rows(&mut preflight_out, &preflight_rows)?;
        let mut cold_out = fs::File::create(final_exam.join("cold-shell-preflight.tsv"))?;
        writeln!(cold_out, "phase\tcapability\tstatus\tfact")?;
        preflight::write_rows(&mut cold_out, &preflight_rows)?;
        let mut env_out = fs::File::create(final_exam.join("composed-env.tsv"))?;
        preflight::write_composed_env(&mut env_out, &composed_env)?;
    }
    if !preflight::all_available(&preflight_rows) {
        let missing: Vec<&str> = preflight_rows
            .iter()
            .filter(|row| matches!(row.status, PreflightStatus::Unavailable { .. }))
            .map(|row| row.capability.as_str())
            .collect();
        return Err(io::Error::other(format!(
            "F0 cold-shell preflight unavailable: {}",
            missing.join(",")
        ))
        .into());
    }
    let facts = harness_facts(&preflight_rows);
    preflight_ready(SelectedBackend::LibAflFrida, &facts).map_err(|refusal| {
        io::Error::other(format!(
            "macroonz_harness::fuzz::preflight_ready refused: {refusal:?}"
        ))
    })?;
    {
        let mut ready_out = fs::File::create(final_exam.join("harness-preflight-ready.tsv"))?;
        writeln!(ready_out, "phase\tclaim\tstatus\tfact")?;
        writeln!(
            ready_out,
            "preflight\tmacroonz_harness::fuzz::preflight_ready\tavailable\tSelectedBackend::LibAflFrida; all required capabilities Available without duplicates"
        )?;
    }

    witnesses::prove_crash_timeout(&final_exam)?;
    witnesses::prove_resource_job(&final_exam)?;

    let _warm = observe(b"struct Warm;");
    let gum = Gum::obtain();
    if !Stalker::is_supported(&gum) {
        return Err(io::Error::other("Frida Stalker is unavailable on this host").into());
    }
    let (target, base) = target_range(&gum)?;
    let blocks = Rc::new(RefCell::new(BlockTrace::default()));
    let borrow_collision = Rc::new(Cell::new(false));
    let mut recorder = BlockRecorder {
        target,
        base,
        blocks: Rc::clone(&blocks),
        borrow_collision: Rc::clone(&borrow_collision),
    };
    let transformer = Transformer::from_callback(&gum, |basic_block, _output| {
        for instruction in basic_block {
            instruction.keep();
        }
    });
    let mut stalker = Stalker::new(&gum);
    stalker.set_trust_threshold(-1);
    stalker.follow_me(&transformer, Some(&mut recorder));

    prove_observation_seam(&blocks, &borrow_collision)?;

    let handoff_cases = [
        ("lawful", SEED_LAWFUL),
        ("refused", SEED_REFUSED),
        ("non-utf8", SEED_NON_UTF8),
    ]
    .into_iter()
    .map(|(name, bytes)| {
        let outcome = observe(bytes);
        let class = classify(outcome, ExitKind::Ok);
        HandoffCase {
            name,
            bytes: bytes.to_vec(),
            outcome,
            class,
        }
    })
    .collect::<Vec<_>>();
    evidence::write_handoff(&evidence_root.join("handoff"), &handoff_cases)?;

    let observer =
        FridaEdgeMapObserver::new("edges", Rc::clone(&blocks), Rc::clone(&borrow_collision));
    let observer_handle: Handle<FridaEdgeMapObserver> = observer.handle();
    let mut feedback = MaxMapFeedback::new(&observer);
    let mut objective = feedback_or!(CrashFeedback::new(), TimeoutFeedback::new());
    let mut state = StdState::new(
        StdRand::with_seed(FIXED_SEED),
        InMemoryCorpus::<BytesInput>::new(),
        InMemoryCorpus::new(),
        &mut feedback,
        &mut objective,
    )?;

    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    let monitor_events = Rc::new(Cell::new(0_usize));
    let monitor_counter = Rc::clone(&monitor_events);
    let mon = SimpleMonitor::new(move |_status| {
        monitor_counter.set(monitor_counter.get().saturating_add(1));
    });
    let mut mgr = SimpleEventManager::new(mon);
    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    let mut harness = |input: &BytesInput| {
        let bytes = input.target_bytes();
        let slice = bytes.as_ref();
        if slice == SEED_CRASH_CUSTODY {
            return ExitKind::Crash;
        }
        if slice == SEED_TIMEOUT_CUSTODY {
            return ExitKind::Timeout;
        }
        let _outcome = observe(slice);
        ExitKind::Ok
    };

    let mut executor = InProcessExecutor::builder()
        .timeout(Duration::from_secs(2))
        .crashdump(false)
        .harness(&mut harness)
        .observers(tuple_list!(observer))
        .fuzzer(&mut fuzzer)
        .state(&mut state)
        .event_mgr(&mut mgr)
        .build::<BytesInput, _>()?;

    let seed_count = 3_usize;
    for seed in [SEED_LAWFUL, SEED_REFUSED, SEED_NON_UTF8] {
        fuzzer.add_input(
            &mut state,
            &mut executor,
            &mut mgr,
            BytesInput::new(seed.to_vec()),
        )?;
    }

    let corpus_after_seeds = state.corpus().count();
    if corpus_after_seeds < seed_count {
        return Err(io::Error::other("seed loading did not retain the declared corpus").into());
    }

    let mutator = HavocScheduledMutator::new(havoc_mutations());
    let stage_iters = NonZeroUsize::new(MUTATION_STAGE_ITERS).ok_or_else(|| {
        io::Error::other("mutation-stage iteration bound must be nonzero")
    })?;
    let mut stages = tuple_list!(StdMutationalStage::with_max_iterations(mutator, stage_iters));
    fuzzer.fuzz_loop_for(&mut stages, &mut executor, &mut state, &mut mgr, BOUNDED_ITERS)?;

    let corpus_after_loop = state.corpus().count();
    let edge_hits = executor
        .observers()
        .get(&observer_handle)
        .ok_or_else(|| io::Error::other("edge observer missing after the bounded loop"))?
        .count_bytes();

    stalker.unfollow_me();
    stalker.flush();

    if corpus_after_loop <= corpus_after_seeds {
        return Err(io::Error::other(
            "bounded LibAFL loop did not grow the in-memory corpus from coverage feedback",
        )
        .into());
    }
    if edge_hits == 0 {
        return Err(io::Error::other("bounded LibAFL loop ended with an empty edge map").into());
    }

    // Retain planted crash/timeout exits through the live objective into the solutions corpus.
    let crash_input = BytesInput::new(SEED_CRASH_CUSTODY.to_vec());
    let (crash_result, _) = fuzzer.evaluate_input(
        &mut state,
        &mut executor,
        &mut mgr,
        &crash_input,
    )?;
    let timeout_input = BytesInput::new(SEED_TIMEOUT_CUSTODY.to_vec());
    let (timeout_result, _) = fuzzer.evaluate_input(
        &mut state,
        &mut executor,
        &mut mgr,
        &timeout_input,
    )?;
    if crash_result != ExecuteInputResult::Solution {
        return Err(io::Error::other(format!(
            "crash custody input was not retained as a solution; got {crash_result:?}"
        ))
        .into());
    }
    if timeout_result != ExecuteInputResult::Solution {
        return Err(io::Error::other(format!(
            "timeout custody input was not retained as a solution; got {timeout_result:?}"
        ))
        .into());
    }
    let solutions_count = state.solutions().count();
    if solutions_count < 2 {
        return Err(io::Error::other(format!(
            "solutions corpus retained fewer than two crash/timeout outcomes; count={solutions_count}"
        ))
        .into());
    }
    {
        let mut custody = fs::File::create(final_exam.join("solutions-custody.tsv"))?;
        writeln!(custody, "phase\tclaim\tstatus\tfact")?;
        writeln!(
            custody,
            "objective\tCrashFeedback|TimeoutFeedback\tavailable\treplaced ConstFeedback::False; live evaluate_input retains ExitKind::Crash and ExitKind::Timeout"
        )?;
        writeln!(
            custody,
            "solutions\tcount\tavailable\t{solutions_count}"
        )?;
        for id in state.solutions().ids() {
            let testcase = state.solutions().get(id)?;
            let borrowed = testcase.borrow();
            let Some(input) = borrowed.input() else {
                continue;
            };
            let owned = input.target_bytes().as_ref().to_vec();
            let kind = if owned.as_slice() == SEED_CRASH_CUSTODY {
                "Crash"
            } else if owned.as_slice() == SEED_TIMEOUT_CUSTODY {
                "Timeout"
            } else {
                "Other"
            };
            writeln!(
                custody,
                "solutions\tretained\tavailable\tkind={kind}; bytes={}",
                owned.len()
            )?;
            fs::write(
                final_exam.join(format!("solution-{kind}.bin")),
                &owned,
            )?;
        }
    }

    // Prefer a LibAFL corpus entry that itself reproduces typed refusal under TextCapture.
    // Prefer evolved (non-seed) refusals when present; never hand Macroonz substituted bytes.
    let mut interesting: Option<Vec<u8>> = None;
    for id in state.corpus().ids() {
        let testcase = state.corpus().get(id)?;
        let borrowed = testcase.borrow();
        if let Some(input) = borrowed.input() {
            let bytes = input.target_bytes().as_ref().to_vec();
            if matches!(observe(&bytes), CaptureOutcome::Refused { .. }) {
                if bytes.as_slice() != SEED_REFUSED {
                    interesting = Some(bytes);
                    break;
                }
                if interesting.is_none() {
                    interesting = Some(bytes);
                }
            }
        }
    }
    let interesting = interesting.ok_or_else(|| {
        io::Error::other(
            "no LibAFL corpus entry reproduced typed refusal for Macroonz handoff; refusing rather than substituting bytes",
        )
    })?;
    compose::prove_libafl_to_macroonz(&final_exam, &interesting)?;

    report_line(&mut output, "phase\tmetric\tvalue")?;
    preflight::write_rows(&mut output, &preflight_rows)?;
    for case in &handoff_cases {
        report_line(
            &mut output,
            &format!(
                "classification\t{}\t{}\t{}",
                case.name,
                case.class.as_str(),
                classify::outcome_label(case.outcome)
            ),
        )?;
    }
    report_line(
        &mut output,
        "classification\tcrash-timeout-resource\tin-process-textcapture\tlive CrashFeedback|TimeoutFeedback objective retained planted ExitKind::Crash and ExitKind::Timeout in solutions corpus; TextCapture seeds remain non-aborting; executor-timeout-bound=2s; Job Object resource witness separate",
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tsolutions-retained\t{solutions_count}"),
    )?;
    report_line(
        &mut output,
        "handoff\treduction-replay\tproved\tLibAFL interesting bytes entered Macroonz reduce+capture_replay under Macroonz typed-refusal fingerprint; see evidence/final-exam/libafl-macroonz-compose.tsv",
    )?;
    evidence::write_cross_host_disposition(&mut output)?;
    evidence::write_cost_ceiling(&mut output)?;
    report_line(
        &mut output,
        &format!("libafl-loop\tfixed-seed\t{FIXED_SEED:#x}"),
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tbounded-iters\t{BOUNDED_ITERS}"),
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tcorpus-after-seeds\t{corpus_after_seeds}"),
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tcorpus-after-loop\t{corpus_after_loop}"),
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tnonempty-edge-entries\t{edge_hits}"),
    )?;
    report_line(
        &mut output,
        "libafl-loop\tstatus\tfeedback-driven-corpus-growth",
    )?;
    report_line(
        &mut output,
        &format!("libafl-loop\tmonitor-events\t{}", monitor_events.get()),
    )?;
    report_line(
        &mut output,
        "selection\tfrida\taccepted\tnarrowed final exam seats passed; residual LNK4098 named as CRT coexistence ceiling; Linux/macOS credible-unexecuted until Wave F",
    )?;

    let mut machine = fs::File::create(evidence_root.join("machine-receipt.tsv"))?;
    // Re-emit the same machine lines into durable evidence.
    report_line(&mut machine, "phase\tmetric\tvalue")?;
    preflight::write_rows(&mut machine, &preflight_rows)?;
    for case in &handoff_cases {
        report_line(
            &mut machine,
            &format!(
                "classification\t{}\t{}\t{}",
                case.name,
                case.class.as_str(),
                classify::outcome_label(case.outcome)
            ),
        )?;
    }
    evidence::write_cross_host_disposition(&mut machine)?;
    evidence::write_cost_ceiling(&mut machine)?;
    report_line(
        &mut machine,
        &format!("libafl-loop\tcorpus-after-loop\t{corpus_after_loop}"),
    )?;
    report_line(
        &mut machine,
        &format!("libafl-loop\tnonempty-edge-entries\t{edge_hits}"),
    )?;
    report_line(
        &mut machine,
        "libafl-loop\tstatus\tfeedback-driven-corpus-growth",
    )?;
    report_line(
        &mut machine,
        "selection\tfrida\taccepted\tnarrowed final exam seats passed with named CRT coexistence ceiling",
    )?;

    output.flush()?;
    Ok(())
}

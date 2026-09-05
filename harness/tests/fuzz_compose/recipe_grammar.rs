//! The compiler's actual grammar crosses bounded coverage feedback, retained corpus, and a separately labeled reduction control.

use super::recipe_compilation::{Instrumentation, compile};
use super::recipe_control::reduce_control;
use super::recipe_deadline::{expired, within};
use super::recipe_observation::{INPUT_LIMIT, Outcome, observe};
use super::support::{
    FuzzRoadFailure, admit_byte_sequences, coverage_budgets, external, ready_for_compiled_root,
    rustc_profile_request, stop_as,
};
use macroonz_harness::corpus::{SeedInput, pack, warm_start};
use macroonz_harness::descriptor::{NamespacedName, PopulationRef, RevisionBinding};
use macroonz_harness::fuzz::{
    CoverageAdmission, CoverageCampaign, CoverageCorpus, CoveragePoint, CoverageProfile,
    FuzzExecution, MutationPlan, ReadyPreflight, neighboring_inputs, observe_rustc_profile,
};
use macroonz_harness::generate::{
    ByteSource, CaseWidth, GenerationPlan, InputOrigin, RejectionAllowance, SizeProgression, drive,
};
use macroonz_harness::report::{ByteBudget, CaseBudget, GenerationProfile};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const SEARCH_CASES: u32 = 96;
const PROCESS_SECONDS: u64 = 2;
const SEARCH_SECONDS: u64 = 1_200;
const SEEDS: [&str; 3] = [
    "pub mod simple { pub struct Marker; bake! { projections { companions; }; } }",
    "pub mod door { pub enum Position { Shut, Open } pub enum Command { Turn } bake! { vocabularies { Position; Command; }; transitions(Position, Command) { (Shut, Turn) => Open with(crate::turn); }; absence(refused); projections { companions; dispatch(apply); }; } }",
    "pub mod policy { pub enum Stage { Draft, Published } pub enum Capability { Read, Write } bake! { vocabularies { Stage; Capability; }; relations { access(Stage, Capability) { (Draft, Read) with(crate::policy::allow); (Published, Write) with(crate::policy::allow); }; }; postures { access { repetition(refused); }; }; projections { companions; relation_tables { access { pub fn lookup(stage: &Stage, capability: &Capability) -> Option<fn() -> bool>; }; }; }; } }",
];

fn supervise(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    within(Duration::from_secs(PROCESS_SECONDS), |signal| {
        supervise_until(child, signal)
    })?
}

fn supervise_until(
    child: &mut std::process::Child,
    signal: &Receiver<()>,
) -> Result<FuzzExecution, String> {
    loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            return Ok(if status.success() {
                FuzzExecution::Success
            } else {
                FuzzExecution::NonzeroExit(status.code())
            });
        }
        if expired(signal) {
            return stop_as(child, FuzzExecution::Timeout);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn generated(input: &[u8], population: PopulationRef) -> Result<Vec<u8>, FuzzRoadFailure> {
    let width = CaseWidth::declared(input.len()).map_err(external)?;
    let bytes = u64::try_from(input.len()).map_err(external)?;
    let plan = GenerationPlan::declared(
        population,
        GenerationProfile::declared("recipe-grammar", 1),
        InputOrigin::Supplied(input.to_vec()),
        CaseBudget::declared(1),
        ByteBudget::declared(bytes),
        RejectionAllowance::NoRejections,
        SizeProgression::Constant { width },
    )
    .map_err(external)?;
    let source = ByteSource::of_plan(&plan);
    let produced = drive::<u8>(
        &plan,
        &source,
        macroonz_harness::generate::decode_arbitrary::<u8>,
        admit_byte_sequences,
    );
    let [candidate] = produced.sequences() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(candidate.input(), input);
    Ok(candidate.input().to_vec())
}

#[test]
fn declared_grammar_seeds_cross_generation_without_becoming_fixture_counts()
-> Result<(), FuzzRoadFailure> {
    let population = PopulationRef::named("harness", "recipe-grammar").map_err(external)?;
    for seed in SEEDS {
        let input = generated(seed.as_bytes(), population)?;
        assert_eq!(input, seed.as_bytes());
        let observed = observe(&input).map_err(external)?;
        let Outcome::Baked(bytes) = &observed else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert!(!bytes.is_empty());
        assert_eq!(observe(&input).map_err(external)?, observed);
    }
    assert!(matches!(observe(b"not a recipe"), Ok(Outcome::Refused(_))));
    assert!(matches!(observe(&[0xff]), Ok(Outcome::Refused(_))));
    Ok(())
}

fn exercise(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    input: &[u8],
) -> Result<CoverageAdmission, FuzzRoadFailure> {
    let observed = observe_rustc_profile(ready, corpus, input, supervise)?;
    assert_eq!(
        observed.execution(),
        FuzzExecution::Success,
        "recipe input failed: {input:?}"
    );
    corpus
        .admit(observed)
        .map_err(FuzzRoadFailure::CoverageAdmission)
}

fn neighbors(input: &[u8]) -> Result<Vec<Vec<u8>>, FuzzRoadFailure> {
    let plan = MutationPlan::declared(
        512,
        32,
        vec![b"bake!".to_vec(), b";".to_vec(), b"=>".to_vec()],
    )
    .map_err(external)?;
    let mut neighbors = Vec::new();
    let middle = input.len().checked_div(2).ok_or(FuzzRoadFailure::Fixture)?;
    for offset in [0, middle, input.len().saturating_sub(16)] {
        let (prefix, remaining) = input.split_at(offset);
        let (window, suffix) = remaining.split_at(remaining.len().min(16));
        let mut seen_kinds = Vec::new();
        for candidate in neighboring_inputs(window, Some(b"{}"), &plan).map_err(external)? {
            if seen_kinds.contains(&candidate.kind()) {
                continue;
            }
            seen_kinds.push(candidate.kind());
            let candidate = [prefix, candidate.bytes(), suffix].concat();
            if candidate.len() <= INPUT_LIMIT {
                neighbors.push(candidate);
            }
        }
    }
    Ok(neighbors)
}

#[test]
#[ignore = "bounded compiler instrumentation and coverage-guided grammar campaign"]
fn the_recipe_grammar_uses_coverage_feedback_corpus_reduction_and_replay()
-> Result<(), FuzzRoadFailure> {
    refuse_wrapper_only_instrumentation()?;
    let (rustc, subject, run, revision) = compile(Instrumentation::CompilerAndWrapper)?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(FuzzRoadFailure::Fixture)?
        .to_path_buf();
    let population = PopulationRef::named("harness", "recipe-grammar").map_err(external)?;
    let campaign = campaign(population, revision)?;
    let ready = ready_for_compiled_root(rustc, subject, &repository, run.join("cases"), campaign)?;
    writeln!(
        std::io::stdout().lock(),
        "Grammar campaign: {:?}; host {}; rustc {}; LLVM {}; deadline {}s; per-process {}s",
        campaign,
        ready.host(),
        ready.release(),
        ready.llvm_version(),
        SEARCH_SECONDS,
        PROCESS_SECONDS
    )
    .map_err(external)?;
    let mut corpus = CoverageCorpus::opening(&ready);
    let (searched, retained) = within(
        Duration::from_secs(SEARCH_SECONDS),
        |signal| -> Result<_, FuzzRoadFailure> {
            let mut seen = warm_seeds(&ready, &mut corpus, population)?;
            search(&ready, &mut corpus, &mut seen, population, signal)?;
            let searched = corpus.attempted_cases();
            assert!(searched > 3 && searched <= SEARCH_CASES);
            assert!(
                corpus.observed().iter().any(compiler_point),
                "only the calling wrapper was instrumented"
            );
            let retained = replay(&ready, &mut corpus, population, signal)?;
            Ok((searched, retained))
        },
    )
    .map_err(external)??;
    let lawful = corpus
        .interesting()
        .iter()
        .find(|input| matches!(observe(input.as_bytes()), Ok(Outcome::Baked(_))))
        .ok_or(FuzzRoadFailure::Fixture)?;
    reduce_control(lawful, ready.standing().target().clone(), revision)?;
    writeln!(
        std::io::stdout().lock(),
        "Grammar search: {searched} attempts; {retained} retained; {} total attempts including replay; {} input bytes; {} line points; no subject failure found",
        corpus.attempted_cases(),
        corpus.attempted_input_bytes(),
        corpus.observed().len(),
    ).map_err(external)?;
    run.removed()?;
    Ok(())
}

fn campaign(
    population: PopulationRef,
    revision: RevisionBinding,
) -> Result<CoverageCampaign, FuzzRoadFailure> {
    let input_bound = u64::try_from(INPUT_LIMIT).map_err(external)?;
    let retained_bytes = u64::from(SEARCH_CASES)
        .checked_mul(input_bound)
        .ok_or(FuzzRoadFailure::Fixture)?;
    let total_bytes = retained_bytes
        .checked_mul(2)
        .ok_or(FuzzRoadFailure::Fixture)?;
    let budget = coverage_budgets(
        SEARCH_CASES * 2,
        total_bytes,
        33_554_432,
        1_000_000,
        SEARCH_CASES,
        retained_bytes,
    )?;
    Ok(CoverageCampaign::declared(
        population,
        revision,
        CoverageProfile::declared(
            NamespacedName::named("harness", "recipe-line-coverage").map_err(external)?,
            1,
        ),
        budget,
    ))
}

fn compiler_point(point: &CoveragePoint) -> bool {
    match point {
        CoveragePoint::Line { source, .. } => {
            source.relative().starts_with("macros/compiler/src/recipe/")
        }
        CoveragePoint::Branch { .. } => false,
    }
}

fn refuse_wrapper_only_instrumentation() -> Result<(), FuzzRoadFailure> {
    let (rustc, subject, run, revision) = compile(Instrumentation::WrapperOnly)?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(FuzzRoadFailure::Fixture)?
        .to_path_buf();
    let population =
        PopulationRef::named("harness", "recipe-instrument-control").map_err(external)?;
    let ready = ready_for_compiled_root(
        rustc,
        subject,
        &repository,
        run.join("cases"),
        campaign(population, revision)?,
    )?;
    let mut corpus = CoverageCorpus::opening(&ready);
    let seed = SEEDS.first().ok_or(FuzzRoadFailure::Fixture)?.as_bytes();
    let observed = observe_rustc_profile(&ready, &mut corpus, seed, supervise)?;
    assert_eq!(observed.execution(), FuzzExecution::Success);
    assert!(!observed.observation().points().is_empty());
    assert!(
        !observed.observation().points().iter().any(compiler_point),
        "negative instrumentation control unexpectedly mapped the compiler"
    );
    writeln!(std::io::stdout().lock(), "Wrapper-only negative control: executable succeeds, but no compiler grammar point qualifies").map_err(external)?;
    run.removed()?;
    Ok(())
}

fn replay(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    population: PopulationRef,
    signal: &Receiver<()>,
) -> Result<usize, FuzzRoadFailure> {
    let retained = pack(
        population,
        corpus
            .interesting()
            .iter()
            .map(|input| SeedInput::declared(input.as_bytes().to_vec()).map_err(external))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map_err(external)?;
    let before = corpus.observed().clone();
    for input in warm_start(&retained) {
        let InputOrigin::Supplied(input) = input else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert!(!expired(signal), "grammar replay deadline reached");
        assert_eq!(exercise(ready, corpus, &input)?, CoverageAdmission::Known);
    }
    assert_eq!(corpus.observed(), &before);
    Ok(retained.seeds().len())
}

fn warm_seeds(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    population: PopulationRef,
) -> Result<BTreeSet<Vec<u8>>, FuzzRoadFailure> {
    let seeds = pack(
        population,
        SEEDS
            .iter()
            .map(|seed| SeedInput::declared(seed.as_bytes().to_vec()).map_err(external))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map_err(external)?;
    let mut seen = BTreeSet::new();
    for input in warm_start(&seeds) {
        let InputOrigin::Supplied(input) = input else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert!(matches!(observe(&input), Ok(Outcome::Baked(_))));
        let input = generated(&input, population)?;
        seen.insert(input.clone());
        assert!(matches!(
            exercise(ready, corpus, &input)?,
            CoverageAdmission::Interesting(_)
        ));
    }
    Ok(seen)
}

fn search(
    ready: &ReadyPreflight,
    corpus: &mut CoverageCorpus,
    seen: &mut BTreeSet<Vec<u8>>,
    population: PopulationRef,
    signal: &Receiver<()>,
) -> Result<(), FuzzRoadFailure> {
    for _round in 0_u8..2 {
        let frontier = corpus
            .interesting()
            .iter()
            .map(|seed| neighbors(seed.as_bytes()))
            .collect::<Result<Vec<_>, _>>()?;
        for input in frontier.into_iter().flatten() {
            if corpus.attempted_cases() == SEARCH_CASES {
                return Ok(());
            }
            assert!(
                !expired(signal),
                "grammar campaign deadline reached before completing search"
            );
            if !seen.insert(input.clone()) {
                continue;
            }
            let input = generated(&input, population)?;
            exercise(ready, corpus, &input)?;
        }
    }
    Ok(())
}

#[test]
fn the_recipe_supervisor_reaps_a_deliberately_parked_subject() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("recipe-deadline")?;
    let mut corpus = CoverageCorpus::opening(&ready);
    let result = observe_rustc_profile(&ready, &mut corpus, &[0xfe], supervise)?;
    assert_eq!(result.execution(), FuzzExecution::Timeout);
    assert!(corpus.admit(result).is_err());
    assert!(corpus.interesting().is_empty());
    run.removed()?;
    Ok(())
}

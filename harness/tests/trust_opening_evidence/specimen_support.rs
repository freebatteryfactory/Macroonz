//! Shared outside-test construction for exact compiled specimen claims and retained composition claims.

use super::{CompiledRosterMeaning, MutationRoadFailure, ORIGINAL_OPERATION};
use macroonz_harness::muterprater::{
    CompiledSpecimenHostRefusal, CompiledSpecimenObservation, CompiledSpecimenRequest,
    CompiledSpecimenRole, EvaluationDirective, SpecimenMaterializerRefusal,
};
use macroonz_harness::report::ForeignText;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);
pub(crate) static SPECIMEN_MATERIALIZER_CALLS: AtomicU32 = AtomicU32::new(0);
pub(crate) static SPECIMEN_HOST_CALLS: AtomicU32 = AtomicU32::new(0);
static SPECIMEN_TEST_LOCK: Mutex<()> = Mutex::new(());
static CACHED_SIBLING_OBSERVATION: Mutex<
    Option<CompiledSpecimenObservation<CompiledRosterMeaning>>,
> = Mutex::new(None);

pub(crate) fn lock_specimen_tests()
-> Result<std::sync::MutexGuard<'static, ()>, MutationRoadFailure> {
    SPECIMEN_TEST_LOCK
        .lock()
        .map_err(|_| MutationRoadFailure::NativeToolchain)
}

pub(crate) fn clear_cached_sibling_observation() -> Result<(), MutationRoadFailure> {
    *CACHED_SIBLING_OBSERVATION
        .lock()
        .map_err(|_| MutationRoadFailure::NativeToolchain)? = None;
    Ok(())
}

/// The materializer-callable shape, on the same terms as the public contract.
type MaterializerFn = fn(EvaluationDirective<'_>) -> Result<Vec<u8>, SpecimenMaterializerRefusal>;

/// The compiled-specimen host shape takes custody of each private-minted request.
type SpecimenHostFn =
    fn(
        CompiledSpecimenRequest<'_, '_, [u32; 3]>,
    )
        -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal>;

pub(crate) fn specimen_source(operation: &[u8]) -> Vec<u8> {
    let mut source = b"fn main() { let input: u32 = std::env::args().nth(1).expect(\"input\").parse().expect(\"u32\"); let a = 1u32; let b = 0u32; if ".to_vec();
    source.extend_from_slice(operation);
    source.extend_from_slice(b" { print!(\"1\"); } else { print!(\"0\"); } }\n");
    source
}

/// This admitted materializer implements both directive postures.
pub(crate) const SPECIMEN_MATERIALIZER: MaterializerFn = |directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    let payload = directive.resolved().map_or(ORIGINAL_OPERATION, |resolved| {
        resolved.alternative().operation()
    });
    Ok(specimen_source(payload))
};

pub(crate) fn omitted_specimen_branch(
    directive: EvaluationDirective<'_>,
) -> Result<Vec<u8>, SpecimenMaterializerRefusal> {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    match directive.resolved() {
        Some(resolved) => Err(SpecimenMaterializerRefusal::ActiveSelectionNotImplemented(
            resolved.selection(),
        )),
        None => Ok(specimen_source(ORIGINAL_OPERATION)),
    }
}

pub(crate) fn omitted_baseline_branch(
    directive: EvaluationDirective<'_>,
) -> Result<Vec<u8>, SpecimenMaterializerRefusal> {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    match directive.resolved() {
        None => Err(SpecimenMaterializerRefusal::NoMutationNotImplemented),
        Some(resolved) => Ok(specimen_source(resolved.alternative().operation())),
    }
}

/// This hostile materializer returns wrong but syntactically valid selected bytes.
pub(crate) const WRONG_SELECTED_SPECIMEN: MaterializerFn = |directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    Ok(match directive.resolved() {
        None => specimen_source(ORIGINAL_OPERATION),
        Some(_) => specimen_source(b"input > 0"),
    })
};

/// This hostile materializer returns byte-identical baseline and selected source.
pub(crate) const UNCHANGED_SPECIMEN_MATERIALIZER: MaterializerFn = |_directive| {
    SPECIMEN_MATERIALIZER_CALLS.fetch_add(1, Ordering::SeqCst);
    Ok(specimen_source(ORIGINAL_OPERATION))
};

fn specimen_path(extension: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "macroonz_harness_specimen_{}_{ordinal}{extension}",
        std::process::id()
    ))
}

fn host_failure(error: &[u8]) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Execution(ForeignText::admitted(error))
}

fn compilation_failure(error: &[u8]) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Compilation(ForeignText::admitted(error))
}

/// The real host consumes each request so one call cannot reuse request custody.
pub(crate) const COMPILED_SPECIMEN_HOST: SpecimenHostFn = |request| specimen_hosted(&request);

/// Compile one specimen through the pinned toolchain, execute it, and read the meaning off its output.
fn specimen_hosted(
    request: &CompiledSpecimenRequest<'_, '_, [u32; 3]>,
) -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal> {
    SPECIMEN_HOST_CALLS.fetch_add(1, Ordering::SeqCst);
    let source = specimen_path(".rs");
    let executable = specimen_path(std::env::consts::EXE_SUFFIX);
    std::fs::write(&source, request.content().bytes())
        .map_err(|error| compilation_failure(error.to_string().as_bytes()))?;
    let target = request.execution().target();
    let compiled = Command::new("rustup")
        .arg("run")
        .arg(target.toolchain().spelling())
        .arg("rustc")
        .arg(&source)
        .arg("--edition=2024")
        .arg("--target")
        .arg(target.target().spelling())
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| compilation_failure(error.to_string().as_bytes()))?;
    drop(std::fs::remove_file(&source));
    if !compiled.status.success() {
        return Err(compilation_failure(&compiled.stderr));
    }
    let executed = Command::new(&executable)
        .arg(request.input()[0].to_string())
        .output()
        .map_err(|error| host_failure(error.to_string().as_bytes()))?;
    drop(std::fs::remove_file(&executable));
    if !executed.status.success() {
        return Err(host_failure(&executed.stderr));
    }
    if !request
        .content()
        .bytes()
        .windows(request.operation().len())
        .any(|window| window == request.operation())
    {
        return Err(CompiledSpecimenHostRefusal::Meaning(ForeignText::admitted(
            request.operation(),
        )));
    }
    let meaning = match executed.stdout.as_slice() {
        b"1" => CompiledRosterMeaning::Stated(1),
        b"0" => CompiledRosterMeaning::Unstated,
        other => {
            return Err(CompiledSpecimenHostRefusal::Meaning(ForeignText::admitted(
                other,
            )));
        }
    };
    Ok(CompiledSpecimenObservation::executed(request, meaning))
}

/// This hostile host answers the selected request with a cached baseline observation.
pub(crate) const CACHED_SIBLING_OBSERVATION_HOST: SpecimenHostFn =
    |request| sibling_cached(&request);

/// The baseline call plants an observation, and the selected call answers with it.
fn sibling_cached(
    request: &CompiledSpecimenRequest<'_, '_, [u32; 3]>,
) -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal> {
    SPECIMEN_HOST_CALLS.fetch_add(1, Ordering::SeqCst);
    let mut cached = CACHED_SIBLING_OBSERVATION
        .lock()
        .map_err(|error| host_failure(error.to_string().as_bytes()))?;
    match request.role() {
        CompiledSpecimenRole::Baseline => {
            *cached = Some(CompiledSpecimenObservation::executed(
                request,
                CompiledRosterMeaning::Unstated,
            ));
            Ok(CompiledSpecimenObservation::executed(
                request,
                CompiledRosterMeaning::Stated(1),
            ))
        }
        CompiledSpecimenRole::Selected(_) => cached
            .take()
            .ok_or_else(|| host_failure(b"cached sibling observation absent")),
    }
}

use super::{
    CompiledPublication, PendingStaging, RefusedStaging, StagedPublication, StagingError,
    StagingObservationError, StagingRun,
};
use crate::compiler::Kind;
use crate::native_compiler::{CompilerOutput, CompilerRequest, CompilerRun};

pub(super) fn run<K: Kind>(
    staged: StagedPublication<K>,
    request: &CompilerRequest,
) -> Result<StagingRun<K>, StagingError> {
    let source = staged.path.join(request.locus().spelling());
    if request.process().directory() != staged.path
        || !staged
            .plan
            .files()
            .any(|(path, _)| staged.path.join(path.spelling()) == source)
        || request.manifest().is_some_and(|manifest| {
            !staged
                .plan
                .files()
                .any(|(path, _)| staged.path.join(path.spelling()) == manifest)
        })
        || request
            .dependency_file()
            .is_none_or(|path| path.starts_with(&staged.path))
        || request
            .output_root()
            .is_none_or(|path| path.starts_with(&staged.path))
    {
        return Err(StagingError::Configuration("compilation must select staged inputs, dependency capture and outputs outside the source tree".to_owned()));
    }
    staged.compared()?;
    let run = crate::native_compiler::compile(request).map_err(StagingError::Compiler)?;
    Ok(finish(staged, run))
}

pub(super) fn finish<K: Kind>(staged: StagedPublication<K>, run: CompilerRun) -> StagingRun<K> {
    match run {
        CompilerRun::Pending(compiler) => {
            StagingRun::Pending(Box::new(PendingStaging { staged, compiler }))
        }
        CompilerRun::Finished(compiler) => match observed(&staged, &compiler) {
            Ok(()) => StagingRun::Compiled(Box::new(CompiledPublication {
                staged,
                compiler: *compiler,
            })),
            Err(reason) => StagingRun::Refused(Box::new(RefusedStaging {
                staged,
                compiler: *compiler,
                reason,
            })),
        },
    }
}

fn observed<K: Kind>(
    staged: &StagedPublication<K>,
    output: &CompilerOutput,
) -> Result<(), StagingObservationError> {
    if !output
        .observed()
        .is_ok_and(|observation| observation.refusal().is_none())
    {
        return Err(StagingObservationError::Compilation);
    }
    staged
        .compared()
        .map_err(|error| StagingObservationError::Source(error.to_string()))?;
    let info = output
        .dependencies()
        .ok_or_else(|| {
            StagingObservationError::Dependencies("dependency capture absent".to_owned())
        })?
        .map_err(|error| StagingObservationError::Dependencies(error.to_string()))?;
    if output.request().manifest().is_some()
        && output.request().dependency_file() != Some(info.artifact().with_extension("d").as_path())
    {
        return Err(StagingObservationError::Dependencies(
            "Cargo dependency file does not sit beside the reported artifact".to_owned(),
        ));
    }
    for file in staged.plan.prepared.files() {
        let expected = staged.path.join(file.path().spelling());
        if !info.files().contains(&expected) {
            return Err(StagingObservationError::Unused(
                file.path().spelling().to_owned(),
            ));
        }
    }
    Ok(())
}

use super::MutationObservationError;
use super::types::{ExecutionContext, Observation};
use crate::harness::muterprater::{
    AdapterQualification, AnnouncedRoster, ArtifactManifestRefusal, BackendVersionPosture,
    GrammarStanding, MutationSite, MutationSourceRevision, wrap,
};
use crate::native_process::{CaptureEnd, ProcessOutput, ProcessStop};
use std::collections::BTreeSet;

pub(super) fn observed(
    context: &ExecutionContext,
    process: &ProcessOutput,
) -> Result<Observation, MutationObservationError> {
    let console = console(process)?;
    let current = super::source::capture(
        context.request.process.directory(),
        &context.request.sources,
    )
    .map_err(|error| MutationObservationError::Sources(error.to_string()))?;
    for (before, after) in context.sources.iter().zip(&current) {
        if before.file != after.file || before.bytes != after.bytes {
            return Err(MutationObservationError::Sources(format!(
                "source moved: {}",
                before.file
            )));
        }
    }
    let reading = wrap::read_output(
        &console,
        BackendVersionPosture::Stated(context.invocation.version().clone()),
        context.owner,
        context.family,
    )
    .map_err(|error| MutationObservationError::Artifact(ArtifactManifestRefusal::Reading(error)))?;
    let AnnouncedRoster::Stated(count) = reading.announced() else {
        return Err(MutationObservationError::Roster);
    };
    if usize::try_from(count).ok() != Some(reading.run().reports().len()) {
        return Err(MutationObservationError::Roster);
    }
    let files: BTreeSet<_> = reading
        .run()
        .reports()
        .iter()
        .filter_map(|report| match report.target().site() {
            MutationSite::Reported(coordinate) => Some(coordinate.file()),
            MutationSite::Declared(_) => None,
        })
        .collect();
    let mut sources = Vec::with_capacity(files.len());
    for file in files {
        let original = context
            .sources
            .iter()
            .find(|source| source.file == file)
            .ok_or_else(|| {
                MutationObservationError::Artifact(ArtifactManifestRefusal::ReportedSourceMissing(
                    file.to_owned(),
                ))
            })?;
        sources.push(
            MutationSourceRevision::from_content(file, &original.bytes)
                .map_err(|error| MutationObservationError::Sources(format!("{error:?}")))?,
        );
    }
    let manifest = wrap::read_artifact(
        &console,
        context.invocation.clone(),
        sources,
        context.owner,
        context.family,
    )
    .map_err(MutationObservationError::Artifact)?;
    let qualification = AdapterQualification::of(
        manifest.reading(),
        GrammarStanding::Checked(context.invocation.version().clone()),
    )
    .map_err(MutationObservationError::Qualification)?;
    Ok(Observation {
        manifest,
        qualification,
        console,
    })
}

fn console(process: &ProcessOutput) -> Result<String, MutationObservationError> {
    if process.stop() != &ProcessStop::Exited
        || !matches!(process.status().code(), Some(0_i32 | 2_i32 | 3_i32))
        || process.stdout().end() != &CaptureEnd::Eof
        || process.stderr().end() != &CaptureEnd::Eof
    {
        return Err(MutationObservationError::Process);
    }
    let stdout = process.stdout().bytes();
    let stderr = process.stderr().bytes();
    if !stdout.is_empty() && !stderr.is_empty() && !stdout.ends_with(b"\n") {
        return Err(MutationObservationError::Console(
            "stdout does not end at a console line boundary".to_owned(),
        ));
    }
    let mut joined = stdout.to_vec();
    joined.extend_from_slice(stderr);
    String::from_utf8(joined).map_err(|error| MutationObservationError::Console(error.to_string()))
}

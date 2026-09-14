use super::types::ExecutionContext;
use super::{MutationError, MutationPhase, MutationRequest};
use crate::harness::fuzz::RUSTC_COVERAGE_TOOLCHAIN;
use crate::harness::muterprater::{
    BackendCommand, BackendVersion, FamilyLookup, MutationBackendInvocation, OwnerLookup,
    WrappedBackend,
};
use crate::harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use crate::native_process::{
    self, CaptureEnd, ProcessOutput, ProcessRequest, ProcessRun, ProcessStop,
};

pub(super) fn source_filter(file: &str) -> Result<String, MutationError> {
    use std::fmt::Write;
    let mut pattern = "^".to_owned();
    for character in file.chars() {
        write!(pattern, "\\x{{{:x}}}", u32::from(character))
            .map_err(|error| MutationError::Configuration(error.to_string()))?;
    }
    pattern.push_str(":[0-9]+:[0-9]+: ");
    Ok(pattern)
}

pub(super) fn context(
    request: MutationRequest,
    owner: OwnerLookup,
    family: FamilyLookup,
) -> Result<ExecutionContext, MutationError> {
    let query = ProcessRequest::informed(
        request.process.executable().to_path_buf(),
        request.process.directory().to_path_buf(),
        vec!["mutants".to_owned(), "--version".to_owned()],
        request.process.environment().to_vec(),
        request.version_limits(),
        &[],
    )
    .map_err(MutationError::Process)?;
    let backend_version = version(&query, MutationPhase::BackendVersion)?;
    let compiler_version = version(&request.compiler, MutationPhase::CompilerVersion)?;
    let compiler = std::str::from_utf8(compiler_version.stdout().bytes())
        .map_err(|error| MutationError::Configuration(error.to_string()))?;
    let arguments: Vec<_> = request
        .process
        .arguments()
        .iter()
        .map(String::as_str)
        .collect();
    let executable =
        request.process.executable().to_str().ok_or_else(|| {
            MutationError::Configuration("non-Unicode backend executable".to_owned())
        })?;
    let command = BackendCommand::declared(executable, &arguments)
        .map_err(|error| MutationError::Configuration(format!("{error:?}")))?;
    let invocation = MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        BackendVersion::stated("27.0.0")
            .map_err(|error| MutationError::Configuration(format!("{error:?}")))?,
        command,
        TargetBinding::bound(
            TargetTriple::declared(&request.target),
            ToolchainIdentity::declared(compiler.trim()),
        ),
    );
    let sources = super::source::capture(request.process.directory(), &request.sources)?;
    Ok(ExecutionContext {
        request,
        invocation,
        backend_version,
        compiler_version,
        sources,
        owner,
        family,
    })
}

fn version(request: &ProcessRequest, phase: MutationPhase) -> Result<ProcessOutput, MutationError> {
    let run = native_process::run(request, None).map_err(MutationError::Process)?;
    let cause = match &run {
        ProcessRun::Pending(_) => Some("version query cleanup remains unfinished".to_owned()),
        ProcessRun::Finished(output) => version_refusal(output, phase),
    };
    if let Some(cause) = cause {
        return Err(MutationError::Query {
            phase,
            request: Box::new(request.clone()),
            run: Box::new(run),
            cause,
        });
    }
    match run {
        ProcessRun::Finished(output) => Ok(output),
        ProcessRun::Pending(pending) => Err(MutationError::Query {
            phase,
            request: Box::new(request.clone()),
            run: Box::new(ProcessRun::Pending(pending)),
            cause: "version query cleanup remains unfinished".to_owned(),
        }),
    }
}

fn version_refusal(output: &ProcessOutput, phase: MutationPhase) -> Option<String> {
    if !output.status().success()
        || output.stop() != &ProcessStop::Exited
        || output.stdout().end() != &CaptureEnd::Eof
        || output.stderr().end() != &CaptureEnd::Eof
    {
        return Some("version query did not finish with complete successful output".to_owned());
    }
    let Ok(text) = std::str::from_utf8(output.stdout().bytes()) else {
        return Some("version query is not UTF-8".to_owned());
    };
    let accepted = match phase {
        MutationPhase::BackendVersion => text.trim() == "cargo-mutants 27.0.0",
        MutationPhase::CompilerVersion => {
            let mut fields = text.split_whitespace();
            text.trim().lines().count() == 1
                && fields.next() == Some("rustc")
                && fields.next() == Some(RUSTC_COVERAGE_TOOLCHAIN)
        }
    };
    if accepted {
        None
    } else {
        Some(format!("unsupported {phase:?}: {}", text.trim()))
    }
}

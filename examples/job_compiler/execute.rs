use macroonz::native_compiler::{self, CompilerOutput, CompilerRequest, CompilerRun};
use macroonz::native_process::{ProcessLimits, ProcessRequest, ProcessRun};
use std::time::Duration;

pub(super) fn compile(request: &CompilerRequest) -> Result<CompilerOutput, String> {
    match native_compiler::compile(request).map_err(|error| error.to_string())? {
        CompilerRun::Finished(output) => Ok(*output),
        CompilerRun::Pending(pending) => Err(format!(
            "compiler cleanup is unfinished: {:?}",
            pending.process()
        )),
    }
}

pub(super) fn lawful_assertion(compiled: &CompilerOutput) -> Result<(), String> {
    let executable = compiled.executable().ok_or("no compiled executable")?;
    let selected = compiled.request().process();
    let limits = ProcessLimits::informed(
        Duration::from_secs(10),
        Duration::from_secs(5),
        65_536,
        65_536,
    )
    .map_err(|error| error.to_string())?;
    let request = ProcessRequest::informed(
        executable.to_path_buf(),
        selected.directory().to_path_buf(),
        Vec::new(),
        selected.environment().to_vec(),
        limits,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let read = match compiled
        .read_back(&request, None)
        .map_err(|error| error.to_string())?
    {
        ProcessRun::Finished(output) => output,
        ProcessRun::Pending(pending) => {
            return Err(format!("read-back cleanup is unfinished: {pending:?}"));
        }
    };
    native_compiler::observed_read_back(&read, |bytes| {
        if bytes.is_empty() {
            Ok(())
        } else {
            Err("the lawful fixture wrote unexpected stdout".to_owned())
        }
    })
    .map_err(|error| {
        format!(
            "lawful execution was not established: {error:?}; {}",
            String::from_utf8_lossy(read.stderr().bytes())
        )
    })
}

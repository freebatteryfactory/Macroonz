//! Materialize exact caller source and compose bounded native compilation and read-back.

use super::types::Sample;
use macroonz::harness::muterprater::{
    CompiledSpecimenHostRefusal, CompiledSpecimenObservation, CompiledSpecimenRequest,
    CompiledSpecimenRole,
};
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation, RelativeSourcePath};
use macroonz::harness::report::ForeignText;
use macroonz::native_compiler::{
    self, CompilerOutput, CompilerRequest, CompilerRun, ReadBackError,
};
use macroonz::native_process::{ProcessLimits, ProcessRequest, ProcessRun, ProcessTool};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

pub(super) fn observe(
    request: &CompiledSpecimenRequest<'_, '_, Sample<'_>>,
) -> Result<CompiledSpecimenObservation<u32>, CompiledSpecimenHostRefusal> {
    let meaning = execute(request)?;
    let count = &request.input().executions;
    count.set(count.get().saturating_add(1));
    Ok(CompiledSpecimenObservation::executed(request, meaning))
}

fn execute(
    request: &CompiledSpecimenRequest<'_, '_, Sample<'_>>,
) -> Result<u32, CompiledSpecimenHostRefusal> {
    let settings = request.input().settings;
    let role = match request.role() {
        CompiledSpecimenRole::Baseline => "baseline",
        CompiledSpecimenRole::Selected(_) => "selected",
    };
    let directory = settings.compiler.directory().join(role);
    std::fs::create_dir(&directory).map_err(|error| compilation(&error.to_string()))?;
    write_new(&directory.join("predicate.rs"), request.content().bytes())
        .map_err(|detail| compilation(&detail))?;
    let input_path = directory.join("input.txt");
    write_new(&input_path, request.input().value.to_string().as_bytes())
        .map_err(|detail| compilation(&detail))?;
    let tool = ProcessTool::informed(
        settings.compiler.executable().to_path_buf(),
        directory.clone(),
        settings.compiler.environment().to_vec(),
        settings.compiler.limits(),
        &[],
    )
    .map_err(|error| compilation(&error.to_string()))?;
    let artifact = directory.join(format!("predicate{}", std::env::consts::EXE_SUFFIX));
    let build = CompilerRequest::rustc(
        &tool,
        RelativeSourcePath::informed("predicate.rs")
            .map_err(|error| compilation(&super::debug(error)))?,
        artifact.clone(),
        &settings.target,
    )
    .map_err(|error| compilation(&error.to_string()))?;
    let compiled =
        match native_compiler::compile(&build).map_err(|error| compilation(&error.to_string()))? {
            CompilerRun::Finished(output) => output,
            CompilerRun::Pending(pending) => {
                return Err(compilation(&format!(
                    "unfinished compiler cleanup: {:?}",
                    pending.process()
                )));
            }
        };
    let verdict = compiled
        .compared(&DeclaredCompilation::compiles())
        .map_err(|error| {
            compilation(&format!(
                "{error:?}; {}",
                String::from_utf8_lossy(compiled.process().stderr().bytes())
            ))
        })?;
    if verdict != CompilationVerdict::Conforms {
        return Err(compilation(&format!("compilation disagreed: {verdict:?}")));
    }
    let reader = ProcessRequest::informed(
        artifact,
        directory,
        Vec::new(),
        settings.compiler.environment().to_vec(),
        ProcessLimits::informed(Duration::from_secs(10), Duration::from_secs(5), 64, 65_536)
            .map_err(|error| execution(&error.to_string()))?,
        &[],
    )
    .map_err(|error| execution(&error.to_string()))?;
    read(&compiled, &reader, &input_path)
}

fn read(
    compiled: &CompilerOutput,
    reader: &ProcessRequest,
    input_path: &Path,
) -> Result<u32, CompiledSpecimenHostRefusal> {
    let read = match compiled
        .read_back(
            reader,
            Some(File::open(input_path).map_err(|error| execution(&error.to_string()))?),
        )
        .map_err(|error| execution(&error.to_string()))?
    {
        ProcessRun::Finished(output) => output,
        ProcessRun::Pending(pending) => {
            return Err(execution(&format!(
                "unfinished reader cleanup: {pending:?}"
            )));
        }
    };
    native_compiler::observed_read_back(&read, |bytes| {
        std::str::from_utf8(bytes)
            .map_err(super::debug)?
            .trim()
            .parse::<u32>()
            .map_err(super::debug)
    })
    .map_err(|error| match error {
        ReadBackError::Decode(message) => {
            CompiledSpecimenHostRefusal::Meaning(ForeignText::admitted(message.as_bytes()))
        }
        ReadBackError::Interrupted(_) | ReadBackError::ProcessFailure => {
            execution(&format!("read-back was not established: {error:?}"))
        }
    })
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(super::debug)?;
    file.write_all(bytes).map_err(super::debug)
}

fn compilation(message: &str) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Compilation(ForeignText::admitted(message.as_bytes()))
}

fn execution(message: &str) -> CompiledSpecimenHostRefusal {
    CompiledSpecimenHostRefusal::Execution(ForeignText::admitted(message.as_bytes()))
}

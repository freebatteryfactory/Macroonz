//! Compile a supplied binary, execute its typed count read-back and compare an independent expectation.

mod configuration;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::harness::oracle::{
    CompilationVerdict, CompiledVerdict, DeclaredBehavior, DeclaredCompilation, DeclaredReadBack,
    DeclaredReadBackRoster, ObservedMember, ObservedValue,
};
use macroonz::native_compiler::{self, CompilerRun};
use macroonz::native_process::ProcessRun;
use std::io::Write;

fn main() -> Result<(), String> {
    let settings = configuration::read()?;
    let compiled =
        match native_compiler::compile(&settings.compiler).map_err(|error| error.to_string())? {
            CompilerRun::Finished(output) => output,
            CompilerRun::Pending(pending) => {
                return Err(format!(
                    "compiler cleanup is unfinished: {:?}",
                    pending.process()
                ));
            }
        };
    let verdict = compiled
        .compared(&DeclaredCompilation::compiles())
        .map_err(|error| {
            format!(
                "compiler observation was not established: {error:?}; {}",
                String::from_utf8_lossy(compiled.process().stderr().bytes())
            )
        })?;
    if verdict != CompilationVerdict::Conforms {
        return Err(format!("compilation disagreed: {verdict:?}"));
    }
    let read = match compiled
        .read_back(&settings.reader, None)
        .map_err(|error| error.to_string())?
    {
        ProcessRun::Finished(output) => output,
        ProcessRun::Pending(pending) => {
            return Err(format!("read-back cleanup is unfinished: {pending:?}"));
        }
    };
    let members = [DeclaredReadBack {
        name: "count",
        value: ObservedValue::Count(settings.expected_count),
    }];
    let declared = DeclaredBehavior::ReadsBack(
        DeclaredReadBackRoster::declared(&members)
            .map_err(|error| format!("declaration: {error:?}"))?,
    );
    let read_verdict = native_compiler::compared_read_back(&read, count, &declared)
        .map_err(|error| format!("read-back was not established: {error:?}"))?;
    if read_verdict != CompiledVerdict::Conforms {
        return Err(format!("read-back disagreed: {read_verdict:?}"));
    }
    writeln!(
        std::io::stdout(),
        "compiled count agrees with the independent expectation"
    )
    .map_err(|error| error.to_string())
}

fn count(bytes: &[u8]) -> Result<Vec<ObservedMember>, String> {
    let value = std::str::from_utf8(bytes)
        .map_err(|error| error.to_string())?
        .trim()
        .parse::<u64>()
        .map_err(|error| error.to_string())?;
    Ok(vec![ObservedMember {
        name: "count".to_owned(),
        value: ObservedValue::Count(value),
    }])
}

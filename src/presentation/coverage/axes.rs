//! Coverage process roles and execution classifications retain their original planes.

#[cfg(feature = "native-tooling")]
use crate::harness::fuzz::CoverageCommand;
use crate::harness::fuzz::{CoverageTool, FuzzExecution, RustcCommand, RustcField};
use crate::presentation::value::tagged;
use serde_json::Value;

#[cfg(feature = "native-tooling")]
pub(in crate::presentation) fn command(value: CoverageCommand) -> Value {
    match value {
        CoverageCommand::Rustc(value) => tagged("rustc", rustc(value).into()),
        CoverageCommand::Version(value) => tagged("version", tool(value).into()),
        CoverageCommand::Target => tagged("target", Value::Null),
        CoverageCommand::Merge => tagged("merge", Value::Null),
        CoverageCommand::Export => tagged("export", Value::Null),
    }
}

pub(super) fn rustc(value: RustcCommand) -> &'static str {
    match value {
        RustcCommand::VerboseVersion => "verbose-version",
        RustcCommand::Sysroot => "sysroot",
    }
}

pub(super) fn field(value: RustcField) -> &'static str {
    match value {
        RustcField::Release => "release",
        RustcField::Host => "host",
        RustcField::LlvmVersion => "llvm-version",
        RustcField::Sysroot => "sysroot",
    }
}

pub(super) fn tool(value: CoverageTool) -> &'static str {
    match value {
        CoverageTool::Profdata => "profdata",
        CoverageTool::Cov => "cov",
    }
}

pub(super) fn execution(value: FuzzExecution) -> Value {
    match value {
        FuzzExecution::Success => tagged("success", Value::Null),
        FuzzExecution::NonzeroExit(code) => tagged("nonzero-exit", code.into()),
        FuzzExecution::Crash(code) => tagged("crash", code.into()),
        FuzzExecution::Timeout => tagged("timeout", Value::Null),
        FuzzExecution::ResourceExhaustion => tagged("resource-exhaustion", Value::Null),
    }
}

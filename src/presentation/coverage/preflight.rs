//! Readiness refusals retain source, compiler and LLVM attribution.

use super::axes;
use crate::harness::fuzz::{CoverageRootsRefusal, CoverageSourceRootRefusal, PreflightIncomplete};
use crate::presentation::{
    Presentation, context,
    path::path,
    value::{object, tagged},
};
use serde_json::Value;
use std::path::Path;

/// Project the owning refusal that prevented coverage readiness.
pub fn coverage_preflight_refusal(record: &PreflightIncomplete) -> Presentation {
    Presentation::projected(
        "coverage-preflight-refusal",
        "macroonz-harness/fuzz",
        "recorded",
        object([("phase", "preflight".into()), ("cause", preflight(record))]),
    )
}

pub(in crate::presentation) fn preflight(record: &PreflightIncomplete) -> Value {
    match record {
        PreflightIncomplete::UnexpectedExecutorReply => {
            tagged("unexpected-executor-reply", Value::Null)
        }
        PreflightIncomplete::SourceRoots(value) => tagged("source-roots", roots(value)),
        PreflightIncomplete::TargetUnavailable { path: at, error } => {
            path_error("target-unavailable", at, error)
        }
        PreflightIncomplete::TargetNotFile(at) => tagged("target-not-file", path(at)),
        PreflightIncomplete::SourceRootUnavailable { path: at, error } => {
            path_error("source-root-unavailable", at, error)
        }
        PreflightIncomplete::SourceRootNotDirectory(at) => {
            tagged("source-root-not-directory", path(at))
        }
        PreflightIncomplete::SourceRootIdentity(value) => {
            tagged("source-root-identity", root_identity(*value).into())
        }
        PreflightIncomplete::StartRustc { command, error } => {
            with_role("start-rustc", axes::rustc(*command), error.as_str().into())
        }
        PreflightIncomplete::RustcFailed { command, code } => {
            with_role("rustc-failed", axes::rustc(*command), (*code).into())
        }
        PreflightIncomplete::RustcOutputNotUtf8(command) => {
            tagged("rustc-output-not-utf8", axes::rustc(*command).into())
        }
        PreflightIncomplete::MissingRustcField(field) => {
            tagged("missing-rustc-field", axes::field(*field).into())
        }
        PreflightIncomplete::RustcRelease { required, observed } => tagged(
            "rustc-release",
            object([
                ("required", (*required).into()),
                ("observed", observed.as_str().into()),
            ]),
        ),
        PreflightIncomplete::RelativeRustcSysroot(at) => tagged("relative-rustc-sysroot", path(at)),
        PreflightIncomplete::StartLlvmTool {
            tool,
            path: at,
            error,
        } => tagged(
            "start-llvm-tool",
            object([
                ("tool", axes::tool(*tool).into()),
                ("path", path(at)),
                ("error", error.as_str().into()),
            ]),
        ),
        PreflightIncomplete::LlvmToolFailed { tool, code } => {
            with_role("llvm-tool-failed", axes::tool(*tool), (*code).into())
        }
        PreflightIncomplete::LlvmToolOutputNotUtf8(tool) => {
            tagged("llvm-tool-output-not-utf8", axes::tool(*tool).into())
        }
        PreflightIncomplete::MissingLlvmToolVersion(tool) => {
            tagged("missing-llvm-tool-version", axes::tool(*tool).into())
        }
        PreflightIncomplete::LlvmToolVersionsDiffer { profdata, cov } => tagged(
            "llvm-tool-versions-differ",
            object([
                ("profdata", profdata.as_str().into()),
                ("cov", cov.as_str().into()),
            ]),
        ),
        PreflightIncomplete::RustcLlvmVersion { rustc, tools } => tagged(
            "rustc-llvm-version",
            object([
                ("rustc", rustc.as_str().into()),
                ("tools", tools.as_str().into()),
            ]),
        ),
    }
}

fn with_role(kind: &str, role: &str, value: Value) -> Value {
    tagged(kind, object([("role", role.into()), ("detail", value)]))
}

fn path_error(kind: &str, at: &Path, error: &str) -> Value {
    tagged(kind, object([("path", path(at)), ("error", error.into())]))
}

fn roots(value: &CoverageRootsRefusal) -> Value {
    match value {
        CoverageRootsRefusal::Empty => tagged("empty", Value::Null),
        CoverageRootsRefusal::DuplicateName(name) => {
            tagged("duplicate-name", context::declared_name(*name))
        }
        CoverageRootsRefusal::OverlappingPaths { left, right } => tagged(
            "overlapping-paths",
            object([("left", (*left).into()), ("right", (*right).into())]),
        ),
    }
}

fn root_identity(value: CoverageSourceRootRefusal) -> &'static str {
    match value {
        CoverageSourceRootRefusal::EmptyCheckout => "empty-checkout",
        CoverageSourceRootRefusal::RelativeCheckout => "relative-checkout",
        CoverageSourceRootRefusal::CheckoutTraversal => "checkout-traversal",
        CoverageSourceRootRefusal::NonUtf8Checkout => "non-utf8-checkout",
    }
}

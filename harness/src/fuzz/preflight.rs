//! Active readiness for the stable rustc coverage road.

use super::types::RustcCoverageTools;
use super::{
    CoverageSourceRoot, CoverageStanding, CoverageTool, PreflightIncomplete,
    RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight, RustcCommand, RustcField, RustcProfileRequest,
};
use crate::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Establish one executable rustc coverage road from its declared compiler and target.
///
/// The LLVM tools are derived from the exact rustc sysroot and host rather than accepted from the caller.
///
/// # Errors
///
/// Refuses an unavailable target or source root, an unusable compiler or LLVM tool, a non-1.98 compiler, or LLVM tools whose reported versions do not match that compiler.
pub fn preflight_ready(
    request: RustcProfileRequest,
) -> Result<ReadyPreflight, PreflightIncomplete> {
    target_available(&request)?;
    let source_root = canonical_source_root(&request)?;

    let verbose = rustc_output(&request, &["-vV"], RustcCommand::VerboseVersion)?;
    let verbose = rustc_text(verbose, RustcCommand::VerboseVersion)?;
    let release = required_field(&verbose, "release: ", RustcField::Release)?;
    if release != RUSTC_COVERAGE_TOOLCHAIN {
        return Err(PreflightIncomplete::RustcRelease {
            required: RUSTC_COVERAGE_TOOLCHAIN,
            observed: release.to_owned(),
        });
    }
    let host = required_field(&verbose, "host: ", RustcField::Host)?;
    let rustc_llvm = required_field(&verbose, "LLVM version: ", RustcField::LlvmVersion)?;

    let sysroot = rustc_output(&request, &["--print", "sysroot"], RustcCommand::Sysroot)?;
    let sysroot = rustc_text(sysroot, RustcCommand::Sysroot)?;
    let sysroot = sysroot.trim();
    if sysroot.is_empty() {
        return Err(PreflightIncomplete::MissingRustcField(RustcField::Sysroot));
    }
    let sysroot = PathBuf::from(sysroot);
    if !sysroot.is_absolute() {
        return Err(PreflightIncomplete::RelativeRustcSysroot(sysroot));
    }

    let directory = sysroot.join("lib").join("rustlib").join(host).join("bin");
    let profdata = directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    let cov = directory.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX));
    let profdata_version = llvm_tool_version(CoverageTool::Profdata, &profdata)?;
    let cov_version = llvm_tool_version(CoverageTool::Cov, &cov)?;
    if profdata_version != cov_version {
        return Err(PreflightIncomplete::LlvmToolVersionsDiffer {
            profdata: profdata_version,
            cov: cov_version,
        });
    }
    let tool_llvm = profdata_version
        .split_once('-')
        .map_or(profdata_version.as_str(), |(version, _suffix)| version);
    if tool_llvm != rustc_llvm {
        return Err(PreflightIncomplete::RustcLlvmVersion {
            rustc: rustc_llvm.to_owned(),
            tools: profdata_version,
        });
    }

    let tools = RustcCoverageTools::established(profdata, cov);
    let toolchain = format!("rustc {release} LLVM {rustc_llvm}");
    let target = TargetBinding::bound(
        TargetTriple::declared(host),
        ToolchainIdentity::declared(&toolchain),
    );
    let standing = CoverageStanding::established(request.campaign(), target);
    Ok(ReadyPreflight {
        request,
        tools,
        source_root,
        standing,
        sysroot,
        release: release.to_owned(),
        host: host.to_owned(),
        llvm_version: rustc_llvm.to_owned(),
    })
}

fn target_available(request: &RustcProfileRequest) -> Result<(), PreflightIncomplete> {
    let path = request.target().executable();
    let metadata =
        std::fs::metadata(path).map_err(|error| PreflightIncomplete::TargetUnavailable {
            path: path.to_path_buf(),
            error: error.to_string(),
        })?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(PreflightIncomplete::TargetNotFile(path.to_path_buf()))
    }
}

fn canonical_source_root(
    request: &RustcProfileRequest,
) -> Result<CoverageSourceRoot, PreflightIncomplete> {
    let path = request.source_root().checkout();
    let metadata =
        std::fs::metadata(path).map_err(|error| PreflightIncomplete::SourceRootUnavailable {
            path: path.to_path_buf(),
            error: error.to_string(),
        })?;
    if metadata.is_dir() {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            PreflightIncomplete::SourceRootUnavailable {
                path: path.to_path_buf(),
                error: error.to_string(),
            }
        })?;
        CoverageSourceRoot::declared(request.source_root().logical(), canonical)
            .map_err(PreflightIncomplete::SourceRootIdentity)
    } else {
        Err(PreflightIncomplete::SourceRootNotDirectory(
            path.to_path_buf(),
        ))
    }
}

fn rustc_output(
    request: &RustcProfileRequest,
    arguments: &[&str],
    command: RustcCommand,
) -> Result<Output, PreflightIncomplete> {
    let output = Command::new(request.rustc())
        .args(arguments)
        .output()
        .map_err(|error| PreflightIncomplete::StartRustc {
            command,
            error: error.to_string(),
        })?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(PreflightIncomplete::RustcFailed {
            command,
            code: output.status.code(),
        })
    }
}

fn rustc_text(output: Output, command: RustcCommand) -> Result<String, PreflightIncomplete> {
    String::from_utf8(output.stdout)
        .map_err(|_error| PreflightIncomplete::RustcOutputNotUtf8(command))
}

fn required_field<'text>(
    text: &'text str,
    prefix: &str,
    field: RustcField,
) -> Result<&'text str, PreflightIncomplete> {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix))
        .filter(|value| !value.is_empty())
        .ok_or(PreflightIncomplete::MissingRustcField(field))
}

fn llvm_tool_version(tool: CoverageTool, path: &Path) -> Result<String, PreflightIncomplete> {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .map_err(|error| PreflightIncomplete::StartLlvmTool {
            tool,
            path: path.to_path_buf(),
            error: error.to_string(),
        })?;
    if !output.status.success() {
        return Err(PreflightIncomplete::LlvmToolFailed {
            tool,
            code: output.status.code(),
        });
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|_error| PreflightIncomplete::LlvmToolOutputNotUtf8(tool))?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("LLVM version "))
        .filter(|version| !version.is_empty())
        .map(str::to_owned)
        .ok_or(PreflightIncomplete::MissingLlvmToolVersion(tool))
}

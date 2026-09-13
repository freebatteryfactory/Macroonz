//! Active readiness for the stable rustc coverage road.

use super::CoverageSourceRoots;
use super::types::RustcCoverageTools;
use super::{
    CoverageCommand, CoverageHostFailure, CoverageInvocation, CoverageReply, CoverageSourceRoot,
    CoverageStanding, CoverageTool, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight,
    RustcCommand, RustcField, RustcProfileRequest,
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
    preflight_ready_with(request, |mut invocation| {
        let path = PathBuf::from(invocation.command.get_program());
        invocation
            .command
            .output()
            .map(CoverageReply::Output)
            .map_err(|error| (path, error))
    })
    .map_err(|failure| match failure {
        CoverageHostFailure::Refused(refusal) => refusal,
        CoverageHostFailure::Executor {
            operation,
            error: (path, error),
            ..
        } => match operation {
            CoverageCommand::Rustc(command) => PreflightIncomplete::StartRustc {
                command,
                error: error.to_string(),
            },
            CoverageCommand::Version(tool) => PreflightIncomplete::StartLlvmTool {
                tool,
                path,
                error: error.to_string(),
            },
            CoverageCommand::Target | CoverageCommand::Merge | CoverageCommand::Export => {
                PreflightIncomplete::UnexpectedExecutorReply
            }
        },
    })
}

/// Establishes the existing coverage standing through an explicitly supplied process executor.
///
/// # Errors
/// Retains executor failures separately from compiler, tool and source-root refusals.
pub fn preflight_ready_with<E>(
    request: RustcProfileRequest,
    mut execute: impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<ReadyPreflight, CoverageHostFailure<E, PreflightIncomplete>> {
    target_available(&request)?;
    let source_roots = request
        .source_roots()
        .iter()
        .map(canonical_source_root)
        .collect::<Result<Vec<_>, _>>()?;
    let source_roots =
        CoverageSourceRoots::declared(source_roots).map_err(PreflightIncomplete::SourceRoots)?;

    let verbose = rustc_output(
        &request,
        &["-vV"],
        RustcCommand::VerboseVersion,
        &mut execute,
    )?;
    let verbose = rustc_text(verbose, RustcCommand::VerboseVersion)?;
    let release = required_field(&verbose, "release: ", RustcField::Release)?;
    if release != RUSTC_COVERAGE_TOOLCHAIN {
        return Err(PreflightIncomplete::RustcRelease {
            required: RUSTC_COVERAGE_TOOLCHAIN,
            observed: release.to_owned(),
        }
        .into());
    }
    let host = required_field(&verbose, "host: ", RustcField::Host)?;
    let rustc_llvm = required_field(&verbose, "LLVM version: ", RustcField::LlvmVersion)?;

    let sysroot = rustc_output(
        &request,
        &["--print", "sysroot"],
        RustcCommand::Sysroot,
        &mut execute,
    )?;
    let sysroot = rustc_text(sysroot, RustcCommand::Sysroot)?;
    let sysroot = sysroot.trim();
    if sysroot.is_empty() {
        return Err(PreflightIncomplete::MissingRustcField(RustcField::Sysroot).into());
    }
    let sysroot = PathBuf::from(sysroot);
    if !sysroot.is_absolute() {
        return Err(PreflightIncomplete::RelativeRustcSysroot(sysroot).into());
    }

    let directory = sysroot.join("lib").join("rustlib").join(host).join("bin");
    let profdata = directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    let cov = directory.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX));
    let tools = matching_tools(profdata, cov, rustc_llvm, &mut execute)?;
    let toolchain = format!("rustc {release} LLVM {rustc_llvm}");
    let target = TargetBinding::bound(
        request
            .target()
            .triple()
            .cloned()
            .unwrap_or_else(|| TargetTriple::declared(host)),
        ToolchainIdentity::declared(&toolchain),
    );
    let standing = CoverageStanding::established(request.campaign(), target);
    Ok(ReadyPreflight {
        request,
        tools,
        source_roots,
        standing,
        sysroot,
        release: release.to_owned(),
        host: host.to_owned(),
        llvm_version: rustc_llvm.to_owned(),
    })
}

fn matching_tools<E>(
    profdata: PathBuf,
    cov: PathBuf,
    rustc_llvm: &str,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<RustcCoverageTools, CoverageHostFailure<E, PreflightIncomplete>> {
    let profdata_version = llvm_tool_version(CoverageTool::Profdata, &profdata, execute)?;
    let cov_version = llvm_tool_version(CoverageTool::Cov, &cov, execute)?;
    if profdata_version != cov_version {
        return Err(PreflightIncomplete::LlvmToolVersionsDiffer {
            profdata: profdata_version,
            cov: cov_version,
        }
        .into());
    }
    let tool_llvm = profdata_version
        .split_once('-')
        .map_or(profdata_version.as_str(), |(version, _suffix)| version);
    if tool_llvm != rustc_llvm {
        return Err(PreflightIncomplete::RustcLlvmVersion {
            rustc: rustc_llvm.to_owned(),
            tools: profdata_version,
        }
        .into());
    }

    Ok(RustcCoverageTools::established(
        profdata,
        cov,
        profdata_version,
    ))
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
    root: &CoverageSourceRoot,
) -> Result<CoverageSourceRoot, PreflightIncomplete> {
    let path = root.checkout();
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
        CoverageSourceRoot::declared(root.logical(), canonical)
            .map_err(PreflightIncomplete::SourceRootIdentity)
    } else {
        Err(PreflightIncomplete::SourceRootNotDirectory(
            path.to_path_buf(),
        ))
    }
}

fn rustc_output<E>(
    request: &RustcProfileRequest,
    arguments: &[&str],
    command: RustcCommand,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<Output, CoverageHostFailure<E, PreflightIncomplete>> {
    let mut selected = Command::new(request.rustc());
    selected.args(arguments);
    let output = tool_output(CoverageCommand::Rustc(command), selected, execute)?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(PreflightIncomplete::RustcFailed {
            command,
            code: output.status.code(),
        }
        .into())
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

fn llvm_tool_version<E>(
    tool: CoverageTool,
    path: &Path,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<String, CoverageHostFailure<E, PreflightIncomplete>> {
    let mut command = Command::new(path);
    command.arg("--version");
    let output = tool_output(CoverageCommand::Version(tool), command, execute)?;
    if !output.status.success() {
        return Err(PreflightIncomplete::LlvmToolFailed {
            tool,
            code: output.status.code(),
        }
        .into());
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|_error| PreflightIncomplete::LlvmToolOutputNotUtf8(tool))?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("LLVM version "))
        .filter(|version| !version.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| PreflightIncomplete::MissingLlvmToolVersion(tool).into())
}

fn tool_output<E>(
    operation: CoverageCommand,
    command: Command,
    execute: &mut impl FnMut(CoverageInvocation) -> Result<CoverageReply, E>,
) -> Result<Output, CoverageHostFailure<E, PreflightIncomplete>> {
    let invocation = CoverageInvocation {
        operation,
        command,
        input: None,
        output_bound: None,
    };
    match execute(invocation) {
        Ok(CoverageReply::Output(output)) => Ok(output),
        Ok(CoverageReply::Target(_)) => Err(PreflightIncomplete::UnexpectedExecutorReply.into()),
        Err(error) => Err(CoverageHostFailure::Executor {
            operation,
            error,
            cleanup: None,
        }),
    }
}

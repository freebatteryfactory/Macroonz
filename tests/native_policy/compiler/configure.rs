use super::types::Host;
use macroonz::harness::oracle::{
    DiagnosticAnchor, PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use macroonz::native_compiler::{self, CompilerOutput, CompilerRequest, CompilerRun};
use macroonz::native_process::{ProcessLimits, ProcessTool};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

pub(crate) fn root() -> Result<PathBuf, String> {
    super::super::check::scratch()
}

pub(crate) fn bounds() -> Result<ProcessLimits, String> {
    ProcessLimits::informed(
        Duration::from_secs(60),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())
}

pub(crate) fn host(root: &Path) -> Result<Host, String> {
    let rustc = which("rustc")?;
    let cargo = which("cargo")?;
    let helper = root.join(format!(
        "compiler-environment{}",
        std::env::consts::EXE_SUFFIX
    ));
    let source = root.join("compiler_environment.rs");
    std::fs::write(&source, include_str!("environment_subject.rs"))
        .map_err(|error| error.to_string())?;
    let compile = Command::new(&rustc)
        .arg(&source)
        .args([
            "--edition=2024",
            "--forbid=unsafe_code",
            "--deny=warnings",
            "-o",
        ])
        .arg(&helper)
        .output()
        .map_err(|error| error.to_string())?;
    if !compile.status.success() {
        return Err(String::from_utf8_lossy(&compile.stderr).into_owned());
    }
    let observed = Command::new(helper)
        .output()
        .map_err(|error| error.to_string())?;
    if !observed.status.success() {
        return Err(String::from_utf8_lossy(&observed.stderr).into_owned());
    }
    let raw = String::from_utf8(observed.stdout).map_err(|error| error.to_string())?;
    let fields: Vec<_> = raw
        .strip_suffix('\0')
        .ok_or("missing environment terminator")?
        .split('\0')
        .collect();
    let mut environment = Vec::new();
    for pair in fields.chunks(2) {
        let [key, value] = pair else {
            return Err("incomplete environment pair".to_owned());
        };
        environment.push(((*key).to_owned(), (*value).to_owned()));
    }
    environment.push(("RUSTC".to_owned(), spelling(&rustc)?));
    environment.push(("CARGO_INCREMENTAL".to_owned(), "0".to_owned()));
    let tool = ProcessTool::informed(
        rustc.clone(),
        root.to_path_buf(),
        environment.clone(),
        bounds()?,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let request = tool
        .invocation(vec!["-vV".to_owned()])
        .map_err(|error| error.to_string())?;
    let version = super::super::process::completed(
        macroonz::native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    if !version.status().success() {
        return Err(format!("rustc identity failed: {version:?}"));
    }
    let identity =
        std::str::from_utf8(version.stdout().bytes()).map_err(|error| error.to_string())?;
    if !identity.lines().any(|line| line == "release: 1.98.1") {
        return Err(format!("unexpected declared test compiler: {identity}"));
    }
    let triple = identity
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or("missing rustc host")?
        .to_owned();
    Ok(Host {
        rustc,
        cargo,
        triple,
        environment,
    })
}

fn which(tool: &str) -> Result<PathBuf, String> {
    let output = Command::new("rustup")
        .args(["which", "--toolchain", "1.98.1", tool])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    Ok(PathBuf::from(
        String::from_utf8(output.stdout)
            .map_err(|error| error.to_string())?
            .trim(),
    ))
}

pub(crate) fn tool(
    host: &Host,
    executable: &Path,
    root: &Path,
    limits: ProcessLimits,
) -> Result<ProcessTool, String> {
    ProcessTool::informed(
        executable.to_path_buf(),
        root.to_path_buf(),
        host.environment.clone(),
        limits,
        &[],
    )
    .map_err(|error| error.to_string())
}

pub(crate) fn finish(run: CompilerRun) -> Result<CompilerOutput, String> {
    match run {
        CompilerRun::Finished(output) => Ok(*output),
        CompilerRun::Pending(pending) => {
            let detail = format!("unexpected compiler cleanup: {:?}", pending.process());
            drop(pending.finish(Duration::from_secs(5)));
            Err(detail)
        }
    }
}

pub(crate) fn locus() -> Result<RelativeSourcePath, String> {
    RelativeSourcePath::informed("fixture.rs").map_err(|error| format!("{error:?}"))
}

pub(crate) fn anchor(code: &str, end: u64) -> Result<DiagnosticAnchor, String> {
    let code = RustcErrorCode::informed(code).map_err(|error| format!("{error:?}"))?;
    let start = SourcePosition::informed(2, 21).map_err(|error| format!("{error:?}"))?;
    let end = SourcePosition::informed(2, end).map_err(|error| format!("{error:?}"))?;
    let span =
        PrimarySourceSpan::informed(locus()?, start, end).map_err(|error| format!("{error:?}"))?;
    Ok(DiagnosticAnchor::at(code, span))
}

pub(crate) fn spelling(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| "non-Unicode fixture path".to_owned())
}

pub(crate) fn target() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_TARGET_TMPDIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "missing Cargo target parent".to_owned())
}

pub(crate) fn standin(
    root: &Path,
    host: &Host,
    name: &str,
    source: &str,
) -> Result<PathBuf, String> {
    let filename = format!("{name}.rs");
    std::fs::write(root.join(&filename), source).map_err(|error| error.to_string())?;
    let executable = root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let source_path =
        RelativeSourcePath::informed(&filename).map_err(|error| format!("{error:?}"))?;
    let request = CompilerRequest::rustc(
        &tool(host, &host.rustc, root, bounds()?)?,
        source_path,
        executable.clone(),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    if output.observed().is_err()
        || output
            .observed()
            .is_ok_and(|value| value.refusal().is_some())
    {
        return Err(format!("stand-in failed to compile: {output:?}"));
    }
    Ok(executable)
}

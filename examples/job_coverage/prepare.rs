use super::types::Settings;
use macroonz::harness::descriptor::NamespacedName;
use macroonz::harness::fuzz::{CoverageSourceRoot, InstrumentedTarget, RustcProfileRequest};
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation};
use macroonz::native_compiler::{self, CompilerRun};
use macroonz::native_coverage::{self, NativeCoverage};
use macroonz::native_process::ProcessLimits;
use std::time::Duration;

pub(super) fn ready(settings: Settings) -> Result<NativeCoverage, String> {
    let compiled = match native_compiler::compile(&settings.compiler).map_err(debug)? {
        CompilerRun::Finished(output) => output,
        CompilerRun::Pending(pending) => {
            return Err(format!(
                "compiler cleanup is unfinished: {:?}",
                pending.process()
            ));
        }
    };
    if compiled.compared(&DeclaredCompilation::compiles()) != Ok(CompilationVerdict::Conforms) {
        return Err(format!(
            "compilation did not conform: {:?}; {}",
            compiled.observed(),
            String::from_utf8_lossy(compiled.process().stderr().bytes())
        ));
    }
    let artifact = compiled
        .executable()
        .ok_or("missing instrumented executable")?;
    let request = RustcProfileRequest::declared(
        settings.tool.executable().to_path_buf(),
        InstrumentedTarget::for_target(artifact.to_path_buf(), Vec::new(), settings.target)
            .map_err(debug)?,
        CoverageSourceRoot::declared(
            NamespacedName::named("neutral-job", "consumer").map_err(debug)?,
            settings.tool.directory().to_path_buf(),
        )
        .map_err(debug)?,
        settings.scratch,
        settings.campaign,
    )
    .map_err(debug)?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(2),
        Duration::from_secs(5),
        65_536,
        65_536,
    )
    .map_err(debug)?;
    native_coverage::preflight(request, settings.tool, limits).map_err(|error| error.to_string())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

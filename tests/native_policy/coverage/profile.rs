use super::configure::{campaign, compiled, debug};
use super::types::Instrumentation;
use crate::compiler::configure::{bounds, host, root, tool};
use macroonz::harness::descriptor::NamespacedName;
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageSourceRoot, CoverageTool, FuzzExecution, InstrumentedTarget,
    RustcProfileRequest,
};
use macroonz::harness::report::TargetTriple;
use macroonz::native_coverage;
use macroonz::native_process;

#[test]
fn actual_target_profile_exposes_the_tools_that_execute_coverage() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let executable = compiled(&run, &host, Instrumentation::Selected)?;
    let request = RustcProfileRequest::declared(
        host.rustc.clone(),
        InstrumentedTarget::for_target(
            executable.clone(),
            Vec::new(),
            TargetTriple::declared(&host.triple),
        )
        .map_err(debug)?,
        CoverageSourceRoot::declared(
            NamespacedName::named("native-test", "source").map_err(debug)?,
            run.clone(),
        )
        .map_err(debug)?,
        run.join("cases"),
        campaign(include_bytes!("subject.rs"))?,
    )
    .map_err(debug)?;
    let coverage = native_coverage::preflight(
        request,
        tool(&host, &host.rustc, &run, bounds()?)?,
        bounds()?,
    )
    .map_err(debug)?;
    let ready = coverage.ready();
    assert_eq!(ready.rustc(), host.rustc);
    assert_eq!(ready.release(), "1.98.1");
    assert_eq!(ready.host(), host.triple);
    assert_eq!(ready.standing().target().target().spelling(), host.triple);
    for (selected, name) in [
        (CoverageTool::Profdata, "llvm-profdata"),
        (CoverageTool::Cov, "llvm-cov"),
    ] {
        let path = ready.tool_path(selected);
        assert_eq!(
            path,
            ready
                .sysroot()
                .join("lib/rustlib")
                .join(&host.triple)
                .join("bin")
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
        );
        let query = tool(&host, path, &run, bounds()?)?
            .invocation(vec!["--version".to_owned()])
            .map_err(debug)?;
        let observed =
            crate::process::completed(native_process::run(&query, None).map_err(debug)?)?;
        assert!(observed.status().success());
        let text = std::str::from_utf8(observed.stdout().bytes()).map_err(debug)?;
        assert!(
            text.lines()
                .any(|line| line.trim() == format!("LLVM version {}", ready.tool_version()))
        );
    }
    let legacy = super::configure::ready(&run, &host, &executable, "unexecuted-legacy", bounds()?)?;
    assert_eq!(ready.standing(), legacy.ready().standing());
    super::presentation::readiness(&coverage, Some(&host.triple))?;
    super::presentation::readiness(&legacy, None)?;
    let mut corpus = coverage.corpus();
    let first = native_coverage::observe(&coverage, &mut corpus, &[1, 2, 3]).map_err(debug)?;
    assert_eq!(first.execution(), FuzzExecution::Success);
    assert!(matches!(
        corpus.admit(first),
        Ok(CoverageAdmission::Interesting(_))
    ));
    let repeated = native_coverage::observe(&coverage, &mut corpus, &[1, 2, 3]).map_err(debug)?;
    assert_eq!(corpus.admit(repeated), Ok(CoverageAdmission::Known));
    Ok(())
}

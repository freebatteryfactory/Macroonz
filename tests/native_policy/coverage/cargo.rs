use super::configure::{campaign_with_export, debug};
use crate::compiler::configure::{bounds, finish, host, locus, root, target, tool};
use macroonz::harness::descriptor::NamespacedName;
use macroonz::harness::fuzz::{
    CoverageAdmission, CoveragePoint, CoverageSourceRoot, CoverageSourceRoots, InstrumentedTarget,
    RustcProfileRequest,
};
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation};
use macroonz::harness::report::TargetTriple;
use macroonz::native_compiler::{self, CargoFixture, CargoTarget, CompilerRequest};
use macroonz::native_coverage;
use std::io::Write;

#[test]
fn instrumented_cargo_registry_decoder_observes_generated_behavior() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let (compiled, roots) = decoder(&run, &host)?;
    let executable = compiled.executable().ok_or("missing decoder executable")?;
    let application = NamespacedName::named("native-test", "application").map_err(debug)?;
    let campaign = campaign_with_export(include_bytes!("decoder_subject.rs"), 16_777_216)?;
    let mapped = RustcProfileRequest::mapped(
        host.rustc.clone(),
        InstrumentedTarget::for_target(
            executable.to_path_buf(),
            Vec::new(),
            TargetTriple::declared(&host.triple),
        )
        .map_err(debug)?,
        roots,
        run.join("cases"),
        campaign,
    )
    .map_err(debug)?;
    let limits = macroonz::native_process::ProcessLimits::informed(
        std::time::Duration::from_secs(60),
        std::time::Duration::from_secs(5),
        16_777_216,
        1_048_576,
    )
    .map_err(debug)?;
    let coverage =
        native_coverage::preflight(mapped, tool(&host, &host.rustc, &run, limits)?, bounds()?)
            .map_err(|failure| failure.to_string())?;
    let mut corpus = coverage.corpus();
    let truncated = native_coverage::observe(&coverage, &mut corpus, &[0])
        .map_err(|failure| failure.to_string())?;
    let truncated_points = truncated.observation().points().to_vec();
    assert!(truncated_points.iter().any(|point| matches!(point, CoveragePoint::Line { source, .. } if source.relative() == "macroonz-0.2.0/src/lib.rs")));
    assert!(matches!(
        corpus.admit(truncated),
        Ok(CoverageAdmission::Interesting(_))
    ));
    let valid_bytes = [0, 0, 0, 0, 0, 0, 0, 7];
    let valid = native_coverage::observe(&coverage, &mut corpus, &valid_bytes)
        .map_err(|failure| failure.to_string())?;
    assert!(valid.observation().points().iter().any(|point| {
        !truncated_points.contains(point) && matches!(point, CoveragePoint::Line { source, line: 10 } if source.root() == application && source.relative() == "fixture.rs")
    }), "generated decoding did not reach the caller's assembly: {:?}", valid.observation());
    assert!(matches!(
        corpus.admit(valid),
        Ok(CoverageAdmission::Interesting(_))
    ));
    let repeated = native_coverage::observe(&coverage, &mut corpus, &valid_bytes)
        .map_err(|failure| failure.to_string())?;
    assert_eq!(corpus.admit(repeated), Ok(CoverageAdmission::Known));
    let input = run.join("independent.bin");
    std::fs::write(&input, valid_bytes).map_err(debug)?;
    let reader = tool(&host, executable, &run, bounds()?)?
        .invocation(Vec::new())
        .map_err(debug)?;
    let output = crate::process::completed(
        compiled
            .read_back(&reader, Some(std::fs::File::open(&input).map_err(debug)?))
            .map_err(debug)?,
    )?;
    assert_eq!(output.stdout().bytes(), b"Ok(Ledger { count: 7 })\n");
    assert_eq!(corpus.attempted_cases(), 3);
    Ok(())
}

fn decoder(
    run: &std::path::Path,
    host: &crate::compiler::types::Host,
) -> Result<(native_compiler::CompilerOutput, CoverageSourceRoots), String> {
    let (package, manifest) = crate::compiler::real::package(
        run,
        "[[bin]]\nname = \"coverage-decoder\"\npath = \"fixture.rs\"\n",
    )?;
    std::fs::OpenOptions::new().append(true).open(&manifest).map_err(debug)?
        .write_all(b"[dependencies]\nbakery = { package = \"macroonz\", version = \"=0.2.0\", default-features = false }\n").map_err(debug)?;
    std::fs::write(run.join("fixture.rs"), include_bytes!("decoder_subject.rs")).map_err(debug)?;
    let dependencies = crate::compiler::registry::registry_graph(run, host, &manifest)?;
    let fixture = CargoFixture::informed(
        manifest,
        target()?,
        package,
        CargoTarget::Binary("coverage-decoder".to_owned()),
        host.triple.clone(),
    )
    .map_err(debug)?;
    let compile =
        CompilerRequest::cargo(&tool(host, &host.cargo, run, bounds()?)?, fixture, locus()?)
            .map_err(debug)?
            .instrumented()
            .map_err(debug)?;
    let compiled = finish(native_compiler::compile(&compile).map_err(debug)?)?;
    assert_eq!(
        compiled.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms),
        "{compiled:?}"
    );
    let application = NamespacedName::named("native-test", "application").map_err(debug)?;
    let mut roots =
        vec![CoverageSourceRoot::declared(application, run.to_path_buf()).map_err(debug)?];
    let mut registry = None;
    for (_name, directory) in dependencies {
        let parent = directory
            .parent()
            .ok_or("missing registry source root")?
            .to_path_buf();
        if let Some(registry) = &registry {
            assert_eq!(registry, &parent);
        }
        registry = Some(parent);
    }
    roots.push(
        CoverageSourceRoot::declared(
            NamespacedName::named("native-test", "registry-sources").map_err(debug)?,
            registry.ok_or("no registry sources")?,
        )
        .map_err(debug)?,
    );
    let roots = CoverageSourceRoots::declared(roots).map_err(debug)?;
    Ok((compiled, roots))
}

#[test]
fn cargo_instrumentation_refuses_conflicting_flag_authority_before_execution() -> Result<(), String>
{
    use macroonz::native_process::ProcessTool;
    let run = root()?;
    for name in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "rustflags"] {
        let selected = ProcessTool::informed(
            run.join("unexecuted-cargo"),
            run.clone(),
            vec![(name.to_owned(), "-Copt-level=3".to_owned())],
            bounds()?,
            &[],
        )
        .map_err(debug)?;
        let fixture = CargoFixture::informed(
            run.join("unread-manifest"),
            target()?,
            "fixture".to_owned(),
            CargoTarget::Library,
            "x86_64-pc-windows-msvc".to_owned(),
        )
        .map_err(debug)?;
        let request = CompilerRequest::cargo(&selected, fixture, locus()?).map_err(debug)?;
        assert!(matches!(
            request.instrumented(),
            Err(native_compiler::CompilerError::Configuration(_))
        ));
    }
    Ok(())
}

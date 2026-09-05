//! The LCOV claims: points are canonical across declared source roots, compiled coverage is identical across two physical roots, and paths with no root-independent identity refuse.

use super::support::{
    FuzzRoadFailure, RunScratch, compile_instrumented_subject, coverage_campaign, external,
    ready_for_compiled_root, rustc_path, wait_for_exit,
};
use macroonz_harness::descriptor::NamespacedName;
use macroonz_harness::fuzz::{
    CoverageCorpus, CoveragePoint, CoverageReadRefusal, CoverageSourceRoot, observe_rustc_profile,
    read_lcov,
};
use std::path::PathBuf;

#[test]
fn lcov_points_are_canonical_across_declared_source_roots() -> Result<(), FuzzRoadFailure> {
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage-source").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let first_checkout = std::env::temp_dir().join("macroonz-coverage-first");
    let second_checkout = std::env::temp_dir().join("macroonz-coverage-second");
    let first_root =
        CoverageSourceRoot::declared(logical, first_checkout.clone()).map_err(external)?;
    let second_root =
        CoverageSourceRoot::declared(logical, second_checkout.clone()).map_err(external)?;
    let first_source = first_checkout.join("src").join("subject.rs");
    let second_source = second_checkout.join("src").join("subject.rs");
    let alpha_lcov = format!(
        "TN:\nSF:{}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n",
        first_source.display()
    );
    let relocated_lcov = format!(
        "TN:\nSF:{}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n",
        second_source.display()
    );
    let alpha = read_lcov(&first_root, alpha_lcov.as_bytes())?;
    let relocated = read_lcov(&second_root, relocated_lcov.as_bytes())?;
    assert_eq!(alpha, relocated);
    let [line_point, branch_point] = alpha.points() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let CoveragePoint::Line {
        source: line_source,
        line: line_number,
    } = line_point
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(line_source.root(), logical);
    assert_eq!(line_source.relative(), "src/subject.rs");
    assert_eq!(*line_number, 10);
    let CoveragePoint::Branch {
        source: branch_source,
        line: branch_line,
        block,
        branch,
    } = branch_point
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(branch_source.root(), logical);
    assert_eq!(branch_source.relative(), "src/subject.rs");
    assert_eq!((*branch_line, *block, *branch), (12, 0, 0));
    assert!(!format!("{alpha:?}").contains(&first_checkout.display().to_string()));

    #[cfg(windows)]
    {
        let verbatim = format!(r"\\?\{}", first_source.display());
        let verbatim_lcov = format!(
            "TN:\nSF:{verbatim}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n"
        );
        assert_eq!(read_lcov(&first_root, verbatim_lcov.as_bytes())?, alpha);
    }

    Ok(())
}

#[test]
fn compiled_coverage_is_identical_across_two_physical_source_roots() -> Result<(), FuzzRoadFailure>
{
    let rustc = rustc_path()?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?
        .to_path_buf();
    let run = RunScratch::created(
        repository
            .join("target")
            .join("qualification")
            .join(format!("fuzz-two-source-roots-{}", std::process::id())),
    )?;
    let first_root = run.join("first-root");
    let second_root = run.join("second-root");
    let first_source = first_root.join("src").join("subject.rs");
    let second_source = second_root.join("src").join("subject.rs");
    std::fs::create_dir_all(first_source.parent().ok_or(FuzzRoadFailure::Fixture)?)
        .map_err(external)?;
    std::fs::create_dir_all(second_source.parent().ok_or(FuzzRoadFailure::Fixture)?)
        .map_err(external)?;
    let source = include_bytes!("rustc_coverage_subject.rs");
    std::fs::write(&first_source, source).map_err(external)?;
    std::fs::write(&second_source, source).map_err(external)?;
    let first_subject = first_root.join(format!("subject{}", std::env::consts::EXE_SUFFIX));
    let second_subject = second_root.join(format!("subject{}", std::env::consts::EXE_SUFFIX));
    compile_instrumented_subject(&rustc, &first_source, &first_subject)?;
    compile_instrumented_subject(&rustc, &second_source, &second_subject)?;
    let campaign = coverage_campaign()?;
    let first_ready = ready_for_compiled_root(
        rustc.clone(),
        first_subject,
        &first_root,
        run.join("first-cases"),
        campaign,
    )?;
    let second_ready = ready_for_compiled_root(
        rustc,
        second_subject,
        &second_root,
        run.join("second-cases"),
        campaign,
    )?;
    let mut first_corpus = CoverageCorpus::opening(&first_ready);
    let mut second_corpus = CoverageCorpus::opening(&second_ready);
    let first = observe_rustc_profile(&first_ready, &mut first_corpus, &[1], wait_for_exit)?;
    let second = observe_rustc_profile(&second_ready, &mut second_corpus, &[1], wait_for_exit)?;
    assert_eq!(first.standing(), second.standing());
    assert_eq!(first.observation(), second.observation());
    assert!(!first.observation().points().is_empty());
    run.removed()?;
    Ok(())
}

#[test]
fn lcov_refuses_paths_that_cannot_have_root_independent_identity() -> Result<(), FuzzRoadFailure> {
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage-hostile").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let checkout = std::env::temp_dir().join("macroonz-coverage-root");
    let root = CoverageSourceRoot::declared(logical, checkout.clone()).map_err(external)?;
    assert_eq!(
        read_lcov(&root, b"TN:\nSF:src/subject.rs\nDA:1,1\nend_of_record\n"),
        Err(CoverageReadRefusal::RelativeSource { record: 2 })
    );
    let traversing = format!(
        "TN:\nSF:{}\nDA:1,1\nend_of_record\n",
        checkout.join("src").join("..").join("escape.rs").display()
    );
    assert_eq!(
        read_lcov(&root, traversing.as_bytes()),
        Err(CoverageReadRefusal::SourceTraversal { record: 2 })
    );
    let outside = format!(
        "TN:\nSF:{}\nDA:1,1\nend_of_record\n",
        std::env::temp_dir().join("macroonz-outside.rs").display()
    );
    assert_eq!(
        read_lcov(&root, outside.as_bytes()),
        Err(CoverageReadRefusal::SourceOutsideRoot { record: 2 })
    );
    let root_only = format!("TN:\nSF:{}\nDA:1,1\nend_of_record\n", checkout.display());
    assert_eq!(
        read_lcov(&root, root_only.as_bytes()),
        Err(CoverageReadRefusal::EmptyRelativeSource { record: 2 })
    );
    assert_eq!(read_lcov(&root, &[0xff]), Err(CoverageReadRefusal::NonUtf8));
    Ok(())
}

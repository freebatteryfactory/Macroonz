use super::configure::{archives, finish, request};
use super::refusal::subject;
use crate::compiler::{
    configure::{bounds, host, root},
    refusal::standin,
};
use macroonz::harness::muterprater::{
    MutationSourceRevision, backend_archive::BackendSourceRefusal,
};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_mutation::{self, MutationError, MutationSources};

#[test]
fn historical_comparison_requires_the_complete_independent_roster() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "historical-roster",
        &subject(
            "std::io::stdout().write_all(b\"Found 1 mutants\\nok Unmutated baseline\\ncaught fixture.rs:1:1: replace value with 0\\n\")?;",
        ),
    )?;
    std::fs::write(root.join("fixture.rs"), b"original").map_err(|error| error.to_string())?;
    let output = finish(
        native_mutation::run(
            request(&host, &root, &backend, bounds()?, "output")?,
            |_| None,
            |_, _| None,
        )
        .map_err(|error| error.to_string())?,
    )?;
    let archive = output
        .retain(archives())
        .map_err(|error| error.to_string())?;
    let source = MutationSourceRevision::from_content("fixture.rs", b"original")
        .map_err(|error| format!("{error:?}"))?;
    let extra = MutationSourceRevision::from_content("extra.rs", b"extra")
        .map_err(|error| format!("{error:?}"))?;
    assert!(
        matches!(archive.compared_sources(vec![source.clone(), source.clone()]), Err(BackendSourceRefusal::Duplicate(file)) if file == "fixture.rs")
    );
    assert!(
        matches!(archive.compared_sources(Vec::new()), Err(BackendSourceRefusal::Missing(file)) if file == "fixture.rs")
    );
    assert!(
        matches!(archive.compared_sources(vec![source, extra]), Err(BackendSourceRefusal::Unexpected(file)) if file == "extra.rs")
    );
    assert!(
        output
            .retain(
                macroonz::harness::muterprater::backend_archive::BackendArchiveLimits::declared(
                    archives().run(),
                    0,
                    0,
                    0
                )
            )
            .is_err()
    );
    assert!(native_mutation::compare_historical(&archive, &root, 1).is_err());
    Ok(())
}

#[test]
fn literal_source_roster_and_regular_files_are_required() -> Result<(), String> {
    for spelling in ["*.rs", "[a].rs", "a?.rs", "a{b}.rs", "a\0.rs"] {
        let file = RelativeSourcePath::informed(spelling).map_err(|error| format!("{error:?}"))?;
        assert!(MutationSources::declared(vec![file], 100).is_err());
    }
    let file = RelativeSourcePath::informed("fixture.rs").map_err(|error| format!("{error:?}"))?;
    assert!(MutationSources::declared(vec![file.clone(), file.clone()], 100).is_err());
    assert!(MutationSources::declared(vec![file], 0).is_err());
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "capture-backend",
        &subject("std::fs::write(\"started\", b\"yes\")?;"),
    )?;
    std::fs::create_dir(root.join("fixture.rs")).map_err(|error| error.to_string())?;
    assert!(matches!(
        native_mutation::run(
            request(&host, &root, &backend, bounds()?, "output")?,
            |_| None,
            |_, _| None
        ),
        Err(MutationError::Filesystem(_))
    ));
    assert!(!root.join("started").exists());
    assert!(!root.join("output").exists());
    Ok(())
}

use macroonz_harness::descriptor::NamespacedName;
use macroonz_harness::fuzz::{
    CoveragePoint, CoverageReadRefusal, CoverageRootsRefusal, CoverageSourceRoot,
    CoverageSourceRoots, read_lcov_mapped,
};
use std::path::Path;

fn root(name: &'static str, path: &Path) -> Result<CoverageSourceRoot, String> {
    CoverageSourceRoot::declared(
        NamespacedName::named("mapped-test", name).map_err(|error| format!("{error:?}"))?,
        path.to_path_buf(),
    )
    .map_err(|error| format!("{error:?}"))
}

#[test]
fn root_admission_refuses_empty_duplicate_and_nested_mappings() -> Result<(), String> {
    let directory = std::env::temp_dir().join("coverage-mapped");
    let first = root("application", &directory)?;
    assert_eq!(
        CoverageSourceRoots::declared(Vec::new()),
        Err(CoverageRootsRefusal::Empty)
    );
    let duplicate = root("application", &directory.with_file_name("other"))?;
    assert!(matches!(
        CoverageSourceRoots::declared(vec![first.clone(), duplicate]),
        Err(CoverageRootsRefusal::DuplicateName(_))
    ));
    for path in [directory.clone(), directory.join("nested")] {
        let second = root("dependency", &path)?;
        for roots in [
            vec![first.clone(), second.clone()],
            vec![second, first.clone()],
        ] {
            assert_eq!(
                CoverageSourceRoots::declared(roots),
                Err(CoverageRootsRefusal::OverlappingPaths { left: 0, right: 1 })
            );
        }
    }
    Ok(())
}

#[test]
fn multiple_roots_keep_distinct_logical_identity_after_independent_relocation() -> Result<(), String>
{
    let base = std::env::temp_dir().join("coverage-mapped");
    let mut previous = None;
    for (application, dependency) in [("first", "registry"), ("moved", "cache")] {
        let application = base.join(application);
        let dependency = base.join(dependency);
        let roots = CoverageSourceRoots::declared(vec![
            root("application", &application)?,
            root("dependency", &dependency)?,
        ])
        .map_err(|error| format!("{error:?}"))?;
        let document = format!(
            "SF:{}\nDA:7,1\nend_of_record\nSF:{}\nDA:7,1\nend_of_record\n",
            application.join("src/lib.rs").display(),
            dependency.join("src/lib.rs").display()
        );
        let observation =
            read_lcov_mapped(&roots, document.as_bytes()).map_err(|error| format!("{error:?}"))?;
        let [
            CoveragePoint::Line {
                source: first,
                line: 7,
            },
            CoveragePoint::Line {
                source: second,
                line: 7,
            },
        ] = observation.points()
        else {
            return Err(format!("unexpected points: {observation:?}"));
        };
        assert_eq!(first.relative(), "src/lib.rs");
        assert_eq!(second.relative(), "src/lib.rs");
        assert_ne!(first.root(), second.root());
        if let Some(previous) = previous {
            assert_eq!(observation, previous);
        }
        previous = Some(observation);
        let outside = format!(
            "SF:{}\nDA:1,0\nend_of_record\n",
            base.join("unrelated.rs").display()
        );
        assert_eq!(
            read_lcov_mapped(&roots, outside.as_bytes()),
            Err(CoverageReadRefusal::SourceOutsideRoot { record: 1 })
        );
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn normalized_windows_aliases_cannot_choose_a_logical_root_arbitrarily() -> Result<(), String> {
    let ordinary = std::path::PathBuf::from(r"C:\coverage-mapped");
    let verbatim = std::path::PathBuf::from(r"\\?\C:\coverage-mapped");
    let roots = CoverageSourceRoots::declared(vec![
        root("application", &ordinary)?,
        root("dependency", &verbatim)?,
    ])
    .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        read_lcov_mapped(&roots, b"SF:C:\\coverage-mapped\\src\\lib.rs\nDA:1,1\n"),
        Err(CoverageReadRefusal::AmbiguousSource { record: 1 })
    );
    Ok(())
}

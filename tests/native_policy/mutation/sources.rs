use crate::compiler::configure::{bounds, host, root, standin, tool};
use crate::presentation_formats::{field, parsed};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_mutation::{
    self, MutationError, MutationObservationError, MutationRequest, MutationSources,
    MutationToolchain,
};

#[test]
fn capture_enforces_aggregate_bounds_and_compares_unreported_declared_files() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "source-backend",
        &super::refusal::subject(
            "std::fs::write(\"extra.rs\", b\"changed\")?; std::io::stderr().write_all(b\"Found 1 mutants\\nok Unmutated baseline\\ncaught fixture.rs:1:1: replace value with 0\\n\")?;",
        ),
    )?;
    std::fs::write(root.join("fixture.rs"), b"original").map_err(|error| error.to_string())?;
    std::fs::write(root.join("extra.rs"), b"extra").map_err(|error| error.to_string())?;
    let selected = tool(&host, &backend, &root, bounds()?)?;
    for (name, bound) in [("too-small", 8usize), ("whole-input", 128usize)] {
        let sources = ["fixture.rs", "extra.rs"]
            .into_iter()
            .map(RelativeSourcePath::informed)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("{error:?}"))?;
        let request = MutationRequest::cargo_mutants(
            &selected,
            MutationToolchain::declared(host.cargo.clone(), host.rustc.clone())
                .map_err(|error| error.to_string())?,
            MutationSources::declared(sources, bound).map_err(|error| error.to_string())?,
            root.join(name),
            &root.join("mutation-target"),
            &host.triple,
        )
        .map_err(|error| error.to_string())?;
        let result = native_mutation::run(request, |_| None, |_, _| None);
        if name == "too-small" {
            let shown = parsed(&macroonz::presentation::mutation_error(
                result.as_ref().err().ok_or("source bound admitted")?,
            ))?;
            assert_eq!(
                field(&shown, "/record/cause")?,
                &serde_json::json!({"kind":"source-bound", "value":8usize})
            );
            assert!(matches!(
                result,
                Err(MutationError::SourceBound { bound: 8 })
            ));
            assert!(!root.join(name).exists());
            assert_eq!(
                std::fs::read(root.join("extra.rs")).map_err(|error| error.to_string())?,
                b"extra"
            );
        } else {
            let output = super::configure::finish(result.map_err(|error| error.to_string())?)?;
            super::presentation::observation_failure(&output, "sources")?;
            assert!(
                matches!(output.manifest(), Err(MutationObservationError::Sources(cause)) if cause.contains("extra.rs"))
            );
            assert_eq!(
                output.original_sources().collect::<Vec<_>>(),
                [
                    ("fixture.rs", b"original".as_slice()),
                    ("extra.rs", b"extra".as_slice())
                ]
            );
        }
    }
    Ok(())
}

#[test]
fn declared_source_link_escape_refuses_without_running_the_backend() -> Result<(), String> {
    let root = root()?;
    let outside = crate::compiler::configure::root()?;
    std::fs::write(outside.join("source.rs"), b"outside").map_err(|error| error.to_string())?;
    let link = root.join("redirect");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).map_err(|error| error.to_string())?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&outside, &link).map_err(|error| error.to_string())?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "escape-backend",
        &super::refusal::subject("std::fs::write(\"started\", b\"yes\")?;"),
    )?;
    let selected = tool(&host, &backend, &root, bounds()?)?;
    let sources = MutationSources::declared(
        vec![
            RelativeSourcePath::informed("redirect/source.rs")
                .map_err(|error| format!("{error:?}"))?,
        ],
        1024,
    )
    .map_err(|error| error.to_string())?;
    let request = MutationRequest::cargo_mutants(
        &selected,
        MutationToolchain::declared(host.cargo.clone(), host.rustc.clone())
            .map_err(|error| error.to_string())?,
        sources,
        root.join("output"),
        &root.join("mutation-target"),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let result = native_mutation::run(request, |_| None, |_, _| None);
    let shown = parsed(&macroonz::presentation::mutation_error(
        result.as_ref().err().ok_or("source escape admitted")?,
    ))?;
    assert_eq!(field(&shown, "/record/cause/kind")?, "filesystem");
    assert!(field(&shown, "/record/cause/value/detail")?.is_string());
    assert!(matches!(result, Err(MutationError::Filesystem(_))));
    assert!(!root.join("started").exists());
    assert!(!root.join("output").exists());
    assert_eq!(
        std::fs::read(outside.join("source.rs")).map_err(|error| error.to_string())?,
        b"outside"
    );
    #[cfg(windows)]
    std::fs::remove_dir(link).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    std::fs::remove_file(link).map_err(|error| error.to_string())?;
    Ok(())
}

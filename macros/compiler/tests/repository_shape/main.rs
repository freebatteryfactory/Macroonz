//! The compiler package's named shape debt, held at its exact current denominator until each owner closes it.

use std::fs;
use std::path::{Path, PathBuf};

fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn occurrence_paths(root: &Path, needle: &str) -> Result<Vec<String>, std::io::Error> {
    let mut observed = Vec::new();
    for path in rust_sources(root)? {
        let source = fs::read_to_string(&path)?;
        let relative = path
            .strip_prefix(root)
            .map_err(|error| std::io::Error::other(error.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        for _occurrence in source.match_indices(needle) {
            observed.push(relative.clone());
        }
    }
    observed.sort();
    Ok(observed)
}

fn assert_occurrences(root: &Path, needle: &str, expected: &[&str]) -> Result<(), std::io::Error> {
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(occurrence_paths(root, needle)?, expected, "{needle}");
    Ok(())
}

#[test]
fn named_compiler_shape_debt_does_not_expand() -> Result<(), std::io::Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert_occurrences(
        &root,
        "PATH_SEGMENT_LIMIT: usize = 8;",
        &[
            "codec/types.rs",
            "descriptor/types.rs",
            "stamp/types.rs",
            "support/types.rs",
        ],
    )?;
    assert_occurrences(
        &root,
        "fn absolute_path(",
        &["descriptor/emitting.rs", "token/generation/compose.rs"],
    )?;
    assert_occurrences(
        &root,
        "fn harness_path(",
        &[
            "descriptor/concurrency/render.rs",
            "descriptor/network/render.rs",
            "recipe/render_evidence.rs",
        ],
    )?;
    assert_occurrences(
        &root,
        "recipe issue category must be formatted exactly once",
        &["recipe/type_contract.rs", "recipe/type_contract.rs"],
    )
}

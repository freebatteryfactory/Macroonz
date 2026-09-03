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
        "RENDERED_PATH_SEGMENT_LIMIT: usize = 8;",
        &["token/types.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const PATH_SEGMENT_LIMIT: usize = crate::token::RENDERED_PATH_SEGMENT_LIMIT;",
        &["descriptor/types.rs", "stamp/types.rs", "support/types.rs"],
    )?;
    assert_occurrences(
        &root,
        "CODEC_PATH_SEGMENT_LIMIT: usize = crate::token::RENDERED_PATH_SEGMENT_LIMIT;",
        &["codec/types.rs"],
    )?;
    assert_occurrences(&root, "fn absolute_path(", &["token/generation/compose.rs"])?;
    assert_occurrences(
        &root,
        "fn harness_path(",
        &[
            "descriptor/concurrency/render.rs",
            "descriptor/network/render.rs",
            "recipe/render_evidence.rs",
        ],
    )?;
    assert_occurrences(&root, "const FAULT_ARMS:", &[])?;
    assert_occurrences(
        &root,
        "const NAME_REFUSAL: (&str, &[&str], &str)",
        &["descriptor/fault.rs"],
    )?;
    assert_occurrences(&root, "macro_rules! subjects", &["identity/stamp.rs"])?;
    assert_occurrences(&root, "const RUST_KEYWORDS: &[&str]", &["token/bank.rs"])?;
    assert_occurrences(
        &root,
        "const RAW_IDENTIFIER_EXCLUSIONS: &[&str]",
        &["token/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub fn rust_keyword(spelling: &str) -> bool",
        &["token/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const CAPTURED_DECLARATION_PROFILE: Profile",
        &["identity/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const MEMBER_CONTRACT: [MemberContract; 5]",
        &["codec/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const RESERVED_BINDINGS: [&str; 12]",
        &["codec/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "recipe issue category must be formatted exactly once",
        &["recipe/type_contract.rs", "recipe/type_contract.rs"],
    )
}

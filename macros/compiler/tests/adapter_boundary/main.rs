//! The first-party descriptor adapter's source boundary, observed over the complete compiler source tree.
//!
//! The adapter may know destination vocabulary; the neutral homes may call no adapter road and may carry no physical package spelling or target switch from it.

use std::fs;
use std::path::{Path, PathBuf};

/// Every Rust source file beneath one directory, derived from the tree rather than mirrored in a roster.
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

/// No neutral compiler home reaches backward into the adapter or carries its destination spellings.
#[test]
fn neutral_homes_do_not_acquire_adapter_vocabulary() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let adapter_root = source_root.join("descriptor");
    let composition_root = source_root.join("lib.rs");
    let vocabulary = [
        "crate::descriptor",
        "macroonz_harness",
        "macroonz-harness",
        "declared-order-permutation",
        "cfg(loom)",
    ];

    for path in rust_sources(&source_root)? {
        if path.starts_with(&adapter_root) || path == composition_root {
            continue;
        }
        let source = fs::read_to_string(&path)?;
        for spelling in vocabulary {
            assert!(
                !source.contains(spelling),
                "{} carries adapter vocabulary {spelling}",
                path.display()
            );
        }
    }
    Ok(())
}

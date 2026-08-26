//! Canonical concurrency content distinguishes authored row order while ignoring clause presentation order.

use macroonz_compiler::descriptor::{Grammar, concurrency};
use macroonz_compiler::{CanonicalContent, TextCapture};

const AUTHORED: &str = r#"
    harness = mh,
    module = generated,
    namespace = "concurrency-projection",
    first { population = "first", interleavings = 2, samples = 3, seed = 5 },
    second { population = "second", interleavings = 7, samples = 11, seed = 13 },
"#;

const CLAUSES_REORDERED: &str = r#"
    namespace = "concurrency-projection",
    first { seed = 5, samples = 3, population = "first", interleavings = 2 },
    harness = mh,
    second { samples = 11, seed = 13, interleavings = 7, population = "second" },
    module = generated,
"#;

const ROWS_REVERSED: &str = r#"
    harness = mh,
    module = generated,
    namespace = "concurrency-projection",
    second { population = "second", interleavings = 7, samples = 11, seed = 13 },
    first { population = "first", interleavings = 2, samples = 3, seed = 5 },
"#;

/// The canonical bytes one declaration publishes, or nothing where the fixture did not capture or declare.
fn content(source: &str) -> Option<Vec<u8>> {
    let captured = TextCapture::read(source).ok()?;
    let declaration = concurrency::declared(
        captured.input(),
        Grammar {
            attribute: "concurrency",
        },
    )
    .ok()?;
    let mut bytes = Vec::new();
    declaration.encode_content_into(&mut bytes);
    Some(bytes)
}

/// Clause order is presentation, while authored row order is canonical projection meaning.
#[test]
fn canonical_content_reads_semantic_order_once() -> Result<(), ()> {
    let authored = content(AUTHORED).ok_or(())?;
    assert_eq!(content(CLAUSES_REORDERED).ok_or(())?, authored);
    assert_ne!(content(ROWS_REVERSED).ok_or(())?, authored);
    Ok(())
}

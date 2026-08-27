//! Real compiler-token capture compared with text capture at their shared normalized boundary.
//!
//! The witness is an isolated non-published qualification workspace, so this observation adds no `macroonz-macros` entry, workspace member, or product dependency.
//! The shared case carries only distinctions both producers can author truthfully; comment lowering and invisible groups keep their separate disposition observers.

use core::convert::Infallible;
use macroonz_compiler::{
    CaptureBuildRefusal, CaptureBuilder, CapturedAtom, CapturedDelimiter, TextCapture,
};

/// The source spelling corresponding exactly to the token stream given to the proc witness below.
const SOURCE: &str = r##"r#type::Item<'a, 'r#kind> && "a\nb" r#"raw"# b"\xff" c"x" { [1..=3] }"##;

/// Source text carrying the two comment dispositions shared with a proc macro.
const COMMENT_SOURCE: &str = "alpha // ordinary\n/// docs\nbeta";

/// Adjacent punctuation that is not limited to Rust's currently recognized multi-character operators.
const ADJACENT_PUNCTUATION_SOURCE: &str = "++ ?# ,, <- + + +/* gap */+";

/// Forward one parsed expression so the compiler inserts the invisible group a text producer cannot author.
macro_rules! capture_expression {
    ($expression:expr) => {
        macroonz_capture_observer::canonical_capture!($expression)
    };
}

/// Lowercase hexadecimal written independently of the proc witness.
fn hexadecimal(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        text.push(hex_digit(byte >> 4));
        text.push(hex_digit(byte & 0x0f));
    }
    text
}

/// One lowercase hexadecimal digit for a masked nibble.
const fn hex_digit(nibble: u8) -> char {
    match nibble {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}

/// A real proc-macro token stream and source text reach one canonical normalized input.
#[test]
fn proc_tokens_and_text_have_one_normalized_identity() -> Result<(), ()> {
    let proc_bytes = macroonz_capture_observer::canonical_capture!(
        r#type::Item<'a, 'r#kind> && "a\nb" r#"raw"# b"\xff" c"x" { [1..=3] }
    );
    let text = TextCapture::read(SOURCE).map_err(|_| ())?;
    assert_eq!(proc_bytes, hexadecimal(&text.input().canonical_bytes()));
    Ok(())
}

/// Ordinary comments disappear and doc comments lower to the same attribute-shaped normalized input on both roads.
#[test]
fn proc_tokens_and_text_share_comment_dispositions() -> Result<(), ()> {
    let proc_bytes = macroonz_capture_observer::canonical_capture!(
        alpha // ordinary
        /// docs
        beta
    );
    let text = TextCapture::read(COMMENT_SOURCE).map_err(|_| ())?;
    assert_eq!(proc_bytes, hexadecimal(&text.input().canonical_bytes()));
    Ok(())
}

/// Every immediately adjacent punctuation pair shares the compiler's spacing disposition on both roads.
#[test]
fn proc_tokens_and_text_share_nonoperator_punctuation_spacing() -> Result<(), ()> {
    let proc_bytes = macroonz_capture_observer::canonical_capture!(++ ?# ,, <- + + +/* gap */+);
    let text = TextCapture::read(ADJACENT_PUNCTUATION_SOURCE).map_err(|_| ())?;
    assert_eq!(proc_bytes, hexadecimal(&text.input().canonical_bytes()));
    Ok(())
}

/// A compiler-inserted invisible group remains a real captured group and is not equated with source text that cannot spell it.
#[test]
fn proc_invisible_group_stays_distinct_from_text() -> Result<(), ()> {
    let proc_bytes = capture_expression!(1 + 2);

    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level
        .group(0u64, CapturedDelimiter::Bare, |_span, inner| {
            let inner = inner.atom(1u64, |_| {
                Ok::<_, Infallible>(CapturedAtom::Number(String::from("1")))
            })?;
            let inner = inner.atom(2u64, |_| Ok::<_, Infallible>(CapturedAtom::Punct('+')))?;
            inner.atom(3u64, |_| {
                Ok::<_, Infallible>(CapturedAtom::Number(String::from("2")))
            })
        })
        .map_err(|_refusal: CaptureBuildRefusal<u64, Infallible>| ())?;
    let expected = level.finish();
    let text = TextCapture::read("1 + 2").map_err(|_| ())?;

    assert_eq!(proc_bytes, hexadecimal(&expected.canonical_bytes()));
    assert_ne!(proc_bytes, hexadecimal(&text.input().canonical_bytes()));
    Ok(())
}

//! Authored-item diagnostics retain independently specified family names, refusal wording, and source-site attribution.

use core::error::Error;
use macroonz_compiler::{AuthoredItemKind, AuthoredItemReadIssue, TextCapture};

/// Public structural families render as their documented Rust item names.
#[test]
fn structural_family_names_are_not_debug_variant_spellings() {
    for (kind, expected) in [
        (AuthoredItemKind::Module, "module"),
        (AuthoredItemKind::Structure, "structure"),
        (AuthoredItemKind::Enumeration, "enumeration"),
        (AuthoredItemKind::Union, "union"),
        (AuthoredItemKind::Trait, "trait"),
        (AuthoredItemKind::Function, "function"),
        (AuthoredItemKind::Implementation, "implementation"),
        (AuthoredItemKind::TypeAlias, "type alias"),
        (AuthoredItemKind::Constant, "constant"),
        (AuthoredItemKind::Static, "static"),
        (AuthoredItemKind::Use, "use item"),
        (AuthoredItemKind::ExternalCrate, "external-crate item"),
    ] {
        assert_eq!(kind.to_string(), expected);
    }
}

/// Malformed source supplies each reachable refusal and distinguishes absent from available source positions.
#[test]
fn item_refusals_preserve_their_sentence_and_source_boundary() -> Result<(), ()> {
    for (source, expected) in [
        (
            "",
            "the declared item boundary carries no token at the declaration boundary",
        ),
        (
            "pub mystery;",
            "the declared item boundary carries no supported structural item-family keyword at captured span 1",
        ),
        (
            "pub struct ;",
            "the declared structure carries no identifier in its name seat at captured span 2",
        ),
        (
            "pub fn read()",
            "the declared function boundary ends without a braced body or semicolon at captured span 3",
        ),
    ] {
        let capture = TextCapture::read(source).map_err(|_| ())?;
        let refusal = capture.input().authored_item().err().ok_or(())?;
        assert_eq!(refusal.to_string(), expected);
        assert!(refusal.source().is_none());
    }
    // The issue is public; this observes its rendering without manufacturing a private refused lens.
    assert_eq!(
        AuthoredItemReadIssue::LensRangeContradiction.to_string(),
        "an authored-item structural coordinate does not belong to its captured boundary"
    );
    Ok(())
}

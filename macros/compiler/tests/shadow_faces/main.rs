//! The shadow road, exercised from outside: a choice of roster names in, both faces of each out, and every malformed choice refused at its own token.
//!
//! The positive lanes establish that the rendering writes exactly the two `cfg`-gated faces per chosen name in authored order; the refusal lanes reverse one clause each of what the grammar promises; the roster lane holds the stated table to its own shape.

use macroonz::descriptor::Grammar;
use macroonz::descriptor::door;
use macroonz::descriptor::shadow::{SHADOW_ROSTER, ShadowFace};
use macroonz::{CrateBinding, Diagnostic, Door, Expansion, Phase, Producer, TextCapture};
use std::collections::BTreeSet;

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.shadow.grammar",
    "lane::shadow",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "shadow",
    },
);

/// The shadow grammar this lane registers.
const SHADOW: Grammar = Grammar {
    attribute: "shadow",
};

/// The shadow road walked over one source, or nothing where the lane's own source did not capture.
fn shadowed(source: &str) -> Option<Result<Expansion<ShadowFace>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::shadow(read.input().clone(), SHADOW, &DOOR))
}

/// The declaration-site text one shadow expansion emits.
fn emitted(expansion: &Expansion<ShadowFace>) -> Option<String> {
    expansion
        .emit()
        .tokens()
        .map(macroonz::GeneratedTree::inspected)
}

/// A lawful choice becomes both faces of each chosen name, in authored order, and nothing else.
#[test]
fn a_choice_becomes_both_faces_of_each_name() -> Result<(), ()> {
    let expansion = shadowed("Arc, Mutex").ok_or(())?.ok().ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    assert_eq!(text.matches("pub use").count(), 4usize);
    assert_eq!(text.matches("cfg").count(), 4usize);
    assert_eq!(text.matches("not").count(), 2usize);
    assert_eq!(text.matches("Arc").count(), 2usize);
    assert_eq!(text.matches("Mutex").count(), 2usize);
    assert_eq!(text.matches("std").count(), 2usize);
    assert_eq!(text.matches("loom").count(), 6usize);
    Ok(())
}

/// A trailing comma is lawful, and a lone module name renders like any other row.
#[test]
fn a_trailing_comma_is_lawful() -> Result<(), ()> {
    let expansion = shadowed("thread,").ok_or(())?.ok().ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    assert_eq!(text.matches("pub use").count(), 2usize);
    assert_eq!(text.matches("thread").count(), 2usize);
    Ok(())
}

/// A name outside the roster, a doubled name, a choice that is not a name, and an empty declaration each refuse at capture.
#[test]
fn a_malformed_choice_refuses_at_capture() -> Result<(), ()> {
    for source in ["Telepathy", "Arc, Arc", "5", "Arc Mutex", ""] {
        let refusal = shadowed(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
    }
    Ok(())
}

/// The stated roster holds its own shape: distinct names, and every row's two paths rooted where the row claims.
#[test]
fn the_roster_is_distinct_and_rooted() {
    let mut seen = BTreeSet::new();
    for row in SHADOW_ROSTER {
        assert!(seen.insert(row.name()), "{} is stated twice", row.name());
        assert_eq!(row.std_path().first(), Some(&"std"));
        assert_eq!(row.loom_path().first(), Some(&"loom"));
        assert_eq!(
            row.std_path().last(),
            row.loom_path().last(),
            "{} does not end on one name",
            row.name()
        );
    }
}

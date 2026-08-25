//! Content, capture, and owner movement observed through the identities one ordinary request mints.
//!
//! The controls are paired: repeating every semantic fact produces the same identities, while moving content, captured material, or the producer moves every downstream identity that reads it.
//! Source coordinates are deliberately absent from the semantic chain, so moving only a producer-held position changes none of them.

use core::convert::Infallible;
use core::fmt;
use macroonz::{
    CanonicalContent, CaptureBuilder, CapturedAtom, CapturedInput, CrateBinding, Door, Expansion,
    GeneratedToken, GeneratedTree, Kind, NoQuestions, Producer, Request, SoleRole,
};

#[derive(Clone, PartialEq, Eq)]
struct Content(u8);

impl fmt::Debug for Content {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.write_str("opaque-content")
    }
}

impl CanonicalContent for Content {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.push(self.0);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Projection;

impl Kind for Projection {
    const NAME: &'static str = "lane.projection";
    type Content = Content;
    type Role = SoleRole;
    type Question = NoQuestions;
}

const FIRST_DOOR: Door = Door::declared(
    "lane",
    "lane.projection.grammar",
    "lane::projection",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "first-producer",
    },
);

const OTHER_PRODUCER: Door = Door::declared(
    "lane",
    "lane.projection.grammar",
    "lane::projection",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "other-producer",
    },
);

fn rendered(
    capture: CapturedInput,
    content: Content,
    door: &Door,
) -> Option<Expansion<Projection>> {
    Request::<Projection>::over(capture, content, door)
        .render(|_plan, out| {
            out.unit(
                SoleRole::Sole,
                GeneratedTree::assembled(vec![GeneratedToken::word("unit")])?,
            )
        })
        .ok()
}

fn captured(word: &str, position: u64) -> Option<CapturedInput> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level
        .atom(position, |_| {
            Ok::<_, Infallible>(CapturedAtom::Word(word.to_owned()))
        })
        .ok()?;
    Some(level.finish())
}

fn key(expansion: &Expansion<Projection>) -> Option<[u8; 32]> {
    expansion
        .closure()
        .rendered()
        .under(SoleRole::Sole)
        .map(|unit| *unit.semantic_key().as_bytes())
}

#[test]
fn one_binding_repeated_mints_one_identity_chain() -> Result<(), ()> {
    let first = rendered(captured("same", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    let repeated = rendered(captured("same", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    assert_eq!(first.plan().account(), repeated.plan().account());
    assert_eq!(first.plan().identity(), repeated.plan().identity());
    assert_eq!(key(&first), key(&repeated));
    Ok(())
}

#[test]
fn content_moves_the_content_intent_member_and_plan_without_debug_hashing() -> Result<(), ()> {
    let first = rendered(captured("same", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    let changed = rendered(captured("same", 10).ok_or(())?, Content(2), &FIRST_DOOR).ok_or(())?;
    assert_eq!(
        format!("{:?}", first.plan().content()),
        format!("{:?}", changed.plan().content())
    );
    assert_eq!(
        first.plan().account().commitment(),
        changed.plan().account().commitment()
    );
    assert_ne!(
        first.plan().account().content_commitment(),
        changed.plan().account().content_commitment()
    );
    assert_ne!(first.plan().intent(), changed.plan().intent());
    assert_ne!(key(&first), key(&changed));
    assert_ne!(first.plan().identity(), changed.plan().identity());
    Ok(())
}

#[test]
fn captured_material_and_owner_each_move_the_bound_chain() -> Result<(), ()> {
    let first = rendered(captured("first", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    let changed_capture =
        rendered(captured("second", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    assert_ne!(
        first.plan().account().commitment(),
        changed_capture.plan().account().commitment()
    );
    assert_ne!(
        first.plan().account().content_commitment(),
        changed_capture.plan().account().content_commitment()
    );
    assert_ne!(first.plan().identity(), changed_capture.plan().identity());

    let changed_owner = rendered(
        captured("first", 10).ok_or(())?,
        Content(1),
        &OTHER_PRODUCER,
    )
    .ok_or(())?;
    assert_eq!(
        first.plan().account().commitment(),
        changed_owner.plan().account().commitment()
    );
    assert_ne!(
        first.plan().account().kind(),
        changed_owner.plan().account().kind()
    );
    assert_ne!(
        first.plan().account().content_commitment(),
        changed_owner.plan().account().content_commitment()
    );
    assert_ne!(first.plan().intent(), changed_owner.plan().intent());
    assert_ne!(key(&first), key(&changed_owner));
    assert_ne!(first.plan().identity(), changed_owner.plan().identity());
    Ok(())
}

#[test]
fn producer_positions_do_not_enter_the_semantic_chain() -> Result<(), ()> {
    let first = rendered(captured("same", 10).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    let moved = rendered(captured("same", 900).ok_or(())?, Content(1), &FIRST_DOOR).ok_or(())?;
    assert_eq!(first.plan().account(), moved.plan().account());
    assert_eq!(first.plan().identity(), moved.plan().identity());
    assert_eq!(key(&first), key(&moved));
    Ok(())
}

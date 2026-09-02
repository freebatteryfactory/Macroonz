//! Bounded deterministic adversaries against the public recipe grammar.

use macroonz_compiler::recipe::{HarnessPosture, bake};
use macroonz_compiler::{
    CaptureBuilder, CapturedAtom, CapturedDelimiter, CapturedInput, CrateBinding, Door, Producer,
    TextCapture,
};
use std::collections::BTreeSet;

const DOOR: Door = Door::declared(
    "recipe-grammar-adversary",
    "recipe-grammar-adversary.grammar",
    "recipe-grammar-adversary::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-grammar-adversary",
        name: "recipe",
    },
);

const AUTHORED_ONLY: &str =
    "pub mod specimen { pub struct Held; bake! { projections { companions; }; } }";

const TRANSITION: &str = r"
pub mod door {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        projections { companions; dispatch(apply); declaration_conformance; };
    }
}
";

const RELATION: &str = r"
pub mod policy {
    pub enum Stage { Draft, Published }
    pub enum Capability { Read, Write }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            allowed(Stage, Capability) {
                (Draft, Read) with(crate::policy::allow);
                (Published, Read) with(crate::policy::allow);
            };
        };
        postures { allowed { repetition(refused); membership(closed); }; };
        projections { companions; relation_tables { allowed; }; };
    }
}
";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Baked(Vec<u8>),
    Refused {
        phase: &'static str,
        observed: &'static str,
        summary: String,
        token: Option<u32>,
    },
    TextRefused(String),
}

fn observe(source: &str) -> Outcome {
    let capture = match TextCapture::read(source) {
        Ok(capture) => capture,
        Err(refusal) => return Outcome::TextRefused(refusal.to_string()),
    };
    match bake(capture.input(), HarnessPosture::Available, &DOOR) {
        Ok(baked) => Outcome::Baked(
            baked
                .emit()
                .tokens()
                .map_or_else(Vec::new, macroonz_compiler::GeneratedTree::canonical_bytes),
        ),
        Err(refusal) => Outcome::Refused {
            phase: refusal.phase().name(),
            observed: refusal.observed().name(),
            summary: refusal.summary().to_owned(),
            token: refusal
                .site()
                .token()
                .map(macroonz_compiler::SpanHandle::index),
        },
    }
}

fn corpus() -> BTreeSet<String> {
    let mut corpus = BTreeSet::new();
    let seeds = [AUTHORED_ONLY, TRANSITION, RELATION];
    let insertions = [
        "!",
        "=>",
        "->",
        "::",
        "#[]",
        "r#type",
        "\"hostile\"",
        "{[()]}",
        "bake!",
        "projections",
        "absence(allowed);",
    ];
    for seed in seeds {
        corpus.insert(seed.to_owned());
        for boundary in seed.char_indices().map(|(position, _character)| position) {
            if boundary % 7 == 0 {
                corpus.insert(seed[..boundary].to_owned());
            }
            if boundary % 29 == 0 {
                corpus.extend(
                    insertions
                        .iter()
                        .map(|insertion| inserted(seed, boundary, insertion)),
                );
            }
        }
    }
    corpus.insert(String::new());
    corpus.insert("pub mod r#type { bake! { projections { }; } }".to_owned());
    corpus.insert("pub mod nested { #[cfg(any())] pub struct Held([[[u8; 1]; 1]; 1]); bake! { projections { }; } }".to_owned());
    corpus.insert(TRANSITION.replace("=>", "= >"));
    corpus.insert(TRANSITION.replace("with(crate::open)", "with(crate : : open)"));
    corpus.insert(TRANSITION.replace("absence(refused);", "absence(refused); absence(allowed);"));
    corpus.insert(RELATION.replace("relations {", "relations { relations {"));
    corpus
}

fn inserted(seed: &str, boundary: usize, insertion: &str) -> String {
    let mut candidate = String::with_capacity(seed.len().saturating_add(insertion.len()));
    candidate.push_str(&seed[..boundary]);
    candidate.push_str(insertion);
    candidate.push_str(&seed[boundary..]);
    candidate
}

fn invisible_group_input() -> Result<CapturedInput, ()> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level
        .group(0_u64, CapturedDelimiter::Bare, |_outer_span, inner| {
            inner.group(1_u64, CapturedDelimiter::Brace, |_group_span, nested| {
                nested.atom(2_u64, |_word_span| {
                    Ok::<CapturedAtom, core::convert::Infallible>(CapturedAtom::Word(
                        "bake".to_owned(),
                    ))
                })
            })
        })
        .map_err(|_refusal| ())?;
    Ok(level.finish())
}

#[test]
fn bounded_adversarial_corpus_is_repeatable_and_total() {
    let corpus = corpus();
    assert!(corpus.len() >= 400);
    assert!(corpus.iter().all(|candidate| candidate.len() <= 2_048));
    let mut baked = 0_usize;
    let mut refused = 0_usize;
    for candidate in corpus {
        let first = std::panic::catch_unwind(|| observe(&candidate));
        let second = std::panic::catch_unwind(|| observe(&candidate));
        assert!(first.is_ok(), "recipe grammar panicked for {candidate:?}");
        assert!(
            second.is_ok(),
            "recipe grammar panicked on replay for {candidate:?}"
        );
        let first = first.unwrap_or_else(|_panic| unreachable!());
        let second = second.unwrap_or_else(|_panic| unreachable!());
        assert_eq!(first, second, "recipe outcome moved for {candidate:?}");
        match first {
            Outcome::Baked(_) => baked = baked.saturating_add(1),
            Outcome::Refused { .. } | Outcome::TextRefused(_) => {
                refused = refused.saturating_add(1);
            }
        }
    }
    assert!(baked >= 3);
    assert!(refused > baked);
}

#[test]
fn lawful_whitespace_normalization_preserves_generated_bytes() {
    let compact = observe(AUTHORED_ONLY);
    let spaced = observe(
        "  pub   mod specimen {\n pub struct Held ;\n bake ! { projections { companions ; } ; }\n }  ",
    );
    assert_eq!(compact, spaced);
    assert!(matches!(compact, Outcome::Baked(_)));
}

#[test]
fn invisible_and_nested_groups_refuse_repeatably_without_partial_output() -> Result<(), ()> {
    let capture = invisible_group_input()?;
    let first = bake(&capture, HarnessPosture::Available, &DOOR)
        .err()
        .ok_or(())?;
    let second = bake(&capture, HarnessPosture::Available, &DOOR)
        .err()
        .ok_or(())?;
    assert_eq!(first.phase(), second.phase());
    assert_eq!(first.observed(), second.observed());
    assert_eq!(first.summary(), second.summary());
    assert_eq!(first.site(), second.site());
    Ok(())
}

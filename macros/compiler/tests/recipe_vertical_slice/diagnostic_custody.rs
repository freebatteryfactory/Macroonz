//! Generic recipe refusals retain their narrowest authored token and declared precedence.

use super::support::{Occurrence, group_after_word, refusal, word_handle};
use macroonz_compiler::{CoordinateRole, Diagnostic, SiteCoordinate, SourceCoordinate, SpanHandle};

fn assert_at_handle(refusal: &Diagnostic, expected: SpanHandle) {
    assert!(refusal.related().carried().is_empty());
    assert_eq!(
        refusal.site().token(),
        Some(expected),
        "{}",
        refusal.summary()
    );
    assert_eq!(
        refusal.site().coordinate(),
        Some(SiteCoordinate::Resolved(SourceCoordinate {
            role: CoordinateRole::SemanticOrigin,
            position: u64::from(expected.index()),
        }))
    );
}

#[test]
fn foreign_relation_endpoints_point_at_the_exact_foreign_member() -> Result<(), ()> {
    let foreign_left = r"
pub mod policy {
    pub enum Stage { Draft }
    pub enum Capability { Read }
    bake! {
        vocabularies { Stage; Capability; };
        relations { policy(Stage, Capability) { (MissingLeft, Read); }; };
        projections { companions; };
    }
}
";
    let left = refusal(foreign_left)?;
    assert!(
        left.summary()
            .contains("undeclared `Stage` member `MissingLeft`")
    );
    assert_at_handle(
        &left,
        word_handle(foreign_left, "MissingLeft", Occurrence::First)?,
    );

    let foreign_right = foreign_left.replace("MissingLeft, Read", "Draft, MissingRight");
    let right = refusal(foreign_right.as_str())?;
    assert!(
        right
            .summary()
            .contains("undeclared `Capability` member `MissingRight`")
    );
    assert_at_handle(
        &right,
        word_handle(foreign_right.as_str(), "MissingRight", Occurrence::First)?,
    );
    Ok(())
}

#[test]
fn repeated_rows_and_mixed_payloads_point_at_the_first_disagreeing_material() -> Result<(), ()> {
    let duplicate = r"
pub mod policy {
    pub enum Stage { Draft }
    pub enum Capability { Read }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            policy(Stage, Capability) {
                (Draft, Read);
                (Draft, Read);
            };
        };
        postures { policy { repetition(refused); }; };
        projections { companions; };
    }
}
";
    let duplicate_refusal = refusal(duplicate)?;
    assert!(duplicate_refusal.summary().contains("more than once"));
    assert_at_handle(
        &duplicate_refusal,
        word_handle(duplicate, "Draft", Occurrence::Last)?,
    );

    let mixed = r"
pub mod policy {
    pub enum Stage { Draft, Published }
    pub enum Capability { Read, Write }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            policy(Stage, Capability) {
                (Draft, Read) with(crate::allow);
                (Published, Write) with { crate::Decision::Audit };
            };
        };
        projections { companions; };
    }
}
";
    let mixed_refusal = refusal(mixed)?;
    assert!(
        mixed_refusal
            .summary()
            .contains("mixes `path` and `exact-rust`")
    );
    assert_at_handle(&mixed_refusal, group_after_word(mixed, "with")?);
    Ok(())
}

#[test]
fn endpoint_and_repetition_refusals_precede_payload_and_posture_questions() -> Result<(), ()> {
    let foreign_before_payload = r"
pub mod policy {
    pub enum Stage { Draft }
    pub enum Capability { Read }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            policy(Stage, Capability) {
                (Draft, Missing) with(crate::allow);
                (Draft, Read) with { crate::Decision::Audit };
            };
        };
        projections { companions; };
    }
}
";
    let foreign = refusal(foreign_before_payload)?;
    assert!(
        foreign
            .summary()
            .contains("undeclared `Capability` member `Missing`")
    );
    assert_at_handle(
        &foreign,
        word_handle(foreign_before_payload, "Missing", Occurrence::First)?,
    );

    let duplicate_before_posture = r"
pub mod policy {
    pub enum Stage { Draft }
    pub enum Capability { Read }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            policy(Stage, Capability) {
                (Draft, Read);
                (Draft, Read);
            };
        };
        postures {
            policy {
                repetition(refused);
                density(dense);
            };
        };
        projections { companions; };
    }
}
";
    let duplicate = refusal(duplicate_before_posture)?;
    assert!(duplicate.summary().contains("more than once"));
    assert_at_handle(
        &duplicate,
        word_handle(duplicate_before_posture, "Draft", Occurrence::Last)?,
    );
    Ok(())
}

#[test]
fn repeated_account_names_point_at_the_repeated_declaration() -> Result<(), ()> {
    let duplicate_vocabulary = r"
pub mod duplicate_vocabulary {
    pub enum Stage { Draft }
    bake! {
        vocabularies { Stage; Stage; };
        projections { companions; };
    }
}
";
    let vocabulary = refusal(duplicate_vocabulary)?;
    assert!(
        vocabulary
            .summary()
            .contains("vocabulary `Stage` is declared more than once")
    );
    assert_at_handle(
        &vocabulary,
        word_handle(duplicate_vocabulary, "Stage", Occurrence::Last)?,
    );

    let duplicate_relation = r"
pub mod duplicate_relation {
    pub enum Stage { Draft }
    pub enum Capability { Read }
    bake! {
        vocabularies { Stage; Capability; };
        relations {
            policy(Stage, Capability) { (Draft, Read); };
            policy(Stage, Capability) { (Draft, Read); };
        };
        projections { companions; };
    }
}
";
    let relation = refusal(duplicate_relation)?;
    assert!(
        relation
            .summary()
            .contains("relation `policy` is declared more than once")
    );
    assert_at_handle(
        &relation,
        word_handle(duplicate_relation, "policy", Occurrence::Last)?,
    );

    let duplicate_codec = r"
pub mod duplicate_codec {
    pub struct Ledger { pub count: u16 }
    bake! {
        codecs {
            ledger(Ledger) {
                direction(encode);
                refusal(FirstDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
            ledger(Ledger) {
                direction(encode);
                refusal(SecondDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections { codec; };
    }
}
";
    let codec = refusal(duplicate_codec)?;
    assert!(
        codec
            .summary()
            .contains("codec `ledger` is declared more than once")
    );
    assert_at_handle(
        &codec,
        word_handle(duplicate_codec, "ledger", Occurrence::Last)?,
    );
    Ok(())
}

#[test]
fn a_repeated_posture_question_points_at_the_repeated_question() -> Result<(), ()> {
    let duplicate_question = r"
pub mod duplicate_question {
    pub enum Stage { Draft }
    bake! {
        vocabularies { Stage; };
        relations { evolution(Stage, Stage) { }; };
        postures {
            evolution {
                repetition(refused);
                repetition(allowed);
            };
        };
        projections { companions; };
    }
}
";
    let question = refusal(duplicate_question)?;
    assert!(
        question
            .summary()
            .contains("question `repetition` more than once")
    );
    assert_at_handle(
        &question,
        word_handle(duplicate_question, "repetition", Occurrence::Last)?,
    );
    Ok(())
}

#[test]
fn transition_absence_sugar_and_posture_refuse_a_second_answer() -> Result<(), ()> {
    let duplicate_absence = r"
pub mod workflow {
    pub enum State { Draft, Published }
    pub enum Event { Publish }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Draft, Publish) => Published with(crate::publish);
        };
        absence(refused);
        postures {
            transitions { absence(allowed); };
        };
        projections { companions; };
    }
}
";
    let refusal = refusal(duplicate_absence)?;
    assert!(
        refusal
            .summary()
            .contains("question `absence` more than once")
    );
    assert_at_handle(
        &refusal,
        word_handle(duplicate_absence, "absence", Occurrence::Last)?,
    );
    Ok(())
}

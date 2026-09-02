//! Structural collisions and malformed behavior refused before projection.

use super::{COMPLETE_RECIPE, DOOR, EVIDENCE_RECIPE, bake, refusal_summary};
use macroonz_compiler::TextCapture;
use macroonz_compiler::recipe::HarnessPosture;

#[test]
fn every_direct_type_namespace_collision_refuses_before_projection() -> Result<(), ()> {
    for declaration in [
        "mod baked {}",
        "struct baked;",
        "enum baked {}",
        "union baked { value: u8 }",
        "trait baked {}",
        "type baked = ();",
        "extern crate baked;",
        "struct r#baked;",
    ] {
        let source = format!(
            r"
pub mod door {{
    {declaration}

    pub enum State {{
        Closed,
        Open,
    }}

    pub enum Event {{
        OpenDoor,
    }}

    bake! {{
        vocabularies {{ State; Event; }};
        transitions(State, Event) {{
            (Closed, OpenDoor) => Open with(crate::effects::open);
        }};
        absence(refused);
        projections {{
            companions;
        }};
    }}
}}
"
        );
        let read = TextCapture::read(&source).map_err(|_| ())?;
        let refusal =
            macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
                .err()
                .ok_or(())?;
        assert!(refusal.summary().contains("generated recipe name `baked`"));
    }
    Ok(())
}

#[test]
fn value_and_macro_names_remain_outside_the_generated_module_collision_universe() -> Result<(), ()>
{
    for declaration in [
        "fn baked() {}",
        "const baked: u8 = 0;",
        "static baked: u8 = 0;",
        "macro_rules! baked { () => {}; }",
    ] {
        let source = format!(
            r"
pub mod door {{
    {declaration}

    pub enum Stage {{ Draft }}

    bake! {{
        vocabularies {{ Stage; }};
        projections {{ companions; }};
    }}
}}
"
        );
        assert!(
            bake(source.as_str()).is_ok(),
            "unexpected cross-namespace refusal for `{declaration}`: {:?}",
            refusal_summary(source.as_str())
        );
    }
    Ok(())
}

#[test]
fn every_recipe_owned_generated_name_collision_refuses_before_rendering() -> Result<(), ()> {
    let doubled_support = EVIDENCE_RECIPE.replace(
        "support = recipe_mutation_support",
        "support = recipe_trials_support",
    );
    assert!(
        refusal_summary(&doubled_support)?
            .contains("generated recipe name `recipe_trials_support` is already occupied")
    );

    let doubled_module =
        EVIDENCE_RECIPE.replace("module = recipe_network", "module = recipe_concurrency");
    assert!(
        refusal_summary(&doubled_module)?
            .contains("generated recipe name `recipe_concurrency` is already occupied")
    );

    let reserved_companion = COMPLETE_RECIPE.replace("dispatch(apply)", "dispatch(STATE_VARIANTS)");
    assert!(
        refusal_summary(&reserved_companion)?
            .contains("generated recipe name `STATE_VARIANTS` is already occupied")
    );
    Ok(())
}

#[test]
fn malformed_behavior_structure_refuses_before_projection() -> Result<(), ()> {
    let widened = COMPLETE_RECIPE.replace(
        "(Closed, OpenDoor) => Open with(crate::effects::open);",
        "(Missing, OpenDoor) => Open with(crate::effects::open);",
    );
    assert!(
        refusal_summary(&widened)?
            .contains("a relation row names undeclared `State` member `Missing`")
    );

    let doubled_row = COMPLETE_RECIPE.replace(
        "(Closed, OpenDoor) => Open with(crate::effects::open);",
        "(Closed, OpenDoor) => Open with(crate::effects::open);\n            (Closed, OpenDoor) => Open with(crate::effects::open);",
    );
    assert!(
        refusal_summary(&doubled_row)?
            .contains("more than one transition occupies state `Closed` and event `OpenDoor`")
    );

    let doubled_posture = COMPLETE_RECIPE.replace(
        "absence(refused);",
        "absence(refused);\n        absence(refused);",
    );
    assert!(refusal_summary(&doubled_posture)?.contains("recipe grammar was not read"));
    Ok(())
}

#[test]
fn collision_preflight_distinguishes_names_that_are_not_owned() -> Result<(), ()> {
    let external = COMPLETE_RECIPE.replacen(
        "    #[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "    extern crate other;\n\n    #[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        1,
    );
    assert!(
        bake(external.as_str()).is_ok(),
        "unexpected external-name refusal: {:?}",
        refusal_summary(external.as_str())
    );

    let same_roster = r"
pub mod workflow {
    pub enum Stage { Draft, Review, Published }
    bake! {
        vocabularies { Stage; };
        transitions(Stage, Stage) {
            (Draft, Review) => Published with(crate::publish);
        };
        absence(refused);
        projections { companions; };
    }
}
";
    assert!(
        bake(same_roster).is_ok(),
        "unexpected same-roster refusal: {:?}",
        refusal_summary(same_roster)
    );

    let payload_collision = r"
pub mod workflow {
    pub enum State { Draft, Published }
    pub enum Event { Publish }
    pub enum Capability { Read }
    bake! {
        vocabularies { State; Event; Capability; };
        transitions(State, Event) {
            (Draft, Publish) => Published with(crate::publish);
        };
        relations {
            policy(State, Capability) {
                (Draft, Read) with(crate::allow);
            };
        };
        absence(refused);
        projections {
            companions;
            dispatch(POLICY_PAYLOADS);
        };
    }
}
";
    assert!(
        refusal_summary(payload_collision)?
            .contains("generated recipe name `POLICY_PAYLOADS` is already occupied")
    );
    let unlabeled = payload_collision.replace(" with(crate::allow)", "");
    assert!(
        bake(unlabeled.as_str()).is_ok(),
        "unexpected unlabeled-relation refusal: {:?}",
        refusal_summary(unlabeled.as_str())
    );
    Ok(())
}

#[test]
fn codec_collision_preflight_requires_both_sides_of_each_conflict() -> Result<(), ()> {
    let selected_surfaces = r"
pub mod combined {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    pub struct Ledger { pub count: u16 }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        codecs {
            ledger(Ledger) {
                direction(decode);
                refusal(LedgerDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections { dispatch; typestate(State); codec; };
    }
}
";
    let _selected_surfaces = bake(selected_surfaces)?;
    let transition_name_without_dispatch = selected_surfaces
        .replace("refusal(LedgerDecodeError)", "refusal(TransitionRefusal)")
        .replace(
            "projections { dispatch; typestate(State); codec; }",
            "projections { codec; }",
        );
    let _transition_name_without_dispatch = bake(transition_name_without_dispatch.as_str())?;
    let typestate_name_without_typestate = selected_surfaces
        .replace("refusal(LedgerDecodeError)", "refusal(typestate)")
        .replace(
            "projections { dispatch; typestate(State); codec; }",
            "projections { codec; }",
        );
    let _typestate_name_without_typestate = bake(typestate_name_without_typestate.as_str())?;

    let codec_pair = r"
pub mod codecs {
    pub struct Ledger { pub count: u16 }
    pub struct Journal { pub count: u16 }
    bake! {
        codecs {
            ledger(Ledger) {
                direction(encode);
                refusal(SharedDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
            journal(Journal) {
                direction(decode);
                refusal(SharedDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections { codec; };
    }
}
";
    let _codec_pair = bake(codec_pair)?;
    let same_owner_split_roads = codec_pair.replace("journal(Journal)", "journal(Ledger)");
    let _same_owner_split_roads = bake(same_owner_split_roads.as_str())?;
    let different_owners_same_write_road = codec_pair.replace(
        "journal(Journal) {\n                direction(decode);\n                refusal(SharedDecodeError)",
        "journal(Journal) {\n                direction(encode);\n                refusal(JournalDecodeError)",
    );
    let _different_owners_same_write_road = bake(different_owners_same_write_road.as_str())?;
    let distinct_read_refusals = codec_pair
        .replace("direction(encode)", "direction(decode)")
        .replacen("SharedDecodeError", "LedgerDecodeError", 1)
        .replacen("SharedDecodeError", "JournalDecodeError", 1);
    let _distinct_read_refusals = bake(distinct_read_refusals.as_str())?;
    Ok(())
}

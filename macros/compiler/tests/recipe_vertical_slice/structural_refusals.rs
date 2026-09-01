//! Structural collisions and malformed behavior refused before projection.

use super::{COMPLETE_RECIPE, DOOR, EVIDENCE_RECIPE, refusal_summary};
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
        vocabularies(State, Event);
        transitions {{
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
            .contains("a transition names undeclared `State` member `Missing`")
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

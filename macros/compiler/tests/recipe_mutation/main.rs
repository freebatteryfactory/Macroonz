//! Independently compiled transition and effect pressure with explicit refusal controls.

#[path = "../support/mod.rs"]
mod support;

mod fixture;
mod consuming;

use fixture::{DOOR, RECIPE, WITNESS, captured, declaration, ordinary, source};
use macroonz_compiler::CanonicalContent;
use macroonz_compiler::descriptor::mutation::{
    EFFECT_BINDING_FAMILY, RecipeMutationError, TRANSITION_TARGET_FAMILY,
    completed_from_effect_bindings, completed_from_transition_targets,
};
use macroonz_compiler::recipe::{HarnessPosture, RecipeEdit, RecipeEditError, bake_edited};
use support::observe_rustc;

#[test]
fn every_target_substitution_executes_and_the_independent_target_witness_objects()
-> Result<(), String> {
    let read = captured(RECIPE)?;
    let declaration = declaration(TRANSITION_TARGET_FAMILY)?;
    let policy = declaration.policy().clone();
    let surface = completed_from_transition_targets(
        declaration,
        read.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    )
    .map_err(|refusal| refusal.to_string())?;
    assert_eq!(surface.policy(), &policy);
    let baseline = format!("{}\n{WITNESS}", source(surface.site().production())?);
    let compiled = observe_rustc("recipe_target_baseline", &baseline, &[])?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert_eq!(surface.site().alternatives().len(), 2);
    for (position, alternative) in surface.site().alternatives().iter().enumerate() {
        assert_eq!(alternative.family().slug(), TRANSITION_TARGET_FAMILY);
        assert_ne!(alternative.operation(), surface.site().unchanged());
        let changed = format!("{}\n{WITNESS}", source(alternative.meaning())?);
        let result = observe_rustc(&format!("recipe_target_changed_{position}"), &changed, &[]);
        assert!(
            matches!(result, Err(refusal) if refusal.contains("independent-transition-target"))
        );
    }
    let [first, second] = surface.site().alternatives() else {
        return Err("all other declared targets must be retained".to_owned());
    };
    assert_ne!(first.operation(), second.operation());
    Ok(())
}

#[test]
fn the_complete_donor_binding_executes_and_the_independent_effect_witness_objects()
-> Result<(), String> {
    let read = captured(RECIPE)?;
    let surface = completed_from_effect_bindings(
        declaration(EFFECT_BINDING_FAMILY)?,
        read.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    )
    .map_err(|refusal| refusal.to_string())?;
    let baseline = format!("{}\n{WITNESS}", source(surface.site().production())?);
    let compiled = observe_rustc("recipe_effect_baseline", &baseline, &[])?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let [alternative] = surface.site().alternatives() else {
        return Err("the other effect must supply its one complete binding".to_owned());
    };
    assert_eq!(alternative.family().slug(), EFFECT_BINDING_FAMILY);
    assert_ne!(alternative.operation(), surface.site().unchanged());
    let changed = format!("{}\n{WITNESS}", source(alternative.meaning())?);
    let result = observe_rustc("recipe_effect_changed", &changed, &[]);
    assert!(matches!(result, Err(refusal) if refusal.contains("independent-transition-effect")));
    Ok(())
}

#[test]
fn a_restatement_preserves_the_ordinary_account_and_emission() -> Result<(), String> {
    let baseline = ordinary(RECIPE)?;
    let read = captured(RECIPE)?;
    for edit in [
        RecipeEdit::TransitionTarget {
            row: 0,
            target: "Busy".to_owned(),
        },
        RecipeEdit::TransitionEffect {
            row: 0,
            from_row: 0,
        },
    ] {
        let restated = bake_edited(read.input(), HarnessPosture::Unavailable, &edit, &DOOR)
            .map_err(|refusal| refusal.to_string())?;
        assert_eq!(
            baseline
                .projection()
                .plan()
                .content()
                .canonical_content_bytes(),
            restated
                .projection()
                .plan()
                .content()
                .canonical_content_bytes(),
        );
        assert_eq!(
            baseline.projection().identity(),
            restated.projection().identity()
        );
        assert_eq!(baseline.emit().tokens(), restated.emit().tokens());
    }
    Ok(())
}

#[test]
fn absent_rows_and_targets_refuse_at_the_edit_owner() -> Result<(), String> {
    let read = captured(RECIPE)?;
    for edit in [
        RecipeEdit::TransitionTarget {
            row: 2,
            target: "Busy".to_owned(),
        },
        RecipeEdit::TransitionEffect {
            row: 2,
            from_row: 0,
        },
        RecipeEdit::TransitionEffect {
            row: 0,
            from_row: 2,
        },
    ] {
        assert!(matches!(
            bake_edited(read.input(), HarnessPosture::Unavailable, &edit, &DOOR),
            Err(RecipeEditError::RowAbsent { position: 2 }),
        ));
    }
    let absent = RecipeEdit::TransitionTarget {
        row: 0,
        target: "Missing".to_owned(),
    };
    assert!(matches!(
        bake_edited(read.input(), HarnessPosture::Unavailable, &absent, &DOOR),
        Err(RecipeEditError::TargetAbsent { spelling }) if spelling == "Missing",
    ));
    assert!(matches!(
        completed_from_effect_bindings(
            declaration(EFFECT_BINDING_FAMILY)?,
            read.input(),
            HarnessPosture::Unavailable,
            2,
            &DOOR,
        ),
        Err(RecipeMutationError::Edit(RecipeEditError::RowAbsent {
            position: 2
        })),
    ));
    Ok(())
}

#[test]
fn identical_effects_do_not_offer_a_second_structural_selection() -> Result<(), String> {
    let identical = RECIPE
        .replace("with(destination)", "with(result)")
        .replace("*log += 10", "*log += 1")
        .replace("Ok(destination)", "Ok(result)");
    let read = captured(&identical)?;
    assert!(matches!(
        completed_from_effect_bindings(
            declaration(EFFECT_BINDING_FAMILY)?,
            read.input(),
            HarnessPosture::Unavailable,
            0,
            &DOOR,
        ),
        Err(RecipeMutationError::NoAlternative),
    ));
    Ok(())
}

#[test]
fn a_nontransition_recipe_and_a_single_choice_refuse_pressure() -> Result<(), String> {
    let nontransition = r"
        pub mod machine {
            pub enum State { Idle }
            bake! {
                vocabularies { State; };
                relations { };
                projections { companions; };
            }
        }
    ";
    let read = captured(nontransition)?;
    assert!(ordinary(nontransition)?.emit().tokens().is_some());
    assert!(matches!(
        completed_from_transition_targets(
            declaration(TRANSITION_TARGET_FAMILY)?,
            read.input(),
            HarnessPosture::Unavailable,
            0,
            &DOOR,
        ),
        Err(RecipeMutationError::Edit(
            RecipeEditError::TransitionRequired
        )),
    ));
    let singleton = r"
        pub mod machine {
            pub enum State { Idle }
            pub enum Event { Stay }
            bake! {
                vocabularies { State; Event; };
                transitions(State, Event) { (Idle, Stay) => Idle with(crate::stay); };
                absence(refused);
                projections { companions; };
            }
        }
    ";
    let one = captured(singleton)?;
    assert!(matches!(
        completed_from_transition_targets(
            declaration(TRANSITION_TARGET_FAMILY)?,
            one.input(),
            HarnessPosture::Unavailable,
            0,
            &DOOR,
        ),
        Err(RecipeMutationError::NoAlternative),
    ));
    assert!(matches!(
        completed_from_effect_bindings(
            declaration(EFFECT_BINDING_FAMILY)?,
            one.input(),
            HarnessPosture::Unavailable,
            0,
            &DOOR,
        ),
        Err(RecipeMutationError::NoAlternative),
    ));
    Ok(())
}

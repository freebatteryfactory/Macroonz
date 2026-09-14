//! Outside admission and identity controls for consuming-target bindings.

use super::{canonical_content, captured, trees};
use macroonz_compiler::descriptor::{
    CaptureCause, CaptureIssue, DeclarationError, Grammar, Seat, trial,
};
use macroonz_compiler::{GeneratedToken, GeneratedTree, SpanHandle};

const BINDING: &str = r"
    subject_revision = { crate::subject_revision() },
    check_revision = { crate::check_revision() },
    call = { crate::independent_check },
";

fn declaration(binding: &str) -> String {
    format!(
        r#"
        support = bound_support,
        module = bound_table,
        table = named("adopter", "table"),
        suite all = named("adopter", "suite") {{
            first {{
                claim = named("adopter", "claim"),
                subject = named("adopter", "subject"),
                check = named("adopter", "check"),
                population = named("adopter", "population"),
                binding = {{ {binding} }},
            }},
        }},
    "#
    )
}

fn read(source: &str) -> Result<Result<trial::Trials, trial::TrialCaptureError>, ()> {
    let input = captured(source)?;
    Ok(trial::captured(
        &trees(&input),
        SpanHandle::at(0),
        Grammar {
            attribute: "trials",
        },
    ))
}

#[test]
fn missing_unknown_and_duplicate_binding_clauses_refuse_the_whole_declaration() -> Result<(), ()> {
    for (offered, cause) in [
        (
            BINDING.replace("call = { crate::independent_check },", ""),
            CaptureCause::ClauseAbsent,
        ),
        (
            format!("{BINDING} guessed = {{ false }},"),
            CaptureCause::ClauseUndeclared,
        ),
        (
            format!("{BINDING} call = {{ crate::another_check }},"),
            CaptureCause::ClauseDoubled,
        ),
    ] {
        let error = read(&declaration(&offered))?.err().ok_or(())?;
        assert_eq!(error.refusal().issue(), CaptureIssue::Grammar { cause });
    }
    assert!(read(&declaration(BINDING))?.is_ok());
    Ok(())
}

#[test]
fn exact_attachment_material_moves_identity_but_source_coordinates_do_not() -> Result<(), ()> {
    let source = declaration(BINDING);
    let baseline = read(&source)?.map_err(|_| ())?;
    let moved = read(&format!("\n\n{source}"))?.map_err(|_| ())?;
    assert_eq!(baseline, moved);
    assert_eq!(canonical_content(&baseline), canonical_content(&moved));
    for (before, after) in [
        (
            "crate::subject_revision()",
            "crate::other_subject_revision()",
        ),
        ("crate::check_revision()", "crate::other_check_revision()"),
        ("crate::independent_check", "crate::another_check"),
    ] {
        let changed = read(&source.replace(before, after))?.map_err(|_| ())?;
        assert_ne!(canonical_content(&baseline), canonical_content(&changed));
    }
    Ok(())
}

#[test]
fn public_attachment_construction_cannot_supply_an_empty_fragment() -> Result<(), ()> {
    let empty = GeneratedTree::assembled(Vec::new()).map_err(|_| ())?;
    let fragment =
        GeneratedTree::assembled(vec![GeneratedToken::word("provided")]).map_err(|_| ())?;
    for fragments in [
        [empty.clone(), fragment.clone(), fragment.clone()],
        [fragment.clone(), empty.clone(), fragment.clone()],
        [fragment.clone(), fragment, empty],
    ] {
        let [subject, check, call] = fragments;
        let error = trial::AttachmentExpressions::declared(subject, check, call)
            .err()
            .ok_or(())?;
        assert_eq!(
            error,
            DeclarationError::Absent {
                seat: Seat::TargetFragment
            }
        );
    }
    let empty_source = declaration(&BINDING.replace("crate::independent_check", ""));
    assert!(
        matches!(read(&empty_source)?, Err(error) if error.refusal().issue() == CaptureIssue::Vocabulary { refusal: DeclarationError::Absent { seat: Seat::TargetFragment } })
    );
    Ok(())
}

#[test]
fn typed_input_is_explicit_nonempty_and_identity_bearing() -> Result<(), ()> {
    let source = declaration(BINDING);
    let untyped = read(&source)?.map_err(|_| ())?;
    let typed_source = format!("input = {{ $consumer::Specimen }}, {source}");
    let typed = read(&typed_source)?.map_err(|_| ())?;
    assert!(untyped.input_type().is_none());
    assert!(typed.input_type().is_some());
    assert_ne!(canonical_content(&untyped), canonical_content(&typed));
    let changed = read(&typed_source.replace("Specimen", "OtherSpecimen"))?.map_err(|_| ())?;
    assert_ne!(canonical_content(&typed), canonical_content(&changed));
    let moved = read(&format!("\n\n{typed_source}"))?.map_err(|_| ())?;
    assert_eq!(canonical_content(&typed), canonical_content(&moved));
    let empty = GeneratedTree::assembled(Vec::new()).map_err(|_| ())?;
    assert_eq!(
        untyped.with_input(empty).err(),
        Some(DeclarationError::Absent {
            seat: Seat::TargetFragment
        })
    );
    let doubled = read(&format!("input = {{ u8 }}, {typed_source}"))?
        .err()
        .ok_or(())?;
    assert_eq!(
        doubled.refusal().issue(),
        CaptureIssue::Grammar {
            cause: CaptureCause::ClauseDoubled
        }
    );
    let absent = read(&format!("input = {{}}, {source}"))?.err().ok_or(())?;
    assert_eq!(
        absent.refusal().issue(),
        CaptureIssue::Vocabulary {
            refusal: DeclarationError::Absent {
                seat: Seat::TargetFragment
            }
        }
    );
    Ok(())
}

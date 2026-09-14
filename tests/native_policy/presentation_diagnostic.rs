//! Diagnostic presentation preserves owner-declared facts independently of wording.

use macroonz::compiler::bounded::{Bounded, Capping};
use macroonz::compiler::diagnostic::{
    Diagnostic, Family, LineBody, Observed, Phase, Placement, RefusalClass, Refused, Repair,
};
use macroonz::compiler::identity::{HumanProjection, OwnerFact};
use macroonz::compiler::request::{CrateBinding, Door, Producer};
use macroonz::compiler::token::{SpanHandle, SpanTable};
use macroonz::presentation;
use serde_json::Value;

struct Refusal<const FAMILY: u8>;

impl<const FAMILY: u8> Refused for Refusal<FAMILY> {
    const PHASE: Phase = Phase::Planning;
    const FAMILY: Family = if FAMILY == 1 {
        Family::declared("fixture/one")
    } else {
        Family::declared("fixture/two")
    };

    fn class(&self) -> RefusalClass {
        RefusalClass::PlanNotStated
    }
    fn first(&self) -> String {
        "<b>missing seat</b>".to_owned()
    }
    fn observed(&self) -> Observed {
        Observed::SeatAbsent
    }
    fn body(&self) -> LineBody {
        if FAMILY == 3 {
            LineBody::Body {
                further: 7,
                capping: Capping::Truncated { omitted: 2 },
            }
        } else {
            LineBody::SingleCause
        }
    }
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
    fn repairs(&self) -> Bounded<Repair, 8> {
        let description = HumanProjection::projected("Supply the independently declared <seat>.");
        match description {
            Ok(description) => Bounded::from_array([Repair {
                declared_by: OwnerFact {
                    home: "fixture",
                    name: "required-seat",
                },
                description,
            }]),
            Err(_) => Bounded::empty(),
        }
    }
}

const DOOR: Door = Door::declared(
    "fixture",
    "fixture/grammar",
    "fixture::compile",
    CrateBinding::declared("renamed"),
    Producer {
        namespace: "fixture",
        name: "projection",
    },
);

fn parsed(value: &presentation::Presentation) -> Result<Value, String> {
    crate::presentation_formats::agree(value)?;
    serde_json::from_str(&value.json()).map_err(|error| error.to_string())
}

#[test]
fn presentation_retains_the_body_account_separately_from_related_capping() -> Result<(), String> {
    let diagnostic = Diagnostic::refused(&Refusal::<3>, &DOOR, &Placement::WholeDeclaration);
    let value = parsed(&presentation::compiler_diagnostic(&diagnostic))?;
    assert_eq!(
        crate::presentation_formats::field(&value, "/record/body")?,
        &serde_json::json!({
            "kind":"body", "value":{"further":7usize,"capping":{"kind":"truncated","value":2usize}},
        })
    );
    assert_eq!(
        crate::presentation_formats::field(&value, "/record/related_capping/kind")?,
        "complete"
    );
    Ok(())
}

#[test]
fn same_summary_does_not_erase_the_typed_cause_family() -> Result<(), String> {
    let one = Diagnostic::refused(&Refusal::<1>, &DOOR, &Placement::WholeDeclaration);
    let two = Diagnostic::refused(&Refusal::<2>, &DOOR, &Placement::WholeDeclaration);
    assert_eq!(one.summary(), two.summary());
    assert_eq!(one.related().capping(), Capping::Complete);
    assert_ne!(one, two);
    assert_eq!(one.family().name(), "fixture/one");
    assert_eq!(two.family().name(), "fixture/two");
    let shown = presentation::compiler_diagnostic(&one);
    let value = parsed(&shown)?;
    for (path, expected) in [
        ("/record/family", "fixture/one"),
        ("/record/phase", "planning"),
        ("/record/class", "plan-not-stated"),
        ("/record/observed/name", "seat-absent"),
        ("/record/body/kind", "single-cause"),
        ("/record/site/kind", "whole-declaration"),
        ("/record/repairs/0/home", "fixture"),
        ("/record/repairs/0/name", "required-seat"),
        (
            "/record/repairs/0/description",
            "Supply the independently declared <seat>.",
        ),
    ] {
        assert_eq!(
            value.pointer(path).and_then(Value::as_str),
            Some(expected),
            "{path}"
        );
    }
    assert!(!shown.html().contains("<seat>"));
    assert!(shown.html().contains("&lt;seat&gt;"));
    assert!(!shown.markdown().contains("<b>missing"));
    assert_ne!(value, parsed(&presentation::compiler_diagnostic(&two))?);
    Ok(())
}

#[test]
fn unresolved_coordinate_retains_its_handle_and_reach() -> Result<(), String> {
    let diagnostic = Diagnostic::refused(
        &Refusal::<1>,
        &DOOR,
        &Placement::AtToken {
            token: SpanHandle::at(42),
            spans: &SpanTable::ByteOffsets(Bounded::empty()),
        },
    );
    let value = parsed(&presentation::compiler_diagnostic(&diagnostic))?;
    assert_eq!(
        value
            .pointer("/record/site/value/token")
            .and_then(Value::as_u64),
        Some(42)
    );
    assert_eq!(
        value
            .pointer("/record/site/value/coordinate/kind")
            .and_then(Value::as_str),
        Some("not-reached")
    );
    assert_eq!(
        value
            .pointer("/record/site/value/coordinate/value/reaches")
            .and_then(Value::as_u64),
        Some(0)
    );
    assert!(
        value
            .pointer("/record/site/value/coordinate/value/position")
            .is_none()
    );
    Ok(())
}

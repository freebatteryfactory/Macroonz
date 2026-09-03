//! The support home's two refusal rosters observed at the public boundary: every support-declaration refusal keeps its slot, byte, and diagnostic posture, and both shell refusals keep their typed causality and complete canonical payloads.

use macroonz_compiler::request;
use macroonz_compiler::support::{
    ASSEMBLY_FACT, DeclarationError as SupportDeclarationError, ShellError,
};
use macroonz_compiler::{
    LineBody, Observed, Overflow, Phase, RefusalClass, Refused, SHELL_FAMILY,
    SUPPORT_DECLARATION_FAMILY, TextCapture, encode_bytes,
};

/// Claim: every support-declaration refusal has one stable slot, one canonical byte, and the typed diagnostic posture declared by the support home.
/// Subject: the public support `DeclarationError` roster and its `Refused` implementation.
/// Population: all five refusal variants.
/// Hostile control: the expected slot and observation are restated independently for every row, so two variants sharing either disagrees.
/// Denominator: the complete public support-declaration refusal roster.
/// Evidence ceiling: these payload-free rows establish their own bytes and diagnostic facts, not a later composed diagnostic identity.
#[test]
fn every_support_declaration_refusal_keeps_its_public_contract() {
    let cases = [
        (
            SupportDeclarationError::EmptyNamespace,
            0,
            Observed::SeatAbsent,
            "a name states no owner",
        ),
        (
            SupportDeclarationError::EmptyStem,
            1,
            Observed::SeatAbsent,
            "a name states no spelling",
        ),
        (
            SupportDeclarationError::SpellingNotAnIdentifier,
            2,
            Observed::ContractDisagreement,
            "a rendered spelling is not one Rust identifier",
        ),
        (
            SupportDeclarationError::PathSegmentsAbsent,
            3,
            Observed::SeatAbsent,
            "a rendered path names no segment past the crate it is rooted at",
        ),
        (
            SupportDeclarationError::PathSegmentsUnbounded,
            4,
            Observed::BoundExceeded,
            "a rendered path carries more segments than the declared magnitude",
        ),
    ];
    for (refusal, slot, observed, first) in cases {
        assert_eq!(refusal.slot(), slot);
        assert_eq!(refusal.canonical_bytes(), vec![slot]);
        let mut appended = vec![u8::MAX];
        refusal.encode_into(&mut appended);
        assert_eq!(appended, vec![u8::MAX, slot]);
        assert_eq!(refusal.to_string(), first);
        assert_eq!(refusal.class(), RefusalClass::CarrierNotDeclared);
        assert_eq!(refusal.first(), first);
        assert_eq!(refusal.observed(), observed);
        assert_eq!(refusal.body(), LineBody::SingleCause);
        assert!(refusal.related().is_empty());
        assert!(refusal.repairs().is_empty());
    }
    assert_eq!(<SupportDeclarationError as Refused>::PHASE, Phase::Capture);
    assert_eq!(
        <SupportDeclarationError as Refused>::FAMILY,
        SUPPORT_DECLARATION_FAMILY
    );
}

/// Claim: both shell-refusal rows keep their typed causality and complete canonical payloads at the public boundary.
/// Subject: `ShellError` construction, encoding, display, and `Refused` projection.
/// Population: the declaration-identity mismatch and generated-tree overflow rows.
/// Hostile control: two independently captured declarations supply distinct identity payloads, while the overflow row carries distinct bound and observed counts.
/// Denominator: the complete public `ShellError` roster, including its `Overflow` conversion.
/// Evidence ceiling: this observes the refusal values directly and does not manufacture an invalid `SupportAssembly` through private seats.
#[test]
fn every_shell_refusal_keeps_its_public_contract() -> Result<(), ()> {
    let stated_capture = TextCapture::read("pub struct Stated;").map_err(|_| ())?;
    let planned_capture = TextCapture::read("pub struct Planned;").map_err(|_| ())?;
    let stated = request::committed(stated_capture.input());
    let planned = request::committed(planned_capture.input());
    let mismatch = ShellError::NotOneDeclaration { stated, planned };
    let mut mismatch_bytes = vec![0];
    encode_bytes(stated.as_bytes(), &mut mismatch_bytes);
    encode_bytes(planned.as_bytes(), &mut mismatch_bytes);
    assert_eq!(mismatch.slot(), 0);
    assert_eq!(mismatch.canonical_bytes(), mismatch_bytes);
    let mut mismatch_appended = vec![u8::MAX];
    mismatch.encode_into(&mut mismatch_appended);
    let mut expected_mismatch_appended = vec![u8::MAX];
    expected_mismatch_appended.extend_from_slice(&mismatch_bytes);
    assert_eq!(mismatch_appended, expected_mismatch_appended);
    assert_eq!(mismatch.class(), RefusalClass::CarrierNotAssembled);
    assert_eq!(mismatch.observed(), Observed::IdentityDisagreement);
    assert_eq!(mismatch.body(), LineBody::SingleCause);
    assert!(mismatch.related().is_empty());
    assert!(mismatch.to_string().contains("other than the one"));
    let mismatch_repairs = mismatch.repairs();
    assert_eq!(mismatch_repairs.len(), 1);
    assert_eq!(
        mismatch_repairs
            .as_slice()
            .first()
            .map(|repair| repair.declared_by),
        Some(ASSEMBLY_FACT)
    );

    let overflow = Overflow {
        capacity: 16,
        offered: 19,
    };
    let unbounded = ShellError::from(overflow);
    assert_eq!(
        unbounded,
        ShellError::TreeUnbounded {
            bound: 16,
            observed: 19,
        }
    );
    let mut unbounded_bytes = vec![1];
    unbounded_bytes.extend_from_slice(&16_u64.to_be_bytes());
    unbounded_bytes.extend_from_slice(&19_u64.to_be_bytes());
    assert_eq!(unbounded.slot(), 1);
    assert_eq!(unbounded.canonical_bytes(), unbounded_bytes);
    let mut unbounded_appended = vec![u8::MAX];
    unbounded.encode_into(&mut unbounded_appended);
    let mut expected_unbounded_appended = vec![u8::MAX];
    expected_unbounded_appended.extend_from_slice(&unbounded_bytes);
    assert_eq!(unbounded_appended, expected_unbounded_appended);
    assert_eq!(unbounded.class(), RefusalClass::MagnitudeNotHeld);
    assert_eq!(unbounded.observed(), Observed::BoundExceeded);
    assert_eq!(unbounded.body(), LineBody::SingleCause);
    assert!(unbounded.related().is_empty());
    assert!(unbounded.repairs().is_empty());
    assert!(
        unbounded
            .to_string()
            .contains("19 offered where 16 are declared")
    );

    assert_eq!(<ShellError as Refused>::PHASE, Phase::Assembly);
    assert_eq!(<ShellError as Refused>::FAMILY, SHELL_FAMILY);
    Ok(())
}

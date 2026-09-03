//! The descriptor provider roster observed through its public composition contract: one provider needs no ceremony, an empty roster refuses as an absent seat, doubled identities are reported once each, the magnitude is settled before duplicate work, and every issue publishes its exact bytes.

use macroonz_compiler::descriptor::{
    Composition, CompositionIssue, DESCRIPTOR_MEANING_FACT, DeclarationError, PROVIDER_LIMIT,
    Provider, Seat,
};
use macroonz_compiler::{
    Capping, CrateBinding, Diagnostic, Door, LineBody, Observed, OwnerFact, OwnerIdentity, Phase,
    Placement, Producer, RefusalClass, Refused, Site, encode_bytes,
};

/// The public door that places composition refusals for this external lane.
const COMPOSITION_DOOR: Door = Door::declared(
    "lane",
    "lane.descriptor-composition.grammar",
    "lane::descriptor_composition",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "lane",
        name: "descriptor-content-receipts",
    },
);

/// Distinct doubled identities that exactly fill the declared provider magnitude.
const MAXIMUM_DOUBLED_PROVIDERS: usize = 32;

const _: () = assert!(MAXIMUM_DOUBLED_PROVIDERS.saturating_mul(2) == PROVIDER_LIMIT);

fn provider(subject: &'static str, discriminator: u8) -> Provider {
    Provider {
        identity: OwnerIdentity {
            subject,
            bytes: [discriminator; 32],
        },
        home: OwnerFact {
            home: "descriptor-fixture",
            name: "provider-is-owned",
        },
        composes: "fixture-kind",
    }
}

/// One declared provider roster with distinct identities.
fn providers(count: usize) -> Vec<Provider> {
    (0..count)
        .map(|position| {
            let discriminator = u8::try_from(position).unwrap_or(u8::MAX);
            provider("lane/bounded-provider", discriminator)
        })
        .collect()
}

/// Claim: the total one-provider road and the ergonomic dynamic road establish the same private non-empty composition shape.
/// Subject: the public `Composition::of_one`, `Composition::declared`, and provider-reading roads.
/// Population: one provider presented once through each constructor.
/// Hostile control: the separate empty-input lane below proves the dynamic road remains fallible where the total road cannot be.
/// Denominator: both public composition constructors.
/// Evidence ceiling: this establishes their one-provider equivalence, not arbitrary roster equivalence.
/// Retained-regression policy: either road changing the retained provider requires an explicit composition-construction ruling.
#[test]
fn one_provider_needs_no_bounded_container_ceremony() -> Result<(), ()> {
    let sole = provider("lane/sole-provider", 1);
    let direct = Composition::of_one(sole);
    let dynamic = Composition::declared(vec![sole]).map_err(|_| ())?;
    assert_eq!(direct, dynamic);
    assert_eq!(direct.first(), &sole);
    assert_eq!(direct.providers().count(), 1usize);
    Ok(())
}

/// Claim: an empty dynamic provider roster refuses through the descriptor home's existing provider-seat declaration vocabulary and projects through the ordinary diagnostic road.
/// Subject: `Composition::declared`, `CompositionIssue`, `Refused`, and `Diagnostic::refused`.
/// Population: the sole empty provider roster.
/// Hostile control: the magnitude and duplicate lanes below establish distinct observations and canonical material.
/// Denominator: the complete empty-input boundary.
/// Evidence ceiling: this proves the typed refusal and projection, not a library-owned stderr side effect.
/// Retained-regression policy: changing the seat, class, observation, phase, placement, or prose requires an explicit public diagnostic ruling.
#[test]
fn empty_composition_refuses_as_an_absent_provider() -> Result<(), ()> {
    let refusal = Composition::declared(Vec::new()).err().ok_or(())?;
    let expected = CompositionIssue::Declaration {
        refusal: DeclarationError::Absent {
            seat: Seat::Provider,
        },
    };
    assert_eq!(refusal.first_issue(), &expected);
    assert_eq!(refusal.to_string(), "the declaration states no provider");
    assert_eq!(Refused::class(&refusal), RefusalClass::CarrierNotDeclared);
    assert_eq!(Refused::observed(&refusal), Observed::SeatAbsent);
    assert_eq!(Refused::body(&refusal), LineBody::SingleCause);
    assert!(Refused::related(&refusal).is_empty());

    let diagnostic = Diagnostic::refused(&refusal, &COMPOSITION_DOOR, &Placement::WholeDeclaration);
    assert_eq!(diagnostic.phase(), Phase::Capture);
    assert_eq!(diagnostic.observed(), Observed::SeatAbsent);
    assert_eq!(diagnostic.site(), Site::WholeDeclaration);
    assert!(
        diagnostic
            .summary()
            .contains("the declaration states no provider")
    );
    assert!(diagnostic.related().carried().is_empty());
    let [repair] = diagnostic.repairs() else {
        return Err(());
    };
    assert_eq!(repair.declared_by, DESCRIPTOR_MEANING_FACT);
    assert_eq!(
        repair.description.shown(),
        "state at least one provider of descriptor material"
    );
    Ok(())
}

/// Claim: a descriptor composition retains every uniquely identified provider in declared order and reports each doubled identity once at its first occurrence.
/// Subject: the public `Composition::declared`, `providers`, and `CompositionError` roads.
/// Population: two lawful providers followed by repeated occurrences of both identities.
/// Hostile control: each identity appears more than twice, so a pairwise scan that reports every collision or reports the later occurrence disagrees with the exact issue roster.
/// Denominator: the complete public duplicate-scan route over this declared provider roster.
/// Evidence ceiling: this establishes identity-keyed duplicate behavior for two providers; the empty and magnitude boundaries have their own lanes below.
#[test]
fn descriptor_composition_reports_each_doubled_provider_once() -> Result<(), ()> {
    let first = provider("lane/first-provider", 1);
    let second = provider("lane/second-provider", 2);
    let first_again = Provider {
        identity: first.identity,
        home: OwnerFact {
            home: "another-fixture",
            name: "same-provider-identity",
        },
        composes: "another-kind",
    };
    let second_again = Provider {
        identity: second.identity,
        home: OwnerFact {
            home: "another-fixture",
            name: "same-provider-identity",
        },
        composes: "another-kind",
    };
    let composition = Composition::declared(vec![first, second]).map_err(|_| ())?;
    assert_eq!(composition.first(), &first);
    assert_eq!(
        composition.providers().iter().copied().collect::<Vec<_>>(),
        vec![first, second]
    );

    let refusal = Composition::declared(vec![first, second, first_again, second_again, first])
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.issues().copied().collect::<Vec<_>>(),
        vec![
            CompositionIssue::ProviderDoubled {
                provider: first.identity,
            },
            CompositionIssue::ProviderDoubled {
                provider: second.identity,
            },
        ]
    );
    assert_eq!(Refused::class(&refusal), RefusalClass::CarrierNotDeclared);
    assert_eq!(Refused::observed(&refusal), Observed::IdentityDisagreement);
    assert_eq!(
        Refused::body(&refusal),
        LineBody::Body {
            further: 1,
            capping: Capping::Complete,
        }
    );
    let related = Refused::related(&refusal);
    let [second_issue_bytes] = related.as_slice() else {
        return Err(());
    };
    assert_ne!(refusal.first_issue().canonical_bytes(), *second_issue_bytes);
    let repairs = Refused::repairs(&refusal);
    let [repair] = repairs.as_slice() else {
        return Err(());
    };
    assert_eq!(repair.declared_by, DESCRIPTOR_MEANING_FACT);
    assert_eq!(
        repair.description.shown(),
        "state each provider identity once"
    );
    Ok(())
}

/// Claim: the provider ceiling itself bounds duplicate findings, so every lawful finding fits and no capping posture exists.
/// Subject: the public `Composition::declared`, `CompositionError::issues`, and `Refused::body` roads.
/// Population: thirty-two distinct identities, each declared exactly twice, filling the sixty-four-provider ceiling.
/// Hostile control: any prefix cap below the derived thirty-two-finding denominator would omit a declared duplicate.
/// Denominator: the maximum number of distinct doubled identities one admitted provider roster can contain.
/// Evidence ceiling: this proves complete maximum-width duplicate reporting under the current provider magnitude, not larger future provider postures.
/// Retained-regression policy: any missing identity or truncated body requires an explicit provider-magnitude and diagnostic ruling.
#[test]
fn descriptor_composition_retains_every_maximum_width_duplicate() -> Result<(), ()> {
    let distinct = providers(MAXIMUM_DOUBLED_PROVIDERS);
    let declared = distinct
        .iter()
        .flat_map(|provider| [*provider, *provider])
        .collect();
    let refusal = Composition::declared(declared).err().ok_or(())?;
    let expected = distinct
        .iter()
        .map(|provider| CompositionIssue::ProviderDoubled {
            provider: provider.identity,
        })
        .collect::<Vec<_>>();
    assert_eq!(refusal.issues().copied().collect::<Vec<_>>(), expected);
    assert_eq!(
        Refused::body(&refusal),
        LineBody::Body {
            further: MAXIMUM_DOUBLED_PROVIDERS.saturating_sub(1),
            capping: Capping::Complete,
        }
    );
    Ok(())
}

/// Claim: the exact provider ceiling is admitted and one further provider refuses through the shared declaration vocabulary with the exact bound and offered count.
/// Subject: the public `Composition::declared` bound crossing.
/// Population: exactly `PROVIDER_LIMIT` distinct identities and one more distinct identity.
/// Hostile control: every identity differs, so the duplicate scan cannot mask the magnitude refusal.
/// Denominator: both sides of the provider-list upper bound.
/// Evidence ceiling: this establishes upper-bound accounting, not empty input.
#[test]
fn descriptor_composition_reports_its_provider_magnitude() -> Result<(), ()> {
    let lawful = Composition::declared(providers(PROVIDER_LIMIT)).map_err(|_| ())?;
    assert_eq!(lawful.providers().count(), PROVIDER_LIMIT);

    let refusal = Composition::declared(providers(PROVIDER_LIMIT.saturating_add(1)))
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &CompositionIssue::Declaration {
            refusal: DeclarationError::Unbounded {
                seat: Seat::Provider,
                bound: u64::try_from(PROVIDER_LIMIT).unwrap_or(u64::MAX),
                observed: u64::try_from(PROVIDER_LIMIT.saturating_add(1)).unwrap_or(u64::MAX),
            },
        }
    );
    assert_eq!(Refused::observed(&refusal), Observed::BoundExceeded);
    let repairs = Refused::repairs(&refusal);
    let [repair] = repairs.as_slice() else {
        return Err(());
    };
    assert_eq!(repair.declared_by, DESCRIPTOR_MEANING_FACT);
    assert_eq!(
        repair.description.shown(),
        "state no more than the declared provider magnitude"
    );
    Ok(())
}

/// Claim: provider magnitude is settled before duplicate exploration, so the pairwise scan receives only the bounded informed roster.
/// Subject: the operation order inside `Composition::declared`.
/// Population: `PROVIDER_LIMIT` distinct providers followed by one duplicate of the first.
/// Hostile control: duplicate-first processing would report the repeated identity instead of the provider-seat overrun.
/// Denominator: the one input where magnitude and duplication are simultaneously false.
/// Evidence ceiling: this establishes refusal precedence and bounded duplicate work, not the scan's asymptotic complexity in isolation.
/// Retained-regression policy: reversing the refusal order requires an explicit work-bound and diagnostic ruling.
#[test]
fn composition_settles_magnitude_before_duplicate_work() -> Result<(), ()> {
    let mut declared = providers(PROVIDER_LIMIT);
    let repeated = declared.first().copied().ok_or(())?;
    declared.push(repeated);
    let refusal = Composition::declared(declared).err().ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &CompositionIssue::Declaration {
            refusal: DeclarationError::Unbounded {
                seat: Seat::Provider,
                bound: u64::try_from(PROVIDER_LIMIT).unwrap_or(u64::MAX),
                observed: u64::try_from(PROVIDER_LIMIT.saturating_add(1)).unwrap_or(u64::MAX),
            },
        }
    );
    Ok(())
}

/// Claim: absence, magnitude, and each doubled identity publish their exact canonical issue material under stable public slots.
/// Subject: `CompositionIssue::canonical_bytes`.
/// Population: one absent provider seat, one overrun provider seat, and two doubled provider identities.
/// Hostile control: expected vectors rebuilt from the public frame disagree with a moved slot, nested declaration row, field order, subject, or identity byte.
/// Denominator: every current composition issue shape and two values of its identity-bearing shape.
/// Evidence ceiling: this fixes the complete current byte grammar, not cryptographic collision resistance.
/// Retained-regression policy: any changed vector requires an explicit canonical-byte ruling.
#[test]
fn composition_issue_bytes_retain_exact_shape_and_identity() {
    let absent = CompositionIssue::Declaration {
        refusal: DeclarationError::Absent {
            seat: Seat::Provider,
        },
    };
    let mut absent_refusal = vec![3];
    encode_bytes(b"provider", &mut absent_refusal);
    let mut absent_expected = vec![1];
    encode_bytes(&absent_refusal, &mut absent_expected);

    let unbounded = CompositionIssue::Declaration {
        refusal: DeclarationError::Unbounded {
            seat: Seat::Provider,
            bound: 64,
            observed: 65,
        },
    };
    let mut unbounded_refusal = vec![5];
    encode_bytes(b"provider", &mut unbounded_refusal);
    unbounded_refusal.extend_from_slice(&64_u64.to_be_bytes());
    unbounded_refusal.extend_from_slice(&65_u64.to_be_bytes());
    let mut unbounded_expected = vec![1];
    encode_bytes(&unbounded_refusal, &mut unbounded_expected);

    let first_doubled = CompositionIssue::ProviderDoubled {
        provider: provider("lane/first-provider", 1).identity,
    };
    let mut first_citation = Vec::new();
    encode_bytes(b"lane/first-provider", &mut first_citation);
    encode_bytes(&[1; 32], &mut first_citation);
    let mut first_expected = vec![0];
    encode_bytes(&first_citation, &mut first_expected);

    let second_doubled = CompositionIssue::ProviderDoubled {
        provider: provider("lane/second-provider", 2).identity,
    };
    let mut second_citation = Vec::new();
    encode_bytes(b"lane/second-provider", &mut second_citation);
    encode_bytes(&[2; 32], &mut second_citation);
    let mut second_expected = vec![0];
    encode_bytes(&second_citation, &mut second_expected);

    let cases = [
        (absent, 1, absent_expected),
        (unbounded, 1, unbounded_expected),
        (first_doubled, 0, first_expected),
        (second_doubled, 0, second_expected),
    ];
    for (issue, slot, expected) in cases {
        assert_eq!(issue.slot(), slot);
        assert_eq!(issue.canonical_bytes(), expected);
    }
}

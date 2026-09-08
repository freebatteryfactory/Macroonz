//! Structural records preserve every public arm, free text and portable caller-stated coordinates.

use super::wire;
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedStructuralDisagreement as Historical, ArchivedVerdict, read_verdict,
    retain_structural,
};
use macroonz_harness::oracle::{StructuralDisagreement as Live, StructuralVerdict};
use macroonz_harness::report::archive::ArchiveLimits;

type Case = (Live, Historical, Vec<u8>);

fn coordinate_cases() -> Vec<Case> {
    let mut cardinality = vec![1];
    cardinality.extend_from_slice(&0u64.to_be_bytes());
    cardinality.extend_from_slice(&0u64.to_be_bytes());
    vec![
        (Live::UnexpectedItem, Historical::UnexpectedItem, vec![0]),
        (
            Live::OutputCardinality {
                declared: 0,
                read: 0,
            },
            Historical::OutputCardinality {
                declared: 0,
                read: 0,
            },
            cardinality,
        ),
        (
            Live::DuplicateImplementation { at: 7 },
            Historical::DuplicateImplementation { at: 7 },
            wire::position(2, 7),
        ),
        (
            Live::ImplementationTarget { at: 9 },
            Historical::ImplementationTarget { at: 9 },
            wire::position(3, 9),
        ),
        (
            Live::TraitPath { at: 11 },
            Historical::TraitPath { at: 11 },
            wire::position(4, 11),
        ),
        (
            Live::ImplPosture { at: 13 },
            Historical::ImplPosture { at: 13 },
            wire::position(5, 13),
        ),
    ]
}

fn text_cases() -> Vec<Case> {
    vec![
        (
            Live::MeaningBearingAttribute {
                at: 2,
                attribute: String::new(),
            },
            Historical::MeaningBearingAttribute {
                at: 2,
                attribute: String::new(),
            },
            wire::positioned_text(6, 2, ""),
        ),
        (
            Live::UnexpectedImplMember {
                at: 3,
                member: "free text".to_owned(),
            },
            Historical::UnexpectedImplMember {
                at: 3,
                member: "free text".to_owned(),
            },
            wire::positioned_text(7, 3, "free text"),
        ),
        (
            Live::DuplicateMember {
                at: 4,
                member: "é\0".to_owned(),
            },
            Historical::DuplicateMember {
                at: 4,
                member: "é\0".to_owned(),
            },
            wire::positioned_text(8, 4, "é\0"),
        ),
        (
            Live::MissingImplMember {
                at: 5,
                member: "MISSING".to_owned(),
            },
            Historical::MissingImplMember {
                at: 5,
                member: "MISSING".to_owned(),
            },
            wire::positioned_text(9, 5, "MISSING"),
        ),
        (
            Live::MemberValueUnread {
                at: 6,
                member: "UNREAD".to_owned(),
            },
            Historical::MemberValueUnread {
                at: 6,
                member: "UNREAD".to_owned(),
            },
            wire::positioned_text(10, 6, "UNREAD"),
        ),
        (
            Live::MemberValue {
                at: 7,
                member: String::new(),
            },
            Historical::MemberValue {
                at: 7,
                member: String::new(),
            },
            wire::positioned_text(11, 7, ""),
        ),
    ]
}

#[test]
fn all_structural_verdict_fields_cross_independent_vectors() -> Result<(), ()> {
    let limits = ArchiveLimits::declared(2048, 256);
    for (live, expected, payload) in coordinate_cases().into_iter().chain(text_cases()) {
        let independent = wire::envelope(3, 1, &payload);
        let written =
            retain_structural(&StructuralVerdict::Deviates(live), limits).map_err(|_| ())?;
        assert_eq!(written.encoded(), independent);
        let loaded =
            read_verdict(&independent, ArchivedMethod::Structural, limits).map_err(|_| ())?;
        assert_eq!(
            loaded.verdict(),
            &ArchivedVerdict::StructuralDeviates(expected)
        );
        assert_eq!(loaded, written);
    }
    for (verdict, expected, slot) in [
        (
            StructuralVerdict::Conforms,
            ArchivedVerdict::StructuralConforms,
            0,
        ),
        (
            StructuralVerdict::Unparsable,
            ArchivedVerdict::StructuralUnparsable,
            2,
        ),
    ] {
        let written = retain_structural(&verdict, limits).map_err(|_| ())?;
        assert_eq!(written.encoded(), wire::envelope(3, slot, &[]));
        assert_eq!(written.verdict(), &expected);
    }
    Ok(())
}

#[test]
fn historical_positions_remain_portable_claims_without_platform_indexing() -> Result<(), ()> {
    let bytes = wire::envelope(3, 1, &wire::position(3, u64::MAX));
    let loaded = read_verdict(
        &bytes,
        ArchivedMethod::Structural,
        ArchiveLimits::declared(128, 0),
    )
    .map_err(|_| ())?;
    assert_eq!(
        loaded.verdict(),
        &ArchivedVerdict::StructuralDeviates(Historical::ImplementationTarget { at: u64::MAX },)
    );
    Ok(())
}

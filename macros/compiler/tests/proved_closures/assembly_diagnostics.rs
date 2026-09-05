//! Assembly diagnostics preserve independently declared field order, append custody, and bounded related causes.

use super::{DECLARATION, OTHER_DECLARATION, expansion};
use macroonz_compiler::support::{
    ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, CargoAxis, DeliveryForm,
};
use macroonz_compiler::{Capping, Destination, LineBody, Refused, encode_bytes};

/// Public issue values encode their own declared fields rather than dropping provenance or reusing a neighbor's tag.
#[test]
fn every_issue_encoding_preserves_its_tag_and_ordered_fields() -> Result<(), ()> {
    let declared = expansion(DECLARATION).ok_or(())?;
    let other = expansion(OTHER_DECLARATION).ok_or(())?;
    let stated = declared.plan().account().commitment();
    let carried = other.plan().account().commitment();
    let source = declared.identity();
    let cases: [(AssemblyIssue, u8, Vec<&[u8]>); 7] = [
        (
            AssemblyIssue::RootsDisagree {
                axis: CargoAxis::Declared,
                stated,
                carried,
            },
            0,
            vec![b"declared", stated.as_bytes(), carried.as_bytes()],
        ),
        (AssemblyIssue::DeclaredAxisRequiresStampedCargo, 7, vec![]),
        (
            AssemblyIssue::CargoConsumedTwice {
                source,
                destination: Destination::TestCarrier,
            },
            2,
            vec![source.as_bytes(), b"test-carrier"],
        ),
        (
            AssemblyIssue::CargoReachesASecondDestination {
                axis: CargoAxis::Bench,
                destination: Destination::TestCarrier,
            },
            3,
            vec![b"bench", b"test-carrier"],
        ),
        (
            AssemblyIssue::CargoNotTheSourcesOwn {
                source,
                destination: Destination::BenchCarrier,
            },
            4,
            vec![source.as_bytes(), b"bench-carrier"],
        ),
        (AssemblyIssue::TwoFormsCarried, 5, vec![]),
        (
            AssemblyIssue::StampedCargoAbsent {
                form: DeliveryForm::Trials,
            },
            6,
            vec![b"trials"],
        ),
    ];
    for (issue, tag, fields) in cases {
        let mut expected = vec![tag];
        for field in fields {
            encode_bytes(field, &mut expected);
        }
        assert_eq!(issue.canonical_bytes(), expected);
        let mut appended = vec![0xa5];
        issue.encode_into(&mut appended);
        let mut prefixed = vec![0xa5];
        prefixed.extend(expected);
        assert_eq!(appended, prefixed);
    }
    Ok(())
}

/// The diagnostic body counts retained secondary causes separately from omitted causes and keeps the first cause primary.
#[test]
fn assembly_error_related_causes_and_capping_remain_distinct() {
    let first = AssemblyIssue::TwoFormsCarried;
    let second = AssemblyIssue::DeclaredAxisRequiresStampedCargo;
    let single = AssemblyError::of(first);
    assert_eq!(single.body(), LineBody::SingleCause);
    assert_eq!(single.capping(), Capping::Complete);
    assert!(single.related().is_empty());
    assert_eq!(single.to_string(), first.to_string());
    assert!(std::error::Error::source(&single).is_none());

    for total in [
        ASSEMBLY_ISSUE_LIMIT - 1,
        ASSEMBLY_ISSUE_LIMIT,
        ASSEMBLY_ISSUE_LIMIT + 1,
    ] {
        let refusal = AssemblyError::over(first, vec![second; total - 1]);
        let retained = total.min(ASSEMBLY_ISSUE_LIMIT);
        let omitted = total - retained;
        let capping = if omitted == 0 {
            Capping::Complete
        } else {
            Capping::Truncated { omitted }
        };
        assert_eq!(refusal.first_issue(), &first);
        assert_eq!(refusal.issues().count(), retained);
        assert_eq!(refusal.capping(), capping);
        assert_eq!(
            refusal.body(),
            LineBody::Body {
                further: retained - 1,
                capping
            }
        );
        assert_eq!(refusal.related(), vec![vec![7]; retained - 1]);
        let expected = if omitted > 0 {
            format!(
                "{first}, and {} further issues, {omitted} of them not carried",
                retained - 1
            )
        } else {
            format!("{first}, and {} further issues", retained - 1)
        };
        assert_eq!(refusal.to_string(), expected);
    }
}

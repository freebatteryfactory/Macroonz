//! The compiled method compares caller-reported compiler outcomes without claiming who produced the report.
//!
//! The observations exercise accepted values, duplicate read-backs, and refusal posture while the test retains the method's explicit provenance ceiling.

use macroonz_harness::oracle::{
    self, CompiledDisagreement, CompiledObservation, CompiledVerdict, DeclaredBehavior,
    DeclaredReadBack, DeclaredReadBackRoster, DeclaredReadBackRosterRefusal, ORACLE_CAUSE_FAMILY,
    ObservedMember, ObservedValue,
};
use macroonz_harness::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};
use std::fmt;

enum CompiledRoadFailure {
    Roster(DeclaredReadBackRosterRefusal),
    ExpectedRefusal,
}

impl fmt::Debug for CompiledRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Roster(refusal) => formatter.debug_tuple("Roster").field(refusal).finish(),
            Self::ExpectedRefusal => formatter.write_str("ExpectedRefusal"),
        }
    }
}

impl From<DeclaredReadBackRosterRefusal> for CompiledRoadFailure {
    fn from(refusal: DeclaredReadBackRosterRefusal) -> Self {
        Self::Roster(refusal)
    }
}

#[test]
fn caller_reported_values_are_compared_without_claiming_compiler_provenance()
-> Result<(), CompiledRoadFailure> {
    let members = [DeclaredReadBack {
        name: "MODE",
        value: ObservedValue::Word("Ready".to_owned()),
    }];
    let roster = DeclaredReadBackRoster::declared(&members)?;
    let declared = DeclaredBehavior::ReadsBack(roster);
    let exact = CompiledObservation::ReadBack(vec![ObservedMember {
        name: "MODE".to_owned(),
        value: ObservedValue::Word("Ready".to_owned()),
    }]);
    assert_eq!(
        oracle::compiled::compared(&exact, &declared),
        CompiledVerdict::Conforms
    );
    assert_eq!(
        oracle::compiled::compared(
            &CompiledObservation::RefusedByCompiler,
            &DeclaredBehavior::RefusedByCompiler,
        ),
        CompiledVerdict::Conforms
    );

    let duplicate = CompiledObservation::ReadBack(vec![
        ObservedMember {
            name: "MODE".to_owned(),
            value: ObservedValue::Word("Ready".to_owned()),
        },
        ObservedMember {
            name: "MODE".to_owned(),
            value: ObservedValue::Word("Ready".to_owned()),
        },
    ]);
    let verdict = oracle::compiled::compared(&duplicate, &declared);
    assert_eq!(
        verdict,
        CompiledVerdict::Deviates(CompiledDisagreement::DuplicateMember {
            member: "MODE".to_owned(),
        })
    );

    let conclusion = verdict.concluded(FindingLocation::at(file!(), line!()));
    let TrialConclusion::Refused(finding) = conclusion else {
        return Err(CompiledRoadFailure::ExpectedRefusal);
    };
    assert_eq!(finding.class(), FailureClass::OracleDisagreement);
    assert_eq!(
        finding.cause(),
        FindingCause::named(ORACLE_CAUSE_FAMILY, "compiled-duplicate-member")
    );
    Ok(())
}

#[test]
fn a_declared_read_back_roster_refuses_two_authorities_for_one_member() {
    let repeated = [
        DeclaredReadBack {
            name: "MODE",
            value: ObservedValue::Truth(true),
        },
        DeclaredReadBack {
            name: "MODE",
            value: ObservedValue::Truth(false),
        },
    ];
    assert_eq!(
        DeclaredReadBackRoster::declared(&repeated),
        Err(DeclaredReadBackRosterRefusal::DuplicateMember { at: 1usize })
    );
}

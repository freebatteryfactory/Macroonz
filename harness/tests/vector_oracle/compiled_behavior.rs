//! The compiled method compares caller-reported compiler outcomes without claiming who produced the report.
//!
//! The observations exercise accepted values, duplicate read-backs, and refusal posture while the test retains the method's explicit provenance ceiling.

use macroonz_harness::oracle::{
    self, CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledObservation,
    CompiledVerdict, DeclaredBehavior, DeclaredCompilation, DeclaredReadBack,
    DeclaredReadBackRoster, DeclaredReadBackRosterRefusal, DiagnosticAnchor, ORACLE_CAUSE_FAMILY,
    ObservedCompilation, ObservedMember, ObservedValue, PrimarySourceSpan,
    PrimarySourceSpanRefusal, RelativeSourcePath, RelativeSourcePathRefusal, RustcErrorCode,
    RustcErrorCodeRefusal, SourcePosition, SourcePositionRefusal,
};
use macroonz_harness::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};
use std::fmt;

enum CompiledRoadFailure {
    Roster(DeclaredReadBackRosterRefusal),
    Code(RustcErrorCodeRefusal),
    Path(RelativeSourcePathRefusal),
    Position(SourcePositionRefusal),
    Span(PrimarySourceSpanRefusal),
    ExpectedRefusal,
}

impl fmt::Debug for CompiledRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Roster(refusal) => formatter.debug_tuple("Roster").field(refusal).finish(),
            Self::Code(refusal) => formatter.debug_tuple("Code").field(refusal).finish(),
            Self::Path(refusal) => formatter.debug_tuple("Path").field(refusal).finish(),
            Self::Position(refusal) => formatter.debug_tuple("Position").field(refusal).finish(),
            Self::Span(refusal) => formatter.debug_tuple("Span").field(refusal).finish(),
            Self::ExpectedRefusal => formatter.write_str("ExpectedRefusal"),
        }
    }
}

impl From<DeclaredReadBackRosterRefusal> for CompiledRoadFailure {
    fn from(refusal: DeclaredReadBackRosterRefusal) -> Self {
        Self::Roster(refusal)
    }
}

impl From<RustcErrorCodeRefusal> for CompiledRoadFailure {
    fn from(refusal: RustcErrorCodeRefusal) -> Self {
        Self::Code(refusal)
    }
}

impl From<RelativeSourcePathRefusal> for CompiledRoadFailure {
    fn from(refusal: RelativeSourcePathRefusal) -> Self {
        Self::Path(refusal)
    }
}

impl From<SourcePositionRefusal> for CompiledRoadFailure {
    fn from(refusal: SourcePositionRefusal) -> Self {
        Self::Position(refusal)
    }
}

impl From<PrimarySourceSpanRefusal> for CompiledRoadFailure {
    fn from(refusal: PrimarySourceSpanRefusal) -> Self {
        Self::Span(refusal)
    }
}

fn anchor(code: &str, line: u64, column: u64) -> Result<DiagnosticAnchor, CompiledRoadFailure> {
    let source = RelativeSourcePath::informed("src/main.rs")?;
    let start = SourcePosition::informed(line, column)?;
    let end = SourcePosition::informed(line, column.saturating_add(1u64))?;
    Ok(DiagnosticAnchor::at(
        RustcErrorCode::informed(code)?,
        PrimarySourceSpan::informed(source, start, end)?,
    ))
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

#[test]
fn exact_compilation_distinguishes_acceptance_code_and_primary_span()
-> Result<(), CompiledRoadFailure> {
    let expected = anchor("E0308", 12u64, 9u64)?;
    let refusal = DeclaredCompilation::refuses(expected.clone());
    assert_eq!(
        oracle::compiled::compared_compilation(&ObservedCompilation::refused(expected), &refusal,),
        CompilationVerdict::Conforms
    );
    assert_eq!(
        oracle::compiled::compared_compilation(
            &ObservedCompilation::refused(anchor("E0277", 12u64, 9u64)?),
            &refusal,
        ),
        CompilationVerdict::Deviates(CompilationDisagreement::ErrorCode {
            expected: RustcErrorCode::informed("E0308")?,
            observed: RustcErrorCode::informed("E0277")?,
        })
    );
    assert!(matches!(
        oracle::compiled::compared_compilation(
            &ObservedCompilation::refused(anchor("E0308", 18u64, 4u64)?),
            &refusal,
        ),
        CompilationVerdict::Deviates(CompilationDisagreement::PrimarySpan { .. })
    ));
    assert_eq!(
        oracle::compiled::compared_compilation(&ObservedCompilation::compiled(), &refusal),
        CompilationVerdict::Deviates(CompilationDisagreement::AcceptedWhereRefusalDeclared)
    );

    let compiles = DeclaredCompilation::compiles();
    assert_eq!(
        oracle::compiled::compared_compilation(&ObservedCompilation::compiled(), &compiles),
        CompilationVerdict::Conforms
    );
    assert!(matches!(
        oracle::compiled::compared_compilation(
            &ObservedCompilation::refused(anchor("E0308", 12u64, 9u64)?),
            &compiles,
        ),
        CompilationVerdict::Deviates(
            CompilationDisagreement::RefusedWhereAcceptanceDeclared { .. }
        )
    ));
    Ok(())
}

#[test]
fn exact_compilation_values_refuse_noncanonical_source_coordinates()
-> Result<(), CompiledRoadFailure> {
    assert_eq!(
        RustcErrorCode::informed("E308"),
        Err(RustcErrorCodeRefusal::Grammar)
    );
    assert_eq!(
        RelativeSourcePath::informed(""),
        Err(RelativeSourcePathRefusal::Empty)
    );
    assert_eq!(
        RelativeSourcePath::informed("/src/main.rs"),
        Err(RelativeSourcePathRefusal::Absolute)
    );
    assert_eq!(
        RelativeSourcePath::informed("C:/src/main.rs"),
        Err(RelativeSourcePathRefusal::Absolute)
    );
    assert_eq!(
        RelativeSourcePath::informed("//server/share/main.rs"),
        Err(RelativeSourcePathRefusal::Absolute)
    );
    assert_eq!(
        RelativeSourcePath::informed(r"src\main.rs"),
        Err(RelativeSourcePathRefusal::Backslash)
    );
    assert_eq!(
        RelativeSourcePath::informed("src/../main.rs"),
        Err(RelativeSourcePathRefusal::NonNormalSegment { at: 1usize })
    );
    assert_eq!(
        RelativeSourcePath::informed("src/./main.rs"),
        Err(RelativeSourcePathRefusal::NonNormalSegment { at: 1usize })
    );
    assert_eq!(
        RelativeSourcePath::informed("src//main.rs"),
        Err(RelativeSourcePathRefusal::NonNormalSegment { at: 1usize })
    );
    assert_eq!(
        SourcePosition::informed(0u64, 1u64),
        Err(SourcePositionRefusal::ZeroLine)
    );
    assert_eq!(
        SourcePosition::informed(1u64, 0u64),
        Err(SourcePositionRefusal::ZeroColumn)
    );

    let source = RelativeSourcePath::informed("src/main.rs")?;
    let later = SourcePosition::informed(2u64, 1u64)?;
    let earlier = SourcePosition::informed(1u64, 1u64)?;
    assert_eq!(
        PrimarySourceSpan::informed(source.clone(), later, earlier),
        Err(PrimarySourceSpanRefusal::Reversed)
    );
    let later_column = SourcePosition::informed(2u64, 2u64)?;
    assert_eq!(
        PrimarySourceSpan::informed(source.clone(), later_column, later),
        Err(PrimarySourceSpanRefusal::Reversed)
    );
    let later_line = SourcePosition::informed(3u64, 1u64)?;
    assert!(PrimarySourceSpan::informed(source.clone(), later_column, later_line).is_ok());
    assert!(PrimarySourceSpan::informed(source, earlier, earlier).is_ok());
    Ok(())
}

#[test]
fn an_exact_compilation_disagreement_uses_the_existing_oracle_conclusion_rail()
-> Result<(), CompiledRoadFailure> {
    let verdict = oracle::compiled::compared_compilation(
        &ObservedCompilation::refused(anchor("E0277", 12u64, 9u64)?),
        &DeclaredCompilation::refuses(anchor("E0308", 12u64, 9u64)?),
    );
    let conclusion = verdict.concluded(FindingLocation::at(file!(), line!()));
    let TrialConclusion::Refused(finding) = conclusion else {
        return Err(CompiledRoadFailure::ExpectedRefusal);
    };
    assert_eq!(finding.class(), FailureClass::OracleDisagreement);
    assert_eq!(
        finding.cause(),
        FindingCause::named(ORACLE_CAUSE_FAMILY, "compiled-diagnostic-error-code",)
    );
    Ok(())
}

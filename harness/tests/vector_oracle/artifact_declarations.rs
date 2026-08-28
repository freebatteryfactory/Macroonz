//! The structural method reads complete paths and declaration membership without claiming that any path resolves.
//!
//! The declaration and rendered artifact are authored independently, while hostile controls exercise the informed path and roster boundaries and the stable public operation mounts.

use macroonz_harness::oracle::{
    self, ConstantReading, DeclaredArtifact, DeclaredImplementation, DeclaredMember,
    DeclaredMemberRoster, DeclaredMemberRosterRefusal, ORACLE_CAUSE_FAMILY, StructuralDisagreement,
    StructuralPath, StructuralPathRefusal, StructuralPathRoot, StructuralPathSegment,
    StructuralVerdict,
};
use macroonz_harness::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};
use std::fmt;

const RENDERED: &str = "#[cfg(any())]\nimpl crate::contract::Declared for ::outside::Subject {\n    const KIND: crate::value::Kind = crate::value::Kind::Ready;\n    const COUNT: u64 = 7;\n}\n";

enum StructuralRoadFailure {
    Path(StructuralPathRefusal),
    Roster(DeclaredMemberRosterRefusal),
    MissingReading,
    MissingImplementation,
    MissingTraitPath,
    ExpectedRefusal,
}

impl fmt::Debug for StructuralRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(refusal) => formatter.debug_tuple("Path").field(refusal).finish(),
            Self::Roster(refusal) => formatter.debug_tuple("Roster").field(refusal).finish(),
            Self::MissingReading => formatter.write_str("MissingReading"),
            Self::MissingImplementation => formatter.write_str("MissingImplementation"),
            Self::MissingTraitPath => formatter.write_str("MissingTraitPath"),
            Self::ExpectedRefusal => formatter.write_str("ExpectedRefusal"),
        }
    }
}

impl From<StructuralPathRefusal> for StructuralRoadFailure {
    fn from(refusal: StructuralPathRefusal) -> Self {
        Self::Path(refusal)
    }
}

impl From<DeclaredMemberRosterRefusal> for StructuralRoadFailure {
    fn from(refusal: DeclaredMemberRosterRefusal) -> Self {
        Self::Roster(refusal)
    }
}

#[test]
fn complete_paths_survive_the_public_parser_and_structural_comparison()
-> Result<(), StructuralRoadFailure> {
    let target = StructuralPath::absolute(&["outside", "Subject"])?;
    let trait_path = StructuralPath::relative(&["crate", "contract", "Declared"])?;
    let attributes = [StructuralPath::relative(&["cfg"])?];
    let members = [
        DeclaredMember {
            name: "KIND",
            reading: ConstantReading::Path(StructuralPath::relative(&[
                "crate", "value", "Kind", "Ready",
            ])?),
        },
        DeclaredMember {
            name: "COUNT",
            reading: ConstantReading::Number("7".to_owned()),
        },
    ];
    let roster = DeclaredMemberRoster::declared(&members)?;
    let implementations = [DeclaredImplementation {
        target: &target,
        trait_path: Some(&trait_path),
        postures: &[],
        attributes: &attributes,
        members: roster,
    }];
    let declared = DeclaredArtifact {
        implementations: &implementations,
    };

    let Some(read) = oracle::parse::declarations_in(RENDERED) else {
        return Err(StructuralRoadFailure::MissingReading);
    };
    let Some(implementation) = read.implementations.first() else {
        return Err(StructuralRoadFailure::MissingImplementation);
    };
    let Some(read_trait_path) = implementation.trait_path.as_ref() else {
        return Err(StructuralRoadFailure::MissingTraitPath);
    };
    let target_segments: Vec<&str> = implementation
        .target
        .segments()
        .iter()
        .map(StructuralPathSegment::spelling)
        .collect();

    assert_eq!(implementation.target.root(), StructuralPathRoot::Absolute);
    assert_eq!(target_segments.as_slice(), &["outside", "Subject"]);
    assert_eq!(implementation.target.spelling(), "::outside::Subject");
    assert_eq!(read_trait_path.spelling(), "crate::contract::Declared");
    assert_eq!(
        oracle::structural::compared(&read, &declared),
        StructuralVerdict::Conforms
    );
    assert_eq!(
        oracle::structural::read(RENDERED, &declared),
        StructuralVerdict::Conforms
    );
    Ok(())
}

#[test]
fn declared_paths_and_member_rosters_refuse_collapsed_authority() {
    assert_eq!(
        StructuralPath::relative(&["crate::value", "Ready"]),
        Err(StructuralPathRefusal::EmbeddedSeparator { at: 0usize })
    );
    let repeated = [
        DeclaredMember {
            name: "MODE",
            reading: ConstantReading::Truth(true),
        },
        DeclaredMember {
            name: "MODE",
            reading: ConstantReading::Truth(false),
        },
    ];
    assert_eq!(
        DeclaredMemberRoster::declared(&repeated),
        Err(DeclaredMemberRosterRefusal::DuplicateMember { at: 1usize })
    );
}

#[test]
fn a_duplicate_observed_member_is_named_as_a_structural_disagreement()
-> Result<(), StructuralRoadFailure> {
    let target = StructuralPath::relative(&["Subject"])?;
    let members = [DeclaredMember {
        name: "VALUE",
        reading: ConstantReading::Number("1".to_owned()),
    }];
    let roster = DeclaredMemberRoster::declared(&members)?;
    let implementations = [DeclaredImplementation {
        target: &target,
        trait_path: None,
        postures: &[],
        attributes: &[],
        members: roster,
    }];
    let declared = DeclaredArtifact {
        implementations: &implementations,
    };
    let rendered = "impl Subject { const VALUE: u8 = 1; const VALUE: u8 = 1; }";
    let verdict = oracle::structural::read(rendered, &declared);
    assert_eq!(
        verdict,
        StructuralVerdict::Deviates(StructuralDisagreement::DuplicateMember {
            at: 0usize,
            member: "VALUE".to_owned(),
        })
    );

    let conclusion = verdict.concluded(FindingLocation::at(file!(), line!()));
    let TrialConclusion::Refused(finding) = conclusion else {
        return Err(StructuralRoadFailure::ExpectedRefusal);
    };
    assert_eq!(finding.class(), FailureClass::OracleDisagreement);
    assert_eq!(
        finding.cause(),
        FindingCause::named(ORACLE_CAUSE_FAMILY, "structural-duplicate-member")
    );
    Ok(())
}

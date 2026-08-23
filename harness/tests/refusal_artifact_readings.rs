//! The refusal-artifact readings: one independently authored declaration is compared with the artifact's structure and with the values `rustc` reads back from it.
//!
//! # Seats
//!
//! The damaged seats are this harness's own, and they have to be. The services
//! carry no road that renders a defective artifact — a producer that writes its
//! own exam is rehearsed only against the defects it already imagined — so a
//! damaged artifact here is damage this plane inflicted on a lawful rendering,
//! and no participant is grading itself when that text goes to a compiler.
//!
//! The lawful compiled seat below is deliberate too: it is the control the damaged ones are measured against, held inside `TestPak` on purpose.
//! The workspace consumer applies the derive through the renamed public road.
//! This lane remains `TestPak`'s lawful compiled-control seat.
//! Packaged-outsider evidence is a separate qualification surface.
//!
//! # Mechanisms
//!
//! **The malformed artifact must FAIL to compile.** Its materialized text is
//! checked in as a trybuild compile-fail fixture under a header that states
//! where it came from. It is checked in rather than written by a running test on
//! purpose: a fixture a test writes into the source tree agrees with whatever
//! the producer had just done, which is not evidence.
//!
//! **The shape-altered artifact COMPILES and declares the wrong shape.** Its
//! materialized text is checked in beside this file and included into the test
//! crate, so `rustc` compiles it exactly as it compiles any other item, and the
//! assertion below reads `SHAPE` back as a VALUE and compares it against the
//! shape this plane declares. Never against text: a text comparison here would
//! be a scan over bytes wearing this lane's name.
//!
//! Both directions. The lawful artifact is materialized and compiled the same
//! way and reads back as declared — a lane that rejected everything would catch
//! both damaged artifacts and prove nothing.
//!
//! # What holds the fixtures to the renderer, and what does not
//!
//! The LAWFUL fixture's provenance is re-derived below on every run: it must
//! still be, byte for byte, what the bound-expansion road produces from the
//! declaration stated here. That is what makes renderer drift loud — when the
//! rendering legitimately changes shape, this file says so by name.
//!
//! The lawful fixture is checked against the renderer on every run.
//! The shape-altered fixture is not re-derived; its evidence is the value-level comparison below, where it agrees with the lawful artifact in every constant except the one it changes.
//! The lawful control keeps renderer drift loud before either damaged fixture is read.
//!
//! The malformed fixture has a narrower ceiling.
//! It proves that rustc refuses the checked-in bytes and carries no claim that those bytes were produced by the current renderer.

use macroonz::{CauseOrderDeclaration, DeclaredCauseOrder, FamilyShape, RefusalFamily};
use threadpak_macroc::compile_refusal_text;
use threadpak_testpak::oracle::{
    self, CompiledDisagreement, CompiledObservation, CompiledVerdict, ConstantReading,
    DeclaredArtifact, DeclaredBehaviour, DeclaredImplementation, DeclaredMember, DeclaredReadBack,
    ORACLE_CAUSE_FAMILY, ObservedMember, ObservedValue, StructuralDisagreement, StructuralVerdict,
};
use threadpak_testpak::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};

#[path = "oracle_artifact/structural_decode.rs"]
mod structural_decode;

/// The declaration handed to the services, stated here beside the values the
/// artifacts are held to.
const DECLARATION: &str = "#[refusal(crate = macroonz, family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// The structural path for the body shape the declaration states.
const DECLARED_SHAPE_PATH: &str = "::macroonz::FamilyShape::SingleCause";

/// The structural path for the body shape the damaged artifact states.
const DAMAGED_SHAPE_PATH: &str = "::macroonz::FamilyShape::IssueCollection";

/// The declared spellings, in declared order.
const DECLARED_SPELLINGS: [&str; 3] = ["NotCanonical", "NotAdmitted", "Unbounded"];

/// The family every declared identity sits in.
const DECLARED_FAMILY: &str = "testpak.demo";

/// The declared local keys, in declared order.
///
/// The family is stated once beside them because the artifact states it once per
/// row, and a lane that wrote the joined name would be asserting over a value no
/// compiled constant carries.
const DECLARED_LOCAL_KEYS: [&str; 3] = ["not-canonical", "not-admitted", "unbounded"];

/// The lawful artifact's compiled seat.
///
/// The enum is the declaration's own; everything after it is materialized text,
/// compiled by `rustc` as ordinary items.
pub mod lawful_artifact {
    /// The declared family. Visible to the whole test binary and no further:
    /// the seat is a compilation site for one materialized artifact, not an
    /// export.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DemoFamily {
        /// A cause outside the machine's roster.
        NotAdmitted,
        /// A declared magnitude was exceeded.
        Unbounded,
        /// A key outside the canonical grammar.
        NotCanonical,
    }

    include!("compiled-mutant/lawful.rs");
}

/// The shape-altered artifact's compiled seat.
///
/// The same declaration, and an artifact that lies about one thing. It compiles,
/// which is exactly why a structural read is not the last word: the
/// disagreement is a VALUE, and only a compiler hands back values.
pub mod shape_altered_artifact {
    /// The declared family, unchanged — the damage is in the artifact.
    /// Visible to the whole test binary and no further, exactly as the lawful
    /// seat's is.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DemoFamily {
        /// A cause outside the machine's roster.
        NotAdmitted,
        /// A declared magnitude was exceeded.
        Unbounded,
        /// A key outside the canonical grammar.
        NotCanonical,
    }

    include!("compiled-mutant/shape-altered.rs");
}

/// The artifact one materialized fixture carries: its last line.
///
/// A materialized artifact is one line — the rendering's own projection carries
/// no newline — and the lines above it are the comment stating where the file
/// came from. Reading the last line rather than the whole file is what lets a
/// checked-in artifact explain itself without the explanation becoming part of
/// what is compared.
fn materialized_artifact(fixture: &str) -> &str {
    fixture.lines().last().unwrap_or_default()
}

/// The lawful rendering, as the bound-expansion road produced and closed over it:
/// the declaration-site cargo's projection, and empty where the road refused
/// or the declaration site was planned nothing.
fn lawful_rendering() -> String {
    compile_refusal_text(DECLARATION)
        .ok()
        .and_then(|(_, closed)| closed.inspected())
        .unwrap_or_default()
}

/// A typed call in the independently authored structural declaration.
fn structural_call(path: &str, arguments: Vec<ConstantReading>) -> ConstantReading {
    ConstantReading::Call {
        path: path.to_owned(),
        arguments,
    }
}

/// One independently authored structural reading of a declared cause.
fn structural_cause(local: &str, spelling: &str) -> ConstantReading {
    structural_call(
        "::macroonz::DeclaredCause::declared",
        vec![
            structural_call(
                "::macroonz::CauseId::declared",
                vec![
                    structural_call(
                        "::macroonz::RefusalFamilyId::declared",
                        vec![ConstantReading::Text(DECLARED_FAMILY.to_owned())],
                    ),
                    structural_call(
                        "::macroonz::LocalCauseKey::declared",
                        vec![ConstantReading::Text(local.to_owned())],
                    ),
                ],
            ),
            ConstantReading::Text(spelling.to_owned()),
        ],
    )
}

/// Compare one rendered artifact with the one independently authored declaration.
fn structural_verdict(rendered: &str, shape_path: &str) -> StructuralVerdict {
    let family_members = [DeclaredMember {
        name: "SHAPE",
        reading: ConstantReading::Path(shape_path.to_owned()),
    }];
    let declared_causes = DECLARED_LOCAL_KEYS
        .iter()
        .zip(DECLARED_SPELLINGS)
        .map(|(local, spelling)| structural_cause(local, spelling))
        .collect();
    let order_members = [DeclaredMember {
        name: "DECLARED_ORDER",
        reading: structural_call(
            "::macroonz::DeclaredCauseOrder::declared",
            vec![ConstantReading::BorrowedArray(declared_causes)],
        ),
    }];
    let selection_members = [DeclaredMember {
        name: "SELECTION_ORDER",
        reading: ConstantReading::BorrowedArray(
            DECLARED_SPELLINGS
                .iter()
                .map(|spelling| ConstantReading::Text((*spelling).to_owned()))
                .collect(),
        ),
    }];
    let implementations = [
        DeclaredImplementation {
            target: "DemoFamily",
            trait_path: Some("::macroonz::RefusalFamily"),
            postures: &[],
            attributes: &[],
            members: &family_members,
        },
        DeclaredImplementation {
            target: "DemoFamily",
            trait_path: None,
            postures: &[],
            attributes: &[],
            members: &selection_members,
        },
        DeclaredImplementation {
            target: "DemoFamily",
            trait_path: Some("::macroonz::CauseOrderDeclaration"),
            postures: &[],
            attributes: &[],
            members: &order_members,
        },
    ];
    let declared = DeclaredArtifact {
        implementations: &implementations,
    };
    match structural_decode::declarations_in(rendered) {
        Some(read) => oracle::structural::compared(&read, &declared),
        None => StructuralVerdict::Unparsable,
    }
}

/// The word for one body shape rustc handed back as a typed value.
fn observed_shape(shape: FamilyShape) -> ObservedValue {
    let word = match shape {
        FamilyShape::SingleCause => "SingleCause",
        FamilyShape::IssueCollection => "IssueCollection",
        FamilyShape::InseparablePair => "InseparablePair",
    };
    ObservedValue::Word(word.to_owned())
}

/// The cause rows one compiled declaration handed back as typed values.
fn observed_order(order: DeclaredCauseOrder) -> ObservedValue {
    ObservedValue::Series(
        order
            .iter()
            .map(|row| {
                ObservedValue::Series(vec![
                    ObservedValue::Text(row.id().family().as_declared().to_owned()),
                    ObservedValue::Text(row.id().local().as_declared().to_owned()),
                    ObservedValue::Text(row.spelling().to_owned()),
                ])
            })
            .collect(),
    )
}

/// The complete rustc-value observation for one compiled refusal artifact.
fn compiled_observation<Family>(selection_order: &[&str]) -> CompiledObservation
where
    Family: RefusalFamily + CauseOrderDeclaration,
{
    CompiledObservation::ReadBack(vec![
        ObservedMember {
            name: "SHAPE".to_owned(),
            value: observed_shape(Family::SHAPE),
        },
        ObservedMember {
            name: "DECLARED_ORDER".to_owned(),
            value: observed_order(Family::DECLARED_ORDER),
        },
        ObservedMember {
            name: "SELECTION_ORDER".to_owned(),
            value: ObservedValue::Series(
                selection_order
                    .iter()
                    .map(|spelling| ObservedValue::Text((*spelling).to_owned()))
                    .collect(),
            ),
        },
    ])
}

/// The independently authored compiled values the declaration requires.
fn declared_read_back(shape: &str) -> Vec<DeclaredReadBack<'static>> {
    let cause_rows = DECLARED_LOCAL_KEYS
        .iter()
        .zip(DECLARED_SPELLINGS)
        .map(|(local, spelling)| {
            ObservedValue::Series(vec![
                ObservedValue::Text(DECLARED_FAMILY.to_owned()),
                ObservedValue::Text((*local).to_owned()),
                ObservedValue::Text(spelling.to_owned()),
            ])
        })
        .collect();
    vec![
        DeclaredReadBack {
            name: "SHAPE",
            value: ObservedValue::Word(shape.to_owned()),
        },
        DeclaredReadBack {
            name: "DECLARED_ORDER",
            value: ObservedValue::Series(cause_rows),
        },
        DeclaredReadBack {
            name: "SELECTION_ORDER",
            value: ObservedValue::Series(
                DECLARED_SPELLINGS
                    .iter()
                    .map(|spelling| ObservedValue::Text((*spelling).to_owned()))
                    .collect(),
            ),
        },
    ]
}

/// The class and cause carried by one normalized oracle refusal.
fn refusal_signature(conclusion: &TrialConclusion) -> Option<(FailureClass, FindingCause)> {
    match conclusion {
        TrialConclusion::Passed => None,
        TrialConclusion::Refused(finding) => Some((finding.class(), finding.cause())),
    }
}

/// The lawful artifact agrees through both method-specific readings.
#[test]
fn the_lawful_artifact_conforms_structurally_and_as_compiled_values() {
    let rendered = lawful_rendering();
    let structural = structural_verdict(&rendered, DECLARED_SHAPE_PATH);
    assert_eq!(structural, StructuralVerdict::Conforms);
    assert_eq!(
        structural.concluded(FindingLocation::at(file!(), line!())),
        TrialConclusion::Passed
    );

    let observed = compiled_observation::<lawful_artifact::DemoFamily>(
        lawful_artifact::DemoFamily::SELECTION_ORDER,
    );
    let expected = declared_read_back("SingleCause");
    let compiled = oracle::compiled::compared(&observed, &DeclaredBehaviour::ReadsBack(&expected));
    assert_eq!(compiled, CompiledVerdict::Conforms);
    assert_eq!(
        compiled.concluded(FindingLocation::at(file!(), line!())),
        TrialConclusion::Passed
    );
}

/// The shape-altered artifact differs at exactly the structural and compiled SHAPE seats.
#[test]
fn the_shape_altered_artifact_reports_one_method_specific_value_difference() {
    let rendered = materialized_artifact(include_str!("compiled-mutant/shape-altered.rs"));
    let structural = structural_verdict(rendered, DECLARED_SHAPE_PATH);
    assert_eq!(
        structural,
        StructuralVerdict::Deviates(StructuralDisagreement::MemberValue {
            at: 0usize,
            member: "SHAPE".to_owned(),
        })
    );
    assert_eq!(
        refusal_signature(&structural.concluded(FindingLocation::at(file!(), line!()))),
        Some((
            FailureClass::OracleDisagreement,
            FindingCause::named(ORACLE_CAUSE_FAMILY, "structural-member-value"),
        ))
    );
    assert_eq!(
        structural_verdict(rendered, DAMAGED_SHAPE_PATH),
        StructuralVerdict::Conforms
    );

    let observed = compiled_observation::<shape_altered_artifact::DemoFamily>(
        shape_altered_artifact::DemoFamily::SELECTION_ORDER,
    );
    let expected = declared_read_back("SingleCause");
    let compiled = oracle::compiled::compared(&observed, &DeclaredBehaviour::ReadsBack(&expected));
    assert_eq!(
        compiled,
        CompiledVerdict::Deviates(CompiledDisagreement::MemberValue {
            member: "SHAPE".to_owned(),
        })
    );
    assert_eq!(
        refusal_signature(&compiled.concluded(FindingLocation::at(file!(), line!()))),
        Some((
            FailureClass::OracleDisagreement,
            FindingCause::named(ORACLE_CAUSE_FAMILY, "compiled-member-value"),
        ))
    );

    let damaged_expected = declared_read_back("IssueCollection");
    let damage_is_only_the_shape =
        oracle::compiled::compared(&observed, &DeclaredBehaviour::ReadsBack(&damaged_expected));
    assert_eq!(damage_is_only_the_shape, CompiledVerdict::Conforms);
    assert_eq!(
        damage_is_only_the_shape.concluded(FindingLocation::at(file!(), line!())),
        TrialConclusion::Passed
    );
}

/// The malformed checked-in artifact is structurally unreadable while trybuild owns rustc's refusal.
#[test]
fn the_malformed_artifact_is_structurally_unparsable() {
    let structural = structural_verdict(
        include_str!("compile-fail/a-materialized-malformed-mutant.rs"),
        DECLARED_SHAPE_PATH,
    );
    assert_eq!(structural, StructuralVerdict::Unparsable);
    assert_eq!(
        refusal_signature(&structural.concluded(FindingLocation::at(file!(), line!()))),
        Some((
            FailureClass::OracleDisagreement,
            FindingCause::named(ORACLE_CAUSE_FAMILY, "structural-unparsable"),
        ))
    );
}

/// The lawful materialized artifact is still exactly what the road produces.
///
/// The freshness check, and the reason a checked-in artifact is honest. Where
/// the renderer legitimately changes shape this fixture goes stale and this test
/// says so, by name, in one place.
///
/// It is the only fixture in this lane whose provenance is re-derived; what
/// stands behind the two damaged ones, and what does not, is this file's page.
#[test]
fn the_lawful_materialized_artifact_is_the_roads_own_output() {
    let lawful = lawful_rendering();
    assert!(
        !lawful.is_empty(),
        "the lawful artifact did not compile through the bound-expansion road"
    );
    assert_eq!(
        materialized_artifact(include_str!("compiled-mutant/lawful.rs")),
        lawful.trim_end()
    );
}

//! Shared declared values for the runner claim modules.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, DerivedRevision,
    ExecutableAttachment, ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role,
    Row, SubjectRoute, Tag, TrialTableRefusal,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FindingCause, FindingLocation, InvocationProfile, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialSite,
};
use macroonz_harness::runner::{Invocation, ReportRecordingRefusal, TrialBinding, TrialTable};
use std::fmt;
use std::io;

pub(super) const OWNER: &str = "runner-evidence";
pub(super) const SUBJECT_REFUSAL: FindingCause = FindingCause::named(OWNER, "subject-refusal");

pub(super) enum LaneFailure {
    Table(TrialTableRefusal),
    Recording(ReportRecordingRefusal),
    Io(io::Error),
    Missing(&'static str),
}

impl fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Table(refusal) => formatter.debug_tuple("Table").field(refusal).finish(),
            Self::Recording(refusal) => formatter.debug_tuple("Recording").field(refusal).finish(),
            Self::Io(refusal) => formatter.debug_tuple("Io").field(refusal).finish(),
            Self::Missing(standing) => formatter.debug_tuple("Missing").field(standing).finish(),
        }
    }
}

impl From<TrialTableRefusal> for LaneFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

impl From<ReportRecordingRefusal> for LaneFailure {
    fn from(refusal: ReportRecordingRefusal) -> Self {
        Self::Recording(refusal)
    }
}

impl From<io::Error> for LaneFailure {
    fn from(refusal: io::Error) -> Self {
        Self::Io(refusal)
    }
}

pub(super) fn passes(_: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

pub(super) fn refused() -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        macroonz_harness::report::FailureClass::RefusedByCheck,
        SUBJECT_REFUSAL,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

pub(super) fn binding(
    stem: &'static str,
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, stem)?;
    let check = CheckRef::named(OWNER, "conclusion")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "runner-preserves-evidence")?,
        ExecutionSuite::named(OWNER, "runner")?,
        Classification::authored(
            vec![Role::named(OWNER, "runner")?],
            vec![Tag::named(OWNER, stem)?],
        )?,
        subject,
        check,
        PopulationRef::named(OWNER, "declared-world")?,
        Origin::HandWritten,
    )?;
    Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check,
            RevisionBinding::derived(DerivedRevision::from_material(stem.as_bytes())),
            RevisionBinding::derived(DerivedRevision::from_material(b"runner-check-v1")),
            call,
        ),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

pub(super) fn world(bindings: Vec<TrialBinding>) -> Result<TrialTable, TrialTableRefusal> {
    TrialTable::authored(
        AuthoredTableName::named(OWNER, "runner-world")?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

pub(super) fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(64u64),
            TimeBudget::declared(1_000_000u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("runner-evidence-target"),
            ToolchainIdentity::declared("rustc-1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "runner-evidence"),
        HarnessClock::unavailable(),
    )
}

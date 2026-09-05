//! Trial construction shared by fuzz and reduction claims.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    GeneratedSupportSchemaId, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, Tag,
};
use macroonz_harness::generate::{FingerprintProbe, ReductionProbeBinding};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, Fingerprint, GenerationProfile,
    InvocationProfile, TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion,
    TrialId, TrialProfile, TrialReport, TrialSite,
};
use macroonz_harness::runner::{Invocation, run_one};

/// An inert target label for synthetic fixture observations, not a host measurement.
pub(crate) fn synthetic_target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("x86_64-pc-windows-msvc"),
        ToolchainIdentity::declared("1.98.0"),
    )
}

/// One package-local trial whose semantic names and callable remain the caller's.
pub(crate) struct TrialFixture {
    row: Row,
    site: TrialSite,
    target: TargetBinding,
}

impl TrialFixture {
    /// Declares the shared trial shape using lane-owned names and an exact caller site.
    pub(crate) fn named(
        claim: &'static str,
        execution_suite: &'static str,
        role: &'static str,
        tag: &'static str,
        population: &'static str,
        trial_site: TrialSite,
        target: TargetBinding,
    ) -> Option<Self> {
        let subject = SubjectRoute::named("harness", "byte-input").ok()?;
        let check = CheckRef::named("harness", "fingerprint-preserved").ok()?;
        let row = Row::declared(
            ClaimRef::named("harness", claim).ok()?,
            ExecutionSuite::named("harness", execution_suite).ok()?,
            Classification::authored(
                vec![Role::named("harness", role).ok()?],
                vec![Tag::named("harness", tag).ok()?],
            )
            .ok()?,
            subject,
            check,
            PopulationRef::named("harness", population).ok()?,
            Origin::HandWritten,
        )
        .ok()?;
        Some(Self {
            row,
            site: trial_site,
            target,
        })
    }

    /// Derives one failure identity from this fixture's declared trial.
    pub(crate) fn fingerprint(&self, cause: FindingCause) -> Fingerprint {
        Fingerprint::over(
            TrialId::of_key(self.row.trial_key(), TrialProfile::Unprofiled),
            cause,
            FailureClass::PropertyDisagreement,
        )
    }

    /// Runs this fixture through one lane-owned callable and shared invocation posture.
    pub(crate) fn report(
        &self,
        call: fn(&Invocation) -> TrialConclusion,
        revision: RevisionBinding,
    ) -> Option<TrialReport> {
        let subject = self.row.subject();
        let check = self.row.check();
        let binding = Binding::bound(
            self.row.clone(),
            ExecutableAttachment::attached(subject, check, revision, revision, call),
            Provenance::Unproduced,
        )
        .ok()?;
        Some(run_one(&binding, &self.invocation()))
    }

    /// Opens one reduction probe from a real report produced by this fixture.
    pub(crate) fn probe_binding(
        &self,
        call: fn(&Invocation) -> TrialConclusion,
        trial_revision: RevisionBinding,
        generation: GenerationProfile,
        schema: GeneratedSupportSchemaId,
        probe_revision: RevisionBinding,
        probe: FingerprintProbe,
    ) -> Option<ReductionProbeBinding> {
        let report = self.report(call, trial_revision)?;
        ReductionProbeBinding::bound(&report, generation, schema, probe_revision, probe).ok()
    }

    fn invocation(&self) -> Invocation {
        Invocation::declared(
            InvocationProfile::declared(
                CaseBudget::declared(1),
                ByteBudget::declared(64),
                TimeBudget::declared(1_000_000),
            ),
            self.target.clone(),
            self.site,
            HarnessClock::unavailable(),
        )
    }
}

//! Joined historical input, census and reached witnesses without current execution authority.

use super::super::{InputRun, RetentionLimits, RetentionRefusal, StoredRun};
use crate::harness::input::{BoundInput, InputBinding, InputEnvelope, InputLimits, InputProfile};
use crate::harness::report::ReplayCapsule;
use crate::harness::report::archive::{
    ArchivedAttempt, ArchivedCapsule, ArchivedConclusion, ArchivedDisposition, ArchivedRun,
};
use crate::harness::runner::{self, Invocation, ReplayedTrial, TrialBinding};
use crate::native_storage::{StorageName, StorageRoot};
use std::collections::BTreeMap;

impl InputRun {
    /// Retain the original input, complete report and explicitly supplied earned capsules in one batch.
    ///
    /// # Errors
    /// Refuses capsule joins and declared bounds before reserving storage, then preserves storage failure.
    pub fn retain(
        &self,
        root: &StorageRoot,
        name: &StorageName,
        capsules: &[ReplayCapsule],
        limits: RetentionLimits,
    ) -> Result<(), RetentionRefusal> {
        crate::workflow::retain::retain(self, root, name, capsules, limits)
    }

    /// Replace an unpublished retention attempt using this run and its declared capsules.
    ///
    /// # Errors
    /// Preserves storage recovery refusal and the same joins and bounds as retention.
    pub fn recover_retention(
        &self,
        root: &StorageRoot,
        name: &StorageName,
        capsules: &[ReplayCapsule],
        limits: RetentionLimits,
    ) -> Result<(), RetentionRefusal> {
        crate::workflow::retain::recover(self, root, name, capsules, limits)
    }
}

impl StoredRun {
    /// Load a retained batch under an independently supplied original input convention.
    ///
    /// # Errors
    /// Preserves storage and decoder-owner refusals and rejects unrelated input/report/capsule joins.
    pub fn load(
        root: &StorageRoot,
        name: &StorageName,
        profile: InputProfile,
        limits: RetentionLimits,
    ) -> Result<Self, RetentionRefusal> {
        crate::workflow::load::load(root, name, profile, limits)
    }

    pub(in crate::workflow) fn joined(
        report: ArchivedRun,
        input: InputEnvelope,
        capsules: BTreeMap<usize, ArchivedCapsule>,
    ) -> Result<Self, RetentionRefusal> {
        let standing = report.input().ok_or(RetentionRefusal::InputJoin)?;
        let profile = input.profile();
        if standing.case().as_bytes() != input.case().address().as_bytes()
            || standing.namespace() != profile.name().namespace().written()
            || standing.profile().name() != profile.name().stem().written()
            || standing.profile().version() != profile.version()
            || standing.schema().as_bytes() != profile.schema().as_bytes()
        {
            return Err(RetentionRefusal::InputJoin);
        }
        for (index, capsule) in &capsules {
            joined_capsule(&report, *index, capsule)?;
        }
        Ok(Self {
            report,
            input,
            capsules,
        })
    }

    /// The complete historical census and its source claims.
    #[must_use]
    pub const fn report(&self) -> &ArchivedRun {
        &self.report
    }

    /// The retained original specimen under the independently supplied convention.
    #[must_use]
    pub const fn input(&self) -> &InputEnvelope {
        &self.input
    }

    /// The historical reached witness retained for one original census position.
    #[must_use]
    pub fn capsule(&self, row: usize) -> Option<&ArchivedCapsule> {
        self.capsules.get(&row)
    }

    /// Execute one retained reached witness under independently supplied current bindings.
    ///
    /// # Errors
    /// Refuses a missing capsule or current input admission before execution; comparison remains in the returned runner record.
    pub fn replay<Input>(
        &self,
        row: usize,
        binding: &TrialBinding<BoundInput<Input>>,
        decoder: &InputBinding<Input>,
        invocation: Invocation,
        limits: InputLimits,
    ) -> Result<ReplayedTrial, RetentionRefusal> {
        let capsule = self.capsule(row).ok_or(RetentionRefusal::CapsuleAbsent)?;
        runner::replay(capsule, binding, decoder, invocation, limits)
            .map_err(RetentionRefusal::Input)
    }
}

fn joined_capsule(
    report: &ArchivedRun,
    index: usize,
    capsule: &ArchivedCapsule,
) -> Result<(), RetentionRefusal> {
    let row = report
        .census()
        .get(index)
        .ok_or(RetentionRefusal::CapsuleJoin)?;
    let ArchivedDisposition::Selected(trial) = row.disposition() else {
        return Err(RetentionRefusal::CapsuleJoin);
    };
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = trial.attempt() else {
        return Err(RetentionRefusal::CapsuleJoin);
    };
    if trial.key() != capsule.key()
        || finding.fingerprint().address() != capsule.fingerprint().address()
    {
        return Err(RetentionRefusal::CapsuleJoin);
    }
    Ok(())
}

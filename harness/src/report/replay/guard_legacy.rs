//! Sparse claims remain separate from complete historical reproduction.

use crate::input::InputEnvelope;
use crate::report::legacy::{LegacyField, LegacyPresence, LegacyRecord};
use crate::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, LegacyClaimRelation, LegacyJoinRefusal,
    LegacyReading,
};
use crate::report::{Fingerprint, RunAttempt, TrialConclusion, TrialReport};

fn uninterpreted<T>(presence: &LegacyPresence<T>) -> LegacyClaimRelation {
    match presence {
        LegacyPresence::Missing => LegacyClaimRelation::Missing,
        LegacyPresence::Null => LegacyClaimRelation::Null,
        LegacyPresence::Present(_) => LegacyClaimRelation::Uninterpreted,
    }
}

fn compared<T, Current>(
    historical: &LegacyPresence<T>,
    current: Option<Current>,
    same: impl FnOnce(&T, Current) -> bool,
) -> LegacyClaimRelation {
    match historical {
        LegacyPresence::Missing => LegacyClaimRelation::Missing,
        LegacyPresence::Null => LegacyClaimRelation::Null,
        LegacyPresence::Present(value) => match current {
            None => LegacyClaimRelation::CurrentUnavailable,
            Some(now) => {
                if same(value, now) {
                    LegacyClaimRelation::SameClaim
                } else {
                    LegacyClaimRelation::MovedClaim
                }
            }
        },
    }
}

fn digest(
    historical: &LegacyPresence<crate::report::archive::AddressClaim>,
    current: Option<crate::identity::ContentAddress>,
) -> LegacyClaimRelation {
    compared(historical, current, |claim, address| {
        claim.as_bytes() == address.as_bytes()
    })
}

fn uninterpreted_fields(
    historical: &LegacyRecord,
) -> std::collections::BTreeMap<LegacyField, LegacyClaimRelation> {
    std::collections::BTreeMap::from([
        (LegacyField::Kind, uninterpreted(historical.kind())),
        (LegacyField::Schema, uninterpreted(historical.schema())),
        (LegacyField::Witness, uninterpreted(historical.witness())),
        (
            LegacyField::InputProfile,
            uninterpreted(historical.input_profile()),
        ),
        (
            LegacyField::TrialName,
            uninterpreted(historical.trial_name()),
        ),
        (
            LegacyField::SubjectName,
            uninterpreted(historical.subject_name()),
        ),
        (
            LegacyField::CheckName,
            uninterpreted(historical.check_name()),
        ),
        (
            LegacyField::SubjectRevision,
            uninterpreted(historical.subject_revision()),
        ),
        (
            LegacyField::CheckRevision,
            uninterpreted(historical.check_revision()),
        ),
        (
            LegacyField::ReportedOutcome,
            uninterpreted(historical.reported_outcome()),
        ),
    ])
}

impl LegacyReading {
    pub(in crate::report::replay) fn joined(
        historical: &LegacyRecord,
        current: &TrialReport,
        witness: &InputEnvelope,
    ) -> Result<Self, LegacyJoinRefusal> {
        let saved = match historical.witness() {
            LegacyPresence::Missing => return Err(LegacyJoinRefusal::MissingWitness),
            LegacyPresence::Null => return Err(LegacyJoinRefusal::NullWitness),
            LegacyPresence::Present(bytes) => bytes,
        };
        super::joined_input(saved, current, witness).map_err(LegacyJoinRefusal::Current)?;
        let key = current.standing().key();
        let fingerprint = match current.attempt() {
            RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
                Some(Fingerprint::of(current.trial(), finding).address())
            }
            RunAttempt::Executed(TrialConclusion::Passed)
            | RunAttempt::SkippedWithReason(_)
            | RunAttempt::TimedOut
            | RunAttempt::InfrastructureFailed(_) => None,
        };
        let mut claims = uninterpreted_fields(historical);
        claims.insert(
            LegacyField::Target,
            compared(
                historical.target(),
                Some(key.target().target().spelling()),
                |value, now| value == now,
            ),
        );
        claims.insert(
            LegacyField::Toolchain,
            compared(
                historical.toolchain(),
                Some(key.target().toolchain().spelling()),
                |value, now| value == now,
            ),
        );
        claims.insert(
            LegacyField::ExecutionDigest,
            digest(historical.execution_digest(), Some(key.address())),
        );
        claims.insert(
            LegacyField::FingerprintDigest,
            digest(historical.fingerprint_digest(), fingerprint),
        );
        let (name, revision) = match historical.input_profile() {
            LegacyPresence::Present(profile) => (
                uninterpreted(profile.name()),
                uninterpreted(profile.revision()),
            ),
            LegacyPresence::Missing => (
                LegacyClaimRelation::ParentMissing,
                LegacyClaimRelation::ParentMissing,
            ),
            LegacyPresence::Null => (
                LegacyClaimRelation::ParentNull,
                LegacyClaimRelation::ParentNull,
            ),
        };
        claims.insert(LegacyField::ProfileName, name);
        claims.insert(LegacyField::ProfileRevision, revision);
        Ok(Self { claims })
    }

    /// Every supported field, preserving absence, null and uninterpreted claims.
    #[must_use]
    pub const fn claims(&self) -> &std::collections::BTreeMap<LegacyField, LegacyClaimRelation> {
        &self.claims
    }

    /// The sparse source's ceiling regardless of matching coordinate claims.
    #[must_use]
    pub const fn standing() -> HistoricalReplayStanding {
        HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::IncompleteLegacyRecord)
    }
}

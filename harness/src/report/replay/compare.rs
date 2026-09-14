//! Compare a historical witness with the current report that actually records it.

use super::{ReplayJoinRefusal, ReplayReading};
use crate::input::InputEnvelope;
use crate::report::{TrialReport, archive::ArchivedCapsule};

/// The saved witness joined to its independently earned current report.
///
/// # Errors
///
/// Refuses different witness bytes, absent current input, or a current profile or case that does not match the admitted envelope.
pub fn compare(
    historical: &ArchivedCapsule,
    current: &TrialReport,
    witness: &InputEnvelope,
) -> Result<ReplayReading, ReplayJoinRefusal> {
    ReplayReading::joined(historical, current, witness)
}

/// Read sparse historical claims against the report earned on their saved witness.
///
/// # Errors
///
/// Refuses an absent witness or a current report/envelope that does not name those saved bytes.
pub fn compare_legacy(
    historical: &crate::report::legacy::LegacyRecord,
    current: &TrialReport,
    witness: &InputEnvelope,
) -> Result<super::LegacyReading, super::LegacyJoinRefusal> {
    super::LegacyReading::joined(historical, current, witness)
}

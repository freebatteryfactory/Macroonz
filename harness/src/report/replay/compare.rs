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

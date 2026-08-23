//! The verification pass over one carrier's axes, and nothing else.
//!
//! The carried set is the quantifier. Every carried axis is examined against the
//! assembly's root and against the axes before it, so "every carried axis was
//! examined" is a fact about the loops rather than a claim about them.
//!
//! Nothing here reaches a private field: the pass reads each axis's proved cargo
//! through the same answers any caller gets, before any assembly exists. The
//! roads that CONSUME this pass live in `type_guard.rs`, because building an
//! assembly and building the refusal body are both what must stay unreachable.

use super::{AssemblyIssue, CargoAxis, ProvedCargo};
use crate::planning::ContentAddressing;

/// Every carried axis whose source terminal stands under a root other than the
/// assembly's, reported once per axis.
///
/// Both roots travel on the issue and neither is elected: which one the caller
/// meant is the caller's own fact, and a carrier composing two declarations'
/// cargo is one exported name delivering material from two places whichever one
/// was intended.
pub(super) fn root_issues(
    addressing: &ContentAddressing,
    carried: &[(CargoAxis, &ProvedCargo)],
) -> Vec<AssemblyIssue> {
    carried
        .iter()
        .filter(|(_, proved)| proved.addressing() != addressing)
        .map(|(axis, proved)| AssemblyIssue::RootsDisagree {
            axis: *axis,
            stated: Box::new(addressing.clone()),
            carried: Box::new(proved.addressing().clone()),
        })
        .collect()
}

/// Every terminal partition two axes read, reported at its SECOND occurrence.
///
/// The second rather than the first, because the first reading is the lawful one
/// and the fact being established is that something read it again: a caller
/// repairing a doubled consumption drops the later axis, and pointing at the
/// earlier one would send it to the reading it means to keep.
///
/// Counted by walking each axis against the axes before it, so the pass runs the
/// carried set to the end and one pair raises one issue rather than two.
pub(super) fn consumption_issues(carried: &[(CargoAxis, &ProvedCargo)]) -> Vec<AssemblyIssue> {
    let mut issues: Vec<AssemblyIssue> = Vec::new();
    for (position, (_, proved)) in carried.iter().enumerate() {
        let earlier = carried.iter().take(position).any(|(_, other)| {
            other.source() == proved.source() && other.partition() == proved.partition()
        });
        if earlier {
            issues.push(AssemblyIssue::CargoConsumedTwice {
                source: proved.source(),
                partition: proved.partition(),
            });
        }
    }
    issues
}

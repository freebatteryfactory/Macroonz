//! The constant answers this home's rosters settle, and the contracts a planning refusal stands under.
//!
//! Each table is total, so a row admitted later stops the compiler in every one of them until somebody says what that row's position, sentence, and classification are.
//! Nothing here decides anything: the passes that establish a planning issue live where the refusal is raised.

use super::{BoundAxis, InvalidationTrigger, PlanError, PlanIssue};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    Family, LineBody, Observed, PLANNING_FAMILY, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
};
use core::fmt;

impl BoundAxis {
    /// The position a canonical encoding writes for this axis.
    ///
    /// Appended and never renumbered: the byte stands inside every identity derived over a refusal that names it.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Declarations => 0,
            Self::Outputs => 1,
            Self::Triggers => 2,
            Self::TraceEntries => 3,
            Self::OriginEdges => 4,
        }
    }

    /// What this axis governs, as a refusal states it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::Declarations => "the captured declarations one account may name",
            Self::Outputs => "the outputs one plan may declare",
            Self::Triggers => "the triggers one plan may watch",
            Self::TraceEntries => "the entries one decision trace may record",
            Self::OriginEdges => "the edges one origin trail may draw",
        }
    }
}

impl InvalidationTrigger {
    /// The discriminant byte a canonical encoding writes ahead of what this row watches.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::CapturedDeclaration { .. } => 0,
            Self::Profile { .. } => 1,
            Self::Generator { .. } => 2,
            Self::Declared { .. } => 3,
        }
    }
}

impl PlanIssue {
    /// This row's position in the declared roster, written ahead of the issue's own material.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::ContradictoryFacts { .. } => 1,
            Self::UnknownKind { .. } => 2,
            Self::ProfileUnsupported { .. } => 3,
            Self::BoundExceeded { .. } => 4,
            Self::MembershipIncomplete { .. } => 5,
            Self::OrphanGeneratedNode { .. } => 6,
            Self::MembershipDoubled { .. } => 7,
            Self::TrailDiscontinuous { .. } => 8,
            Self::CauseSetUnwatchable { .. } => 9,
            Self::MembershipForeign { .. } => 10,
            Self::AddressInert { .. } => 11,
        }
    }

    /// How what this issue observed differs from the contract that was expected.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::ContradictoryFacts { .. }
            | Self::UnknownKind { .. }
            | Self::MembershipDoubled { .. }
            | Self::MembershipForeign { .. }
            | Self::AddressInert { .. } => Observed::ContractDisagreement,
            Self::ProfileUnsupported { .. } => Observed::ProfileDisagreement,
            Self::BoundExceeded { .. } | Self::CauseSetUnwatchable { .. } => {
                Observed::BoundExceeded
            }
            Self::MembershipIncomplete { .. } => Observed::SeatAbsent,
            Self::OrphanGeneratedNode { .. } | Self::TrailDiscontinuous { .. } => {
                Observed::OriginAbsent
            }
        }
    }
}

impl fmt::Display for PlanIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContradictoryFacts { between } => write!(
                into,
                "two facts that decided this plan disagree: {}.{} and {}.{}",
                between.left.home, between.left.name, between.right.home, between.right.name
            ),
            Self::UnknownKind { .. } => {
                into.write_str("the plan names a kind this compiler was not handed")
            }
            Self::ProfileUnsupported { profile } => write!(
                into,
                "the profile {}/{} at version {} offers no such projection",
                profile.stem(),
                profile.name(),
                profile.version().position()
            ),
            Self::BoundExceeded {
                axis,
                bound,
                observed,
            } => write!(
                into,
                "{}: {observed} offered where {bound} are declared",
                axis.described()
            ),
            Self::MembershipIncomplete { .. } => {
                into.write_str("a declared output is absent from the plan's membership")
            }
            Self::OrphanGeneratedNode { .. } => {
                into.write_str("a generated unit arrived with no origin")
            }
            Self::MembershipDoubled {
                role_slot,
                observed,
            } => write!(
                into,
                "{observed} members stand under the seat at roster position {role_slot}"
            ),
            Self::TrailDiscontinuous { at } => write!(
                into,
                "the origin trail's edge at position {at} does not start where the edge before it ended"
            ),
            Self::CauseSetUnwatchable { named, watchable } => write!(
                into,
                "the account names {named} declarations and this reading watches {watchable}"
            ),
            Self::MembershipForeign { seat } => write!(
                into,
                "the member under the seat {seat} stands outside the kind's declared roster"
            ),
            Self::AddressInert { seat } => write!(
                into,
                "an address was stated for the seat {seat}, which no publication act consumes"
            ),
        }
    }
}

impl fmt::Display for PlanError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.first_issue())?;
        let further = self.issues().count().saturating_sub(1);
        if further > 0 {
            write!(into, ", and {further} further issues")?;
        }
        if let Capping::Truncated { omitted } = self.capping() {
            write!(into, ", {omitted} of them not carried")?;
        }
        Ok(())
    }
}

impl core::error::Error for PlanError {}

impl Refused for PlanError {
    const PHASE: Phase = Phase::Planning;
    const FAMILY: Family = PLANNING_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::PlanNotStated
    }

    fn first(&self) -> String {
        self.first_issue().to_string()
    }

    fn observed(&self) -> Observed {
        self.first_issue().observed()
    }

    fn body(&self) -> LineBody {
        let further = self.issues().count().saturating_sub(1);
        let capping = self.capping();
        if further == 0 && capping == Capping::Complete {
            LineBody::SingleCause
        } else {
            LineBody::Body { further, capping }
        }
    }

    /// The issues established beyond the primary cause; the primary is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        self.issues()
            .iter()
            .skip(1)
            .map(PlanIssue::canonical_bytes)
            .collect()
    }

    /// This home declares no repair of its own.
    ///
    /// Every issue above is about what the caller's own declaration states, so the repair is that declaration; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

//! The constant tables this home's rosters settle, and the one posture a span table's answer takes.
//!
//! Nothing here computes and nothing here decides; the deciding happened where the table was asked.
//! Each table is total, so a row admitted later stops the compiler until somebody says what that row's stable name and sentence are.

use super::{
    DiagnosticNameRefusal, Observed, Phase, RefusalClass, RenderedMagnitude, SiteCoordinate,
};
use crate::token::{SourceCoordinate, SpanResolutionRefusal};

impl Phase {
    /// Every step of the road, in the order the steps run.
    pub const ALL: &'static [Self] = &[
        Self::Capture,
        Self::Planning,
        Self::Rendering,
        Self::Closure,
        Self::Explanation,
        Self::Binding,
        Self::Assembly,
    ];

    /// The step's stable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Planning => "planning",
            Self::Rendering => "rendering",
            Self::Closure => "closure",
            Self::Explanation => "explanation",
            Self::Binding => "binding",
            Self::Assembly => "assembly",
        }
    }
}

impl RefusalClass {
    /// Every class the compiler's own seams compose, in declaration order.
    ///
    /// An adopter's declared classes are its own and no roster here enumerates them.
    pub const ALL: &'static [Self] = &[
        Self::DeclarationNotRead,
        Self::PlanNotStated,
        Self::RenderingNotProduced,
        Self::RenderingNotClosed,
        Self::ExplanationNotCovered,
        Self::MagnitudeNotHeld,
        Self::ExpansionNotBound,
        Self::CarrierNotAssembled,
        Self::CarrierNotDeclared,
    ];

    /// The class's stable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::DeclarationNotRead => "declaration-not-read",
            Self::PlanNotStated => "plan-not-stated",
            Self::RenderingNotProduced => "rendering-not-produced",
            Self::RenderingNotClosed => "rendering-not-closed",
            Self::ExplanationNotCovered => "explanation-not-covered",
            Self::MagnitudeNotHeld => "magnitude-not-held",
            Self::ExpansionNotBound => "expansion-not-bound",
            Self::CarrierNotAssembled => "carrier-not-assembled",
            Self::CarrierNotDeclared => "carrier-not-declared",
            Self::Declared { name, .. } => name.spelling(),
        }
    }

    /// The second clause of every composed line.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::DeclarationNotRead => "the declaration was not read",
            Self::PlanNotStated => "planning refused",
            Self::RenderingNotProduced => "the renderer did not produce the planned unit",
            Self::RenderingNotClosed => {
                "the rendering does not close over the plan it claims to materialize"
            }
            Self::ExplanationNotCovered => "the explanation does not cover its kind's questions",
            Self::MagnitudeNotHeld => "a rendering would pass a declared magnitude",
            Self::ExpansionNotBound => "the three values do not belong to one expansion",
            Self::CarrierNotAssembled => "the closed outputs do not compose into one carrier",
            Self::CarrierNotDeclared => "the carrier's own vocabulary was not declared",
            Self::Declared { described, .. } => described,
        }
    }
}

impl Observed {
    /// Every classification the compiler's own seams state, in declaration order.
    ///
    /// An adopter's declared classifications are its own and no roster here enumerates them.
    pub const ALL: &'static [Self] = &[
        Self::SeatAbsent,
        Self::ContractDisagreement,
        Self::IdentityDisagreement,
        Self::ProfileDisagreement,
        Self::BoundExceeded,
        Self::OriginAbsent,
    ];

    /// The classification's stable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::SeatAbsent => "seat-absent",
            Self::ContractDisagreement => "contract-disagreement",
            Self::IdentityDisagreement => "identity-disagreement",
            Self::ProfileDisagreement => "profile-disagreement",
            Self::BoundExceeded => "bound-exceeded",
            Self::OriginAbsent => "origin-absent",
            Self::Declared { name, .. } => name.spelling(),
        }
    }
}

impl core::fmt::Display for DiagnosticNameRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::Empty => "a declared diagnostic name is empty",
            Self::NotKebabCase => "a declared diagnostic name is not lowercase ASCII kebab-case",
        })
    }
}

impl core::error::Error for DiagnosticNameRefusal {}

impl RenderedMagnitude {
    /// Every declared magnitude a rendering can pass, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::RenderedBytes,
        Self::RenderedUnits,
        Self::GeneratedTokens,
    ];

    /// The magnitude's stable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::RenderedBytes => "rendered-bytes",
            Self::RenderedUnits => "rendered-units",
            Self::GeneratedTokens => "generated-tokens",
        }
    }

    /// What the magnitude governs, for the line that says a rendering passed it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::RenderedBytes => "the bytes one rendered unit may carry",
            Self::RenderedUnits => "the units one rendering may carry",
            Self::GeneratedTokens => "the tokens one generated tree may carry at one nesting level",
        }
    }
}

impl SiteCoordinate {
    /// The posture one span table's answer takes.
    #[must_use]
    pub const fn answered(answer: Result<SourceCoordinate, SpanResolutionRefusal>) -> Self {
        match answer {
            Ok(coordinate) => Self::Resolved(coordinate),
            Err(refusal) => Self::NotReached(refusal),
        }
    }

    /// The resolved coordinate, where the table reached the handle.
    #[must_use]
    pub const fn resolved(self) -> Option<SourceCoordinate> {
        match self {
            Self::Resolved(coordinate) => Some(coordinate),
            Self::NotReached(_) => None,
        }
    }
}

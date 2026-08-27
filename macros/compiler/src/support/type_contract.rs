//! Shared declaration and delivery-form contracts.
use super::{DeclarationError, DeliveryForm};
use crate::bounded::Bounded;
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
    SUPPORT_DECLARATION_FAMILY,
};
use core::fmt;
impl DeliveryForm {
    /// Reads the opaque-seat clause.
    #[must_use]
    pub const fn opaque(self) -> &'static str {
        match self {
            Self::Trials => "deferred",
            Self::Benches => "reporter",
        }
    }
}
impl DeclarationError {
    /// Reads the stable canonical slot.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::EmptyNamespace => 0,
            Self::EmptyStem => 1,
            Self::SpellingNotAnIdentifier => 2,
            Self::PathSegmentsAbsent => 3,
            Self::PathSegmentsUnbounded => 4,
        }
    }
}
impl fmt::Display for DeclarationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.write_str(match self {
            Self::EmptyNamespace => "a name states no owner",
            Self::EmptyStem => "a name states no spelling",
            Self::SpellingNotAnIdentifier => "a rendered spelling is not one Rust identifier",
            Self::PathSegmentsAbsent => {
                "a rendered path names no segment past the crate it is rooted at"
            }
            Self::PathSegmentsUnbounded => {
                "a rendered path carries more segments than the declared magnitude"
            }
        })
    }
}
impl core::error::Error for DeclarationError {}
impl Refused for DeclarationError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = SUPPORT_DECLARATION_FAMILY;
    fn class(&self) -> RefusalClass {
        RefusalClass::CarrierNotDeclared
    }
    fn first(&self) -> String {
        self.to_string()
    }
    fn observed(&self) -> Observed {
        match self {
            Self::EmptyNamespace | Self::EmptyStem | Self::PathSegmentsAbsent => {
                Observed::SeatAbsent
            }
            Self::SpellingNotAnIdentifier => Observed::ContractDisagreement,
            Self::PathSegmentsUnbounded => Observed::BoundExceeded,
        }
    }
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

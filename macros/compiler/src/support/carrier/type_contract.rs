//! Carrier kind and diagnostic contracts.
use super::{ShellError, SupportCarrier};
use crate::bounded::{Bounded, Overflow};
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, RenderedMagnitude,
    Repair, SHELL_FAMILY,
};
use crate::identity::human_projection;
use crate::kind::{Kind, NoQuestions, SoleRole};
use crate::support::assembly::{ASSEMBLY_FACT, SupportAssembly};
use core::fmt;
impl Kind for SupportCarrier {
    const NAME: &'static str = "support-carrier";
    type Content = SupportAssembly;
    type Role = SoleRole;
    type Question = NoQuestions;
}
impl ShellError {
    /// Reads the stable canonical slot.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::NotOneDeclaration { .. } => 0,
            Self::TreeUnbounded { .. } => 1,
        }
    }
}
impl fmt::Display for ShellError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::NotOneDeclaration { .. } => into.write_str("the carrier's own plan stands over a declaration other than the one this assembly composed"), Self::TreeUnbounded { bound, observed } => write!(into, "the composed carrier passed {}: {observed} offered where {bound} are declared", RenderedMagnitude::GeneratedTokens.described()) }
    }
}
impl core::error::Error for ShellError {}
impl From<Overflow> for ShellError {
    fn from(overflow: Overflow) -> Self {
        Self::TreeUnbounded {
            bound: overflow.capacity,
            observed: overflow.offered,
        }
    }
}
impl Refused for ShellError {
    const PHASE: Phase = Phase::Assembly;
    const FAMILY: Family = SHELL_FAMILY;
    fn class(&self) -> RefusalClass {
        match self {
            Self::NotOneDeclaration { .. } => RefusalClass::CarrierNotAssembled,
            Self::TreeUnbounded { .. } => RefusalClass::MagnitudeNotHeld,
        }
    }
    fn first(&self) -> String {
        self.to_string()
    }
    fn observed(&self) -> Observed {
        match self {
            Self::NotOneDeclaration { .. } => Observed::IdentityDisagreement,
            Self::TreeUnbounded { .. } => Observed::BoundExceeded,
        }
    }
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        match self {
            Self::NotOneDeclaration { .. } => Bounded::from_array([Repair {
                declared_by: ASSEMBLY_FACT,
                description: human_projection!(
                    "a carrier is rendered from the plan that declares it and from the assembly composed for that same declaration, so a plan and an assembly naming two declarations are refused rather than rendered into one exported name"
                ),
            }]),
            Self::TreeUnbounded { .. } => Bounded::empty(),
        }
    }
}

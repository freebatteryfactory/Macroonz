//! The mutation home's stated tables: what the kind is, where its one unit lands, and how its grammar refuses.

use super::{MutationCaptureError, MutationSurface, Surface, SurfaceRole};
use crate::bounded::Bounded;
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
    SECOND_HELPER_FAMILY,
};
use crate::kind::{Destination, Kind, NoQuestions, Role};

impl Kind for MutationSurface {
    const NAME: &'static str = "mutation-surface";

    type Content = Surface;
    type Role = SurfaceRole;
    type Question = NoQuestions;
}

impl Role for SurfaceRole {
    const ALL: &'static [Self] = &[Self::Module];

    fn name(self) -> &'static str {
        match self {
            Self::Module => "module",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Module => Destination::TestCarrier,
        }
    }
}

impl Refused for MutationCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = SECOND_HELPER_FAMILY;

    fn class(&self) -> RefusalClass {
        self.refusal().class()
    }

    fn first(&self) -> String {
        self.refusal().first()
    }

    fn observed(&self) -> Observed {
        self.refusal().classified()
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        vec![self.refusal().canonical_bytes()]
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        self.refusal().repairs()
    }
}

impl core::fmt::Display for MutationCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for MutationCaptureError {}

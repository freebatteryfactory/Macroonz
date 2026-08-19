//! The one road from a verified assembly to a rendered carrier.
//!
//! # Why the road is here and the carrier is not
//!
//! The wall declares ONE physical carrier and it is declared in the
//! test-descriptor home, because that is the first crossing and a carrier
//! declared twice is two carriers. What is declared HERE is the composition:
//! which verified material fills which seat of that carrier, and the fact that
//! nothing else can.
//!
//! The carrier's own composition road is crate-internal and this file is its one
//! caller, so the public road to an exported shell runs through a
//! [`SupportAssembly`] that verified. A public composition road would take a
//! deferred cargo anybody can declare, and unproved tokens would cross the wall
//! through the vehicle this home exists to keep them out of.
//!
//! # One act
//!
//! The tokens are rendered from the assembly and from nothing beside it, in one
//! call, so an exported carrier's bytes are a function of the assembly the
//! caller holds. That is what the assembly's recorded source identities are
//! worth: the bytes deliver what those terminals proved, because there is no
//! road on which they could deliver anything else.

use super::{AxisCargo, SupportAssembly};
use crate::test_descriptor::{
    DeferredDelivery, DescriptorPlan, GeneratedSupportShell, ShellRendering, TrialDelivery,
};

/// Render one generated support shell over what the carrier's plan decided and
/// what the assembly verified.
///
/// # The two seats, from the two axes
///
/// The TRIAL axis fills the gate's trials seat and the EVALUATION axis fills its
/// deferred seat. An absent axis renders its seat empty rather than leaving it
/// out, so trials-only, evaluation-only, and both are one grammar with one arm.
///
/// The BENCH axis is not read, and its absence from this road is the same stated
/// opening condition the assembly refuses a carried bench axis under: the
/// published grammar writes two cargo seats and neither is the bench seat, so
/// there is nothing here for bench material to be rendered into. When that seat
/// is declared it is read here, beside the two.
///
/// # Errors
///
/// Returns the carrier's own rendering family naming the magnitude that bit,
/// exactly as the carrier's composition road returns it. Nothing is re-wrapped:
/// a token tree that outgrew its bound is the carrier's fact, and a second
/// family for it would be a second answer to one question.
pub fn assembled_shell(
    stated: &DescriptorPlan,
    assembly: &SupportAssembly,
) -> Result<GeneratedSupportShell, ShellRendering> {
    let trials = match assembly.trial() {
        AxisCargo::Absent { .. } => TrialDelivery::NothingDeclared,
        AxisCargo::Carried(payload) => TrialDelivery::Declared(payload.clone()),
    };
    let deferred = match assembly.evaluation() {
        AxisCargo::Absent { .. } => DeferredDelivery::NothingDeferred,
        AxisCargo::Carried(proved) => DeferredDelivery::Carried(proved.cargo().clone()),
    };
    GeneratedSupportShell::rendered(stated, &trials, &deferred)
}

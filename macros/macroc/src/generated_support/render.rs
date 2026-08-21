//! The one road from a carrier plan and a verified assembly to a rendered
//! carrier.
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
//!
//! # Two values arrive here, and the join between them is established here
//!
//! The road takes a CARRIER PLAN and an ASSEMBLY, and until they are compared
//! nothing in the services has ever held both. The assembly proves its cargo is
//! one declaration's; the plan says which declaration the vehicle is FOR. A
//! carrier plan for declaration B closing around declaration A's assembly agrees
//! with every reading downstream — the rendered unit is born wearing B's plan, so
//! B's key, B's origin, and B's expectation all match a tree built out of A's
//! proved cargo — and one exported name then delivers another declaration's
//! material. So the join is established at this seam, where both values exist,
//! and not in whichever caller happens to hold them.

use super::{AssemblyIssue, AxisCargo, CarrierAssembly, ShellComposition, SupportAssembly};
use crate::test_descriptor::{
    DeferredDelivery, DescriptorPlan, GeneratedSupportShell, TrialDelivery,
};

/// Render one generated support shell over what the carrier's plan decided and
/// what the assembly verified.
///
/// # The root join, first
///
/// The plan's declared root and the assembly's root are compared before a seat
/// is read, and a disagreement refuses. This is the seam at which "one carrier
/// delivers one declaration's proved cargo" stops being a claim about the
/// assembly alone: the assembly's own pass compares each carried axis against
/// the root it was handed, and a carrier plan is not an axis — it is the
/// vehicle, and nothing before this call has compared the vehicle's declaration
/// against the cargo's.
///
/// The comparison is made HERE rather than in whichever door wrapper joins the
/// two. This is the public road, so a wrapper-side check would leave the road
/// itself open to any caller holding a plan and somebody else's assembly, and
/// the shape would be a review note instead of a refusal.
///
/// Both roots travel on the refusal and neither is elected, for the reason the
/// axis-level issue states: which of the two a caller meant is the caller's own
/// fact.
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
/// Returns [`ShellComposition::NotOneDeclarations`] carrying this home's own
/// assembly body, established at
/// [`AssemblyIssue::CarrierRootIsNotTheAssemblys`] with both roots, where the
/// carrier plan's declared root is not the assembly's.
///
/// Returns [`ShellComposition::Rendering`] carrying the carrier's own rendering
/// family, naming the magnitude that bit, exactly as the carrier's composition
/// road returns it. Nothing is re-wrapped: a token tree that outgrew its bound
/// is the carrier's fact, and a second family for it would be a second answer to
/// one question — which is why the two arms carry two homes' bodies rather than
/// one roster this seam invented.
///
/// The two are DEPENDENT and in that order — a shell rendered over the wrong
/// declaration is not made lawful by fitting its bound — so exactly one of them
/// is ever established.
pub fn assembled_shell(
    stated: &DescriptorPlan,
    assembly: &SupportAssembly,
) -> Result<GeneratedSupportShell, ShellComposition> {
    // The join: this vehicle's plan and this cargo's assembly name one
    // declaration, or there is no lawful shell for the pair.
    if stated.declaration != assembly.root() {
        return Err(ShellComposition::NotOneDeclarations(
            CarrierAssembly::established(AssemblyIssue::CarrierRootIsNotTheAssemblys {
                stated: assembly.root(),
                planned: stated.declaration,
            }),
        ));
    }
    let trials = match assembly.trial() {
        AxisCargo::Absent { .. } => TrialDelivery::NothingDeclared,
        AxisCargo::Carried(payload) => TrialDelivery::Declared(payload),
    };
    let deferred = match assembly.evaluation() {
        AxisCargo::Absent { .. } => DeferredDelivery::NothingDeferred,
        AxisCargo::Carried(proved) => DeferredDelivery::Carried(proved.cargo()),
    };
    GeneratedSupportShell::rendered(stated, trials, deferred).map_err(ShellComposition::Rendering)
}

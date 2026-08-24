//! The per-seat pass, and the readings the proof takes over a rendering.
//!
//! The roster is the quantifier.
//! Every seat the kind declares is examined, in roster order, so "every applicable seat was checked" is a fact about the loop rather than a claim about it, and a seat that establishes an issue contributes no rebuilt member.
//!
//! Nothing here reaches a private field: the pass reads a rendered unit through the same answers any caller gets, and the rebuild it hands back is the renderer's own answer in the shape a plan states it.
//! The roads that consume this pass live in `type_guard.rs`, because building a closure and building a refusal body are both what must stay unreachable.

use super::ClosureIssue;
use crate::identity::{self, Identity, OwnerIdentity, Transcript};
use crate::kind::{Destination, Role};
use crate::plan::{DigestContract, Membership, PlannedMember, PlannedOutput};
use crate::render::{RenderedProjection, RenderedUnit};

/// The per-seat pass: every issue the two establish at a seat, and the members rebuilt at the seats where they agreed.
pub(super) fn examined<R: Role>(
    planned: &Membership<R>,
    rendered: &RenderedProjection<R>,
) -> (Vec<ClosureIssue<R>>, Vec<PlannedMember<R>>) {
    let mut issues: Vec<ClosureIssue<R>> = Vec::new();
    let mut rebuilt: Vec<PlannedMember<R>> = Vec::new();
    for role in R::ALL.iter().copied() {
        // What the plan declared under the seat is checked in its own right and
        // before anything is compared: a seat is declared once, so a planned
        // count of two is a defect in the plan rather than a shape the check
        // accommodates, and `under` yields the first match, which would hide it.
        let declared = planned.count_under(role);
        if declared > 1 {
            issues.push(ClosureIssue::MemberPlannedTwice {
                role,
                observed: counted(declared),
            });
            continue;
        }
        let materialized = count_under(rendered, role);
        if materialized > 1 {
            issues.push(ClosureIssue::MemberDuplicated {
                role,
                observed: counted(materialized),
            });
            continue;
        }
        match (planned.under(role), under(rendered, role)) {
            (Some(_), None) => issues.push(ClosureIssue::MemberMissing { role }),
            (None, Some(_)) => issues.push(ClosureIssue::MemberUnplanned { role }),
            (None, None) => {}
            (Some(member), Some(unit)) => match compared(member, unit, role) {
                Ok(agreed) => rebuilt.push(agreed),
                Err(issue) => issues.push(issue),
            },
        }
    }
    (issues, rebuilt)
}

/// The occupancy of the publication delivery, which is an occupancy by ADDRESS.
///
/// Two units written to one address pass every other check: they stand under different seats and carry different material, so the address is the only place they collide.
/// A unit delivered to an address with none named is the same walk's other answer, because an address nobody declared is one nothing downstream can complete.
pub(super) fn addressed<R: Role>(rendered: &RenderedProjection<R>) -> Vec<ClosureIssue<R>> {
    let mut issues: Vec<ClosureIssue<R>> = Vec::new();
    let mut taken: Vec<OwnerIdentity> = Vec::new();
    for unit in units_to(rendered, Destination::PublicationArtifact) {
        let role = unit.role();
        let Some(address) = unit.address() else {
            issues.push(ClosureIssue::ArtifactAddressAbsent { role });
            continue;
        };
        if taken.contains(&address) {
            issues.push(ClosureIssue::ArtifactAddressDoubled { role, address });
        } else {
            taken.push(address);
        }
    }
    issues
}

/// The membership row one rendered unit reconstructs — the renderer's own answer, in exactly the shape a plan states it.
fn reconstructed<R: Role>(unit: &RenderedUnit<R>) -> PlannedMember<R> {
    PlannedMember {
        role: unit.role(),
        output: PlannedOutput {
            semantic_key: unit.semantic_key(),
            origin: unit.origin().clone(),
            expected_profile: unit.profile(),
            address: unit.address(),
            digest_contract: DigestContract {
                anchored_to: unit.semantic_key(),
            },
        },
    }
}

/// The one unit rendered under a seat, where exactly one was.
pub(super) fn under<R: Role>(
    rendered: &RenderedProjection<R>,
    role: R,
) -> Option<&RenderedUnit<R>> {
    rendered.units().iter().find(|unit| unit.role() == role)
}

/// How many units were rendered under one seat.
pub(super) fn count_under<R: Role>(rendered: &RenderedProjection<R>, role: R) -> usize {
    rendered
        .units()
        .iter()
        .filter(|unit| unit.role() == role)
        .count()
}

/// Every unit this rendering materialized into one delivery, in roster order.
///
/// A unit's delivery is its seat's own constant answer, so this road elects nothing and interprets nothing.
///
/// # Ordering
///
/// Roster order, never rendering order: the roster is declared and the renderer's own sequencing is not, so what is emitted is stable under a renderer that produced its units in another order.
/// EVERY unit standing under a seat is yielded rather than the first — a rendering that doubled a seat is one the proof refuses, and a reading that quietly dropped the second unit would hide the doubling from anybody looking here instead.
pub(super) fn units_to<R: Role>(
    rendered: &RenderedProjection<R>,
    destination: Destination,
) -> impl Iterator<Item = &RenderedUnit<R>> {
    R::ALL.iter().flat_map(move |role| {
        rendered
            .units()
            .iter()
            .filter(move |unit| unit.role() == *role && role.destination() == destination)
    })
}

/// The digest recomputed from the bytes one unit actually carries, under one stated contract.
///
/// A digest that does not survive being recomputed under the plan's contract is a digest of something else.
fn digest_under<R: Role>(
    unit: &RenderedUnit<R>,
    contract: DigestContract,
) -> Identity<identity::OutputBytes> {
    let raw = unit.tree().canonical_bytes();
    Identity::derived(Transcript::under_projection(
        identity::Role::OutputBytes,
        &contract.anchored_to,
        &raw,
        u32::from(unit.role().slot()),
    ))
}

/// The seat where both sides stand: the rebuilt member, or the first way the two disagree at it.
fn compared<R: Role>(
    member: &PlannedMember<R>,
    unit: &RenderedUnit<R>,
    role: R,
) -> Result<PlannedMember<R>, ClosureIssue<R>> {
    let rebuild = reconstructed(unit);
    if rebuild.output.semantic_key != member.output.semantic_key {
        Err(ClosureIssue::SemanticKeyMismatch { role })
    } else if rebuild.output.origin != member.output.origin {
        Err(ClosureIssue::OriginOrphan { role })
    } else if digest_under(unit, member.output.digest_contract) != unit.digest() {
        Err(ClosureIssue::DigestMismatch { role })
    } else if rebuild.output.expected_profile != member.output.expected_profile
        || rebuild.output.address != member.output.address
    {
        Err(ClosureIssue::MaterializationMismatch { role })
    } else {
        Ok(rebuild)
    }
}

/// One observed count, at the width an issue carries it.
fn counted(observed: usize) -> u32 {
    u32::try_from(observed).unwrap_or(u32::MAX)
}

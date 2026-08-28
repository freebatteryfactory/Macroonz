//! Claim: Executable mutation readings and surfaces issue only from the closed discovery-lowering operation.
//! Subject: Every private discovery mint used to assemble the retained reading and executable surface.
//! Population: The point, entry, discovery reading, surface, and joined lowering constructors.
//! Hostile control: The outside fixture calls each private mint directly with otherwise well-typed inputs.
//! Denominator: All five private constructors in the discovery mint chain.
//! Evidence ceiling: This compile refusal establishes outside unwritability under Rust 1.98 only.
//! Retained regression: Any private mint becoming externally callable remains a permanent compile-refusal regression.

use macroonz_harness::descriptor::ClaimRef;
use macroonz_harness::muterprater::{
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry, EvaluationSurface,
    MutationDiscoveryReading, MutationPoint, MutationPolicy, MutationSurfaceLowering,
};

fn bypass(
    policy: &MutationPolicy,
    claim: ClaimRef,
    point_site: DiscoveredMutationSite,
    entry_site: DiscoveredMutationSite,
    point: MutationPoint,
    discovery: MutationDiscoveryReading,
    surface: EvaluationSurface,
    disposition: DiscoveryDisposition,
) {
    let _ = MutationPoint::admitted(policy, claim, point_site);
    let entry = DiscoveryEntry::recorded(entry_site, disposition);
    let _ = MutationDiscoveryReading::recorded(policy, vec![entry]);
    let _ = EvaluationSurface::admitted(policy, vec![point]);
    let _ = MutationSurfaceLowering::lowered(discovery, surface);
}

fn main() {}

//! Executable mutation points and surfaces can issue only from the closed discovery-lowering operation.
//!
//! An outside caller may author untrusted discoveries and owner policy, but it cannot mint an executable point, derived disposition row, surface, or joined lowering beside that operation.

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
    let _ = DiscoveryEntry::recorded(entry_site, disposition);
    let _ = EvaluationSurface::admitted(policy, vec![point]);
    let _ = MutationSurfaceLowering::lowered(discovery, surface);
}

fn main() {}

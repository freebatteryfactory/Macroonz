//! Complete producer discovery, lowered through one owner mutation policy.
//!
//! Every discovered site stays in the reading, in producer order.
//! Only an owner-mapped site whose whole alternative roster is permitted becomes an executable point; unmapped and unpermitted sites stay visible and cannot enter the surface.

use super::{
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry, DiscoveryLoweringRefusal,
    EvaluationSurface, MappedUnpermittedCause, MutationDiscoveryReading, MutationPoint,
    MutationPolicy, MutationSurfaceLowering, OwnerClaimMapping,
};
use crate::descriptor::MutationPointRef;
use std::collections::BTreeSet;

/// Lower one complete discovery denominator into its owner-admitted executable surface.
///
/// Admission is all-or-nothing per site: one unpermitted candidate family withholds the whole point rather than silently narrowing its alternative roster.
/// A point-free surface is lawful, and no discovered site disappears from the retained reading.
///
/// # Errors
///
/// Refuses two discovered sites stating one point identity, before admitting any point.
pub fn lower_discoveries(
    policy: &MutationPolicy,
    sites: Vec<DiscoveredMutationSite>,
) -> Result<MutationSurfaceLowering, DiscoveryLoweringRefusal> {
    let mut seen: BTreeSet<MutationPointRef> = BTreeSet::new();
    for (at, site) in sites.iter().enumerate() {
        if !seen.insert(site.identity()) {
            return Err(DiscoveryLoweringRefusal::DuplicateSite {
                at,
                point: site.identity(),
            });
        }
    }

    let mut entries = Vec::with_capacity(sites.len());
    let mut points = Vec::new();
    for site in sites {
        let disposition = admit(policy, &site, &mut points);
        entries.push(DiscoveryEntry::recorded(site, disposition));
    }

    let discovery = MutationDiscoveryReading::recorded(policy, entries);
    let surface = EvaluationSurface::admitted(policy, points);
    Ok(MutationSurfaceLowering::lowered(discovery, surface))
}

/// Read one site's disposition, pushing the admitted point where policy permits the whole roster.
fn admit(
    policy: &MutationPolicy,
    site: &DiscoveredMutationSite,
    points: &mut Vec<MutationPoint>,
) -> DiscoveryDisposition {
    let OwnerClaimMapping::Mapped(claim) = site.mapping() else {
        return DiscoveryDisposition::OwnerUnmapped;
    };
    let Some(permission) = policy.permission(claim) else {
        return DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Claim(claim),
        };
    };
    let outside = site
        .alternatives()
        .iter()
        .enumerate()
        .find(|(_, alternative)| !permission.admits(alternative.family()));
    if let Some((at, alternative)) = outside {
        return DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Family {
                at,
                family: alternative.family(),
            },
        };
    }
    let point = MutationPoint::admitted(policy, claim, site.clone());
    let identity = point.identity();
    points.push(point);
    DiscoveryDisposition::Mapped { point: identity }
}

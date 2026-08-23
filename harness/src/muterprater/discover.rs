//! Complete producer discovery lowered through one owner mutation policy.
//!
//! Every discovered site remains in the reading in producer order. Only an owner-mapped site whose complete alternative roster is permitted becomes an executable point; owner-unmapped and mapped-but-unpermitted sites remain visible and cannot enter the surface.

use super::{
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry, DiscoveryLoweringRefusal,
    EvaluationSurface, MappedUnpermittedCause, MutationDiscoveryReading, MutationPoint,
    MutationPolicy, MutationSurfaceLowering, OwnerClaimMapping,
};
use crate::descriptor::MutationPointRef;
use std::collections::BTreeSet;

/// Lower one complete discovery denominator into its owner-admitted executable surface.
///
/// Admission is all-or-nothing per site: one unpermitted candidate family withholds the whole point rather than silently narrowing its alternative roster. A point-free surface is lawful, while no discovered site disappears from the retained reading.
///
/// # Errors
///
/// Refuses two discovered sites stating one point identity before admitting any point.
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
        let disposition = match site.mapping() {
            OwnerClaimMapping::OwnerUnmapped => DiscoveryDisposition::OwnerUnmapped,
            OwnerClaimMapping::Mapped(claim) => match policy.permission(claim) {
                None => DiscoveryDisposition::MappedUnpermitted {
                    cause: MappedUnpermittedCause::Claim(claim),
                },
                Some(permission) => {
                    let outside = site
                        .alternatives()
                        .iter()
                        .enumerate()
                        .find(|(_, alternative)| !permission.admits(alternative.family()));
                    if let Some((at, alternative)) = outside {
                        DiscoveryDisposition::MappedUnpermitted {
                            cause: MappedUnpermittedCause::Family {
                                at,
                                family: alternative.family(),
                            },
                        }
                    } else {
                        let point = MutationPoint::admitted(policy, claim, site.clone());
                        let identity = point.identity();
                        points.push(point);
                        DiscoveryDisposition::Mapped { point: identity }
                    }
                }
            },
        };
        entries.push(DiscoveryEntry::recorded(site, disposition));
    }

    let discovery = MutationDiscoveryReading::recorded(policy, entries);
    let surface = EvaluationSurface::admitted(policy, points);
    Ok(MutationSurfaceLowering::lowered(discovery, surface))
}

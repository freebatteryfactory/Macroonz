//! An origin trail can be minted only by the constructor that establishes one continuous bounded walk.

use macroonz_compiler::identity::{Identity, OriginNode, Role, Transcript};
use macroonz_compiler::{NonEmpty, ORIGIN_EDGE_LIMIT, OriginEdge, OriginRelation, OriginTrail};

fn main() {
    let node: Identity<OriginNode> =
        Identity::derived(Transcript::rooted(Role::OriginNode, b"node", 0));
    let edges = NonEmpty::<OriginEdge, ORIGIN_EDGE_LIMIT>::one(OriginEdge {
        from: node,
        relation: OriginRelation::ExplicitLink,
        to: node,
    });
    let _trail = OriginTrail { edges };
}

//! The canonical bytes every value in this home contributes to a transcript.
//!
//! Written through the plane's one length framing and nothing else, so no home
//! invents a second spelling of a length.
//! Every road here reads the trail and the trace through their public walks — the
//! same answers any caller gets — so an encoding can never commit to more than a
//! reader can see.
//! The count is written ahead of the members, which is what keeps two differently
//! split walks from encoding alike.

use super::{DecisionTrace, Nonclaim, OriginEdge, OriginTrail, TraceDecision, TraceEntry};
use crate::plane::{encode_bytes, encode_length};

impl OriginEdge {
    /// Append this edge's canonical bytes: the node it starts at, the relation
    /// slot, the node it produces.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.from.as_bytes(), into);
        into.push(self.relation.slot());
        encode_bytes(self.to.as_bytes(), into);
    }
}

impl OriginTrail {
    /// Append this trail's canonical bytes: the edge count, then every edge in
    /// walk order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.count(), into);
        for edge in self.iter() {
            edge.encode_into(into);
        }
    }
}

impl TraceDecision {
    /// Append this decision's canonical bytes: the discriminant, then the cited
    /// fact where one was cited.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::SelectedBecause(cited) | Self::OmittedBecause(cited) => {
                encode_bytes(&cited.citation_bytes(), into);
            }
            Self::NotRun => encode_bytes(&[], into),
        }
    }
}

impl TraceEntry {
    /// Append this entry's canonical bytes: the subject, then the decision.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.subject.as_bytes(), into);
        self.decision.encode_into(into);
    }
}

impl DecisionTrace {
    /// Append this trace's canonical bytes: the entry count, then every entry in
    /// selection order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.count(), into);
        for entry in self.iter() {
            entry.encode_into(into);
        }
    }
}

impl Nonclaim {
    /// Append this nonclaim's canonical bytes: the unclaimed subject, then the
    /// fact that leaves it unclaimed.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.unclaimed.as_bytes(), into);
        encode_bytes(&self.because.citation_bytes(), into);
    }
}

//! The canonical bytes every value here contributes to a transcript.
//!
//! Written through the one length framing the identity home owns, and over the public readers alone, so an encoding can never commit to more than a reader is shown.
//! A count is written ahead of its members, which is what keeps two differently split walks from encoding alike.

use super::{DecisionTrace, Nonclaim, OriginEdge, OriginTrail, TraceDecision, TraceEntry};
use crate::identity::{encode_bytes, encode_length};

impl OriginEdge {
    /// Appends this edge's canonical bytes: the node it starts at, the relation slot, the node it produces.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.from.as_bytes(), into);
        into.push(self.relation.slot());
        encode_bytes(self.to.as_bytes(), into);
    }
}

impl OriginTrail {
    /// Appends this trail's canonical bytes: the edge count, then every edge in walk order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.edges().count(), into);
        for edge in self.edges() {
            edge.encode_into(into);
        }
    }
}

impl TraceDecision {
    /// Appends this decision's canonical bytes: the discriminant, then the cited fact where one was cited.
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
    /// Appends this entry's canonical bytes: the subject, then the decision.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.subject.as_bytes(), into);
        self.decision.encode_into(into);
    }
}

impl DecisionTrace {
    /// Appends this trace's canonical bytes: the entry count, then every entry in selection order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(self.entries().count(), into);
        for entry in self.entries() {
            entry.encode_into(into);
        }
    }
}

impl Nonclaim {
    /// Appends this nonclaim's canonical bytes: the unclaimed subject, then the fact that leaves it unclaimed.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.unclaimed.as_bytes(), into);
        encode_bytes(&self.because.citation_bytes(), into);
    }
}

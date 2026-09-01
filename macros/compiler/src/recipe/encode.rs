//! Canonical recipe and final-emission content.

use super::types::{ProjectionStanding, RecipeShellContent};
use super::{Recipe, RecipeRole};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Role};

impl CanonicalContent for Recipe {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.module_name().as_bytes(), into);
        encode_bytes(&self.module_head().canonical_bytes(), into);
        encode_bytes(&self.authored_body().canonical_bytes(), into);
        encode_bytes(self.states_name().as_bytes(), into);
        encode_length(self.states().count(), into);
        for member in self.states().members() {
            encode_bytes(member.spelling().as_bytes(), into);
        }
        encode_bytes(self.events_name().as_bytes(), into);
        encode_length(self.events().count(), into);
        for member in self.events().members() {
            encode_bytes(member.spelling().as_bytes(), into);
        }
        encode_length(self.transitions().count(), into);
        for transition in self.transitions().members() {
            encode_bytes(transition.from().as_bytes(), into);
            encode_bytes(transition.event().as_bytes(), into);
            encode_bytes(transition.to().as_bytes(), into);
            encode_bytes(&transition.effect().canonical_bytes(), into);
        }
        encode_bytes(self.absence().name().as_bytes(), into);
        for role in RecipeRole::ALL {
            encode_bytes(role.name().as_bytes(), into);
            match self.standing(*role) {
                ProjectionStanding::NotRequested => into.push(0),
                ProjectionStanding::Generated(lowering) => {
                    into.push(1);
                    encode_lowering(lowering, into);
                }
                ProjectionStanding::FeatureUnavailable => into.push(2),
                ProjectionStanding::TargetUnavailable => into.push(3),
            }
            encode_bytes(role.destination().name().as_bytes(), into);
            encode_evidence(self.evidence(*role), into);
        }
        match self.support() {
            None => into.push(0),
            Some(address) => {
                into.push(1);
                encode_bytes(address.spelling().as_bytes(), into);
            }
        }
    }
}

fn encode_lowering(lowering: &super::EffectiveProjection, into: &mut Vec<u8>) {
    encode_bytes(lowering.source().name().as_bytes(), into);
    encode_optional_name(lowering.name(), into);
    let Some(exact) = lowering.exact_rust() else {
        return;
    };
    encode_bytes(&exact.canonical_bytes(), into);
}

fn encode_evidence(evidence: Option<&super::RecipeEvidence>, into: &mut Vec<u8>) {
    let Some(evidence) = evidence else {
        into.push(0);
        return;
    };
    into.push(1);
    encode_optional_name(evidence.target().map(super::EvidenceTarget::name), into);
    encode_bytes(&evidence.body().canonical_bytes(), into);
}

fn encode_optional_name(name: Option<&str>, into: &mut Vec<u8>) {
    match name {
        None => into.push(0),
        Some(name) => {
            into.push(1);
            encode_bytes(name.as_bytes(), into);
        }
    }
}

impl CanonicalContent for RecipeShellContent {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.recipe.as_bytes(), into);
        match self.support {
            None => into.push(0),
            Some(support) => {
                into.push(1);
                encode_bytes(support.as_bytes(), into);
            }
        }
    }
}

//! Canonical recipe and final-emission content.

use super::types::{ProjectionStanding, RecipeRelationPayload, RecipeShellContent};
use super::{Recipe, RecipeRelationRequirements, RecipeRole, RecipeTransitionEffect};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Role};

impl CanonicalContent for Recipe {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.module_name().as_bytes(), into);
        encode_bytes(&self.module_head().canonical_bytes(), into);
        encode_bytes(&self.authored_body().canonical_bytes(), into);
        encode_length(self.vocabularies().count(), into);
        for vocabulary in self.vocabularies() {
            encode_bytes(vocabulary.name().as_bytes(), into);
            encode_length(vocabulary.members().count(), into);
            for member in vocabulary.members().members() {
                encode_bytes(member.spelling().as_bytes(), into);
            }
        }
        encode_length(self.relations().count(), into);
        for relation in self.relations() {
            encode_bytes(relation.name().as_bytes(), into);
            encode_bytes(relation.left_vocabulary().as_bytes(), into);
            encode_bytes(relation.right_vocabulary().as_bytes(), into);
            encode_bytes(relation.payload_kind().name().as_bytes(), into);
            encode_length(relation.row_count(), into);
            for row in relation.rows() {
                encode_bytes(row.left().as_bytes(), into);
                encode_bytes(row.right().as_bytes(), into);
                encode_relation_payload(row.payload(), into);
            }
            encode_relation_requirements(relation.requirements(), into);
        }
        encode_length(self.codecs().count(), into);
        for codec in self.codecs() {
            encode_bytes(codec.name().as_bytes(), into);
            encode_bytes(&codec.content().canonical_content_bytes(), into);
        }
        encode_optional_name(
            self.transition_relation().map(super::RecipeRelation::name),
            into,
        );
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

fn encode_relation_payload(payload: &RecipeRelationPayload, into: &mut Vec<u8>) {
    match payload {
        RecipeRelationPayload::Unlabeled => into.push(0),
        RecipeRelationPayload::Path(path) => {
            into.push(1);
            encode_bytes(&path.canonical_bytes(), into);
        }
        RecipeRelationPayload::ExactRust(exact) => {
            into.push(2);
            encode_bytes(&exact.canonical_bytes(), into);
        }
        RecipeRelationPayload::Transition { target, effect, .. } => {
            into.push(3);
            encode_bytes(target.as_bytes(), into);
            match effect {
                RecipeTransitionEffect::Path(path) => {
                    into.push(0);
                    encode_bytes(&path.canonical_bytes(), into);
                }
                RecipeTransitionEffect::ExactRust {
                    target_binding,
                    body,
                } => {
                    into.push(1);
                    target_binding.encode_into(into);
                    encode_bytes(&body.canonical_bytes(), into);
                }
            }
        }
    }
}

fn encode_relation_requirements(requirements: &RecipeRelationRequirements, into: &mut Vec<u8>) {
    encode_optional_name(
        requirements
            .empty()
            .map(crate::relation::EmptyPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .repetition()
            .map(crate::relation::RepetitionPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .left_membership()
            .map(crate::relation::MembershipPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .right_membership()
            .map(crate::relation::MembershipPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .left_completeness()
            .map(crate::relation::CompletenessPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .right_completeness()
            .map(crate::relation::CompletenessPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .density()
            .map(crate::relation::DensityPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .absence()
            .map(crate::relation::AbsencePosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .self_relation()
            .map(crate::relation::SelfRelationPosture::name),
        into,
    );
    encode_optional_name(
        requirements
            .cycle()
            .map(crate::relation::CyclePosture::name),
        into,
    );
}

fn encode_lowering(lowering: &super::EffectiveProjection, into: &mut Vec<u8>) {
    encode_bytes(lowering.source().name().as_bytes(), into);
    encode_optional_name(lowering.name(), into);
    encode_optional_name(lowering.subject(), into);
    match lowering.dispatch_bindings() {
        None => into.push(0),
        Some([state, event]) => {
            into.push(1);
            encode_bytes(state.as_bytes(), into);
            encode_bytes(event.as_bytes(), into);
        }
    }
    let relation_tables = lowering.relation_tables().collect::<Vec<_>>();
    encode_length(relation_tables.len(), into);
    for table in relation_tables {
        encode_bytes(table.relation().as_bytes(), into);
        encode_bytes(table.function().as_bytes(), into);
        encode_bytes(table.source().name().as_bytes(), into);
        match table.exact_rust() {
            None => into.push(0),
            Some(exact) => {
                into.push(1);
                encode_bytes(&exact.canonical_bytes(), into);
            }
        }
    }
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

//! Informed recipe construction and structural-account readback.

use super::admit::{informed_codecs, informed_members, informed_relations, informed_vocabularies};
use super::collisions::ensure_standard_names;
use super::contracts::{
    ensure_evidence_targets, ensure_projection_contracts, ensure_transition_account,
};
use super::restore::{restore_projection_references, restore_relation_references};
use super::settle::missing_vocabulary;
use super::{
    EffectiveProjection, EvidenceTarget, PROJECTION_LIMIT, ProjectionDisposition,
    ProjectionStanding, Recipe, RecipeCodec, RecipeError, RecipeEvidence, RecipeIssue,
    RecipeMember, RecipeParts, RecipeRelation, RecipeRole, VOCABULARY_LIMIT,
};
use crate::bounded::KeyedRoster;
use crate::recipe::evidence_position;
use crate::support::SupportName;
use crate::token::{GeneratedTree, SpanHandle};

impl RecipeMember {
    pub(in crate::recipe) fn authored(
        spelling: String,
        name: crate::token::GeneratedToken,
        at: SpanHandle,
    ) -> Self {
        Self { spelling, name, at }
    }

    /// Reads the authored member spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }

    /// Reads the exact ordinary or raw identifier token that names this member.
    #[must_use]
    pub const fn name_token(&self) -> &crate::token::GeneratedToken {
        &self.name
    }

    /// Reads the captured producer span for this member.
    #[must_use]
    pub(in crate::recipe) const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl EvidenceTarget {
    pub(in crate::recipe) const fn named(vocabulary: String) -> Self {
        Self { vocabulary }
    }

    /// Reads the caller-authored vocabulary name selected for evidence.
    #[must_use]
    pub fn name(&self) -> &str {
        self.vocabulary.as_str()
    }
}

impl RecipeCodec {
    pub(in crate::recipe) fn informed(
        name: String,
        content: crate::codec::CodecContent,
        at: SpanHandle,
        refusal_at: SpanHandle,
        direction_at: SpanHandle,
    ) -> Self {
        Self {
            name,
            content,
            at,
            refusal_at,
            direction_at,
        }
    }

    /// Reads the caller-owned codec declaration name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Reads the existing codec owner's informed content.
    #[must_use]
    pub const fn content(&self) -> &crate::codec::CodecContent {
        &self.content
    }

    pub(in crate::recipe) const fn at(&self) -> SpanHandle {
        self.at
    }

    pub(in crate::recipe) const fn refusal_at(&self) -> SpanHandle {
        self.refusal_at
    }

    pub(in crate::recipe) const fn direction_at(&self) -> SpanHandle {
        self.direction_at
    }
}

impl super::RecipeVocabulary {
    pub(super) fn informed(
        name: String,
        name_token: crate::token::GeneratedToken,
        members: Vec<RecipeMember>,
        at: SpanHandle,
    ) -> Result<Self, RecipeError> {
        let members = informed_members(name.as_str(), members, at)?;
        Ok(Self {
            name,
            name_token,
            members,
            at,
        })
    }

    /// Reads the caller-authored vocabulary name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Reads the exact ordinary or raw identifier token naming the vocabulary.
    #[must_use]
    pub const fn name_token(&self) -> &crate::token::GeneratedToken {
        &self.name_token
    }

    /// Reads the informed member roster in caller-authored order.
    #[must_use]
    pub const fn members(&self) -> &KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT> {
        &self.members
    }
}

impl RecipeEvidence {
    pub(in crate::recipe) const fn captured(
        role: RecipeRole,
        target: Option<EvidenceTarget>,
        body: crate::token::CapturedInput,
        at: SpanHandle,
    ) -> Self {
        Self {
            role,
            target,
            body,
            at,
        }
    }

    /// Reads the descriptor-native role this block declares.
    #[must_use]
    pub const fn role(&self) -> RecipeRole {
        self.role
    }

    /// Reads the explicitly selected vocabulary where this evidence form requires one.
    #[must_use]
    pub const fn target(&self) -> Option<&EvidenceTarget> {
        self.target.as_ref()
    }

    /// Reads the exact captured descriptor declaration body.
    #[must_use]
    pub const fn body(&self) -> &crate::token::CapturedInput {
        &self.body
    }

    pub(crate) const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl Recipe {
    pub(in crate::recipe) fn informed(offered: RecipeParts) -> Result<Self, RecipeError> {
        let RecipeParts {
            module_name,
            module_name_token,
            module_head,
            authored_body,
            authored_declaration,
            module_body_at,
            vocabularies,
            relations,
            transition_relation,
            codecs,
            mut projections,
            evidence,
            support,
        } = offered;
        let vocabularies = informed_vocabularies(vocabularies)?;
        let relations = informed_relations(
            relations,
            vocabularies.as_ref(),
            transition_relation.as_deref(),
        )?;
        let codecs = informed_codecs(codecs)?;
        ensure_evidence_targets(&evidence, vocabularies.as_ref())?;
        ensure_transition_account(
            transition_relation.as_deref(),
            relations.as_ref(),
            vocabularies.as_ref(),
        )?;
        resolve_typestate_subject(&mut projections, vocabularies.as_ref())?;
        ensure_projection_contracts(
            &projections,
            codecs.as_ref(),
            relations.as_ref(),
            transition_relation.as_deref(),
        )?;
        ensure_standard_names(
            &projections,
            vocabularies.as_ref(),
            relations.as_ref(),
            transition_relation.as_deref(),
        )?;
        Ok(Self {
            module_name,
            module_name_token,
            module_head,
            authored_body,
            authored_declaration,
            module_body_at,
            vocabularies,
            relations,
            transition_relation,
            codecs,
            projections,
            evidence,
            support,
        })
    }

    /// Reads the authored module name.
    #[must_use]
    pub fn module_name(&self) -> &str {
        self.module_name.as_str()
    }

    /// Reads the exact ordinary or raw identifier token that names the recipe module.
    #[must_use]
    pub const fn module_name_token(&self) -> &crate::token::GeneratedToken {
        &self.module_name_token
    }

    pub(in crate::recipe) const fn module_head(&self) -> &GeneratedTree {
        &self.module_head
    }

    pub(in crate::recipe) const fn authored_body(&self) -> &GeneratedTree {
        &self.authored_body
    }

    /// Restore authored reference spans onto the generated material that repeats them.
    pub(in crate::recipe) fn restore_authored_references(
        &self,
        tree: &GeneratedTree,
    ) -> GeneratedTree {
        let mut restored = tree.clone();
        for relation in self.relations() {
            restored = restore_relation_references(&restored, relation);
        }
        for role in RecipeRole::ALL.iter().copied() {
            let Some(effective) = self.effective(role) else {
                continue;
            };
            restored = restore_projection_references(&restored, effective);
        }
        restored.restored_references_from(&self.authored_declaration)
    }

    pub(in crate::recipe) const fn module_body_at(&self) -> Option<SpanHandle> {
        self.module_body_at
    }

    /// Reads every informed vocabulary in caller-authored order.
    pub fn vocabularies(&self) -> impl Iterator<Item = &super::RecipeVocabulary> {
        self.vocabularies.iter().flat_map(KeyedRoster::members)
    }

    /// Reads one informed vocabulary by its caller-authored name.
    #[must_use]
    pub fn vocabulary(&self, name: &str) -> Option<&super::RecipeVocabulary> {
        self.vocabularies
            .as_ref()
            .and_then(|vocabularies| vocabularies.get(name))
    }

    /// Reads every informed relation in caller-authored order.
    pub fn relations(&self) -> impl Iterator<Item = &RecipeRelation> {
        self.relations.iter().flat_map(KeyedRoster::members)
    }

    /// Reads one informed relation by its caller-authored name.
    #[must_use]
    pub fn relation(&self, name: &str) -> Option<&RecipeRelation> {
        self.relations
            .as_ref()
            .and_then(|relations| relations.get(name))
    }

    /// Reads the relation selected by the ergonomic transition lowering.
    #[must_use]
    pub fn transition_relation(&self) -> Option<&RecipeRelation> {
        self.transition_relation
            .as_deref()
            .and_then(|name| self.relation(name))
    }

    /// Reads every caller-named codec declaration in authored order.
    pub fn codecs(&self) -> impl Iterator<Item = &RecipeCodec> {
        self.codecs.iter().flat_map(KeyedRoster::members)
    }

    /// Reads one codec declaration by its caller-owned name.
    #[must_use]
    pub fn codec(&self, name: &str) -> Option<&RecipeCodec> {
        self.codecs.as_ref().and_then(|codecs| codecs.get(name))
    }

    pub(in crate::recipe) fn transition_account(
        &self,
    ) -> Option<(
        &super::RecipeVocabulary,
        &super::RecipeVocabulary,
        &RecipeRelation,
    )> {
        let relation = self.transition_relation()?;
        let left = self.vocabulary(relation.left_vocabulary())?;
        let right = self.vocabulary(relation.right_vocabulary())?;
        Some((left, right, relation))
    }

    /// Reads the complete standing for one projection role.
    #[must_use]
    pub(in crate::recipe) fn standing(&self, role: RecipeRole) -> &ProjectionStanding {
        role.standing(&self.projections)
    }

    /// Reads the complete public disposition of one possible projection.
    #[must_use]
    pub fn projection_disposition(&self, role: RecipeRole) -> ProjectionDisposition {
        match self.standing(role) {
            ProjectionStanding::Generated(_) => ProjectionDisposition::Generated,
            ProjectionStanding::NotRequested => ProjectionDisposition::NotRequested,
            ProjectionStanding::FeatureUnavailable => ProjectionDisposition::FeatureUnavailable,
            ProjectionStanding::TargetUnavailable => ProjectionDisposition::TargetUnavailable,
        }
    }

    /// Reads the effective mechanical configuration for one selected role.
    #[must_use]
    pub fn effective(&self, role: RecipeRole) -> Option<&EffectiveProjection> {
        match self.standing(role) {
            ProjectionStanding::Generated(effective) => Some(effective),
            ProjectionStanding::NotRequested
            | ProjectionStanding::FeatureUnavailable
            | ProjectionStanding::TargetUnavailable => None,
        }
    }

    /// Reads every generated role in declared role order.
    pub(in crate::recipe) fn selected_roles(&self) -> impl Iterator<Item = RecipeRole> + '_ {
        RecipeRole::ALL
            .iter()
            .copied()
            .filter(|role| matches!(self.standing(*role), ProjectionStanding::Generated(_)))
    }

    /// Reads the evidence carrier's explicit public address where one was declared.
    #[must_use]
    pub(crate) const fn support(&self) -> Option<&SupportName> {
        self.support.as_ref()
    }

    /// Reads the exact descriptor-native evidence block for one generated evidence role.
    #[must_use]
    pub fn evidence(&self, role: RecipeRole) -> Option<&RecipeEvidence> {
        let position = evidence_position(role)?;
        self.evidence.get(position).and_then(Option::as_ref)
    }

    pub(crate) fn baked_type_names(&self) -> Vec<String> {
        let mut names = self
            .effective(RecipeRole::RelationTables)
            .into_iter()
            .flat_map(EffectiveProjection::relation_tables)
            .map(|table| table.relation().to_owned())
            .collect::<Vec<_>>();
        if self.effective(RecipeRole::Codec).is_some() {
            names.extend(
                self.codecs()
                    .filter(|codec| codec.content().direction.reads())
                    .map(|codec| codec.content().shape.refusal().to_owned()),
            );
        }
        if self.effective(RecipeRole::Dispatch).is_some() {
            names.push("TransitionRefusal".to_owned());
        }
        if self.effective(RecipeRole::Typestate).is_some() {
            names.push("typestate".to_owned());
        }
        names
    }
}

fn resolve_typestate_subject(
    projections: &mut [ProjectionStanding; PROJECTION_LIMIT],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
) -> Result<(), RecipeError> {
    let ProjectionStanding::Generated(effective) = RecipeRole::Typestate.standing_mut(projections)
    else {
        return Ok(());
    };
    let subject = effective.subject().map(str::to_owned).or_else(|| {
        let mut candidates = vocabularies.into_iter().flat_map(KeyedRoster::members);
        let only = candidates.next()?;
        candidates.next().is_none().then(|| only.name().to_owned())
    });
    let Some(subject) = subject else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Typestate,
                expected: "one named vocabulary when several are declared",
            },
            None,
        ));
    };
    if vocabularies
        .and_then(|vocabularies| vocabularies.get(subject.as_str()))
        .is_none()
    {
        return Err(missing_vocabulary(subject.as_str(), None));
    }
    effective.select_subject(subject);
    Ok(())
}

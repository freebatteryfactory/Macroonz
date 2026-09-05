//! Informed relation construction from authored rows and vocabularies.

use super::settle::{missing_vocabulary, referenced_refusal, settle_relation_requirements};
use super::{
    RELATION_ROW_LIMIT, RecipeError, RecipeIssue, RecipeRelation, RecipeRelationParts,
    RecipeRelationPayload, RecipeRelationPayloadKind, RecipeRelationRequirements,
    RecipeRelationRow, RecipeTransitionEffect, RecipeVocabulary, RelationLowering,
    VOCABULARY_LIMIT,
};
use crate::bounded::{Bounded, KeyedRoster};
use crate::relation::{EmptyPosture, KeyedRosterRows, RepetitionPosture};
use crate::token::{GeneratedTree, SpanHandle};

impl RecipeRelationPayload {
    /// Reads the one row-payload contract this material follows.
    #[must_use]
    pub const fn kind(&self) -> RecipeRelationPayloadKind {
        match self {
            Self::Unlabeled => RecipeRelationPayloadKind::Unlabeled,
            Self::Path(_) => RecipeRelationPayloadKind::Path,
            Self::ExactRust(_) => RecipeRelationPayloadKind::ExactRust,
            Self::Transition { .. } => RecipeRelationPayloadKind::Transition,
        }
    }

    pub(in crate::recipe) fn transition(
        target: String,
        target_name: crate::token::GeneratedToken,
        effect: GeneratedTree,
    ) -> Self {
        Self::Transition {
            target,
            target_name,
            effect: RecipeTransitionEffect::Path(effect),
        }
    }

    pub(in crate::recipe) fn transition_exact(
        target: String,
        target_name: crate::token::GeneratedToken,
        target_binding: crate::token::GeneratedToken,
        body: GeneratedTree,
    ) -> Self {
        Self::Transition {
            target,
            target_name,
            effect: RecipeTransitionEffect::ExactRust {
                target_binding,
                body,
            },
        }
    }

    pub(in crate::recipe) const fn transition_parts(
        &self,
    ) -> Option<(&str, &crate::token::GeneratedToken, &RecipeTransitionEffect)> {
        match self {
            Self::Transition {
                target,
                target_name,
                effect,
            } => Some((target.as_str(), target_name, effect)),
            Self::Unlabeled | Self::Path(_) | Self::ExactRust(_) => None,
        }
    }
}

impl RecipeRelationRow {
    pub(in crate::recipe) fn authored(
        left: (String, crate::token::GeneratedToken, SpanHandle),
        right: (String, crate::token::GeneratedToken, SpanHandle),
        payload: RecipeRelationPayload,
        payload_at: SpanHandle,
        effect_binding_at: Option<SpanHandle>,
    ) -> Self {
        Self {
            left: left.0,
            left_name: left.1,
            left_at: left.2,
            right: right.0,
            right_name: right.1,
            right_at: right.2,
            payload,
            payload_at,
            effect_binding_at,
        }
    }

    /// Reads the left endpoint member spelling.
    #[must_use]
    pub fn left(&self) -> &str {
        self.left.as_str()
    }

    /// Reads the exact ordinary or raw identifier token naming the left endpoint.
    #[must_use]
    pub const fn left_name_token(&self) -> &crate::token::GeneratedToken {
        &self.left_name
    }

    /// Reads the right endpoint member spelling.
    #[must_use]
    pub fn right(&self) -> &str {
        self.right.as_str()
    }

    /// Reads the exact ordinary or raw identifier token naming the right endpoint.
    #[must_use]
    pub const fn right_name_token(&self) -> &crate::token::GeneratedToken {
        &self.right_name
    }

    /// Reads the caller-owned material attached to this relation row.
    #[must_use]
    pub const fn payload(&self) -> &RecipeRelationPayload {
        &self.payload
    }

    pub(in crate::recipe) const fn effect_binding_at(&self) -> Option<SpanHandle> {
        self.effect_binding_at
    }
}

impl RecipeRelationRequirements {
    pub(in crate::recipe) const fn unspecified() -> Self {
        Self {
            empty: None,
            repetition: None,
            membership: None,
            completeness: None,
            density: None,
            absence: None,
            self_relation: None,
            cycle: None,
        }
    }

    pub(in crate::recipe) const fn transitions(absence: crate::relation::AbsencePosture) -> Self {
        Self {
            empty: Some(EmptyPosture::Refusal),
            repetition: Some(RepetitionPosture::Refusal),
            membership: None,
            completeness: None,
            density: None,
            absence: Some(absence),
            self_relation: None,
            cycle: None,
        }
    }

    pub(in crate::recipe) fn with_empty(mut self, posture: EmptyPosture) -> Option<Self> {
        (self.empty.is_none()).then(|| {
            self.empty = Some(posture);
            self
        })
    }

    pub(in crate::recipe) fn with_repetition(mut self, posture: RepetitionPosture) -> Option<Self> {
        (self.repetition.is_none()).then(|| {
            self.repetition = Some(posture);
            self
        })
    }

    pub(in crate::recipe) fn with_membership(
        mut self,
        left: crate::relation::MembershipPosture,
        right: crate::relation::MembershipPosture,
    ) -> Option<Self> {
        self.membership.is_none().then(|| {
            self.membership = Some([left, right]);
            self
        })
    }

    pub(in crate::recipe) fn with_completeness(
        mut self,
        left: crate::relation::CompletenessPosture,
        right: crate::relation::CompletenessPosture,
    ) -> Option<Self> {
        self.completeness.is_none().then(|| {
            self.completeness = Some([left, right]);
            self
        })
    }

    pub(in crate::recipe) fn with_density(
        mut self,
        posture: crate::relation::DensityPosture,
    ) -> Option<Self> {
        (self.density.is_none()).then(|| {
            self.density = Some(posture);
            self
        })
    }

    pub(in crate::recipe) fn with_absence(
        mut self,
        posture: crate::relation::AbsencePosture,
    ) -> Option<Self> {
        (self.absence.is_none()).then(|| {
            self.absence = Some(posture);
            self
        })
    }

    pub(in crate::recipe) fn with_self_relation(
        mut self,
        posture: crate::relation::SelfRelationPosture,
    ) -> Option<Self> {
        (self.self_relation.is_none()).then(|| {
            self.self_relation = Some(posture);
            self
        })
    }

    pub(in crate::recipe) fn with_cycle(
        mut self,
        posture: crate::relation::CyclePosture,
    ) -> Option<Self> {
        (self.cycle.is_none()).then(|| {
            self.cycle = Some(posture);
            self
        })
    }

    /// Reads the declared empty-relation posture when the caller asked that question.
    #[must_use]
    pub const fn empty(&self) -> Option<EmptyPosture> {
        self.empty
    }

    /// Reads the declared repetition posture when the caller asked that question.
    #[must_use]
    pub const fn repetition(&self) -> Option<RepetitionPosture> {
        self.repetition
    }

    /// Reads the declared left-membership posture when the caller asked that question.
    #[must_use]
    pub const fn left_membership(&self) -> Option<crate::relation::MembershipPosture> {
        match self.membership {
            Some([left, _]) => Some(left),
            None => None,
        }
    }

    /// Reads the declared right-membership posture when the caller asked that question.
    #[must_use]
    pub const fn right_membership(&self) -> Option<crate::relation::MembershipPosture> {
        match self.membership {
            Some([_, right]) => Some(right),
            None => None,
        }
    }

    /// Reads the declared left-completeness posture when the caller asked that question.
    #[must_use]
    pub const fn left_completeness(&self) -> Option<crate::relation::CompletenessPosture> {
        match self.completeness {
            Some([left, _]) => Some(left),
            None => None,
        }
    }

    /// Reads the declared right-completeness posture when the caller asked that question.
    #[must_use]
    pub const fn right_completeness(&self) -> Option<crate::relation::CompletenessPosture> {
        match self.completeness {
            Some([_, right]) => Some(right),
            None => None,
        }
    }

    /// Reads the declared density posture when the caller asked that question.
    #[must_use]
    pub const fn density(&self) -> Option<crate::relation::DensityPosture> {
        self.density
    }

    /// Reads the declared absent-row posture when the caller asked that question.
    #[must_use]
    pub const fn absence(&self) -> Option<crate::relation::AbsencePosture> {
        self.absence
    }

    /// Reads the declared self-relation posture when the caller asked that question.
    #[must_use]
    pub const fn self_relation(&self) -> Option<crate::relation::SelfRelationPosture> {
        self.self_relation
    }

    /// Reads the declared cycle posture when the caller asked that question.
    #[must_use]
    pub const fn cycle(&self) -> Option<crate::relation::CyclePosture> {
        self.cycle
    }
}

impl RecipeRelation {
    pub(super) fn informed(
        parts: RecipeRelationParts,
        vocabularies: &KeyedRoster<RecipeVocabulary, String, VOCABULARY_LIMIT>,
        lowering: RelationLowering,
    ) -> Result<Self, RecipeError> {
        let RecipeRelationParts {
            name,
            name_token,
            name_at,
            left_vocabulary,
            left_vocabulary_at,
            right_vocabulary,
            right_vocabulary_at,
            rows,
            requirements,
        } = parts;
        let at = rows.first().map(|row| row.left_at);
        let left = vocabularies.get(left_vocabulary.as_str()).ok_or_else(|| {
            missing_vocabulary(left_vocabulary.as_str(), Some(left_vocabulary_at))
        })?;
        let right = vocabularies.get(right_vocabulary.as_str()).ok_or_else(|| {
            missing_vocabulary(right_vocabulary.as_str(), Some(right_vocabulary_at))
        })?;
        let informed = KeyedRosterRows::referenced(
            left.members(),
            right.members(),
            rows.clone(),
            |row| row.left.clone(),
            |row| row.right.clone(),
        )
        .map_err(|refusal| referenced_refusal(left.name(), right.name(), &rows, refusal))?;
        let payload_kind = rows
            .first()
            .map_or(RecipeRelationPayloadKind::Unlabeled, |row| {
                row.payload.kind()
            });
        if let Some(row) = rows.iter().find(|row| row.payload.kind() != payload_kind) {
            return Err(RecipeError::at(
                RecipeIssue::RelationPayloadShapeMismatch {
                    relation: name,
                    expected: payload_kind,
                    observed: row.payload.kind(),
                },
                Some(row.payload_at),
            ));
        }
        if requirements.repetition == Some(RepetitionPosture::Refusal)
            && let Err(repeated) = informed.clone().distinct()
        {
            let Some(pair) = repeated.iter().next() else {
                return Err(RecipeError::at(RecipeIssue::FragmentNotGenerated, at));
            };
            let Some(repeated_position) = pair.repeated_positions().iter().next().copied() else {
                return Err(RecipeError::at(RecipeIssue::FragmentNotGenerated, at));
            };
            let Some(row) = rows.get(repeated_position) else {
                return Err(RecipeError::at(RecipeIssue::FragmentNotGenerated, at));
            };
            let issue = if lowering == RelationLowering::Transition {
                RecipeIssue::DuplicateTransition {
                    state: row.left.clone(),
                    event: row.right.clone(),
                }
            } else {
                RecipeIssue::DuplicateRelationRow {
                    relation: name.clone(),
                    left: row.left.clone(),
                    right: row.right.clone(),
                }
            };
            return Err(RecipeError::at(issue, Some(row.left_at)));
        }
        settle_relation_requirements(name.as_str(), &informed, requirements, at)?;
        let overflow_at = rows.get(RELATION_ROW_LIMIT).map(|row| row.left_at);
        let rows = Bounded::new(rows).map_err(|overflow| {
            RecipeError::at(
                RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                    limit: overflow.capacity,
                }),
                overflow_at.or(at),
            )
        })?;
        Ok(Self {
            name,
            name_token,
            name_at,
            left_vocabulary,
            right_vocabulary,
            rows,
            payload_kind,
            requirements,
        })
    }

    /// Reads the caller-authored relation name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Reads the exact ordinary or raw identifier token naming the relation.
    #[must_use]
    pub const fn name_token(&self) -> &crate::token::GeneratedToken {
        &self.name_token
    }

    /// Reads the name of the relation's left endpoint vocabulary.
    #[must_use]
    pub fn left_vocabulary(&self) -> &str {
        self.left_vocabulary.as_str()
    }

    /// Reads the name of the relation's right endpoint vocabulary.
    #[must_use]
    pub fn right_vocabulary(&self) -> &str {
        self.right_vocabulary.as_str()
    }

    /// Reads every informed relation row in caller-authored order.
    pub fn rows(&self) -> impl Iterator<Item = &RecipeRelationRow> {
        self.rows.iter()
    }

    /// Reads how many informed rows the relation carries.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Reads the payload contract shared by every row in this relation.
    #[must_use]
    pub const fn payload_kind(&self) -> RecipeRelationPayloadKind {
        self.payload_kind
    }

    /// Reads the structural questions this relation declaration chose to answer.
    #[must_use]
    pub const fn requirements(&self) -> &RecipeRelationRequirements {
        &self.requirements
    }
}

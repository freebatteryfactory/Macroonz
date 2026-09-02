//! Informed recipe construction and structural-account readback.

use super::relation_account::{
    missing_relation, missing_vocabulary, relation_account_refusal, validate_transition_relation,
};
use super::{
    CODEC_LIMIT, EffectiveProjection, EvidenceTarget, PROJECTION_LIMIT, ProjectionDisposition,
    ProjectionStanding, RELATION_LIMIT, Recipe, RecipeCodec, RecipeError, RecipeEvidence,
    RecipeIssue, RecipeMember, RecipeParts, RecipeRelation, RecipeRelationParts,
    RecipeRelationPayloadKind, RecipeRole, RecipeVocabularyParts, RelationLowering,
    VOCABULARY_LIMIT,
};
use crate::bounded::{KeyedRoster, KeyedRosterError};
use crate::codec::{DECODE_ROAD, ENCODE_ROAD};
use crate::recipe::evidence_position;
use crate::recipe::names::{companion_constant, identifier_key};
use crate::relation::AbsencePosture;
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
    fn informed(
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

fn informed_vocabularies(
    offered: Vec<RecipeVocabularyParts>,
) -> Result<Option<KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>, RecipeError> {
    let informed = offered
        .into_iter()
        .map(|vocabulary| {
            super::RecipeVocabulary::informed(
                vocabulary.name,
                vocabulary.name_token,
                vocabulary.members,
                vocabulary.at,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    if informed.is_empty() {
        return Ok(None);
    }
    let informed_for_refusal = informed.clone();
    KeyedRoster::new(informed, |vocabulary| vocabulary.name.clone())
        .map(Some)
        .map_err(|refusal| vocabulary_account_refusal(&informed_for_refusal, refusal))
}

fn informed_relations(
    offered: Vec<RecipeRelationParts>,
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<Option<KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>, RecipeError> {
    if offered.is_empty() {
        return Ok(None);
    }
    let Some(vocabularies) = vocabularies else {
        let name = offered
            .first()
            .map_or("<relation>", |relation| relation.left_vocabulary.as_str());
        return Err(missing_vocabulary(name, None));
    };
    let relations = offered
        .into_iter()
        .map(|relation| {
            let lowering = if transition_relation == Some(relation.name.as_str()) {
                RelationLowering::Transition
            } else {
                RelationLowering::Generic
            };
            RecipeRelation::informed(relation, vocabularies, lowering)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let informed_for_refusal = relations.clone();
    KeyedRoster::new(relations, |relation| relation.name.clone())
        .map(Some)
        .map_err(|refusal| relation_account_refusal(&informed_for_refusal, refusal))
}

fn informed_codecs(
    codecs: Vec<RecipeCodec>,
) -> Result<Option<KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>, RecipeError> {
    if codecs.is_empty() {
        return Ok(None);
    }
    let offered = codecs.clone();
    KeyedRoster::new(codecs, |codec| codec.name.clone())
        .map(Some)
        .map_err(|refusal| codec_account_refusal(&offered, refusal))
}

fn ensure_evidence_targets(
    evidence: &[Option<RecipeEvidence>],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
) -> Result<(), RecipeError> {
    for declaration in evidence.iter().flatten() {
        let Some(target) = declaration.target() else {
            continue;
        };
        if vocabularies
            .and_then(|vocabularies| vocabularies.get(target.name()))
            .is_none()
        {
            return Err(RecipeError::at(
                RecipeIssue::VocabularyNotFound {
                    name: target.name().to_owned(),
                },
                Some(declaration.at()),
            ));
        }
    }
    Ok(())
}

fn ensure_transition_account(
    transition_relation: Option<&str>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
) -> Result<(), RecipeError> {
    let Some(name) = transition_relation else {
        return Ok(());
    };
    let relation = relations
        .and_then(|relations| relations.get(name))
        .ok_or_else(|| missing_relation(name))?;
    let vocabularies =
        vocabularies.ok_or_else(|| missing_vocabulary(relation.left_vocabulary(), None))?;
    validate_transition_relation(relation, vocabularies)
}

fn selected_roles(projections: &[ProjectionStanding; PROJECTION_LIMIT]) -> Vec<RecipeRole> {
    RecipeRole::ALL
        .iter()
        .copied()
        .filter(|role| matches!(role.standing(projections), ProjectionStanding::Generated(_)))
        .collect()
}

fn ensure_projection_contracts(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    codecs: Option<&KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    let selected = selected_roles(projections);
    if selected.is_empty() {
        return Err(RecipeError::at(RecipeIssue::ProjectionRequired, None));
    }
    ensure_codec_projection(&selected, codecs, projections)?;
    ensure_relation_table_projection(&selected, relations, projections)?;
    ensure_projection_dependencies(&selected)?;
    ensure_dispatch_projection(&selected, relations, transition_relation)
}

fn ensure_relation_table_projection(
    selected: &[RecipeRole],
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::RelationTables) {
        return Ok(());
    }
    let ProjectionStanding::Generated(effective) = RecipeRole::RelationTables.standing(projections)
    else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::RelationTables,
                expected: "at least one caller-named relation",
            },
            None,
        ));
    };
    let tables = effective.relation_tables().collect::<Vec<_>>();
    if tables.is_empty() {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::RelationTables,
                expected: "at least one caller-named relation",
            },
            None,
        ));
    }
    for table in tables {
        let relation = relations
            .and_then(|relations| relations.get(table.relation()))
            .ok_or_else(|| missing_relation(table.relation()))?;
        match relation.payload_kind() {
            RecipeRelationPayloadKind::Unlabeled => {}
            RecipeRelationPayloadKind::Path | RecipeRelationPayloadKind::ExactRust
                if table.exact_rust().is_some() => {}
            RecipeRelationPayloadKind::Path | RecipeRelationPayloadKind::ExactRust => {
                return Err(RecipeError::at(
                    RecipeIssue::RelationTableExactRequired {
                        relation: relation.name().to_owned(),
                    },
                    None,
                ));
            }
            RecipeRelationPayloadKind::Transition => {
                return Err(RecipeError::at(
                    RecipeIssue::RelationTableTransitionUnsupported {
                        relation: relation.name().to_owned(),
                    },
                    None,
                ));
            }
        }
    }
    Ok(())
}

fn ensure_codec_projection(
    selected: &[RecipeRole],
    codecs: Option<&KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::Codec) {
        return Ok(());
    }
    let Some(codecs) = codecs else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Codec,
                expected: "at least one existing-owner codec declaration",
            },
            None,
        ));
    };
    if let Some((name, at)) = codec_surface_collision(codecs, projections) {
        return Err(RecipeError::at(
            RecipeIssue::GeneratedNameCollision { name },
            Some(at),
        ));
    }
    Ok(())
}

fn ensure_projection_dependencies(selected: &[RecipeRole]) -> Result<(), RecipeError> {
    for role in [
        RecipeRole::CompileContract,
        RecipeRole::DeclarationConformance,
    ] {
        if selected.contains(&role) && !selected.contains(&RecipeRole::Dispatch) {
            return Err(RecipeError::at(
                RecipeIssue::ProjectionDependencyAbsent {
                    role,
                    required: RecipeRole::Dispatch,
                },
                None,
            ));
        }
    }
    Ok(())
}

fn ensure_dispatch_projection(
    selected: &[RecipeRole],
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::Dispatch) {
        return Ok(());
    }
    let Some(transition_relation) = transition_relation else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Dispatch,
                expected: "one typed transition lowering",
            },
            None,
        ));
    };
    let absence = relations
        .and_then(|relations| relations.get(transition_relation))
        .and_then(|relation| relation.requirements.absence);
    if absence == Some(AbsencePosture::Allowed) {
        return Err(RecipeError::at(
            RecipeIssue::AllowedAbsenceNeedsFallback,
            None,
        ));
    }
    Ok(())
}

fn ensure_standard_names(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    let Some((name, at)) =
        standard_name_collision(projections, vocabularies, relations, transition_relation)
    else {
        return Ok(());
    };
    Err(RecipeError::at(
        RecipeIssue::GeneratedNameCollision { name },
        Some(at),
    ))
}

fn standard_name_collision(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Option<(String, SpanHandle)> {
    let companions = matches!(
        RecipeRole::Companions.standing(projections),
        ProjectionStanding::Generated(_)
    );
    let mut names = if companions {
        companion_names(vocabularies, relations, transition_relation)
    } else {
        Vec::new()
    };
    if let ProjectionStanding::Generated(effective) =
        RecipeRole::RelationTables.standing(projections)
    {
        names.extend(
            effective
                .relation_tables()
                .map(|table| (table.relation().to_owned(), table.at())),
        );
    }
    if let ProjectionStanding::Generated(effective) = RecipeRole::Typestate.standing(projections) {
        names.push(("typestate".to_owned(), effective.at()));
    }
    for (position, (name, at)) in names.iter().enumerate() {
        if names
            .iter()
            .take(position)
            .any(|(earlier, _)| identifier_key(earlier) == identifier_key(name))
        {
            return Some((name.clone(), *at));
        }
    }
    let ProjectionStanding::Generated(dispatch) = RecipeRole::Dispatch.standing(projections) else {
        return None;
    };
    let name = dispatch.name().unwrap_or("apply");
    names
        .iter()
        .any(|(reserved, _)| identifier_key(reserved) == identifier_key(name))
        .then(|| (name.to_owned(), dispatch.at()))
}

fn codec_surface_collision(
    codecs: &KeyedRoster<RecipeCodec, String, CODEC_LIMIT>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Option<(String, SpanHandle)> {
    let declarations = codecs.members().collect::<Vec<_>>();
    let mut reserved_types = Vec::new();
    if let ProjectionStanding::Generated(effective) =
        RecipeRole::RelationTables.standing(projections)
    {
        reserved_types.extend(
            effective
                .relation_tables()
                .map(super::RelationTableProjection::relation),
        );
    }
    if matches!(
        RecipeRole::Dispatch.standing(projections),
        ProjectionStanding::Generated(_)
    ) {
        reserved_types.push("TransitionRefusal");
    }
    if matches!(
        RecipeRole::Typestate.standing(projections),
        ProjectionStanding::Generated(_)
    ) {
        reserved_types.push("typestate");
    }
    for declaration in &declarations {
        let content = declaration.content();
        let refusal = content.shape.refusal();
        if content.direction.reads()
            && reserved_types
                .iter()
                .any(|reserved| identifier_key(reserved) == identifier_key(refusal))
        {
            return Some((refusal.to_owned(), declaration.refusal_at()));
        }
    }
    for (position, first) in declarations.iter().enumerate() {
        for second in declarations.iter().skip(position.saturating_add(1)) {
            let second_refusal_at = second.refusal_at();
            let second_direction_at = second.direction_at();
            let first = first.content();
            let second = second.content();
            if first.direction.reads()
                && second.direction.reads()
                && identifier_key(first.shape.refusal()) == identifier_key(second.shape.refusal())
            {
                return Some((second.shape.refusal().to_owned(), second_refusal_at));
            }
            if let Some(road) = shared_codec_road(first, second) {
                return Some((road.to_owned(), second_direction_at));
            }
        }
    }
    None
}

fn companion_names(
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Vec<(String, SpanHandle)> {
    let mut names = vocabularies
        .into_iter()
        .flat_map(KeyedRoster::members)
        .map(|vocabulary| {
            (
                companion_constant(vocabulary.name(), "VARIANTS"),
                vocabulary.at,
            )
        })
        .collect::<Vec<_>>();
    for relation in relations.into_iter().flat_map(KeyedRoster::members) {
        if Some(relation.name()) == transition_relation {
            names.push(("TRANSITIONS".to_owned(), relation.name_at));
            continue;
        }
        names.push((
            companion_constant(relation.name(), "ROWS"),
            relation.name_at,
        ));
        if relation.payload_kind() != RecipeRelationPayloadKind::Unlabeled {
            names.push((
                companion_constant(relation.name(), "PAYLOADS"),
                relation.name_at,
            ));
        }
    }
    names
}

fn shared_codec_road(
    first: &crate::codec::CodecContent,
    second: &crate::codec::CodecContent,
) -> Option<&'static str> {
    if first.shape.owner() != second.shape.owner() {
        return None;
    }
    if first.direction.writes() && second.direction.writes() {
        return Some(ENCODE_ROAD);
    }
    (first.direction.reads() && second.direction.reads()).then_some(DECODE_ROAD)
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

fn informed_members(
    vocabulary: &str,
    members: Vec<RecipeMember>,
    vocabulary_at: SpanHandle,
) -> Result<KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>, RecipeError> {
    let offered = members.clone();
    KeyedRoster::new(members, |member| member.spelling.clone()).map_err(|refusal| match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let duplicate = duplicates.first();
            let at = offered
                .get(*duplicate.repeated_positions().first())
                .map(RecipeMember::at);
            RecipeError::at(
                RecipeIssue::DuplicateMember {
                    vocabulary: vocabulary.to_owned(),
                    member: duplicate.key().clone(),
                },
                at,
            )
        }
        KeyedRosterError::Empty(_) => RecipeError::at(
            RecipeIssue::VocabularyEmpty {
                name: vocabulary.to_owned(),
            },
            Some(vocabulary_at),
        ),
        KeyedRosterError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(RecipeMember::at)
                .or(Some(vocabulary_at)),
        ),
    })
}

fn vocabulary_account_refusal(
    offered: &[super::RecipeVocabulary],
    refusal: KeyedRosterError<String, VOCABULARY_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let name = duplicates.first().key();
            let at = offered
                .get(*duplicates.first().repeated_positions().first())
                .map(|vocabulary| vocabulary.at);
            RecipeError::at(RecipeIssue::DuplicateVocabulary { name: name.clone() }, at)
        }
        KeyedRosterError::Empty(_) => RecipeError::at(RecipeIssue::FragmentNotGenerated, None),
        KeyedRosterError::Overflow(overflow) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: overflow.capacity,
            }),
            offered
                .get(overflow.capacity)
                .map(|vocabulary| vocabulary.at),
        ),
    }
}

fn codec_account_refusal(
    offered: &[RecipeCodec],
    refusal: KeyedRosterError<String, CODEC_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::Empty(_) => RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Codec,
                expected: "at least one existing-owner codec declaration",
            },
            offered.first().map(RecipeCodec::at),
        ),
        KeyedRosterError::Overflow(_) => RecipeError::at(
            RecipeIssue::CodecDeclaration {
                name: "<recipe>".to_owned(),
                reason: format!("the codec roster exceeds its declared bound of {CODEC_LIMIT}"),
            },
            offered.first().map(RecipeCodec::at),
        ),
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let duplicate = duplicates.first();
            let at = offered
                .get(*duplicate.repeated_positions().first())
                .map(RecipeCodec::at);
            RecipeError::at(
                RecipeIssue::DuplicateCodec {
                    name: duplicate.key().clone(),
                },
                at,
            )
        }
    }
}

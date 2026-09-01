//! Recipe invariant construction and capability readers.

use super::{
    EffectiveProjection, LoweringSource, ProjectionError, ProjectionOffered, ProjectionRequest,
    ProjectionSink, ProjectionStanding, Recipe, RecipeBake, RecipeError, RecipeIssue, RecipeMember,
    RecipeParts, RecipeProjection, RecipeRole, RecipeShell, RecipeShellContent, RecipeTransition,
    RecipeView, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};
use crate::bounded::{
    AbsencePosture, KeyedRoster, KeyedRosterError, KeyedRosterRows, KeyedRosterRowsError,
};
use crate::expansion::Expansion;
use crate::kind::Role;
use crate::render::Output;
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

impl RecipeTransition {
    pub(in crate::recipe) fn authored(
        from: (String, crate::token::GeneratedToken),
        event: (String, crate::token::GeneratedToken),
        to: (String, crate::token::GeneratedToken),
        effect: GeneratedTree,
        at: SpanHandle,
    ) -> Self {
        Self {
            from: from.0,
            from_name: from.1,
            event: event.0,
            event_name: event.1,
            to: to.0,
            to_name: to.1,
            effect,
            at,
        }
    }

    /// Reads the source-state member.
    #[must_use]
    pub fn from(&self) -> &str {
        self.from.as_str()
    }

    /// Reads the exact ordinary or raw identifier token for the source state.
    #[must_use]
    pub const fn source_name_token(&self) -> &crate::token::GeneratedToken {
        &self.from_name
    }

    /// Reads the event member.
    #[must_use]
    pub fn event(&self) -> &str {
        self.event.as_str()
    }

    /// Reads the exact ordinary or raw identifier token for the event.
    #[must_use]
    pub const fn event_name_token(&self) -> &crate::token::GeneratedToken {
        &self.event_name
    }

    /// Reads the target-state member.
    #[must_use]
    pub fn to(&self) -> &str {
        self.to.as_str()
    }

    /// Reads the exact ordinary or raw identifier token for the target state.
    #[must_use]
    pub const fn target_name_token(&self) -> &crate::token::GeneratedToken {
        &self.to_name
    }

    /// Reads the exact caller-authored effect path.
    #[must_use]
    pub const fn effect(&self) -> &GeneratedTree {
        &self.effect
    }

    /// Reads the source coordinate of this row.
    #[must_use]
    pub(in crate::recipe) const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl EffectiveProjection {
    pub(in crate::recipe) fn effective(
        role: RecipeRole,
        name: Option<String>,
        source: LoweringSource,
    ) -> Self {
        Self { role, name, source }
    }

    /// Reads the selected role.
    #[must_use]
    pub const fn role(&self) -> RecipeRole {
        self.role
    }

    /// Reads the effective public spelling where this role declares one.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Reads where the effective value came from.
    #[must_use]
    pub const fn source(&self) -> LoweringSource {
        self.source
    }
}

impl Recipe {
    pub(in crate::recipe) fn informed(parts: RecipeParts) -> Result<Self, RecipeError> {
        let RecipeParts {
            module_name,
            module_name_token,
            module_head,
            authored_body,
            states_name,
            states_name_token,
            state_members,
            events_name,
            events_name_token,
            event_members,
            transitions,
            absence,
            projections,
            support,
        } = parts;
        let states = informed_members(states_name.as_str(), state_members)?;
        let events = informed_members(events_name.as_str(), event_members)?;
        let transitions =
            informed_transitions(&states_name, &states, &events_name, &events, transitions)?;
        let selected = RecipeRole::ALL
            .iter()
            .copied()
            .filter(|role| {
                matches!(
                    standing_in(&projections, *role),
                    ProjectionStanding::Generated(_)
                )
            })
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Err(RecipeError::at(RecipeIssue::ProjectionRequired, None));
        }
        for role in [RecipeRole::CompileContract, RecipeRole::Property] {
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
        if absence == AbsencePosture::Allowed && selected.contains(&RecipeRole::Dispatch) {
            return Err(RecipeError::at(
                RecipeIssue::AllowedAbsenceNeedsFallback,
                None,
            ));
        }
        Ok(Self {
            module_name,
            module_name_token,
            module_head,
            authored_body,
            states_name,
            states_name_token,
            states,
            events_name,
            events_name_token,
            events,
            transitions,
            absence,
            projections,
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

    /// Reads the state enum name.
    #[must_use]
    pub fn states_name(&self) -> &str {
        self.states_name.as_str()
    }

    /// Reads the exact ordinary or raw identifier token that names the state vocabulary.
    #[must_use]
    pub const fn states_name_token(&self) -> &crate::token::GeneratedToken {
        &self.states_name_token
    }

    /// Reads the informed state roster in authored order.
    #[must_use]
    pub const fn states(&self) -> &KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT> {
        &self.states
    }

    /// Reads the event enum name.
    #[must_use]
    pub fn events_name(&self) -> &str {
        self.events_name.as_str()
    }

    /// Reads the exact ordinary or raw identifier token that names the event vocabulary.
    #[must_use]
    pub const fn events_name_token(&self) -> &crate::token::GeneratedToken {
        &self.events_name_token
    }

    /// Reads the informed event roster in authored order.
    #[must_use]
    pub const fn events(&self) -> &KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT> {
        &self.events
    }

    /// Reads the duplicate-free transition roster in authored order.
    #[must_use]
    pub const fn transitions(
        &self,
    ) -> &KeyedRoster<RecipeTransition, (String, String), TRANSITION_LIMIT> {
        &self.transitions
    }

    /// Reads the caller's one absence posture.
    #[must_use]
    pub const fn absence(&self) -> AbsencePosture {
        self.absence
    }

    /// Reads the complete standing for one projection role.
    #[must_use]
    pub(in crate::recipe) fn standing(&self, role: RecipeRole) -> &ProjectionStanding {
        let [companions, dispatch, compile_contract, property, typestate] = &self.projections;
        match role {
            RecipeRole::Companions => companions,
            RecipeRole::Dispatch => dispatch,
            RecipeRole::CompileContract => compile_contract,
            RecipeRole::Property => property,
            RecipeRole::Typestate => typestate,
        }
    }

    /// Reads the effective mechanical configuration for one selected role.
    #[must_use]
    pub fn effective(&self, role: RecipeRole) -> Option<&EffectiveProjection> {
        match self.standing(role) {
            ProjectionStanding::Generated(effective) => Some(effective),
            ProjectionStanding::NotRequested => None,
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
    pub(in crate::recipe) const fn support(&self) -> Option<&SupportName> {
        self.support.as_ref()
    }
}

impl<'recipe> RecipeView<'recipe> {
    pub(in crate::recipe) const fn over(recipe: &'recipe Recipe) -> Self {
        Self { recipe }
    }

    /// Reads the informed recipe without plan or mutation authority.
    #[must_use]
    pub const fn recipe(self) -> &'recipe Recipe {
        self.recipe
    }
}

impl<'recipe> ProjectionRequest<'recipe> {
    pub(in crate::recipe) const fn selected(effective: &'recipe EffectiveProjection) -> Self {
        Self { effective }
    }

    /// Reads the exact selected role this invocation answers.
    #[must_use]
    pub const fn role(self) -> RecipeRole {
        self.effective.role()
    }

    /// Reads the complete effective mechanical configuration for this invocation.
    #[must_use]
    pub const fn effective(self) -> &'recipe EffectiveProjection {
        self.effective
    }

    /// Reads the destination owned by the selected role.
    #[must_use]
    pub fn destination(self) -> crate::kind::Destination {
        self.role().destination()
    }
}

impl<'output, 'plan> ProjectionSink<'output, 'plan> {
    pub(in crate::recipe) const fn bound(
        output: &'output mut Output<'plan, RecipeProjection>,
        role: RecipeRole,
    ) -> Self {
        Self { output, role }
    }

    /// Offers one tree under the exact role bound into this one-use capability.
    ///
    /// # Errors
    ///
    /// Returns the existing output refusal when the plan does not admit the role or the rendered bytes exceed their bound.
    pub fn offer(self, tree: GeneratedTree) -> Result<ProjectionOffered, ProjectionError> {
        self.output
            .unit(self.role, tree)
            .map_err(ProjectionError::Render)?;
        Ok(ProjectionOffered { _private: () })
    }
}

impl RecipeError {
    pub(in crate::recipe) const fn at(issue: RecipeIssue, at: Option<SpanHandle>) -> Self {
        Self { issue, at }
    }

    /// Reads the exact recipe issue.
    #[must_use]
    pub(in crate::recipe) const fn issue(&self) -> &RecipeIssue {
        &self.issue
    }

    /// Reads the captured producer span available for this issue.
    #[must_use]
    pub(in crate::recipe) const fn token(&self) -> Option<SpanHandle> {
        self.at
    }
}

impl RecipeBake {
    pub(in crate::recipe) const fn baked(
        projection: Expansion<RecipeProjection>,
        emitted: Expansion<RecipeShell>,
    ) -> Self {
        Self {
            projection,
            emitted,
        }
    }

    /// Reads the selected projection expansion before final module assembly.
    pub const fn projection(&self) -> &Expansion<RecipeProjection> {
        &self.projection
    }

    /// Reads the proved declaration-site cargo emitted by the paved proc host.
    pub fn emit(&self) -> &crate::closure::PartitionCargo {
        self.emitted.emit()
    }
}

impl RecipeShellContent {
    pub(in crate::recipe) const fn composed(
        recipe: crate::identity::ClosedExpansionId,
        support: Option<crate::identity::ClosedExpansionId>,
    ) -> Self {
        Self { recipe, support }
    }
}

fn standing_in(projections: &[ProjectionStanding; 5], role: RecipeRole) -> &ProjectionStanding {
    let [companions, dispatch, compile_contract, property, typestate] = projections;
    match role {
        RecipeRole::Companions => companions,
        RecipeRole::Dispatch => dispatch,
        RecipeRole::CompileContract => compile_contract,
        RecipeRole::Property => property,
        RecipeRole::Typestate => typestate,
    }
}

fn informed_members(
    vocabulary: &str,
    members: Vec<RecipeMember>,
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
        KeyedRosterError::Empty(_) | KeyedRosterError::Overflow(_) => RecipeError::at(
            RecipeIssue::VocabularyNotFound {
                name: vocabulary.to_owned(),
            },
            offered.first().map(RecipeMember::at),
        ),
    })
}

fn informed_transitions(
    states_name: &str,
    states: &KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>,
    events_name: &str,
    events: &KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>,
    transitions: Vec<RecipeTransition>,
) -> Result<KeyedRoster<RecipeTransition, (String, String), TRANSITION_LIMIT>, RecipeError> {
    let relation = KeyedRosterRows::referenced(
        states,
        events,
        transitions.clone(),
        |row| row.from.clone(),
        |row| row.event.clone(),
    )
    .map_err(|refusal| referenced_refusal(states_name, events_name, &transitions, refusal))?;
    if let Err(repeated) = relation.distinct() {
        let Some(pair) = repeated.iter().next() else {
            return Err(fragment_at_first(&transitions));
        };
        let Some(row) = transitions.get(pair.first_position()) else {
            return Err(fragment_at_first(&transitions));
        };
        return Err(RecipeError::at(
            RecipeIssue::DuplicateTransition {
                state: row.from.clone(),
                event: row.event.clone(),
            },
            Some(row.at),
        ));
    }
    if let Some(row) = transitions
        .iter()
        .find(|row| states.get(row.to.as_str()).is_none())
    {
        return Err(RecipeError::at(
            RecipeIssue::ForeignMember {
                vocabulary: states_name.to_owned(),
                member: row.to.clone(),
            },
            Some(row.at),
        ));
    }
    KeyedRoster::new(transitions, |row| (row.from.clone(), row.event.clone()))
        .map_err(transition_roster_refusal)
}

fn fragment_at_first(transitions: &[RecipeTransition]) -> RecipeError {
    RecipeError::at(
        RecipeIssue::FragmentNotGenerated,
        transitions.first().map(RecipeTransition::at),
    )
}

fn referenced_refusal(
    states: &str,
    events: &str,
    offered: &[RecipeTransition],
    refusal: KeyedRosterRowsError<String, String, TRANSITION_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterRowsError::ForeignLeft(foreign) => {
            let first = foreign.first();
            let at = offered
                .get(first.offered_position())
                .map(RecipeTransition::at);
            RecipeError::at(
                RecipeIssue::ForeignMember {
                    vocabulary: states.to_owned(),
                    member: first.key().clone(),
                },
                at,
            )
        }
        KeyedRosterRowsError::ForeignRight(foreign) => {
            let first = foreign.first();
            let at = offered
                .get(first.offered_position())
                .map(RecipeTransition::at);
            RecipeError::at(
                RecipeIssue::ForeignMember {
                    vocabulary: events.to_owned(),
                    member: first.key().clone(),
                },
                at,
            )
        }
        KeyedRosterRowsError::Overflow(_) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: TRANSITION_LIMIT,
            }),
            offered.first().map(RecipeTransition::at),
        ),
    }
}

fn transition_roster_refusal(
    refusal: KeyedRosterError<(String, String), TRANSITION_LIMIT>,
) -> RecipeError {
    match refusal {
        KeyedRosterError::DuplicateKeys(duplicates) => {
            let pair = duplicates.first().key();
            RecipeError::at(
                RecipeIssue::DuplicateTransition {
                    state: pair.0.clone(),
                    event: pair.1.clone(),
                },
                None,
            )
        }
        KeyedRosterError::Empty(_) | KeyedRosterError::Overflow(_) => RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: TRANSITION_LIMIT,
            }),
            None,
        ),
    }
}

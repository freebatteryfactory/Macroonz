//! Recipe kind, role, lowering, refusal, and host-emission contracts.

#[cfg(feature = "host")]
use super::RecipeBake;
use super::types::{RECIPE_FACT, RecipeError, RecipeIssue, RecipeShell, RecipeShellContent};
use super::{
    EvidenceTarget, HarnessPosture, LoweringSource, ProjectionDisposition, ProjectionError, Recipe,
    RecipeProjection, RecipeRole,
};
use crate::bounded::{Bounded, Overflow};
use crate::diagnostic::{LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair};
use crate::identity::human_projection;
use crate::kind::{Destination, Kind, NoQuestions, Role, SoleRole};
use crate::render::RenderError;
use core::fmt;

impl HarnessPosture {
    /// Reads the stable declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
        }
    }
}

impl LoweringSource {
    /// Reads the stable declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Preset => "preset",
            Self::Configuration => "configuration",
        }
    }
}

impl EvidenceTarget {
    /// Reads the stable declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::States => "states",
            Self::Events => "events",
        }
    }
}

impl ProjectionDisposition {
    /// Reads the stable declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::NotRequested => "not-requested",
            Self::FeatureUnavailable => "feature-unavailable",
            Self::TargetUnavailable => "target-unavailable",
        }
    }
}

impl Role for RecipeRole {
    const ALL: &'static [Self] = Self::ALL;

    fn name(self) -> &'static str {
        Self::name(self)
    }

    fn destination(self) -> Destination {
        match self {
            Self::Companions
            | Self::Dispatch
            | Self::Typestate
            | Self::Trials
            | Self::Mutation
            | Self::Benchmarks
            | Self::Network
            | Self::Concurrency => Destination::DeclarationSite,
            Self::CompileContract | Self::Property => Destination::TestCarrier,
        }
    }
}

impl Kind for RecipeProjection {
    const NAME: &'static str = "recipe-projection";
    type Content = Recipe;
    type Role = RecipeRole;
    type Question = NoQuestions;
}

impl Kind for RecipeShell {
    const NAME: &'static str = "recipe-emission";
    type Content = RecipeShellContent;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tokens(overflow) => write!(into, "{overflow}"),
            Self::Render(refusal) => write!(into, "{refusal}"),
        }
    }
}

impl core::error::Error for ProjectionError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Tokens(overflow) => Some(overflow),
            Self::Render(refusal) => Some(refusal),
        }
    }
}

impl From<Overflow> for ProjectionError {
    fn from(overflow: Overflow) -> Self {
        Self::Tokens(overflow)
    }
}

impl From<ProjectionError> for RenderError {
    fn from(refusal: ProjectionError) -> Self {
        match refusal {
            ProjectionError::Tokens(overflow) => Self::TokensUnbounded {
                bound: overflow.capacity,
                observed: overflow.offered,
            },
            ProjectionError::Render(render) => render,
        }
    }
}

impl fmt::Display for RecipeIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InlineModuleRequired => {
                into.write_str("a recipe must contain exactly one inline Rust module")
            }
            Self::BakeRequiredLast => {
                into.write_str("the recipe module must end with exactly one `bake!` declaration")
            }
            Self::GeneratedNameCollision { name } => {
                write!(into, "generated recipe name `{name}` is already occupied")
            }
            Self::GeneratedNameNotIdentifier { name } => {
                write!(
                    into,
                    "generated spelling `{name}` is not one Rust identifier"
                )
            }
            Self::Grammar(issue) => write!(into, "the recipe grammar was not read: {issue}"),
            Self::VocabularyNotFound { name } => {
                write!(into, "the recipe names no authored enum `{name}`")
            }
            Self::VariantNotUnit {
                vocabulary,
                variant,
            } => write!(
                into,
                "authored enum `{vocabulary}` variant `{variant}` is not a unit variant"
            ),
            Self::DuplicateMember { vocabulary, member } => write!(
                into,
                "authored enum `{vocabulary}` states member `{member}` more than once"
            ),
            Self::ForeignMember { vocabulary, member } => write!(
                into,
                "a transition names undeclared `{vocabulary}` member `{member}`"
            ),
            Self::DuplicateTransition { state, event } => write!(
                into,
                "more than one transition occupies state `{state}` and event `{event}`"
            ),
            Self::DuplicateProjection { role } => {
                write!(
                    into,
                    "projection `{}` is requested more than once",
                    role.name()
                )
            }
            Self::ProjectionRequired => {
                into.write_str("a recipe must request at least one projection")
            }
            Self::ProjectionDependencyAbsent { role, required } => write!(
                into,
                "projection `{}` requires selected projection `{}`",
                role.name(),
                required.name()
            ),
            Self::AllowedAbsenceNeedsFallback => into.write_str(
                "dispatch with allowed absence requires an explicit caller-owned fallback",
            ),
            Self::HarnessUnavailable { role } => write!(
                into,
                "projection `{}` requires the facade harness feature, which is unavailable",
                role.name()
            ),
            Self::SupportAddressRequired => into
                .write_str("an evidence projection requires one explicit exported support address"),
            Self::SupportAddressUnneeded => into.write_str(
                "a support address was declared although no evidence projection was requested",
            ),
            Self::ReplacementUnplanned { role } => write!(
                into,
                "a caller-owned projector was supplied for unselected role `{}`",
                role.name()
            ),
            Self::FragmentNotGenerated => into.write_str(
                "captured caller-authored Rust could not be preserved as generated tokens",
            ),
        }
    }
}

impl fmt::Display for RecipeError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.issue())
    }
}

impl core::error::Error for RecipeError {}

impl Refused for RecipeError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: crate::diagnostic::Family = super::types::RECIPE_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::DeclarationNotRead
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        match self.issue() {
            RecipeIssue::InlineModuleRequired
            | RecipeIssue::BakeRequiredLast
            | RecipeIssue::VocabularyNotFound { .. }
            | RecipeIssue::SupportAddressRequired
            | RecipeIssue::ProjectionRequired
            | RecipeIssue::ProjectionDependencyAbsent { .. } => Observed::SeatAbsent,
            RecipeIssue::HarnessUnavailable { .. } => Observed::ProfileDisagreement,
            RecipeIssue::DuplicateMember { .. }
            | RecipeIssue::DuplicateTransition { .. }
            | RecipeIssue::DuplicateProjection { .. }
            | RecipeIssue::GeneratedNameCollision { .. } => Observed::IdentityDisagreement,
            RecipeIssue::GeneratedNameNotIdentifier { .. }
            | RecipeIssue::Grammar(_)
            | RecipeIssue::VariantNotUnit { .. }
            | RecipeIssue::ForeignMember { .. }
            | RecipeIssue::AllowedAbsenceNeedsFallback
            | RecipeIssue::SupportAddressUnneeded
            | RecipeIssue::ReplacementUnplanned { .. }
            | RecipeIssue::FragmentNotGenerated => Observed::ContractDisagreement,
        }
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        let description = match self.issue() {
            RecipeIssue::HarnessUnavailable { .. } => human_projection!(
                "enable the facade harness feature or remove the harness-owned projection from this recipe"
            ),
            RecipeIssue::SupportAddressRequired | RecipeIssue::SupportAddressUnneeded => {
                human_projection!(
                    "state one support address exactly when the recipe selects test-carrier projections"
                )
            }
            RecipeIssue::ProjectionRequired | RecipeIssue::ProjectionDependencyAbsent { .. } => {
                human_projection!(
                    "select at least one projection and include every projection dependency it names"
                )
            }
            RecipeIssue::AllowedAbsenceNeedsFallback => human_projection!(
                "declare typed refusal for absent rows or state an explicit caller-owned fallback before requesting dispatch"
            ),
            RecipeIssue::DuplicateMember { .. }
            | RecipeIssue::DuplicateTransition { .. }
            | RecipeIssue::DuplicateProjection { .. }
            | RecipeIssue::GeneratedNameCollision { .. } => human_projection!(
                "state each authored member, transition seat, projection role, and generated name once"
            ),
            RecipeIssue::InlineModuleRequired
            | RecipeIssue::BakeRequiredLast
            | RecipeIssue::GeneratedNameNotIdentifier { .. }
            | RecipeIssue::Grammar(_)
            | RecipeIssue::VocabularyNotFound { .. }
            | RecipeIssue::VariantNotUnit { .. }
            | RecipeIssue::ForeignMember { .. }
            | RecipeIssue::ReplacementUnplanned { .. }
            | RecipeIssue::FragmentNotGenerated => human_projection!(
                "write one inline module whose final bake declaration names authored enum vocabularies, checked transitions, one absence posture, and every requested projection"
            ),
        };
        Bounded::from_array([Repair {
            declared_by: RECIPE_FACT,
            description,
        }])
    }
}

#[cfg(feature = "host")]
impl crate::host::Emittable for RecipeBake {
    fn cargos(&self) -> impl Iterator<Item = &crate::closure::PartitionCargo> {
        core::iter::once(self.emit())
    }
}

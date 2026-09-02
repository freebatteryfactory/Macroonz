//! Recipe kind, role, lowering, refusal, and host-emission contracts.

#[cfg(feature = "host")]
use super::RecipeBake;
use super::types::{RECIPE_FACT, RecipeError, RecipeIssue, RecipeShell, RecipeShellContent};
use super::{
    HarnessPosture, LoweringSource, ProjectionDisposition, ProjectionError, Recipe,
    RecipeProjection, RecipeRelationPayloadKind, RecipeRole,
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
            Self::ExactRust => "exact-rust",
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

impl RecipeRelationPayloadKind {
    /// Reads the stable declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Unlabeled => "unlabeled",
            Self::Path => "path",
            Self::ExactRust => "exact-rust",
            Self::Transition => "transition",
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
            | Self::RelationTables
            | Self::Dispatch
            | Self::Typestate
            | Self::Trials
            | Self::Mutation
            | Self::Benchmarks
            | Self::Network
            | Self::Concurrency
            | Self::Codec => Destination::DeclarationSite,
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
        self.write_declaration(into)
    }
}

impl RecipeIssue {
    fn write_declaration(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::DuplicateVocabulary { name } => {
                write!(
                    into,
                    "recipe vocabulary `{name}` is declared more than once"
                )
            }
            Self::ForeignMember { vocabulary, member } => write!(
                into,
                "a relation row names undeclared `{vocabulary}` member `{member}`"
            ),
            Self::DuplicateTransition { state, event } => write!(
                into,
                "more than one transition occupies state `{state}` and event `{event}`"
            ),
            Self::DuplicateRelationRow {
                relation,
                left,
                right,
            } => write!(
                into,
                "relation `{relation}` states endpoint pair `{left}` and `{right}` more than once"
            ),
            Self::DuplicateRelation { name } => {
                write!(into, "recipe relation `{name}` is declared more than once")
            }
            Self::DuplicateCodec { name } => {
                write!(into, "recipe codec `{name}` is declared more than once")
            }
            Self::CodecDeclaration { name, reason } => {
                write!(into, "recipe codec `{name}` was refused: {reason}")
            }
            Self::CodecOwnerNotRecord { codec, owner } => write!(
                into,
                "recipe codec `{codec}` owner `{owner}` is not an authored record struct"
            ),
            Self::RelationNotFound { .. }
            | Self::DuplicateRelationPosture { .. }
            | Self::DuplicateRelationQuestion { .. }
            | Self::RelationPostureMismatch { .. }
            | Self::RelationPostureInapplicable { .. }
            | Self::RelationPayloadShapeMismatch { .. }
            | Self::DuplicateProjection { .. }
            | Self::DuplicateRelationTable { .. }
            | Self::RelationTableExactRequired { .. }
            | Self::RelationTableTransitionUnsupported { .. }
            | Self::ProjectionRequired
            | Self::ProjectionDependencyAbsent { .. }
            | Self::ProjectionSubjectRequired { .. }
            | Self::AllowedAbsenceNeedsFallback
            | Self::HarnessUnavailable { .. }
            | Self::SupportAddressRequired
            | Self::SupportAddressUnneeded
            | Self::ReplacementUnplanned { .. }
            | Self::FragmentNotGenerated
            | Self::ExactDispatchFunctionRequired
            | Self::ExactDispatchBodyRefused
            | Self::ExactDispatchParameterCount { .. }
            | Self::ExactDispatchParameterBinding { .. }
            | Self::ExactRelationTableFunctionRequired
            | Self::ExactRelationTableBodyRefused
            | Self::ExactRelationTableParameterCount { .. }
            | Self::ExactRelationTableParameterBinding { .. } => self.write_relation(into),
        }
    }

    fn write_relation(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RelationNotFound { name } => {
                write!(into, "the recipe names no relation `{name}`")
            }
            Self::DuplicateRelationPosture { relation } => write!(
                into,
                "relation `{relation}` carries more than one posture block"
            ),
            Self::DuplicateRelationQuestion { relation, question } => write!(
                into,
                "relation `{relation}` answers structural question `{question}` more than once"
            ),
            Self::RelationPostureMismatch {
                relation,
                question,
                required,
                observed,
            } => write!(
                into,
                "relation `{relation}` requires {question} `{required}` but its rows compute `{observed}`"
            ),
            Self::RelationPostureInapplicable { relation, question } => write!(
                into,
                "relation `{relation}` asks same-roster question `{question}` across two vocabularies"
            ),
            Self::RelationPayloadShapeMismatch {
                relation,
                expected,
                observed,
            } => write!(
                into,
                "relation `{relation}` mixes `{}` and `{}` row payload contracts",
                expected.name(),
                observed.name()
            ),
            Self::InlineModuleRequired
            | Self::BakeRequiredLast
            | Self::GeneratedNameCollision { .. }
            | Self::GeneratedNameNotIdentifier { .. }
            | Self::Grammar(_)
            | Self::VocabularyNotFound { .. }
            | Self::VariantNotUnit { .. }
            | Self::DuplicateMember { .. }
            | Self::DuplicateVocabulary { .. }
            | Self::ForeignMember { .. }
            | Self::DuplicateTransition { .. }
            | Self::DuplicateRelationRow { .. }
            | Self::DuplicateRelation { .. }
            | Self::DuplicateCodec { .. }
            | Self::CodecDeclaration { .. }
            | Self::CodecOwnerNotRecord { .. }
            | Self::DuplicateProjection { .. }
            | Self::DuplicateRelationTable { .. }
            | Self::RelationTableExactRequired { .. }
            | Self::RelationTableTransitionUnsupported { .. }
            | Self::ProjectionRequired
            | Self::ProjectionDependencyAbsent { .. }
            | Self::ProjectionSubjectRequired { .. }
            | Self::AllowedAbsenceNeedsFallback
            | Self::HarnessUnavailable { .. }
            | Self::SupportAddressRequired
            | Self::SupportAddressUnneeded
            | Self::ReplacementUnplanned { .. }
            | Self::FragmentNotGenerated
            | Self::ExactDispatchFunctionRequired
            | Self::ExactDispatchBodyRefused
            | Self::ExactDispatchParameterCount { .. }
            | Self::ExactDispatchParameterBinding { .. }
            | Self::ExactRelationTableFunctionRequired
            | Self::ExactRelationTableBodyRefused
            | Self::ExactRelationTableParameterCount { .. }
            | Self::ExactRelationTableParameterBinding { .. } => self.write_projection(into),
        }
    }

    fn write_projection(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateProjection { role } => {
                write!(
                    into,
                    "projection `{}` is requested more than once",
                    role.name()
                )
            }
            Self::DuplicateRelationTable { relation } => write!(
                into,
                "relation table `{relation}` is requested more than once"
            ),
            Self::RelationTableExactRequired { relation } => write!(
                into,
                "relation table `{relation}` carries payloads and requires one exact Rust function signature"
            ),
            Self::RelationTableTransitionUnsupported { relation } => write!(
                into,
                "relation table `{relation}` carries transition payloads owned by the dispatch projection"
            ),
            Self::ProjectionRequired => {
                into.write_str("a recipe must request at least one projection")
            }
            Self::ProjectionDependencyAbsent { role, required } => write!(
                into,
                "projection `{}` requires selected projection `{}`",
                role.name(),
                required.name()
            ),
            Self::ProjectionSubjectRequired { role, expected } => {
                write!(into, "projection `{}` requires {expected}", role.name())
            }
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
            Self::ExactDispatchFunctionRequired
            | Self::ExactDispatchBodyRefused
            | Self::ExactDispatchParameterCount { .. }
            | Self::ExactDispatchParameterBinding { .. }
            | Self::ExactRelationTableFunctionRequired
            | Self::ExactRelationTableBodyRefused
            | Self::ExactRelationTableParameterCount { .. }
            | Self::ExactRelationTableParameterBinding { .. } => self.write_exact_projection(into),
            Self::InlineModuleRequired
            | Self::BakeRequiredLast
            | Self::GeneratedNameCollision { .. }
            | Self::GeneratedNameNotIdentifier { .. }
            | Self::Grammar(_)
            | Self::VocabularyNotFound { .. }
            | Self::VariantNotUnit { .. }
            | Self::DuplicateMember { .. }
            | Self::DuplicateVocabulary { .. }
            | Self::ForeignMember { .. }
            | Self::DuplicateTransition { .. }
            | Self::DuplicateRelationRow { .. }
            | Self::DuplicateRelation { .. }
            | Self::DuplicateCodec { .. }
            | Self::CodecDeclaration { .. }
            | Self::CodecOwnerNotRecord { .. }
            | Self::RelationNotFound { .. }
            | Self::DuplicateRelationPosture { .. }
            | Self::DuplicateRelationQuestion { .. }
            | Self::RelationPostureMismatch { .. }
            | Self::RelationPostureInapplicable { .. }
            | Self::RelationPayloadShapeMismatch { .. } => {
                unreachable!("recipe issue category must be formatted exactly once")
            }
        }
    }

    fn write_exact_projection(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExactDispatchFunctionRequired => into.write_str(
                "exact dispatch braces must contain one semicolon-terminated Rust function signature",
            ),
            Self::ExactDispatchBodyRefused => into.write_str(
                "exact dispatch cannot carry a caller-authored body because the standard projector owns the relation-accounted body",
            ),
            Self::ExactDispatchParameterCount { observed } => write!(
                into,
                "exact dispatch requires two parameters but the signature states {observed}"
            ),
            Self::ExactDispatchParameterBinding { position } => write!(
                into,
                "exact dispatch parameter {position} must use one simple identifier binding"
            ),
            Self::ExactRelationTableFunctionRequired => exact_relation_table_function(into),
            Self::ExactRelationTableBodyRefused => exact_relation_table_body(into),
            Self::ExactRelationTableParameterCount { observed } => {
                exact_relation_table_count(*observed, into)
            }
            Self::ExactRelationTableParameterBinding { position } => {
                exact_relation_table_binding(*position, into)
            }
            Self::InlineModuleRequired
            | Self::BakeRequiredLast
            | Self::GeneratedNameCollision { .. }
            | Self::GeneratedNameNotIdentifier { .. }
            | Self::Grammar(_)
            | Self::VocabularyNotFound { .. }
            | Self::VariantNotUnit { .. }
            | Self::DuplicateMember { .. }
            | Self::DuplicateVocabulary { .. }
            | Self::ForeignMember { .. }
            | Self::DuplicateTransition { .. }
            | Self::DuplicateRelationRow { .. }
            | Self::DuplicateRelation { .. }
            | Self::DuplicateCodec { .. }
            | Self::CodecDeclaration { .. }
            | Self::CodecOwnerNotRecord { .. }
            | Self::RelationNotFound { .. }
            | Self::DuplicateRelationPosture { .. }
            | Self::DuplicateRelationQuestion { .. }
            | Self::RelationPostureMismatch { .. }
            | Self::RelationPostureInapplicable { .. }
            | Self::RelationPayloadShapeMismatch { .. }
            | Self::DuplicateProjection { .. }
            | Self::DuplicateRelationTable { .. }
            | Self::RelationTableExactRequired { .. }
            | Self::RelationTableTransitionUnsupported { .. }
            | Self::ProjectionRequired
            | Self::ProjectionDependencyAbsent { .. }
            | Self::ProjectionSubjectRequired { .. }
            | Self::AllowedAbsenceNeedsFallback
            | Self::HarnessUnavailable { .. }
            | Self::SupportAddressRequired
            | Self::SupportAddressUnneeded
            | Self::ReplacementUnplanned { .. }
            | Self::FragmentNotGenerated => {
                unreachable!("recipe issue category must be formatted exactly once")
            }
        }
    }
}

fn exact_relation_table_function(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str(
        "exact relation-table braces must contain one semicolon-terminated Rust function signature",
    )
}

fn exact_relation_table_body(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str(
        "an exact relation table cannot carry a caller-authored body because the standard projector owns the row-accounted body",
    )
}

fn exact_relation_table_count(observed: usize, into: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        into,
        "an exact relation table requires two parameters but the signature states {observed}"
    )
}

fn exact_relation_table_binding(position: usize, into: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        into,
        "exact relation-table parameter {position} must use one simple identifier binding"
    )
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
            | RecipeIssue::RelationNotFound { .. }
            | RecipeIssue::SupportAddressRequired
            | RecipeIssue::ProjectionRequired
            | RecipeIssue::ProjectionDependencyAbsent { .. }
            | RecipeIssue::ProjectionSubjectRequired { .. } => Observed::SeatAbsent,
            RecipeIssue::HarnessUnavailable { .. } => Observed::ProfileDisagreement,
            RecipeIssue::DuplicateMember { .. }
            | RecipeIssue::DuplicateVocabulary { .. }
            | RecipeIssue::DuplicateTransition { .. }
            | RecipeIssue::DuplicateRelationRow { .. }
            | RecipeIssue::DuplicateRelation { .. }
            | RecipeIssue::DuplicateCodec { .. }
            | RecipeIssue::DuplicateRelationPosture { .. }
            | RecipeIssue::DuplicateRelationQuestion { .. }
            | RecipeIssue::DuplicateProjection { .. }
            | RecipeIssue::DuplicateRelationTable { .. }
            | RecipeIssue::GeneratedNameCollision { .. } => Observed::IdentityDisagreement,
            RecipeIssue::GeneratedNameNotIdentifier { .. }
            | RecipeIssue::Grammar(_)
            | RecipeIssue::VariantNotUnit { .. }
            | RecipeIssue::ForeignMember { .. }
            | RecipeIssue::AllowedAbsenceNeedsFallback
            | RecipeIssue::SupportAddressUnneeded
            | RecipeIssue::ReplacementUnplanned { .. }
            | RecipeIssue::FragmentNotGenerated
            | RecipeIssue::ExactDispatchFunctionRequired
            | RecipeIssue::ExactDispatchBodyRefused
            | RecipeIssue::ExactDispatchParameterCount { .. }
            | RecipeIssue::ExactDispatchParameterBinding { .. }
            | RecipeIssue::RelationTableExactRequired { .. }
            | RecipeIssue::RelationTableTransitionUnsupported { .. }
            | RecipeIssue::ExactRelationTableFunctionRequired
            | RecipeIssue::ExactRelationTableBodyRefused
            | RecipeIssue::ExactRelationTableParameterCount { .. }
            | RecipeIssue::ExactRelationTableParameterBinding { .. }
            | RecipeIssue::RelationPostureMismatch { .. }
            | RecipeIssue::RelationPostureInapplicable { .. }
            | RecipeIssue::RelationPayloadShapeMismatch { .. } => Observed::ContractDisagreement,
            RecipeIssue::CodecDeclaration { .. } | RecipeIssue::CodecOwnerNotRecord { .. } => {
                Observed::ContractDisagreement
            }
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
            RecipeIssue::ProjectionSubjectRequired { .. } => {
                human_projection!("name the vocabulary or typed lowering this projection consumes")
            }
            RecipeIssue::AllowedAbsenceNeedsFallback => human_projection!(
                "declare typed refusal for absent rows or state an explicit caller-owned fallback before requesting dispatch"
            ),
            RecipeIssue::ExactDispatchBodyRefused => human_projection!(
                "remove the exact function body and leave the semicolon-terminated signature for the standard dispatch projector to fill"
            ),
            RecipeIssue::ExactDispatchFunctionRequired
            | RecipeIssue::ExactDispatchParameterCount { .. }
            | RecipeIssue::ExactDispatchParameterBinding { .. } => human_projection!(
                "write `dispatch { fn apply(state: State, event: Event) -> Result<State, TransitionRefusal>; };` with exactly two simple identifier bindings"
            ),
            RecipeIssue::RelationTableExactRequired { .. }
            | RecipeIssue::ExactRelationTableFunctionRequired
            | RecipeIssue::ExactRelationTableParameterCount { .. }
            | RecipeIssue::ExactRelationTableParameterBinding { .. } => human_projection!(
                "write `relation_tables { policy { fn lookup(left: Left, right: Right) -> Option<Payload>; }; };` with exactly two simple identifier bindings"
            ),
            RecipeIssue::ExactRelationTableBodyRefused => human_projection!(
                "remove the exact function body and leave the semicolon-terminated signature for the standard relation-table projector to fill"
            ),
            RecipeIssue::RelationTableTransitionUnsupported { .. } => human_projection!(
                "request dispatch for the transition lowering or select a non-transition relation for a typed relation table"
            ),
            RecipeIssue::DuplicateMember { .. }
            | RecipeIssue::DuplicateVocabulary { .. }
            | RecipeIssue::DuplicateTransition { .. }
            | RecipeIssue::DuplicateRelationRow { .. }
            | RecipeIssue::DuplicateRelation { .. }
            | RecipeIssue::DuplicateCodec { .. }
            | RecipeIssue::DuplicateRelationPosture { .. }
            | RecipeIssue::DuplicateRelationQuestion { .. }
            | RecipeIssue::DuplicateProjection { .. }
            | RecipeIssue::DuplicateRelationTable { .. }
            | RecipeIssue::GeneratedNameCollision { .. } => human_projection!(
                "state each authored member, relation endpoint pair, transition seat, projection role, and generated name once"
            ),
            RecipeIssue::RelationPostureMismatch { .. } => human_projection!(
                "change the declared relation rows or state the structural posture those rows actually satisfy"
            ),
            RecipeIssue::RelationPostureInapplicable { .. } => human_projection!(
                "use one vocabulary on both relation sides before requiring a self-relation or cycle answer"
            ),
            RecipeIssue::RelationPayloadShapeMismatch { .. } => human_projection!(
                "use one unlabeled, path, exact-Rust, or transition payload contract for every row in one relation"
            ),
            RecipeIssue::CodecDeclaration { .. } => human_projection!(
                "repair the codec declaration under the existing codec owner's typed contract"
            ),
            RecipeIssue::CodecOwnerNotRecord { .. } => human_projection!(
                "name one record-shaped struct authored in the recipe module as this codec's owner"
            ),
            RecipeIssue::InlineModuleRequired
            | RecipeIssue::BakeRequiredLast
            | RecipeIssue::GeneratedNameNotIdentifier { .. }
            | RecipeIssue::Grammar(_)
            | RecipeIssue::VocabularyNotFound { .. }
            | RecipeIssue::RelationNotFound { .. }
            | RecipeIssue::VariantNotUnit { .. }
            | RecipeIssue::ReplacementUnplanned { .. }
            | RecipeIssue::FragmentNotGenerated => human_projection!(
                "write one inline module whose final bake declaration names only the vocabularies, relations, codecs, postures, evidence, support, and projections the recipe actually uses"
            ),
            RecipeIssue::ForeignMember { .. } => human_projection!(
                "name a member declared by the relation endpoint vocabulary or repair that endpoint vocabulary"
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

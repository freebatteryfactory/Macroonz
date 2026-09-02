//! The informed recipe, its projection vocabulary, and the capability boundary shared by both execution hosts.

use crate::bounded::{Bounded, KeyedRoster};
use crate::diagnostic::{Diagnostic, Family};
use crate::expansion::Expansion;
use crate::identity::OwnerFact;
use crate::relation::{
    AbsencePosture, CompletenessPosture, CyclePosture, DensityPosture, EmptyPosture,
    MembershipPosture, RepetitionPosture, SelfRelationPosture,
};
use crate::render::Output;
use crate::request::Door;
use crate::support::SupportName;
use crate::token::{CapturedInput, GeneratedToken, GeneratedTree, SpanHandle};

#[path = "account.rs"]
mod account;

#[path = "relation_account.rs"]
mod relation_account;

#[path = "type_guard.rs"]
mod guard;

/// The maximum number of members in one recipe vocabulary.
pub const VOCABULARY_LIMIT: usize = 64;

/// The maximum number of named relations in one recipe.
pub const RELATION_LIMIT: usize = 64;

/// The maximum number of rows in one recipe relation.
pub const RELATION_ROW_LIMIT: usize = 128;

/// The maximum number of relation tables selected by one projection family.
pub const RELATION_TABLE_LIMIT: usize = RELATION_LIMIT;

/// The maximum number of transition rows in one recipe.
///
/// Transition syntax is one ergonomic lowering over the generic relation-row ceiling.
pub const TRANSITION_LIMIT: usize = RELATION_ROW_LIMIT;

/// The complete number of descriptor-native evidence forms one recipe may carry.
pub const EVIDENCE_LIMIT: usize = 5;

/// The maximum number of codec declarations carried by one recipe.
pub const CODEC_LIMIT: usize = 16;

/// The complete number of fixed recipe projection families.
pub const PROJECTION_LIMIT: usize = 12;

/// The diagnostic family owned by the recipe declaration.
pub(super) const RECIPE_FAMILY: Family = Family::declared("macroonz/recipe");

/// The structural fact this recipe owner declares.
pub(super) const RECIPE_FACT: OwnerFact = OwnerFact {
    home: "recipe",
    name: "one-informed-recipe-selects-and-delivers-every-requested-projection",
};

/// Whether the facade posture makes harness-owned evidence projections available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessPosture {
    /// The facade carries its optional harness owner.
    Available,
    /// The facade omits its optional harness owner.
    Unavailable,
}

/// One member read from an authored Rust enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeMember {
    spelling: String,
    name: GeneratedToken,
    at: SpanHandle,
}

/// One caller-named vocabulary and its informed authored members.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeVocabulary {
    name: String,
    name_token: GeneratedToken,
    members: KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>,
    at: SpanHandle,
}

/// The optional caller-owned material attached to one relation row.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecipeRelationPayload {
    /// The row states only its two endpoints.
    Unlabeled,
    /// The row carries one ordinary caller-owned Rust path.
    Path(GeneratedTree),
    /// The row carries exact caller-authored Rust material.
    ExactRust(GeneratedTree),
    /// The row carries the target and effect required by the transition lowering.
    Transition {
        /// The target member spelling.
        target: String,
        /// The exact ordinary or raw identifier token naming the target member.
        target_name: GeneratedToken,
        /// The exact caller-authored effect path.
        effect: GeneratedTree,
    },
}

/// Which one row-payload contract every row in one relation follows.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipeRelationPayloadKind {
    /// Relation rows carry endpoints only.
    Unlabeled,
    /// Relation rows carry ordinary caller-owned paths.
    Path,
    /// Relation rows carry exact caller-authored Rust material.
    ExactRust,
    /// Relation rows carry the target and effect required by transition lowering.
    Transition,
}

/// One informed row in a caller-named binary relation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeRelationRow {
    left: String,
    left_name: GeneratedToken,
    left_at: SpanHandle,
    right: String,
    right_name: GeneratedToken,
    right_at: SpanHandle,
    payload: RecipeRelationPayload,
    payload_at: SpanHandle,
}

/// The structural questions one relation declaration chose to answer.
///
/// An absent field means the recipe did not ask that question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecipeRelationRequirements {
    empty: Option<EmptyPosture>,
    repetition: Option<RepetitionPosture>,
    membership: Option<[MembershipPosture; 2]>,
    completeness: Option<[CompletenessPosture; 2]>,
    density: Option<DensityPosture>,
    absence: Option<AbsencePosture>,
    self_relation: Option<SelfRelationPosture>,
    cycle: Option<CyclePosture>,
}

/// One caller-named binary relation over two informed vocabularies.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeRelation {
    name: String,
    name_token: GeneratedToken,
    name_at: SpanHandle,
    left_vocabulary: String,
    right_vocabulary: String,
    rows: Bounded<RecipeRelationRow, RELATION_ROW_LIMIT>,
    payload_kind: RecipeRelationPayloadKind,
    requirements: RecipeRelationRequirements,
}

/// Whether one generic relation came from the paved transition lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RelationLowering {
    /// The caller declared one generic relation directly.
    Generic,
    /// The caller used the ergonomic transition grammar.
    Transition,
}

/// One caller-named codec declaration owned semantically by the compiler codec home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeCodec {
    name: String,
    content: crate::codec::CodecContent,
    at: SpanHandle,
}

crate::roster! {
    /// The complete projection vocabulary understood by the first recipe slice.
    #[non_exhaustive]
    pub enum RecipeRole {
        /// Enum-member and relation companions inside the generated child module.
        Companions = "companions",
        /// Typed membership and payload lookup tables over selected relations.
        RelationTables = "relation-tables",
        /// The generated sparse dispatch function and typed absence refusal.
        Dispatch = "dispatch",
        /// Rustc-owned compile-contract material carried to a test target.
        CompileContract = "compile-contract",
        /// An independently invoked harness property carried to a test target.
        Property = "property",
        /// One selected vocabulary projected as type-level stage markers.
        Typestate = "typestate",
        /// One existing descriptor trial carrier over caller-declared rows.
        Trials = "trials",
        /// One existing descriptor mutation surface over an explicitly selected vocabulary.
        Mutation = "mutation",
        /// One existing descriptor benchmark carrier over caller-declared work.
        Benchmarks = "benchmarks",
        /// One existing descriptor network module over caller-declared topology and schedules.
        Network = "network",
        /// One existing descriptor concurrency module over caller-declared exploration rows.
        Concurrency = "concurrency",
        /// Canonical encode and decode roads from one or more existing-owner codec declarations.
        Codec = "codec",
    }
}

/// Which informed recipe vocabulary a mutation evidence block presses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvidenceTarget {
    vocabulary: String,
}

/// One exact descriptor-native evidence declaration carried by the recipe.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeEvidence {
    role: RecipeRole,
    target: Option<EvidenceTarget>,
    body: CapturedInput,
    at: SpanHandle,
}

/// The already sealed output for each selected standard evidence projection.
pub(crate) struct PreparedEvidence {
    pub(super) trees: [Option<GeneratedTree>; EVIDENCE_LIMIT],
}

/// The one crate-internal preparation capability the composition root supplies.
pub(crate) trait EvidenceCompiler {
    /// Prepare every selected standard evidence projection without giving the recipe home adapter vocabulary.
    fn prepared(
        capture: &CapturedInput,
        recipe: &Recipe,
        door: &Door,
        replaced: Option<RecipeRole>,
    ) -> Result<PreparedEvidence, Diagnostic>;
}

/// The sealed marker whose sole implementation lives at the crate composition root.
pub(crate) struct ConfiguredEvidence;

/// Where one effective mechanical projection value came from.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoweringSource {
    /// The projector's documented conventional spelling.
    Preset,
    /// A named recipe seat replaced the conventional spelling.
    Configuration,
    /// Exact caller-authored Rust replaced the conventional mechanical seat.
    ExactRust,
}

/// The effective mechanical configuration of one generated projection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectiveProjection {
    role: RecipeRole,
    name: Option<String>,
    subject: Option<String>,
    source: LoweringSource,
    exact_rust: Option<GeneratedTree>,
    exact_dispatch_bindings: Option<[GeneratedToken; 2]>,
    exact_dispatch_imports: Option<[bool; 2]>,
    relation_tables: Option<Box<Bounded<RelationTableProjection, RELATION_TABLE_LIMIT>>>,
}

/// One selected typed relation table and its effective function surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationTableProjection {
    relation: String,
    function: String,
    source: LoweringSource,
    exact_rust: Option<GeneratedTree>,
    bindings: Option<[GeneratedToken; 2]>,
    imports: Option<[bool; 2]>,
}

/// What happened to one role in the recipe's complete projection account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum ProjectionStanding {
    /// The role enters the selected request membership under this effective lowering.
    Generated(EffectiveProjection),
    /// The caller deliberately did not request this role.
    NotRequested,
    /// The facade feature posture does not carry the harness owner this role requires.
    FeatureUnavailable,
    /// The caller declared that the target plane for this role is unavailable.
    TargetUnavailable,
}

/// The public readback of what happened to one possible recipe projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionDisposition {
    /// The role is selected and generated.
    Generated,
    /// The caller did not request the role.
    NotRequested,
    /// The facade feature posture does not carry the required harness owner.
    FeatureUnavailable,
    /// The caller declared that the target plane is unavailable.
    TargetUnavailable,
}

/// One informed recipe over caller-owned vocabularies, relations, and selected projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    module_name: String,
    module_name_token: GeneratedToken,
    module_head: GeneratedTree,
    authored_body: GeneratedTree,
    module_body_at: Option<SpanHandle>,
    vocabularies: Option<KeyedRoster<RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<String>,
    codecs: Option<KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>,
    projections: [ProjectionStanding; PROJECTION_LIMIT],
    evidence: [Option<RecipeEvidence>; EVIDENCE_LIMIT],
    support: Option<SupportName>,
}

/// The mechanically read seats offered to the recipe invariant constructor.
pub(super) struct RecipeParts {
    pub(super) module_name: String,
    pub(super) module_name_token: GeneratedToken,
    pub(super) module_head: GeneratedTree,
    pub(super) authored_body: GeneratedTree,
    pub(super) module_body_at: Option<SpanHandle>,
    pub(super) vocabularies: Vec<RecipeVocabularyParts>,
    pub(super) relations: Vec<RecipeRelationParts>,
    pub(super) transition_relation: Option<String>,
    pub(super) codecs: Vec<RecipeCodec>,
    pub(super) projections: [ProjectionStanding; PROJECTION_LIMIT],
    pub(super) evidence: [Option<RecipeEvidence>; EVIDENCE_LIMIT],
    pub(super) support: Option<SupportName>,
}

/// The mechanically read seats offered to one vocabulary constructor.
pub(super) struct RecipeVocabularyParts {
    pub(super) name: String,
    pub(super) name_token: GeneratedToken,
    pub(super) members: Vec<RecipeMember>,
    pub(super) at: SpanHandle,
}

/// The mechanically read seats offered to one relation constructor.
pub(super) struct RecipeRelationParts {
    pub(super) name: String,
    pub(super) name_token: GeneratedToken,
    pub(super) name_at: SpanHandle,
    pub(super) left_vocabulary: String,
    pub(super) left_vocabulary_at: SpanHandle,
    pub(super) right_vocabulary: String,
    pub(super) right_vocabulary_at: SpanHandle,
    pub(super) rows: Vec<RecipeRelationRow>,
    pub(super) requirements: RecipeRelationRequirements,
}

/// The kind whose selected roles are the recipe's generated projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeProjection;

/// A projector's read-only view of one informed recipe.
#[derive(Clone, Copy)]
pub struct RecipeView<'recipe> {
    recipe: &'recipe Recipe,
}

/// The one selected role one projector invocation answers.
#[derive(Clone, Copy)]
pub struct ProjectionRequest<'recipe> {
    effective: &'recipe EffectiveProjection,
}

/// A consuming output capability already bound to one selected recipe role.
pub struct ProjectionSink<'output, 'plan> {
    output: &'output mut Output<'plan, RecipeProjection>,
    role: RecipeRole,
}

/// Opaque evidence that one projector used its bound sink successfully.
#[must_use = "a successful offer is evidence that the selected projection seat was filled"]
pub struct ProjectionOffered {
    _private: (),
}

/// The authority-neutral projection operation shared by built-in and caller-owned clients.
pub trait RecipeProjector {
    /// Project one selected role through its one-use sink.
    ///
    /// # Errors
    ///
    /// Returns the first token or render refusal established by the implementation.
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError>;
}

/// The built-in projector catalog used by the paved proc host.
pub(super) struct StandardProjector<'evidence> {
    pub(super) evidence: &'evidence PreparedEvidence,
}

/// Why one projector invocation produced no admitted unit.
#[must_use = "a projection refusal states why the selected role was not filled"]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionError {
    /// Token construction exceeded a generated-tree magnitude.
    Tokens(crate::bounded::Overflow),
    /// The existing output owner refused the offered unit.
    Render(crate::render::RenderError),
}

/// Why one recipe could not be informed or baked.
#[must_use = "a recipe refusal states the exact structural or capability disagreement"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecipeError {
    issue: RecipeIssue,
    at: Option<SpanHandle>,
}

/// One recipe declaration or capability disagreement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum RecipeIssue {
    /// The wrapper input is not one inline Rust module.
    InlineModuleRequired,
    /// The module does not end with exactly one `bake!` declaration.
    BakeRequiredLast,
    /// The authored module already declares the generated child name.
    GeneratedNameCollision {
        /// The colliding name.
        name: String,
    },
    /// A requested generated spelling is not one ordinary Rust identifier.
    GeneratedNameNotIdentifier {
        /// The refused spelling.
        name: String,
    },
    /// A mechanical recipe clause was absent or malformed.
    Grammar(crate::token::CaptureReadIssue),
    /// One named vocabulary does not resolve to an authored enum in the module.
    VocabularyNotFound {
        /// The requested enum name.
        name: String,
    },
    /// One selected vocabulary carries an enum variant shape the generic roster does not enumerate.
    VariantNotUnit {
        /// The enum name.
        vocabulary: String,
        /// The variant name.
        variant: String,
    },
    /// One vocabulary carries a repeated member spelling.
    DuplicateMember {
        /// The enum name.
        vocabulary: String,
        /// The repeated member spelling.
        member: String,
    },
    /// One vocabulary name was declared more than once in the recipe account.
    DuplicateVocabulary {
        /// The repeated vocabulary name.
        name: String,
    },
    /// One transition names a member outside its declared vocabulary.
    ForeignMember {
        /// The vocabulary whose roster was crossed.
        vocabulary: String,
        /// The foreign member spelling.
        member: String,
    },
    /// Two transitions occupy the same state-and-event seat.
    DuplicateTransition {
        /// The state member spelling.
        state: String,
        /// The event member spelling.
        event: String,
    },
    /// Two rows occupy the same relation endpoint seat while repetition is refused.
    DuplicateRelationRow {
        /// The relation carrying the repeated endpoint pair.
        relation: String,
        /// The left endpoint member spelling.
        left: String,
        /// The right endpoint member spelling.
        right: String,
    },
    /// One relation name was declared more than once in the recipe account.
    DuplicateRelation {
        /// The repeated relation name.
        name: String,
    },
    /// One codec declaration name was stated more than once.
    DuplicateCodec {
        /// The repeated codec declaration name.
        name: String,
    },
    /// One codec declaration was refused by the existing codec owner.
    CodecDeclaration {
        /// The caller-owned codec declaration name.
        name: String,
        /// The exact codec-owner refusal.
        reason: String,
    },
    /// One codec owner does not name an authored record-shaped structure in the recipe module.
    CodecOwnerNotRecord {
        /// The caller-owned codec declaration name.
        codec: String,
        /// The owner spelling the declaration selected.
        owner: String,
    },
    /// One posture block names no declared relation.
    RelationNotFound {
        /// The unavailable relation name.
        name: String,
    },
    /// One relation posture block was declared more than once.
    DuplicateRelationPosture {
        /// The relation with repeated posture declarations.
        relation: String,
    },
    /// One structural question was answered more than once for one relation.
    DuplicateRelationQuestion {
        /// The relation carrying the repeated answer.
        relation: String,
        /// The repeated structural question.
        question: &'static str,
    },
    /// One caller-required structural answer disagrees with the computed relation answer.
    RelationPostureMismatch {
        /// The relation whose structural question disagreed.
        relation: String,
        /// The structural question that was settled.
        question: &'static str,
        /// The caller-required answer.
        required: &'static str,
        /// The independently computed answer.
        observed: &'static str,
    },
    /// One same-roster structural question was asked of a cross-roster relation.
    RelationPostureInapplicable {
        /// The relation whose structural question has no lawful subject.
        relation: String,
        /// The same-roster structural question that was requested.
        question: &'static str,
    },
    /// Rows in one relation disagree about which payload contract they carry.
    RelationPayloadShapeMismatch {
        /// The relation carrying mixed payload shapes.
        relation: String,
        /// The first row's payload contract.
        expected: RecipeRelationPayloadKind,
        /// The later row's disagreeing payload contract.
        observed: RecipeRelationPayloadKind,
    },
    /// One projection was requested more than once.
    DuplicateProjection {
        /// The repeated role.
        role: RecipeRole,
    },
    /// One relation was selected more than once inside the relation-table family.
    DuplicateRelationTable {
        /// The relation carrying the repeated table request.
        relation: String,
    },
    /// One payload-bearing relation table omitted its exact result contract.
    RelationTableExactRequired {
        /// The relation whose payload type remains caller authority.
        relation: String,
    },
    /// A transition payload was offered to the generic relation-table projector.
    RelationTableTransitionUnsupported {
        /// The transition relation that already has a dedicated dispatch projection.
        relation: String,
    },
    /// The recipe selected no projection at all.
    ProjectionRequired,
    /// One selected projection requires another role that the recipe did not select.
    ProjectionDependencyAbsent {
        /// The role that cannot operate alone.
        role: RecipeRole,
        /// The role it requires.
        required: RecipeRole,
    },
    /// One selected projection has no structural subject it can lawfully consume.
    ProjectionSubjectRequired {
        /// The projection without a subject.
        role: RecipeRole,
        /// The structural subject family the projection requires.
        expected: &'static str,
    },
    /// Sparse dispatch cannot infer what an allowed absent row means.
    AllowedAbsenceNeedsFallback,
    /// A harness-owned role was requested from a facade posture without the harness.
    HarnessUnavailable {
        /// The unavailable role.
        role: RecipeRole,
    },
    /// Evidence cargo was requested without an exported support address.
    SupportAddressRequired,
    /// A support address was declared while no evidence cargo was requested.
    SupportAddressUnneeded,
    /// A caller-owned projector was offered for a role the recipe did not select.
    ReplacementUnplanned {
        /// The unselected role.
        role: RecipeRole,
    },
    /// Exact captured Rust could not be preserved as generated tokens.
    FragmentNotGenerated,
    /// Exact dispatch braces did not contain one semicolon-terminated function signature.
    ExactDispatchFunctionRequired,
    /// Exact dispatch supplied a caller-authored body that would bypass row accounting.
    ExactDispatchBodyRefused,
    /// Exact dispatch did not declare exactly two parameters.
    ExactDispatchParameterCount {
        /// The number of parameter rows supplied.
        observed: usize,
    },
    /// One exact dispatch parameter did not use a simple identifier binding.
    ExactDispatchParameterBinding {
        /// The one-based parameter position.
        position: usize,
    },
    /// Exact relation-table braces did not contain one semicolon-terminated function signature.
    ExactRelationTableFunctionRequired,
    /// Exact relation-table syntax supplied a body that would bypass row accounting.
    ExactRelationTableBodyRefused,
    /// Exact relation-table syntax did not declare exactly two parameters.
    ExactRelationTableParameterCount {
        /// The number of parameter rows supplied.
        observed: usize,
    },
    /// One exact relation-table parameter did not use a simple identifier binding.
    ExactRelationTableParameterBinding {
        /// The one-based parameter position.
        position: usize,
    },
}

/// The complete baked result: selected recipe projections plus the sealed declaration-site emission.
#[must_use = "a baked recipe carries the selected projection expansion and its sealed emitted module"]
pub struct RecipeBake {
    pub(super) projection: Expansion<RecipeProjection>,
    pub(super) emitted: Expansion<RecipeShell>,
}

/// The private final-emission kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RecipeShell;

/// The semantic parentage of the final emitted module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecipeShellContent {
    pub(super) recipe: crate::identity::ClosedExpansionId,
    pub(super) support: Option<crate::identity::ClosedExpansionId>,
}

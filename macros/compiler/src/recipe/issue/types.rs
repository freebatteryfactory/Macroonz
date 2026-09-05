//! Every recipe refusal the capture and account roads can name.

use crate::recipe::{RecipeRelationPayloadKind, RecipeRole};
use crate::token::SpanHandle;

/// Why one recipe could not be informed or baked.
#[must_use = "a recipe refusal states the exact structural or capability disagreement"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::recipe) struct RecipeError {
    issue: RecipeIssue,
    at: Option<SpanHandle>,
}

/// Which standard exact-function projection owns one shared syntax refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::recipe) enum ExactProjectionSeat {
    /// The transition dispatch projection.
    Dispatch,
    /// One typed relation-table projection.
    RelationTable,
}

/// One refusal shared by both exact-function projection seats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::recipe) enum ExactFunctionIssue {
    /// The exact seat did not contain one semicolon-terminated function signature.
    FunctionRequired,
    /// The exact seat supplied a caller-authored body that would bypass row accounting.
    BodyRefused,
    /// The exact signature did not declare exactly two parameters.
    ParameterCount {
        /// The number of parameter rows supplied.
        observed: usize,
    },
    /// One exact parameter did not use a simple identifier binding.
    ParameterBinding {
        /// The one-based parameter position.
        position: usize,
    },
}

/// One recipe declaration or capability disagreement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(in crate::recipe) enum RecipeIssue {
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
    /// One named vocabulary resolves to an authored enum with no variants.
    VocabularyEmpty {
        /// The authored enum name.
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
    /// More than one caller-owned projector was offered for the same selected role.
    DuplicateReplacement {
        /// The role with more than one caller-owned projector.
        role: RecipeRole,
    },
    /// One bake offered more caller-owned projectors than the complete role vocabulary can hold.
    ReplacementRosterUnbounded {
        /// The number of caller-owned projectors offered.
        observed: usize,
    },
    /// Exact captured Rust could not be preserved as generated tokens.
    FragmentNotGenerated,
    /// One exact-function projection seat refused shared signature mechanics.
    ExactFunction {
        /// The projection seat that refused.
        seat: ExactProjectionSeat,
        /// The shared exact-function disagreement.
        issue: ExactFunctionIssue,
    },
    /// One exact dispatch selector did not name a simple parameter binding in its signature.
    ExactDispatchBindingAbsent {
        /// The selector spelling absent from the exact signature.
        binding: String,
    },
}

#[path = "type_guard.rs"]
mod guard;

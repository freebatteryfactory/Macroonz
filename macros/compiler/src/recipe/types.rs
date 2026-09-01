//! The informed recipe, its projection vocabulary, and the capability boundary shared by both execution hosts.

use crate::bounded::{AbsencePosture, KeyedRoster};
use crate::diagnostic::Family;
use crate::expansion::Expansion;
use crate::identity::OwnerFact;
use crate::render::Output;
use crate::support::SupportName;
use crate::token::{GeneratedTree, SpanHandle};

#[path = "type_guard.rs"]
mod guard;

/// The maximum number of members in one recipe vocabulary.
pub const VOCABULARY_LIMIT: usize = 64;

/// The maximum number of transition rows in one recipe.
pub const TRANSITION_LIMIT: usize = 128;

/// The diagnostic family owned by the recipe declaration.
pub(super) const RECIPE_FAMILY: Family = Family::declared("macroonz/recipe");

/// The structural fact this recipe owner declares.
pub const RECIPE_FACT: OwnerFact = OwnerFact {
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
    at: SpanHandle,
}

/// One informed transition row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeTransition {
    from: String,
    event: String,
    to: String,
    effect: GeneratedTree,
    at: SpanHandle,
}

crate::roster! {
    /// The complete projection vocabulary understood by the first recipe slice.
    pub enum RecipeRole {
        /// Enum-member and relation companions inside the generated child module.
        Companions = "companions",
        /// The generated sparse dispatch function and typed absence refusal.
        Dispatch = "dispatch",
        /// Rustc-owned compile-contract material carried to a test target.
        CompileContract = "compile-contract",
        /// An independently invoked harness property carried to a test target.
        Property = "property",
    }
}

/// Where one effective mechanical projection value came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoweringSource {
    /// The projector's documented conventional spelling.
    Preset,
    /// A named recipe seat replaced the conventional spelling.
    Configuration,
    /// Exact caller-authored Rust supplies the seat.
    ExactRust,
}

/// The effective mechanical configuration of one generated projection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectiveProjection {
    role: RecipeRole,
    name: Option<String>,
    source: LoweringSource,
}

/// What happened to one role in the recipe's complete projection account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectionStanding {
    /// The role enters the selected request membership under this effective lowering.
    Generated(EffectiveProjection),
    /// The caller deliberately did not request this role.
    NotRequested,
}

/// One informed recipe over two caller-owned enum vocabularies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    module_name: String,
    module_head: GeneratedTree,
    authored_body: GeneratedTree,
    states_name: String,
    states: KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>,
    events_name: String,
    events: KeyedRoster<RecipeMember, String, VOCABULARY_LIMIT>,
    transitions: KeyedRoster<RecipeTransition, (String, String), TRANSITION_LIMIT>,
    absence: AbsencePosture,
    projections: [ProjectionStanding; 4],
    support: Option<SupportName>,
}

/// The mechanically read seats offered to the recipe invariant constructor.
pub(super) struct RecipeParts {
    pub(super) module_name: String,
    pub(super) module_head: GeneratedTree,
    pub(super) authored_body: GeneratedTree,
    pub(super) states_name: String,
    pub(super) state_members: Vec<RecipeMember>,
    pub(super) events_name: String,
    pub(super) event_members: Vec<RecipeMember>,
    pub(super) transitions: Vec<RecipeTransition>,
    pub(super) absence: AbsencePosture,
    pub(super) projections: [ProjectionStanding; 4],
    pub(super) support: Option<SupportName>,
}

/// The kind whose selected roles are the recipe's generated projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeProjection;

/// A projector's read-only view of one informed recipe.
pub struct RecipeView<'recipe> {
    recipe: &'recipe Recipe,
}

/// The one selected role one projector invocation answers.
pub struct ProjectionRequest {
    role: RecipeRole,
}

/// A consuming output capability already bound to one selected recipe role.
pub struct ProjectionSink<'output, 'plan> {
    output: &'output mut Output<'plan, RecipeProjection>,
    role: RecipeRole,
}

/// Opaque evidence that one projector used its bound sink successfully.
#[must_use = "a successful offer is evidence that the selected projection seat was filled"]
pub struct Offered {
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
        view: &RecipeView<'_>,
        request: &ProjectionRequest,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<Offered, ProjectionError>;
}

/// Why one projector invocation produced no admitted unit.
#[must_use = "a projection refusal states why the selected role was not filled"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionError {
    /// The recipe's plan does not select the requested role.
    Unplanned {
        /// The role that was requested.
        role: RecipeRole,
    },
    /// Token construction exceeded a generated-tree magnitude.
    Tokens(crate::bounded::Overflow),
    /// The existing output owner refused the offered unit.
    Render(crate::render::RenderError),
}

/// Why one recipe could not be informed or baked.
#[must_use = "a recipe refusal states the exact structural or capability disagreement"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeError {
    issue: RecipeIssue,
    at: Option<SpanHandle>,
}

/// One recipe declaration or capability disagreement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecipeIssue {
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
    /// One vocabulary enum carries a variant shape this first projection does not enumerate.
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
    /// One projection was requested more than once.
    DuplicateProjection {
        /// The repeated role.
        role: RecipeRole,
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

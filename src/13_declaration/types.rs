//! The shared authoring algebra: phase roots, name roles, the linker's
//! families, the six facets, staged meta, and the frontend roles.
//!
//! # The front-door law
//!
//! Producer versus judge is the boundary; front door versus front door is
//! not. Every authoring route converges before semantic closure on
//! [`DeclarationFragment`] — the convergence point is not tokens, a common
//! parser AST, a compiler-private IR, or a global registry. Name resolution,
//! collision law, normalization, and linking have ONE production owner.
//!
//! # The language-death line
//!
//! The application-language frontend is dead-for-now behind the plug-in bar:
//! its notation (grammar, lexer, capsule, formatter, precedence) is not baked
//! anywhere. This algebra is frontend-neutral — the live Rust-declaration
//! frontend feeds it today, and a future language re-enters as a second
//! producer of the same fragments through the same normalization, validation,
//! and admission (the no-secret-second-language law).
//!
//! # Source never rides inside a refusal body
//!
//! A refusal states facts about source as typed classification: symbol
//! identity, explicit coordinate role, profile identity, the typed class of
//! the violated constraint, and at most ONE offending Unicode scalar. Two or
//! more contiguous scalars reconstruct the input — so no canonical body
//! carries a scalar sequence, a spelling, a skeleton, a glyph string, or any
//! substring of the source. This binds the CANONICAL body, not merely its
//! released projection: there is nothing to redact, because the material
//! never entered.

use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::refusal::{AdmittedPrefix, CompletionPosture, FamilyShape, RefusalFamily};
use crate::types::{Bounded, ConstLimit, EvidenceRef, Limit, NonEmptyBounded};
use crate::value::BoundedText;

// ---------------------------------------------------------------------------
// Coordinates.
// ---------------------------------------------------------------------------

/// The six explicit coordinate roles — checked conversions relate them; no
/// role substitutes for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateRole {
    /// Byte position.
    Byte,
    /// Unicode-scalar position.
    UnicodeScalar,
    /// UTF-16 position.
    Utf16,
    /// Line/column position.
    LineColumn,
    /// Normalized-source position.
    NormalizedSource,
    /// Semantic-origin position.
    SemanticOrigin,
}

/// One typed source coordinate: a position under a declared role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceCoordinate {
    /// The coordinate role.
    pub role: CoordinateRole,
    /// The position under that role.
    pub position: u64,
}

// ---------------------------------------------------------------------------
// Phase roots — separate types; neither one generic SemanticThing nor a
// public type for every private storage detail.
// ---------------------------------------------------------------------------

/// The domain marker for source-form content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceFormDomain;

/// The lossless authored phase root — NEVER runtime authority: nothing
/// executes source text, and a parsed tree, type-check success, compiler
/// identity, or source digest proves no admission, capability, or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceForm {
    /// The lossless content's commitment.
    pub content: Commitment<SourceFormDomain>,
}

/// The domain marker for fragment identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FragmentIdentityDomain;

/// The claim marker for origin relationships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginClaim;

/// The convergence point of every authoring route — origin-bound, immutable,
/// members private by default. Identity is APPLICATIVE by default: same
/// fragment identity + same argument commitments + same profiles = same
/// meaning; a deliberately distinct instance requires an explicit
/// identity-bearing argument, and expansion count, order, formatting, alias,
/// and position mint NO identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationFragment {
    /// The applicative fragment identity.
    pub identity: Commitment<FragmentIdentityDomain>,
    /// The origin evidence.
    pub origin: EvidenceRef<OriginClaim>,
}

/// The domain marker for linked graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkedGraphDomain;

/// The closed declaration graph: symbols resolved, owners bound, no missing,
/// conflicting, or duplicate claims. Minted ONLY by the linker seam — the
/// linker never repairs through last-writer-wins, source order, silent
/// renaming, or numeric suffixes, and no ambient inventory, startup
/// constructor, directory scan, or process-global registration substitutes
/// for the composition root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclarationGraph {
    linked: Commitment<LinkedGraphDomain>,
}

impl DeclarationGraph {
    /// In-crate mint for laws. Test-gated until the linker seam exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(linked: Commitment<LinkedGraphDomain>) -> Self {
        Self { linked }
    }

    /// The linked fact set's commitment.
    #[must_use]
    pub fn linked(&self) -> &Commitment<LinkedGraphDomain> {
        &self.linked
    }
}

/// The claim marker for origin-graph relationships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginGraphClaim;

/// The authoring→parsed→validated→generated relationship record. Creates no
/// semantic identity unless the owning role commits to it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OriginGraph {
    /// The relationship evidence.
    pub relationships: EvidenceRef<OriginGraphClaim>,
}

// ---------------------------------------------------------------------------
// Name roles — three, never merged.
// ---------------------------------------------------------------------------

/// Limit family for name text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NameTextLimit;
impl Limit for NameTextLimit {}

/// The authored name: strict versioned Unicode source and display spelling.
/// NOT required to be NFC — resolution compares the validated NFC form while
/// lossless source preserves the exact authored spelling; that admission is
/// precisely what leaves normalized collision a real fact rather than a
/// restatement of duplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthoredName {
    /// The authored spelling.
    pub spelling: BoundedText<NameTextLimit>,
}

/// The domain marker for symbol identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolDomain;

/// The semantic symbol identity — typed, owner-derived, NOT the glyph string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolIdentity(pub Commitment<SymbolDomain>);

/// The identity role marker for projection profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionProfileRole;

/// One projection profile — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionProfileId(Occurrence<ProjectionProfileRole>);

impl IdentityRole for ProjectionProfileId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl ProjectionProfileId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<ProjectionProfileRole>) -> Self {
        Self(occurrence)
    }
}

crate::scope_guard_version! {
    /// One version of a projection profile — Class C, scoped to its profile.
    pub struct ProjectionProfileVersion over ProjectionProfileId, seated in mod projection_profile_version;
}

/// The export alias: the exact target-safe spelling faithfully projecting one
/// semantic symbol under one projection profile — a PRESENTATION value (no
/// identity authority, explicitly not a Class-E pointer), derived by
/// [`ExportAliasDerivation`]'s gate order, never hand-authored. A projection
/// that cannot represent an authored name refuses rather than silently
/// mangling, suffixing, or transliterating.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportAlias {
    /// The projected symbol.
    pub symbol: SymbolIdentity,
    /// The projection profile and version.
    pub profile: ProjectionProfileVersion,
    /// The target-safe spelling.
    pub spelling: BoundedText<NameTextLimit>,
}

// ---------------------------------------------------------------------------
// Frontends, authoring roles, stages, facets.
// ---------------------------------------------------------------------------

/// The two production front doors — AUTHORED plain names (the old codenames
/// are dead). Each owns its own notation; both converge through the one
/// declaration algebra and the one canonical semantic linker. The second seat
/// is the plug-in bar the dead-for-now application language re-enters
/// through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrontendRole {
    /// The live Rust-declaration frontend (the macros crate's shell + its
    /// deterministic engine).
    RustDeclaration,
    /// The pluggable application-language frontend — dead-for-now.
    ApplicationLanguage,
}

/// The four top-level authoring forms — distinct boundaries no operator,
/// pipeline, or formatter makes implicit. PEND is deliberately not spelled
/// await: nothing waits — the Turn pends. It arms a one-shot continuation
/// upon a named observation; delivery resumes it — a durable operation
/// posture that survives restart, which no awaited stack ever does. It
/// produces and reuses no `Truth::Pending` value: the knowledge axes speak
/// Pending about answers that can lag; PEND speaks about a Turn that
/// suspends. The words rhyme deliberately; the roles never substitute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevelForm {
    /// Bounded pure evaluation over admitted read-only inputs.
    Ask,
    /// Explicitly effectful and receipted.
    Do,
    /// Create or reuse one durable effect-intent identity; return without
    /// waiting.
    Request,
    /// Admit the intent, begin one immediate bounded physical attempt,
    /// observe within bounds.
    Pend,
}

/// The four progressive-disclosure authoring roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthoringRole {
    /// A direct declaration.
    DirectDeclaration,
    /// A parameterized declaration fragment.
    Fragment,
    /// A typed meta function.
    MetaFunction,
    /// Typed semantic templates: a quoted fragment is typed data, not text. A
    /// template is authored against this algebra, instantiated with typed
    /// arguments, and produces origin-bound [`DeclarationFragment`] material
    /// that re-enters the ordinary validation and linking path with no
    /// shortcut. Splicing substitutes typed values only — a string never
    /// becomes an identifier, a symbol, or a coordinate — and instantiation
    /// mints no authority: every produced fragment carries the instantiating
    /// site's origin and is judged there. Frontend-neutral: any front door may
    /// offer a template surface, or none, without changing this algebra.
    /// (Role name pending review.)
    Quotation,
}

/// Six-fold hygiene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HygieneClass {
    /// Lexical hygiene.
    Lexical,
    /// Identity hygiene.
    Identity,
    /// Authority hygiene.
    Authority,
    /// Effect hygiene.
    Effect,
    /// Origin hygiene.
    Origin,
    /// Profile hygiene.
    Profile,
}

/// The judgment's stage — exactly four values, never open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    /// Authoring stage.
    Authoring,
    /// Meta stage.
    Meta,
    /// Semantic stage.
    Semantic,
    /// Runtime stage.
    Runtime,
}

/// The six semantic inspection facets. Each carries authoring semantics, not
/// just a slot: WHO declares REQUIRED authority and grants nothing; WHEN's
/// capture-current elaborates before execution into one explicit captured
/// exact cut per participating authority — never a cross-authority sequence,
/// distributed snapshot, or transaction; the compiler PROVES HOW refines
/// WHAT — never two competing implementations; WHY never asserts evidence
/// exists. An applicable facet may be omitted when its meaning is absent or
/// completely derivable — no mandatory six-field envelope exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facet {
    /// Required authority.
    Who,
    /// The intended contract.
    What,
    /// The semantic location material.
    Where,
    /// Each temporal role, separately.
    When,
    /// The bounded computation realizing WHAT.
    How,
    /// Purpose, evidence requests, explanation requests.
    Why,
}

/// The canonical facet sequence — authoring order is author-free; the
/// formatter normalizes to this one fixed sequence.
pub const CANONICAL_FACET_SEQUENCE: [Facet; 6] = [
    Facet::Who,
    Facet::What,
    Facet::Where,
    Facet::When,
    Facet::How,
    Facet::Why,
];

/// WHO's closed content roster.
pub const WHO_FACET_CONTENT: [&str; 6] = [
    "subject",
    "actor",
    "correlation",
    "partition",
    "process",
    "grouping",
];

/// WHAT's closed content roster.
pub const WHAT_FACET_CONTENT: [&str; 8] = [
    "value",
    "record",
    "state",
    "decision",
    "transition",
    "event",
    "effect",
    "result",
];

/// WHERE's closed content roster.
pub const WHERE_FACET_CONTENT: [&str; 6] = [
    "accepted-history",
    "projection",
    "saved-program",
    "artifact",
    "resource",
    "application-input",
];

/// WHEN's closed content roster — a durable program declares its deadline
/// policy; a live monotonic value never appears in source.
pub const WHEN_FACET_CONTENT: [&str; 6] = [
    "cut",
    "interval",
    "durable-order",
    "application-generation",
    "partition-epoch",
    "deadline-policy",
];

/// HOW's closed content roster — the richest single roster in this home.
pub const HOW_FACET_CONTENT: [&str; 14] = [
    "fold",
    "match",
    "traverse",
    "join",
    "group",
    "order",
    "page",
    "bound",
    "admit",
    "execute",
    "suspend",
    "recover",
    "reconcile",
    "qualify",
];

/// WHY's closed content roster.
pub const WHY_FACET_CONTENT: [&str; 10] = [
    "purpose",
    "explanation",
    "provenance",
    "evidence",
    "completeness",
    "freshness",
    "proof",
    "work",
    "margin",
    "receipt",
];

/// The registered named facet forms — a closed typed roster, never a set of
/// spellings. How a front door spells each form is that frontend's own
/// vocabulary and never travels into this algebra.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FacetForm {
    /// WHEN's elaboration into one explicit captured exact cut per
    /// participating authority.
    CaptureCurrent,
    /// WHY's demand that named evidence be presented.
    RequiresEvidence,
    /// WHY's declaration that this work produces named evidence.
    ProducesEvidence,
    /// WHY's demand for an explanation of the judgment.
    Explain,
}

/// The linker contract's seven acts, in order — it emits a complete linked
/// fact set or a typed refusal, and never repairs.
pub const LINKER_CONTRACT: [&str; 7] = [
    "resolve-declared-symbols-and-imports",
    "bind-definitions-owners-and-coordinate-families",
    "reject-missing-conflicting-duplicate-claims",
    "validate-version-and-profile-compatibility",
    "canonicalize-order-only-where-source-order-carries-no-meaning",
    "build-closed-dispatch-and-configuration-tables",
    "emit-complete-linked-fact-set-or-typed-refusal",
];

/// The four convergence routes onto [`DeclarationFragment`].
pub const CONVERGENCE_ROUTES: [&str; 4] = [
    "direct-declarations",
    "fragment-instantiations",
    "typed-meta-expansion",
    "typed-quotation",
];

/// The three locks every meta evaluation declares BEFORE evaluation — it
/// refuses before over-limit allocation and never returns a partial fragment
/// set.
pub const META_EVALUATION_LOCKS: [&str; 3] = [
    "symbolic-formula-over-validated-inputs",
    "hard-compiler-profile-ceiling",
    "checked-evaluation-meter",
];

/// The six stage laws governing the meta stage — the closed roster every
/// meta or template evaluation obeys, owned here beside the stage vocabulary
/// it governs. The compiler plane cites this roster; it never restates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaStageLaw {
    /// Runtime material crosses into meta only through an explicit bounded
    /// lift with a closed portable schema, identity, and source commitment.
    ExplicitBoundedLift,
    /// A meta descriptor is data describing meaning — parsing or serializing
    /// it neither grants authority nor injects runtime meaning.
    DescriptorIsData,
    /// Live capabilities, grants, ports, continuations, attempts, handles,
    /// secrets, clocks, entropy, and host state cannot become meta values.
    NoLiveAuthorityAsMetaValue,
    /// Meta output re-enters ordinary fragment validation and linking; it
    /// arrives neither trusted nor prequalified.
    OutputReentersUntrusted,
    /// Compile-time refusal, runtime refusal, image-admission refusal, and
    /// qualification failure remain different result families.
    RefusalFamiliesStayDistinct,
    /// Finite input, work, memory, recursion, declaration, symbol,
    /// diagnostic, and output-byte bounds are declared BEFORE evaluation.
    BoundsDeclaredBeforeEvaluation,
}

// ---------------------------------------------------------------------------
// Classification domain markers for the families' typed payloads.
// ---------------------------------------------------------------------------

/// Representability-class domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepresentabilityDomain;
/// Reserved-word identity domain marker (identity within the closed
/// owner-declared set — never the spelling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservedWordDomain;
/// Target-profile-rule identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetRuleDomain;
/// Number-system identity domain marker (never the digits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberSystemDomain;
/// Identifier-status value domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierStatusDomain;
/// Confusable-relation domain marker (never a glyph, never a skeleton).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfusableRelationDomain;
/// Normalization-relation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NormalizationRelationDomain;
/// Reference-role domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReferenceRoleDomain;
/// Lifetime-relation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifetimeRelationDomain;
/// Conflict-relation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConflictRelationDomain;

// ---------------------------------------------------------------------------
// ExportAliasDerivation — single cause under the declared gate order.
// ---------------------------------------------------------------------------

/// Alias derivation's refusal — a pure single-cause enum under the DECLARED
/// derivation gate order (fixed here once, never Rust variant order): profile
/// support → representability → character repertoire → reserved word → length
/// → target-profile constraint → collision. The order is total and public, so
/// which gates ran is derivable from the named cause — every earlier gate
/// passed, every later gate did not run — and the family carries NO posture
/// member (a member restating what the declared order already fixes would be
/// a second home for one fact). A derivation refusal names one established
/// cause and claims nothing about the gates that did not run. No cause is
/// payload-free; no payload carries a spelling.
#[must_use = "a derivation refusal carries the lawful reason the alias was not derived"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExportAliasDerivation {
    /// The compiler supports no such projection profile — refused before any
    /// proposed spelling exists; type existence fabricates no supported row.
    UnsupportedTargetProfile {
        /// The unsupported profile.
        profile: ProjectionProfileVersion,
    },
    /// No lawful target-safe spelling of this symbol exists under this
    /// profile.
    Unrepresentable {
        /// The symbol.
        symbol: SymbolIdentity,
        /// The profile.
        profile: ProjectionProfileVersion,
        /// The typed representability class.
        class: Commitment<RepresentabilityDomain>,
    },
    /// A produced scalar leaves the profile's declared repertoire.
    CharacterSetViolation {
        /// The symbol.
        symbol: SymbolIdentity,
        /// The profile.
        profile: ProjectionProfileVersion,
        /// The ONE offending scalar.
        scalar: char,
        /// Its typed coordinate.
        coordinate: SourceCoordinate,
    },
    /// The proposed spelling is a reserved word of the target profile —
    /// carries the word's identity within the closed set, never the spelling.
    ReservedWord {
        /// The reserved word's typed identity.
        word: Commitment<ReservedWordDomain>,
    },
    /// The proposed spelling exceeds the profile's declared length bound.
    LengthExceeded {
        /// The declared bound.
        bound: u64,
        /// The measured length.
        measured: u64,
        /// The counting coordinate role both are stated under.
        counting_role: CoordinateRole,
    },
    /// A produced spelling broke one named target-profile rule that is not
    /// repertoire, reserved word, length, or collision.
    TargetProfileConstraintViolation {
        /// The violated rule's typed identity.
        rule: Commitment<TargetRuleDomain>,
        /// The profile.
        profile: ProjectionProfileVersion,
    },
    /// Two distinct semantic symbols would derive the same alias in one
    /// target namespace — never disambiguated by numeric suffix, source
    /// order, or last-writer-wins.
    Collision {
        /// The first symbol.
        first: SymbolIdentity,
        /// The second symbol.
        second: SymbolIdentity,
        /// The profile.
        profile: ProjectionProfileVersion,
    },
}

impl RefusalFamily for ExportAliasDerivation {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "UnsupportedTargetProfile",
        "Unrepresentable",
        "CharacterSetViolation",
        "ReservedWord",
        "LengthExceeded",
        "TargetProfileConstraintViolation",
        "Collision",
    ];
}

// ---------------------------------------------------------------------------
// AuthoredNameConstruction — issue collection over one scalar sequence.
// ---------------------------------------------------------------------------

/// The authored-name issues — one scan of the pinned identifier profile
/// (Unicode 17.0.0; Start = `XID_Start` or `_`, Continue = `XID_Continue`)
/// establishes all of them at once. Every scalar-level issue carries the one
/// offending scalar with its typed coordinate and nothing further.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthoredNameConstructionIssue {
    /// The first scalar is neither `XID_Start` nor `_`.
    InvalidIdentifierStart {
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
    /// A later scalar is not `XID_Continue`.
    InvalidIdentifierContinue {
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
    /// A bidirectional ordering control appears inside the identifier.
    BidirectionalControl {
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
    /// A default-ignorable scalar other than U+200C/U+200D appears.
    DisallowedDefaultIgnorable {
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
    /// U+200C or U+200D appears outside an admitted contextual joining
    /// position — a separate cause because its repair is a different act.
    InvalidJoinControlContext {
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
    /// Digits of more than one number system appear — carries the typed
    /// number-system identities, never the digits.
    MixedNumberSystem {
        /// The observed number systems' typed identities.
        systems: Commitment<NumberSystemDomain>,
        /// The coordinate of the observation.
        coordinate: SourceCoordinate,
    },
    /// The name is confusable, under the declared UTS #39 tailoring, with a
    /// reserved word — never the spelling, never its skeleton.
    ConfusableWithReservedWord {
        /// The reserved word's typed identity.
        word: Commitment<ReservedWordDomain>,
        /// The typed confusable relation.
        relation: Commitment<ConfusableRelationDomain>,
    },
    /// A scalar whose identifier status is not Allowed under the declared
    /// tailoring.
    IdentifierStatusRestricted {
        /// The typed identifier-status value.
        status: Commitment<IdentifierStatusDomain>,
        /// The offending scalar.
        scalar: char,
        /// Its coordinate.
        coordinate: SourceCoordinate,
    },
}

/// Limit family for authored-name issues — a DECLARED finite issue bound
/// (several scalars may each violate at once, so the roster's cardinality is
/// not the cap; the bound value is evidence-selected).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthoredNameIssueLimit;
impl Limit for AuthoredNameIssueLimit {}

/// Authored-name construction: a non-empty bounded canonical issue
/// collection, ordered by declared cause order then ascending scalar
/// coordinate. Owns no scope-relative fact — collisions are
/// [`ClosureNamespace`]'s, where the set they are relative to exists.
#[must_use = "a construction refusal carries every established issue with the name"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthoredNameConstruction {
    body: AdmittedPrefix<AuthoredNameConstructionIssue, AuthoredNameIssueLimit>,
}

impl AuthoredNameConstruction {
    /// The established issues — at least one, at most the declared bound.
    #[must_use]
    pub const fn issues(
        &self,
    ) -> &NonEmptyBounded<AuthoredNameConstructionIssue, AuthoredNameIssueLimit> {
        self.body.carried()
    }

    /// What this body says about its own coverage.
    #[must_use]
    pub const fn posture(&self) -> CompletionPosture {
        self.body.completion()
    }
}

impl RefusalFamily for AuthoredNameConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// ClosureNamespace — issue collection over one closed namespace.
// ---------------------------------------------------------------------------

/// The closure-namespace issues — seven, one per declared namespace law, the
/// collision law having exactly two stated species. Both collision species
/// live here rather than at name construction because a collision is a fact
/// about a set, and a profile check has no scope. Neither is ever repaired by
/// a numeric suffix, source order, silent renaming, or last-writer-wins.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClosureNamespaceIssue {
    /// The declared symbol-count bound is exceeded — the namespace refuses
    /// rather than dropping a symbol.
    SymbolCountExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// The declared byte bound is exceeded — this is where a byte cap on
    /// names lives, and why no name-construction family carries one.
    NamespaceBytesExceeded {
        /// The declared bound.
        bound: u64,
        /// The measured size.
        measured: u64,
        /// The byte coordinate role the measurement is stated under.
        byte_role: CoordinateRole,
    },
    /// Two declarations bind the same symbol in one closure.
    DuplicateSymbol {
        /// The first binding.
        first: SymbolIdentity,
        /// The second binding.
        second: SymbolIdentity,
        /// The first site.
        first_site: SourceCoordinate,
        /// The second site.
        second_site: SourceCoordinate,
    },
    /// Two distinct authored names compare equal under validated NFC.
    NormalizedCollision {
        /// The first symbol.
        first: SymbolIdentity,
        /// The second symbol.
        second: SymbolIdentity,
        /// The first site.
        first_site: SourceCoordinate,
        /// The second site.
        second_site: SourceCoordinate,
        /// The typed normalization relation — never the two spellings.
        relation: Commitment<NormalizationRelationDomain>,
    },
    /// An authored name is confusable with an in-scope visible symbol.
    ConfusableCollision {
        /// The first symbol.
        first: SymbolIdentity,
        /// The second symbol.
        second: SymbolIdentity,
        /// The first site.
        first_site: SourceCoordinate,
        /// The second site.
        second_site: SourceCoordinate,
        /// The typed confusable relation — never a glyph, never a skeleton.
        relation: Commitment<ConfusableRelationDomain>,
    },
    /// A reference names no bound symbol — no symbol identity exists to
    /// carry; the exact bytes stay in lossless source form.
    UnresolvedReference {
        /// The reference site.
        site: SourceCoordinate,
        /// The typed reference role.
        role: Commitment<ReferenceRoleDomain>,
    },
    /// A symbol is observed outside the lifetime its declaration admits.
    LifetimeViolation {
        /// The symbol.
        symbol: SymbolIdentity,
        /// The use site.
        site: SourceCoordinate,
        /// The typed lifetime relation.
        relation: Commitment<LifetimeRelationDomain>,
    },
}

/// Limit family for closure-namespace issues — a declared finite bound,
/// evidence-selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClosureNamespaceIssueLimit;
impl Limit for ClosureNamespaceIssueLimit {}

/// Closure-namespace refusal: the namespace is closed as a whole and checked
/// as a whole. Ordering: declared cause order, then the typed source
/// coordinate of the issue's site, then the closure's declared member
/// sequence. Owns no cross-fragment claim (those are [`LinkResolution`]'s)
/// and no target-namespace alias collision ([`ExportAliasDerivation`]'s) —
/// three collision questions, three owners, three types, no shared enum, no
/// conversion.
#[must_use = "a namespace refusal carries every established issue with the closure"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClosureNamespace {
    body: AdmittedPrefix<ClosureNamespaceIssue, ClosureNamespaceIssueLimit>,
}

impl ClosureNamespace {
    /// The established issues — at least one, at most the declared bound.
    #[must_use]
    pub const fn issues(
        &self,
    ) -> &NonEmptyBounded<ClosureNamespaceIssue, ClosureNamespaceIssueLimit> {
        self.body.carried()
    }

    /// What this body says about its own coverage.
    #[must_use]
    pub const fn posture(&self) -> CompletionPosture {
        self.body.completion()
    }
}

impl RefusalFamily for ClosureNamespace {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// LinkResolution — issue collection over the closed graph.
// ---------------------------------------------------------------------------

/// The closed four claim kinds the linker ranges over — carried as a typed
/// member of the issue rather than multiplying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimKind {
    /// A route claim.
    Route,
    /// A field claim.
    Field,
    /// An operation claim.
    Operation,
    /// An identity claim.
    Identity,
}

/// Limit family for duplicate-claim site sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DuplicateSiteLimit;
impl Limit for DuplicateSiteLimit {}

/// The link-resolution issues — five, closed. An export alias derived under
/// one projection profile or version and presented against another is a LINK
/// fact, not a derivation fact, and refuses here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkResolutionIssue {
    /// A required claim is absent from the linked set.
    MissingClaim {
        /// The claim kind.
        kind: ClaimKind,
        /// The requiring site.
        site: SourceCoordinate,
        /// The requiring declaration's symbol.
        requiring: SymbolIdentity,
    },
    /// Two admitted claims of one kind assert incompatible facts about one
    /// subject.
    ConflictingClaims {
        /// The claim kind.
        kind: ClaimKind,
        /// The first claim's coordinate.
        first_site: SourceCoordinate,
        /// The second claim's coordinate.
        second_site: SourceCoordinate,
        /// The first owning symbol.
        first_owner: SymbolIdentity,
        /// The second owning symbol.
        second_owner: SymbolIdentity,
        /// The typed conflict relation.
        relation: Commitment<ConflictRelationDomain>,
    },
    /// One claim is declared more than once where the algebra admits exactly
    /// one.
    DuplicateClaim {
        /// The claim kind.
        kind: ClaimKind,
        /// The declaring coordinates.
        sites: Bounded<SourceCoordinate, DuplicateSiteLimit>,
    },
    /// A claim is presented against a version its declaration does not admit.
    VersionIncompatibility {
        /// The claim kind.
        kind: ClaimKind,
        /// The required version.
        required: u64,
        /// The presented version.
        presented: u64,
        /// The coordinate.
        site: SourceCoordinate,
    },
    /// A claim is presented under a profile the receiving declaration does
    /// not admit.
    ProfileIncompatibility {
        /// The claim kind.
        kind: ClaimKind,
        /// The required profile.
        required: ProjectionProfileVersion,
        /// The presented profile.
        presented: ProjectionProfileVersion,
        /// The coordinate.
        site: SourceCoordinate,
    },
}

/// Limit family for link-resolution issues — a declared finite bound,
/// evidence-selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkResolutionIssueLimit;
impl Limit for LinkResolutionIssueLimit {}

/// Link-resolution refusal: the linker closes one complete graph in one pass
/// and several claims may be defective at once — reporting one is a missing
/// diagnostic. Ordering: declared cause order, then the typed origin
/// coordinate of the offending claim. The linker refuses; it never repairs.
#[must_use = "a resolution refusal carries every established issue with the link"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkResolution {
    body: AdmittedPrefix<LinkResolutionIssue, LinkResolutionIssueLimit>,
}

impl LinkResolution {
    /// The established issues — at least one, at most the declared bound.
    #[must_use]
    pub const fn issues(&self) -> &NonEmptyBounded<LinkResolutionIssue, LinkResolutionIssueLimit> {
        self.body.carried()
    }

    /// What this body says about its own coverage.
    #[must_use]
    pub const fn posture(&self) -> CompletionPosture {
        self.body.completion()
    }
}

impl RefusalFamily for LinkResolution {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// ProjectionContract and its construction family.
// ---------------------------------------------------------------------------

/// Projection-target domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionTargetDomain;
/// Projection-audience domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionAudienceDomain;
/// Loss-posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LossPostureDomain;
/// Projection-configuration domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionConfigurationDomain;

/// The closed five orthogonal projection claims — the enum that must not
/// grow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionClaim {
    /// Coverage.
    Coverage,
    /// Reversibility.
    Reversibility,
    /// Disclosure.
    Disclosure,
    /// Actionability.
    Actionability,
    /// Representation.
    Representation,
}

/// Compile-time bound for a contract's stated claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionClaimLimit;
impl Limit for ProjectionClaimLimit {}
impl ConstLimit for ProjectionClaimLimit {
    const MAX: usize = 5;
}

/// Caller-authored checked intent — a sibling-artifact REQUEST, not a stage.
/// A projection contract states each of its five orthogonal claims at
/// construction; an unstated claim refuses rather than defaulting. Its
/// constructor judges whether the request is a valid VALUE, never whether the
/// compiler can presently serve it — support is answered at target admission
/// and projection execution. Owner and consumer are not members; the
/// no-projection-without-a-named-owner-and-consumer rule refuses at the
/// post-link projection request.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionContract {
    /// The target.
    pub target: Commitment<ProjectionTargetDomain>,
    /// The audience.
    pub audience: Commitment<ProjectionAudienceDomain>,
    /// The projection profile.
    pub profile: ProjectionProfileVersion,
    /// The loss posture.
    pub loss: Commitment<LossPostureDomain>,
    /// The configuration.
    pub configuration: Commitment<ProjectionConfigurationDomain>,
    /// The stated claims.
    pub claims: Bounded<ProjectionClaim, ProjectionClaimLimit>,
}

/// The projection-contract construction issues — five absent-member causes,
/// each deliberately payload-free (the member set is closed, the issue's
/// identity is the whole fact, and there is no observed value to classify;
/// each is `…Missing`, never `…Incomplete`), plus the unstated-claim cause
/// carrying its typed claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionContractConstructionIssue {
    /// The target member is absent.
    TargetMissing,
    /// The audience member is absent.
    AudienceMissing,
    /// The profile member is absent.
    ProfileMissing,
    /// The loss-posture member is absent.
    LossPostureMissing,
    /// The configuration member is absent.
    ConfigurationMissing,
    /// One orthogonal claim was not stated.
    ClaimUnstated(ProjectionClaim),
}

/// Compile-time bound for projection-contract issues: five member kinds plus
/// at most five unstated claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIssueLimit;
impl Limit for ProjectionIssueLimit {}
impl ConstLimit for ProjectionIssueLimit {
    const MAX: usize = 10;
}

/// Projection-contract construction: a caller can omit several members and
/// claims at once. Ordering: declared cause order, then the fixed member and
/// claim order of the two five-item lists.
#[must_use = "a construction refusal carries every established issue with the contract"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionContractConstruction {
    body: AdmittedPrefix<ProjectionContractConstructionIssue, ProjectionIssueLimit>,
}

impl ProjectionContractConstruction {
    /// The established issues — at least one, at most the declared bound.
    #[must_use]
    pub const fn issues(
        &self,
    ) -> &NonEmptyBounded<ProjectionContractConstructionIssue, ProjectionIssueLimit> {
        self.body.carried()
    }

    /// What this body says about its own coverage.
    #[must_use]
    pub const fn posture(&self) -> CompletionPosture {
        self.body.completion()
    }
}

impl RefusalFamily for ProjectionContractConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

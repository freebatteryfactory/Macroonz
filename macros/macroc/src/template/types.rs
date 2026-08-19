//! The template home's declarations: category-typed holes, the three locks, the
//! template and its application, the semantic invocation key, and the five
//! magnitudes this home's capacities are governed by.
//!
//! Declarations only. Every road that reaches a private field lives in
//! `type_guard.rs`, this file's own child, which is what makes a cross-category
//! binding, a ceiling missing an axis, and an application with an unbound hole
//! values nobody can build.

use crate::origin_graph::Nonclaim;
use crate::plane::{
    ApplicationDistinctnessSubject, BoundFormulaSubject, InputDescriptorSubject,
    LanguageProfileSubject, MetaProfileSubject, OwnerFactRef, OwnerIdentityRef, ProfileVersion,
    SourceSnapshotSubject, TemplateArgumentSubject, TemplateParameterSubject, TemplateSubject,
};
use threadpak::declaration::Stage;
use threadpak::declaration::types::{FragmentIdentityDomain, ProjectionConfigurationDomain};
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many typed holes one template may declare —
    /// and therefore how many bindings one application of it may supply, since a
    /// complete application binds one commitment per declared hole, exactly.
    ///
    /// # Bounds
    ///
    /// Thirty-two. Every hole is a category-typed seat an application must fill
    /// and a reader must hold at once, and a template past thirty-two holes has
    /// stopped being one declaration shape — the repair is a template that takes
    /// an assembled fragment, not a longer hole roster here.
    TemplateParameterLimit = 32,
    /// The magnitude governing how many validated input descriptors one bound
    /// formula stands over and one invocation key commits to.
    ///
    /// # Bounds
    ///
    /// Thirty-two. The descriptors are the population a meta evaluation ranges
    /// over, and they enter an invocation key's commitment: a key committing to
    /// more than thirty-two is a key nobody can read back against the
    /// declaration it stands for.
    ///
    /// # Nonclaims
    ///
    /// It is its own family and not [`TemplateParameterLimit`], even though the
    /// two numbers agree today. That one bounds the HOLES a template declares;
    /// this one bounds the descriptors an evaluation ranges over, and one family
    /// standing for both would be one authority answering two questions.
    InputDescriptorLimit = 32,
    /// The magnitude governing how many declaration fragments one invocation key
    /// names as a dependency.
    ///
    /// # Bounds
    ///
    /// Sixty-four. A key's dependency set is what the invocation stands on, and
    /// it is written whole into the key's own commitment: a key that named more
    /// than sixty-four fragments would be committing to a graph rather than to
    /// an invocation.
    FragmentDependencyLimit = 64,
    /// The magnitude governing how many axis ceilings one profile ceiling
    /// carries.
    ///
    /// # Bounds
    ///
    /// Eight — the meta bound-axis roster's own cardinality
    /// ([`META_BOUND_AXES`]), because a complete ceiling names each axis exactly
    /// once. It is not a number this home chose out of taste: a ninth ceiling
    /// would have to be a ninth AXIS, and the roster declares eight.
    MetaBoundAxisLimit = 8,
    /// The magnitude governing how many issues one template-construction refusal
    /// body may carry.
    ///
    /// # Bounds
    ///
    /// Ninety-six, sized by the WIDEST of the three passes rather than by the
    /// narrowest.
    ///
    /// The binding pass is that one, and it asks two independent questions per
    /// declared hole: how many bindings name it, and whether one of them
    /// disagrees with its declared category. Both can hold of one hole at once,
    /// so the pass establishes up to two issues per declared parameter, plus one
    /// unknown-parameter issue per supplied binding — three times the parameter
    /// magnitude. The hole pass and the ceiling pass are narrower and fit inside
    /// it.
    TemplateIssueLimit = 96,
}

// ---------------------------------------------------------------------------
// Category-typed holes.
// ---------------------------------------------------------------------------

/// The closed roster of splice categories: which kind of hole a template
/// declares, and which kind of material may fill it.
///
/// Category-typed holes are the law this roster exists for: a string can never
/// become an identifier; a type cannot enter an expression hole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpliceCategory {
    /// A hole that evaluates to a value.
    Expression,
    /// A hole naming a type.
    Type,
    /// A hole standing where a pattern is matched.
    Pattern,
    /// A hole standing where a whole declaration goes.
    Declaration,
    /// A hole filled by declaration material assembled elsewhere.
    Fragment,
    /// A hole that binds a name — the category a string is never admitted to.
    IdentifierBinding,
}

/// The declared splice-category roster, in the order this contract states it.
pub const SPLICE_CATEGORIES: [SpliceCategory; 6] = [
    SpliceCategory::Expression,
    SpliceCategory::Type,
    SpliceCategory::Pattern,
    SpliceCategory::Declaration,
    SpliceCategory::Fragment,
    SpliceCategory::IdentifierBinding,
];

/// One typed hole a template declares: which category it admits, and which
/// parameter it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateParameter {
    /// The category of material this hole admits.
    pub category: SpliceCategory,
    /// The parameter's own identity.
    pub parameter: OwnerIdentityRef<TemplateParameterSubject>,
}

/// One typed commitment offered to fill a hole: which category the material
/// is, and which exact argument it commits to.
///
/// The argument is a commitment, never a spelling: nothing here carries source
/// text, and no rendering of this value can be read back as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateArgument {
    /// The category of the offered material.
    pub category: SpliceCategory,
    /// The argument's exact commitment.
    pub commitment: OwnerIdentityRef<TemplateArgumentSubject>,
}

/// How one binding fails.
///
/// A single-cause family with one inhabited cause: binding one argument to one
/// parameter runs exactly one check, so one cause is all that can truthfully
/// exist and a collection body would claim checks that never ran.
#[must_use = "a binding refusal carries the two categories that disagreed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateBindingIssue {
    /// The argument's category is not the parameter's category.
    CategoryMismatch {
        /// The category the parameter declared.
        expected: SpliceCategory,
        /// The category the argument offered.
        found: SpliceCategory,
    },
}

/// One argument bound to one parameter of the same category.
///
/// Holding one *is* the category proof: the only road here is
/// [`TemplateBinding::bound`], which refuses a category disagreement, so a
/// cross-category binding is not a value that exists and then fails a check —
/// it is a value nobody can build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateBinding {
    parameter: TemplateParameter,
    argument: TemplateArgument,
}

// ---------------------------------------------------------------------------
// The three locks.
// ---------------------------------------------------------------------------

/// The first lock: a symbolic bound formula stated over validated inputs.
///
/// This is a commitment to the formula, never a small language: the plane
/// names the formula its owner declared, names the owner fact that declares
/// it, and names the validated inputs it stands over. Nothing here evaluates,
/// parses, or rewrites a formula — evaluating one is a mechanism, and the
/// mechanism is not this home's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolicBoundFormula {
    /// The declared formula, by identity.
    pub formula: OwnerIdentityRef<BoundFormulaSubject>,
    /// The owner fact that declares it.
    pub declared_by: OwnerFactRef,
    /// The validated inputs the formula stands over — at least one, by shape:
    /// a formula over nothing bounds nothing.
    pub over_inputs:
        NonEmptyBounded<OwnerIdentityRef<InputDescriptorSubject>, InputDescriptorLimit>,
}

/// The closed roster of meta bound axes a profile ceiling covers.
///
/// A ceiling that named "too big" without naming which magnitude would be an
/// unlocated bound, so every axis is a member of this roster and a ceiling
/// names each one exactly once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaBoundAxis {
    /// Validated input descriptors one evaluation may range over.
    InputDescriptors,
    /// Work one evaluation may perform, in its declared currency.
    Work,
    /// Memory one evaluation may hold.
    Memory,
    /// Recursion depth one evaluation may reach.
    Recursion,
    /// Declarations one evaluation may produce.
    Declarations,
    /// Symbols one evaluation may introduce.
    Symbols,
    /// Diagnostics one evaluation may carry.
    Diagnostics,
    /// Output bytes one evaluation may commit to.
    OutputBytes,
}

/// The declared meta bound-axis roster, in the order this contract states it.
pub const META_BOUND_AXES: [MetaBoundAxis; 8] = [
    MetaBoundAxis::InputDescriptors,
    MetaBoundAxis::Work,
    MetaBoundAxis::Memory,
    MetaBoundAxis::Recursion,
    MetaBoundAxis::Declarations,
    MetaBoundAxis::Symbols,
    MetaBoundAxis::Diagnostics,
    MetaBoundAxis::OutputBytes,
];

/// One axis of a profile ceiling: which magnitude, how large, and on whose
/// declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxisCeiling {
    /// The magnitude this ceiling bounds.
    pub axis: MetaBoundAxis,
    /// The declared maximum on that axis.
    pub magnitude: u64,
    /// The owner fact that declares it.
    pub declared_by: OwnerFactRef,
}

/// The second lock: the hard profile ceiling, one magnitude per meta bound
/// axis.
///
/// Complete by construction: [`ProfileCeiling::declared`] admits a ceiling only
/// when every axis in [`META_BOUND_AXES`] appears exactly once, so a ceiling
/// silently missing an axis — the shape that lets one magnitude run unbounded
/// while the others look governed — cannot be held.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileCeiling {
    axes: Bounded<AxisCeiling, MetaBoundAxisLimit>,
}

/// The third lock: the declared posture of the checked evaluation meter.
///
/// The obligation is the owner's — an actual meter refuses BEFORE over-limit
/// allocation and never returns a partial fragment set — and this carrier states
/// it by citation.
///
/// # Nonclaims
///
/// Holding one is not evidence a meter ran: the meter is a gated mechanism, and
/// a posture read as a measurement would be the plane answering a question it
/// has no standing to answer. The [`Nonclaim`] seat says so in the value itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckedMeterPosture {
    /// The owner fact declaring the checked-evaluation meter obligation.
    pub obliged_by: OwnerFactRef,
    /// What this carrier explicitly does not claim: that a meter ran here.
    pub unmeasured: Nonclaim,
}

// ---------------------------------------------------------------------------
// The template.
// ---------------------------------------------------------------------------

/// One profile named together with the version it was read at. Two versions of
/// two different profiles are not comparable and are never ranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionedProfile<Profile> {
    /// The profile.
    pub profile: OwnerIdentityRef<Profile>,
    /// That profile's version.
    pub version: ProfileVersion,
}

/// The seats a template seam can overrun. Only seats that can actually hold a
/// caller-supplied count appear; every other seat is bounded by the shape it
/// is built from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateSeat {
    /// The holes one template declares.
    DeclaredParameters,
    /// The bindings one application supplies.
    SuppliedBindings,
    /// The axis ceilings one profile ceiling carries.
    AxisCeilings,
}

/// The closed template-construction issue set.
///
/// No issue is payload-free: each names the parameter, axis, or seat it is
/// about, because a bare variant makes the caller guess which hole moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateConstructionIssue {
    /// Two declared holes claim the same parameter identity.
    DuplicateParameter {
        /// The doubled parameter.
        parameter: OwnerIdentityRef<TemplateParameterSubject>,
    },
    /// A binding names a parameter this template does not declare.
    UnknownParameter {
        /// The unrecognized parameter.
        parameter: OwnerIdentityRef<TemplateParameterSubject>,
    },
    /// A declared hole was left unbound.
    MissingBinding {
        /// The unbound parameter.
        parameter: OwnerIdentityRef<TemplateParameterSubject>,
    },
    /// A declared hole was bound more than once.
    DuplicateBinding {
        /// The doubly bound parameter.
        parameter: OwnerIdentityRef<TemplateParameterSubject>,
    },
    /// A binding names a declared parameter under the wrong category. Distinct
    /// from [`TemplateBindingIssue::CategoryMismatch`], which is the argument
    /// disagreeing with its own parameter: this is the supplied parameter
    /// disagreeing with the template's declaration of it.
    DeclaredCategoryDisagreement {
        /// The parameter both sides name.
        parameter: OwnerIdentityRef<TemplateParameterSubject>,
        /// The category the template declared.
        declared: SpliceCategory,
        /// The category the binding carried.
        bound: SpliceCategory,
    },
    /// No ceiling bounds this axis.
    CeilingAxisAbsent {
        /// The unbounded axis.
        axis: MetaBoundAxis,
    },
    /// Two ceilings bound the same axis.
    CeilingAxisDoubled {
        /// The doubled axis.
        axis: MetaBoundAxis,
    },
    /// A seat's declared magnitude was exceeded.
    SeatBoundExceeded {
        /// Which seat overran.
        seat: TemplateSeat,
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The template-construction refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
pub use guard::TemplateConstruction;

/// One authored declaration template: its identity, its typed holes, the three
/// locks it declares before any evaluation, and the stage its owner declared it
/// is evaluated at.
///
/// Every seat is required. The parameter seat is structurally non-empty — a
/// template with no hole is a declaration, and the machine already has those.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationTemplate {
    identity: OwnerIdentityRef<TemplateSubject>,
    parameters: NonEmptyBounded<TemplateParameter, TemplateParameterLimit>,
    formula: SymbolicBoundFormula,
    ceiling: ProfileCeiling,
    meter: CheckedMeterPosture,
    stage: Stage,
}

// ---------------------------------------------------------------------------
// Application.
// ---------------------------------------------------------------------------

/// Whether one application is the applicative one or a deliberately distinct
/// one.
///
/// Not an option and not a flag: distinctness is an explicit, identity-bearing
/// declaration, so "these two applications are deliberately different" and
/// "somebody set a boolean" can never read the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicativeDistinctness {
    /// The ordinary posture: same template, same arguments, same profiles,
    /// same meaning.
    Applicative,
    /// Deliberately distinct from an otherwise identical application, under
    /// this declared distinctness identity.
    DeliberatelyDistinct(OwnerIdentityRef<ApplicationDistinctnessSubject>),
}

/// One application of one template: which template, the canonical argument
/// commitments that fill its holes, the profiles it was applied under, and its
/// distinctness posture.
///
/// # Identity
///
/// Same template + same arguments + same profiles = same meaning. Expansion
/// count, expansion order, formatting, an alias, and the position an
/// application sits at mint nothing: none of them is a member here, so none of
/// them can be read back as a difference.
///
/// The binding set is order-insensitive: nothing identity-bearing may be
/// derived from the order [`TemplateApplication::bindings`] yields, and an
/// applicative identity computed over these bindings canonicalizes by
/// parameter identity first.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateApplication {
    template: OwnerIdentityRef<TemplateSubject>,
    bindings: NonEmptyBounded<TemplateBinding, TemplateParameterLimit>,
    language_profile: VersionedProfile<LanguageProfileSubject>,
    meta_profile: VersionedProfile<MetaProfileSubject>,
    distinctness: ApplicativeDistinctness,
}

// ---------------------------------------------------------------------------
// The invocation key.
// ---------------------------------------------------------------------------

/// The semantic invocation key: the complete set of lawful inputs one template
/// invocation is keyed by.
///
/// Every member is a typed commitment. A cached expansion is disposable: it is
/// an optimization keyed by this value and never a replacement for recomputing
/// under it, so discarding every cache anywhere changes no meaning at all.
///
/// What may never participate is [`INVOCATION_KEY_NEVER`], and it is a roster
/// rather than a warning: none of those facts is a member of this record, so a
/// key that varied with the checkout path or the wall clock is not a key
/// somebody must remember not to build.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateInvocationKey {
    /// The template's semantic identity.
    pub template: OwnerIdentityRef<TemplateSubject>,
    /// The validated input descriptors this invocation commits to.
    pub inputs: Bounded<OwnerIdentityRef<InputDescriptorSubject>, InputDescriptorLimit>,
    /// The exact source snapshot the invocation was read against.
    pub source_snapshot: OwnerIdentityRef<SourceSnapshotSubject>,
    /// The declaration fragments this invocation depends on.
    pub fragment_dependencies:
        Bounded<OwnerIdentityRef<FragmentIdentityDomain>, FragmentDependencyLimit>,
    /// The language profile and version.
    pub language_profile: VersionedProfile<LanguageProfileSubject>,
    /// The meta profile and version.
    pub meta_profile: VersionedProfile<MetaProfileSubject>,
    /// The configuration commitment in force.
    pub configuration: OwnerIdentityRef<ProjectionConfigurationDomain>,
}

/// The closed roster of facts that never participate in an invocation key.
///
/// Each is a way the same declared input could yield two different answers,
/// which is exactly what determinism forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForbiddenKeyFact {
    /// Where the tree happens to be checked out.
    CheckoutPath,
    /// Which directory the caller happened to run from.
    CurrentDirectory,
    /// When a file was last written.
    ModificationTime,
    /// Which process is running.
    ProcessIdentity,
    /// Whatever the ambient environment happens to say.
    AmbientEnvironment,
    /// What the wall clock reads.
    WallTime,
    /// Anything drawn from entropy.
    Entropy,
    /// Which host the work runs on.
    HostAddress,
    /// The order a hash map happened to iterate in.
    MapIterationOrder,
}

/// The declared never-roster of an invocation key, in the order this contract
/// states it.
pub const INVOCATION_KEY_NEVER: [ForbiddenKeyFact; 9] = [
    ForbiddenKeyFact::CheckoutPath,
    ForbiddenKeyFact::CurrentDirectory,
    ForbiddenKeyFact::ModificationTime,
    ForbiddenKeyFact::ProcessIdentity,
    ForbiddenKeyFact::AmbientEnvironment,
    ForbiddenKeyFact::WallTime,
    ForbiddenKeyFact::Entropy,
    ForbiddenKeyFact::HostAddress,
    ForbiddenKeyFact::MapIterationOrder,
];

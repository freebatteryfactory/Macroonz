//! The derive-implementation home's invariant nucleus: every road that reaches
//! a private field, and the one road that turns a pass's established issues into
//! the pair a refusal body is built from.
//!
//! Declared inside `types.rs` as its own child, which is what makes the control
//! structural: [`MutationPointTable::over`] SEATS the control, there is no road
//! that takes one, and the seat is unreachable from anywhere else — so a table
//! without a control is a value nobody can write rather than a state a reader
//! has to notice.
//!
//! Both surfaces are built here for the same reason. A production surface exists
//! only where the plan declared a member at the declaration site under its role,
//! and an evaluation surface exists only where the single walk stood every
//! admitted point in exactly once — so there is no half-composed delivery for a
//! reader to mistake for a whole one, and no parity statement about two surfaces
//! that were never rendered from one plan.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared
//! beside the rest of this home's declarations would put all of them inside the
//! wall. That module's entire content is the record and its inherent
//! implementations, so the module IS the complete set of roads that reach the
//! private seat.
//!
//! A private seat excludes every SIBLING: the rest of this file, `types.rs`
//! above it, `plan.rs` and `render.rs` beside it, anywhere else in the services,
//! and any crate downstream cannot write the literal, and the compiler says so
//! with `E0451`. It does not exclude DESCENDANTS, so the reversal for this seat
//! is a compile-fail fixture outside the crate.

use super::super::plan::{SurfacePlan, surface_plan};
use super::super::render;
use super::{
    EvaluationBinding, ImplementationSurface, ImplementationSurfaceIssue, ImplementationSurfaces,
    MutationClaimRef, MutationEvaluationSurface, MutationOperation, MutationPoint,
    MutationPointLimit, MutationPointName, MutationPointTable, NO_MUTATION_NAMESPACE,
    NO_MUTATION_STEM, NoMutationControl, ProductionSurface, SurfaceDeclarationRefusal,
    SurfaceParity,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, GeneratedUnitSubject, GeneratorVersionSubject, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, ProjectionRole, ProjectionTranscript,
    RenderedRole, RenderedUnitSubject,
};
use crate::planning::{
    CauseAnchoring, DeriveImplProjection, ProjectionPlan, RenderedImplementation,
};
use crate::token::{GeneratedTree, TokenPath};
use std::collections::BTreeSet;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

// ---------------------------------------------------------------------------
// The vocabulary's nuclei.
// ---------------------------------------------------------------------------

impl MutationPointName {
    /// This name, parsed from the owner that declares it and the spelling it
    /// carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem. The checks are dependent
    /// and in that order, so exactly one cause is true of any refused name.
    pub fn named(namespace: &str, stem: &str) -> Result<Self, SurfaceDeclarationRefusal> {
        if namespace.is_empty() {
            return Err(SurfaceDeclarationRefusal::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(SurfaceDeclarationRefusal::EmptyStem);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            stem: stem.to_owned(),
        })
    }

    /// The owner that declares the spelling.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    /// The spelling itself.
    #[must_use]
    pub fn stem(&self) -> &str {
        self.stem.as_str()
    }

    /// Whether this name is the no-mutation control's reserved one.
    ///
    /// The one place the two constants are compared against a name, so the
    /// table's refusal and the control's own seating read the same answer from
    /// the same road.
    #[must_use]
    pub fn is_control(&self) -> bool {
        self.namespace == NO_MUTATION_NAMESPACE && self.stem == NO_MUTATION_STEM
    }
}

impl MutationClaimRef {
    /// The claim reference, over a name already parsed. Total: a parsed name is
    /// a reference, and this road adds nothing to check.
    #[must_use]
    pub fn over(name: MutationPointName) -> Self {
        Self(name)
    }

    /// The namespaced name this reference carries.
    #[must_use]
    pub const fn name(&self) -> &MutationPointName {
        &self.0
    }
}

impl MutationOperation {
    /// One operation, under the spelling the harness reads and the tokens the
    /// rendering substitutes.
    ///
    /// # Errors
    ///
    /// Returns [`SurfaceDeclarationRefusal::OperationEmpty`] where the tree
    /// carries no tokens. An operation with no tokens names no site: a walk
    /// looking for it would match at every position, and a substitution standing
    /// it in would stand nothing in.
    pub fn spelled(
        spelling: MutationPointName,
        tree: GeneratedTree,
    ) -> Result<Self, SurfaceDeclarationRefusal> {
        if tree.is_empty() {
            return Err(SurfaceDeclarationRefusal::OperationEmpty);
        }
        Ok(Self { spelling, tree })
    }

    /// How this operation is NAMED — the data that crosses the wall.
    #[must_use]
    pub const fn spelling(&self) -> &MutationPointName {
        &self.spelling
    }

    /// How this operation is WRITTEN — the tokens a rendering substitutes.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl MutationPoint {
    /// Declare one mutation point.
    ///
    /// # Errors
    ///
    /// Returns [`SurfaceDeclarationRefusal::AlternativesAbsent`] where no
    /// alternative was supplied — a selection among one thing selects nothing —
    /// [`SurfaceDeclarationRefusal::AlternativeSpellingDoubled`] where two of
    /// this point's alternatives carry one spelling, and
    /// [`SurfaceDeclarationRefusal::AlternativesUnbounded`] where they outgrow
    /// the declared magnitude.
    ///
    /// The door reads a runtime count because a caller arrives holding a list;
    /// the VALUE cannot be empty at all, because the seat behind this road
    /// carries a first alternative by signature.
    pub fn declared(
        name: MutationPointName,
        claim: MutationClaimRef,
        original: MutationOperation,
        alternatives: Vec<MutationOperation>,
        activation: TokenPath,
    ) -> Result<Self, SurfaceDeclarationRefusal> {
        let mut supplied = alternatives.into_iter();
        let Some(first) = supplied.next() else {
            return Err(SurfaceDeclarationRefusal::AlternativesAbsent);
        };
        let rest: Vec<MutationOperation> = supplied.collect();
        if spellings_doubled(&first, &rest) {
            return Err(SurfaceDeclarationRefusal::AlternativeSpellingDoubled);
        }
        let admitted = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| SurfaceDeclarationRefusal::AlternativesUnbounded)?;
        Ok(Self {
            name,
            claim,
            original,
            alternatives: admitted,
            activation,
        })
    }

    /// This point's own identity.
    #[must_use]
    pub const fn name(&self) -> &MutationPointName {
        &self.name
    }

    /// The owner claim this point stands under.
    #[must_use]
    pub const fn claim(&self) -> &MutationClaimRef {
        &self.claim
    }

    /// The operation this point is about, as the production surface writes it.
    #[must_use]
    pub const fn original(&self) -> &MutationOperation {
        &self.original
    }

    /// The alternatives admitted against that operation; structurally at least
    /// one.
    pub fn alternatives(&self) -> impl Iterator<Item = &MutationOperation> {
        self.alternatives.iter()
    }

    /// How many alternatives this point admits; structurally at least one.
    #[must_use]
    pub fn alternative_count(&self) -> usize {
        self.alternatives.len()
    }

    /// The route from the root of the captured declaration to the operation
    /// this point activates at.
    #[must_use]
    pub const fn activation(&self) -> &TokenPath {
        &self.activation
    }
}

impl NoMutationControl {
    /// The control, seated under its declared name.
    ///
    /// Total and private: the name is two declared constants, so there is no
    /// count to read and no refusal to return, and the only caller is the
    /// table's own road.
    fn seated() -> Self {
        Self {
            name: MutationPointName {
                namespace: NO_MUTATION_NAMESPACE.to_owned(),
                stem: NO_MUTATION_STEM.to_owned(),
            },
        }
    }

    /// The control's declared name.
    #[must_use]
    pub const fn name(&self) -> &MutationPointName {
        &self.name
    }
}

impl MutationPointTable {
    /// The table over one surface's admitted points, with the control seated at
    /// its first position by this road and by no other.
    ///
    /// # Errors
    ///
    /// Returns the composition family naming
    /// [`ImplementationSurfaceIssue::PointNameDoubled`] for every name two
    /// points carry, [`ImplementationSurfaceIssue::ControlNameClaimed`] for
    /// every point wearing the control's reserved name, and
    /// [`ImplementationSurfaceIssue::PointsUnbounded`] where the admitted set
    /// outgrows the declared magnitude. The naming issues are reported together,
    /// because a caller repairing a table one point per attempt is a caller this
    /// road failed.
    pub fn over(points: Vec<MutationPoint>) -> Result<Self, ImplementationSurfaceComposition> {
        if let Some((first, rest)) = ImplementationSurfaceIssue::established(naming_issues(&points))
        {
            return Err(ImplementationSurfaceComposition::established(first, rest));
        }
        let observed = points.len();
        let admitted = Bounded::admitted_const(
            points,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| {
            ImplementationSurfaceComposition::established(
                ImplementationSurfaceIssue::PointsUnbounded {
                    bound: u64::try_from(MutationPointLimit::MAX).unwrap_or(u64::MAX),
                    observed: u64::try_from(observed).unwrap_or(u64::MAX),
                },
                Vec::new(),
            )
        })?;
        Ok(Self {
            control: NoMutationControl::seated(),
            admitted,
        })
    }

    /// The control at the table's first position.
    #[must_use]
    pub const fn control(&self) -> &NoMutationControl {
        &self.control
    }

    /// The admitted points, after the control, in the order they were declared.
    ///
    /// # Ordering
    ///
    /// This order IS meaning here, unlike a declared output set: a point's
    /// position is what its rendered variants are spelled from, so the same
    /// points supplied in another order render a different active-point roster.
    pub fn admitted(&self) -> impl Iterator<Item = &MutationPoint> {
        self.admitted.iter()
    }

    /// How many rows the table carries: the control, plus every admitted point.
    #[must_use]
    pub fn len(&self) -> usize {
        self.admitted.len().saturating_add(1)
    }

    /// Always `false`: a table without its control is unrepresentable.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl EvaluationBinding {
    /// The binding, over the two spellings the evaluation copy cannot invent.
    ///
    /// # Errors
    ///
    /// Returns [`SurfaceDeclarationRefusal::SpellingNotAnIdentifier`] where
    /// either spelling is not one Rust identifier. A spelling that is not an
    /// identifier renders tokens the consumer's compiler reads as something
    /// else, and the place that failure would surface is a consumer's build with
    /// no idea where the name came from.
    pub fn declared(active_enum: &str, selector: &str) -> Result<Self, SurfaceDeclarationRefusal> {
        if !is_identifier(active_enum) || !is_identifier(selector) {
            return Err(SurfaceDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            active_enum: active_enum.to_owned(),
            selector: selector.to_owned(),
        })
    }

    /// The active-point enum's declared name.
    #[must_use]
    pub fn active_enum(&self) -> &str {
        self.active_enum.as_str()
    }

    /// The name the selector is read through at every activation site.
    #[must_use]
    pub fn selector(&self) -> &str {
        self.selector.as_str()
    }
}

// ---------------------------------------------------------------------------
// The two surfaces.
// ---------------------------------------------------------------------------

impl ProductionSurface {
    /// Which of the two surfaces this is.
    pub const SURFACE: ImplementationSurface = ImplementationSurface::Production;

    /// The production surface, over what the plan decided and what the kind's
    /// renderer produced.
    ///
    /// Total and private: every seat is the plan's own answer or the tree the
    /// caller rendered, and the one question that could fail — whether the plan
    /// landed this member where its ROLE declares, which for the production half
    /// is the declaration site — was asked before this road was reached.
    fn rendered(stated: &SurfacePlan, tree: GeneratedTree) -> Self {
        Self {
            role: stated.role,
            semantic_key: stated.production_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin.clone(),
            tree,
        }
    }

    /// The rendered role this surface stands under.
    #[must_use]
    pub const fn role(&self) -> RenderedImplementation {
        self.role
    }

    /// The planned member's semantic key this surface answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
    }

    /// The profile the plan expected to render it.
    #[must_use]
    pub const fn profile(&self) -> ProjectionIdentity<ProjectionProfileSubject> {
        self.profile
    }

    /// That profile's version.
    #[must_use]
    pub const fn profile_version(&self) -> ProfileVersion {
        self.profile_version
    }

    /// The trail this surface walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The rendered tree — the implementation the normal build compiles.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl MutationEvaluationSurface {
    /// Which of the two surfaces this is.
    pub const SURFACE: ImplementationSurface = ImplementationSurface::MutationEvaluation;

    /// The rendered role this copy stands under — the production member's own.
    #[must_use]
    pub const fn role(&self) -> RenderedImplementation {
        self.role
    }

    /// This copy's own identity, derived over its canonical bytes under the
    /// contract the plan stated.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<RenderedUnitSubject> {
        self.identity
    }

    /// How this copy names its active-point enum and its selector.
    #[must_use]
    pub const fn binding(&self) -> &EvaluationBinding {
        &self.binding
    }

    /// The whole of what is selectable: the control, and every admitted point.
    #[must_use]
    pub const fn table(&self) -> &MutationPointTable {
        &self.table
    }

    /// The rendered copy — the enum, and the implementation with every point
    /// standing under its selection.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl SurfaceParity {
    /// The ONE address both surfaces stand on.
    #[must_use]
    pub const fn declaration(&self) -> CauseAnchoring {
        self.declaration
    }

    /// The ONE rendering engine both surfaces were written by.
    #[must_use]
    pub const fn engine(&self) -> ProjectionIdentity<GeneratorVersionSubject> {
        self.engine
    }

    /// The production member's semantic key.
    #[must_use]
    pub const fn production(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.production
    }

    /// The evaluation copy's own identity.
    #[must_use]
    pub const fn evaluation(&self) -> ProjectionIdentity<RenderedUnitSubject> {
        self.evaluation
    }
}

impl ImplementationSurfaces {
    /// Compose one implementation meaning's two surfaces.
    ///
    /// The order is the road: what the plan decided about BOTH members, then the
    /// table with its control seated, then the evaluation copy transformed out
    /// of the production tree, then the copy's identity over its own planned
    /// key, and only then the parity — which is derived from those values rather
    /// than asserted about them.
    ///
    /// The role a caller hands over names ONE half of a pair, and either half
    /// names the whole: the plan is read for the production member and for its
    /// twin together ([`surface_plan`]), so the two surfaces can never be
    /// composed backwards.
    ///
    /// # Errors
    ///
    /// Returns the composition family naming the plan's disagreement (one half
    /// of the pair was not planned, or its member lands somewhere other than
    /// where its role says), the table's (a doubled name, a point claiming the
    /// control's name, or too many points), or the transform's (a point's
    /// operation absent from the production tree, occurring there more than
    /// once, overlapped by another point's, or a copy past the declared token
    /// magnitude).
    pub fn composed(
        plan: &ProjectionPlan<DeriveImplProjection>,
        role: RenderedImplementation,
        production: GeneratedTree,
        points: Vec<MutationPoint>,
        binding: EvaluationBinding,
    ) -> Result<Self, ImplementationSurfaceComposition> {
        let stated = surface_plan(plan, role)
            .map_err(|issue| ImplementationSurfaceComposition::established(issue, Vec::new()))?;
        let table = MutationPointTable::over(points)?;
        let tree = render::evaluation_copy(&binding, &table, &production)
            .map_err(|(first, rest)| ImplementationSurfaceComposition::established(first, rest))?;
        let identity = evaluation_identity(&stated, &tree);
        Ok(Self {
            production: ProductionSurface::rendered(&stated, production),
            evaluation: MutationEvaluationSurface {
                role: stated.evaluation_role,
                identity,
                binding,
                table,
                tree,
            },
            parity: SurfaceParity {
                declaration: stated.declaration,
                engine: stated.engine,
                production: stated.production_key,
                evaluation: identity,
            },
        })
    }

    /// The implementation the normal build compiles.
    #[must_use]
    pub const fn production(&self) -> &ProductionSurface {
        &self.production
    }

    /// The copy every admitted mutation point is selected from.
    #[must_use]
    pub const fn evaluation(&self) -> &MutationEvaluationSurface {
        &self.evaluation
    }

    /// What the two share, and what that sharing is silent about.
    #[must_use]
    pub const fn parity(&self) -> &SurfaceParity {
        &self.parity
    }
}

impl ImplementationSurfaceIssue {
    /// One pass's established issues as the pair a refusal body is built from,
    /// or nothing where the pass established none.
    ///
    /// Seated here rather than beside a pass because the body is here: a pass
    /// hands over what it found, and the shape a body requires — a first issue
    /// and the rest — is decided once, where bodies are made.
    #[must_use]
    pub fn established(issues: Vec<Self>) -> Option<(Self, Vec<Self>)> {
        let mut walk = issues.into_iter();
        let first = walk.next()?;
        Some((first, walk.collect()))
    }
}

// ---------------------------------------------------------------------------
// The passes that reach a private seat, and the seat itself.
// ---------------------------------------------------------------------------

/// The naming pass: what the admitted points say about each other's identities
/// and about the control's reserved one.
fn naming_issues(points: &[MutationPoint]) -> Vec<ImplementationSurfaceIssue> {
    let mut issues: Vec<ImplementationSurfaceIssue> = Vec::new();
    let mut seen: BTreeSet<&MutationPointName> = BTreeSet::new();
    for point in points {
        if point.name().is_control() {
            issues.push(ImplementationSurfaceIssue::ControlNameClaimed {
                point: point.name().clone(),
            });
        }
        if !seen.insert(point.name()) {
            issues.push(ImplementationSurfaceIssue::PointNameDoubled {
                point: point.name().clone(),
            });
        }
    }
    issues
}

/// Whether two of one point's alternatives carry one spelling.
///
/// Counted rather than walked with an early return, so the answer is one
/// comparison between what was supplied and what was distinct.
fn spellings_doubled(first: &MutationOperation, rest: &[MutationOperation]) -> bool {
    let supplied = rest.len().saturating_add(1);
    let distinct: BTreeSet<&MutationPointName> = core::iter::once(first.spelling())
        .chain(rest.iter().map(MutationOperation::spelling))
        .collect();
    distinct.len() != supplied
}

/// Whether one spelling is a single Rust identifier this home is willing to
/// render.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern
/// rather than a name.
fn is_identifier(spelling: &str) -> bool {
    let mut characters = spelling.chars();
    let Some(head) = characters.next() else {
        return false;
    };
    if !head.is_ascii_alphabetic() && head != '_' {
        return false;
    }
    if spelling == "_" {
        return false;
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// Derive the evaluation copy's identity over exactly the bytes the copy
/// carries, anchored on the copy's OWN planned semantic key at its OWN role's
/// roster position.
///
/// The derivation is the one every planned member's rendered-unit identity is
/// taken under — role [`ProjectionRole::RenderedUnit`], anchored on the member's
/// semantic key, at that member's role slot — so the copy is identified the way
/// [`RenderedUnit::materialized`] identifies anything the plan declared, and
/// this home holds no second rule for it.
///
/// The seats it reads are the PLAN's: the copy is a planned member, so its key
/// and its role are read off the membership rather than borrowed from the
/// production half. Anchoring it on the production member's key would be one
/// identity standing for two members, which is exactly what the role-by-role
/// closure exists to tell apart.
///
/// [`RenderedUnit::materialized`]: crate::closure::RenderedUnit::materialized
fn evaluation_identity(
    stated: &SurfacePlan,
    tree: &GeneratedTree,
) -> ProjectionIdentity<RenderedUnitSubject> {
    let material = tree.canonical_bytes();
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::RenderedUnit,
        &stated.evaluation_key,
        &material,
        stated.evaluation_role.slot(),
    ))
}

pub use seat::ImplementationSurfaceComposition;

mod seat {
    use super::super::{ImplementationSurfaceIssue, SurfaceIssueLimit};
    use crate::plane::AuthoringLimitProfile;
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The surface-composition refusal family body.
    ///
    /// Independent members: several points may be doubled while another claims
    /// the control's name, and several may name operations the production tree
    /// does not contain, so no primary issue is ever elected.
    #[must_use = "a refusal family body carries every disagreement the composition passes established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ImplementationSurfaceComposition {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its pass
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record
        /// hands the whole record back as a literal, so any holder of a body
        /// built for one pass could write it into another pass's refusal. Read
        /// back through [`ImplementationSurfaceComposition::body`].
        body: AdmittedPrefix<ImplementationSurfaceIssue, SurfaceIssueLimit>,
    }

    impl ImplementationSurfaceComposition {
        /// The body a composition pass refuses with.
        ///
        /// Each pass walks its whole subject before a body exists, so the
        /// posture here is about the REPORT rather than the pass: where every
        /// established issue fits the declared bound the body carries all of
        /// them; where it does not, the body carries what the bound holds and
        /// names how many established issues stand outside it.
        ///
        /// Reaches the guard file and no further, so a body exists only where
        /// one of the passes beside it ran.
        pub(super) fn established(
            first: ImplementationSurfaceIssue,
            rest: Vec<ImplementationSurfaceIssue>,
        ) -> Self {
            Self {
                body: AdmittedPrefix::examined_completely(
                    first,
                    rest,
                    &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                    StopBound::DeclaredIssueBound,
                ),
            }
        }

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<ImplementationSurfaceIssue, SurfaceIssueLimit> {
            &self.body
        }
    }
}

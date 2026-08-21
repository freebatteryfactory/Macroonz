//! The host-wrapper home's invariant nucleus: every road that reaches a private
//! field, the composition pass the component roster quantifies, and the one road
//! that turns a pass's established issues into the pair a refusal body is built
//! from.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! walls structural: a shape's stages are seated by one road that refuses an
//! empty roster and a spelling that is not an identifier, and there is no second
//! road that seats them — so a wrapper whose rendered call would not parse in the
//! host target is a value nobody can write rather than a state a reader has to
//! notice.
//!
//! The surface is built here for the same reason. A wrapper surface exists only
//! where the plan declared a member under its role, only where that member is
//! written as a standalone artifact, only where the plan's context binds a host
//! contract, and only where every selected component earned exactly one stage and
//! every stage's component was selected — so there is no half-composed delivery
//! for a reader to mistake for a whole one.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared beside
//! the rest of this home's declarations would put all of them inside the wall.
//! That module's entire content is the record and its inherent implementations,
//! so the module IS the complete set of roads that reach the private seat.

use super::super::plan::host_wrapper_plan;
use super::super::render;
use super::{
    HostTargetLanding, WrapperDeclarationRefusal, WrapperPathRooting, WrapperShape, WrapperStage,
    WrapperStageLimit, WrapperSurface, WrapperSurfaceIssue, WrapperTypePath,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, ByteRoleSubject, GeneratedUnitSubject, OwnerIdentityRef, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit, WrapperComponentLimit,
};
use crate::planning::{
    HostWrapperProjection, MemberDestination, ProjectionPlan, WRAPPER_COMPONENTS, WrapperComponent,
};
use crate::token::GeneratedTree;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

// ---------------------------------------------------------------------------
// The rendered vocabulary's nuclei.
// ---------------------------------------------------------------------------

impl WrapperTypePath {
    /// One type path, rooted as the caller stated and spelled from the segments
    /// it named.
    ///
    /// # Errors
    ///
    /// Returns [`WrapperDeclarationRefusal::PathSegmentsAbsent`] where no segment
    /// was supplied — a path naming nothing names nothing —
    /// [`WrapperDeclarationRefusal::SegmentNotAnIdentifier`] where a segment is
    /// not one Rust identifier, and
    /// [`WrapperDeclarationRefusal::PathSegmentsUnbounded`] where the segments
    /// outgrow the declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true
    /// of any refused path.
    pub fn spelled(
        rooting: WrapperPathRooting,
        segments: Vec<String>,
    ) -> Result<Self, WrapperDeclarationRefusal> {
        let mut supplied = segments.into_iter();
        let Some(first) = supplied.next() else {
            return Err(WrapperDeclarationRefusal::PathSegmentsAbsent);
        };
        let rest: Vec<String> = supplied.collect();
        if !is_wrapper_identifier(first.as_str())
            || rest.iter().any(|segment| !is_wrapper_identifier(segment))
        {
            return Err(WrapperDeclarationRefusal::SegmentNotAnIdentifier);
        }
        let admitted = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| WrapperDeclarationRefusal::PathSegmentsUnbounded)?;
        Ok(Self {
            rooting,
            segments: admitted,
        })
    }

    /// Where this path is rooted.
    #[must_use]
    pub const fn rooting(&self) -> WrapperPathRooting {
        self.rooting
    }

    /// The segments, from the root inward; structurally at least one.
    pub fn segments(&self) -> impl Iterator<Item = &String> {
        self.segments.iter()
    }

    /// How many segments the path carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.len()
    }
}

impl WrapperStage {
    /// Declare one stage of a wrapper shape.
    ///
    /// # Errors
    ///
    /// Returns [`WrapperDeclarationRefusal::EmptyStageRoad`] where the stage
    /// states no road, and
    /// [`WrapperDeclarationRefusal::StageRoadNotAnIdentifier`] where the road is
    /// not one Rust identifier. The two are dependent — there is no alphabet to
    /// check until there are characters — so exactly one is ever established.
    pub fn declared(
        component: WrapperComponent,
        road: &str,
    ) -> Result<Self, WrapperDeclarationRefusal> {
        if road.is_empty() {
            return Err(WrapperDeclarationRefusal::EmptyStageRoad);
        }
        if !is_wrapper_identifier(road) {
            return Err(WrapperDeclarationRefusal::StageRoadNotAnIdentifier);
        }
        Ok(Self {
            component,
            road: road.to_owned(),
        })
    }

    /// The component this stage composes.
    #[must_use]
    pub const fn component(&self) -> WrapperComponent {
        self.component
    }

    /// The road on the host contract's own type this stage calls.
    #[must_use]
    pub fn road(&self) -> &str {
        self.road.as_str()
    }
}

impl WrapperShape {
    /// Declare one complete wrapper shape.
    ///
    /// # Errors
    ///
    /// Returns [`WrapperDeclarationRefusal::EmptyEntrySpelling`] where the entry
    /// states no spelling,
    /// [`WrapperDeclarationRefusal::EntrySpellingNotAnIdentifier`] where that
    /// spelling is not one Rust identifier,
    /// [`WrapperDeclarationRefusal::StagesAbsent`] where no stage was supplied,
    /// and [`WrapperDeclarationRefusal::StagesUnbounded`] where the stages
    /// outgrow the declared magnitude.
    ///
    /// The door reads a runtime count because a caller arrives holding a list;
    /// the VALUE cannot be empty at all, because the seat behind this road
    /// carries a first stage by signature.
    ///
    /// A stage roster naming one component twice is NOT refused here. Whether the
    /// stages agree with the components a plan selected is the composition pass's
    /// question, and answering half of it at this door would report a doubled
    /// component one attempt before reporting the selection it disagreed with.
    pub fn declared(
        host: WrapperTypePath,
        carried: WrapperTypePath,
        refusal: WrapperTypePath,
        entry: &str,
        stages: Vec<WrapperStage>,
    ) -> Result<Self, WrapperDeclarationRefusal> {
        if entry.is_empty() {
            return Err(WrapperDeclarationRefusal::EmptyEntrySpelling);
        }
        if !is_wrapper_identifier(entry) {
            return Err(WrapperDeclarationRefusal::EntrySpellingNotAnIdentifier);
        }
        let mut supplied = stages.into_iter();
        let Some(first) = supplied.next() else {
            return Err(WrapperDeclarationRefusal::StagesAbsent);
        };
        let rest: Vec<WrapperStage> = supplied.collect();
        let stages = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| WrapperDeclarationRefusal::StagesUnbounded)?;
        Ok(Self {
            host,
            carried,
            refusal,
            entry: entry.to_owned(),
            stages,
        })
    }

    /// The host contract's own type, whose roads every stage calls.
    #[must_use]
    pub const fn host(&self) -> &WrapperTypePath {
        &self.host
    }

    /// The type the value threaded through the wrapper stands at.
    #[must_use]
    pub const fn carried(&self) -> &WrapperTypePath {
        &self.carried
    }

    /// The refusal every stage's call is checked into.
    #[must_use]
    pub const fn refusal(&self) -> &WrapperTypePath {
        &self.refusal
    }

    /// The spelling the rendered wrapper is declared under.
    #[must_use]
    pub fn entry(&self) -> &str {
        self.entry.as_str()
    }

    /// The stages, in the order the shape declares them.
    ///
    /// # Ordering
    ///
    /// This order is NOT meaning. A wrapper composes its components in the
    /// PLANE's declared roster order, which `composition_order` in `render.rs`
    /// walks; a caller that reorders its stages renders the same wrapper.
    pub fn stages(&self) -> impl Iterator<Item = &WrapperStage> {
        self.stages.iter()
    }

    /// The stage declared for one component, where exactly one is.
    ///
    /// Hands back the FIRST match, and the composition pass is what makes that
    /// unambiguous: a shape whose stages doubled a component never reaches a
    /// rendering, so the first match is the only match wherever this is read
    /// after the pass agreed.
    #[must_use]
    pub fn staged(&self, component: WrapperComponent) -> Option<&WrapperStage> {
        self.stages
            .iter()
            .find(|declared| declared.component() == component)
    }

    /// How many stages the shape declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.stages.len()
    }
}

impl HostTargetLanding {
    /// The landing one plan declared: the host target's own file, under the byte
    /// role the planned member is written as an artifact beneath.
    ///
    /// Reached from `plan.rs`, which reads the byte role off the planned member's
    /// destination. There is no road that invents one: a byte role this home
    /// chose would be this home deciding which bytes somebody else's target
    /// carries.
    #[must_use]
    pub const fn in_host_target(byte_role: OwnerIdentityRef<ByteRoleSubject>) -> Self {
        Self { byte_role }
    }

    /// The byte role the artifact is written under.
    #[must_use]
    pub const fn byte_role(&self) -> OwnerIdentityRef<ByteRoleSubject> {
        self.byte_role
    }

    /// The destination this landing IS, rebuilt as the plan's own vocabulary.
    ///
    /// Composed rather than stored, so a landing whose destination disagreed with
    /// its byte role is not a value anybody can hold.
    #[must_use]
    pub const fn destination(&self) -> MemberDestination {
        MemberDestination::AsArtifact {
            byte_role: self.byte_role,
        }
    }
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

impl WrapperSurface {
    /// Compose one wrapper surface.
    ///
    /// The order is the road: what the plan decided, then the composition pass
    /// over the plane's component roster, then the rendering — so a surface never
    /// exists that the passes did not agree on.
    ///
    /// # Errors
    ///
    /// Returns the composition family naming the plan's disagreement (the role
    /// was not planned, its member is spliced at the declaration site rather than
    /// written as an artifact, or the context binds no host contract), the
    /// composition pass's (a selected component nobody staged, a stage on a
    /// component nobody selected, or two stages under one component), or the
    /// rendering's (a wrapper past the declared token magnitude).
    ///
    /// The plan pass, the composition pass, and the rendering are DEPENDENT —
    /// there is nothing to compose until the plan has been read and nothing to
    /// render until the composition agrees — so a plan issue never co-establishes
    /// with a component one. Component issues co-establish freely with each
    /// other, which is why the body is a collection.
    pub fn composed(
        plan: &ProjectionPlan<HostWrapperProjection>,
        shape: &WrapperShape,
    ) -> Result<Self, WrapperComposition> {
        let stated = host_wrapper_plan(plan).map_err(sole)?;
        if let Some((first, rest)) =
            WrapperSurfaceIssue::established(composition_issues(&stated.components, shape))
        {
            return Err(WrapperComposition::established(first, rest));
        }
        let order = render::composition_order(&stated.components);
        let tree = render::wrapper_shell(shape, &order).map_err(sole)?;
        let composed = seated(order)?;
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin,
            landing: stated.landing,
            composed,
            tree,
        })
    }

    /// The rendered role this wrapper stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
        self.role
    }

    /// The planned member's semantic key this wrapper answers to.
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

    /// The trail this wrapper walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// Where the rendered artifact lands.
    #[must_use]
    pub const fn landing(&self) -> &HostTargetLanding {
        &self.landing
    }

    /// The components actually composed, in the plane's declared roster order.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: it is the order the rendered wrapper calls the
    /// host's roads in, and a wrapper that decoded before it admitted is a
    /// different wrapper.
    pub fn composed_components(&self) -> impl Iterator<Item = &WrapperComponent> {
        self.composed.iter()
    }

    /// The rendered wrapper — the entry road and the stages the plan composed.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl WrapperSurfaceIssue {
    /// One pass's established issues as the pair a refusal body is built from, or
    /// nothing where the pass established none.
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

/// The composition pass: what the plan's selected components and the shape's
/// declared stages say about each other.
///
/// The PLANE's component roster is the quantifier, in both directions and in one
/// walk — every component is asked whether the plan selects it and whether the
/// shape stages it, and the three disagreements the pair can produce are mutually
/// exclusive, so a component establishes at most one issue and the roster's
/// cardinality is the body's magnitude.
///
/// Walking the roster rather than the plan's own list is what makes a plan that
/// named one component twice harmless here: selection is a MEMBERSHIP question,
/// and asking it once per component answers it once.
fn composition_issues(
    selected: &NonEmptyBounded<WrapperComponent, WrapperComponentLimit>,
    shape: &WrapperShape,
) -> Vec<WrapperSurfaceIssue> {
    let mut issues: Vec<WrapperSurfaceIssue> = Vec::new();
    for component in WRAPPER_COMPONENTS {
        let demanded = selected.iter().any(|named| *named == component);
        let staged = shape
            .stages()
            .filter(|declared| declared.component() == component)
            .count();
        match (demanded, staged) {
            (true, 0) => {
                issues.push(WrapperSurfaceIssue::SelectedComponentNotStaged { component });
            }
            (true, 1) | (false, 0) => {}
            (true, _) => issues.push(WrapperSurfaceIssue::ComponentStageDoubled { component }),
            (false, _) => issues.push(WrapperSurfaceIssue::StageComponentNotSelected { component }),
        }
    }
    issues
}

/// The composed roster, seated under this home's own stage magnitude.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::ComposedSeatBoundExceeded`] where the roster
/// outran the declared magnitude — foreclosed on this seam's own route, since the
/// roster is one filtered walk over a roster the magnitude is sized by, and
/// present so the seat's construction has a truthful road rather than a
/// fabricated one.
fn seated(
    order: Vec<WrapperComponent>,
) -> Result<Bounded<WrapperComponent, WrapperStageLimit>, WrapperComposition> {
    let observed = order.len();
    Bounded::admitted_const(
        order,
        &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
    )
    .map_err(|_| {
        sole(WrapperSurfaceIssue::ComposedSeatBoundExceeded {
            bound: u64::try_from(WrapperStageLimit::MAX).unwrap_or(u64::MAX),
            observed: u64::try_from(observed).unwrap_or(u64::MAX),
        })
    })
}

/// One established issue as the body a refusal is built from.
fn sole(issue: WrapperSurfaceIssue) -> WrapperComposition {
    WrapperComposition::established(issue, Vec::new())
}

/// Whether one spelling is a single Rust identifier this home is willing to
/// render.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern rather
/// than a name. Published from `types.rs` so every road that renders a spelling
/// reads one alphabet.
#[must_use]
pub fn is_wrapper_identifier(spelling: &str) -> bool {
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

pub use seat::WrapperComposition;

mod seat {
    use super::super::{WrapperCompositionIssueLimit, WrapperSurfaceIssue};
    use crate::plane::AuthoringLimitProfile;
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The wrapper-composition refusal family body.
    ///
    /// Independent members: a plan may select several components nobody staged
    /// while a shape stages several nobody selected, so no primary issue is ever
    /// elected.
    #[must_use = "a refusal family body carries every disagreement the composition passes established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct WrapperComposition {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its pass
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record hands
        /// the whole record back as a literal, so any holder of a body built for
        /// one pass could write it into another pass's refusal. Read back through
        /// [`WrapperComposition::body`].
        body: AdmittedPrefix<WrapperSurfaceIssue, WrapperCompositionIssueLimit>,
    }

    impl WrapperComposition {
        /// The body a composition pass refuses with.
        ///
        /// Each pass walks its whole subject before a body exists, so the posture
        /// here is about the REPORT rather than the pass: where every established
        /// issue fits the declared bound the body carries all of them; where it
        /// does not, the body carries what the bound holds and names how many
        /// established issues stand outside it.
        ///
        /// Reaches the guard file and no further, so a body exists only where one
        /// of the passes beside it ran.
        pub(super) fn established(
            first: WrapperSurfaceIssue,
            rest: Vec<WrapperSurfaceIssue>,
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
        /// Borrowed and never owned, for the reason the machine's refusal home
        /// borrows its carry: an owned body is a value a caller can seat under
        /// another refusal, which is the pairing the coupled seat exists to end.
        pub const fn body(
            &self,
        ) -> &AdmittedPrefix<WrapperSurfaceIssue, WrapperCompositionIssueLimit> {
            &self.body
        }
    }
}

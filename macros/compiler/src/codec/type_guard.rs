//! The codec home's invariant nucleus: every road that reaches a private field,
//! and the one road that turns a pass's established issues into the pair a
//! refusal body is built from.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! walls structural: a shape's members are seated by one road that refuses an
//! empty roster and a doubled spelling, and there is no second road that seats
//! them — so a codec whose decode road could not refuse is a value nobody can
//! write rather than a state a reader has to notice.
//!
//! The surface is built here for the same reason. A codec surface exists only
//! where the plan declared a member at the declaration site under its role, and
//! only where no member's spelling collides with a local the decode road
//! declares for itself — so there is no half-composed delivery for a reader to
//! mistake for a whole one.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared
//! beside the rest of this home's declarations would put all of them inside the
//! wall. That module's entire content is the record and its inherent
//! implementations, so the module IS the complete set of roads that reach the
//! private seat.

use super::super::plan::codec_plan;
use super::super::render;
use super::super::type_contract::RESERVED_BINDINGS;
use super::{
    AssemblyPosture, CodecAssembly, CodecDeclarationRefusal, CodecMember, CodecMemberShape,
    CodecPlacement, CodecShape, CodecSurface, CodecSurfaceIssue, CodecTypePath, ModuleSpelling,
    PathRooting,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, GeneratedUnitSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::{CodecDirection, CodecProjection, MemberDestination, ProjectionPlan};
use crate::token::GeneratedTree;
use macroonz::{FieldCardinality, NonEmptyBounded, PositiveLimit};
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// The rendered vocabulary's nuclei.
// ---------------------------------------------------------------------------

impl CodecTypePath {
    /// One type path, rooted as the caller stated and spelled from the segments
    /// it named.
    ///
    /// # Errors
    ///
    /// Returns [`CodecDeclarationRefusal::PathSegmentsAbsent`] where no segment
    /// was supplied — a path naming nothing names nothing —
    /// [`CodecDeclarationRefusal::SegmentNotAnIdentifier`] where a segment is not
    /// one Rust identifier, and
    /// [`CodecDeclarationRefusal::PathSegmentsUnbounded`] where the segments
    /// outgrow the declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true
    /// of any refused path.
    pub fn spelled(
        rooting: PathRooting,
        segments: Vec<String>,
    ) -> Result<Self, CodecDeclarationRefusal> {
        let mut supplied = segments.into_iter();
        let Some(first) = supplied.next() else {
            return Err(CodecDeclarationRefusal::PathSegmentsAbsent);
        };
        let rest: Vec<String> = supplied.collect();
        if !is_codec_identifier(first.as_str())
            || rest.iter().any(|segment| !is_codec_identifier(segment))
        {
            return Err(CodecDeclarationRefusal::SegmentNotAnIdentifier);
        }
        let admitted = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| CodecDeclarationRefusal::PathSegmentsUnbounded)?;
        Ok(Self {
            rooting,
            segments: admitted,
        })
    }

    /// Where this path is rooted.
    #[must_use]
    pub const fn rooting(&self) -> PathRooting {
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

impl ModuleSpelling {
    /// One published module's spelling.
    ///
    /// # Errors
    ///
    /// Returns [`CodecDeclarationRefusal::ModuleSpellingNotAnIdentifier`] where
    /// the spelling is not one Rust identifier.
    pub fn spelled(spelling: &str) -> Result<Self, CodecDeclarationRefusal> {
        if !is_codec_identifier(spelling) {
            return Err(CodecDeclarationRefusal::ModuleSpellingNotAnIdentifier);
        }
        Ok(Self {
            spelling: spelling.to_owned(),
        })
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl CodecMember {
    /// Declare one member of a codec shape.
    ///
    /// # Errors
    ///
    /// Returns [`CodecDeclarationRefusal::EmptyMemberSpelling`] where the member
    /// states no spelling, and
    /// [`CodecDeclarationRefusal::MemberSpellingNotAnIdentifier`] where the
    /// spelling is not one Rust identifier. The two are dependent — there is no
    /// alphabet to check until there are characters — so exactly one is ever
    /// established.
    pub fn declared(
        spelling: &str,
        held_as: CodecTypePath,
        shape: CodecMemberShape,
        cardinality: FieldCardinality,
    ) -> Result<Self, CodecDeclarationRefusal> {
        if spelling.is_empty() {
            return Err(CodecDeclarationRefusal::EmptyMemberSpelling);
        }
        if !is_codec_identifier(spelling) {
            return Err(CodecDeclarationRefusal::MemberSpellingNotAnIdentifier);
        }
        Ok(Self {
            spelling: spelling.to_owned(),
            held_as,
            shape,
            cardinality,
        })
    }

    /// What the owner calls this member.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }

    /// The type this member is held at.
    #[must_use]
    pub const fn held_as(&self) -> &CodecTypePath {
        &self.held_as
    }

    /// How this member is written.
    #[must_use]
    pub const fn shape(&self) -> CodecMemberShape {
        self.shape
    }

    /// How many of this member there are.
    #[must_use]
    pub const fn cardinality(&self) -> FieldCardinality {
        self.cardinality
    }
}

impl CodecAssembly {
    /// The assembly road, under the posture the caller stated.
    ///
    /// # Errors
    ///
    /// Returns [`CodecDeclarationRefusal::EmptyAssemblyRoad`] where the road
    /// states no spelling, and
    /// [`CodecDeclarationRefusal::AssemblyRoadNotAnIdentifier`] where the
    /// spelling is not one Rust identifier.
    pub fn stated(road: &str, posture: AssemblyPosture) -> Result<Self, CodecDeclarationRefusal> {
        if road.is_empty() {
            return Err(CodecDeclarationRefusal::EmptyAssemblyRoad);
        }
        if !is_codec_identifier(road) {
            return Err(CodecDeclarationRefusal::AssemblyRoadNotAnIdentifier);
        }
        Ok(Self {
            road: road.to_owned(),
            posture,
        })
    }

    /// The associated road the decode surface calls.
    #[must_use]
    pub fn road(&self) -> &str {
        self.road.as_str()
    }

    /// The posture that road stands under.
    #[must_use]
    pub const fn posture(&self) -> &AssemblyPosture {
        &self.posture
    }
}

impl CodecShape {
    /// Declare one complete codec shape.
    ///
    /// # Errors
    ///
    /// Returns [`CodecDeclarationRefusal::RefusalSpellingNotAnIdentifier`] where
    /// the rendered refusal's spelling is not one Rust identifier,
    /// [`CodecDeclarationRefusal::MembersAbsent`] where no member was supplied,
    /// [`CodecDeclarationRefusal::MemberSpellingDoubled`] where two members carry
    /// one spelling, and [`CodecDeclarationRefusal::MembersUnbounded`] where the
    /// members outgrow the declared magnitude.
    ///
    /// The door reads a runtime count because a caller arrives holding a list;
    /// the VALUE cannot be empty at all, because the seat behind this road
    /// carries a first member by signature.
    pub fn declared(
        owner: CodecTypePath,
        refusal: &str,
        assembly: CodecAssembly,
        members: Vec<CodecMember>,
    ) -> Result<Self, CodecDeclarationRefusal> {
        if !is_codec_identifier(refusal) {
            return Err(CodecDeclarationRefusal::RefusalSpellingNotAnIdentifier);
        }
        let mut supplied = members.into_iter();
        let Some(first) = supplied.next() else {
            return Err(CodecDeclarationRefusal::MembersAbsent);
        };
        let rest: Vec<CodecMember> = supplied.collect();
        if spellings_doubled(&first, &rest) {
            return Err(CodecDeclarationRefusal::MemberSpellingDoubled);
        }
        let admitted = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| CodecDeclarationRefusal::MembersUnbounded)?;
        Ok(Self {
            owner,
            refusal: refusal.to_owned(),
            assembly,
            members: admitted,
        })
    }

    /// The type the codec is written for.
    #[must_use]
    pub const fn owner(&self) -> &CodecTypePath {
        &self.owner
    }

    /// The spelling the rendered decode refusal is declared under.
    #[must_use]
    pub fn refusal(&self) -> &str {
        self.refusal.as_str()
    }

    /// The road the decoded members are assembled by.
    #[must_use]
    pub const fn assembly(&self) -> &CodecAssembly {
        &self.assembly
    }

    /// The members, in the order the shape declares them.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: it is the order the encode road writes and the
    /// decode road reads, so the same members supplied in another order are a
    /// different byte string for the same value — which is exactly what a
    /// canonical encoding may not have two of.
    pub fn members(&self) -> impl Iterator<Item = &CodecMember> {
        self.members.iter()
    }

    /// How many members the shape declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.members.len()
    }
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

impl CodecSurface {
    /// Where a codec surface lands, stated once as a constant rather than
    /// carried as a seat that could say something else.
    ///
    /// Both admitted placements are expansion deliveries: one splices beside the
    /// owner's item and the other wraps the same tokens in a published module,
    /// and neither writes a byte anywhere.
    pub const DESTINATION: MemberDestination = MemberDestination::AtDeclarationSite;

    /// Compose one codec surface.
    ///
    /// The order is the road: what the plan decided, then the binding pass over
    /// the declared members, then the rendering — so a surface never exists that
    /// the passes did not agree on.
    ///
    /// # Errors
    ///
    /// Returns the composition family naming the plan's disagreement (the role
    /// was not planned, or its member lands somewhere other than the declaration
    /// site), the binding pass's (a member whose spelling is one of the locals
    /// the decode road declares for itself), or the rendering's (a surface past
    /// the declared token magnitude).
    ///
    /// The plan pass and the binding pass are DEPENDENT — there is nothing to
    /// render until the plan has been read — so a plan issue never co-establishes
    /// with a binding one, while binding issues co-establish freely with each
    /// other.
    pub fn composed(
        plan: &ProjectionPlan<CodecProjection>,
        shape: &CodecShape,
        placement: CodecPlacement,
    ) -> Result<Self, CodecComposition> {
        let stated = codec_plan(plan).map_err(sole)?;
        if let Some((first, rest)) = CodecSurfaceIssue::established(binding_issues(shape)) {
            return Err(CodecComposition::established(first, rest));
        }
        let tree = render::codec_surface(shape, &placement, stated.direction).map_err(sole)?;
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin,
            placement,
            covered: stated.direction,
            tree,
        })
    }

    /// The rendered role this surface stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
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

    /// Where the rendered tokens land.
    #[must_use]
    pub const fn placement(&self) -> &CodecPlacement {
        &self.placement
    }

    /// The direction the plan declared, and therefore which roads this surface
    /// carries.
    ///
    /// # Nonclaims
    ///
    /// A surface covering [`CodecDirection::Encode`] carries NO validator, and
    /// says so here rather than leaving a reader to infer it from a tree with no
    /// decode road in it.
    #[must_use]
    pub const fn covered(&self) -> CodecDirection {
        self.covered
    }

    /// The rendered surface — the refusal declaration and whichever roads the
    /// direction covers.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl CodecSurfaceIssue {
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

/// The binding pass: what the declared members say about the locals the decode
/// road declares for itself.
///
/// Every member is asked, and every collision is reported, because a caller
/// repairing a shape one member per attempt is a caller this home failed.
fn binding_issues(shape: &CodecShape) -> Vec<CodecSurfaceIssue> {
    let mut issues: Vec<CodecSurfaceIssue> = Vec::new();
    for member in shape.members() {
        for binding in RESERVED_BINDINGS {
            if member.spelling() == binding {
                issues.push(CodecSurfaceIssue::MemberShadowsRenderedBinding {
                    member: member.spelling().to_owned(),
                    binding,
                });
            }
        }
    }
    issues
}

/// Whether two of one shape's members carry one spelling.
///
/// Counted rather than walked with an early return, so the answer is one
/// comparison between what was supplied and what was distinct.
fn spellings_doubled(first: &CodecMember, rest: &[CodecMember]) -> bool {
    let supplied = rest.len().saturating_add(1);
    let distinct: BTreeSet<&str> = core::iter::once(first.spelling())
        .chain(rest.iter().map(CodecMember::spelling))
        .collect();
    distinct.len() != supplied
}

/// One established issue as the body a refusal is built from.
fn sole(issue: CodecSurfaceIssue) -> CodecComposition {
    CodecComposition::established(issue, Vec::new())
}

/// Whether one spelling is a single Rust identifier this home is willing to
/// render.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern
/// rather than a name. Published from `types.rs` so every road that renders a
/// spelling reads one alphabet.
#[must_use]
pub fn is_codec_identifier(spelling: &str) -> bool {
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

pub use seat::CodecComposition;

mod seat {
    use super::super::{CodecSurfaceIssue, CodecSurfaceIssueLimit};
    use crate::plane::AuthoringLimitProfile;
    use macroonz::{AdmittedPrefix, PositiveLimit, StopBound};

    /// The codec-composition refusal family body.
    ///
    /// Independent members: several members may shadow the decode road's own
    /// bindings at once, so no primary issue is ever elected.
    #[must_use = "a refusal family body carries every disagreement the composition passes established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct CodecComposition {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its pass
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record
        /// hands the whole record back as a literal, so any holder of a body
        /// built for one pass could write it into another pass's refusal. Read
        /// back through [`CodecComposition::body`].
        body: AdmittedPrefix<CodecSurfaceIssue, CodecSurfaceIssueLimit>,
    }

    impl CodecComposition {
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
        pub(super) fn established(first: CodecSurfaceIssue, rest: Vec<CodecSurfaceIssue>) -> Self {
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
        pub const fn body(&self) -> &AdmittedPrefix<CodecSurfaceIssue, CodecSurfaceIssueLimit> {
            &self.body
        }
    }
}

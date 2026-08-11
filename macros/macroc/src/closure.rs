//! What was actually rendered, and the proof that it is what was planned.
//!
//! # Why a plan and a rendering are two values
//!
//! A plan is made before anything exists. It states what WILL be materialized:
//! under which roles, with which semantic keys, landing where, coming from where,
//! and whose digests will be anchored to what. A rendering is what a renderer
//! actually produced: token trees, their bytes, and the digests over those bytes.
//!
//! Collapsing the two is the defect this module exists to make unrepresentable.
//! A plan that carried its own rendered-byte digest would either be carrying a
//! placeholder or carrying a digest from a rendering that already happened — and
//! in the second case, any later "check" compares the value against itself and
//! passes on every input.
//!
//! # The closure is a reconstruction, not an assertion
//!
//! [`ProjectionClosure::proved`] does not ask the renderer whether it obeyed the
//! plan. It **rebuilds the membership out of the rendered units** — role by role,
//! reading each unit's own semantic key, destination, profile, origin, and
//! recomputed digest — and then compares that reconstruction against the
//! membership the plan declared. Every way the two can disagree is a typed
//! refusal naming the role it disagreed at:
//!
//! - a planned role nothing rendered ([`ClosureIssue::MemberMissing`]);
//! - a rendered role nothing planned ([`ClosureIssue::MemberUnplanned`]);
//! - one role rendered twice ([`ClosureIssue::MemberDuplicated`]);
//! - a rendered unit whose origin is not the planned one
//!   ([`ClosureIssue::OriginOrphan`]);
//! - a digest that is not the digest of the bytes actually rendered, under the
//!   contract the plan stated ([`ClosureIssue::DigestMismatch`]);
//! - a unit standing under the right role and answering to a different semantic
//!   key ([`ClosureIssue::SemanticKeyMismatch`]);
//! - a unit rendered under a profile or to a destination the plan did not name
//!   ([`ClosureIssue::MaterializationMismatch`]).
//!
//! **Tokens are emitted only after a closure exists.** Holding one is the proof;
//! there is no partial closure and no closure with a warning attached.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ClosureIssueLimit, ClosureSubject, GeneratedUnitSubject, MembershipLimit, OutputBytesSubject,
    ProfileVersion, ProjectionIdentity, ProjectionPreimage, ProjectionProfileSubject,
    ProjectionRole, RenderedByteLimit, RenderedRole, RenderedUnitSubject,
};
use crate::planning::{
    DigestContract, MemberDestination, PlannedMember, PlannedMembership, PlannedOutput,
};
use crate::token::GeneratedTree;
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{Bounded, NonEmptyBounded, NonEmptyBoundedConstruction};

// ---------------------------------------------------------------------------
// What a renderer produced.
// ---------------------------------------------------------------------------

/// How one rendering failed to materialize a unit at all.
///
/// Distinct from a closure disagreement: nothing has been compared yet. These
/// are the two ways the act of materializing refuses, and both are magnitudes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderingRefusal {
    /// The rendered bytes exceed the declared magnitude. A renderer that would
    /// emit past it refuses rather than materializing part of a unit.
    BytesUnbounded,
    /// The rendering carries more units than the declared membership magnitude
    /// admits.
    UnitsUnbounded,
}

/// One unit a renderer actually materialized.
///
/// Everything a closure needs to rebuild the plan's membership is here and is
/// the RENDERER's own answer: the role it rendered under, the semantic key it
/// answers to, where it lands, the profile it was rendered under, where it came
/// from, the token tree itself, and the digest over that tree's canonical bytes.
///
/// The Rust source text is not a member. It is
/// [`GeneratedTree::inspected`] — a projection of the tree, for a person.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedUnit<R: RenderedRole> {
    role: R,
    identity: ProjectionIdentity<RenderedUnitSubject>,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    destination: MemberDestination,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    tree: GeneratedTree,
    bytes: Bounded<u8, RenderedByteLimit>,
    digest: ProjectionIdentity<OutputBytesSubject>,
}

impl<R: RenderedRole> RenderedUnit<R> {
    /// Materialize one rendered unit from the tree a renderer produced.
    ///
    /// The digest is taken HERE, over the tree's own canonical bytes, under the
    /// contract's anchor. Nothing is supplied by the caller, so a renderer
    /// cannot hand in a digest of bytes it did not emit.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::BytesUnbounded`] when the rendered bytes
    /// exceed the declared magnitude.
    pub fn materialized(
        role: R,
        semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
        destination: MemberDestination,
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        profile_version: ProfileVersion,
        origin: OriginTrail,
        tree: GeneratedTree,
    ) -> Result<Self, RenderingRefusal> {
        let raw = tree.canonical_bytes();
        let digest = ProjectionIdentity::derived(ProjectionPreimage::under_projection(
            ProjectionRole::OutputBytes,
            &semantic_key,
            &raw,
            role.slot(),
        ));
        let identity = ProjectionIdentity::derived(ProjectionPreimage::under_projection(
            ProjectionRole::RenderedUnit,
            &semantic_key,
            &raw,
            role.slot(),
        ));
        let bytes = Bounded::admitted_const(raw).map_err(|_| RenderingRefusal::BytesUnbounded)?;
        Ok(Self {
            role,
            identity,
            semantic_key,
            destination,
            profile,
            profile_version,
            origin,
            tree,
            bytes,
            digest,
        })
    }

    /// The role this unit was rendered under.
    #[must_use]
    pub const fn role(&self) -> R {
        self.role
    }

    /// This rendered unit's own identity.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<RenderedUnitSubject> {
        self.identity
    }

    /// The semantic key this unit answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
    }

    /// The digest over this unit's canonical bytes.
    #[must_use]
    pub const fn digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.digest
    }

    /// Where this unit came from.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The token tree this unit is.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The unit's canonical bytes.
    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        self.bytes.iter()
    }

    /// How many canonical bytes the unit carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the unit rendered nothing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The membership row this unit reconstructs — the renderer's own answer to
    /// what it materialized, in exactly the shape a plan states it.
    #[must_use]
    pub fn reconstructed(&self) -> PlannedMember<R> {
        PlannedMember {
            role: self.role,
            output: PlannedOutput {
                semantic_key: self.semantic_key,
                destination: self.destination,
                origin: self.origin.clone(),
                expected_profile: self.profile,
                expected_profile_version: self.profile_version,
                digest_contract: DigestContract::over(self.semantic_key),
            },
        }
    }

    /// The digest recomputed from the bytes this unit actually carries, under
    /// one stated contract.
    ///
    /// This is what the closure compares against [`RenderedUnit::digest`]: a
    /// digest that does not survive being recomputed under the plan's contract
    /// is a digest of something else.
    #[must_use]
    pub fn digest_under(&self, contract: DigestContract) -> ProjectionIdentity<OutputBytesSubject> {
        let raw: Vec<u8> = self.bytes.iter().copied().collect();
        ProjectionIdentity::derived(ProjectionPreimage::under_projection(
            contract.role,
            &contract.anchored_to,
            &raw,
            self.role.slot(),
        ))
    }
}

/// Everything one renderer produced for one plan.
///
/// Structurally non-empty: a rendering that materialized nothing is not a
/// rendering, and a plan whose membership is non-empty can never close over one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedProjection<R: RenderedRole> {
    units: NonEmptyBounded<RenderedUnit<R>, MembershipLimit>,
}

impl<R: RenderedRole> RenderedProjection<R> {
    /// The one-unit rendering. Total: one unit always fits.
    #[must_use]
    pub fn of_one(unit: RenderedUnit<R>) -> Self {
        Self {
            units: NonEmptyBounded::singleton(unit),
        }
    }

    /// The several-unit rendering.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::UnitsUnbounded`] when the rendering outgrows
    /// the declared membership magnitude.
    pub fn materialized(
        first: RenderedUnit<R>,
        rest: Vec<RenderedUnit<R>>,
    ) -> Result<Self, RenderingRefusal> {
        NonEmptyBounded::admitted_const(first, rest)
            .map(|units| Self { units })
            .map_err(|_| RenderingRefusal::UnitsUnbounded)
    }

    /// The rendered units, in the order the renderer produced them.
    pub fn units(&self) -> impl Iterator<Item = &RenderedUnit<R>> {
        self.units.iter()
    }

    /// How many units were rendered; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Always `false`: an empty rendering is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// The one unit rendered under a role, where exactly one was.
    #[must_use]
    pub fn under(&self, role: R) -> Option<&RenderedUnit<R>> {
        self.units.iter().find(|unit| unit.role() == role)
    }

    /// How many units were rendered under one role.
    #[must_use]
    pub fn count_under(&self, role: R) -> usize {
        self.units.iter().filter(|unit| unit.role() == role).count()
    }

    /// The token tree the whole rendering is, in role-roster order.
    ///
    /// Role order, never rendering order: the roster is declared and the
    /// renderer's own sequencing is not, so what is emitted is stable under a
    /// renderer that happened to produce its units in another order.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::BytesUnbounded`] when the joined tree
    /// outgrows the declared token magnitude.
    pub fn joined_tree(&self) -> Result<GeneratedTree, RenderingRefusal> {
        let mut tokens = Vec::new();
        for role in R::ROLES {
            if let Some(unit) = self.under(*role) {
                tokens.extend(unit.tree().tokens().cloned());
            }
        }
        GeneratedTree::assembled(tokens).map_err(|_| RenderingRefusal::BytesUnbounded)
    }
}

// ---------------------------------------------------------------------------
// The closure.
// ---------------------------------------------------------------------------

/// How one rendering and the plan it claims to materialize disagree.
///
/// Every issue names the ROLE it was established at, because "the membership is
/// wrong" is not an answer anybody can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosureIssue<R: RenderedRole> {
    /// A role the plan declared was not rendered at all.
    MemberMissing {
        /// The planned role nothing materialized.
        role: R,
    },
    /// A role was rendered that the plan never declared — the output firewall's
    /// own reversal.
    MemberUnplanned {
        /// The rendered role nothing planned.
        role: R,
    },
    /// One role was rendered more than once.
    MemberDuplicated {
        /// The doubled role.
        role: R,
        /// How many units stood under it.
        observed: u32,
    },
    /// A rendered unit's origin trail is not the trail the plan declared. A
    /// generated unit that walks back somewhere else is orphaned from the
    /// declaration it claims to project.
    OriginOrphan {
        /// The role whose origin disagreed.
        role: R,
    },
    /// The digest a rendered unit carries is not the digest of the bytes it
    /// actually carries, taken under the contract the plan stated.
    DigestMismatch {
        /// The role whose digest disagreed.
        role: R,
    },
    /// A unit stood under the planned role and answered to a different semantic
    /// key: the right seat, filled by the wrong thing.
    SemanticKeyMismatch {
        /// The role whose semantic key disagreed.
        role: R,
    },
    /// A unit was rendered to a destination or under a profile the plan did not
    /// name.
    MaterializationMismatch {
        /// The role whose materialization disagreed.
        role: R,
    },
}

/// The closure refusal family body.
///
/// Independent members: a rendering may drop one role and orphan another in one
/// pass, and reporting one of them would leave a caller repairing a rendering
/// one role per attempt.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionClosureRefusal<R: RenderedRole> {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<ClosureIssue<R>, ClosureIssueLimit>,
    /// Whether every applicable role was examined.
    pub posture: CompletionPosture,
}

impl<R: RenderedRole> RefusalFamily for ProjectionClosureRefusal<R> {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl<R: RenderedRole> ProjectionClosureRefusal<R> {
    /// The body a closure check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there — it never silently drops the remainder.
    fn established(first: ClosureIssue<R>, rest: Vec<ClosureIssue<R>>) -> Self {
        match NonEmptyBounded::admitted_const(first, rest) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}

/// The proof that what was rendered is what was planned.
///
/// Holding one means: the membership was rebuilt out of the rendered units, and
/// the rebuild equals the plan's declared membership role for role, key for key,
/// origin for origin, and digest for digest. There is no partial closure.
///
/// **Tokens are emitted only from a value of this type.** That is the whole
/// point of the type existing: the road from a declaration to emitted tokens
/// passes through here or it does not exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionClosure<R: RenderedRole> {
    reconstructed: PlannedMembership<R>,
    rendered: RenderedProjection<R>,
    identity: ProjectionIdentity<ClosureSubject>,
}

impl<R: RenderedRole> ProjectionClosure<R> {
    /// Prove the closure between one plan's membership and one rendering.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionClosureRefusal`] naming every role the two disagree
    /// at. All of them are reported together: a caller repairing a rendering one
    /// role per attempt is a caller the check failed.
    pub fn proved(
        planned: &PlannedMembership<R>,
        rendered: RenderedProjection<R>,
    ) -> Result<Self, ProjectionClosureRefusal<R>> {
        let mut issues: Vec<ClosureIssue<R>> = Vec::new();
        let mut rebuilt: Vec<PlannedMember<R>> = Vec::new();

        for role in R::ROLES {
            let role = *role;
            let rendered_count = rendered.count_under(role);
            if rendered_count > 1 {
                issues.push(ClosureIssue::MemberDuplicated {
                    role,
                    observed: u32::try_from(rendered_count).unwrap_or(u32::MAX),
                });
                continue;
            }
            match (planned.under(role), rendered.under(role)) {
                (Some(_), None) => issues.push(ClosureIssue::MemberMissing { role }),
                (None, Some(_)) => issues.push(ClosureIssue::MemberUnplanned { role }),
                (None, None) => {}
                (Some(member), Some(unit)) => {
                    let reconstruction = unit.reconstructed();
                    if reconstruction.output.semantic_key != member.output.semantic_key {
                        issues.push(ClosureIssue::SemanticKeyMismatch { role });
                    } else if reconstruction.output.origin != member.output.origin {
                        issues.push(ClosureIssue::OriginOrphan { role });
                    } else if unit.digest_under(member.output.digest_contract) != unit.digest() {
                        issues.push(ClosureIssue::DigestMismatch { role });
                    } else if reconstruction.output.destination != member.output.destination
                        || reconstruction.output.expected_profile != member.output.expected_profile
                        || reconstruction.output.expected_profile_version
                            != member.output.expected_profile_version
                    {
                        issues.push(ClosureIssue::MaterializationMismatch { role });
                    } else {
                        rebuilt.push(reconstruction);
                    }
                }
            }
        }

        let mut established = issues.into_iter();
        if let Some(first) = established.next() {
            return Err(ProjectionClosureRefusal::established(
                first,
                established.collect(),
            ));
        }

        let mut rows = rebuilt.into_iter();
        let Some(first) = rows.next() else {
            // Unreachable while a plan's membership is structurally non-empty:
            // a non-empty plan either matched a role or established an issue.
            // Stated as a refusal rather than as an assumption.
            return Err(ProjectionClosureRefusal::established(
                ClosureIssue::MemberMissing {
                    role: planned.first().role,
                },
                Vec::new(),
            ));
        };
        let reconstructed = PlannedMembership::declared(first, rows.collect()).map_err(|_| {
            ProjectionClosureRefusal::established(
                ClosureIssue::MemberDuplicated {
                    role: planned.first().role,
                    observed: u32::MAX,
                },
                Vec::new(),
            )
        })?;

        let mut material: Vec<u8> = Vec::new();
        for role in R::ROLES {
            if let Some(unit) = rendered.under(*role) {
                material.extend_from_slice(unit.digest().as_bytes());
            }
        }
        let identity = ProjectionIdentity::derived(ProjectionPreimage::under_projection(
            ProjectionRole::Closure,
            &planned.first().output.semantic_key,
            &material,
            0,
        ));

        Ok(Self {
            reconstructed,
            rendered,
            identity,
        })
    }

    /// The membership rebuilt out of the rendered units.
    #[must_use]
    pub const fn reconstructed(&self) -> &PlannedMembership<R> {
        &self.reconstructed
    }

    /// What the renderer produced.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<R> {
        &self.rendered
    }

    /// This closure's own identity. Inspection and emission both read THIS
    /// value, so there is no second closure identity anywhere to disagree with.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<ClosureSubject> {
        self.identity
    }
}

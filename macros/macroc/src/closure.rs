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
//!   ([`ClosureIssue::MaterializationMismatch`]);
//! - a role the PLAN ITSELF declared twice ([`ClosureIssue::MemberPlannedTwice`]);
//! - a rebuild that is not the planned membership as a complete SET
//!   ([`ClosureIssue::MembershipDisagreement`]).
//!
//! **Tokens are emitted only FROM a closure.** The closure joins the rendered
//! units in role-roster order, keeps the resulting tree, and commits to its
//! digest inside its own identity — so the exact byte stream a caller emits is
//! part of what was proved rather than something assembled afterwards. Holding a
//! closure is the proof; there is no partial closure and no closure with a
//! warning attached.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ClosureId, ClosureIssueLimit, GeneratedUnitSubject, MembershipLimit, OutputBytesSubject,
    PlanId, ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, ProjectionProvenance,
    ProjectionRole, ProjectionTranscript, RenderedByteLimit, RenderedRole, RenderedUnitSubject,
    encode_bytes, encode_length,
};
use crate::planning::{
    DigestContract, MemberDestination, PlannedMember, PlannedMembership, PlannedOutput,
};
use crate::question::EXPLANATION_PROTOCOL_VERSION;
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
        let digest = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::OutputBytes,
            &semantic_key,
            &raw,
            role.slot(),
        ));
        let identity = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
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
        ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            contract.role,
            &contract.anchored_to,
            &raw,
            self.role.slot(),
        ))
    }

    /// Append this unit's canonical bytes: the role it stood under, its own
    /// identity, the semantic key it answers to, where it landed, the profile
    /// and version it was rendered under, where it came from, and the digest of
    /// the bytes it carries.
    ///
    /// The rendered bytes themselves are not written. They do not need to be:
    /// the digest is derived over them at full width, so a byte that changed
    /// changes the digest and therefore this encoding.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.role.slot().to_be_bytes());
        encode_bytes(self.identity.as_bytes(), into);
        encode_bytes(self.semantic_key.as_bytes(), into);
        self.destination.encode_into(into);
        encode_bytes(self.profile.as_bytes(), into);
        into.extend_from_slice(&self.profile_version.position().to_be_bytes());
        self.origin.encode_into(into);
        encode_bytes(self.digest.as_bytes(), into);
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

    /// The rendering of a roster fixed by its own shape — a *total structural*
    /// constructor, for the same reason [`PlannedMembership::complete`] is one.
    ///
    /// A renderer that knows before it starts exactly which roles it will
    /// materialize has no runtime count to read, so there is no refusal here to
    /// swallow and no shorter rendering to fall back to.
    #[must_use]
    pub fn complete<const N: usize>(first: RenderedUnit<R>, rest: [RenderedUnit<R>; N]) -> Self {
        Self {
            units: NonEmptyBounded::from_array(first, rest),
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
    /// # It is a step INSIDE the proof, not a road beside it
    ///
    /// This is crate-internal and has exactly one caller:
    /// [`ProjectionClosure::proved`]. It used to be public, and the compile road
    /// called it AFTER the closure was proved — so the exact token stream a
    /// compiler was handed was a concatenation performed past the proof
    /// boundary, over which the closure identity said nothing. A second caller
    /// joining the same units in another order, or joining a subset, would have
    /// produced a different emission that the same closure still vouched for.
    /// Now the closure performs the join, owns the result, and commits to its
    /// digest, and there is no second road to a joined tree at all.
    ///
    /// # Errors
    ///
    /// Returns [`RenderingRefusal::BytesUnbounded`] when the joined tree
    /// outgrows the declared token magnitude.
    pub(crate) fn joined_tree(&self) -> Result<GeneratedTree, RenderingRefusal> {
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
/// Every issue that is ABOUT a role names it, because "the membership is wrong"
/// is not an answer anybody can act on. The three that are about the whole
/// reconstruction — an empty rebuild, a rebuild that will not declare, and a
/// joined tree past its magnitude — name none, and that is the honest shape:
/// there is no role to name, and electing one to fill the seat would be exactly
/// the neighbouring-value repair this roster exists to refuse.
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
    /// The PLAN declared one role twice. Independent of what was rendered: a
    /// membership carrying two members under one role makes the role-to-unit
    /// match elect one of them, and a proof that elected its own subject proves
    /// nothing.
    MemberPlannedTwice {
        /// The doubled role.
        role: R,
        /// How many members the plan declared under it.
        observed: u32,
    },
    /// The membership rebuilt out of the rendered units and the membership the
    /// plan declared are not the same SET under this role.
    ///
    /// The final theorem, checked as sets rather than as first-per-role pairs: a
    /// pairwise walk that compared one member per role would agree about two
    /// memberships that differ in their second.
    MembershipDisagreement {
        /// The role the two sets disagree under.
        role: R,
    },
    /// The rebuild produced no member at all.
    ReconstructionEmpty,
    /// The rebuild produced members that will not declare as a complete output
    /// set.
    ReconstructionUndeclarable {
        /// How many members the rebuild produced.
        observed: u32,
    },
    /// The joined token tree the rendering amounts to outgrows the declared
    /// token magnitude. Established DURING the proof, because the closure owns
    /// the join.
    JoinedTreeUnbounded,
}

impl<R: RenderedRole> ClosureIssue<R> {
    /// The issue kind's position in the declared roster, written ahead of the
    /// issue's own material so two kinds never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::MemberMissing { .. } => 0,
            Self::MemberUnplanned { .. } => 1,
            Self::MemberDuplicated { .. } => 2,
            Self::OriginOrphan { .. } => 3,
            Self::DigestMismatch { .. } => 4,
            Self::SemanticKeyMismatch { .. } => 5,
            Self::MaterializationMismatch { .. } => 6,
            Self::MemberPlannedTwice { .. } => 7,
            Self::MembershipDisagreement { .. } => 8,
            Self::ReconstructionEmpty => 9,
            Self::ReconstructionUndeclarable { .. } => 10,
            Self::JoinedTreeUnbounded => 11,
        }
    }

    /// The role this issue was established at, where it is about one.
    #[must_use]
    pub const fn role(&self) -> Option<R> {
        match self {
            Self::MemberMissing { role }
            | Self::MemberUnplanned { role }
            | Self::MemberDuplicated { role, .. }
            | Self::OriginOrphan { role }
            | Self::DigestMismatch { role }
            | Self::SemanticKeyMismatch { role }
            | Self::MaterializationMismatch { role }
            | Self::MemberPlannedTwice { role, .. }
            | Self::MembershipDisagreement { role } => Some(*role),
            Self::ReconstructionEmpty
            | Self::ReconstructionUndeclarable { .. }
            | Self::JoinedTreeUnbounded => None,
        }
    }

    /// How the two disagreed, rendered for a person. A projection of the typed
    /// value: nothing reads it back.
    #[must_use]
    pub const fn described(&self) -> &'static str {
        match self {
            Self::MemberMissing { .. } => "a planned role nothing materialized",
            Self::MemberUnplanned { .. } => "a rendered role nothing planned",
            Self::MemberDuplicated { .. } => "a role rendered more than once",
            Self::OriginOrphan { .. } => "a rendered unit whose origin is not the planned one",
            Self::DigestMismatch { .. } => "a digest that is not the digest of the bytes rendered",
            Self::SemanticKeyMismatch { .. } => "the planned role, answering to another key",
            Self::MaterializationMismatch { .. } => {
                "a destination or profile the plan did not name"
            }
            Self::MemberPlannedTwice { .. } => "a role the plan itself declared twice",
            Self::MembershipDisagreement { .. } => {
                "the rebuilt membership and the planned one are not the same set under this role"
            }
            Self::ReconstructionEmpty => "the rebuild produced no member at all",
            Self::ReconstructionUndeclarable { .. } => {
                "the rebuild will not declare as a complete output set"
            }
            Self::JoinedTreeUnbounded => "the joined token tree outgrows its declared magnitude",
        }
    }
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

/// The per-role pass: every issue the two establish at a role, and the members
/// rebuilt at the roles where they agreed.
///
/// The roster is the quantifier. Every role the kind declares is examined, in
/// roster order, and a role that establishes an issue contributes no rebuilt
/// member — so a rebuild is never a partial reading of a disagreement.
fn examined<R: RenderedRole>(
    planned: &PlannedMembership<R>,
    rendered: &RenderedProjection<R>,
) -> (Vec<ClosureIssue<R>>, Vec<PlannedMember<R>>) {
    let mut issues: Vec<ClosureIssue<R>> = Vec::new();
    let mut rebuilt: Vec<PlannedMember<R>> = Vec::new();
    for role in R::ROLES {
        let role = *role;
        // What the PLAN declared under the role is checked in its own right, and
        // before anything is compared. Today every role a plan declares is
        // declared exactly once, so a planned count of two is a defect in the
        // plan rather than a shape the check has to accommodate — and reading
        // the plan's own count through `under`, which yields the first match,
        // would have hidden it.
        let planned_count = planned.count_under(role);
        if planned_count > 1 {
            issues.push(ClosureIssue::MemberPlannedTwice {
                role,
                observed: u32::try_from(planned_count).unwrap_or(u32::MAX),
            });
            continue;
        }
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
    (issues, rebuilt)
}

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for every pass in [`ProjectionClosure::proved`], so no pass can
/// establish issues and then walk on past them.
fn refused<R: RenderedRole>(issues: Vec<ClosureIssue<R>>) -> Option<ProjectionClosureRefusal<R>> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ProjectionClosureRefusal::established(
        first,
        established.collect(),
    ))
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
    plan: PlanId,
    reconstructed: PlannedMembership<R>,
    rendered: RenderedProjection<R>,
    emitted: GeneratedTree,
    emitted_digest: ProjectionIdentity<OutputBytesSubject>,
    identity: ClosureId,
    provenance: ProjectionProvenance,
}

impl<R: RenderedRole> ProjectionClosure<R> {
    /// Prove the closure between one plan's membership and one rendering.
    ///
    /// # The closure transcript
    ///
    /// The identity is derived under [`ProjectionRole::Closure`], anchored on
    /// the PLAN's own identity, over a content transcript that commits to the
    /// COMPLETE closure claim, in this order:
    ///
    /// 1. the explanation protocol version
    ///    ([`EXPLANATION_PROTOCOL_VERSION`]) — a closure claims a rendering
    ///    answers a protocol, and a claim made under a different protocol is a
    ///    different claim;
    /// 2. the full planned membership, in role-roster order — every semantic
    ///    key, destination, origin trail, expected profile and version, and
    ///    digest contract the plan declared;
    /// 3. the role roster's own length;
    /// 4. for every role in roster order: the role slot, how many units stood
    ///    under it, and the unit that did — its identity, semantic key,
    ///    destination, profile and version, origin trail, and digest;
    /// 5. the digest of the EMITTED joined tree, at full width.
    ///
    /// So the identity names the whole agreement rather than a sample of it.
    /// The earlier design anchored on the first planned member's semantic key
    /// and hashed the concatenated digests: it committed to no destination, no
    /// origin, no profile, no plan, and — because bare concatenation admits two
    /// splits of one byte string — not reliably to the digest sequence either.
    ///
    /// # Member 5 is why the emission is inside the proof
    ///
    /// The joined tree is built HERE, from the rendered units in role-roster
    /// order, and the closure keeps it. The compile road used to join the units
    /// itself after the proof returned, which meant the exact token stream the
    /// compiler was handed was assembled past the proof boundary and the closure
    /// identity said nothing about it. Committing to the joined tree's digest
    /// closes that: the bytes a caller emits are the bytes this identity names.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionClosureRefusal`] naming every role the two disagree
    /// at. All of them are reported together: a caller repairing a rendering one
    /// role per attempt is a caller the check failed.
    pub fn proved(
        plan: PlanId,
        planned: &PlannedMembership<R>,
        rendered: RenderedProjection<R>,
    ) -> Result<Self, ProjectionClosureRefusal<R>> {
        let (issues, rebuilt) = examined(planned, &rendered);
        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }

        let observed = u32::try_from(rebuilt.len()).unwrap_or(u32::MAX);
        let mut rows = rebuilt.into_iter();
        let Some(first) = rows.next() else {
            return Err(ProjectionClosureRefusal::established(
                ClosureIssue::ReconstructionEmpty,
                Vec::new(),
            ));
        };
        let reconstructed = PlannedMembership::declared(first, rows.collect()).map_err(|_| {
            ProjectionClosureRefusal::established(
                ClosureIssue::ReconstructionUndeclarable { observed },
                Vec::new(),
            )
        })?;

        // The theorem, stated over the whole set: role by role, the rebuild and
        // the plan hold the same members. Every check above is about one seat;
        // this one is about the collection, and it is what a first-per-role walk
        // could never establish.
        let disagreements: Vec<ClosureIssue<R>> = R::ROLES
            .iter()
            .copied()
            .filter(|role| !reconstructed.agrees_under(planned, *role))
            .map(|role| ClosureIssue::MembershipDisagreement { role })
            .collect();
        if let Some(refusal) = refused(disagreements) {
            return Err(refusal);
        }

        let emitted = rendered.joined_tree().map_err(|_| {
            ProjectionClosureRefusal::established(ClosureIssue::JoinedTreeUnbounded, Vec::new())
        })?;
        let emitted_bytes = emitted.canonical_bytes();
        let emitted_digest = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::OutputBytes,
            &plan,
            &emitted_bytes,
            0,
        ));

        let mut material: Vec<u8> = Vec::new();
        material.extend_from_slice(&EXPLANATION_PROTOCOL_VERSION.to_be_bytes());
        planned.encode_into(&mut material);
        encode_length(R::ROLES.len(), &mut material);
        for role in R::ROLES {
            material.extend_from_slice(&role.slot().to_be_bytes());
            encode_length(rendered.count_under(*role), &mut material);
            if let Some(unit) = rendered.under(*role) {
                unit.encode_into(&mut material);
            }
        }
        encode_bytes(emitted_digest.as_bytes(), &mut material);
        let (identity, provenance) = ClosureId::derived_with_provenance(
            ProjectionTranscript::under_projection(ProjectionRole::Closure, &plan, &material, 0),
        );

        Ok(Self {
            plan,
            reconstructed,
            rendered,
            emitted,
            emitted_digest,
            identity,
            provenance,
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

    /// The token tree this closure proved, joined in role-roster order and owned
    /// here.
    ///
    /// The one road to emitted tokens. Nothing joins the rendered units a second
    /// time, and the digest this closure's identity commits to is the digest of
    /// exactly these bytes.
    #[must_use]
    pub const fn emitted(&self) -> &GeneratedTree {
        &self.emitted
    }

    /// The digest of the emitted joined tree, as this closure's identity commits
    /// to it.
    #[must_use]
    pub const fn emitted_digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.emitted_digest
    }

    /// The plan this closure was proved against.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// This closure's own identity. Inspection and emission both read THIS
    /// value, so there is no second closure identity anywhere to disagree with.
    #[must_use]
    pub const fn identity(&self) -> ClosureId {
        self.identity
    }

    /// How this closure's identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }
}

//! The plane's declarative trait implementations and the constant answers its
//! closed rosters settle: the sole-unit rendered role, the profile every
//! preimage family versions itself under, and the family every identity role
//! stands in.
//!
//! A constant roster, a constant slot, a constant sentence, and two constant
//! answers — nothing here is computed.
//!
//! Constant answers and never derivations: a family admitted later stops the
//! compiler in the first `match` below until somebody declares its profile, and
//! a role admitted later stops it in the second until somebody says which
//! grammar the role's preimage belongs to. That is the whole reason both
//! answers are written over closed rosters rather than looked up: a role with
//! no family and a family with no version are both unrepresentable rather than
//! defaulted.

use super::{
    BUNDLE_IDENTITY_PROFILE, CAPTURED_DECLARATION_IDENTITY_PROFILE,
    CLOSED_EXPANSION_IDENTITY_PROFILE, CLOSURE_IDENTITY_PROFILE,
    DECLARATION_DOCUMENTATION_IDENTITY_PROFILE, DECLARED_NAME_IDENTITY_PROFILE,
    DIAGNOSTIC_RELATION_IDENTITY_PROFILE, EXPLANATION_IDENTITY_PROFILE,
    GENERATED_UNIT_IDENTITY_PROFILE, GENERATOR_VERSION_IDENTITY_PROFILE, IdentityProfile,
    MUTATION_DECLARATION_IDENTITY_PROFILE, ORIGIN_NODE_IDENTITY_PROFILE,
    PLAN_IDENTITY_PROFILE, PROJECTION_INTENT_IDENTITY_PROFILE, PreimageFamily, ProjectionRole,
    RENDERED_UNIT_IDENTITY_PROFILE, RenderedRole, RenderedRoleSeal, SoleRenderedUnit,
    TRIAL_DECLARATION_IDENTITY_PROFILE,
};

impl RenderedRole for SoleRenderedUnit {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[Self::Sole];

    fn slot(self) -> u32 {
        0
    }

    fn described(self) -> &'static str {
        "the kind's one rendered unit"
    }
}

impl PreimageFamily {
    /// The profile this family derives under: its declared stem segment and its
    /// own version position.
    ///
    /// Total, and the ONE road from a family to a version. Every derivation in
    /// the plane reaches its ladder through here, so a family's position is
    /// stated at exactly one seat — the constant this answers with — and a bump
    /// is an edit to that constant's declaration rather than to a number
    /// somebody found at a call site.
    ///
    /// # Bounds
    ///
    /// It is a bijection today and is not required to stay one: two families
    /// could never share a profile, because a profile carries the family that
    /// declared it. What the answer settles is the version; what separates the
    /// key spaces is the family segment inside it.
    #[must_use]
    pub const fn profile(self) -> IdentityProfile {
        match self {
            Self::CapturedDeclaration => CAPTURED_DECLARATION_IDENTITY_PROFILE,
            Self::ProjectionIntent => PROJECTION_INTENT_IDENTITY_PROFILE,
            Self::Plan => PLAN_IDENTITY_PROFILE,
            Self::OriginNode => ORIGIN_NODE_IDENTITY_PROFILE,
            Self::GeneratedUnit => GENERATED_UNIT_IDENTITY_PROFILE,
            Self::RenderedUnit => RENDERED_UNIT_IDENTITY_PROFILE,
            Self::Bundle => BUNDLE_IDENTITY_PROFILE,
            Self::Closure => CLOSURE_IDENTITY_PROFILE,
            Self::Explanation => EXPLANATION_IDENTITY_PROFILE,
            Self::ClosedExpansion => CLOSED_EXPANSION_IDENTITY_PROFILE,
            Self::DeclarationDocumentation => DECLARATION_DOCUMENTATION_IDENTITY_PROFILE,
            Self::DeclaredName => DECLARED_NAME_IDENTITY_PROFILE,
            Self::GeneratorVersion => GENERATOR_VERSION_IDENTITY_PROFILE,
            Self::DiagnosticRelation => DIAGNOSTIC_RELATION_IDENTITY_PROFILE,
            Self::TrialDeclaration => TRIAL_DECLARATION_IDENTITY_PROFILE,
            Self::MutationDeclaration => MUTATION_DECLARATION_IDENTITY_PROFILE,
        }
    }
}

impl ProjectionRole {
    /// The preimage family a transcript at this role stands in.
    ///
    /// Total, and the ONE road from a mint site to a version ladder: a call
    /// site names the role it is deriving for, and the profile follows from it
    /// rather than being passed alongside it. A road that took both would admit
    /// a rendered unit derived under the plan family's position, which is a
    /// disagreement no reader of the resulting bytes could see.
    ///
    /// # Bounds
    ///
    /// It is a total function and never an injection. Two roles standing over
    /// ONE preimage grammar answer with one family and are separated inside it
    /// by the role, which is a member of the transcript and a segment of the
    /// derive-key context:
    /// [`ProjectionRole::RenderedUnit`] names a rendered unit and
    /// [`ProjectionRole::OutputBytes`] names the digest of exactly that unit's
    /// bytes, both over the same rendered material, so both read to
    /// [`PreimageFamily::RenderedUnit`] and neither collides with the other.
    ///
    /// Nothing reads back: a family names no role, because a family is a
    /// grammar several roles may stand over and a role is one seat inside it.
    ///
    /// # Every declared family is reached
    ///
    /// The five roles at the end of the roster were added so that five preimages
    /// stopped standing on a neighbour's grammar: an explanation and a captured
    /// declaration's documentation are their own facts, and a declared name, the
    /// generator version, and a diagnostic's related identities are preimages
    /// that rode the PLAN and CLOSED-EXPANSION ladders while holding no member
    /// of either grammar. A preimage on a neighbour's ladder is renamed by that
    /// neighbour's bumps, which is a rename nobody's meaning moved by.
    #[must_use]
    pub const fn family(self) -> PreimageFamily {
        match self {
            Self::CapturedDeclaration => PreimageFamily::CapturedDeclaration,
            Self::Plan => PreimageFamily::Plan,
            Self::OriginNode => PreimageFamily::OriginNode,
            Self::GeneratedUnit => PreimageFamily::GeneratedUnit,
            Self::RenderedUnit | Self::OutputBytes => PreimageFamily::RenderedUnit,
            Self::Bundle => PreimageFamily::Bundle,
            Self::Closure => PreimageFamily::Closure,
            Self::ClosedExpansion => PreimageFamily::ClosedExpansion,
            Self::ProjectionIntent => PreimageFamily::ProjectionIntent,
            Self::Explanation => PreimageFamily::Explanation,
            Self::DeclarationDocumentation => PreimageFamily::DeclarationDocumentation,
            Self::DeclaredName => PreimageFamily::DeclaredName,
            Self::GeneratorVersion => PreimageFamily::GeneratorVersion,
            Self::DiagnosticRelation => PreimageFamily::DiagnosticRelation,
            Self::TrialDeclaration => PreimageFamily::TrialDeclaration,
            Self::MutationDeclaration => PreimageFamily::MutationDeclaration,
        }
    }
}

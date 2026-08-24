//! The constant answers the identity home's two closed rosters settle.
//!
//! A role's declared name, its published slot, the sentence it reads as, and the grammar it stands in are four tables over one roster; an anchoring's discriminant and its commitment are two over the other.
//! Each is total, so a row admitted later stops the compiler in every table until somebody says what that row's name, slot, sentence, and grammar are — a role with no grammar and a role with no slot are both unrepresentable rather than defaulted.
//!
//! Nothing here is computed, and nothing here reaches a private field.

use super::{
    Anchoring, BUNDLE_PROFILE, CAPTURED_DECLARATION_PROFILE, CAPTURED_HELPER_PROFILE,
    CLOSED_EXPANSION_PROFILE, CLOSURE_PROFILE, DECLARATION_DOCUMENTATION_PROFILE,
    DECLARED_NAME_PROFILE, DIAGNOSTIC_RELATION_PROFILE, EXPLANATION_PROFILE,
    GENERATED_UNIT_PROFILE, GENERATOR_VERSION_PROFILE, ORIGIN_NODE_PROFILE, PLAN_PROFILE,
    PROJECTION_INTENT_PROFILE, Profile, RENDERED_UNIT_PROFILE, Role,
};

impl Role {
    /// The complete roster, in slot order.
    pub const ALL: &'static [Self] = &[
        Self::CapturedDeclaration,
        Self::Plan,
        Self::OriginNode,
        Self::GeneratedUnit,
        Self::RenderedUnit,
        Self::OutputBytes,
        Self::Bundle,
        Self::Closure,
        Self::ClosedExpansion,
        Self::ProjectionIntent,
        Self::Explanation,
        Self::DeclarationDocumentation,
        Self::DeclaredName,
        Self::GeneratorVersion,
        Self::DiagnosticRelation,
        Self::CapturedHelper,
    ];

    /// The role's declared segment of the derive-key context.
    ///
    /// Declared rather than taken from the Rust spelling, for the reason [a subject's name](super::Subject::NAME) is: changing one of these literals renames every identity ever derived at that seat.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CapturedDeclaration => "captured-declaration",
            Self::Plan => "plan",
            Self::OriginNode => "origin-node",
            Self::GeneratedUnit => "generated-unit",
            Self::RenderedUnit => "rendered-unit",
            Self::OutputBytes => "output-bytes",
            Self::Bundle => "bundle",
            Self::Closure => "closure",
            Self::ClosedExpansion => "closed-expansion",
            Self::ProjectionIntent => "projection-intent",
            Self::Explanation => "explanation",
            Self::DeclarationDocumentation => "declaration-documentation",
            Self::DeclaredName => "declared-name",
            Self::GeneratorVersion => "generator-version",
            Self::DiagnosticRelation => "diagnostic-relation",
            Self::CapturedHelper => "captured-helper",
        }
    }

    /// The published byte member six of every transcript carries for this role.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::CapturedDeclaration => 0,
            Self::Plan => 1,
            Self::OriginNode => 2,
            Self::GeneratedUnit => 3,
            Self::RenderedUnit => 4,
            Self::OutputBytes => 5,
            Self::Bundle => 6,
            Self::Closure => 7,
            Self::ClosedExpansion => 8,
            Self::ProjectionIntent => 9,
            Self::Explanation => 10,
            Self::DeclarationDocumentation => 11,
            Self::DeclaredName => 12,
            Self::GeneratorVersion => 13,
            Self::DiagnosticRelation => 14,
            Self::CapturedHelper => 15,
        }
    }

    /// The seat rendered for a person. A projection: nothing reads it back.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::CapturedDeclaration => "the token material one expansion was handed",
            Self::Plan => "one projection plan",
            Self::OriginNode => "one node of the origin graph",
            Self::GeneratedUnit => "one generated unit a plan declares it will materialize",
            Self::RenderedUnit => "one rendered unit a renderer actually materialized",
            Self::OutputBytes => "the canonical bytes of one rendered unit",
            Self::Bundle => "one bundle materialized across a single publication boundary",
            Self::Closure => "one proved closure between a plan and its rendering",
            Self::ClosedExpansion => "one closed expansion",
            Self::ProjectionIntent => "one projection intent, ahead of anything decided about it",
            Self::Explanation => "one explanation answered over a plan and its closure",
            Self::DeclarationDocumentation => {
                "the documentation rows one captured declaration carries"
            }
            Self::DeclaredName => "one stable name this compiler wrote down",
            Self::GeneratorVersion => "the generator's declared name and its shape position",
            Self::DiagnosticRelation => {
                "one refusal body or one established issue a diagnostic points at"
            }
            Self::CapturedHelper => "one helper attribute's material, read beside a declaration",
        }
    }

    /// The preimage grammar a transcript at this role stands in.
    ///
    /// Total, and the ONE road from a mint site to a version ladder: a call site names the seat it is deriving for, and the grammar follows rather than being passed alongside it.
    /// A road that took both would admit a rendered unit derived under the plan grammar's position, which is a disagreement no reader of the resulting bytes could see.
    ///
    /// # Bounds
    ///
    /// Total and never injective.
    /// Two seats standing over ONE grammar answer with one profile and are separated inside it by the role, which is a member of the transcript and a segment of the context: [`Role::RenderedUnit`] names a rendered unit and [`Role::OutputBytes`] names the digest of exactly that unit's bytes, so both read here to [`RENDERED_UNIT_PROFILE`].
    ///
    /// Nothing reads back: a grammar names no role, because a grammar is a preimage several seats may stand over and a role is one seat inside it.
    #[must_use]
    pub const fn profile(self) -> Profile {
        match self {
            Self::CapturedDeclaration => CAPTURED_DECLARATION_PROFILE,
            Self::Plan => PLAN_PROFILE,
            Self::OriginNode => ORIGIN_NODE_PROFILE,
            Self::GeneratedUnit => GENERATED_UNIT_PROFILE,
            Self::RenderedUnit | Self::OutputBytes => RENDERED_UNIT_PROFILE,
            Self::Bundle => BUNDLE_PROFILE,
            Self::Closure => CLOSURE_PROFILE,
            Self::ClosedExpansion => CLOSED_EXPANSION_PROFILE,
            Self::ProjectionIntent => PROJECTION_INTENT_PROFILE,
            Self::Explanation => EXPLANATION_PROFILE,
            Self::DeclarationDocumentation => DECLARATION_DOCUMENTATION_PROFILE,
            Self::DeclaredName => DECLARED_NAME_PROFILE,
            Self::GeneratorVersion => GENERATOR_VERSION_PROFILE,
            Self::DiagnosticRelation => DIAGNOSTIC_RELATION_PROFILE,
            Self::CapturedHelper => CAPTURED_HELPER_PROFILE,
        }
    }
}

const _: () = assert!(
    slots_are_ordered(Role::ALL, 0),
    "a role whose published slot disagrees with its position in the roster",
);

/// Whether every row's published slot is its own position in the roster.
///
/// The slot is what a transcript carries, and the roster order is what a reader walks; two rows at one slot would derive one identity for two seats.
const fn slots_are_ordered(roles: &[Role], at: u8) -> bool {
    match roles.split_first() {
        None => true,
        Some((first, rest)) => first.slot() == at && slots_are_ordered(rest, at.saturating_add(1)),
    }
}

impl Anchoring {
    /// The discriminant byte member seven of every transcript carries for this posture.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Rooted => 0,
            Self::UnderOwner(_) => 1,
            Self::UnderProjection(_) => 2,
        }
    }

    /// The anchor commitment at full width, where there is one.
    #[must_use]
    pub const fn commitment(&self) -> Option<&[u8; 32]> {
        match self {
            Self::Rooted => None,
            Self::UnderOwner(anchor) | Self::UnderProjection(anchor) => Some(anchor),
        }
    }
}

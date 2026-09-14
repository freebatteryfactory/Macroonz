//! The mutation home's declarations: the kind, its one seat, the policy a surface is lowered under, and the site whose alternatives an evaluation selects between.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.
//!
//! # Every fact here is the declaration's
//!
//! An owner fact is a name the consumer declares, an operator family is a slug the consumer declares, and an alternative is DATA: its semantic operation bytes and the value it means, both stated rather than derived.
//! These types carry alternatives rather than computing them.

use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{HelperRefusal, ModuleName, Name, SupportName, TypeName};
use crate::token::GeneratedToken;

#[path = "type_guard.rs"]
mod guard;

/// Owner facts one policy may map to claims.
pub const MAPPING_LIMIT: usize = 16;

/// Claims one policy may state a permission for.
pub const PERMISSION_LIMIT: usize = 16;

/// Operator families one permission may name.
pub const OPERATOR_FAMILY_LIMIT: usize = 16;

/// Alternatives one site may declare.
///
/// Each alternative is one rendered constant and one arm of the evaluation's dispatch, so the count is what a consumer's test binary pays for.
pub const ALTERNATIVE_LIMIT: usize = 64;

/// The transcript position a captured reading of this grammar is separated by.
///
/// The three attribute-helper readings of one declaration share the captured-helper role and are told apart by position alone: this one is the second.
pub const MUTATION_HELPER_POSITION: u32 = 1;

/// The harness bank's operator slug for transposing one adjacent pair in a declared order.
pub const DECLARED_ORDER_FAMILY: &str = "declared-order-permutation";

/// The kind one mutation declaration produces: the module a mutation harness lowers, delivered to the consumer's test target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationSurface;

/// The one seat a mutation rendering fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceRole {
    /// The module carrying the discovery, the policy, and the evaluation.
    Module,
}

/// One operator family, by the slug the address resolves it under.
///
/// A slug rather than a namespaced name, because that is the shape the address's own road takes; what the slug MEANS is the consumer's declaration and nothing here reads it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FamilySlug(String);

/// One owner fact mapped to the claim that permits pressure on it.
///
/// Both are names the consumer declares. This home resolves neither and admits any pair, because which facts exist and which claims cover them is the declaration's whole subject.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FactMapping {
    /// The owner fact pressure is applied to.
    pub fact: Name,
    /// The claim that permits it.
    pub claim: Name,
}

/// One claim's permission over a roster of operator families.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission {
    claim: Name,
    families: NonEmpty<FamilySlug, OPERATOR_FAMILY_LIMIT>,
}

/// The complete policy one surface is lowered under: the evaluation family it belongs to, which owner facts map to which claims, and what each claim permits.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Policy {
    family: Name,
    mappings: Bounded<FactMapping, MAPPING_LIMIT>,
    permissions: Bounded<Permission, PERMISSION_LIMIT>,
}

/// One declared alternative at a site: which operator family produced it, the semantic bytes that identify the operation, and the value it means.
///
/// The bytes and the meaning travel together because they are one statement.
/// A rendering that carried the meaning without the bytes would emit a value nothing could select, and one that carried the bytes without the meaning would select a value nothing renders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alternative {
    family: FamilySlug,
    operation: Vec<u8>,
    meaning: Vec<GeneratedToken>,
}

/// One mutation site: the point, the owner fact it stands on, the production the unchanged declaration answers with, and the alternatives an active selection chooses between.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Site {
    point: Name,
    fact: Name,
    order: Vec<GeneratedToken>,
    production: Vec<GeneratedToken>,
    unchanged: Vec<u8>,
    alternatives: Bounded<Alternative, ALTERNATIVE_LIMIT>,
}

/// Where a rendered mutation surface lands, what a person invokes it by, and what its own refusal is called.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address {
    /// The module the surface is written as.
    pub module: ModuleName,
    /// The exported name a consumption target invokes the carrier by, where this declaration owns that address.
    pub support: Option<SupportName>,
    /// The refusal type the rendered module declares and every lowering road answers in.
    pub refusal: TypeName,
}

/// What one mutation helper body states on its own: where the surface lands, the policy it is lowered under, and which point and owner fact its site stands at.
///
/// Everything else a site carries is token material and semantic bytes a door computes from the declaration it captured, so the helper states what an author can state and the door completes it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declaration {
    address: Address,
    policy: Policy,
    point: Name,
    fact: Name,
}

/// The complete payload one mutation surface is declared from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Surface {
    address: Address,
    policy: Policy,
    site: Site,
}

/// How one mutation helper body was not read.
///
/// Its own type, because a diagnostic's family tag is a fact about the type: this grammar is a declaration's SECOND helper reading, and the trial grammar is its first.
#[must_use = "a mutation capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationCaptureError(HelperRefusal);

/// Why a codec declaration could not supply structural mutation material.
#[must_use = "a refusal identifies the declaration or rendering that withheld mutation material"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodecMutationError {
    /// The declared member roster contains no adjacent pair.
    NoAdjacentMembers,
    /// The codec owner refused a changed declaration.
    Codec(crate::codec::CodecError),
    /// The mutation owner refused the completed site.
    Declaration(crate::descriptor::DeclarationError),
    /// The codec renderer could not supply informed Rust tokens.
    Tokens(crate::token::GeneratedTreeRefusal),
}

/// The harness bank's operator slug for replacing a declared transition destination.
pub const TRANSITION_TARGET_FAMILY: &str = "transition-target-substitution";

/// The harness bank's operator slug for replacing a declared transition effect binding.
pub const EFFECT_BINDING_FAMILY: &str = "effect-binding-substitution";

/// Why one recipe row could not supply complete structural mutation material.
#[must_use = "a refusal withholds the complete site rather than silently reducing its alternatives"]
#[derive(Debug)]
pub enum RecipeMutationError {
    /// The selected row has no structurally distinct alternative in the declared roster.
    NoAlternative,
    /// The recipe compiler refused unchanged or selected source.
    Compiler(crate::diagnostic::Diagnostic),
    /// The recipe owner refused the selected re-declaration.
    Edit(crate::recipe::RecipeEditError),
    /// The mutation owner refused the site or its alternative magnitude.
    Declaration(crate::descriptor::DeclarationError),
    /// A completed recipe provided no source material.
    SourceAbsent,
}

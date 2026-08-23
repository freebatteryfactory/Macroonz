//! The mutation-declaration shapes the helper grammar reads and the renderer consumes.

use crate::test_descriptor::{ShellDeclarationRefusal, SupportMacroName, WallName};
use crate::token::SpanHandle;

#[path = "type_guard.rs"]
mod guard;

/// The Rust module one generated mutation declaration writes into the consumption target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationModuleName(String);

/// The sealed generated fact an owner may map to one claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutationOwnerFact {
    /// The refusal family's owner-declared canonical cause order.
    DeclaredOrder,
}

/// The generated mutation family this producer can materialize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeneratedMutationFamily {
    /// Exchange one adjacent pair in an owner-declared semantic order.
    DeclaredOrderPermutation,
}

/// One sealed generated fact mapped to the owner claim that permits pressure on it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnerClaimDeclaration {
    fact: MutationOwnerFact,
    claim: WallName,
}

/// One owner claim's permission to use a nonempty roster of generated operator families.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperatorPermissionDeclaration {
    claim: WallName,
    families: Vec<GeneratedMutationFamily>,
}

/// One closed mutation-policy declaration captured from a helper attribute.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationDeclaration {
    at: SpanHandle,
    module: MutationModuleName,
    support: Option<(SupportMacroName, SpanHandle)>,
    family: WallName,
    mappings: Vec<OwnerClaimDeclaration>,
    permissions: Vec<OperatorPermissionDeclaration>,
}

threadpak::closed_register! {
    /// Why one mutation helper body was not read.
    #[must_use = "a refusal is the reason one mutation declaration was not captured"]
    pub enum MutationDeclarationCause {
        /// The declaration carries the helper more than once.
        NotDeclaredOnce = "not-declared-once",
            "the declaration carries the mutation helper more than once";
        /// The helper carries no parenthesized body.
        NotBodied = "not-bodied", "the mutation helper states no body";
        /// One comma-delimited group is not a declared clause.
        NotAClause = "not-a-clause", "one mutation clause has no declared shape";
        /// One clause key or opening word is outside this helper's grammar.
        NotADeclarableClause = "not-a-declarable-clause",
            "one clause is outside the mutation helper grammar";
        /// One singleton clause is stated twice.
        NotDistinct = "not-distinct", "one singleton mutation clause is stated twice";
        /// One required singleton clause is absent.
        NotCovered = "not-covered", "one required mutation clause is absent";
        /// A mutation-only declaration omitted the support address its consumer invokes.
        SupportNotDeclared = "support-not-declared",
            "a mutation-only declaration states no support address";
        /// A mutation helper tried to re-author the support address the trial helper owns.
        SupportAlreadyDeclared = "support-already-declared",
            "the mutation helper re-declares the trial helper's support address";
        /// The mutation helper chose the module spelling the trial helper already owns.
        ModuleAlreadyDeclared = "module-already-declared",
            "the mutation helper re-declares the trial helper's module address";
        /// One namespaced reference is not `named("<namespace>", "<stem>")`.
        NotANamedReference = "not-a-named-reference",
            "one mutation reference is not a namespaced name";
        /// One owner-fact mapping is malformed.
        NotAMapping = "not-a-mapping", "one owner-fact mapping is malformed";
        /// One mapping names a fact this producer does not discover.
        UnknownOwnerFact = "unknown-owner-fact",
            "one mapping names no sealed generated fact";
        /// The declaration shape carries no instance of the sealed fact a mapping names.
        OwnerFactNotAvailable = "owner-fact-not-available",
            "the declaration shape carries no instance of the mapped fact";
        /// One sealed owner fact is mapped twice.
        DuplicateOwnerFact = "duplicate-owner-fact",
            "one sealed generated fact is mapped twice";
        /// One operator-family permission is malformed.
        NotAPermission = "not-a-permission", "one operator permission is malformed";
        /// Two permission rows name one owner claim.
        DuplicatePermissionClaim = "duplicate-permission-claim",
            "two permission rows name one owner claim";
        /// One permission row names no operator family.
        EmptyOperatorFamilies = "empty-operator-families",
            "one permission names no operator family";
        /// One permission row repeats an operator family.
        DuplicateOperatorFamily = "duplicate-operator-family",
            "one permission repeats an operator family";
        /// One permission names an operator family this producer cannot materialize.
        UnknownOperatorFamily = "unknown-operator-family",
            "one permission names no generated operator family";
    }
}

/// Why one mutation helper was not captured into its typed declaration.
#[must_use = "a refusal is the reason one mutation helper was not captured"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationDeclarationRefusal {
    /// The helper's token grammar refused at this token.
    Grammar {
        /// The exact grammar cause.
        cause: MutationDeclarationCause,
        /// The producer-issued token handle where it was established.
        at: SpanHandle,
    },
    /// One parsed spelling is not a lawful generated-support value.
    Carrier {
        /// The carrier-vocabulary refusal.
        refusal: ShellDeclarationRefusal,
        /// The producer-issued token handle where it was established.
        at: SpanHandle,
    },
}

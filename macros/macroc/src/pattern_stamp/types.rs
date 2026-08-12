//! The pattern-stamp home's declarations: the owner facts a scope-guard stamp
//! cites, and the exact identities one stamp is planned against.
//!
//! Declarations only. Every seat is public and required, so nothing here is
//! private and this home has no invariant nucleus beside this file.

use crate::plane::{
    GeneratedUnitSubject, OriginNodeSubject, OwnerFactRef, OwnerIdentityRef,
    PatternArgumentSubject, PatternInstanceSubject, PatternSubject, ProjectionIdentity,
    TracedSubject,
};
use crate::planning::ProjectionContext;

/// The owner facts one scope-guard stamp cites.
///
/// Both belong to the machine's identity home. The stamp writes nothing they do
/// not already declare, and the plan's trace says so by naming them rather than
/// by asserting that a rule was followed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeGuardOwnerFacts {
    /// The identity home's fact that a Class-C position carries no ordering
    /// operator of its own.
    pub class_c_carries_no_ordering: OwnerFactRef,
    /// The identity home's fact that comparison is total within one scope and
    /// refuses across scopes.
    pub comparison_is_scope_guarded: OwnerFactRef,
}

/// The exact identities one scope-guard stamp is planned against.
///
/// There is no constructor and no default: every seat is required, because a
/// stamp plan that could omit its pattern, its instantiation, or its arguments
/// would be an account that sometimes says less than it knows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeGuardStampAnchors {
    /// The shared plan context: closed graph, profile and version, cause set,
    /// generator version, and target binding.
    pub context: ProjectionContext,
    /// The authored pattern — the machine's scope-guard version pattern.
    pub pattern: OwnerIdentityRef<PatternSubject>,
    /// This instantiation of it.
    pub instance: OwnerIdentityRef<PatternInstanceSubject>,
    /// The first typed argument: the guard type the caller named.
    pub guard_name: OwnerIdentityRef<PatternArgumentSubject>,
    /// The second typed argument: the scope type the caller named. A string
    /// never becomes an argument here — the caller states a type.
    pub scope_type: OwnerIdentityRef<PatternArgumentSubject>,
    /// The authored declaration the invocation sits in.
    pub authored_node: ProjectionIdentity<OriginNodeSubject>,
    /// The instantiated pattern as an origin node.
    pub instantiated_node: ProjectionIdentity<OriginNodeSubject>,
    /// The rendered guard as an origin node.
    pub rendered_node: ProjectionIdentity<OriginNodeSubject>,
    /// The generated unit the stamp materializes.
    pub stamped_unit: ProjectionIdentity<GeneratedUnitSubject>,
    /// The subject the plan's decisions are recorded about.
    pub traced: ProjectionIdentity<TracedSubject>,
    /// The owner facts the stamp rests on.
    pub owner_facts: ScopeGuardOwnerFacts,
}

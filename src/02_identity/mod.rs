//! Band 02 — identity: the six-class calculus, the two-column law, the
//! derived-seat law, and the scope guards. Concrete identities live with their
//! owner homes and instantiate these shapes.

pub mod types;

pub use types::{
    ApplicationScope, AuthorityPosition, ByteIdentity, Commitment, CreationLaw, IdentityClass,
    IdentityRole, Occurrence, OccurrenceForm, OrderComparison, TypedRef,
};

/// Stamps one Class-C scope-guard version newtype over
/// [`AuthorityPosition`](types::AuthorityPosition).
///
/// # What it stamps, and what it refuses to guess
///
/// The caller states everything: the documentation, the visibility, the type
/// name, and the scope type. Nothing is inferred — no name is built from
/// another name, no scope is derived from a spelling, and no default is
/// supplied for anything. A stamp that guessed would be legislating; this one
/// only writes down what the caller already declared.
///
/// The stamped pattern is exactly the pattern this home rules for Class C, and
/// the walls come with it: the newtype derives `Debug`, `Clone`, `PartialEq`,
/// `Eq`, and `Hash` and **never** `Ord` or `PartialOrd`, so `a < b` on a stamped
/// guard does not typecheck; the one comparison is `try_cmp_same_scope`, which
/// forwards to the machinery this home already owns and refuses across scopes
/// with [`OrderComparison`](types::OrderComparison). Cross-scope order is a cut
/// vector, never integers.
///
/// # Where the stamp lives
///
/// This home owns the Class-C shape and its guard law, so this home stamps it.
/// Rust exports `macro_rules!` at the crate root; that placement is Rust's rule
/// about macro namespacing and is not a root admission of a semantic noun — the
/// stamp declares no type of its own and owns no meaning.
///
/// # The invocation
///
/// ```
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # pub struct DemoScopeId(u8);
/// threadpak::scope_guard_version! {
///     /// The demo family's version, positioned by its own authority.
///     pub struct DemoVersion over DemoScopeId;
/// }
/// ```
#[macro_export]
macro_rules! scope_guard_version {
    (
        $(#[$note:meta])*
        $vis:vis struct $name:ident over $scope:ty;
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $name($vis $crate::identity::AuthorityPosition<$scope>);

        impl $name {
            /// The one lawful comparison: total within one scope, refused
            /// across scopes. Forwards to the Class-C machinery the identity
            /// home owns; this stamp adds no comparison of its own.
            ///
            /// # Errors
            ///
            /// Returns the `OrderComparison` family body when the two positions
            /// do not share one scope.
            pub fn try_cmp_same_scope(
                &self,
                other: &Self,
            ) -> ::core::result::Result<
                ::core::cmp::Ordering,
                $crate::identity::OrderComparison,
            > {
                self.0.try_cmp_same_scope(&other.0)
            }
        }
    };
}

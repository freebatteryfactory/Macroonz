//! Band 02 — identity: the six-class calculus, the two-column law, the
//! derived-seat law, and the scope guards. Concrete identities live with their
//! owner homes and instantiate these shapes.

pub mod types;

pub use types::{
    AdmittedIdentityColumns, AdmittedIdentityRole, ApplicationScope, AuthorityPosition,
    ByteIdentity, Commitment, CreationLaw, IdentityClass, IdentityRole, IdentityRoleAdmission,
    Occurrence, OccurrenceForm, OrderComparison, TypedRef,
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
/// # The representation is not a road
///
/// The inner [`AuthorityPosition`](types::AuthorityPosition) is a PRIVATE field,
/// exactly as every hand-written guard in the machine already writes it. A
/// stamped guard is a role, and a role's whole content is that this position was
/// positioned under THIS role's authority — so a representation that could be
/// taken out of one role and put into another would make the role a label rather
/// than a wall. Outside the module the stamp expanded in, the tuple form
/// `Role(position)` is not a constructor and `value.0` is not a field: both
/// refuse, and `testpak/tests/compile-fail/a-stamped-representation-cannot-be-laundered.rs`
/// is the reversal that proves they do.
///
/// So the stamp emits ONE road in and NO road out. The road in is
/// `positioned`, which takes a position the caller already holds and says which
/// role it is being read under. There is deliberately no accessor: an accessor
/// handing back the inner position would re-open the laundering road the private
/// field closes, because `AuthorityPosition` is `Clone` and the returned value
/// would be re-wrappable under any other role. Nothing in the machine needs to
/// read the position back out — the one operation a Class-C guard supports is
/// the comparison below, and it reads the field from inside.
///
/// Both operations carry the caller's own `$vis`, so the road in and the
/// comparison are reachable exactly as far as the role they serve and never one
/// step further. A guard stamped privately gets private operations, which is
/// what the hand-written twin in `laws.rs` has always written; a guard stamped
/// `pub` exports both. The stamp does not decide a surface the caller did not
/// ask for.
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
        $vis struct $name($crate::identity::AuthorityPosition<$scope>);

        impl $name {
            /// The one road in: read one position under this role.
            ///
            /// The caller supplies a position it already holds; this states
            /// which role that position is being read under. There is no road
            /// out, and that asymmetry is the point — a representation that
            /// could leave this role could be re-entered under another one, and
            /// the role would have stopped being a wall.
            #[must_use]
            $vis fn positioned(
                position: $crate::identity::AuthorityPosition<$scope>,
            ) -> Self {
                Self(position)
            }

            /// The one lawful comparison: total within one scope, refused
            /// across scopes. Forwards to the Class-C machinery the identity
            /// home owns; this stamp adds no comparison of its own.
            ///
            /// # Errors
            ///
            /// Returns the `OrderComparison` family body when the two positions
            /// do not share one scope.
            $vis fn try_cmp_same_scope(
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

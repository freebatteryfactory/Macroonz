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
/// # The representation is not a road, and the compiler is what says so
///
/// The inner [`AuthorityPosition`](types::AuthorityPosition) is a PRIVATE field.
/// A stamped guard is a role, and a role's whole content is that this position
/// was positioned under THIS role's authority — so a representation that could
/// be taken out of one role and put into another would make the role a label
/// rather than a wall.
///
/// Rust's privacy is MODULE-scoped, so a private field is private to the module
/// the declaration landed in and to that module's descendants. A `macro_rules!`
/// expansion lands in the invoking module, so a stamp that wrote the newtype
/// straight into a home's `types.rs` put the seat within reach of every other
/// type, function and implementation in that file — dozens of them — and the
/// only remaining question was whether a person had written a road out anywhere
/// among them. That question is a whole-file audit, and it was asked twelve
/// times and answered wrong twelve times, in twelve different Rust shapes: a
/// receiver of another type, a wrapper, a collection, a tuple, an opaque
/// iterator, a type alias, a nested `Result`, a free function, a trait
/// implementation for a reference.
///
/// So the stamp does not write into the invoking module. It writes into a module
/// of its own — `seated in mod <name>` — and re-exports the type out of it:
///
/// ```text
/// mod frame_version {            // NOTHING is hand-written in here
///     pub struct FrameVersion(AuthorityPosition<ReferenceFrameId>);
///     impl FrameVersion { … }    // exactly the roads this stamp writes
/// }
/// pub use frame_version::FrameVersion;
/// ```
///
/// The module's ENTIRE content is this transcriber's output, because a
/// `macro_rules!` expansion is closed: no `#[path]`, no second `mod` block, and
/// no hand-written item can be added to a module that exists only inside an
/// expansion. The complete set of roads out of a stamped guard is therefore
/// exactly the set written below, and it is `rustc` rather than a reader that
/// establishes it: from the invoking module, `version.0` is `E0616` and
/// `FrameVersion(position)` is `E0423`, whatever anybody writes beside them.
///
/// The set written below is ONE road in and NO road out. The road in is
/// `positioned`, which takes a position the caller already holds and says which
/// role it is being read under. There is deliberately no accessor: an accessor
/// handing back the inner position would re-open the laundering road the private
/// field closes, because `AuthorityPosition` is `Clone` and the returned value
/// would be re-wrappable under any other role. Nothing in the machine needs to
/// read the position back out — the one operation a Class-C guard supports is
/// the comparison below, and it reads the field from inside.
///
/// # The module name is the caller's, and the caller's alone
///
/// `macro_rules!` cannot build an identifier out of another identifier on
/// stable, and this repository carries no dependency that can, so the module
/// name arrives as an argument. It is written in `snake_case` because a module
/// named after its type trips `non_snake_case`, which this workspace's lint wall
/// denies — the name says what the module is a home FOR, and the compiler holds
/// the spelling with no attribute suppressing anything.
///
/// Two stamps naming one module in one file collide as a duplicate definition,
/// so the names cannot silently merge two roles into one seat.
///
/// # Its stated ceiling
///
/// The emitted module opens with `use super::*;`, which is how the scope type
/// the caller wrote in ITS module is nameable inside the stamp's. So the scope
/// must be nameable from the invoking module — it always is, since the caller
/// wrote it there — and a scope spelled as a path that needs no import leaves
/// that glob unused, which is a denied warning at the call site rather than
/// anything silent.
///
/// The caller's visibility is emitted once on the canonical re-export, at the
/// coordinate where the caller wrote it. The front grammar separately moves
/// that reach one level into the generated child: private and `self` become
/// `super`, `super` gains one `super` segment, and absolute paths and `pub`
/// remain absolute. The type and both methods carry that transported reach.
/// Therefore a generated path can be named only from a scope already allowed
/// by the caller; a same-coordinate re-export, alias, or signature cannot
/// publish the guard farther. The child module itself stays private, so the
/// caller-coordinate re-export remains the canonical exported spelling.
///
/// Direct source tokens cover Rust's private, `pub`, shorthand, relative
/// `pub(in super::...)`, and absolute `pub(in ...)` families. An absolute path
/// produced with an outer macro's `$crate` is preserved unchanged. A whole
/// visibility first captured as `$vis:vis` by another macro is opaque to
/// `macro_rules!`, so this stamp refuses it instead of guessing a reach; the
/// wrapper must author the literal front syntax or explicitly own an internal
/// transcriber call.
///
/// # Where the stamp lives
///
/// This home owns the Class-C shape and its guard law, so this home stamps it.
/// Rust exports `macro_rules!` at the crate root; that placement is Rust's rule
/// about macro namespacing and is not a root admission of a semantic noun — the
/// stamp declares no type of its own and owns no meaning.
///
/// The `@transcribe` rule is the stamp's shared implementation arm, following
/// the root `closed_register!` precedent; it is not the public front grammar.
/// Because `macro_rules!` exports all arms together, Rust does not make that
/// spelling private. A direct caller is hand-authoring both visibilities and
/// therefore owns their relationship; the front grammar's exact-transport
/// guarantee does not apply. The transcriber still owns one body with a private
/// tuple field and exactly two methods, and a direct call cannot alter an
/// existing guard because its generated module or item name collides. Rust also
/// refuses a requested re-export that exceeds the hand-authored internal item.
///
/// A direct arm call that asks the re-export to exceed the internal reach is
/// refused by rustc:
///
/// ```compile_fail
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # pub struct DemoScopeId;
/// threadpak::scope_guard_version! {
///     @transcribe
///     [pub(crate)]
///     [pub]
///     /// No public re-export can widen this crate-confined item.
///     struct TooWideVersion over DemoScopeId, seated in mod too_wide_version;
/// }
/// ```
///
/// # The invocation
///
/// ```
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # pub struct DemoScopeId(u8);
/// threadpak::scope_guard_version! {
///     /// The demo family's version, positioned by its own authority.
///     pub struct DemoVersion over DemoScopeId, seated in mod demo_version;
/// }
/// # fn main() {}
/// ```
///
/// The `fn main` is written out because the stamp emits a MODULE, and a module
/// emitted inside the function body rustdoc would otherwise wrap this in has a
/// `super` that is the crate root rather than the caller — so the scope type
/// declared beside the invocation would not be nameable from inside the seat.
#[macro_export]
macro_rules! scope_guard_version {
    (
        $(#[$note:meta])*
        pub struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub]
            [pub]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(crate) struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(crate)]
            [pub(crate)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(self) struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(super)]
            [pub(self)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(super) struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(in super::super)]
            [pub(super)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(in self $(:: $relative:ident)*) struct $name:ident over $scope:ty,
            seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(in super $(:: $relative)*)]
            [pub(in self $(:: $relative)*)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(in super $(:: $relative:ident)*) struct $name:ident over $scope:ty,
            seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(in super::super $(:: $relative)*)]
            [pub(in super $(:: $relative)*)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(in crate $(:: $absolute:ident)*) struct $name:ident over $scope:ty,
            seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(in crate $(:: $absolute)*)]
            [pub(in crate $(:: $absolute)*)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        pub(in $absolute:path) struct $name:ident over $scope:ty,
            seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(in $absolute)]
            [pub(in $absolute)]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        $(#[$note:meta])*
        struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @capture_private
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        @capture_private
        $(#[$note:meta])*
        $caller_vis:vis struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        $crate::scope_guard_version! {
            @transcribe
            [pub(super)]
            [$caller_vis]
            $(#[$note])*
            struct $name over $scope, seated in mod $home;
        }
    };
    (
        @transcribe
        [$internal_vis:vis]
        [$caller_vis:vis]
        $(#[$note:meta])*
        struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        mod $home {
            use super::*;

            $(#[$note])*
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            $internal_vis struct $name($crate::identity::AuthorityPosition<$scope>);

            impl $name {
                /// The one road in: read one position under this role.
                ///
                /// The caller supplies a position it already holds; this states
                /// which role that position is being read under. There is no
                /// road out, and that asymmetry is the point — a representation
                /// that could leave this role could be re-entered under another
                /// one, and the role would have stopped being a wall.
                #[must_use]
                $internal_vis fn positioned(
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
                /// Returns the `OrderComparison` family body when the two
                /// positions do not share one scope.
                $internal_vis fn try_cmp_same_scope(
                    &self,
                    other: &Self,
                ) -> ::core::result::Result<
                    ::core::cmp::Ordering,
                    $crate::identity::OrderComparison,
                > {
                    self.0.try_cmp_same_scope(&other.0)
                }
            }
        }

        $caller_vis use $home::$name;
    };
    (
        $(#[$note:meta])*
        $opaque_visibility:vis struct $name:ident over $scope:ty, seated in mod $home:ident;
    ) => {
        compile_error!(
            "scope_guard_version! requires visibility tokens at its public front door; an opaque forwarded `vis` fragment cannot be transported one module deeper"
        );
    };
}

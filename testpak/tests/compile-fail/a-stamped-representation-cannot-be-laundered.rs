//! The stamped guard's second owed red twin, discharged: a role is not a label
//! over a shared representation.
//!
//! The cross-scope fixture proves two guards over DIFFERENT scopes are different
//! types. This one removes that help entirely: both roles below are stamped over
//! ONE scope type, so the inner `AuthorityPosition<OneScopeId>` is literally the
//! same type in both. Nothing about the scope refuses here — the only thing
//! standing between role A's position and role B is that the position is not
//! reachable, in either direction, from outside the module the stamp expanded
//! in.
//!
//! Both halves of the laundering are attempted in one expression: taking the
//! position OUT of role A (`role_a.0`) and putting it back IN under role B
//! (`roles::RoleBVersion( … )`). Each refuses on its own. No value is
//! constructed; the signature and the field access alone are the proof.

mod roles {
    /// The one scope both roles are positioned in.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct OneScopeId;

    threadpak::scope_guard_version! {
        /// Role A's version, positioned under role A's authority.
        pub struct RoleAVersion over OneScopeId;
    }

    threadpak::scope_guard_version! {
        /// Role B's version, positioned under role B's authority — over the very
        /// same scope.
        pub struct RoleBVersion over OneScopeId;
    }
}

fn main() {
    let launder: fn(roles::RoleAVersion) -> roles::RoleBVersion =
        |role_a| roles::RoleBVersion(role_a.0);
    let _ = launder;
}

//! Every narrow visibility arm refuses a wider same-coordinate re-export.
//!
//! The same fixture also crosses each local and ancestor boundary, rejects the
//! private generated-module path for a public guard, and proves that an opaque
//! forwarded `vis` fragment fails closed instead of selecting a wrong arm.

macro_rules! shallow_dollar_guard {
    () => {
        threadpak::scope_guard_version! {
            pub(in $crate::shallow_ancestor) struct DollarGuard over Scope,
                seated in mod dollar_guard;
        }
    };
}

macro_rules! deep_dollar_guard {
    () => {
        threadpak::scope_guard_version! {
            pub(in $crate::deep::ancestor) struct DollarGuard over Scope,
                seated in mod dollar_guard;
        }
    };
}

pub mod shallow_ancestor {
    pub mod owner {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Scope;

        threadpak::scope_guard_version! { struct PrivateGuard over Scope, seated in mod private_guard; }
        threadpak::scope_guard_version! { pub(self) struct SelfGuard over Scope, seated in mod self_guard; }
        threadpak::scope_guard_version! { pub(in self) struct InSelfGuard over Scope, seated in mod in_self_guard; }
        threadpak::scope_guard_version! { pub(super) struct SuperGuard over Scope, seated in mod super_guard; }
        threadpak::scope_guard_version! { pub(in super) struct InSuperGuard over Scope, seated in mod in_super_guard; }
        threadpak::scope_guard_version! { pub(in super::super) struct RelativeGuard over Scope, seated in mod relative_guard; }
        threadpak::scope_guard_version! { pub(crate) struct CrateGuard over Scope, seated in mod crate_guard; }
        threadpak::scope_guard_version! { pub(in crate) struct InCrateGuard over Scope, seated in mod in_crate_guard; }
        threadpak::scope_guard_version! {
            pub(in crate::shallow_ancestor) struct AbsoluteGuard over Scope,
                seated in mod absolute_guard;
        }
        shallow_dollar_guard!();
        threadpak::scope_guard_version! { pub struct PublicGuard over Scope, seated in mod public_guard; }

        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use private_guard::PrivateGuard as WiderPrivate;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use self_guard::SelfGuard as WiderSelf;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use in_self_guard::InSelfGuard as WiderInSelf;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use super_guard::SuperGuard as WiderSuper;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use in_super_guard::InSuperGuard as WiderInSuper;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub use relative_guard::RelativeGuard as WiderRelative;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub use crate_guard::CrateGuard as WiderCrate;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub use in_crate_guard::InCrateGuard as WiderInCrate;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use absolute_guard::AbsoluteGuard as WiderAbsolute;
        #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
        pub(crate) use dollar_guard::DollarGuard as WiderDollar;
    }

    fn outside_invocation_coordinate() {
        let _ = owner::PrivateGuard::positioned;
        let _ = owner::SelfGuard::try_cmp_same_scope;
        let _ = owner::InSelfGuard::positioned;
    }
}

pub mod deep {
    pub mod ancestor {
        pub mod owner {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct Scope;

            threadpak::scope_guard_version! { struct PrivateGuard over Scope, seated in mod private_guard; }
            threadpak::scope_guard_version! { pub(self) struct SelfGuard over Scope, seated in mod self_guard; }
            threadpak::scope_guard_version! { pub(in self) struct InSelfGuard over Scope, seated in mod in_self_guard; }
            threadpak::scope_guard_version! { pub(super) struct SuperGuard over Scope, seated in mod super_guard; }
            threadpak::scope_guard_version! { pub(in super) struct InSuperGuard over Scope, seated in mod in_super_guard; }
            threadpak::scope_guard_version! { pub(in super::super) struct RelativeGuard over Scope, seated in mod relative_guard; }
            threadpak::scope_guard_version! { pub(crate) struct CrateGuard over Scope, seated in mod crate_guard; }
            threadpak::scope_guard_version! { pub(in crate) struct InCrateGuard over Scope, seated in mod in_crate_guard; }
            threadpak::scope_guard_version! {
                pub(in crate::deep::ancestor) struct AbsoluteGuard over Scope,
                    seated in mod absolute_guard;
            }
            deep_dollar_guard!();
            threadpak::scope_guard_version! { pub struct PublicGuard over Scope, seated in mod public_guard; }

            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use private_guard::PrivateGuard as WiderPrivate;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use self_guard::SelfGuard as WiderSelf;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use in_self_guard::InSelfGuard as WiderInSelf;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use super_guard::SuperGuard as WiderSuper;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use in_super_guard::InSuperGuard as WiderInSuper;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use relative_guard::RelativeGuard as WiderRelative;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub use crate_guard::CrateGuard as WiderCrate;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub use in_crate_guard::InCrateGuard as WiderInCrate;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use absolute_guard::AbsoluteGuard as WiderAbsolute;
            #[expect(unused_imports, reason = "the deliberately illegal re-export is the reversal subject")]
            pub(crate) use dollar_guard::DollarGuard as WiderDollar;
        }

        fn outside_invocation_coordinate() {
            let _ = owner::PrivateGuard::positioned;
            let _ = owner::SelfGuard::try_cmp_same_scope;
            let _ = owner::InSelfGuard::positioned;
        }
    }

    fn outside_parent_and_named_ancestor_reach() {
        let _ = ancestor::owner::SuperGuard::positioned;
        let _ = ancestor::owner::InSuperGuard::try_cmp_same_scope;
        let _ = ancestor::owner::AbsoluteGuard::positioned;
        let _ = ancestor::owner::DollarGuard::try_cmp_same_scope;
    }
}

macro_rules! forward_opaque_visibility {
    ($visibility:vis) => {
        mod forwarded {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct Scope;

            threadpak::scope_guard_version! {
                $visibility struct Guard over Scope, seated in mod guard;
            }
        }
    };
}

forward_opaque_visibility!(pub(super));

fn main() {
    let _ = shallow_ancestor::owner::SuperGuard::positioned;
    let _ = shallow_ancestor::owner::InSuperGuard::try_cmp_same_scope;
    let _ = shallow_ancestor::owner::AbsoluteGuard::positioned;
    let _ = shallow_ancestor::owner::DollarGuard::try_cmp_same_scope;
    let _ = shallow_ancestor::owner::public_guard::PublicGuard::positioned;
    let _ = deep::ancestor::owner::RelativeGuard::try_cmp_same_scope;
    let _ = deep::ancestor::owner::public_guard::PublicGuard::try_cmp_same_scope;
}

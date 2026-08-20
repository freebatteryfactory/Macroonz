//! Every narrow visibility arm refuses a wider same-coordinate type alias.

#![deny(private_interfaces)]

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

        pub(crate) type WiderPrivate = private_guard::PrivateGuard;
        pub(crate) type WiderSelf = self_guard::SelfGuard;
        pub(crate) type WiderInSelf = in_self_guard::InSelfGuard;
        pub(crate) type WiderSuper = super_guard::SuperGuard;
        pub(crate) type WiderInSuper = in_super_guard::InSuperGuard;
        pub type WiderRelative = relative_guard::RelativeGuard;
        pub type WiderCrate = crate_guard::CrateGuard;
        pub type WiderInCrate = in_crate_guard::InCrateGuard;
        pub(crate) type WiderAbsolute = absolute_guard::AbsoluteGuard;
        pub(crate) type WiderDollar = dollar_guard::DollarGuard;
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

            pub(crate) type WiderPrivate = private_guard::PrivateGuard;
            pub(crate) type WiderSelf = self_guard::SelfGuard;
            pub(crate) type WiderInSelf = in_self_guard::InSelfGuard;
            pub(crate) type WiderSuper = super_guard::SuperGuard;
            pub(crate) type WiderInSuper = in_super_guard::InSuperGuard;
            pub(crate) type WiderRelative = relative_guard::RelativeGuard;
            pub type WiderCrate = crate_guard::CrateGuard;
            pub type WiderInCrate = in_crate_guard::InCrateGuard;
            pub(crate) type WiderAbsolute = absolute_guard::AbsoluteGuard;
            pub(crate) type WiderDollar = dollar_guard::DollarGuard;
        }
    }
}

fn main() {}

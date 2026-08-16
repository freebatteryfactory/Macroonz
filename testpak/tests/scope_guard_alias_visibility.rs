//! Positive controls for same-reach scope-guard type aliases.
//!
//! Each declaration below aliases a guard at exactly the reach its front
//! spelling grants; the compile-refusal twin
//! `tests/compile-fail/a-scope-guard-alias-cannot-widen-reach.rs` crosses that
//! boundary.

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

mod shallow_ancestor {
    pub(crate) mod owner {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub(crate) struct Scope;

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

        type PrivateAlias = private_guard::PrivateGuard;
        type SelfAlias = self_guard::SelfGuard;
        type InSelfAlias = in_self_guard::InSelfGuard;
        pub(super) type SuperAlias = super_guard::SuperGuard;
        pub(super) type InSuperAlias = in_super_guard::InSuperGuard;
        pub(in super::super) type RelativeAlias = relative_guard::RelativeGuard;
        pub(crate) type CrateAlias = crate_guard::CrateGuard;
        pub(crate) type InCrateAlias = in_crate_guard::InCrateGuard;
        pub(in crate::shallow_ancestor) type AbsoluteAlias = absolute_guard::AbsoluteGuard;
        pub(in crate::shallow_ancestor) type DollarAlias = dollar_guard::DollarGuard;

        pub(super) fn local_aliases() {
            let _ = PrivateAlias::positioned;
            let _ = SelfAlias::positioned;
            let _ = InSelfAlias::positioned;
            let _ = SuperAlias::positioned;
            let _ = InSuperAlias::positioned;
            let _ = RelativeAlias::positioned;
            let _ = CrateAlias::positioned;
            let _ = InCrateAlias::positioned;
            let _ = AbsoluteAlias::positioned;
            let _ = DollarAlias::positioned;
        }
    }

    pub(super) fn ancestor_aliases() {
        owner::local_aliases();
        let _ = owner::SuperAlias::try_cmp_same_scope;
        let _ = owner::InSuperAlias::try_cmp_same_scope;
        let _ = owner::RelativeAlias::try_cmp_same_scope;
        let _ = owner::CrateAlias::try_cmp_same_scope;
        let _ = owner::InCrateAlias::try_cmp_same_scope;
        let _ = owner::AbsoluteAlias::try_cmp_same_scope;
        let _ = owner::DollarAlias::try_cmp_same_scope;
    }
}

mod deep {
    pub(crate) mod ancestor {
        pub(crate) mod owner {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub(crate) struct Scope;

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

            type PrivateAlias = private_guard::PrivateGuard;
            type SelfAlias = self_guard::SelfGuard;
            type InSelfAlias = in_self_guard::InSelfGuard;
            pub(super) type SuperAlias = super_guard::SuperGuard;
            pub(super) type InSuperAlias = in_super_guard::InSuperGuard;
            pub(in super::super) type RelativeAlias = relative_guard::RelativeGuard;
            pub(crate) type CrateAlias = crate_guard::CrateGuard;
            pub(crate) type InCrateAlias = in_crate_guard::InCrateGuard;
            pub(in crate::deep::ancestor) type AbsoluteAlias = absolute_guard::AbsoluteGuard;
            pub(in crate::deep::ancestor) type DollarAlias = dollar_guard::DollarGuard;

            pub(super) fn local_aliases() {
                let _ = PrivateAlias::positioned;
                let _ = SelfAlias::positioned;
                let _ = InSelfAlias::positioned;
                let _ = SuperAlias::positioned;
                let _ = InSuperAlias::positioned;
                let _ = RelativeAlias::positioned;
                let _ = CrateAlias::positioned;
                let _ = InCrateAlias::positioned;
                let _ = AbsoluteAlias::positioned;
                let _ = DollarAlias::positioned;
            }
        }

        pub(crate) fn ancestor_aliases() {
            owner::local_aliases();
            let _ = owner::SuperAlias::try_cmp_same_scope;
            let _ = owner::InSuperAlias::try_cmp_same_scope;
            let _ = owner::RelativeAlias::try_cmp_same_scope;
            let _ = owner::CrateAlias::try_cmp_same_scope;
            let _ = owner::InCrateAlias::try_cmp_same_scope;
            let _ = owner::AbsoluteAlias::try_cmp_same_scope;
            let _ = owner::DollarAlias::try_cmp_same_scope;
        }
    }
}

/// Every narrow front spelling permits a type alias at exactly its own reach,
/// at both invocation depths.
#[test]
fn same_reach_scope_guard_aliases_are_lawful() {
    shallow_ancestor::ancestor_aliases();
    deep::ancestor::ancestor_aliases();
    let _ = shallow_ancestor::owner::RelativeAlias::try_cmp_same_scope;
    let _ = shallow_ancestor::owner::CrateAlias::try_cmp_same_scope;
    let _ = shallow_ancestor::owner::InCrateAlias::try_cmp_same_scope;
    let _ = deep::ancestor::owner::CrateAlias::try_cmp_same_scope;
    let _ = deep::ancestor::owner::InCrateAlias::try_cmp_same_scope;
}

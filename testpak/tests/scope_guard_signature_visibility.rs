//! Positive controls for same-reach scope guards in function signatures.

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

        fn private_signature(_: private_guard::PrivateGuard) {}
        fn self_signature(_: self_guard::SelfGuard) {}
        fn in_self_signature(_: in_self_guard::InSelfGuard) {}
        pub(super) fn super_signature(_: super_guard::SuperGuard) {}
        pub(super) fn in_super_signature(_: in_super_guard::InSuperGuard) {}
        pub(in super::super) fn relative_signature(_: relative_guard::RelativeGuard) {}
        pub(crate) fn crate_signature(_: crate_guard::CrateGuard) {}
        pub(crate) fn in_crate_signature(_: in_crate_guard::InCrateGuard) {}
        pub(in crate::shallow_ancestor) fn absolute_signature(_: absolute_guard::AbsoluteGuard) {}
        pub(in crate::shallow_ancestor) fn dollar_signature(_: dollar_guard::DollarGuard) {}

        pub(super) fn local_signatures() {
            let _ = private_signature;
            let _ = self_signature;
            let _ = in_self_signature;
            let _ = super_signature;
            let _ = in_super_signature;
            let _ = relative_signature;
            let _ = crate_signature;
            let _ = in_crate_signature;
            let _ = absolute_signature;
            let _ = dollar_signature;
        }
    }

    pub(super) fn ancestor_signatures() {
        owner::local_signatures();
        let _ = owner::super_signature;
        let _ = owner::in_super_signature;
        let _ = owner::relative_signature;
        let _ = owner::crate_signature;
        let _ = owner::in_crate_signature;
        let _ = owner::absolute_signature;
        let _ = owner::dollar_signature;
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

            fn private_signature(_: private_guard::PrivateGuard) {}
            fn self_signature(_: self_guard::SelfGuard) {}
            fn in_self_signature(_: in_self_guard::InSelfGuard) {}
            pub(super) fn super_signature(_: super_guard::SuperGuard) {}
            pub(super) fn in_super_signature(_: in_super_guard::InSuperGuard) {}
            pub(in super::super) fn relative_signature(_: relative_guard::RelativeGuard) {}
            pub(crate) fn crate_signature(_: crate_guard::CrateGuard) {}
            pub(crate) fn in_crate_signature(_: in_crate_guard::InCrateGuard) {}
            pub(in crate::deep::ancestor) fn absolute_signature(_: absolute_guard::AbsoluteGuard) {}
            pub(in crate::deep::ancestor) fn dollar_signature(_: dollar_guard::DollarGuard) {}

            pub(super) fn local_signatures() {
                let _ = private_signature;
                let _ = self_signature;
                let _ = in_self_signature;
                let _ = super_signature;
                let _ = in_super_signature;
                let _ = relative_signature;
                let _ = crate_signature;
                let _ = in_crate_signature;
                let _ = absolute_signature;
                let _ = dollar_signature;
            }
        }

        pub(crate) fn ancestor_signatures() {
            owner::local_signatures();
            let _ = owner::super_signature;
            let _ = owner::in_super_signature;
            let _ = owner::relative_signature;
            let _ = owner::crate_signature;
            let _ = owner::in_crate_signature;
            let _ = owner::absolute_signature;
            let _ = owner::dollar_signature;
        }
    }
}

/// Every narrow front spelling permits a public-signature road at exactly its
/// own reach, at both invocation depths.
///
/// green: identity.scope-guard-signature-cannot-widen
#[test]
fn same_reach_scope_guard_signatures_are_lawful() {
    shallow_ancestor::ancestor_signatures();
    deep::ancestor::ancestor_signatures();
    let _ = shallow_ancestor::owner::relative_signature;
    let _ = shallow_ancestor::owner::crate_signature;
    let _ = shallow_ancestor::owner::in_crate_signature;
    let _ = deep::ancestor::owner::crate_signature;
    let _ = deep::ancestor::owner::in_crate_signature;
}

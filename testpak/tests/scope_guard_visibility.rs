//! Positive compiler evidence for caller-relative scope-guard visibility.
//!
//! The population is the public macro's admitted front grammar, exercised at
//! two invocation depths. Each declaration uses both emitted operations inside
//! its lawful reach; the compile-refusal twin crosses each narrower boundary.

use threadpak::identity::{AuthorityPosition, OrderComparison};

fn assert_surface<Guard, Scope>(
    _: fn(AuthorityPosition<Scope>) -> Guard,
    _: fn(&Guard, &Guard) -> Result<core::cmp::Ordering, OrderComparison>,
) {
}

macro_rules! crate_relative_scope_guard {
    ($name:ident, $scope:ty, $home:ident) => {
        threadpak::scope_guard_version! {
            /// An absolute visibility rooted by the invoking macro's crate.
            pub(in $crate::scope_guard_visibility) struct $name over $scope,
                seated in mod $home;
        }
    };
}

mod scope_guard_visibility {
    use super::assert_surface;

    pub(crate) mod shallow {
        use super::assert_surface;

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub(crate) struct Scope;

        threadpak::scope_guard_version! {
            struct Private over Scope, seated in mod private;
        }
        threadpak::scope_guard_version! {
            pub(self) struct SelfShort over Scope, seated in mod self_short;
        }
        threadpak::scope_guard_version! {
            pub(in self) struct SelfLong over Scope, seated in mod self_long;
        }
        threadpak::scope_guard_version! {
            pub(super) struct SuperShort over Scope, seated in mod super_short;
        }
        threadpak::scope_guard_version! {
            pub(in super) struct SuperLong over Scope, seated in mod super_long;
        }
        threadpak::scope_guard_version! {
            pub(in super::super) struct RelativeAncestor over Scope,
                seated in mod relative_ancestor;
        }
        threadpak::scope_guard_version! {
            pub(crate) struct CrateShort over Scope, seated in mod crate_short;
        }
        threadpak::scope_guard_version! {
            pub(in crate) struct CrateLong over Scope, seated in mod crate_long;
        }
        threadpak::scope_guard_version! {
            pub(in crate::scope_guard_visibility) struct AbsoluteAncestor over Scope,
                seated in mod absolute_ancestor;
        }
        crate_relative_scope_guard!(DollarCrateAncestor, Scope, dollar_crate_ancestor);

        pub(in crate::scope_guard_visibility) use absolute_ancestor::AbsoluteAncestor as AbsoluteSameReach;
        pub(crate) use crate_long::CrateLong as CrateLongSameReach;
        pub(crate) use crate_short::CrateShort as CrateShortSameReach;
        pub(in crate::scope_guard_visibility) use dollar_crate_ancestor::DollarCrateAncestor as DollarCrateSameReach;
        use private::Private as PrivateSameReach;
        pub(in super::super) use relative_ancestor::RelativeAncestor as RelativeSameReach;
        use self_long::SelfLong as SelfLongSameReach;
        use self_short::SelfShort as SelfShortSameReach;
        pub(super) use super_long::SuperLong as SuperLongSameReach;
        pub(super) use super_short::SuperShort as SuperShortSameReach;

        pub(super) fn local_surfaces() {
            assert_surface(Private::positioned, Private::try_cmp_same_scope);
            assert_surface(
                PrivateSameReach::positioned,
                PrivateSameReach::try_cmp_same_scope,
            );
            assert_surface(SelfShort::positioned, SelfShort::try_cmp_same_scope);
            assert_surface(
                SelfShortSameReach::positioned,
                SelfShortSameReach::try_cmp_same_scope,
            );
            assert_surface(SelfLong::positioned, SelfLong::try_cmp_same_scope);
            assert_surface(
                SelfLongSameReach::positioned,
                SelfLongSameReach::try_cmp_same_scope,
            );
            assert_surface(SuperShort::positioned, SuperShort::try_cmp_same_scope);
            assert_surface(
                SuperShortSameReach::positioned,
                SuperShortSameReach::try_cmp_same_scope,
            );
            assert_surface(SuperLong::positioned, SuperLong::try_cmp_same_scope);
            assert_surface(
                SuperLongSameReach::positioned,
                SuperLongSameReach::try_cmp_same_scope,
            );
            assert_surface(
                RelativeAncestor::positioned,
                RelativeAncestor::try_cmp_same_scope,
            );
            assert_surface(
                RelativeSameReach::positioned,
                RelativeSameReach::try_cmp_same_scope,
            );
            assert_surface(CrateShort::positioned, CrateShort::try_cmp_same_scope);
            assert_surface(
                CrateShortSameReach::positioned,
                CrateShortSameReach::try_cmp_same_scope,
            );
            assert_surface(CrateLong::positioned, CrateLong::try_cmp_same_scope);
            assert_surface(
                CrateLongSameReach::positioned,
                CrateLongSameReach::try_cmp_same_scope,
            );
            assert_surface(
                AbsoluteAncestor::positioned,
                AbsoluteAncestor::try_cmp_same_scope,
            );
            assert_surface(
                AbsoluteSameReach::positioned,
                AbsoluteSameReach::try_cmp_same_scope,
            );
            assert_surface(
                DollarCrateAncestor::positioned,
                DollarCrateAncestor::try_cmp_same_scope,
            );
            assert_surface(
                DollarCrateSameReach::positioned,
                DollarCrateSameReach::try_cmp_same_scope,
            );
        }
    }

    pub(crate) mod deep {
        use super::assert_surface;

        pub(crate) mod inner {
            use super::assert_surface;

            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub(crate) struct Scope;

            threadpak::scope_guard_version! {
                struct Private over Scope, seated in mod private;
            }
            threadpak::scope_guard_version! {
                pub(self) struct SelfShort over Scope, seated in mod self_short;
            }
            threadpak::scope_guard_version! {
                pub(in self) struct SelfLong over Scope, seated in mod self_long;
            }
            threadpak::scope_guard_version! {
                pub(super) struct SuperShort over Scope, seated in mod super_short;
            }
            threadpak::scope_guard_version! {
                pub(in super) struct SuperLong over Scope, seated in mod super_long;
            }
            threadpak::scope_guard_version! {
                pub(in super::super) struct RelativeAncestor over Scope,
                    seated in mod relative_ancestor;
            }
            threadpak::scope_guard_version! {
                pub(crate) struct CrateShort over Scope, seated in mod crate_short;
            }
            threadpak::scope_guard_version! {
                pub(in crate) struct CrateLong over Scope, seated in mod crate_long;
            }
            threadpak::scope_guard_version! {
                pub(in crate::scope_guard_visibility::deep) struct AbsoluteAncestor over Scope,
                    seated in mod absolute_ancestor;
            }
            crate_relative_scope_guard!(DollarCrateAncestor, Scope, dollar_crate_ancestor);

            pub(in crate::scope_guard_visibility::deep) use absolute_ancestor::AbsoluteAncestor as AbsoluteSameReach;
            pub(crate) use crate_long::CrateLong as CrateLongSameReach;
            pub(crate) use crate_short::CrateShort as CrateShortSameReach;
            pub(in crate::scope_guard_visibility) use dollar_crate_ancestor::DollarCrateAncestor as DollarCrateSameReach;
            use private::Private as PrivateSameReach;
            pub(in super::super) use relative_ancestor::RelativeAncestor as RelativeSameReach;
            use self_long::SelfLong as SelfLongSameReach;
            use self_short::SelfShort as SelfShortSameReach;
            pub(super) use super_long::SuperLong as SuperLongSameReach;
            pub(super) use super_short::SuperShort as SuperShortSameReach;

            pub(super) fn local_surfaces() {
                assert_surface(Private::positioned, Private::try_cmp_same_scope);
                assert_surface(
                    PrivateSameReach::positioned,
                    PrivateSameReach::try_cmp_same_scope,
                );
                assert_surface(SelfShort::positioned, SelfShort::try_cmp_same_scope);
                assert_surface(
                    SelfShortSameReach::positioned,
                    SelfShortSameReach::try_cmp_same_scope,
                );
                assert_surface(SelfLong::positioned, SelfLong::try_cmp_same_scope);
                assert_surface(
                    SelfLongSameReach::positioned,
                    SelfLongSameReach::try_cmp_same_scope,
                );
                assert_surface(SuperShort::positioned, SuperShort::try_cmp_same_scope);
                assert_surface(
                    SuperShortSameReach::positioned,
                    SuperShortSameReach::try_cmp_same_scope,
                );
                assert_surface(SuperLong::positioned, SuperLong::try_cmp_same_scope);
                assert_surface(
                    SuperLongSameReach::positioned,
                    SuperLongSameReach::try_cmp_same_scope,
                );
                assert_surface(
                    RelativeAncestor::positioned,
                    RelativeAncestor::try_cmp_same_scope,
                );
                assert_surface(
                    RelativeSameReach::positioned,
                    RelativeSameReach::try_cmp_same_scope,
                );
                assert_surface(CrateShort::positioned, CrateShort::try_cmp_same_scope);
                assert_surface(
                    CrateShortSameReach::positioned,
                    CrateShortSameReach::try_cmp_same_scope,
                );
                assert_surface(CrateLong::positioned, CrateLong::try_cmp_same_scope);
                assert_surface(
                    CrateLongSameReach::positioned,
                    CrateLongSameReach::try_cmp_same_scope,
                );
                assert_surface(
                    AbsoluteAncestor::positioned,
                    AbsoluteAncestor::try_cmp_same_scope,
                );
                assert_surface(
                    AbsoluteSameReach::positioned,
                    AbsoluteSameReach::try_cmp_same_scope,
                );
                assert_surface(
                    DollarCrateAncestor::positioned,
                    DollarCrateAncestor::try_cmp_same_scope,
                );
                assert_surface(
                    DollarCrateSameReach::positioned,
                    DollarCrateSameReach::try_cmp_same_scope,
                );
            }
        }

        pub(super) fn ancestor_surfaces() {
            inner::local_surfaces();
            assert_surface(
                inner::SuperShort::positioned,
                inner::SuperShort::try_cmp_same_scope,
            );
            assert_surface(
                inner::SuperShortSameReach::positioned,
                inner::SuperShortSameReach::try_cmp_same_scope,
            );
            assert_surface(
                inner::SuperLong::positioned,
                inner::SuperLong::try_cmp_same_scope,
            );
            assert_surface(
                inner::RelativeAncestor::positioned,
                inner::RelativeAncestor::try_cmp_same_scope,
            );
            assert_surface(
                inner::AbsoluteAncestor::positioned,
                inner::AbsoluteAncestor::try_cmp_same_scope,
            );
            assert_surface(
                inner::CrateShort::positioned,
                inner::CrateShort::try_cmp_same_scope,
            );
            assert_surface(
                inner::CrateLong::positioned,
                inner::CrateLong::try_cmp_same_scope,
            );
        }
    }

    pub(super) fn ancestor_surfaces() {
        shallow::local_surfaces();
        assert_surface(
            shallow::SuperShort::positioned,
            shallow::SuperShort::try_cmp_same_scope,
        );
        assert_surface(
            shallow::SuperLong::positioned,
            shallow::SuperLong::try_cmp_same_scope,
        );
        assert_surface(
            shallow::AbsoluteAncestor::positioned,
            shallow::AbsoluteAncestor::try_cmp_same_scope,
        );
        assert_surface(
            shallow::DollarCrateAncestor::positioned,
            shallow::DollarCrateAncestor::try_cmp_same_scope,
        );
        assert_surface(
            shallow::CrateShort::positioned,
            shallow::CrateShort::try_cmp_same_scope,
        );
        assert_surface(
            shallow::CrateLong::positioned,
            shallow::CrateLong::try_cmp_same_scope,
        );

        deep::ancestor_surfaces();
        assert_surface(
            deep::inner::RelativeAncestor::positioned,
            deep::inner::RelativeAncestor::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::DollarCrateAncestor::positioned,
            deep::inner::DollarCrateAncestor::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::CrateShort::positioned,
            deep::inner::CrateShort::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::CrateLong::positioned,
            deep::inner::CrateLong::try_cmp_same_scope,
        );
    }
}

/// The admitted narrow visibility grammar preserves its caller coordinate at
/// two nesting depths, including shorthand/equivalent forms, relative chains,
/// absolute `crate` paths, and macro-authored `$crate` paths. The downstream
/// `pub` control lives in `xtask/fixtures/macro-consumer/src/lib.rs`.
///
/// green: identity.scope-guard-visibility-is-caller-relative
#[test]
fn every_narrow_scope_guard_visibility_form_keeps_its_caller_coordinate() {
    scope_guard_visibility::ancestor_surfaces();
    assert_surface(
        scope_guard_visibility::shallow::CrateShort::positioned,
        scope_guard_visibility::shallow::CrateShort::try_cmp_same_scope,
    );
    assert_surface(
        scope_guard_visibility::shallow::CrateLong::positioned,
        scope_guard_visibility::shallow::CrateLong::try_cmp_same_scope,
    );
    assert_surface(
        scope_guard_visibility::deep::inner::CrateShort::positioned,
        scope_guard_visibility::deep::inner::CrateShort::try_cmp_same_scope,
    );
    assert_surface(
        scope_guard_visibility::deep::inner::CrateLong::positioned,
        scope_guard_visibility::deep::inner::CrateLong::try_cmp_same_scope,
    );
}

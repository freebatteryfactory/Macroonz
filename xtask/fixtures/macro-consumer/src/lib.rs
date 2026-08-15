//! The outside consumer fixture: the composition proof, shaped like a real
//! consumer.
//!
//! A compiler service never depends on its frontend surfaces, even for tests.
//! So the question "does a caller who holds the machine's types and wears the
//! shell's derive actually compile?" cannot be answered from inside either
//! participant — an answer from inside is the participant grading itself, and
//! it buys that answer with a dependency edge that reverses the topology.
//!
//! This crate answers it from outside. It depends on `threadpak` and on
//! `threadpak-macros`, exactly as an application would, and on neither of their
//! internals. Its public scope-guard specimen is intentional: rustdoc compiles
//! its examples as a crate outside this fixture, which proves the public
//! re-export crosses a real crate boundary while the narrower forms do not.
//!
//! It lives under `xtask/fixtures/` because it is tooling — first-class, never
//! on the production dependency path.

/// The complete visibility grammar of `scope_guard_version!`, exercised at two
/// nesting depths by a crate outside `threadpak`.
///
/// The public forms cross this fixture's crate boundary with both emitted
/// operations intact:
///
/// ```
/// use threadpak::identity::{AuthorityPosition, OrderComparison};
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::{
///     ShallowPublic, ShallowScope,
/// };
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::{
///     DeepPublic, DeepScope,
/// };
///
/// let _positioned: fn(AuthorityPosition<ShallowScope>) -> ShallowPublic =
///     ShallowPublic::positioned;
/// let _comparison: fn(&ShallowPublic, &ShallowPublic) -> Result<
///     core::cmp::Ordering,
///     OrderComparison,
/// > = ShallowPublic::try_cmp_same_scope;
/// let _deep_positioned: fn(AuthorityPosition<DeepScope>) -> DeepPublic =
///     DeepPublic::positioned;
/// let _deep_comparison: fn(&DeepPublic, &DeepPublic) -> Result<
///     core::cmp::Ordering,
///     OrderComparison,
/// > = DeepPublic::try_cmp_same_scope;
/// ```
///
/// Every narrower shallow form stops at this crate boundary independently:
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowPrivate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowSelf;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowInSelf;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowInSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowInSuperSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowCrate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowInCrate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowInAncestor;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::shallow::ShallowDollarCrate;
/// ```
///
/// The same independent refusals hold one module deeper:
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepPrivate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepSelf;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepInSelf;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepInSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepInSuperSuper;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepCrate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepInCrate;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepInAncestor;
/// ```
///
/// ```compile_fail
/// use threadpak_macro_consumer::scope_guard_visibility::deep::inner::DeepDollarCrate;
/// ```
pub mod scope_guard_visibility {
    use threadpak::identity::{AuthorityPosition, OrderComparison};

    macro_rules! crate_relative_scope_guard {
        ($name:ident, $scope:ty, $home:ident) => {
            threadpak::scope_guard_version! {
                /// A guard whose absolute visibility root was minted by an outer macro.
                pub(in $crate::scope_guard_visibility) struct $name over $scope,
                    seated in mod $home;
            }
        };
    }

    /// Ask the compiler to coerce both emitted operations to their exact public
    /// signatures. Calling this helper needs no runtime position mint.
    pub(crate) fn assert_surface<Guard, Scope>(
        _: fn(AuthorityPosition<Scope>) -> Guard,
        _: fn(&Guard, &Guard) -> Result<core::cmp::Ordering, OrderComparison>,
    ) {
    }

    /// One-module-deep invocations covering every admitted visibility form.
    pub mod shallow {
        use super::assert_surface;

        /// The scope shared by the shallow visibility specimens.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct ShallowScope;

        threadpak::scope_guard_version! {
            /// A guard private at its invocation coordinate.
            struct ShallowPrivate over ShallowScope, seated in mod shallow_private;
        }

        threadpak::scope_guard_version! {
            /// A guard explicitly private at its invocation coordinate.
            pub(self) struct ShallowSelf over ShallowScope, seated in mod shallow_self;
        }

        threadpak::scope_guard_version! {
            /// The long spelling of invocation-coordinate privacy.
            pub(in self) struct ShallowInSelf over ShallowScope, seated in mod shallow_in_self;
        }

        threadpak::scope_guard_version! {
            /// A guard visible to the invocation module's parent.
            pub(super) struct ShallowSuper over ShallowScope, seated in mod shallow_super;
        }

        threadpak::scope_guard_version! {
            /// The long spelling of parent visibility.
            pub(in super) struct ShallowInSuper over ShallowScope,
                seated in mod shallow_in_super;
        }

        threadpak::scope_guard_version! {
            /// A guard visible through two relative ancestor steps.
            pub(in super::super) struct ShallowInSuperSuper over ShallowScope,
                seated in mod shallow_in_super_super;
        }

        threadpak::scope_guard_version! {
            /// A guard visible throughout this fixture crate.
            pub(crate) struct ShallowCrate over ShallowScope, seated in mod shallow_crate;
        }

        threadpak::scope_guard_version! {
            /// The long spelling of crate visibility.
            pub(in crate) struct ShallowInCrate over ShallowScope,
                seated in mod shallow_in_crate;
        }

        threadpak::scope_guard_version! {
            /// A guard visible in the named ancestor module.
            pub(in crate::scope_guard_visibility) struct ShallowInAncestor over ShallowScope,
                seated in mod shallow_in_ancestor;
        }

        crate_relative_scope_guard!(ShallowDollarCrate, ShallowScope, shallow_dollar_crate);

        threadpak::scope_guard_version! {
            /// A guard exported from this downstream fixture.
            pub struct ShallowPublic over ShallowScope, seated in mod shallow_public;
        }

        /// Proves every shallow guard and both operations are reachable inside
        /// the invocation module.
        pub(super) fn local_surfaces_are_reachable() {
            assert_surface(
                ShallowPrivate::positioned,
                ShallowPrivate::try_cmp_same_scope,
            );
            assert_surface(ShallowSelf::positioned, ShallowSelf::try_cmp_same_scope);
            assert_surface(ShallowInSelf::positioned, ShallowInSelf::try_cmp_same_scope);
            assert_surface(ShallowSuper::positioned, ShallowSuper::try_cmp_same_scope);
            assert_surface(
                ShallowInSuper::positioned,
                ShallowInSuper::try_cmp_same_scope,
            );
            assert_surface(
                ShallowInSuperSuper::positioned,
                ShallowInSuperSuper::try_cmp_same_scope,
            );
            assert_surface(ShallowCrate::positioned, ShallowCrate::try_cmp_same_scope);
            assert_surface(
                ShallowInCrate::positioned,
                ShallowInCrate::try_cmp_same_scope,
            );
            assert_surface(
                ShallowInAncestor::positioned,
                ShallowInAncestor::try_cmp_same_scope,
            );
            assert_surface(
                ShallowDollarCrate::positioned,
                ShallowDollarCrate::try_cmp_same_scope,
            );
            assert_surface(ShallowPublic::positioned, ShallowPublic::try_cmp_same_scope);
        }
    }

    /// Two-module-deep invocations covering every admitted visibility form.
    pub mod deep {
        use super::assert_surface;

        /// The invocation module at the second nesting depth.
        pub mod inner {
            use super::assert_surface;

            /// The scope shared by the deep visibility specimens.
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct DeepScope;

            threadpak::scope_guard_version! {
                /// A guard private at its invocation coordinate.
                struct DeepPrivate over DeepScope, seated in mod deep_private;
            }

            threadpak::scope_guard_version! {
                /// A guard explicitly private at its invocation coordinate.
                pub(self) struct DeepSelf over DeepScope, seated in mod deep_self;
            }

            threadpak::scope_guard_version! {
                /// The long spelling of invocation-coordinate privacy.
                pub(in self) struct DeepInSelf over DeepScope, seated in mod deep_in_self;
            }

            threadpak::scope_guard_version! {
                /// A guard visible to the invocation module's parent.
                pub(super) struct DeepSuper over DeepScope, seated in mod deep_super;
            }

            threadpak::scope_guard_version! {
                /// The long spelling of parent visibility.
                pub(in super) struct DeepInSuper over DeepScope, seated in mod deep_in_super;
            }

            threadpak::scope_guard_version! {
                /// A guard visible through two relative ancestor steps.
                pub(in super::super) struct DeepInSuperSuper over DeepScope,
                    seated in mod deep_in_super_super;
            }

            threadpak::scope_guard_version! {
                /// A guard visible throughout this fixture crate.
                pub(crate) struct DeepCrate over DeepScope, seated in mod deep_crate;
            }

            threadpak::scope_guard_version! {
                /// The long spelling of crate visibility.
                pub(in crate) struct DeepInCrate over DeepScope, seated in mod deep_in_crate;
            }

            threadpak::scope_guard_version! {
                /// A guard visible in the named ancestor module.
                pub(in crate::scope_guard_visibility::deep) struct DeepInAncestor over DeepScope,
                    seated in mod deep_in_ancestor;
            }

            crate_relative_scope_guard!(DeepDollarCrate, DeepScope, deep_dollar_crate);

            threadpak::scope_guard_version! {
                /// A guard exported from this downstream fixture.
                pub struct DeepPublic over DeepScope, seated in mod deep_public;
            }

            /// Proves every deep guard and both operations are reachable inside
            /// the invocation module.
            pub(super) fn local_surfaces_are_reachable() {
                assert_surface(DeepPrivate::positioned, DeepPrivate::try_cmp_same_scope);
                assert_surface(DeepSelf::positioned, DeepSelf::try_cmp_same_scope);
                assert_surface(DeepInSelf::positioned, DeepInSelf::try_cmp_same_scope);
                assert_surface(DeepSuper::positioned, DeepSuper::try_cmp_same_scope);
                assert_surface(DeepInSuper::positioned, DeepInSuper::try_cmp_same_scope);
                assert_surface(
                    DeepInSuperSuper::positioned,
                    DeepInSuperSuper::try_cmp_same_scope,
                );
                assert_surface(DeepCrate::positioned, DeepCrate::try_cmp_same_scope);
                assert_surface(DeepInCrate::positioned, DeepInCrate::try_cmp_same_scope);
                assert_surface(
                    DeepInAncestor::positioned,
                    DeepInAncestor::try_cmp_same_scope,
                );
                assert_surface(
                    DeepDollarCrate::positioned,
                    DeepDollarCrate::try_cmp_same_scope,
                );
                assert_surface(DeepPublic::positioned, DeepPublic::try_cmp_same_scope);
            }
        }

        /// Proves the parent- and named-ancestor forms, plus the wider forms,
        /// remain reachable at the deep invocation's parent.
        pub(super) fn ancestor_surfaces_are_reachable() {
            inner::local_surfaces_are_reachable();
            assert_surface(
                inner::DeepSuper::positioned,
                inner::DeepSuper::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepInSuper::positioned,
                inner::DeepInSuper::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepInSuperSuper::positioned,
                inner::DeepInSuperSuper::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepInAncestor::positioned,
                inner::DeepInAncestor::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepCrate::positioned,
                inner::DeepCrate::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepInCrate::positioned,
                inner::DeepInCrate::try_cmp_same_scope,
            );
            assert_surface(
                inner::DeepPublic::positioned,
                inner::DeepPublic::try_cmp_same_scope,
            );
        }
    }

    /// Proves every form reachable in this common ancestor keeps both emitted
    /// operations there.
    pub fn ancestor_surfaces_are_reachable() {
        shallow::local_surfaces_are_reachable();
        assert_surface(
            shallow::ShallowSuper::positioned,
            shallow::ShallowSuper::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowInSuper::positioned,
            shallow::ShallowInSuper::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowInSuperSuper::positioned,
            shallow::ShallowInSuperSuper::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowCrate::positioned,
            shallow::ShallowCrate::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowInCrate::positioned,
            shallow::ShallowInCrate::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowInAncestor::positioned,
            shallow::ShallowInAncestor::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowDollarCrate::positioned,
            shallow::ShallowDollarCrate::try_cmp_same_scope,
        );
        assert_surface(
            shallow::ShallowPublic::positioned,
            shallow::ShallowPublic::try_cmp_same_scope,
        );

        deep::ancestor_surfaces_are_reachable();
        assert_surface(
            deep::inner::DeepCrate::positioned,
            deep::inner::DeepCrate::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::DeepInCrate::positioned,
            deep::inner::DeepInCrate::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::DeepDollarCrate::positioned,
            deep::inner::DeepDollarCrate::try_cmp_same_scope,
        );
        assert_surface(
            deep::inner::DeepPublic::positioned,
            deep::inner::DeepPublic::try_cmp_same_scope,
        );
    }
}

/// Proves the two `pub(crate)` forms and both public forms are reachable from
/// this downstream fixture's crate root, outside either invocation ancestry.
pub fn scope_guard_crate_surfaces_are_reachable() {
    use scope_guard_visibility::{assert_surface, deep, shallow};

    assert_surface(
        shallow::ShallowCrate::positioned,
        shallow::ShallowCrate::try_cmp_same_scope,
    );
    assert_surface(
        shallow::ShallowInSuperSuper::positioned,
        shallow::ShallowInSuperSuper::try_cmp_same_scope,
    );
    assert_surface(
        shallow::ShallowPublic::positioned,
        shallow::ShallowPublic::try_cmp_same_scope,
    );
    assert_surface(
        deep::inner::DeepCrate::positioned,
        deep::inner::DeepCrate::try_cmp_same_scope,
    );
    assert_surface(
        deep::inner::DeepPublic::positioned,
        deep::inner::DeepPublic::try_cmp_same_scope,
    );
}

#[cfg(test)]
mod tests {
    use threadpak::refusal::{
        CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
        LocalCauseKey, RefusalFamily, RefusalFamilyId,
    };

    /// The family identity both implementations below declare, written once
    /// here because the hand-written twin states it as a value exactly as the
    /// derived implementation emits it.
    const FAMILY: RefusalFamilyId = RefusalFamilyId::declared("consumer.demo");

    /// The derived family. The caller states three things — the shape, the
    /// variants, and one local key per cause — and writes no
    /// selection-order string anywhere. The declared order is deliberately NOT
    /// the order the variants are written in, so a derive that read the body
    /// layout instead of the order clause would fail the parity below.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, threadpak_macros::RefusalFamily)]
    #[refusal(
        family = "consumer.demo",
        shape = single_cause,
        order(
            NotCanonical = "not-canonical",
            NotAdmitted = "not-admitted",
            Unbounded = "unbounded",
        )
    )]
    enum DerivedDemoFamily {
        NotAdmitted,
        Unbounded,
        NotCanonical,
    }

    /// The hand-written twin: the same family, authored the way every refusal
    /// family in the machine is authored today. It is the bar the derive has to
    /// meet, and it is written here — outside the services — so that neither
    /// participant grades itself.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HandWrittenDemoFamily {
        NotAdmitted,
        Unbounded,
        NotCanonical,
    }

    impl RefusalFamily for HandWrittenDemoFamily {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] =
            &["NotCanonical", "NotAdmitted", "Unbounded"];
    }

    impl CauseOrderDeclaration for HandWrittenDemoFamily {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(
                CauseId::declared(FAMILY, LocalCauseKey::declared("not-canonical")),
                "NotCanonical",
            ),
            DeclaredCause::declared(
                CauseId::declared(FAMILY, LocalCauseKey::declared("not-admitted")),
                "NotAdmitted",
            ),
            DeclaredCause::declared(
                CauseId::declared(FAMILY, LocalCauseKey::declared("unbounded")),
                "Unbounded",
            ),
        ]);
    }

    /// The parity proof: the derived implementation and the hand-written twin
    /// declare the same shape, the same textual selection order, and the same
    /// typed cause order — identity for identity and position for position.
    #[test]
    fn the_derived_family_matches_its_hand_written_twin() {
        assert_eq!(DerivedDemoFamily::SHAPE, HandWrittenDemoFamily::SHAPE);
        assert_eq!(
            DerivedDemoFamily::SELECTION_ORDER,
            HandWrittenDemoFamily::SELECTION_ORDER
        );
        assert_eq!(
            DerivedDemoFamily::DECLARED_ORDER.len(),
            HandWrittenDemoFamily::DECLARED_ORDER.len()
        );
        let identical = DerivedDemoFamily::DECLARED_ORDER
            .iter()
            .zip(HandWrittenDemoFamily::DECLARED_ORDER.iter())
            .all(|(derived, twin)| {
                derived.id() == twin.id() && derived.spelling() == twin.spelling()
            });
        assert!(identical);
    }

    /// The projection law holds of the derived implementation exactly as it
    /// holds of the twin: the textual selection order is the typed order's
    /// projection, and the derive emitted it from the typed rows rather than
    /// from anything the caller wrote as a string.
    #[test]
    fn the_derived_selection_order_projects_the_derived_typed_order() {
        assert!(DerivedDemoFamily::DECLARED_ORDER.projects_to(DerivedDemoFamily::SELECTION_ORDER));
        assert!(
            HandWrittenDemoFamily::DECLARED_ORDER
                .projects_to(HandWrittenDemoFamily::SELECTION_ORDER)
        );
        assert_eq!(
            DerivedDemoFamily::DECLARED_ORDER
                .ordinal_of(CauseId::declared(
                    FAMILY,
                    LocalCauseKey::declared("unbounded")
                ))
                .map(threadpak::refusal::CauseOrdinal::position),
            Some(2)
        );
        // The family seat travels in the value: every derived row says which
        // family owns it without anybody cutting a string apart.
        assert!(
            DerivedDemoFamily::DECLARED_ORDER
                .iter()
                .all(|row| row.id().family() == FAMILY)
        );
    }

    /// The caller-relative visibility grammar compiles in an outside consumer;
    /// both public forms are checked again by rustdoc from one crate farther
    /// out. The routed positive control lives in testpak's dedicated matrix.
    #[test]
    fn every_scope_guard_visibility_form_keeps_its_caller_coordinate() {
        crate::scope_guard_visibility::ancestor_surfaces_are_reachable();
        crate::scope_guard_crate_surfaces_are_reachable();
    }

    /// The values themselves still behave like an ordinary Rust enum: the
    /// derive added declared facts and took nothing away.
    #[test]
    fn the_derived_family_is_still_an_ordinary_enum() {
        let causes = [
            DerivedDemoFamily::NotCanonical,
            DerivedDemoFamily::NotAdmitted,
            DerivedDemoFamily::Unbounded,
        ];
        let twins = [
            HandWrittenDemoFamily::NotCanonical,
            HandWrittenDemoFamily::NotAdmitted,
            HandWrittenDemoFamily::Unbounded,
        ];
        assert_eq!(causes.len(), twins.len());
        assert_ne!(causes.first(), causes.get(1));
        assert_ne!(twins.first(), twins.get(1));
        assert_eq!(causes.first(), Some(&DerivedDemoFamily::NotCanonical));
    }
}

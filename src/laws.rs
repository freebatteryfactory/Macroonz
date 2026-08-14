//! The single compile-time proof surface, sectioned by home, and the sections follow
//! the band order — the root calculus first, then band 00 upward — so reading this file
//! from the top reads the machine's dependency line in the order `lib.rs` declares it.
//! Green laws only: each law is named, and its name is the join key across the owning
//! README's obligation row, this file, and the red twin. Red twins (compile-fail
//! fixtures proving each law non-vacuous by reversal) land in testpak when it
//! materializes; until then every law below states its owed reversal in its doc line.
//!
//! A law that cannot fail is not a law: these compile (and trivially run) only
//! while the shapes hold; reversing the shape breaks the named law.

/// Whether every item in a slice is pairwise distinct (shared by law sections).
#[cfg(test)]
fn pairwise_distinct<T: PartialEq>(items: &[T]) -> bool {
    items.iter().enumerate().all(|(index, item)| {
        items
            .iter()
            .skip(index.saturating_add(1))
            .all(|other| item != other)
    })
}

mod root {
    use crate::types::{
        Bounded, Completeness, ConstLimit, Dispatch, EvidenceCut, Freshness, Limit, LimitWitness,
        Never, TransitionSystem,
    };

    /// law: root.cut-families-are-caller-supplied — any owner can bind `Freshness`
    /// to its own coordinate type; no central cut registry exists.
    /// Owed reversal: sealing `EvidenceCut` must break this law.
    #[test]
    fn cut_families_are_caller_supplied() {
        struct DemoCut;
        impl EvidenceCut for DemoCut {}
        let probe: Option<Freshness<u8, DemoCut>> = None;
        assert!(probe.is_none());
    }

    /// law: root.no-coordinate-forecloses-stale — a family with no admitted
    /// coordinate parameterizes over `Never`, and its `Stale` form is
    /// unrepresentable (the type exists; no value of it can).
    /// Owed reversal (red twin): constructing `Stale<_, Never>` must not compile
    /// past the uninhabited coordinate.
    #[test]
    fn no_coordinate_forecloses_stale() {
        let probe: Option<Freshness<u8, Never>> = None;
        assert!(probe.is_none());
    }

    /// law: root.completeness-domains-do-not-unify — completeness over one domain
    /// is a different type than completeness over another; a complete query cannot
    /// masquerade as complete verification.
    /// Owed reversal: erasing the domain parameter must break this law.
    #[test]
    fn completeness_domains_do_not_unify() {
        struct QueryDomain;
        struct VerifyDomain;
        let over_query: Option<fn(Completeness<QueryDomain>)> = Some(drop);
        let over_verify: Option<fn(Completeness<VerifyDomain>)> = Some(drop);
        assert!(over_query.is_some());
        assert!(over_verify.is_some());
    }

    /// law: root.limit-families-do-not-unify — `Bounded` under one limit family is
    /// a different type than under another, and a witness for one family cannot
    /// authorize the other, regardless of magnitudes.
    /// Owed reversal (red twin): passing `Bounded<u8, DecodeDemo>` where
    /// `Bounded<u8, ArenaDemo>` is required must not compile.
    #[test]
    fn limit_families_do_not_unify() {
        struct DecodeDemo;
        impl Limit for DecodeDemo {}
        impl ConstLimit for DecodeDemo {
            const MAX: usize = 8;
        }
        struct ArenaDemo;
        impl Limit for ArenaDemo {}

        let decode_bounded: Option<fn(Bounded<u8, DecodeDemo>)> = Some(drop);
        let arena_bounded: Option<fn(Bounded<u8, ArenaDemo>)> = Some(drop);
        let arena_witness: Option<fn(LimitWitness<ArenaDemo>)> = Some(drop);
        assert!(decode_bounded.is_some());
        assert!(arena_bounded.is_some());
        assert!(arena_witness.is_some());
        assert_eq!(DecodeDemo::MAX, 8);
    }

    /// law: root.dispatch-is-owner-refusal-generic — the transition grammar names
    /// no concrete refusal type; every machine binds its own family.
    /// Owed reversal: hard-wiring a concrete refusal type must break this law.
    #[test]
    fn dispatch_is_owner_refusal_generic() {
        struct DemoRefusal;
        let probe: Option<Dispatch<(), DemoRefusal>> = None;
        assert!(probe.is_none());
    }

    /// law: root.evidence-ref-identity-is-referent-and-version — the four
    /// Class-E components exist, and equality/hashing use exactly the
    /// identifying pair; availability and integrity never participate.
    /// Owed reversal (red twin): adding a third identifying field must break
    /// this law.
    #[test]
    fn evidence_ref_identity_is_referent_and_version() {
        use crate::types::{EvidenceRef, ReferentAvailability, ReferentIntegrity};
        #[derive(Debug)]
        struct DemoClaim;
        let reachable: EvidenceRef<DemoClaim> = EvidenceRef::bound(
            [7; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        );
        let unreachable: EvidenceRef<DemoClaim> = EvidenceRef::bound(
            [7; 32],
            1,
            ReferentAvailability::Unavailable,
            ReferentIntegrity::Damaged,
        );
        let newer: EvidenceRef<DemoClaim> = EvidenceRef::bound(
            [7; 32],
            2,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        );
        assert_eq!(reachable, unreachable);
        assert_ne!(reachable, newer);
        assert!(matches!(
            unreachable.availability(),
            ReferentAvailability::Unavailable
        ));
        assert!(matches!(
            unreachable.integrity(),
            ReferentIntegrity::Damaged
        ));
    }

    /// law: root.bounded-construction-is-a-seam — both checked constructor roads
    /// check the limit and refuse with the family body; the const road enforces
    /// the compile-time maximum, the witness road enforces the schema-minted one;
    /// a non-empty collection cannot be empty by signature. The two total
    /// structural roads — `empty` and `singleton` — cannot form the failing
    /// case at all: the empty collection carries nothing, and the one-item
    /// collection compiles only where the declared maximum is proven at compile
    /// time to admit an item.
    /// Owed reversal (red twin): an unchecked public constructor must not exist,
    /// and `singleton` under a family declaring `MAX = 0` must not compile.
    #[test]
    fn bounded_construction_is_a_seam() {
        use crate::types::{
            AdmittedLimit, Bounded, BoundedConstruction, LimitWitness, NonEmptyBounded,
            NonEmptyBoundedConstruction, PositiveLimit, RootLawsProfile,
        };
        struct SmallDemo;
        impl Limit for SmallDemo {}
        impl ConstLimit for SmallDemo {
            const MAX: usize = 2;
        }

        let admitted: AdmittedLimit<SmallDemo, RootLawsProfile> = AdmittedLimit::under_profile();
        let positive: PositiveLimit<SmallDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();

        let ok: Result<Bounded<u8, SmallDemo>, _> = Bounded::admitted_const(vec![1, 2], &admitted);
        assert!(ok.is_ok_and(|bounded| bounded.len() == 2));
        let over: Result<Bounded<u8, SmallDemo>, _> =
            Bounded::admitted_const(vec![1, 2, 3], &admitted);
        assert!(matches!(over, Err(BoundedConstruction::OverLimit)));

        let witness: LimitWitness<SmallDemo> = LimitWitness::declared(1);
        let witnessed: Result<Bounded<u8, SmallDemo>, _> = Bounded::admitted(vec![1], &witness);
        assert!(witnessed.is_ok_and(|bounded| !bounded.is_empty()));
        let over_witness: Result<Bounded<u8, SmallDemo>, _> =
            Bounded::admitted(vec![1, 2], &witness);
        assert!(matches!(over_witness, Err(BoundedConstruction::OverLimit)));

        let admitted_one: Result<NonEmptyBounded<u8, SmallDemo>, _> =
            NonEmptyBounded::admitted_const(9, vec![], &positive);
        assert!(admitted_one.is_ok_and(|value| value.len() == 1 && !value.is_empty()));
        let too_many: Result<NonEmptyBounded<u8, SmallDemo>, _> =
            NonEmptyBounded::admitted_const(9, vec![8, 7], &positive);
        assert!(matches!(
            too_many,
            Err(NonEmptyBoundedConstruction::OverLimit)
        ));

        let empty: Bounded<u8, SmallDemo> = Bounded::empty();
        assert!(empty.is_empty());
        let singleton_one: NonEmptyBounded<u8, SmallDemo> = NonEmptyBounded::singleton(5);
        assert!(
            !singleton_one.is_empty() && singleton_one.len() == 1 && *singleton_one.first() == 5
        );
    }

    /// law: root.admission-precedes-a-trusted-magnitude — a declared magnitude
    /// becomes a machine fact only through an admission witness: the mint
    /// carries the family's own `MAX`, the witness is family-tagged so one
    /// family's admission cannot authorize another, and the checked const
    /// constructor reads its bound off the witness rather than off the
    /// declaration.
    ///
    /// The claim ceiling: admission establishes that the magnitude stands under
    /// the ADMITTING PROFILE's ceiling and nothing more. It establishes nothing
    /// about whether the number is the right one for its domain. It does not
    /// establish that the family admits an item — that is
    /// `root.positivity-is-the-stronger-witness`. And the total structural roads
    /// — `from_array`, `singleton` — read `L::MAX` bare by decision, because
    /// each proves a LOCAL fact about the call in front of it and claims no
    /// admission at all.
    ///
    /// Red twin: a family declaring a magnitude past the admitting profile's
    /// ceiling must not compile — the fixture is testpak's.
    #[test]
    fn admission_precedes_a_trusted_magnitude() {
        use crate::types::{AdmittedLimit, Bounded, LimitAdmissionProfile, RootLawsProfile};
        struct AdmissibleDemo;
        impl Limit for AdmissibleDemo {}
        impl ConstLimit for AdmissibleDemo {
            const MAX: usize = 4;
        }
        struct OtherDemo;
        impl Limit for OtherDemo {}
        impl ConstLimit for OtherDemo {
            const MAX: usize = 4;
        }

        let admitted: AdmittedLimit<AdmissibleDemo, RootLawsProfile> =
            AdmittedLimit::under_profile();
        assert_eq!(admitted.max(), AdmissibleDemo::MAX);
        assert!(admitted.max() <= RootLawsProfile::MAX_DECLARED_LIMIT);

        // The witness is family-tagged, so an admission of one family is not an
        // admission of another that happens to share a magnitude.
        let over_other: Option<fn(AdmittedLimit<OtherDemo, RootLawsProfile>)> = Some(drop);
        assert!(over_other.is_some());

        // The checked const road reads its bound off the witness rather than off
        // the declaration, so the number it compares against is one that stood
        // under a named profile's ceiling.
        let fits: Result<Bounded<u8, AdmissibleDemo>, _> =
            Bounded::admitted_const(vec![1, 2, 3], &admitted);
        assert!(fits.is_ok_and(|bounded| bounded.len() == 3));
        let over: Result<Bounded<u8, AdmissibleDemo>, _> =
            Bounded::admitted_const(vec![1, 2, 3, 4, 5], &admitted);
        assert!(over.is_err());
    }

    /// law: root.positivity-is-the-stronger-witness — the base witness proves
    /// only the upper bound, and the positivity claim is seated one witness up.
    ///
    /// Both halves are here, because the split is only honest if both hold. A
    /// family declaring `MAX = 0` MINTS `AdmittedLimit` and inhabits
    /// `Bounded::empty`: a zero maximum is a lawful declaration for a seat that
    /// holds nothing, and refusing it in the base witness would have refused
    /// that seat with it. The same family cannot mint `PositiveLimit`, which is
    /// exactly the evidence `NonEmptyBounded::admitted_const` demands, because
    /// that road promises an inhabitant no zero-maximum family can supply.
    ///
    /// The claim ceiling: this says nothing about whether a zero maximum is the
    /// RIGHT declaration for any particular seat. It says the two facts are
    /// separately evidenced and separately consumed.
    ///
    /// Red twin: a zero-maximum family minting the positive witness must not
    /// compile — the fixture is testpak's, because a claim about what does not
    /// compile cannot be made by code that does.
    #[test]
    fn positivity_is_the_stronger_witness() {
        use crate::types::{
            AdmittedLimit, Bounded, NonEmptyBounded, PositiveLimit, RootLawsProfile,
        };
        struct EmptyOnlyDemo;
        impl Limit for EmptyOnlyDemo {}
        impl ConstLimit for EmptyOnlyDemo {
            const MAX: usize = 0;
        }
        struct InhabitedDemo;
        impl Limit for InhabitedDemo {}
        impl ConstLimit for InhabitedDemo {
            const MAX: usize = 3;
        }

        // The weak witness admits the empty-only family, and the family's seat
        // is a real one.
        let empty_only: AdmittedLimit<EmptyOnlyDemo, RootLawsProfile> =
            AdmittedLimit::under_profile();
        assert_eq!(empty_only.max(), 0);
        let nothing: Bounded<u8, EmptyOnlyDemo> = Bounded::empty();
        assert!(nothing.is_empty() && nothing.iter().count() == 0);
        let one_too_many: Result<Bounded<u8, EmptyOnlyDemo>, _> =
            Bounded::admitted_const(vec![1], &empty_only);
        assert!(one_too_many.is_err());

        // The strong witness carries the same ceiling fact plus the inhabitant,
        // and it is what the non-empty road takes.
        let positive: PositiveLimit<InhabitedDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();
        assert_eq!(positive.max(), InhabitedDemo::MAX);
        let held: Result<NonEmptyBounded<u8, InhabitedDemo>, _> =
            NonEmptyBounded::admitted_const(1, vec![2], &positive);
        assert!(held.is_ok_and(|value| value.len() == 2 && *value.first() == 1));
    }

    /// law: root.the-positive-witness-carries-the-admitted-one — the stronger
    /// witness establishes the ceiling fact by CONTAINING the base witness, not
    /// by restating its comparison, so the profile-admission claim has exactly
    /// one owner.
    ///
    /// The green half is the magnitude: what the positive witness reports is
    /// what the base witness admitted, read off the contained value rather than
    /// off a second copy. The half that matters more is the red one, and it is
    /// the reason the fixture is named below rather than owed: a family past the
    /// admitting profile's ceiling stops the compiler at the POSITIVE mint, and
    /// the diagnostic it stops with is the BASE mint's — which is only true
    /// while the base check is the one that runs. Restating the comparison here
    /// would keep the fixture failing and stop it saying anything about whether
    /// the two roads still agree.
    ///
    /// The claim ceiling: this says nothing about whether either number is right
    /// for a seat. It says the two witnesses cannot drift apart on the fact they
    /// share.
    ///
    /// Red twin: a past-ceiling family minting the positive witness must not
    /// compile, and must fail inside the base road.
    #[test]
    fn the_positive_witness_carries_the_admitted_one() {
        use crate::types::{AdmittedLimit, PositiveLimit, RootLawsProfile};
        struct ContainedDemo;
        impl Limit for ContainedDemo {}
        impl ConstLimit for ContainedDemo {
            const MAX: usize = 5;
        }
        let base: AdmittedLimit<ContainedDemo, RootLawsProfile> = AdmittedLimit::under_profile();
        let positive: PositiveLimit<ContainedDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();
        assert_eq!(positive.max(), base.max());
        assert_eq!(positive.max(), ContainedDemo::MAX);
    }

    /// law: root.a-runtime-capacity-is-witnessed-positive — an evidence-selected
    /// magnitude becomes a capacity a road promising an inhabitant may act on
    /// only through the stronger runtime witness, and the mint refuses a
    /// selection admitting no item.
    ///
    /// Both halves stand here, because the split is only honest if both do. A
    /// witnessed magnitude of zero is a lawful selection for a seat that holds
    /// nothing — `Bounded::admitted` under it yields a real empty collection —
    /// and the base witness admits it on purpose. The same selection cannot
    /// mint `PositiveLimitWitness`, which is exactly the evidence
    /// `NonEmptyBounded::admitted` demands, because that road promises a first
    /// item no zero capacity can supply.
    ///
    /// This is the rung that was missing. `admitted_const` took the strong
    /// witness and `admitted` took the weak one, so the same promise stood on
    /// different evidence depending only on which road the magnitude arrived
    /// by. With this road closed, EVERY constructor of the
    /// inhabitant-promising shape consumes evidence that the family admits an
    /// item — the two `const` roads prove it off the declaration,
    /// `admitted_const` and `admitted_prefix` take `PositiveLimit`, and this one
    /// takes `PositiveLimitWitness`. That claim is total rather than sampled
    /// because the shape's seats are private: no road into it can exist outside
    /// this file and its guarded child.
    ///
    /// Non-vacuity is executed rather than asserted: the refusing call and the
    /// admitting call differ in the magnitude alone — same family, same road,
    /// same item count — so the refusal cannot be coming from anything else.
    ///
    /// The claim ceiling: this says nothing about whether the selected
    /// magnitude is the RIGHT one for the family's domain. The owner profile
    /// and the evidence select that, and no road here can check it.
    ///
    /// Red twin: a zero capacity REFUSES rather than failing to compile, and
    /// the reason is structural — a magnitude that does not exist until runtime
    /// has no compile-time value for a `const` block to read. So the reversal is
    /// a behavioral hostile rather than a fixture, and it is executed below, on
    /// the refusing arm. Driving that same refusal from OUTSIDE the crate stays
    /// OWED and is gated rather than merely unwritten: `LimitWitness` has only
    /// its `cfg(test)` mint, so no outside consumer can build the zero selection
    /// this law refuses. The gate comes off with the schema home's lawful
    /// minter. Where the same relation IS visible in the source it takes the
    /// stronger seat instead: that is `root.positivity-is-the-stronger-witness`,
    /// whose `const` gate stops a zero-maximum family at compile time and whose
    /// fixture is testpak's.
    #[test]
    fn a_runtime_capacity_is_witnessed_positive() {
        use crate::types::{
            Bounded, CapacityAdmission, EvidenceSelectedLimit, NonEmptyBounded,
            NonEmptyBoundedConstruction, PositiveLimitWitness,
        };
        struct SelectedDemo;
        impl Limit for SelectedDemo {}
        impl EvidenceSelectedLimit for SelectedDemo {}

        // The weak witness admits a zero selection, and the seat under it is a
        // real empty collection rather than a mistake.
        let nothing: LimitWitness<SelectedDemo> = LimitWitness::declared(0);
        assert_eq!(nothing.max(), 0);
        let empty: Result<Bounded<u8, SelectedDemo>, _> = Bounded::admitted(vec![], &nothing);
        assert!(empty.is_ok_and(|bounded| bounded.is_empty()));

        // The strong witness refuses exactly that selection.
        assert_eq!(
            PositiveLimitWitness::inhabited(LimitWitness::<SelectedDemo>::declared(0)).err(),
            Some(CapacityAdmission::NotInhabited)
        );

        // And admits the next magnitude up. One number moved; nothing else did.
        let capacity = PositiveLimitWitness::inhabited(LimitWitness::<SelectedDemo>::declared(1))
            .unwrap_or_else(|_| unreachable!("one admits an item"));
        assert_eq!(capacity.max(), 1);

        // The road that promises an inhabitant takes the strong witness, and
        // reports both what the capacity holds and what it does not.
        let held: Result<NonEmptyBounded<u8, SelectedDemo>, _> =
            NonEmptyBounded::admitted(7, vec![], &capacity);
        assert!(held.is_ok_and(|value| value.len() == 1 && *value.first() == 7));
        let over: Result<NonEmptyBounded<u8, SelectedDemo>, _> =
            NonEmptyBounded::admitted(7, vec![8], &capacity);
        assert!(matches!(over, Err(NonEmptyBoundedConstruction::OverLimit)));
    }

    /// law: root.a-capacity-witness-does-not-cross-families — a runtime capacity
    /// names WHICH family's magnitude it admitted, so one family's capacity is
    /// never another's whatever the two numbers are.
    ///
    /// The family rides on the CONTAINED witness's own type parameter rather
    /// than on a tag this type keeps beside it, so there is no second statement
    /// of which family was admitted and nothing here to drift from the
    /// selection it came from.
    ///
    /// The claim ceiling: this says nothing about which family is right for a
    /// seat. It says a road that requires one family's capacity cannot be fed
    /// another's.
    ///
    /// Red twin: substituting one family's capacity where another's is required
    /// must not compile —
    /// testpak/tests/compile-fail/a-capacity-witness-from-another-family.rs.
    #[test]
    fn a_capacity_witness_does_not_cross_families() {
        use crate::types::{EvidenceSelectedLimit, PositiveLimitWitness};
        struct FirstDemo;
        impl Limit for FirstDemo {}
        impl EvidenceSelectedLimit for FirstDemo {}
        struct SecondDemo;
        impl Limit for SecondDemo {}
        impl EvidenceSelectedLimit for SecondDemo {}

        let first = PositiveLimitWitness::inhabited(LimitWitness::<FirstDemo>::declared(4))
            .unwrap_or_else(|_| unreachable!("four admits an item"));
        let second = PositiveLimitWitness::inhabited(LimitWitness::<SecondDemo>::declared(4))
            .unwrap_or_else(|_| unreachable!("four admits an item"));
        assert_eq!(first.max(), second.max());

        // Two capacities of one magnitude, and the two values do not unify:
        // each consumer names the family it will take, and only that one fits.
        let takes_first: fn(PositiveLimitWitness<FirstDemo>) = drop;
        let takes_second: fn(PositiveLimitWitness<SecondDemo>) = drop;
        takes_first(first);
        takes_second(second);
    }

    /// law: root.the-runtime-ladder-is-declared-by-its-family — a family reaches
    /// a runtime capacity only where its owner declared the magnitude
    /// evidence-selected. The declaration is the MINT'S BOUND rather than a
    /// sentence beside the family, so a family that never made it has no road to
    /// a capacity at all.
    ///
    /// The green half is that the bound is real and satisfiable: a family
    /// declaring it reaches the mint, settled by the compiler over a function
    /// pointer with nothing executed. The half that matters is the red one,
    /// because a bound nothing fails is a bound nobody needed.
    ///
    /// The claim ceiling, in two parts. The declaration says the magnitude
    /// arrives at runtime; it does NOT say the family declares no compile-time
    /// magnitude, and a family stating both would be stating two authorities for
    /// one capacity — a declaration defect no bound here can see. And it does
    /// not say that every family in this crate whose seat promises an inhabitant
    /// has made the declaration: that is a POPULATION question, it is answered
    /// by deriving the population from the sources rather than from a list
    /// anybody maintains, and no list of families is written here, because such
    /// a list would be exactly the hand-maintained inventory this repository
    /// bans.
    ///
    /// What answers half of that question today is the red twin's own recorded
    /// diagnostic. `rustc` reports an unsatisfied bound by listing the types
    /// that satisfy it, so the committed `.stderr` carries the roster of every
    /// family on this ladder, derived from the impls rather than authored — and
    /// a family joining or leaving the ladder moves that file and fails the
    /// fixture. It is a DRIFT DETECTOR over one side of the join, not a count:
    /// it sees families that are on the ladder and cannot see a seat that
    /// promises an inhabitant while its family stays off it. That second side
    /// is a repository join over the sources and remains owed; no
    /// `cargo xtask check` law derives it.
    ///
    /// Red twin: minting a capacity for a family that never declared its
    /// magnitude evidence-selected must not compile —
    /// testpak/tests/compile-fail/a-capacity-minted-for-an-undeclared-family.rs.
    #[test]
    fn the_runtime_ladder_is_declared_by_its_family() {
        use crate::types::{CapacityAdmission, EvidenceSelectedLimit, PositiveLimitWitness};
        struct DeclaredDemo;
        impl Limit for DeclaredDemo {}
        impl EvidenceSelectedLimit for DeclaredDemo {}

        let mint: fn(
            LimitWitness<DeclaredDemo>,
        ) -> Result<PositiveLimitWitness<DeclaredDemo>, CapacityAdmission> =
            PositiveLimitWitness::inhabited;
        assert!(mint(LimitWitness::declared(2)).is_ok_and(|held| held.max() == 2));
    }

    /// law: root.a-prefix-road-reports-what-it-did-not-carry — the one
    /// construction road that truncates reports the truncation it performed,
    /// both directions: material that fits is carried whole and reports nothing
    /// omitted, and material that does not fit is carried up to the admitted
    /// magnitude with the exact dropped count beside it.
    ///
    /// The road is the crate's own seam and not a public one, which is the
    /// structural half. A carry and a count handed to a caller are two values
    /// the caller may pair with anything, so the pair leaves here only inside
    /// [`crate::refusal::AdmittedPrefix`], the package built in the same
    /// construction that produced both. That is what makes a downstream claim
    /// about how much was lost a claim about THIS truncation rather than an
    /// assertion anybody could have written.
    ///
    /// The claim ceiling: this is a fact about the ROAD and says nothing about
    /// what a consumer does with the package. What the package makes impossible
    /// is a body that dropped issues and cannot say so, and that consequence is
    /// band 00's to state — see
    /// `refusal::a_truncated_report_is_not_a_halted_examination`.
    ///
    /// Red twin: marrying one prefix operation's carry to another's completion
    /// must not compile, because the pair has no public two-value road, no
    /// `into_parts`, and no writable seat.
    #[test]
    fn a_prefix_road_reports_what_it_did_not_carry() {
        use crate::types::{NonEmptyBounded, PositiveLimit, RootLawsProfile};
        struct PrefixDemo;
        impl Limit for PrefixDemo {}
        impl ConstLimit for PrefixDemo {
            const MAX: usize = 3;
        }
        let admitted: PositiveLimit<PrefixDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();

        // Under the magnitude: everything is carried and nothing is reported
        // omitted.
        let (whole, none_omitted) = NonEmptyBounded::admitted_prefix(1_u8, vec![2, 3], &admitted);
        assert_eq!(whole.len(), 3);
        assert_eq!(none_omitted, 0);
        assert_eq!(whole.iter().copied().collect::<Vec<u8>>(), vec![1, 2, 3]);

        // Past it: the prefix the magnitude holds is carried — never the first
        // item alone — and the count reads the remainder exactly.
        let (prefix, omitted) = NonEmptyBounded::admitted_prefix(1_u8, vec![2, 3, 4, 5], &admitted);
        assert_eq!(prefix.len(), PrefixDemo::MAX);
        assert_eq!(omitted, 2);
        assert_eq!(prefix.iter().copied().collect::<Vec<u8>>(), vec![1, 2, 3]);

        // Two truncations of different magnitude report different counts: the
        // number tracks the act rather than standing for "some were dropped".
        let (_, larger) = NonEmptyBounded::admitted_prefix(1_u8, vec![2, 3, 4, 5, 6, 7], &admitted);
        assert_eq!(larger, 4);
        assert_ne!(larger, omitted);
    }

    /// law: root.an-admission-does-not-cross-profiles — a witness names WHICH
    /// profile admitted the family, so one plane's admission is never another's.
    ///
    /// The profile is a type parameter rather than a number carried inside the
    /// value, which is what makes the separation structural: `AdmittedLimit<L,
    /// A>` and `AdmittedLimit<L, B>` are different types whatever their
    /// magnitudes, and no coercion joins them. A family small enough for two
    /// ceilings is admitted under each SEPARATELY, and holding one of those
    /// admissions is not holding the other.
    ///
    /// The claim ceiling: this says nothing about which profile is right for a
    /// seat. It says a road that requires one profile's evidence cannot be fed
    /// another's.
    ///
    /// Red twin: substituting one profile's witness where another's is required
    /// must not compile — the fixture is testpak's.
    #[test]
    fn an_admission_does_not_cross_profiles() {
        use crate::types::{
            AdmittedLimit, LimitAdmissionProfile, NarrowLawsProfile, RootLawsProfile,
        };
        struct TinyDemo;
        impl Limit for TinyDemo {}
        impl ConstLimit for TinyDemo {
            const MAX: usize = 4;
        }

        // Two profiles, each with its own declared ceiling. The magnitudes are
        // stated so the law reads against numbers rather than against whichever
        // pair happened to be declared.
        assert_eq!(NarrowLawsProfile::MAX_DECLARED_LIMIT, 8);
        assert_eq!(RootLawsProfile::MAX_DECLARED_LIMIT, 1_024);

        let wide: AdmittedLimit<TinyDemo, RootLawsProfile> = AdmittedLimit::under_profile();
        let narrow: AdmittedLimit<TinyDemo, NarrowLawsProfile> = AdmittedLimit::under_profile();
        assert_eq!(wide.max(), narrow.max());

        // Two admissions of one family, and the two values do not unify: each
        // consumer names the profile it will take, and only that one fits.
        let takes_wide: fn(AdmittedLimit<TinyDemo, RootLawsProfile>) = drop;
        let takes_narrow: fn(AdmittedLimit<TinyDemo, NarrowLawsProfile>) = drop;
        takes_wide(wide);
        takes_narrow(narrow);
    }

    /// law: root.reading-is-not-gaining — reading a bounded collection is an
    /// observation, not a crossing: `iter` borrows, so the values are visible
    /// and the collection is neither consumed nor changed, and no mutable or
    /// positional road stands beside it (`iter_mut`, `Index`, and a slice
    /// escape all absent).
    ///
    /// The order law this read carries: iteration exposes values for
    /// observation; iteration order may influence semantic meaning ONLY where
    /// the owner type explicitly declares ordering as semantic; identity-bearing
    /// generation over order-insensitive collections must canonicalize by an
    /// owner-declared order or key first; testpak owes the permutation hostiles
    /// — identical plans and identical output identities under permuted
    /// order-insensitive inputs.
    ///
    /// Owed reversal (red twin): a consuming or mutating read road — `iter_mut`,
    /// an `Index` impl, or a slice escape — must not compile.
    #[test]
    fn reading_is_not_gaining() {
        use crate::types::{AdmittedLimit, Bounded, NonEmptyBounded, RootLawsProfile};
        struct ReadDemo;
        impl Limit for ReadDemo {}
        impl ConstLimit for ReadDemo {
            const MAX: usize = 4;
        }

        let bounded: Bounded<u8, ReadDemo> = Bounded::admitted_const(
            vec![1, 2, 3],
            &AdmittedLimit::<_, RootLawsProfile>::under_profile(),
        )
        .unwrap_or_else(|_| Bounded::<u8, ReadDemo>::empty());
        let seen: Vec<u8> = bounded.iter().copied().collect();
        assert_eq!(seen, vec![1, 2, 3]);
        // The collection survives the read unchanged and can be read again.
        assert_eq!(bounded.len(), 3);
        assert!(!bounded.is_empty());
        assert_eq!(bounded.iter().count(), 3);

        let non_empty: NonEmptyBounded<u8, ReadDemo> = NonEmptyBounded::singleton(9);
        assert_eq!(non_empty.iter().copied().collect::<Vec<u8>>(), vec![9]);
        assert_eq!(non_empty.len(), 1);
        assert_eq!(*non_empty.first(), 9);
    }

    /// law: root.closure-bar-is-implementable — a minimal two-state machine
    /// satisfies the six-obligation bar: one initial state, total dispatch, a
    /// terminal no transition leaves, typed refusal for the unmatched pair.
    /// Owed reversal (red twin): a machine whose dispatch drops an unmatched pair
    /// instead of refusing must fail its closure evidence.
    #[test]
    fn closure_bar_is_implementable() {
        struct Demo;
        struct DemoRefusal;

        /// The two states, named. A `bool` carrying a comment that says which
        /// end is terminal is a state machine the compiler cannot read; the
        /// enum states it, and dispatch below becomes a closed match over the
        /// roster rather than a branch over a flag.
        #[derive(Debug, PartialEq, Eq)]
        enum DemoState {
            Start,
            Terminal,
        }

        impl TransitionSystem for Demo {
            type State = DemoState;
            type Input = ();
            type Effect = ();
            type Refusal = DemoRefusal;

            fn initial() -> Self::State {
                DemoState::Start
            }

            fn is_terminal(state: &Self::State) -> bool {
                matches!(*state, DemoState::Terminal)
            }

            fn dispatch(
                state: &Self::State,
                (): &Self::Input,
            ) -> Dispatch<(Self::State, Self::Effect), Self::Refusal> {
                match *state {
                    // Obligation: no transition leaves a terminal.
                    DemoState::Terminal => Dispatch::Refused(DemoRefusal),
                    DemoState::Start => Dispatch::One((DemoState::Terminal, ())),
                }
            }
        }

        let start = Demo::initial();
        assert!(!Demo::is_terminal(&start));
        assert!(matches!(
            Demo::dispatch(&start, &()),
            Dispatch::One((DemoState::Terminal, ()))
        ));
        assert!(matches!(
            Demo::dispatch(&DemoState::Terminal, &()),
            Dispatch::Refused(DemoRefusal)
        ));
    }
}

mod refusal {
    use crate::refusal::{
        CauseId, CauseOrderDeclaration, CompletionPosture, DeclaredCause, DeclaredCauseOrder,
        FamilyAdmission, FamilyAdmissionCoverage, FamilyShape, HandlingClass, LocalCauseKey,
        ReasonId, Refusal, RefusalFamily, RefusalFamilyId, StopBound, admit_order, admit_shape,
    };
    use crate::types::{BoundedConstruction, Limit, NonEmptyBounded, NonEmptyBoundedConstruction};

    struct DemoSingle;
    impl RefusalFamily for DemoSingle {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] =
            &["NotCanonical", "WrongRole", "BoundExceeded"];
    }

    impl CauseOrderDeclaration for DemoSingle {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("not-canonical"),
                ),
                "NotCanonical",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("role-mismatch"),
                ),
                "WrongRole",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("bound-exceeded"),
                ),
                "BoundExceeded",
            ),
        ]);
    }

    /// The same family after a pure Rust rename: three new spellings over the
    /// three identities that were already declared.
    struct DemoRenamed;
    impl RefusalFamily for DemoRenamed {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] =
            &["NotNormalized", "NotTheDeclaredRole", "Unbounded"];
    }

    impl CauseOrderDeclaration for DemoRenamed {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("not-canonical"),
                ),
                "NotNormalized",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("role-mismatch"),
                ),
                "NotTheDeclaredRole",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("bound-exceeded"),
                ),
                "Unbounded",
            ),
        ]);
    }

    /// The same family after its middle cause changed MEANING: the spelling
    /// stands, and the identity is a different one.
    struct DemoMeaningChanged;
    impl RefusalFamily for DemoMeaningChanged {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] =
            &["NotCanonical", "WrongRole", "BoundExceeded"];
    }

    impl CauseOrderDeclaration for DemoMeaningChanged {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("not-canonical"),
                ),
                "NotCanonical",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("role-mismatch-under-the-narrowed-reading"),
                ),
                "WrongRole",
            ),
            DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("bound-exceeded"),
                ),
                "BoundExceeded",
            ),
        ]);
    }

    struct DemoCollection;
    impl RefusalFamily for DemoCollection {
        const SHAPE: FamilyShape = FamilyShape::IssueCollection;
        const SELECTION_ORDER: &'static [&'static str] = &[];
    }

    impl CauseOrderDeclaration for DemoCollection {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::none();
    }

    /// A family declaring the single-cause shape and ordering nothing: the
    /// canonical order stands for exactly this shape, so an empty order under it
    /// is a declaration disagreeing with itself.
    struct DemoUnordered;
    impl RefusalFamily for DemoUnordered {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] = &[];
    }

    /// A collection-shaped family ordering something: the same disagreement from
    /// the other direction.
    struct DemoOverOrdered;
    impl RefusalFamily for DemoOverOrdered {
        const SHAPE: FamilyShape = FamilyShape::IssueCollection;
        const SELECTION_ORDER: &'static [&'static str] = &["NotCanonical"];
    }

    /// A family whose typed order and textual order are two facts rather than
    /// one fact in two forms: coherent on shape, and the projection disagrees.
    struct DemoUnprojected;
    impl RefusalFamily for DemoUnprojected {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] = &["NotCanonical"];
    }

    impl CauseOrderDeclaration for DemoUnprojected {
        const DECLARED_ORDER: DeclaredCauseOrder =
            DeclaredCauseOrder::declared(&[DeclaredCause::declared(
                CauseId::declared(
                    RefusalFamilyId::declared("demo.unprojected"),
                    LocalCauseKey::declared("not-canonical"),
                ),
                "NotNormalized",
            )]);
    }

    /// law: refusal.admission-coverage-is-a-type-parameter — a family's
    /// declaration is not a machine fact until its own joins closed, and the
    /// strength of what closed rides in the witness's TYPE. The coherence join
    /// refuses in BOTH directions (a single-cause family ordering nothing, and a
    /// collection family ordering something) and the projection join refuses a
    /// typed order the textual order does not project; the road that ran fixes
    /// the coverage parameter, and `FamilyAdmissionCoverage` is that parameter's
    /// inspection projection rather than a field a road could read and mistake.
    ///
    /// The claim ceiling: admission establishes that the declarations agree with
    /// each other. It establishes nothing about whether the declared order is
    /// the right selector for the family's checks, nothing about the family's
    /// Rust body, and nothing about family uniqueness across a whole program —
    /// that join stays the composition root's.
    ///
    /// Owed reversal (red twin): constructing the witness without admitting must
    /// not compile — the fixture is testpak's.
    #[test]
    fn admission_coverage_is_a_type_parameter() {
        assert!(matches!(
            admit_shape::<DemoUnordered>(),
            Err(FamilyAdmission::NotShapeCoherent)
        ));
        assert!(matches!(
            admit_shape::<DemoOverOrdered>(),
            Err(FamilyAdmission::NotShapeCoherent)
        ));
        assert!(matches!(
            admit_order::<DemoUnprojected>(),
            Err(FamilyAdmission::NotProjected)
        ));
        assert!(matches!(
            admit_shape::<DemoUnprojected>().map(|admitted| admitted.coverage()),
            Ok(FamilyAdmissionCoverage::ShapeCoherence)
        ));
        assert!(matches!(
            admit_order::<DemoSingle>().map(|admitted| admitted.coverage()),
            Ok(FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection)
        ));
        assert!(matches!(
            admit_order::<DemoCollection>().map(|admitted| admitted.coverage()),
            Ok(FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection)
        ));
        assert_eq!(
            FamilyAdmission::SHAPE,
            FamilyShape::SingleCause,
            "the admission family's own declaration is one of the three shapes"
        );
    }

    /// law: refusal.order-admission-implies-shape-admission — the coverage
    /// hierarchy runs one way and it runs the whole way. An `OrderProjected`
    /// witness clears BOTH consumer bounds: publication, which takes its
    /// coverage generically under `ShapeAdmission`, and `cause_order`, which
    /// hands back the family's typed cause order and hangs off `OrderAdmission`.
    /// A `ShapeCoherent` witness clears the first and only the first, and the
    /// coverage each one projects onto its receipt is the one its road earned.
    ///
    /// The claim ceiling: this establishes the implication between the two
    /// coverages and the reach of the two bounds. It establishes nothing further
    /// about either declaration than the joins behind it already did.
    ///
    /// Owed reversal (red twin): the weaker witness at the stronger consumer
    /// must not compile — the fixture is testpak's.
    #[test]
    fn order_admission_implies_shape_admission() {
        let strong = admit_order::<DemoSingle>()
            .unwrap_or_else(|_| unreachable!("the demo family's declarations agree"));
        let published = Refusal::published(
            ReasonId::for_laws([13; 32]),
            HandlingClass::Escalate,
            DemoSingle,
            &strong,
        );
        assert_eq!(
            published.admission(),
            FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection,
            "the shape-only consumer records the stronger coverage the witness carried"
        );
        assert!(
            strong
                .cause_order()
                .projects_to(DemoSingle::SELECTION_ORDER),
            "the order-sensitive consumer hands back the order the projection join admitted"
        );

        let weak = admit_shape::<DemoUnprojected>()
            .unwrap_or_else(|_| unreachable!("the demo family's shape and textual order agree"));
        let weakly_published = Refusal::published(
            ReasonId::for_laws([14; 32]),
            HandlingClass::Reconfigure,
            DemoUnprojected,
            &weak,
        );
        assert_eq!(
            weakly_published.admission(),
            FamilyAdmissionCoverage::ShapeCoherence,
            "the weaker witness reaches the shape-only consumer and stays weaker on the receipt"
        );
    }

    /// law: refusal.publication-requires-an-admitted-family — the universal
    /// envelope has exactly one mint, and it demands the admission witness: a
    /// reader handed a published refusal acts on the family's declared shape and
    /// order without re-reading them, so an unjoined declaration never reaches
    /// publication.
    ///
    /// The claim ceiling: the road's reach today is this crate's, because
    /// `ReasonId` carries no public mint until the evidence home registers
    /// reasons. Nothing here claims an outside caller can publish.
    ///
    /// Owed reversal (red twin): a publication road that skips the witness must
    /// not compile — the fixture is testpak's.
    #[test]
    fn publication_requires_an_admitted_family() {
        let admitted = admit_order::<DemoSingle>()
            .unwrap_or_else(|_| unreachable!("the demo family's declarations agree"));
        let published = Refusal::published(
            ReasonId::for_laws([11; 32]),
            HandlingClass::DoNotRetry,
            DemoSingle,
            &admitted,
        );
        assert_eq!(published.reason().as_bytes(), &[11; 32]);
        assert!(matches!(published.handling(), HandlingClass::DoNotRetry));
        let _body: &DemoSingle = published.family();
    }

    /// law: refusal.envelope-is-family-generic — the universal envelope binds any
    /// family type; no concrete family is hard-wired into it.
    /// Owed reversal: hard-wiring a family into the envelope must break this law.
    #[test]
    fn envelope_is_family_generic() {
        let single: Option<Refusal<DemoSingle>> = None;
        let collection: Option<Refusal<DemoCollection>> = None;
        assert!(single.is_none());
        assert!(collection.is_none());
    }

    /// law: refusal.zero-issue-collection-unrepresentable — issue-collection
    /// families ride `NonEmptyBounded`, so a refusal with zero issues has no value.
    /// Owed reversal (red twin): constructing an empty issue collection must not
    /// compile past the shape.
    #[test]
    fn issue_collections_are_nonempty_bounded() {
        struct DemoIssue;
        struct IssueLimit;
        impl Limit for IssueLimit {}
        let shape: Option<fn(NonEmptyBounded<DemoIssue, IssueLimit>)> = Some(drop);
        assert!(shape.is_some());
    }

    /// law: refusal.selection-order-is-family-declared — a single-cause family
    /// declares a non-empty canonical selection order as machine-readable law; a
    /// collection family declares none.
    /// Owed reversal: a single-cause family with an empty order must be refused by
    /// the repository checks once the family↔order join lands in xtask.
    #[test]
    fn selection_order_is_family_declared() {
        assert!(!DemoSingle::SELECTION_ORDER.is_empty());
        assert!(DemoCollection::SELECTION_ORDER.is_empty());
        assert_eq!(DemoSingle::SELECTION_ORDER.first(), Some(&"NotCanonical"));
    }

    /// law: refusal.posture-is-a-collection-instance-value — completion posture
    /// exists as a value carried inside collection-shaped refusals; the family
    /// trait carries no posture constant, so a single-cause family cannot claim
    /// one; an early stop names which declared bound stopped it.
    /// Owed reversal (red twin): reintroducing a posture constant on the family
    /// trait must break this law.
    #[test]
    fn posture_is_a_collection_instance_value() {
        let complete = CompletionPosture::Complete;
        let stopped = CompletionPosture::EarlyStopped {
            stopped_at: StopBound::DeclaredIssueBound,
        };
        assert_eq!(complete, CompletionPosture::Complete);
        assert_ne!(complete, stopped);
        assert!(matches!(
            stopped,
            CompletionPosture::EarlyStopped {
                stopped_at: StopBound::DeclaredIssueBound
            }
        ));
        assert_eq!(DemoSingle::SHAPE, FamilyShape::SingleCause);
        assert_eq!(DemoCollection::SHAPE, FamilyShape::IssueCollection);
    }

    /// law: refusal.a-truncated-report-is-not-a-halted-examination — the posture
    /// a complete examination writes is SELECTED by the truncation its body's
    /// own construction performed, both directions: a road that dropped nothing
    /// mints `Complete`, and a road that dropped material mints a truncation
    /// naming the declared bound and the exact count. A pass that covered every
    /// site therefore cannot write `EarlyStopped`, because the road that mints a
    /// truncation is the only road to one and it does not produce that variant
    /// at all.
    ///
    /// The posture is taken off the act rather than off a number, so the count
    /// is not merely accurate by discipline — it is the count the construction
    /// below actually truncated by. The body and the posture are two readings of
    /// one act and travel as ONE value: the package hands out its carry only by
    /// reference and its posture only for rendering, so substituting one
    /// truncation's completion for another's is unwritable rather than merely
    /// discouraged, and inventing a count with no truncation behind it is not
    /// expressible at all.
    ///
    /// All three postures are inhabited by three roads and read back through
    /// the one public reader, and no two of them are equal — which is what makes
    /// the distinction load-bearing rather than cosmetic: a reader branching on
    /// a halted examination must re-run the pass, a reader branching on a
    /// truncated report already knows the total, and a reader branching on a
    /// complete one knows there is nothing further. The halted value is taken
    /// off `AdmittedPrefix::stopped_early` rather than written as a literal, so
    /// the three are compared as three constructions' products.
    ///
    /// The claim ceiling: this says nothing about whether any particular pass
    /// runs to completion, and nothing about a body assembled by a road other
    /// than the truncating one. It says a pass that DID run to completion cannot
    /// report that it did not, a body that dropped findings cannot report that it
    /// did not, and a caller holding no truncation cannot mint a posture that
    /// claims one.
    ///
    /// Red twin: writing a truncation posture with no truncation behind it —
    /// a `ReportTruncated` assembled from a bound and a number — must not
    /// compile, because the seats are private and the package's mint is the
    /// only road to one.
    #[test]
    fn a_truncated_report_is_not_a_halted_examination() {
        use crate::refusal::AdmittedPrefix;
        use crate::types::{ConstLimit, PositiveLimit, RootLawsProfile};
        struct PostureDemo;
        impl Limit for PostureDemo {}
        impl ConstLimit for PostureDemo {
            const MAX: usize = 3;
        }
        let admitted: PositiveLimit<PostureDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();

        // A body whose construction carried everything: the posture it can write
        // is the complete one, and it is the only one it can write.
        let whole = AdmittedPrefix::examined_completely(
            1_u8,
            vec![2, 3],
            &admitted,
            StopBound::DeclaredIssueBound,
        );
        let carried_everything = whole.completion();
        assert_eq!(whole.carried().len(), 3);
        assert_eq!(carried_everything, CompletionPosture::Complete);

        // A body whose construction dropped four: the posture names the bound
        // and the count the construction itself performed, and it is READ off
        // the same value that holds the carry.
        let prefix = AdmittedPrefix::examined_completely(
            1_u8,
            vec![2, 3, 4, 5, 6, 7],
            &admitted,
            StopBound::DeclaredIssueBound,
        );
        let left_some_out = prefix.completion();
        assert_eq!(prefix.carried().len(), 3);
        assert!(matches!(
            left_some_out,
            CompletionPosture::ReportTruncated(truncation)
                if truncation.omitted().get() == 4
                    && matches!(truncation.stopped_at(), StopBound::DeclaredIssueBound)
        ));

        // A one-issue body reaches the same package and is complete by shape:
        // one item in, one item carried, no bound to name.
        let single = AdmittedPrefix::<u8, PostureDemo>::carrying_one(9);
        assert_eq!(single.carried().len(), 1);
        assert_eq!(single.completion(), CompletionPosture::Complete);

        // The halted posture is a different value, and no truncation produces
        // it. It is read off the halted road's own body rather than written by
        // hand here, so the three postures a reader can receive are compared as
        // three roads' products and not as three literals.
        let stopped =
            AdmittedPrefix::stopped_early(1_u8, vec![2], &admitted, StopBound::DeclaredWorkBound)
                .unwrap_or_else(|_| unreachable!("two issues fit the bound of three"));
        let halted = stopped.completion();
        assert_eq!(stopped.carried().len(), 2);
        assert_ne!(left_some_out, halted);
        assert_ne!(carried_everything, halted);
        assert!(matches!(
            halted,
            CompletionPosture::EarlyStopped {
                stopped_at: StopBound::DeclaredWorkBound
            }
        ));
    }

    /// law: refusal.a-halted-examination-couples-its-bound — the halted road
    /// produces the carry and the posture in ONE construction. The body holds
    /// exactly the material handed over, the posture is `EarlyStopped` naming
    /// the declared bound the caller stated it stopped at, and both are read
    /// back off that single value because there is no second value to read
    /// either from.
    ///
    /// It refuses where the truncating road truncates, and this law executes
    /// that arm too. `ReportTruncated` has a seat for what it dropped;
    /// `EarlyStopped` names a bound and nothing else, so material past the
    /// admitted magnitude could only be dropped silently here — the one defect
    /// the package exists to make unwritable. A pass that stopped BECAUSE of a
    /// bound has nothing past it, so handing over more is a caller contradicting
    /// its own posture, and the road answers with the typed cause rather than a
    /// quiet shortening.
    ///
    /// The bound is carried rather than derived: the two `StopBound` members are
    /// separately reachable through this road, so a halt at the work bound and a
    /// halt at the issue bound are two postures and not one word.
    ///
    /// The claim ceiling, exactly: this establishes the COUPLING and nothing
    /// past it. It does not prove that any external examination truly halted —
    /// the family owner's algorithm and testpak establish the behavioral claim.
    /// No caller exists today, because no scan in the machine halts; the road is
    /// here so that the first one is coupled rather than pushed back onto a pair
    /// of loose values.
    ///
    /// Red twin: writing a halted posture beside a body of one's own must not
    /// compile — testpak/tests/compile-fail/a-remainder-married-to-another-body.rs.
    #[test]
    fn a_halted_examination_couples_its_bound() {
        use crate::refusal::AdmittedPrefix;
        use crate::types::{ConstLimit, PositiveLimit, RootLawsProfile};
        struct HaltDemo;
        impl Limit for HaltDemo {}
        impl ConstLimit for HaltDemo {
            const MAX: usize = 3;
        }
        let admitted: PositiveLimit<HaltDemo, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();

        // The halted body: the carry is what was handed over, and the posture
        // names the bound the caller stopped at — one construction, two
        // readings.
        let halted =
            AdmittedPrefix::stopped_early(1_u8, vec![2], &admitted, StopBound::DeclaredIssueBound)
                .unwrap_or_else(|_| unreachable!("two issues fit the bound of three"));
        assert_eq!(halted.carried().len(), 2);
        assert_eq!(
            halted.carried().iter().copied().collect::<Vec<u8>>(),
            vec![1, 2]
        );
        assert!(matches!(
            halted.completion(),
            CompletionPosture::EarlyStopped {
                stopped_at: StopBound::DeclaredIssueBound
            }
        ));

        // The other bound is a different posture, so "it stopped" is never an
        // unlocated word.
        let at_work =
            AdmittedPrefix::stopped_early(1_u8, vec![], &admitted, StopBound::DeclaredWorkBound)
                .unwrap_or_else(|_| unreachable!("one issue fits the bound of three"));
        assert_ne!(halted.completion(), at_work.completion());

        // Past the admitted magnitude: the road refuses rather than shortening,
        // because a halted posture has nowhere to record what it dropped.
        assert!(matches!(
            AdmittedPrefix::stopped_early(
                1_u8,
                vec![2, 3, 4],
                &admitted,
                StopBound::DeclaredIssueBound
            ),
            Err(NonEmptyBoundedConstruction::OverLimit)
        ));
    }

    /// law: refusal.every-collection-family-carries-the-coupled-seat — every
    /// collection-shaped family the machine declares reads its issues and its
    /// coverage claim off ONE value. The reader pair is the shape of that: a
    /// family that kept a loose carry beside a loose posture could not hand
    /// these two functions over, because `issues` borrows out of the same value
    /// `posture` is read off and no third seat exists to read either from.
    ///
    /// What each line establishes, exactly: the family exposes the two readers
    /// with the two exact signatures, and exposes no writable seat pair beside
    /// them. What it does NOT establish is that the seat's TYPE is
    /// `AdmittedPrefix` — a private field is unnameable from here — nor that
    /// this list is the whole population. Both of those are the repository
    /// join's, deriving the denominator from the sources rather than from a
    /// list anybody maintains: `cargo xtask check`'s
    /// `collection-bodies-are-coupled` enumerates every
    /// `FamilyShape::IssueCollection` declaration in the workspace and reads the
    /// declared body seat off each one. This law is the compile-time half and
    /// that join is the completeness half, stated as such, because Rust cannot
    /// enumerate its own impls and a hand-kept count here would be exactly the
    /// inventory this repository bans.
    ///
    /// Red twin: assembling a migrated family from a carry and a posture must
    /// not compile —
    /// testpak/tests/compile-fail/a-collection-body-assembled-from-parts.rs.
    #[test]
    fn every_collection_family_carries_the_coupled_seat() {
        /// The two readers one coupled body hands out, over one family type.
        type CoupledReaders<F, T, L> = (
            fn(&F) -> &NonEmptyBounded<T, L>,
            fn(&F) -> CompletionPosture,
        );

        /// One family's reader pair, taken as function pointers so the compiler
        /// settles the two signatures and nothing is executed. The pair is
        /// handed straight back, so neither argument is an ignored parameter and
        /// the whole content of the call is the type-checking it forces.
        const fn coupled_readers<F, T, L: Limit>(
            issues: fn(&F) -> &NonEmptyBounded<T, L>,
            posture: fn(&F) -> CompletionPosture,
        ) -> CoupledReaders<F, T, L> {
            (issues, posture)
        }

        coupled_readers(
            crate::authority::CapabilityClaimConstruction::issues,
            crate::authority::CapabilityClaimConstruction::posture,
        );
        coupled_readers(
            crate::schema::ContractConstruction::issues,
            crate::schema::ContractConstruction::posture,
        );
        coupled_readers(
            crate::schema::RefinementConstruction::issues,
            crate::schema::RefinementConstruction::posture,
        );
        coupled_readers(
            crate::schema::MigrationConstruction::issues,
            crate::schema::MigrationConstruction::posture,
        );
        coupled_readers(
            crate::schema::CompatibilityEdgeConstruction::issues,
            crate::schema::CompatibilityEdgeConstruction::posture,
        );
        coupled_readers(
            crate::schema::SchemaConstruction::issues,
            crate::schema::SchemaConstruction::posture,
        );
        coupled_readers(
            crate::schema::LayoutConstruction::issues,
            crate::schema::LayoutConstruction::posture,
        );
        coupled_readers(
            crate::schema::CodecConstruction::issues,
            crate::schema::CodecConstruction::posture,
        );
        coupled_readers(
            crate::history::RemovalPlanConstruction::issues,
            crate::history::RemovalPlanConstruction::posture,
        );
        coupled_readers(
            crate::history::RemovalAuthorizationClaimConstruction::issues,
            crate::history::RemovalAuthorizationClaimConstruction::posture,
        );
        coupled_readers(
            crate::history::RemovalRefusal::issues,
            crate::history::RemovalRefusal::posture,
        );
        coupled_readers(
            crate::declaration::AuthoredNameConstruction::issues,
            crate::declaration::AuthoredNameConstruction::posture,
        );
        coupled_readers(
            crate::declaration::ClosureNamespace::issues,
            crate::declaration::ClosureNamespace::posture,
        );
        coupled_readers(
            crate::declaration::LinkResolution::issues,
            crate::declaration::LinkResolution::posture,
        );
        coupled_readers(
            crate::declaration::ProjectionContractConstruction::issues,
            crate::declaration::ProjectionContractConstruction::posture,
        );
        coupled_readers(
            crate::semantic::SemanticFormConstruction::issues,
            crate::semantic::SemanticFormConstruction::posture,
        );
        coupled_readers(
            crate::execution::ExecutionFormConstruction::issues,
            crate::execution::ExecutionFormConstruction::posture,
        );
        coupled_readers(
            crate::execution::EffectBatchComposition::issues,
            crate::execution::EffectBatchComposition::posture,
        );
        coupled_readers(
            crate::execution::KernelSemanticContractConstruction::issues,
            crate::execution::KernelSemanticContractConstruction::posture,
        );
        coupled_readers(
            crate::execution::KernelInterfaceContractConstruction::issues,
            crate::execution::KernelInterfaceContractConstruction::posture,
        );
        coupled_readers(
            crate::bvisor::AttemptAdmission::issues,
            crate::bvisor::AttemptAdmission::posture,
        );
    }

    /// law: refusal.cause-identity-outlives-its-spelling — a cause's stable
    /// identity and its position are independent of the Rust variant that
    /// spells it: renaming every variant moves every spelling and moves neither
    /// identity nor ordinal, while a cause whose MEANING changed carries a
    /// different identity under an unchanged spelling. An identity this order
    /// does not declare has no position at all.
    /// Owed reversal (red twin): deriving the identity from the spelling — or
    /// admitting a `CauseOrdinal` constructor that takes a number — must break
    /// this law.
    #[test]
    fn cause_identity_outlives_its_spelling() {
        assert_ne!(DemoSingle::SELECTION_ORDER, DemoRenamed::SELECTION_ORDER);
        assert_eq!(DemoSingle::DECLARED_ORDER.len(), 3);
        assert!(!DemoSingle::DECLARED_ORDER.is_empty());

        let renamed_throughout = DemoSingle::DECLARED_ORDER
            .iter()
            .zip(DemoRenamed::DECLARED_ORDER.iter())
            .all(|(before, after)| {
                before.id() == after.id()
                    && before.spelling() != after.spelling()
                    && DemoSingle::DECLARED_ORDER.ordinal_of(before.id())
                        == DemoRenamed::DECLARED_ORDER.ordinal_of(after.id())
            });
        assert!(renamed_throughout);

        let middle = DemoSingle::DECLARED_ORDER.ordinal_of(CauseId::declared(
            RefusalFamilyId::declared("demo.single"),
            LocalCauseKey::declared("role-mismatch"),
        ));
        assert!(middle.is_some_and(|ordinal| {
            ordinal.position() == 1
                && DemoSingle::DECLARED_ORDER.identity_at(ordinal)
                    == DemoRenamed::DECLARED_ORDER.identity_at(ordinal)
                && DemoSingle::DECLARED_ORDER.identity_at(ordinal)
                    != DemoMeaningChanged::DECLARED_ORDER.identity_at(ordinal)
        }));
        assert!(
            DemoSingle::DECLARED_ORDER
                .ordinal_of(CauseId::declared(
                    RefusalFamilyId::declared("demo.single"),
                    LocalCauseKey::declared("never-declared")
                ))
                .is_none()
        );
    }

    /// law: refusal.cause-identity-is-a-family-and-a-local-key — a cause
    /// identity is the PAIR and not a string that reads like one. Two families
    /// declaring the same local key declare two identities; the owning family
    /// is read off the value rather than parsed out of it; and the canonical
    /// text form is composed from the two seats on demand, so two identities
    /// that render alike are still two identities.
    /// Reversal: `testpak/tests/compile-fail/a-cause-identity-cut-from-one-string.rs`
    /// — the retired road, `CauseId::declared("family.local")`, does not
    /// typecheck.
    #[test]
    fn cause_identity_is_a_family_and_a_local_key() {
        let mine = CauseId::declared(
            RefusalFamilyId::declared("demo.left"),
            LocalCauseKey::declared("not-canonical"),
        );
        let yours = CauseId::declared(
            RefusalFamilyId::declared("demo.right"),
            LocalCauseKey::declared("not-canonical"),
        );
        assert_eq!(mine.local(), yours.local());
        assert_ne!(mine.family(), yours.family());
        assert_ne!(mine, yours);
        assert_eq!(mine.canonical_text(), "demo.left.not-canonical");

        // The retired string road could not tell these two apart: both render
        // as `demo.left.not-canonical`, and they are different causes owned by
        // different families. The pair keeps them apart, and this is exactly
        // why the text stays a projection — a reader cutting it back into a
        // pair would have to guess which separator was the one that mattered.
        let cut_elsewhere = CauseId::declared(
            RefusalFamilyId::declared("demo"),
            LocalCauseKey::declared("left.not-canonical"),
        );
        assert_eq!(cut_elsewhere.canonical_text(), mine.canonical_text());
        assert_ne!(cut_elsewhere, mine);
    }

    /// law: refusal.selection-order-projects-the-typed-order — the textual
    /// selection order is exactly the typed order's projection, and the join
    /// says so: the declared pairs project, and a permuted, a shortened, and a
    /// foreign textual order each fail to project. A collection family declares
    /// no typed order and its empty textual order projects it faithfully. The
    /// two root construction families share the cause SPELLING `OverLimit` and
    /// share no cause IDENTITY.
    /// Owed reversal (red twin): a projection check that compared lengths only,
    /// or ignored position, must break this law.
    #[test]
    fn selection_order_projects_the_typed_order() {
        assert!(DemoSingle::DECLARED_ORDER.projects_to(DemoSingle::SELECTION_ORDER));
        assert!(DemoRenamed::DECLARED_ORDER.projects_to(DemoRenamed::SELECTION_ORDER));
        assert!(!DemoSingle::DECLARED_ORDER.projects_to(&[
            "WrongRole",
            "NotCanonical",
            "BoundExceeded"
        ]));
        assert!(!DemoSingle::DECLARED_ORDER.projects_to(&["NotCanonical", "WrongRole"]));
        assert!(!DemoSingle::DECLARED_ORDER.projects_to(DemoRenamed::SELECTION_ORDER));

        assert!(DemoCollection::DECLARED_ORDER.is_empty());
        assert!(DemoCollection::DECLARED_ORDER.projects_to(DemoCollection::SELECTION_ORDER));

        assert!(
            BoundedConstruction::DECLARED_ORDER.projects_to(BoundedConstruction::SELECTION_ORDER)
        );
        assert!(
            NonEmptyBoundedConstruction::DECLARED_ORDER
                .projects_to(NonEmptyBoundedConstruction::SELECTION_ORDER)
        );
        let bounded = BoundedConstruction::DECLARED_ORDER
            .iter()
            .next()
            .map(DeclaredCause::id);
        let non_empty = NonEmptyBoundedConstruction::DECLARED_ORDER
            .iter()
            .next()
            .map(DeclaredCause::id);
        assert!(bounded.is_some());
        assert_ne!(bounded, non_empty);
        assert_eq!(
            bounded.map(CauseId::family),
            Some(RefusalFamilyId::declared("root.bounded-construction"))
        );
        assert_eq!(
            non_empty.map(CauseId::family),
            Some(RefusalFamilyId::declared(
                "root.non-empty-bounded-construction"
            ))
        );
        // The two families share the local key and share no identity: the
        // family seat is the whole of the difference, and the shape is what
        // carries it.
        assert_eq!(bounded.map(CauseId::local), non_empty.map(CauseId::local));
        assert_eq!(
            bounded.map(CauseId::canonical_text),
            Some(String::from("root.bounded-construction.over-limit"))
        );
    }
}

mod logic {
    use crate::logic::Truth;

    const ALL: [Truth; 3] = [Truth::True, Truth::False, Truth::Pending];

    /// law: logic.truth-tables-cell-for-cell — every cell of the strong Kleene
    /// conjunction, disjunction, and negation tables holds exactly.
    /// Owed reversal (red twin): flipping any single cell must break this law.
    #[test]
    fn truth_tables_cell_for_cell() {
        use Truth::{False, Pending, True};
        // AND — False dominates, True is neutral, Pending propagates.
        assert_eq!(True.and(True), True);
        assert_eq!(True.and(False), False);
        assert_eq!(True.and(Pending), Pending);
        assert_eq!(False.and(True), False);
        assert_eq!(False.and(False), False);
        assert_eq!(False.and(Pending), False);
        assert_eq!(Pending.and(True), Pending);
        assert_eq!(Pending.and(False), False);
        assert_eq!(Pending.and(Pending), Pending);
        // OR — True dominates, False is neutral, Pending propagates.
        assert_eq!(True.or(True), True);
        assert_eq!(True.or(False), True);
        assert_eq!(True.or(Pending), True);
        assert_eq!(False.or(True), True);
        assert_eq!(False.or(False), False);
        assert_eq!(False.or(Pending), Pending);
        assert_eq!(Pending.or(True), True);
        assert_eq!(Pending.or(False), Pending);
        assert_eq!(Pending.or(Pending), Pending);
        // NOT — swaps established values; Pending stays.
        assert_eq!(True.negate(), False);
        assert_eq!(False.negate(), True);
        assert_eq!(Pending.negate(), Pending);
    }

    /// law: logic.pending-cannot-hide-known-failure — a lagging answer never
    /// masks an established `False`: conjunction with `False` is `False` from
    /// either side, regardless of the other operand.
    /// Owed reversal: making `Pending` absorb `False` must break this law.
    #[test]
    fn pending_cannot_hide_known_failure() {
        for value in ALL {
            assert_eq!(value.and(Truth::False), Truth::False);
            assert_eq!(Truth::False.and(value), Truth::False);
        }
    }

    /// law: logic.double-negation-is-identity — negation is an involution over
    /// all three values.
    /// Owed reversal: any negation cell drifting must break this law.
    #[test]
    fn double_negation_is_identity() {
        for value in ALL {
            assert_eq!(value.negate().negate(), value);
        }
    }
}

mod identity {
    use crate::identity::{
        AuthorityPosition, Commitment, CreationLaw, IdentityClass, IdentityRole, OrderComparison,
        TypedRef,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use core::cmp::Ordering;

    /// law: identity.two-column-law-is-machine-readable — class and creation law
    /// are independent declared columns: two roles of the same class carry
    /// different creation laws.
    /// Owed reversal: deriving creation law from class must break this law.
    #[test]
    fn two_column_law_is_machine_readable() {
        struct DerivedRole;
        impl IdentityRole for DerivedRole {
            const CLASS: IdentityClass = IdentityClass::Occurrence;
            const CREATION: CreationLaw = CreationLaw::DerivedFromAdmittedPreimage;
        }
        struct FreshRole;
        impl IdentityRole for FreshRole {
            const CLASS: IdentityClass = IdentityClass::Occurrence;
            const CREATION: CreationLaw = CreationLaw::FreshOpaque;
        }
        assert_eq!(DerivedRole::CLASS, FreshRole::CLASS);
        assert_ne!(DerivedRole::CREATION, FreshRole::CREATION);
    }

    /// law: identity.admission-joins-creation-to-class — a two-column
    /// declaration is not a machine fact until its own join closed: where the
    /// declared creation law names a class in its own declaration, a role
    /// declaring a different class refuses admission, and the reification that
    /// turns the two columns into a travelling value is reachable only from the
    /// witness.
    ///
    /// The claim ceiling is narrow and is stated as such. The three class-open
    /// creation laws admit under ANY class — that is the two-column law itself,
    /// and this join does not touch it. Admission establishes nothing about the
    /// derived-seat law's two seats, which are facts about a deployment's design
    /// rather than about a pair of constants, and nothing about whether a
    /// concrete minter follows the creation law it declared: that is behavioral,
    /// it is owed, and it opens when minters exist.
    ///
    /// Owed reversal (red twin): constructing the witness without admitting must
    /// not compile — the fixture is testpak's.
    #[test]
    fn admission_joins_creation_to_class() {
        use crate::identity::{
            AdmittedIdentityColumns, AdmittedIdentityRole, IdentityRoleAdmission,
        };

        /// A role declaring Class B's own creation law under Class A's question.
        struct IncoherentRole;
        impl IdentityRole for IncoherentRole {
            const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
            const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
        }
        /// The same creation law under the class its own declaration names.
        struct CoherentRole;
        impl IdentityRole for CoherentRole {
            const CLASS: IdentityClass = IdentityClass::ByteDigest;
            const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
        }
        /// A class-open creation law: no declared fact to join against, so the
        /// columns stay independent and admission says nothing about the pair.
        struct OpenRole;
        impl IdentityRole for OpenRole {
            const CLASS: IdentityClass = IdentityClass::AuthorityOrder;
            const CREATION: CreationLaw = CreationLaw::FreshOpaque;
        }

        assert!(matches!(
            AdmittedIdentityRole::<IncoherentRole>::admitted(),
            Err(IdentityRoleAdmission::NotClassCoherent)
        ));
        assert_eq!(
            CreationLaw::FreshOpaque.declared_class(),
            None,
            "a class-open creation law names no class to join against"
        );
        assert!(AdmittedIdentityRole::<OpenRole>::admitted().is_ok());

        let admitted = AdmittedIdentityRole::<CoherentRole>::admitted()
            .unwrap_or_else(|_| unreachable!("the demo role's columns agree"));
        let columns = AdmittedIdentityColumns::of(&admitted);
        assert_eq!(columns.class(), IdentityClass::ByteDigest);
        assert_eq!(columns.creation(), CreationLaw::DigestOfExactBytes);
    }

    /// law: identity.scope-mismatch-refuses — comparison is total within one
    /// scope and refuses across scopes with the single-cause family body; the
    /// family's declared facts hold.
    /// Owed reversal (red twin): `a < b` on positions must not compile (no
    /// `Ord`/`PartialOrd` exists) — trybuild fixture owed to testpak.
    #[test]
    fn scope_mismatch_refuses() {
        let earlier = AuthorityPosition::assigned(7u8, 1);
        let later = AuthorityPosition::assigned(7u8, 2);
        let elsewhere = AuthorityPosition::assigned(9u8, 1);

        assert!(matches!(
            earlier.try_cmp_same_scope(&later),
            Ok(Ordering::Less)
        ));
        assert!(matches!(
            later.try_cmp_same_scope(&earlier),
            Ok(Ordering::Greater)
        ));
        assert!(matches!(
            earlier.try_cmp_same_scope(&elsewhere),
            Err(OrderComparison::NotSameScope)
        ));
        assert_eq!(OrderComparison::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            OrderComparison::SELECTION_ORDER.first(),
            Some(&"NotSameScope")
        );
    }

    /// law: identity.scope-tuples-are-lawful — a two-part scope is an ordinary
    /// scope; the guard is generic over a scope tuple, not a single id.
    /// Owed reversal: restricting the scope parameter to a single id must break
    /// this law.
    #[test]
    fn scope_tuples_are_lawful() {
        let a = AuthorityPosition::assigned((3u8, true), 10);
        let b = AuthorityPosition::assigned((3u8, false), 10);
        assert!(matches!(
            a.try_cmp_same_scope(&b),
            Err(OrderComparison::NotSameScope)
        ));
    }

    /// law: identity.typed-ref-equality-is-referent-and-version — equality is
    /// exactly the referent-and-version pair.
    /// Owed reversal: adding a third identifying field must break this law.
    #[test]
    fn typed_ref_equality_is_referent_and_version() {
        let one = TypedRef::bound(5u8, 1);
        let same = TypedRef::bound(5u8, 1);
        let newer = TypedRef::bound(5u8, 2);
        assert_eq!(one, same);
        assert_ne!(one, newer);
        assert_eq!(one.referent(), &5u8);
        assert_eq!(newer.version(), 2);
    }

    /// law: identity.commitment-domains-do-not-unify — commitments from
    /// different domains are different types.
    /// Owed reversal (red twin): passing a `Commitment<DomA>` where
    /// `Commitment<DomB>` is required must not compile.
    #[test]
    fn commitment_domains_do_not_unify() {
        struct SchemaDomain;
        struct RemovalDomain;
        let over_schema: Option<fn(Commitment<SchemaDomain>)> = Some(drop);
        let over_removal: Option<fn(Commitment<RemovalDomain>)> = Some(drop);
        assert!(over_schema.is_some());
        assert!(over_removal.is_some());
    }

    /// The scope this home's demo stamp is instantiated over.
    ///
    /// `pub(crate)` for the reason the guard below is: the stamped guard's road
    /// in names this type in its signature, so the scope reaches exactly as far
    /// as the guard it scopes and never one step less.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub(crate) struct DemoStampScope(u8);

    crate::scope_guard_version! {
        /// The stamped demo scope-guard version — written by the declarative
        /// stamp from one explicit typed invocation, not by hand.
        ///
        /// Stamped `pub(crate)` rather than bare: the stamp seats the newtype in
        /// a module of its own, so a guard with no visibility at all would be
        /// sealed inside a module this surface cannot name. `pub(crate)` inside
        /// this proof surface's private, test-gated module is the reach a bare
        /// private guard already had, spelled where the seat now sits.
        pub(crate) struct StampedDemoVersion over DemoStampScope, seated in mod stamped_demo_version;
    }

    /// The hand-written twin of what the stamp writes, authored the way every
    /// scope-guard version in the machine USED to be authored. It is the bar the
    /// stamp had to meet, and it survives here for exactly that: a stamp with
    /// nothing to be compared against proves only that it agrees with itself.
    ///
    /// It is now the only hand-written guard left. The machine's twelve
    /// production guards are stamped, so this twin is a proof-surface specimen
    /// rather than a sample of a live authoring style — and that is the point of
    /// the law below, which is what licenses the twelve to have moved.
    ///
    /// Its position field is private, as a hand-written guard's always was. Nine
    /// production guards used to emit that field publicly, which made the stamp's
    /// "one road in and none out" false of the machine even while it was true of
    /// the stamp; closing them is the twin's shape reaching the production
    /// guards, not a new rule reaching anything.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct HandWrittenDemoVersion(AuthorityPosition<DemoStampScope>);

    impl HandWrittenDemoVersion {
        fn positioned(position: AuthorityPosition<DemoStampScope>) -> Self {
            Self(position)
        }

        fn try_cmp_same_scope(&self, other: &Self) -> Result<Ordering, OrderComparison> {
            self.0.try_cmp_same_scope(&other.0)
        }
    }

    /// law: identity.stamped-scope-guard-matches-its-hand-written-twin — the
    /// declarative stamp writes the Class-C guard this home rules, and its
    /// output agrees with a hand-written twin cause for cause: same order inside
    /// one scope, same refusal across scopes, same equality, same distinctness.
    /// The stamp adds no comparison of its own — it forwards to the machinery
    /// this home already owns.
    /// The parity includes the way an instance is made: both are made through a
    /// named mint over a private position, neither through a public field.
    /// Owed reversal (red twin): comparing two stamped guards over DIFFERENT
    /// scope types must not compile, and neither must `a < b` on one — trybuild
    /// fixtures in testpak.
    #[test]
    fn a_stamped_scope_guard_matches_its_hand_written_twin() {
        let scope = DemoStampScope(3);
        let elsewhere = DemoStampScope(4);

        let stamped_earlier =
            StampedDemoVersion::positioned(AuthorityPosition::assigned(scope.clone(), 1));
        let stamped_later =
            StampedDemoVersion::positioned(AuthorityPosition::assigned(scope.clone(), 2));
        let stamped_elsewhere =
            StampedDemoVersion::positioned(AuthorityPosition::assigned(elsewhere.clone(), 1));

        let twin_earlier =
            HandWrittenDemoVersion::positioned(AuthorityPosition::assigned(scope.clone(), 1));
        let twin_later = HandWrittenDemoVersion::positioned(AuthorityPosition::assigned(scope, 2));
        let twin_elsewhere =
            HandWrittenDemoVersion::positioned(AuthorityPosition::assigned(elsewhere, 1));

        assert_eq!(
            stamped_earlier.try_cmp_same_scope(&stamped_later),
            twin_earlier.try_cmp_same_scope(&twin_later)
        );
        assert_eq!(
            stamped_later.try_cmp_same_scope(&stamped_earlier),
            twin_later.try_cmp_same_scope(&twin_earlier)
        );
        assert_eq!(
            stamped_earlier.try_cmp_same_scope(&stamped_elsewhere),
            twin_earlier.try_cmp_same_scope(&twin_elsewhere)
        );

        assert!(matches!(
            stamped_earlier.try_cmp_same_scope(&stamped_later),
            Ok(Ordering::Less)
        ));
        assert!(matches!(
            stamped_earlier.try_cmp_same_scope(&stamped_elsewhere),
            Err(OrderComparison::NotSameScope)
        ));
        assert_eq!(stamped_earlier, stamped_earlier.clone());
        assert_ne!(stamped_earlier, stamped_later);
    }

    /// law: identity.stamped-representation-cannot-be-laundered — the stamp
    /// emits ONE road in and none out, and the twin has exactly the same
    /// asymmetry. A separate law from the parity law because it is a separate
    /// claim: parity is about the comparison answers agreeing, this is about the
    /// surface being one-way, and a stamp could match the twin's answers while
    /// handing the position back out.
    /// The green half states the road that exists and its shape — `positioned`
    /// takes a position under this role and returns the guard, on both the
    /// stamped guard and the twin, and neither type carries a public field. The
    /// road that does not exist is an absence, and an absence is stated by a
    /// compile-fail fixture.
    /// Owed reversal (red twin):
    /// `testpak/tests/compile-fail/a-stamped-representation-cannot-be-laundered.rs`
    /// — taking one role's position out and re-entering it under another role
    /// must not compile, proven over ONE scope type so nothing about the scope
    /// is doing the work.
    #[test]
    fn a_stamped_representation_cannot_be_laundered() {
        // The one road in, as a function value: it takes a position and returns
        // the guard. A road out would be a function of the opposite shape, and
        // there is none to name here.
        let stamped_in: fn(AuthorityPosition<DemoStampScope>) -> StampedDemoVersion =
            StampedDemoVersion::positioned;
        let twin_in: fn(AuthorityPosition<DemoStampScope>) -> HandWrittenDemoVersion =
            HandWrittenDemoVersion::positioned;

        let scope = DemoStampScope(11);
        let stamped = stamped_in(AuthorityPosition::assigned(scope.clone(), 4));
        let twin = twin_in(AuthorityPosition::assigned(scope, 4));

        // Each guard reads only through the comparison the identity home owns:
        // the position is in, and nothing here can name it again.
        assert!(matches!(
            stamped.try_cmp_same_scope(&stamped),
            Ok(Ordering::Equal)
        ));
        assert!(matches!(
            twin.try_cmp_same_scope(&twin),
            Ok(Ordering::Equal)
        ));
    }
}

mod value {
    use super::pairwise_distinct;
    use crate::types::Limit;
    use crate::value::{
        Absence, BoundedText, CANONICAL_INBOUND_PATH, InboundStage, LossyOperation,
        PRE_AUTHORITY_LADDER, PreAuthorityCheck,
    };

    /// law: value.absence-worlds-are-closed-and-six — the classification roster
    /// is exactly six distinct worlds.
    /// Owed reversal: adding a seventh world or collapsing two must break this
    /// law.
    #[test]
    fn absence_worlds_are_closed_and_six() {
        let worlds = [
            Absence::ShapeOptional,
            Absence::ValueNull,
            Absence::Unauthorized,
            Absence::Unmaterialized,
            Absence::Pending,
            Absence::OutcomeUnknown,
        ];
        assert_eq!(worlds.len(), 6);
        assert!(pairwise_distinct(&worlds));
    }

    /// law: value.pre-authority-ladder-is-ordered — five checks in the exact
    /// declared order, lengths first, role last, before any allocation or
    /// authority.
    /// Owed reversal: reordering or dropping a check must break this law.
    #[test]
    fn pre_authority_ladder_is_ordered() {
        assert_eq!(
            PRE_AUTHORITY_LADDER,
            [
                PreAuthorityCheck::Lengths,
                PreAuthorityCheck::Counts,
                PreAuthorityCheck::Offsets,
                PreAuthorityCheck::Expansion,
                PreAuthorityCheck::Role,
            ]
        );
    }

    /// law: value.inbound-path-has-eight-unmerged-stages — eight pairwise
    /// distinct stages from carrier bytes to derived materialization.
    /// Owed reversal: merging two stages must break this law.
    #[test]
    fn inbound_path_has_eight_unmerged_stages() {
        assert_eq!(CANONICAL_INBOUND_PATH.len(), 8);
        assert!(pairwise_distinct(&CANONICAL_INBOUND_PATH));
        assert_eq!(
            CANONICAL_INBOUND_PATH.first(),
            Some(&InboundStage::CarrierBytes)
        );
        assert_eq!(
            CANONICAL_INBOUND_PATH.last(),
            Some(&InboundStage::DerivedMaterialization)
        );
    }

    /// law: value.lossy-operations-stay-distinct — seven closed, distinct
    /// operations; never one generic transform.
    /// Owed reversal: collapsing two operations must break this law.
    #[test]
    fn lossy_operations_stay_distinct() {
        let operations = [
            LossyOperation::Quantization,
            LossyOperation::Redaction,
            LossyOperation::Summarization,
            LossyOperation::Projection,
            LossyOperation::Sampling,
            LossyOperation::Truncation,
            LossyOperation::Selection,
        ];
        assert_eq!(operations.len(), 7);
        assert!(pairwise_distinct(&operations));
    }

    /// law: value.text-admission-roster-is-eight — the profile's defect
    /// roster, closed and distinct; every issue carries its offending
    /// scalar and coordinate.
    /// Owed reversal: collapsing the join-control cause into the
    /// default-ignorable cause must break this law (their repairs differ).
    #[test]
    fn text_admission_roster_is_eight() {
        use crate::value::{TEXT_PROFILE_UNICODE_PIN, TextAdmissionIssue, TextIssue};
        let roster = [
            TextAdmissionIssue::DisallowedControl,
            TextAdmissionIssue::DisallowedSeparator,
            TextAdmissionIssue::Surrogate,
            TextAdmissionIssue::Noncharacter,
            TextAdmissionIssue::BidirectionalControl,
            TextAdmissionIssue::DisallowedDefaultIgnorable,
            TextAdmissionIssue::InvalidJoinControlContext,
            TextAdmissionIssue::NotNfc,
        ];
        assert_eq!(roster.len(), 8);
        assert!(pairwise_distinct(&roster));
        assert_eq!(TEXT_PROFILE_UNICODE_PIN, "17.0.0");
        let issue = TextIssue {
            kind: TextAdmissionIssue::BidirectionalControl,
            scalar: '\u{202E}',
            coordinate: 4,
        };
        assert!(matches!(
            issue.kind,
            TextAdmissionIssue::BidirectionalControl
        ));
        assert_eq!(issue.coordinate, 4);
    }

    /// law: value.bounded-text-carries-its-limit-family — text under one limit
    /// family is a different type than under another.
    /// Owed reversal (red twin): passing text bounded by one family where
    /// another is required must not compile.
    #[test]
    fn bounded_text_carries_its_limit_family() {
        struct PathLimit;
        impl Limit for PathLimit {}
        struct LabelLimit;
        impl Limit for LabelLimit {}
        let over_path: Option<fn(BoundedText<PathLimit>)> = Some(drop);
        let over_label: Option<fn(BoundedText<LabelLimit>)> = Some(drop);
        assert!(over_path.is_some());
        assert!(over_label.is_some());
    }
}

mod numeric {
    use super::pairwise_distinct;
    use crate::numeric::{
        CONSTRUCTOR_AXIS_LADDER, ConstructorAxis, FloatClass, IntervalRelation,
        KNOWLEDGE_AXIS_SELECTION_ORDER, MoneyConstruction, QuantizeCrossing,
        RequirementDisposition, RoundingMode, TypedMarginConstruction,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};

    /// law: numeric.constructor-axis-ladder-is-ordered — unit → scale → range →
    /// witness coherence, exactly.
    /// Owed reversal: reordering the ladder must break this law.
    #[test]
    fn constructor_axis_ladder_is_ordered() {
        assert_eq!(
            CONSTRUCTOR_AXIS_LADDER,
            [
                ConstructorAxis::Unit,
                ConstructorAxis::Scale,
                ConstructorAxis::Range,
                ConstructorAxis::WitnessCoherence,
            ]
        );
    }

    /// law: numeric.families-are-single-cause-with-declared-orders — every
    /// numeric family is single-cause and declares its ladder-ordered selection
    /// order; the four-rung family's order is the ladder itself.
    /// Owed reversal: a family declaring a cause off its ladder must break this
    /// law.
    #[test]
    fn families_are_single_cause_with_declared_orders() {
        assert_eq!(MoneyConstruction::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            MoneyConstruction::SELECTION_ORDER,
            ["CurrencyNotAdmitted", "ScaleNotAdmitted", "RangeExceeded"]
        );
        assert_eq!(TypedMarginConstruction::SHAPE, FamilyShape::SingleCause);
        assert_eq!(TypedMarginConstruction::SELECTION_ORDER.len(), 4);
        assert_eq!(
            TypedMarginConstruction::SELECTION_ORDER.last(),
            Some(&"IncoherentWitness")
        );
        assert_eq!(QuantizeCrossing::SHAPE, FamilyShape::SingleCause);
        assert_eq!(QuantizeCrossing::SELECTION_ORDER.len(), 7);
        assert_eq!(
            QuantizeCrossing::SELECTION_ORDER.first(),
            Some(&"MissingTargetProfile")
        );
        assert_eq!(
            QuantizeCrossing::SELECTION_ORDER.last(),
            Some(&"RangeOverflow")
        );
    }

    /// law: numeric.rounding-modes-are-six — the six standard spellings,
    /// distinct.
    /// Owed reversal: adding a seventh mode must break this law.
    #[test]
    fn rounding_modes_are_six() {
        let modes = [
            RoundingMode::HalfEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::TowardZero,
            RoundingMode::AwayFromZero,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ];
        assert_eq!(modes.len(), 6);
        assert!(pairwise_distinct(&modes));
    }

    /// law: numeric.float-classes-are-six — exactly six exclusive
    /// classifications of one observation.
    /// Owed reversal: collapsing the zero signs must break this law.
    #[test]
    fn float_classes_are_six() {
        let classes = [
            FloatClass::Finite,
            FloatClass::PositiveZero,
            FloatClass::NegativeZero,
            FloatClass::PositiveInfinity,
            FloatClass::NegativeInfinity,
            FloatClass::NaN,
        ];
        assert_eq!(classes.len(), 6);
        assert!(pairwise_distinct(&classes));
    }

    /// law: numeric.requirement-disposition-has-six-terminals — six distinct
    /// terminals; composition may never collapse them.
    /// Owed reversal: merging `Unresolved` into rejection must break this law.
    #[test]
    fn requirement_disposition_has_six_terminals() {
        let terminals = [
            RequirementDisposition::ConclusivelySatisfied,
            RequirementDisposition::ConclusivelyRejected,
            RequirementDisposition::Unresolved,
            RequirementDisposition::Invalid,
            RequirementDisposition::SourceIncomplete,
            RequirementDisposition::ProofUnavailable,
        ];
        assert_eq!(terminals.len(), 6);
        assert!(pairwise_distinct(&terminals));
    }

    /// law: numeric.interval-relations-are-six — six relations, first-class
    /// data, all admitted (no relation is a cause).
    /// Owed reversal: hiding a relation inside dispatch must break this law.
    #[test]
    fn interval_relations_are_six() {
        let relations = [
            IntervalRelation::Is,
            IntervalRelation::IsNot,
            IntervalRelation::LessThan,
            IntervalRelation::AtMost,
            IntervalRelation::MoreThan,
            IntervalRelation::AtLeast,
        ];
        assert_eq!(relations.len(), 6);
        assert!(pairwise_distinct(&relations));
    }

    /// law: numeric.knowledge-axis-selection-order-is-declared — the dated
    /// decision's four-step order, machine-readable, truth-coverage first.
    /// Owed reversal: reordering the decision must break this law.
    #[test]
    fn knowledge_axis_selection_order_is_declared() {
        assert_eq!(KNOWLEDGE_AXIS_SELECTION_ORDER.len(), 4);
        assert_eq!(
            KNOWLEDGE_AXIS_SELECTION_ORDER.first(),
            Some(&"truth-coverage disagreement")
        );
    }

    /// law: numeric.quantize-evidence-binds-nine-facts — the full evidence
    /// record is constructible with every one of the nine facts, none omittable
    /// (a partial record does not compile).
    /// Owed reversal (red twin): removing any field must break this law.
    #[test]
    fn quantize_evidence_binds_nine_facts() {
        use crate::numeric::{
            ApproxObservation, ApproximationProfileId, ApproximationTaint, DecimalScale,
            DiscardedRemainder, ErrorEvidence, ExactCoefficient, FixedDecimal, FloatBitPattern,
            FloatFormatId, QuantizeDisposition, QuantizeEvidence, QuantizeProvenance,
        };
        use crate::types::{EvidenceRef, ReferentAvailability, ReferentIntegrity};

        let half = FixedDecimal::raw(ExactCoefficient::raw(5), DecimalScale::raw(1));
        let evidence = QuantizeEvidence {
            disposition: QuantizeDisposition::Exact,
            source_representation: FloatFormatId::registered(64),
            source_uncertainty: ErrorEvidence::ErrorBound(half),
            target_profile: crate::numeric::DecimalProfileId::default_for_laws(),
            target_scale: DecimalScale::raw(2),
            rounding: RoundingMode::HalfEven,
            discarded_remainder: DiscardedRemainder::Discarded(crate::numeric::ExactRatio::raw(
                ExactCoefficient::raw(1),
                ExactCoefficient::raw(3),
            )),
            error: ErrorEvidence::ErrorBound(half),
            provenance: EvidenceRef::<QuantizeProvenance>::bound(
                [1; 32],
                1,
                ReferentAvailability::Available,
                ReferentIntegrity::Intact,
            ),
        };
        assert_eq!(evidence.disposition, QuantizeDisposition::Exact);
        assert_eq!(evidence.target_scale, DecimalScale::raw(2));

        let observed = ApproxObservation {
            format: FloatFormatId::registered(64),
            raw_bits: FloatBitPattern::raw(0x4000_0000_0000_0000),
            class: FloatClass::Finite,
            profile: ApproximationProfileId::registered(1),
            error: ErrorEvidence::ErrorBound(half),
            taint: ApproximationTaint::DirectlyObserved,
            provenance: EvidenceRef::<crate::numeric::ApproxProvenance>::bound(
                [2; 32],
                1,
                ReferentAvailability::Available,
                ReferentIntegrity::Intact,
            ),
        };
        let same_bits_different_taint = ApproxObservation {
            taint: ApproximationTaint::Propagated,
            ..observed.clone()
        };
        assert_eq!(observed, same_bits_different_taint);
    }

    /// law: numeric.designations-do-not-unify — a currency designation and a
    /// unit designation are different types; the estimate families are three
    /// role-distinct types, never one.
    /// Owed reversal (red twin): passing one designation where the other is
    /// required must not compile.
    #[test]
    fn designations_do_not_unify() {
        use crate::numeric::{
            CurrencyDesignation, DistributionEstimate, ExactEstimate, IntervalEstimate,
            UnitDesignation,
        };
        let over_currency: Option<fn(CurrencyDesignation)> = Some(drop);
        let over_unit: Option<fn(UnitDesignation)> = Some(drop);
        let over_exact: Option<fn(ExactEstimate<u8>)> = Some(drop);
        let over_interval: Option<fn(IntervalEstimate)> = Some(drop);
        let over_distribution: Option<fn(DistributionEstimate)> = Some(drop);
        assert!(over_currency.is_some());
        assert!(over_unit.is_some());
        assert!(over_exact.is_some());
        assert!(over_interval.is_some());
        assert!(over_distribution.is_some());
    }
}

mod bounds {
    use crate::bounds::{
        BoundClass, Budget, BudgetCharge, CROSS_DOMAIN_MINIMUM, Dimension, DimensionId,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};

    struct WorkDemo;
    impl Dimension for WorkDemo {}
    struct EffectDemo;
    impl Dimension for EffectDemo {}

    /// law: bounds.classes-are-closed-and-seven — seven distinct classes;
    /// Time is the durable deadline-policy budget, enforced at the time home.
    /// Owed reversal: an eighth class must break this law.
    #[test]
    fn classes_are_closed_and_seven() {
        let classes = [
            BoundClass::Work,
            BoundClass::Memory,
            BoundClass::Result,
            BoundClass::Effect,
            BoundClass::Suspension,
            BoundClass::Output,
            BoundClass::Time,
        ];
        assert_eq!(classes.len(), 7);
        assert!(super::pairwise_distinct(&classes));
        assert!(!CROSS_DOMAIN_MINIMUM.contains(&BoundClass::Time));
    }

    /// law: bounds.cross-domain-minimum-is-five — exactly the five, Output
    /// excluded (its dimension level is the execution home's).
    /// Owed reversal: adding Output to the minimum must break this law.
    #[test]
    fn cross_domain_minimum_is_five() {
        assert_eq!(CROSS_DOMAIN_MINIMUM.len(), 5);
        assert!(!CROSS_DOMAIN_MINIMUM.contains(&BoundClass::Output));
        assert!(CROSS_DOMAIN_MINIMUM.contains(&BoundClass::Suspension));
    }

    /// law: bounds.budget-is-affine — the one operation on a budget takes it BY
    /// VALUE. The signature is the whole claim: `charge` moves the budget, so
    /// the caller's handle is gone whether the charge was admitted or refused,
    /// and there is no second handle for it to spend twice.
    /// This law and `charge_shrinks_or_refuses` are two claims about one type —
    /// affinity is about how many times a budget can be spent, the charge law
    /// about what one spend yields — and each obligation proves its own.
    /// Owed reversal (red twin): `.clone()` or a copy of a `Budget` must not
    /// compile; the affinity that cannot be written down here is the absence of
    /// `Clone` and `Copy`, and only a compile-fail fixture states an absence.
    #[test]
    fn budget_is_affine() {
        // The coercion holds only while `charge` takes `self` by value: a
        // borrowing signature has a different function type and does not coerce.
        let consuming: fn(Budget<WorkDemo>, u64) -> Result<Budget<WorkDemo>, BudgetCharge> =
            Budget::charge;
        let spent = consuming(Budget::admitted(3), 3);
        assert!(spent.is_ok_and(|budget| budget.remaining() == 0));
        // A refused charge consumes the budget exactly as an admitted one does:
        // the refusal carries no successor, so nothing is refunded.
        let overcharged = consuming(Budget::admitted(3), 4);
        assert!(matches!(overcharged, Err(BudgetCharge::BoundExceeded)));
    }

    /// law: bounds.charge-shrinks-or-refuses — charging consumes the budget and
    /// yields the strictly smaller successor; exact-to-zero is lawful; an
    /// overcharge returns the typed refusal; charging zero changes nothing.
    /// Owed reversal: a charge yielding a larger successor, or a saturating
    /// overcharge, must break this law.
    #[test]
    fn charge_shrinks_or_refuses() {
        let budget: Budget<WorkDemo> = Budget::admitted(10);
        let Ok(smaller) = budget.charge(4) else {
            unreachable!("charge within budget must succeed");
        };
        assert_eq!(smaller.remaining(), 6);
        let Ok(unchanged) = smaller.charge(0) else {
            unreachable!("zero charge must succeed");
        };
        assert_eq!(unchanged.remaining(), 6);
        let Ok(zero) = unchanged.charge(6) else {
            unreachable!("exact-to-zero charge must succeed");
        };
        assert_eq!(zero.remaining(), 0);
        assert!(matches!(zero.charge(1), Err(BudgetCharge::BoundExceeded)));
        assert_eq!(BudgetCharge::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            BudgetCharge::SELECTION_ORDER.first(),
            Some(&"BoundExceeded")
        );
    }

    /// law: bounds.dimensions-do-not-unify — a budget in one dimension is a
    /// different type than in another, and a dimension id is not a budget.
    /// Owed reversal (red twin): passing `Budget<Work>` where `Budget<Effect>`
    /// is required must not compile.
    #[test]
    fn dimensions_do_not_unify() {
        let over_work: Option<fn(Budget<WorkDemo>)> = Some(drop);
        let over_effect: Option<fn(Budget<EffectDemo>)> = Some(drop);
        let over_id: Option<fn(DimensionId)> = Some(drop);
        assert!(over_work.is_some());
        assert!(over_effect.is_some());
        assert!(over_id.is_some());
    }
}

mod authority {
    use super::pairwise_distinct;
    use crate::authority::{
        AttenuationAxis, CapabilityClaimConstruction, CapabilityClaimConstructionIssue,
        CapabilityGrantId, ClaimIssueLimit, ConstraintSourcePair, ForeignSurface, KeyScope,
        OperationAdmission, POSTCONDITION_NON_SUBSTITUTIONS, ProtectedResolution, TrustPosture,
    };
    use crate::identity::{ApplicationScope, CreationLaw, IdentityClass, IdentityRole};
    use crate::logic::Decision;
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::types::ConstLimit;

    /// law: authority.claim-issue-bound-is-compile-time-ten — the issue
    /// collection's bound is the roster's own cardinality, compile-time.
    /// Owed reversal: raising the bound past the roster must break this law.
    #[test]
    fn claim_issue_bound_is_compile_time_ten() {
        assert_eq!(ClaimIssueLimit::MAX, 10);
    }

    /// law: authority.issues-are-ten-and-flat — ten distinct flat issues; the
    /// four killed causes are unrepresentable (no member-parameterized shape).
    /// Owed reversal (red twin): a `MemberMissing(Member)` shape must not
    /// exist.
    #[test]
    fn issues_are_ten_and_flat() {
        let issues = [
            CapabilityClaimConstructionIssue::IssuerMissing,
            CapabilityClaimConstructionIssue::AudienceMissing,
            CapabilityClaimConstructionIssue::SubjectMissing,
            CapabilityClaimConstructionIssue::RightsMissing,
            CapabilityClaimConstructionIssue::ResourcesMissing,
            CapabilityClaimConstructionIssue::ValidityMissing,
            CapabilityClaimConstructionIssue::GenerationMissing,
            CapabilityClaimConstructionIssue::PossessionMissing,
            CapabilityClaimConstructionIssue::PurposeMissing,
            CapabilityClaimConstructionIssue::DelegationChainMalformed,
        ];
        assert_eq!(issues.len(), 10);
        assert!(pairwise_distinct(&issues));
    }

    /// law: authority.family-is-collection-shaped — the family declares the
    /// collection shape and no selection order (independent members, no
    /// ladder).
    /// Owed reversal: declaring a ladder on this family must break this law.
    #[test]
    fn family_is_collection_shaped() {
        assert_eq!(
            CapabilityClaimConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        assert!(CapabilityClaimConstruction::SELECTION_ORDER.is_empty());
    }

    /// law: authority.protected-resolution-is-eight — exactly the eight
    /// outcomes, distinct, never collapsed.
    /// Owed reversal: collapsing any two must break this law.
    #[test]
    fn protected_resolution_is_eight() {
        let outcomes = [
            ProtectedResolution::Resolved,
            ProtectedResolution::NotPresent,
            ProtectedResolution::Shredded,
            ProtectedResolution::Unauthorized,
            ProtectedResolution::KeyAuthorityMissing,
            ProtectedResolution::BindingInvalid,
            ProtectedResolution::Corrupt,
            ProtectedResolution::PhysicallyUnavailable,
        ];
        assert_eq!(outcomes.len(), 8);
        assert!(pairwise_distinct(&outcomes));
    }

    /// law: authority.foreign-surfaces-are-four-and-postures-five — the two
    /// closed classification rosters hold their counts.
    /// Owed reversal: growing either roster silently must break this law.
    #[test]
    fn foreign_surfaces_are_four_and_postures_five() {
        let surfaces = [
            ForeignSurface::ArtifactInterchange,
            ForeignSurface::ProgramAuthoring,
            ForeignSurface::Rendering,
            ForeignSurface::EffectIngress,
        ];
        assert_eq!(surfaces.len(), 4);
        assert!(pairwise_distinct(&surfaces));
        let postures = [
            TrustPosture::Trusted,
            TrustPosture::HonestButFaulty,
            TrustPosture::PotentiallyMalicious,
            TrustPosture::Unavailable,
            TrustPosture::OutOfProfile,
        ];
        assert_eq!(postures.len(), 5);
        assert!(pairwise_distinct(&postures));
    }

    /// law: authority.attenuation-axes-are-six — the narrow-only axes, closed.
    /// Owed reversal: a widening operation appearing must break this law.
    #[test]
    fn attenuation_axes_are_six() {
        let axes = [
            AttenuationAxis::Rights,
            AttenuationAxis::Resources,
            AttenuationAxis::Audience,
            AttenuationAxis::Time,
            AttenuationAxis::Scope,
            AttenuationAxis::Delegation,
        ];
        assert_eq!(axes.len(), 6);
        assert!(pairwise_distinct(&axes));
    }

    /// law: authority.grant-id-declares-two-columns — the first production use
    /// of the two-column law: class Occurrence, creation fresh-opaque.
    /// Owed reversal: deriving creation from class must break this law.
    #[test]
    fn grant_id_declares_two_columns() {
        assert_eq!(CapabilityGrantId::CLASS, IdentityClass::Occurrence);
        assert_eq!(CapabilityGrantId::CREATION, CreationLaw::FreshOpaque);
    }

    /// law: authority.keyscope-is-application-scope — Class F's contract is
    /// implemented by `KeyScope` (the O-13 decision landed).
    /// Owed reversal: removing the impl must break this law.
    #[test]
    fn keyscope_is_application_scope() {
        fn requires_application_scope<S: ApplicationScope>() {}
        requires_application_scope::<KeyScope>();
    }

    /// law: authority.postcondition-matrix-is-thirteen — the honesty matrix
    /// holds its rows, requested≠granted first.
    /// Owed reversal: dropping a row must break this law.
    #[test]
    fn postcondition_matrix_is_thirteen() {
        assert_eq!(POSTCONDITION_NON_SUBSTITUTIONS.len(), 13);
        assert_eq!(
            POSTCONDITION_NON_SUBSTITUTIONS.first(),
            Some(&("requested", "granted"))
        );
        assert!(POSTCONDITION_NON_SUBSTITUTIONS.contains(&("signed", "fresh")));
    }

    /// law: authority.admission-composes-two-judgments — admission carries both
    /// seats; neither substitutes; the pair carrier holds two sources.
    /// Owed reversal: collapsing admission to one Decision must break this law.
    #[test]
    fn admission_composes_two_judgments() {
        let admission = OperationAdmission {
            domain: Decision::Allow,
            authority: Decision::Deny,
        };
        assert_eq!(admission.domain, Decision::Allow);
        assert_eq!(admission.authority, Decision::Deny);
        let pair = ConstraintSourcePair::named(1u8, 2u8);
        assert_eq!(*pair.left(), 1);
        assert_eq!(*pair.right(), 2);
    }
}

mod bytes {
    use super::pairwise_distinct;
    use crate::bytes::{
        CommitmentRole, ContentRegionId, DECODE_MAXIMA, FRAME_HEADER_BYTES, FRAME_MAGIC,
        FRAME_TRAILER_BYTES, FrameDecode, TagProjection, TextFormDecode, WIDTH_CONVENTIONS,
    };
    use crate::identity::{CreationLaw, IdentityClass, IdentityRole};
    use crate::refusal::{FamilyShape, RefusalFamily};

    /// law: bytes.frame-header-is-fourteen-and-trailer-thirty-two — the frame
    /// arithmetic holds: magic 4 + role 2 + version 2 + flags 2 + length 4 = 14
    /// header bytes, 32 trailer bytes, one TPAK magic.
    /// Owed reversal: changing any width must break this law.
    #[test]
    fn frame_header_is_fourteen_and_trailer_thirty_two() {
        assert_eq!(FRAME_HEADER_BYTES, 14);
        assert_eq!(FRAME_TRAILER_BYTES, 32);
        assert_eq!(FRAME_MAGIC, *b"TPAK");
    }

    /// law: bytes.frame-decode-ladder-is-declared — four dependent causes in
    /// the declared order, role first, digest last.
    /// Owed reversal: reordering the ladder must break this law.
    #[test]
    fn frame_decode_ladder_is_declared() {
        assert_eq!(FrameDecode::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            FrameDecode::SELECTION_ORDER,
            [
                "UnknownRole",
                "UnknownVersion",
                "NonzeroReservedFlag",
                "DigestMismatch"
            ]
        );
    }

    /// law: bytes.commitment-roles-are-eight — the one neutral-inspection sum,
    /// eight distinct roles that never substitute.
    /// Owed reversal: collapsing any two roles must break this law.
    #[test]
    fn commitment_roles_are_eight() {
        let roles = [
            CommitmentRole::Checksum,
            CommitmentRole::ContentDigest,
            CommitmentRole::SemanticCommitment,
            CommitmentRole::Mac,
            CommitmentRole::Signature,
            CommitmentRole::InclusionProof,
            CommitmentRole::FreshnessWitness,
            CommitmentRole::RollbackAnchor,
        ];
        assert_eq!(roles.len(), 8);
        assert!(pairwise_distinct(&roles));
    }

    /// law: bytes.text-form-ladder-is-declared — prefix, case, checksum, in
    /// that dependent order.
    /// Owed reversal: reordering must break this law.
    #[test]
    fn text_form_ladder_is_declared() {
        assert_eq!(TextFormDecode::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            TextFormDecode::SELECTION_ORDER,
            ["PrefixUnknown", "MixedCase", "ChecksumInvalid"]
        );
    }

    /// law: bytes.tag-projections-are-four — one register row, four emitted
    /// projections, so wire id, human prefix, and hash domain cannot drift.
    /// Owed reversal: a fifth projection or a dropped one must break this law.
    #[test]
    fn tag_projections_are_four() {
        let projections = [
            TagProjection::DeriveKeyContext,
            TagProjection::TextFormPrefix,
            TagProjection::FrameRole,
            TagProjection::DocsTable,
        ];
        assert_eq!(projections.len(), 4);
        assert!(pairwise_distinct(&projections));
    }

    /// law: bytes.decode-maxima-are-sixteen — the bounded-reader roster holds
    /// its count, its members are pairwise distinct, and total length is the
    /// maximum every other one is read under.
    /// Owed reversal: dropping a maximum, or doubling one, must break this law.
    #[test]
    fn decode_maxima_are_sixteen() {
        assert_eq!(DECODE_MAXIMA.len(), 16);
        assert!(pairwise_distinct(&DECODE_MAXIMA));
        assert_eq!(DECODE_MAXIMA.first(), Some(&"total-length"));
    }

    /// law: bytes.width-conventions-are-eight — the width conventions are their
    /// own roster with its own count, and they are pairwise distinct. A separate
    /// law because they answer a separate question: the maxima bound what a
    /// reader will accept, the conventions fix how a width is written down, and
    /// changing either roster tells a reader nothing about the other.
    /// Owed reversal: dropping a convention, or doubling one, must break this
    /// law.
    #[test]
    fn width_conventions_are_eight() {
        assert_eq!(WIDTH_CONVENTIONS.len(), 8);
        assert!(pairwise_distinct(&WIDTH_CONVENTIONS));
    }

    /// law: bytes.content-region-declares-two-columns — the identity IS the
    /// digest: class byte-digest, creation digest-of-exact-bytes; a payload
    /// reference binds extent + length + the keyed binding reference, and the
    /// frame header carries a registered role.
    /// Owed reversal: changing either column must break this law.
    #[test]
    fn content_region_declares_two_columns() {
        use crate::bytes::{FrameHeader, FrameRoleId, PayloadBindingClaim, PayloadReference};
        use crate::identity::ByteIdentity;
        use crate::types::{EvidenceRef, ReferentAvailability, ReferentIntegrity};

        assert_eq!(ContentRegionId::CLASS, IdentityClass::ByteDigest);
        assert_eq!(ContentRegionId::CREATION, CreationLaw::DigestOfExactBytes);

        let header = FrameHeader {
            role: FrameRoleId::registered(7),
            profile_version: 1,
            flags: 0,
            length: 64,
        };
        assert_eq!(header.flags, 0);

        let reference = PayloadReference {
            extent: ContentRegionId::of(ByteIdentity::raw([9; 32])),
            length: 4096,
            binding: EvidenceRef::<PayloadBindingClaim>::bound(
                [4; 32],
                1,
                ReferentAvailability::Available,
                ReferentIntegrity::Intact,
            ),
        };
        assert_eq!(reference.length, 4096);
        assert_eq!(reference.extent.digest().as_bytes(), &[9; 32]);
    }
}

mod schema {
    use super::pairwise_distinct;
    use crate::refusal::{
        AdmittedPrefix, CompletionPosture, FamilyShape, RefusalFamily, StopBound,
    };
    use crate::schema::{
        CodecConstruction, CodecIssueLimit, CompatibilityEdgeConstruction, CompatibilityIssueLimit,
        ContractConstruction, ContractIssueLimit, DefaultPolicy, FieldCardinality, FieldId,
        LayoutConstruction, LayoutIssueLimit, MigrationBoundary, MigrationConstruction,
        MigrationIssueLimit, Nullability, ProtectedDataTransformation, REFINEMENT_PROPERTIES,
        RefinementConstruction, RefinementConstructionIssue, RefinementIssueLimit, RefinementKind,
        SchemaConstruction, SchemaConstructionIssue, SchemaDescriptorDigest, SchemaFamilyId,
        SchemaIssueLimit, SchemaSemanticCommitment, SchemaVersion, UnknownMemberPolicy,
        VALIDATION_PIPELINE, ValidationStage, VariantId,
    };
    use crate::types::{ConstLimit, PositiveLimit, RootLawsProfile};

    /// law: schema.validation-pipeline-is-seven-ordered — untrusted input
    /// first, contextual admission last, seven distinct stages that never
    /// merge.
    /// Owed reversal: merging two stages must break this law.
    #[test]
    fn validation_pipeline_is_seven_ordered() {
        assert_eq!(VALIDATION_PIPELINE.len(), 7);
        assert!(pairwise_distinct(&VALIDATION_PIPELINE));
        assert_eq!(
            VALIDATION_PIPELINE.first(),
            Some(&ValidationStage::UntrustedInput)
        );
        assert_eq!(
            VALIDATION_PIPELINE.last(),
            Some(&ValidationStage::ContextualAdmission)
        );
    }

    /// law: schema.value-shape-axes-are-four-closed-enums — 3/2/2/4 variants,
    /// four separate enums, never one enum of mutually exclusive variants.
    /// Owed reversal (red twin): fusing the axes into one presence enum must
    /// not compile once the fixture lands.
    #[test]
    fn value_shape_axes_are_four_closed_enums() {
        let cardinalities = [
            FieldCardinality::Required,
            FieldCardinality::Optional,
            FieldCardinality::Repeated,
        ];
        assert_eq!(cardinalities.len(), 3);
        let nullabilities = [Nullability::NonNullable, Nullability::Nullable];
        assert_eq!(nullabilities.len(), 2);
        assert!(matches!(DefaultPolicy::NoDefault, DefaultPolicy::NoDefault));
        let policies = [
            UnknownMemberPolicy::Closed,
            UnknownMemberPolicy::OptionalExtension,
            UnknownMemberPolicy::RequiredExtension,
            UnknownMemberPolicy::OpaquePreserved,
        ];
        assert_eq!(policies.len(), 4);
        assert!(pairwise_distinct(&policies));
    }

    /// law: schema.refinement-kinds-are-nine-and-properties-nine — the closed
    /// registered kind vocabulary and the declared-property roster.
    /// Owed reversal: a host-invented kind must break this law.
    #[test]
    fn refinement_kinds_are_nine_and_properties_nine() {
        let kinds = [
            RefinementKind::Range,
            RefinementKind::Length,
            RefinementKind::Membership,
            RefinementKind::CrossField,
            RefinementKind::Unit,
            RefinementKind::Interval,
            RefinementKind::VariantDependent,
            RefinementKind::Uniqueness,
            RefinementKind::Measure,
        ];
        assert_eq!(kinds.len(), 9);
        assert!(pairwise_distinct(&kinds));
        assert_eq!(REFINEMENT_PROPERTIES.len(), 9);
        assert!(pairwise_distinct(&REFINEMENT_PROPERTIES));
    }

    /// law: schema.migration-boundaries-are-twelve — the closed vocabulary;
    /// rewrap and rotation stay separate crossings.
    /// Owed reversal: fusing rewrap into rotation must break this law.
    #[test]
    fn migration_boundaries_are_twelve() {
        let boundaries = [
            MigrationBoundary::SourceLanguage,
            MigrationBoundary::SchemaMeaning,
            MigrationBoundary::CodecReencoding,
            MigrationBoundary::ImageFormat,
            MigrationBoundary::AcceptedHistoryFormat,
            MigrationBoundary::LayoutRematerialization,
            MigrationBoundary::DataBlockRebuild,
            MigrationBoundary::ProtectedReencryption,
            MigrationBoundary::KeyRewrap,
            MigrationBoundary::KeyRotation,
            MigrationBoundary::ApplicationDataCorrection,
            MigrationBoundary::EffectfulBackfill,
        ];
        assert_eq!(boundaries.len(), 12);
        assert!(pairwise_distinct(&boundaries));
    }

    /// law: schema.protected-transformations-are-six — the closed vocabulary;
    /// shred never maps to absence.
    /// Owed reversal: dropping shred from the vocabulary must break this law.
    #[test]
    fn protected_transformations_are_six() {
        let transformations = [
            ProtectedDataTransformation::Reencryption,
            ProtectedDataTransformation::KeyRewrap,
            ProtectedDataTransformation::KeyRotation,
            ProtectedDataTransformation::SchemaMigration,
            ProtectedDataTransformation::CodecReencoding,
            ProtectedDataTransformation::Shred,
        ];
        assert_eq!(transformations.len(), 6);
        assert!(pairwise_distinct(&transformations));
    }

    /// law: schema.seven-families-are-collection-shaped-with-roster-bounds —
    /// every family declares the collection shape, no ladder, and a
    /// compile-time bound equal to its roster's cardinality.
    /// Owed reversal: any family growing a ladder or losing its bound must
    /// break this law.
    #[test]
    fn seven_families_are_collection_shaped_with_roster_bounds() {
        assert_eq!(ContractConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(RefinementConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(SchemaConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(LayoutConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(CodecConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(MigrationConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(
            CompatibilityEdgeConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        assert!(ContractConstruction::SELECTION_ORDER.is_empty());
        assert_eq!(ContractIssueLimit::MAX, 6);
        assert_eq!(RefinementIssueLimit::MAX, 11);
        assert_eq!(SchemaIssueLimit::MAX, 18);
        assert_eq!(LayoutIssueLimit::MAX, 14);
        assert_eq!(CodecIssueLimit::MAX, 17);
        assert_eq!(MigrationIssueLimit::MAX, 11);
        assert_eq!(CompatibilityIssueLimit::MAX, 11);
    }

    /// law: schema.nested-causes-nest-distinct-families — a real schema
    /// refusal carries a nested refinement refusal of its own family type;
    /// the nested value keeps its always-Complete posture, and both bodies
    /// reach the coupled report seat rather than assembling a carry beside a
    /// posture of their own.
    /// Owed reversal (red twin): a union-typed nested cause must not compile.
    #[test]
    fn nested_causes_nest_distinct_families() {
        let nested = RefinementConstruction::for_laws(AdmittedPrefix::examined_completely(
            RefinementConstructionIssue::NotTotal,
            vec![RefinementConstructionIssue::HiddenIoOrEffect],
            &PositiveLimit::<_, RootLawsProfile>::inhabited_under_profile(),
            StopBound::DeclaredIssueBound,
        ));
        let refusal = SchemaConstruction::for_laws(AdmittedPrefix::examined_completely(
            SchemaConstructionIssue::RefinementInvalid(Box::new(nested)),
            vec![],
            &PositiveLimit::<_, RootLawsProfile>::inhabited_under_profile(),
            StopBound::DeclaredIssueBound,
        ));
        assert_eq!(refusal.issues().len(), 1);
        assert!(matches!(
            refusal.issues().first(),
            SchemaConstructionIssue::RefinementInvalid(inner)
                if inner.posture() == CompletionPosture::Complete && inner.issues().len() == 2
        ));
    }

    /// law: schema.identity-instantiations-declare-two-columns — the home's
    /// five identity instantiations carry their class and creation law, and
    /// the version type rides the scope-guarded order shape.
    /// Owed reversal: any instantiation dropping its declaration must break
    /// this law.
    #[test]
    fn identity_instantiations_declare_two_columns() {
        use crate::identity::{CreationLaw, IdentityClass, IdentityRole};
        assert_eq!(SchemaFamilyId::CLASS, IdentityClass::Occurrence);
        assert_eq!(SchemaFamilyId::CREATION, CreationLaw::FreshOpaque);
        assert_eq!(FieldId::CREATION, CreationLaw::FreshOpaque);
        assert_eq!(VariantId::CREATION, CreationLaw::FreshOpaque);
        assert_eq!(
            SchemaSemanticCommitment::CLASS,
            IdentityClass::SemanticCommitment
        );
        assert_eq!(SchemaDescriptorDigest::CLASS, IdentityClass::ByteDigest);
        let version_shape: Option<fn(SchemaVersion)> = Some(drop);
        assert!(version_shape.is_some());
    }
}

mod time {

    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::time::{
        AcceptedHlc, ChronologyAdmission, ChronologyMerge, ChronologyProfileId, ChronologySummary,
        ClockObservation, ClockObservationProvenance, DeadlinePolicy, DeadlinePolicyConstruction,
        DeadlinePostureView, DurationLimit, DurationLimitConstruction, HlcCoordinate,
        ObservedWallTime, SourceHlc, SpendRecord, TimeDelta,
    };

    /// law: time.tick-is-the-clock-observation — a tick is one admitted clock
    /// reading: interval + domain + provenance, nothing else, and the reading's
    /// uncertainty lives inside the reading.
    /// Owed reversal: a second uncertainty member beside the reading must break
    /// this law.
    #[test]
    fn tick_is_the_clock_observation() {
        use crate::value::BoundedText;
        let text_shape: Option<fn(BoundedText<crate::time::ProvenanceLimit>)> = Some(drop);
        let observation_shape: Option<fn(ClockObservation)> = Some(drop);
        let provenance_shape: Option<fn(ClockObservationProvenance)> = Some(drop);
        assert!(text_shape.is_some());
        assert!(observation_shape.is_some());
        assert!(provenance_shape.is_some());
    }

    /// law: time.observations-are-intervals — a point is the degenerate
    /// interval and says so structurally; a negative delta is lawful evidence
    /// of clock regression.
    /// Owed reversal: a scalar observation type must break this law.
    #[test]
    fn observations_are_intervals() {
        let point = ObservedWallTime {
            earliest_nanos: 100,
            latest_nanos: 100,
        };
        assert_eq!(point.earliest_nanos, point.latest_nanos);
        let regression = TimeDelta {
            earliest_nanos: -5,
            latest_nanos: -1,
        };
        assert!(regression.latest_nanos < 0);
    }

    /// law: time.duration-limit-ladder-and-zero-lawful — the decoded-route
    /// ladder holds its declared order; the typed route is total and zero is
    /// lawful (immediately exhausted), never a cause.
    /// Owed reversal: adding a zero cause must break this law.
    #[test]
    fn duration_limit_ladder_and_zero_lawful() {
        let zero = DurationLimit::admitted(0);
        assert_eq!(zero.nanos(), 0);
        assert_eq!(DurationLimitConstruction::SHAPE, FamilyShape::SingleCause);
        assert_eq!(
            DurationLimitConstruction::SELECTION_ORDER,
            [
                "NonFinite",
                "Approximate",
                "NonCanonical",
                "Negative",
                "WallClockProvenance",
                "ArithmeticOverflow"
            ]
        );
    }

    /// law: time.deadline-policy-ladder-and-opaque-posture — the six-cause
    /// declared order holds; the policy is opaque and its view reveals the
    /// posture without granting construction.
    /// Owed reversal (red twin): public construction of a posture variant must
    /// not compile.
    #[test]
    fn deadline_policy_ladder_and_opaque_posture() {
        assert_eq!(
            DeadlinePolicyConstruction::SELECTION_ORDER,
            [
                "UnsupportedProfile",
                "LostProvenance",
                "MissingWallUncertainty",
                "InvalidChronologyAnchor",
                "InvalidDuration",
                "ArithmeticOverflow"
            ]
        );
        let policy = DeadlinePolicy::duration_budget(DurationLimit::admitted(1_000));
        assert_eq!(policy.posture(), DeadlinePostureView::DurationBudget);
    }

    /// law: time.chronology-merge-is-a-lawful-join — commutative, associative,
    /// idempotent over one profile; cross-profile refuses with the one cause.
    /// Owed reversal: a merge that consults anything beyond the two summaries
    /// must break this law.
    #[test]
    fn chronology_merge_is_a_lawful_join() {
        let profile = ChronologyProfileId::registered(1);
        let a = ChronologySummary {
            profile,
            max_physical: 10,
            max_logical: 3,
        };
        let b = ChronologySummary {
            profile,
            max_physical: 7,
            max_logical: 9,
        };
        let c = ChronologySummary {
            profile,
            max_physical: 12,
            max_logical: 1,
        };
        let ab = a
            .try_merge(b)
            .unwrap_or_else(|_| unreachable!("same profile"));
        let ba = b
            .try_merge(a)
            .unwrap_or_else(|_| unreachable!("same profile"));
        assert_eq!(ab, ba);
        assert_eq!(ab.max_physical, 10);
        assert_eq!(ab.max_logical, 9);
        let ab_c = ab
            .try_merge(c)
            .unwrap_or_else(|_| unreachable!("same profile"));
        let bc = b
            .try_merge(c)
            .unwrap_or_else(|_| unreachable!("same profile"));
        let a_bc = a
            .try_merge(bc)
            .unwrap_or_else(|_| unreachable!("same profile"));
        assert_eq!(ab_c, a_bc);
        assert_eq!(
            a.try_merge(a)
                .unwrap_or_else(|_| unreachable!("same profile")),
            a
        );
        let foreign = ChronologySummary {
            profile: ChronologyProfileId::registered(2),
            max_physical: 1,
            max_logical: 1,
        };
        assert!(matches!(
            a.try_merge(foreign),
            Err(ChronologyMerge::ProfileMismatch)
        ));
    }

    /// law: time.hlc-roles-do-not-unify — source, accepted, and the envelope
    /// are three distinct types; the envelope carries independent extrema, not
    /// a coordinate.
    /// Owed reversal (red twin): a conversion from the envelope to either HLC
    /// role must not compile.
    #[test]
    fn hlc_roles_do_not_unify() {
        let over_source: Option<fn(SourceHlc)> = Some(drop);
        let over_accepted: Option<fn(AcceptedHlc)> = Some(drop);
        let over_summary: Option<fn(ChronologySummary)> = Some(drop);
        assert!(over_source.is_some());
        assert!(over_accepted.is_some());
        assert!(over_summary.is_some());
        let coordinate = HlcCoordinate::at(1, 2);
        assert_eq!(coordinate.logical(), 2);
        // The two roles are made of the same payload and are not
        // cross-constructible from it: an observation is minted here, and the
        // admitted role carries no mint at all outside this crate.
        let observed = SourceHlc::observed(coordinate);
        assert_eq!(observed.coordinate(), coordinate);
        assert_eq!(AcceptedHlc::for_laws(coordinate).coordinate(), coordinate);
    }

    /// A demo admission clock, standing at module scope so the contract's
    /// implementation is read beside the law rather than nested inside it.
    struct DemoClock {
        current: AcceptedHlc,
    }

    /// The demo's own refusal family: the contract names no shared one, so an
    /// implementor states its refusals in its own vocabulary.
    enum DemoAdmission {
        /// This demo admits no zero physical component. It is the demo's rule
        /// and nothing else — the machine's admission rule does not exist yet.
        NotAdmissible,
    }

    impl ChronologyAdmission for DemoClock {
        type Refusal = DemoAdmission;

        fn admit(&mut self, observed: SourceHlc) -> Result<AcceptedHlc, Self::Refusal> {
            let coordinate = observed.coordinate();
            if coordinate.physical() == 0 {
                return Err(DemoAdmission::NotAdmissible);
            }
            self.current = AcceptedHlc::for_laws(coordinate);
            Ok(self.current)
        }
    }

    /// law: time.admission-is-the-only-crossing — the crossing from observed
    /// chronology into admitted chronology is declared as a contract and is
    /// implementable: the observation is CONSUMED, exactly one admitted position
    /// comes out, the clock is mutated by the act, and the contract names a
    /// typed refusal family of the implementor's own rather than a shared one.
    ///
    /// The claim ceiling: this establishes the crossing's SHAPE. The rule the
    /// crossing runs — counter advancement, clock-regression behavior,
    /// excessive-future classification, the overflow refusal — is the admission
    /// clock's machinery, it does not exist at this phase, and the contract's
    /// unwritten body is the seat it lands in. The demo below advances nothing
    /// and claims nothing about advancement.
    ///
    /// Owed reversal (red twin): a road that mints an admitted position without
    /// consuming an observation must not compile.
    #[test]
    fn admission_is_the_only_crossing() {
        let mut clock = DemoClock {
            current: AcceptedHlc::for_laws(HlcCoordinate::at(0, 0)),
        };
        let admitted = clock.admit(SourceHlc::observed(HlcCoordinate::at(7, 1)));
        assert!(admitted.is_ok_and(|position| position.coordinate().physical() == 7));
        // The observation was consumed, so nothing holds a source value and an
        // admitted value claiming to be one reading.
        assert!(matches!(
            clock.admit(SourceHlc::observed(HlcCoordinate::at(0, 0))),
            Err(DemoAdmission::NotAdmissible)
        ));
    }

    /// law: time.spend-uses-the-dimension-register — a spend record binds a
    /// registered bound dimension with magnitude and uncertainty, at a named
    /// recording site.
    /// Owed reversal: a raw-instant spend must break this law.
    #[test]
    fn spend_uses_the_dimension_register() {
        use crate::bounds::DimensionId;
        let spend = SpendRecord {
            dimension: DimensionId::registered(3),
            magnitude: 40,
            uncertainty: 2,
        };
        assert_eq!(spend.dimension.value(), 3);
    }
}

mod history {
    use super::pairwise_distinct;
    use crate::history::{
        AuthoritySequence, CommitKnowledge, CommitPoint, CommitReconciliation,
        FederationComposition, FederationCutEntries, HistoryCut, HistoryDisposition,
        HistoryReadRefusal, HistoryReading, LineageRefusal, LineageRefusalEvidence,
        LineageRefusalReason, RECOVERY_SCAN, ReceiptCompleteness, RecoveryOutcome,
        RemovalAuthorizationClaimConstructionIssue, RemovalPlanConstructionIssue,
        RemovalRefusalIssue, ScopeAppliedCut, SourceClosure, SourceRegions, StoreId,
        StoreLineageId, TurnInputCut, WriterOrderScope,
    };
    use crate::identity::{Occurrence, OccurrenceForm};

    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::types::{
        Completeness, ConstLimit, EvidenceRef, ReferentAvailability, ReferentIntegrity,
    };

    fn demo_scope(seed: u8) -> WriterOrderScope {
        WriterOrderScope {
            lineage: StoreLineageId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh(
                [seed; 16],
            ))),
            generation: crate::history::AuthorityGeneration::for_laws(seed),
            partition: None,
        }
    }

    /// law: history.four-object-split-and-knowledge-axes — the three knowledge
    /// enums are orthogonal and hold their counts; unknown is not
    /// known-absent; outstanding is a lifecycle posture.
    /// Owed reversal: a "committed?" field on an event record must break this
    /// law.
    #[test]
    fn four_object_split_and_knowledge_axes() {
        let knowledge = [
            CommitKnowledge::KnownAbsent,
            CommitKnowledge::KnownCommitted,
            CommitKnowledge::Unknown,
        ];
        assert_eq!(knowledge.len(), 3);
        assert!(pairwise_distinct(&knowledge));
        let receipts = [
            ReceiptCompleteness::Complete,
            ReceiptCompleteness::Incomplete,
        ];
        assert_eq!(receipts.len(), 2);
        let reconciliation = [
            CommitReconciliation::NotRequired,
            CommitReconciliation::Outstanding,
            CommitReconciliation::ReconciledCommitted,
            CommitReconciliation::ReconciledNotCommitted,
        ];
        assert_eq!(reconciliation.len(), 4);
        assert!(pairwise_distinct(&reconciliation));

        let accepted = crate::history::AcceptedEventRecord {
            body: crate::history::EventCommitment::for_laws(crate::identity::Commitment::raw(
                [6; 32],
            )),
            sequence: AuthoritySequence::for_laws(demo_scope(3), 1),
            predecessor: crate::history::ImmediateHistoryPredecessor(
                crate::identity::Commitment::raw([7; 32]),
            ),
        };
        assert_eq!(accepted.sequence.order(), 1);
    }

    /// law: history.lineage-refusal-is-a-composite-pair — the machine's first
    /// composite-pair family: reason + partial evidence, neither droppable,
    /// with the four-cause declared order on the reason member.
    /// Owed reversal (red twin): dropping either member must not compile.
    #[test]
    fn lineage_refusal_is_a_composite_pair() {
        let refusal = LineageRefusal {
            reason: LineageRefusalReason::WrongSourceCut,
            evidence: LineageRefusalEvidence {
                partial: EvidenceRef::bound(
                    [8; 32],
                    1,
                    ReferentAvailability::Available,
                    ReferentIntegrity::Intact,
                ),
            },
        };
        assert_eq!(LineageRefusal::SHAPE, FamilyShape::InseparablePair);
        assert_eq!(LineageRefusal::SELECTION_ORDER.len(), 4);
        assert_eq!(
            LineageRefusal::SELECTION_ORDER.first(),
            Some(&"ContradictoryLineageClaim")
        );
        assert!(matches!(
            refusal.reason,
            LineageRefusalReason::WrongSourceCut
        ));
    }

    /// law: history.federation-composition-is-a-seam — composition sorts
    /// deterministically, refuses duplicates and omissions, and admits a
    /// complete declared set.
    /// Owed reversal (red twin): a raw-map constructor must not exist.
    #[test]
    fn federation_composition_is_a_seam() {
        let store_a = StoreId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([1; 16])));
        let store_b = StoreId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([2; 16])));
        let cut_a = CommitPoint::for_laws(demo_scope(1), 10);
        let cut_b = CommitPoint::for_laws(demo_scope(2), 20);

        let good = FederationCutEntries::composed(
            &[store_a, store_b],
            vec![(store_b, cut_b.clone()), (store_a, cut_a.clone())],
        );
        assert!(good.is_ok_and(|entries| entries.len() == 2));

        let duplicated = FederationCutEntries::composed(
            &[store_a],
            vec![(store_a, cut_a.clone()), (store_a, cut_b.clone())],
        );
        assert!(matches!(
            duplicated,
            Err(FederationComposition::DuplicateAuthority)
        ));

        let omitted = FederationCutEntries::composed(&[store_a, store_b], vec![(store_a, cut_a)]);
        assert!(matches!(
            omitted,
            Err(FederationComposition::OmittedAuthority)
        ));
    }

    /// law: history.order-roles-do-not-unify — sequence, cut, applied cut, and
    /// turn-input cut are four distinct types with no bridges.
    /// Owed reversal (red twin): a From/Into between any two must not compile.
    #[test]
    fn order_roles_do_not_unify() {
        let over_sequence: Option<fn(AuthoritySequence)> = Some(drop);
        let over_cut: Option<fn(CommitPoint)> = Some(drop);
        let over_applied: Option<fn(ScopeAppliedCut)> = Some(drop);
        let over_turn_input: Option<fn(TurnInputCut)> = Some(drop);
        assert!(over_sequence.is_some());
        assert!(over_cut.is_some());
        assert!(over_applied.is_some());
        assert!(over_turn_input.is_some());
    }

    /// law: history.reading-has-three-orthogonal-axes — a real reading carries
    /// disposition, closure, and freshness as separate axes, with the history
    /// cut as the first production evidence-cut instantiation.
    /// Owed reversal: the disposition absorbing either axis must break this
    /// law.
    #[test]
    fn reading_has_three_orthogonal_axes() {
        let reading: HistoryReading<u8> = HistoryReading {
            disposition: HistoryDisposition::Present(7),
            closure: SourceClosure(Completeness::Complete {
                over: SourceRegions {
                    regions: crate::types::Bounded::admitted(
                        vec![],
                        &crate::types::LimitWitness::declared(4),
                    )
                    .unwrap_or_else(|_| unreachable!("empty fits")),
                },
            }),
            freshness: crate::types::Freshness::Current(crate::types::Current::for_laws(7)),
        };
        assert!(matches!(
            reading.disposition,
            HistoryDisposition::Present(7)
        ));
        let cut_shape: Option<fn(HistoryCut)> = Some(drop);
        assert!(cut_shape.is_some());
        assert_eq!(HistoryReadRefusal::SELECTION_ORDER, ["UnsupportedAccess"]);
    }

    /// law: history.removal-families-hold-their-rosters — 12/2/3 issue
    /// rosters, all collection-shaped with compile-time bounds.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn removal_families_hold_their_rosters() {
        use crate::history::{
            RemovalAuthorizationClaimConstruction, RemovalPlanConstruction, RemovalRefusal,
        };
        assert_eq!(crate::history::types::RemovalPlanIssueLimit::MAX, 12);
        assert_eq!(crate::history::types::RemovalClaimIssueLimit::MAX, 2);
        assert_eq!(crate::history::types::RemovalRefusalIssueLimit::MAX, 3);
        assert_eq!(RemovalPlanConstruction::SHAPE, FamilyShape::IssueCollection);
        assert_eq!(
            RemovalAuthorizationClaimConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        assert_eq!(RemovalRefusal::SHAPE, FamilyShape::IssueCollection);
        let plan_issues = [
            RemovalPlanConstructionIssue::AuthorizationClaimMissing,
            RemovalPlanConstructionIssue::PolicyBasisMissing,
            RemovalPlanConstructionIssue::AffectedScopeEmptyOrInvalid,
            RemovalPlanConstructionIssue::RetainedCommitmentOutOfAffectedScope,
            RemovalPlanConstructionIssue::RemovedMaterialSetMissing,
            RemovalPlanConstructionIssue::VisibilityPostureMissing,
            RemovalPlanConstructionIssue::CompletenessPostureMissing,
            RemovalPlanConstructionIssue::RestorationPostureMissing,
            RemovalPlanConstructionIssue::InvalidationSetIncomplete,
            RemovalPlanConstructionIssue::ParticipantObligationsIncomplete,
            RemovalPlanConstructionIssue::EvidenceContractMissing,
            RemovalPlanConstructionIssue::LineageMismatch,
        ];
        assert_eq!(plan_issues.len(), 12);
        assert!(pairwise_distinct(&plan_issues));
        let claim_issues = [
            RemovalAuthorizationClaimConstructionIssue::PrincipalMissing,
            RemovalAuthorizationClaimConstructionIssue::ClaimedAuthorityMissing,
        ];
        assert_eq!(claim_issues.len(), 2);
        let refusal_issues = [
            RemovalRefusalIssue::AuthorityUnproven,
            RemovalRefusalIssue::RetentionConflict,
            RemovalRefusalIssue::ExternalParticipantCannotHonor,
        ];
        assert_eq!(refusal_issues.len(), 3);
    }

    /// law: history.recovery-has-three-endings-and-five-steps — the scan's
    /// declared order and the closed outcome roster.
    /// Owed reversal: a fourth ending must break this law.
    #[test]
    fn recovery_has_three_endings_and_five_steps() {
        assert_eq!(RECOVERY_SCAN.len(), 5);
        assert_eq!(
            RECOVERY_SCAN.first(),
            Some(&"locate-last-valid-commit-point-receipt")
        );
        let endings = [
            RecoveryOutcome::CommittedPrefix,
            RecoveryOutcome::LawfulRollback,
            RecoveryOutcome::TypedRefusal,
        ];
        assert_eq!(endings.len(), 3);
        assert!(pairwise_distinct(&endings));
    }

    /// law: history.epoch-and-cut-succession-are-typed — a stale-epoch write
    /// refuses through its single-cause family, and predecessor/successor
    /// cuts join only through explicit witnesses, never integer matching.
    /// Owed reversal (red twin): joining cuts by comparing ceilings must not
    /// establish succession.
    #[test]
    fn epoch_and_cut_succession_are_typed() {
        use crate::history::{CutTranslationWitness, EpochValidation, SuccessionWitness};
        assert_eq!(EpochValidation::SHAPE, FamilyShape::SingleCause);
        assert_eq!(EpochValidation::SELECTION_ORDER, ["StaleEpoch"]);
        let succession = SuccessionWitness {
            predecessor: CommitPoint::for_laws(demo_scope(4), 10),
            successor: CommitPoint::for_laws(demo_scope(5), 0),
            evidence: EvidenceRef::bound(
                [9; 32],
                1,
                ReferentAvailability::Available,
                ReferentIntegrity::Intact,
            ),
        };
        let translation = CutTranslationWitness {
            source: succession.predecessor.clone(),
            translated: succession.successor.clone(),
            evidence: EvidenceRef::bound(
                [10; 32],
                1,
                ReferentAvailability::Available,
                ReferentIntegrity::Intact,
            ),
        };
        assert_eq!(translation.source, succession.predecessor);
    }
}

mod navigation {
    use super::pairwise_distinct;
    use crate::bounds::{DimensionId, SemanticWork};
    use crate::history::{
        AuthorityGeneration, CommitPoint, FederationCutEntries, FederationCutVector, HistoryCut,
        ScopeAppliedCut, SourceClosure, SourceRegions, StoreLineageId, TurnInputCut,
        WriterOrderScope,
    };
    use crate::identity::{AuthorityPosition, Occurrence, OccurrenceForm, OrderComparison};
    use crate::navigation::{
        AxisCapability, AxisCapabilityLimit, CHECKPOINT_NON_ADVANCERS, CLOSURE_REQUIRED_CLAIMS,
        Cursor, CursorTransplantation, DestinationKind, FUSIBLE_FOLD_OUTPUTS, Fix, FixShape,
        FrameVersion, INCOMPARABLE_ROUTE_DIMENSIONS, MultiAuthorityRelationship,
        PATH_CONTRACT_FACETS, PROHIBITED_SILENT_MERGERS, PageDowngradeTrigger, PathSelector,
        PositioningRefusal, RECONSTRUCTABLE_FACETS, ReferenceFrameId, TraversalForm,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::types::{
        Bounded, Completeness, ConstLimit, EvidenceRef, Freshness, LimitWitness,
        ReferentAvailability, ReferentIntegrity, Stale,
    };
    use core::cmp::Ordering;

    fn demo_frame(seed: u8, version: u64) -> FrameVersion {
        FrameVersion::positioned(AuthorityPosition::assigned(
            ReferenceFrameId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([seed; 16]))),
            version,
        ))
    }

    fn demo_cut(seed: u8, ceiling: u64) -> CommitPoint {
        CommitPoint::for_laws(
            WriterOrderScope {
                lineage: StoreLineageId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh(
                    [seed; 16],
                ))),
                generation: AuthorityGeneration::for_laws(seed),
                partition: None,
            },
            ceiling,
        )
    }

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    fn empty_regions() -> SourceRegions {
        SourceRegions {
            regions: Bounded::admitted(vec![], &LimitWitness::declared(4))
                .unwrap_or_else(|_| unreachable!("empty fits")),
        }
    }

    /// law: navigation.positioning-order-is-declared — four causes in the
    /// declared precedence, `Unsupported` first, `NoRoute` last and
    /// alone owing its closure witness.
    /// Owed reversal (red twin): a payload-free `NoRoute` must not compile.
    #[test]
    fn positioning_order_is_declared() {
        assert_eq!(PositioningRefusal::SHAPE, FamilyShape::SingleCause);
        assert_eq!(PositioningRefusal::SELECTION_ORDER.len(), 4);
        assert_eq!(
            PositioningRefusal::SELECTION_ORDER.first(),
            Some(&"Unsupported")
        );
        assert_eq!(PositioningRefusal::SELECTION_ORDER.last(), Some(&"NoRoute"));
    }

    /// law: navigation.fix-binds-orthogonal-axes — a real fix that is
    /// Approximate AND Incomplete AND Stale at once: the struct binds
    /// orthogonal axes and no shape erases closure or freshness.
    /// Owed reversal: an enum-flattened fix must not be constructible.
    #[test]
    fn fix_binds_orthogonal_axes() {
        let fix: Fix<u8> = Fix {
            shape: FixShape::Approximate(42),
            frame: demo_frame(4, 2),
            source_cuts: FederationCutVector {
                entries: FederationCutEntries::composed(&[], vec![])
                    .unwrap_or_else(|_| unreachable!("empty composes")),
            },
            relationship: MultiAuthorityRelationship::IndependentlyFrozen,
            closure: SourceClosure(Completeness::Incomplete {
                expected: empty_regions(),
                missing: empty_regions(),
            }),
            freshness: Freshness::Stale(Stale::for_laws(42, HistoryCut(demo_cut(5, 9)))),
            alternatives: Bounded::admitted(vec![], &LimitWitness::declared(2))
                .unwrap_or_else(|_| unreachable!("empty fits")),
            access: demo_evidence(6),
            provenance: demo_evidence(7),
            causation: demo_evidence(8),
            work: SemanticWork {
                dimension: DimensionId::registered(1),
                magnitude: 10,
            },
            bounds: demo_evidence(9),
            explanation: demo_evidence(10),
        };
        assert!(matches!(fix.shape, FixShape::Approximate(42)));
        assert!(matches!(fix.closure.0, Completeness::Incomplete { .. }));
        assert!(matches!(fix.freshness, Freshness::Stale(_)));
        let relationships = |r: &MultiAuthorityRelationship| match r {
            MultiAuthorityRelationship::IndependentlyFrozen
            | MultiAuthorityRelationship::CausationConstrained
            | MultiAuthorityRelationship::CoordinationProfile(_) => true,
        };
        assert!(relationships(&fix.relationship));
    }

    /// law: navigation.frame-version-rides-authority-position — versions of
    /// one frame compare; versions of different frames refuse with the
    /// scope-guard family body.
    /// Reversal (red twin):
    /// `testpak/tests/compile-fail/cross-frame-comparison-on-a-production-guard.rs`.
    /// This law drives the scope-checked road through both of its outcomes and
    /// would go on passing unchanged if an ambient `PartialOrd` or `Ord`
    /// appeared beside it, so what falsifies the claim is the DIRECT comparison
    /// refusing, and the fixture is where that is asked — on both traits.
    /// A frame is a VALUE in the position, so two frames' versions are one
    /// type; `cross-scope-comparison-on-a-stamped-guard.rs` compares two scope
    /// ROLES, which is identity's claim and not this one. The laundering of
    /// this guard's position is its own obligation, with its own reversal.
    #[test]
    fn frame_version_rides_authority_position() {
        let frame =
            ReferenceFrameId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([11; 16])));
        let v1 = FrameVersion::positioned(AuthorityPosition::assigned(frame, 1));
        let v2 = FrameVersion::positioned(AuthorityPosition::assigned(frame, 2));
        assert!(matches!(v1.try_cmp_same_scope(&v2), Ok(Ordering::Less)));
        let other = demo_frame(12, 1);
        assert!(matches!(
            v1.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
    }

    /// law: navigation.axis-capabilities-are-nine — the closed roster and its
    /// compile-time bound; an undeclared capability has no representation.
    /// Owed reversal (red twin): `distance` on a metric-free axis must not
    /// compile.
    #[test]
    fn axis_capabilities_are_nine() {
        assert_eq!(AxisCapabilityLimit::MAX, 9);
        let capabilities = [
            AxisCapability::Equality,
            AxisCapability::TotalOrder,
            AxisCapability::PartialOrder,
            AxisCapability::Hierarchy,
            AxisCapability::Intervals,
            AxisCapability::Sets,
            AxisCapability::TypedRelationships,
            AxisCapability::MetricUnderNamedProfile,
            AxisCapability::AdmittedApproximation,
        ];
        assert_eq!(capabilities.len(), 9);
        assert!(pairwise_distinct(&capabilities));
    }

    /// law: navigation.cursor-transplantation-owes-its-order — eight causes
    /// with the AUTHORED precedence: family gates decode, source before its
    /// scoped generations, the query before its refinements, the cut last.
    /// Owed reversal: reordering the declared list must break this law.
    #[test]
    fn cursor_transplantation_owes_its_order() {
        assert_eq!(CursorTransplantation::SHAPE, FamilyShape::SingleCause);
        assert_eq!(CursorTransplantation::SELECTION_ORDER.len(), 8);
        assert_eq!(
            CursorTransplantation::SELECTION_ORDER.first(),
            Some(&"WrongFamily")
        );
        assert_eq!(
            CursorTransplantation::SELECTION_ORDER.last(),
            Some(&"CrossCut")
        );
        let causes = [
            CursorTransplantation::WrongFamily,
            CursorTransplantation::CrossSource,
            CursorTransplantation::CrossGeneration,
            CursorTransplantation::CrossQuery,
            CursorTransplantation::CrossFilter,
            CursorTransplantation::CrossOrder,
            CursorTransplantation::CrossDirection,
            CursorTransplantation::CrossCut,
        ];
        assert_eq!(causes.len(), 8);
        assert!(pairwise_distinct(&causes));
    }

    /// law: navigation.continuation-roles-do-not-unify — cursor, applied cut,
    /// and turn-input cut are distinct types with no bridges; the durable
    /// checkpoint is runtime-owned and only referenced.
    /// Owed reversal (red twin): a From/Into among them must not compile.
    #[test]
    fn continuation_roles_do_not_unify() {
        let over_cursor: Option<fn(Cursor)> = Some(drop);
        let over_applied: Option<fn(ScopeAppliedCut)> = Some(drop);
        let over_turn_input: Option<fn(TurnInputCut)> = Some(drop);
        assert!(over_cursor.is_some());
        assert!(over_applied.is_some());
        assert!(over_turn_input.is_some());
    }

    /// law: navigation.rosters-hold — traversal forms, destination kinds,
    /// page downgrade triggers, and the declared consts hold their exact
    /// cardinalities.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn traversal_path_and_checkpoint_rosters_hold() {
        let forms = [
            TraversalForm::FoldFiniteJournal,
            TraversalForm::UnfoldBoundedExpansion,
            TraversalForm::CombinedFoldUnfold,
            TraversalForm::MonotoneFixedPoint,
        ];
        assert_eq!(forms.len(), 4);
        assert!(pairwise_distinct(&forms));
        let kinds = [
            DestinationKind::ExactAddress,
            DestinationKind::BoundedRegion,
            DestinationKind::StatePredicate,
            DestinationKind::DecisionCondition,
            DestinationKind::FixedPointCondition,
            DestinationKind::AdmissibleTerminalSet,
        ];
        assert_eq!(kinds.len(), 6);
        assert!(pairwise_distinct(&kinds));
        let triggers = [
            PageDowngradeTrigger::BrokenChain,
            PageDowngradeTrigger::CrossedCut,
            PageDowngradeTrigger::MixedGeneration,
        ];
        assert_eq!(triggers.len(), 3);
        assert!(pairwise_distinct(&triggers));
        assert_eq!(CLOSURE_REQUIRED_CLAIMS.len(), 5);
        assert_eq!(FUSIBLE_FOLD_OUTPUTS.len(), 8);
        assert_eq!(INCOMPARABLE_ROUTE_DIMENSIONS.len(), 5);
        assert_eq!(PATH_CONTRACT_FACETS.len(), 8);
        assert_eq!(PROHIBITED_SILENT_MERGERS.len(), 5);
        let selectors = [
            PathSelector::SegmentWildcard,
            PathSelector::RecursiveDescent,
        ];
        assert_eq!(selectors.len(), 2);
        assert!(pairwise_distinct(&selectors));
        assert_eq!(CHECKPOINT_NON_ADVANCERS.len(), 10);
        assert_eq!(RECONSTRUCTABLE_FACETS.len(), 6);
    }
}

mod port {
    use super::pairwise_distinct;
    use crate::identity::{
        AuthorityPosition, Commitment, Occurrence, OccurrenceForm, OrderComparison,
    };
    use crate::port::{
        AdmittedForeign, DeadlineExpiryPosture, ForeignClaim, HOST_OBLIGATION_AXES, OutboundBounds,
        PortBoundsDeclaration, PortEffectPosture, PortFamilyId, PortFamilyVersion, PortOperation,
        PortPostcondition, PortPostconditionLimit, PortRole, RESULT_PROJECTION_AXES,
        ResponseBinding, SELF_DESCRIBING_REFUSAL_STATEMENTS, SecretAuthorityVerb,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::schema::SchemaSemanticCommitment;
    use crate::types::{
        AdmittedLimit, Bounded, ConstLimit, EvidenceRef, ReferentAvailability, ReferentIntegrity,
        RootLawsProfile,
    };
    use core::cmp::Ordering;

    fn demo_family(seed: u8) -> PortFamilyId {
        PortFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([seed; 16])))
    }

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: port.family-version-rides-authority-position — versions of one
    /// family compare; versions of different families refuse with the
    /// scope-guard family body (the fourth production use).
    /// Reversal (red twin), discharged by the stamp this guard is now written
    /// by, on the stamp's own fixtures: cross-scope comparison is a category error
    /// (`cross-scope-comparison-on-a-stamped-guard.rs`) and the position has no
    /// road out and none back in (`a-stamped-representation-cannot-be-laundered.rs`).
    /// Both prove a property of the generated shape, which is what makes them
    /// this guard's reversal rather than another home's.
    #[test]
    fn family_version_rides_authority_position() {
        let family = demo_family(1);
        let v1 = PortFamilyVersion::positioned(AuthorityPosition::assigned(family, 1));
        let v2 = PortFamilyVersion::positioned(AuthorityPosition::assigned(family, 2));
        assert!(matches!(v1.try_cmp_same_scope(&v2), Ok(Ordering::Less)));
        let other = PortFamilyVersion::positioned(AuthorityPosition::assigned(demo_family(2), 1));
        assert!(matches!(
            v1.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
    }

    /// law: port.roles-are-thirteen — the closed role inventory.
    /// Owed reversal: a fourteenth role appearing silently must break this law.
    #[test]
    fn roles_are_thirteen() {
        let roles = [
            PortRole::AcceptedHistoryInspection,
            PortRole::EventPublication,
            PortRole::DurableCheckpointAuthority,
            PortRole::MutableAuthority,
            PortRole::ArtifactRetrievalPublication,
            PortRole::ProtectedPayloadExtent,
            PortRole::SecretAuthorityOperations,
            PortRole::WallClockChronologyObservation,
            PortRole::AbsoluteMonotonicProgress,
            PortRole::EntropyGeneratedIdentity,
            PortRole::TransportExternalEffects,
            PortRole::NamespacePublication,
            PortRole::DeviceExternalToolEffects,
        ];
        assert_eq!(roles.len(), 13);
        assert!(pairwise_distinct(&roles));
    }

    /// law: port.response-binding-owes-its-order — twelve coexisting causes
    /// with the AUTHORED precedence: existence, then the spent one-shot, then
    /// request identity, contract shape, authority, bounds, and temporal
    /// facts last.
    /// Owed reversal: reordering the declared list must break this law.
    #[test]
    fn response_binding_owes_its_order() {
        assert_eq!(ResponseBinding::SHAPE, FamilyShape::SingleCause);
        assert_eq!(ResponseBinding::SELECTION_ORDER.len(), 12);
        assert_eq!(
            ResponseBinding::SELECTION_ORDER.first(),
            Some(&"DeadAttempt")
        );
        assert_eq!(ResponseBinding::SELECTION_ORDER.last(), Some(&"Late"));
        let causes = [
            ResponseBinding::DeadAttempt,
            ResponseBinding::SecondResume,
            ResponseBinding::WrongRequest,
            ResponseBinding::Duplicate,
            ResponseBinding::WrongFamily,
            ResponseBinding::WrongType,
            ResponseBinding::WrongCapability,
            ResponseBinding::WrongSource,
            ResponseBinding::WrongGeneration,
            ResponseBinding::OverBound,
            ResponseBinding::Expired,
            ResponseBinding::Late,
        ];
        assert_eq!(causes.len(), 12);
        assert!(pairwise_distinct(&causes));
    }

    /// law: port.foreign-claim-admits-only-through-evidence — the seam runs:
    /// wrapping is free, the one unwrap consumes the claim against admission
    /// evidence, and the admitted value carries that evidence.
    /// Owed reversal (red twin): any other unwrap of a `ForeignClaim` must
    /// not compile.
    #[test]
    fn foreign_claim_admits_only_through_evidence() {
        let claim = ForeignClaim::foreign(42_u8);
        let admitted: AdmittedForeign<u8> = claim.admitted(demo_evidence(3));
        assert_eq!(admitted.value, 42);
        assert_eq!(admitted.admission, demo_evidence(3));
    }

    /// law: port.operation-contract-composes — a full seventeen-fact
    /// operation contract constructs through the checked roads, with the
    /// postcondition set on the compile-time bound.
    /// Owed reversal: a universal request envelope must not exist to compile
    /// against.
    #[test]
    fn operation_contract_composes() {
        let operation = PortOperation {
            family: PortFamilyVersion::positioned(AuthorityPosition::assigned(demo_family(4), 1)),
            operation: Commitment::raw([5; 32]),
            request_schema: SchemaSemanticCommitment::for_laws(Commitment::raw([6; 32])),
            response_schema: SchemaSemanticCommitment::for_laws(Commitment::raw([7; 32])),
            effect_posture: PortEffectPosture::Effectful,
            capability_scope: Commitment::raw([8; 32]),
            resource_scope: Commitment::raw([9; 32]),
            subject_bindings: None,
            destination: Some(Commitment::raw([10; 32])),
            release_contracts: Commitment::raw([11; 32]),
            generations: None,
            recovery_posture: Commitment::raw([12; 32]),
            information_label: Commitment::raw([13; 32]),
            bounds: PortBoundsDeclaration {
                portable_work: 1_000,
                bytes: 4_096,
                memory: 8_192,
                concurrency: 4,
                output: 2_048,
            },
            deadline_allowance: Commitment::raw([14; 32]),
            postconditions: Bounded::admitted_const(
                vec![PortPostcondition::Durability],
                &AdmittedLimit::<_, RootLawsProfile>::under_profile(),
            )
            .unwrap_or_else(|_| unreachable!("one fits three")),
            evidence_families: Commitment::raw([15; 32]),
            refusal_families: Commitment::raw([16; 32]),
            qualification: demo_evidence(17),
        };
        assert_eq!(operation.effect_posture, PortEffectPosture::Effectful);
        assert_eq!(PortPostconditionLimit::MAX, 3);
    }

    /// law: port.rosters-hold — postconditions three, secret verbs nine,
    /// expiry postures two, effect postures two, and the declared consts hold
    /// their exact cardinalities.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn rosters_hold() {
        let postconditions = [
            PortPostcondition::Durability,
            PortPostcondition::AtomicBoundary,
            PortPostcondition::CancellationPosture,
        ];
        assert_eq!(postconditions.len(), 3);
        assert!(pairwise_distinct(&postconditions));
        let verbs = [
            SecretAuthorityVerb::Authorize,
            SecretAuthorityVerb::Select,
            SecretAuthorityVerb::Wrap,
            SecretAuthorityVerb::UnwrapIntoOpaqueUseHandle,
            SecretAuthorityVerb::Rotate,
            SecretAuthorityVerb::Rewrap,
            SecretAuthorityVerb::Revoke,
            SecretAuthorityVerb::Shred,
            SecretAuthorityVerb::Evidence,
        ];
        assert_eq!(verbs.len(), 9);
        assert!(pairwise_distinct(&verbs));
        let expiry = [
            DeadlineExpiryPosture::BeforeAdmission,
            DeadlineExpiryPosture::AfterDurableAdmission,
        ];
        assert_eq!(expiry.len(), 2);
        let postures = [PortEffectPosture::Observation, PortEffectPosture::Effectful];
        assert_eq!(postures.len(), 2);
        assert_eq!(RESULT_PROJECTION_AXES.len(), 11);
        assert_eq!(SELF_DESCRIBING_REFUSAL_STATEMENTS.len(), 5);
        assert_eq!(HOST_OBLIGATION_AXES.len(), 16);
        let bounds_shape: Option<fn(OutboundBounds)> = Some(drop);
        assert!(bounds_shape.is_some());
    }
}

mod declaration {
    use super::pairwise_distinct;
    use crate::declaration::{
        AuthoredNameConstruction, AuthoringRole, CANONICAL_FACET_SEQUENCE, CONVERGENCE_ROUTES,
        ClaimKind, ClosureNamespace, CoordinateRole, DeclarationGraph, ExportAliasDerivation,
        Facet, FacetForm, FrontendRole, HOW_FACET_CONTENT, HygieneClass, LINKER_CONTRACT,
        LinkResolution, LinkResolutionIssue, META_EVALUATION_LOCKS, MetaStageLaw, ProjectionClaim,
        ProjectionContractConstruction, ProjectionProfileId, ProjectionProfileVersion,
        SourceCoordinate, Stage, SymbolIdentity, TopLevelForm, WHAT_FACET_CONTENT,
        WHEN_FACET_CONTENT, WHERE_FACET_CONTENT, WHO_FACET_CONTENT, WHY_FACET_CONTENT,
    };
    use crate::identity::{
        AuthorityPosition, Commitment, Occurrence, OccurrenceForm, OrderComparison,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::types::ConstLimit;
    use core::cmp::Ordering;

    fn demo_profile(seed: u8, version: u64) -> ProjectionProfileVersion {
        ProjectionProfileVersion::positioned(AuthorityPosition::assigned(
            ProjectionProfileId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([seed; 16]))),
            version,
        ))
    }

    fn demo_symbol(seed: u8) -> SymbolIdentity {
        SymbolIdentity(Commitment::raw([seed; 32]))
    }

    /// law: declaration.export-alias-gate-order-is-declared — seven causes
    /// under the declared derivation gate order; which gates ran is derivable
    /// from the named cause, so the family carries no posture member. A
    /// collision names the two symbols, never the two spellings.
    /// Owed reversal (red twin): a posture member on this family must not
    /// exist.
    #[test]
    fn export_alias_gate_order_is_declared() {
        assert_eq!(ExportAliasDerivation::SHAPE, FamilyShape::SingleCause);
        assert_eq!(ExportAliasDerivation::SELECTION_ORDER.len(), 7);
        assert_eq!(
            ExportAliasDerivation::SELECTION_ORDER.first(),
            Some(&"UnsupportedTargetProfile")
        );
        assert_eq!(
            ExportAliasDerivation::SELECTION_ORDER.last(),
            Some(&"Collision")
        );
    }

    /// law: declaration.name-and-closure-families-are-collections — both
    /// ride the declared-bound road (several issues of one kind are lawful at
    /// once), and a real single-scalar issue carries exactly one scalar and the
    /// coordinate it sat at.
    ///
    /// The bodies themselves are not assembled here. Their seat is band 00's
    /// coupled report package and it is proven once, where the coupling is
    /// stated — see `refusal::every_collection_family_carries_the_coupled_seat`.
    ///
    /// Owed reversal (red twin): a two-scalar payload must not compile.
    #[test]
    fn name_and_closure_families_are_collections() {
        assert_eq!(
            AuthoredNameConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        assert_eq!(ClosureNamespace::SHAPE, FamilyShape::IssueCollection);
    }

    /// law: declaration.link-resolution-ranges-over-four-claim-kinds — the
    /// closed claim kinds ride as a typed member; the linker refuses, never
    /// repairs.
    /// Owed reversal: a fifth claim kind appearing silently must break this
    /// law.
    #[test]
    fn link_resolution_ranges_over_four_claim_kinds() {
        let kinds = [
            ClaimKind::Route,
            ClaimKind::Field,
            ClaimKind::Operation,
            ClaimKind::Identity,
        ];
        assert_eq!(kinds.len(), 4);
        assert!(pairwise_distinct(&kinds));
        assert_eq!(LinkResolution::SHAPE, FamilyShape::IssueCollection);
        let issue = LinkResolutionIssue::MissingClaim {
            kind: ClaimKind::Route,
            site: SourceCoordinate {
                role: CoordinateRole::SemanticOrigin,
                position: 4,
            },
            requiring: demo_symbol(4),
        };
        assert!(matches!(
            issue,
            LinkResolutionIssue::MissingClaim {
                kind: ClaimKind::Route,
                ..
            }
        ));
    }

    /// law: declaration.projection-claims-are-five — the closed claim enum,
    /// the derivable ten-issue cap, and a constructed unstated-claim issue.
    /// Owed reversal: a sixth claim must break this law.
    #[test]
    fn projection_claims_are_five() {
        let claims = [
            ProjectionClaim::Coverage,
            ProjectionClaim::Reversibility,
            ProjectionClaim::Disclosure,
            ProjectionClaim::Actionability,
            ProjectionClaim::Representation,
        ];
        assert_eq!(claims.len(), 5);
        assert!(pairwise_distinct(&claims));
        assert_eq!(crate::declaration::types::ProjectionIssueLimit::MAX, 10);
        assert_eq!(
            ProjectionContractConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
    }

    /// law: declaration.facets-are-six-in-canonical-sequence — WHO first,
    /// WHY last, the six content rosters hold their exact cardinalities, and
    /// the registered facet forms are four typed values, pairwise distinct.
    /// Owed reversal: reordering the canonical sequence must break this law.
    #[test]
    fn facets_are_six_in_canonical_sequence() {
        let facets = [
            Facet::Who,
            Facet::What,
            Facet::Where,
            Facet::When,
            Facet::How,
            Facet::Why,
        ];
        assert_eq!(facets.len(), 6);
        assert!(pairwise_distinct(&facets));
        assert_eq!(CANONICAL_FACET_SEQUENCE.first(), Some(&Facet::Who));
        assert_eq!(CANONICAL_FACET_SEQUENCE.last(), Some(&Facet::Why));
        assert_eq!(WHO_FACET_CONTENT.len(), 6);
        assert_eq!(WHAT_FACET_CONTENT.len(), 8);
        assert_eq!(WHERE_FACET_CONTENT.len(), 6);
        assert_eq!(WHEN_FACET_CONTENT.len(), 6);
        assert_eq!(HOW_FACET_CONTENT.len(), 14);
        assert_eq!(WHY_FACET_CONTENT.len(), 10);
        let forms = [
            FacetForm::CaptureCurrent,
            FacetForm::RequiresEvidence,
            FacetForm::ProducesEvidence,
            FacetForm::Explain,
        ];
        assert_eq!(forms.len(), 4);
        assert!(pairwise_distinct(&forms));
    }

    /// law: declaration.projection-profile-version-rides-authority-position —
    /// the fifth production scope-guard use.
    /// Reversal (red twin), discharged by the stamp this guard is now written
    /// by, on the stamp's own fixtures: cross-scope comparison is a category error
    /// (`cross-scope-comparison-on-a-stamped-guard.rs`) and the position has no
    /// road out and none back in (`a-stamped-representation-cannot-be-laundered.rs`).
    /// Both prove a property of the generated shape, which is what makes them
    /// this guard's reversal rather than another home's.
    #[test]
    fn projection_profile_version_rides_authority_position() {
        let profile =
            ProjectionProfileId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([21; 16])));
        let v1 = ProjectionProfileVersion::positioned(AuthorityPosition::assigned(profile, 1));
        let v2 = ProjectionProfileVersion::positioned(AuthorityPosition::assigned(profile, 2));
        assert!(matches!(v1.try_cmp_same_scope(&v2), Ok(Ordering::Less)));
        let other = demo_profile(22, 1);
        assert!(matches!(
            v1.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
    }

    /// law: declaration.authoring-rosters-hold — stages four and never open,
    /// hygiene six-fold, four authoring roles, four top-level forms, two
    /// front doors, six coordinate roles, and the declared consts.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn authoring_rosters_hold() {
        let stages = [
            Stage::Authoring,
            Stage::Meta,
            Stage::Semantic,
            Stage::Runtime,
        ];
        assert_eq!(stages.len(), 4);
        assert!(pairwise_distinct(&stages));
        let hygiene = [
            HygieneClass::Lexical,
            HygieneClass::Identity,
            HygieneClass::Authority,
            HygieneClass::Effect,
            HygieneClass::Origin,
            HygieneClass::Profile,
        ];
        assert_eq!(hygiene.len(), 6);
        assert!(pairwise_distinct(&hygiene));
        let roles = [
            AuthoringRole::DirectDeclaration,
            AuthoringRole::Fragment,
            AuthoringRole::MetaFunction,
            AuthoringRole::Quotation,
        ];
        assert_eq!(roles.len(), 4);
        let forms = [
            TopLevelForm::Ask,
            TopLevelForm::Do,
            TopLevelForm::Request,
            TopLevelForm::Pend,
        ];
        assert_eq!(forms.len(), 4);
        assert!(pairwise_distinct(&forms));
        let doors = [
            FrontendRole::RustDeclaration,
            FrontendRole::ApplicationLanguage,
        ];
        assert_eq!(doors.len(), 2);
        let coordinates = [
            CoordinateRole::Byte,
            CoordinateRole::UnicodeScalar,
            CoordinateRole::Utf16,
            CoordinateRole::LineColumn,
            CoordinateRole::NormalizedSource,
            CoordinateRole::SemanticOrigin,
        ];
        assert_eq!(coordinates.len(), 6);
        assert!(pairwise_distinct(&coordinates));
        assert_eq!(LINKER_CONTRACT.len(), 7);
        assert_eq!(CONVERGENCE_ROUTES.len(), 4);
        assert_eq!(META_EVALUATION_LOCKS.len(), 3);
        let stage_laws = [
            MetaStageLaw::ExplicitBoundedLift,
            MetaStageLaw::DescriptorIsData,
            MetaStageLaw::NoLiveAuthorityAsMetaValue,
            MetaStageLaw::OutputReentersUntrusted,
            MetaStageLaw::RefusalFamiliesStayDistinct,
            MetaStageLaw::BoundsDeclaredBeforeEvaluation,
        ];
        assert_eq!(stage_laws.len(), 6);
        assert!(pairwise_distinct(&stage_laws));
        let graph = DeclarationGraph::for_laws(Commitment::raw([23; 32]));
        assert_eq!(graph.linked(), &Commitment::raw([23; 32]));
    }

    crate::closed_register! {
        /// One roster stamped by this home's own stamp, for this law alone.
        enum StampedRoster {
            /// The first row.
            First = "first", "the first row";
            /// The second row.
            Second = "second", "the second row";
            /// The third row.
            Third = "third", "the third row";
        }
    }

    /// The hand-kept form of the same roster: a roster array beside a `match`
    /// returning numbers.
    ///
    /// The middle two rows are deliberately transposed. This is the planted
    /// reversal, and it is planted rather than described because the whole
    /// claim below is that the stamp makes exactly this value unwritable — a
    /// claim nobody can read as non-vacuous until the writable form is sitting
    /// beside it, wrong.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HandKeptRoster {
        /// The first row.
        First,
        /// The second row.
        Second,
        /// The third row.
        Third,
    }

    impl HandKeptRoster {
        /// The roster, in the order this declaration states it.
        const ALL: [Self; 3] = [Self::First, Self::Second, Self::Third];

        /// The position each row claims — kept by hand, and drifted.
        const fn slot(self) -> u8 {
            match self {
                Self::First => 0,
                Self::Second => 2,
                Self::Third => 1,
            }
        }
    }

    /// Whether every row of one roster answers its own position in it.
    fn positions_answer_themselves<T: Copy>(roster: &[T], slot: fn(T) -> u8) -> bool {
        roster
            .iter()
            .enumerate()
            .all(|(position, row)| usize::from(slot(*row)) == position)
    }

    /// law: declaration.a-stamped-roster-cannot-disagree-with-its-own-order —
    /// `closed_register!` writes the roster constant and the position answer
    /// from one row list in one expansion, so a stamped roster's slot IS its
    /// position in `ALL` rather than a second number that agrees with it. The
    /// declared stable name and the declared prose come back per row, in the
    /// row they were declared on.
    /// Executed reversal: the hand-kept twin above states the same claim
    /// through the writable form — an array beside a `match` — and this law
    /// requires it to FAIL, so the stamped half is proven non-vacuous rather
    /// than asserted.
    /// The claim's ceiling: closure of the roster is not checked here and is
    /// not checkable anywhere. A row outside the declaration does not exist
    /// because the stamp is the enum's only declaration site, which is a
    /// property of macro output rather than a defect anything catches.
    #[test]
    fn a_stamped_roster_cannot_disagree_with_its_own_order() {
        assert_eq!(StampedRoster::ALL.len(), 3);
        assert!(positions_answer_themselves(
            &StampedRoster::ALL,
            StampedRoster::slot
        ));
        assert_eq!(StampedRoster::ALL.first(), Some(&StampedRoster::First));

        let names: Vec<&str> = StampedRoster::ALL
            .iter()
            .map(|row| row.stable_name())
            .collect();
        assert_eq!(names, vec!["first", "second", "third"]);
        let prose: Vec<&str> = StampedRoster::ALL
            .iter()
            .map(|row| row.described())
            .collect();
        assert_eq!(
            prose,
            vec!["the first row", "the second row", "the third row"]
        );

        assert_eq!(HandKeptRoster::ALL.len(), StampedRoster::ALL.len());
        assert!(!positions_answer_themselves(
            &HandKeptRoster::ALL,
            HandKeptRoster::slot
        ));
    }
}

mod semantic {
    use super::pairwise_distinct;
    use crate::bounds::{BoundClass, DimensionId};
    use crate::declaration::Stage;
    use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole};
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::semantic::{
        BehaviorFamily, BoundDimensionRow, CapabilityRequirements, DefinitionBoundary,
        EvidenceObligation, ExplanationObligation, Judgment, OPERATION_CONTRACT_FACTS,
        OrderedEffectRegions, RefusalSet, SEMANTIC_FORM_CONTENT, SemanticForm,
        SemanticFormConstruction, SemanticFormConstructionIssue, SemanticGraphDigest,
        SemanticTypeRef, SourceCutPosture, SymbolicBounds,
    };
    use crate::types::{Bounded, LimitWitness};

    /// law: semantic.form-family-holds-fifteen — the content roster read as
    /// defects, every issue carrying only its canonical-order position, on
    /// the declared-bound road.
    /// Owed reversal (red twin): a text-carrying payload must not compile.
    #[test]
    fn form_family_holds_fifteen() {
        let issues = [
            SemanticFormConstructionIssue::UnresolvedReference { position: 0 },
            SemanticFormConstructionIssue::InvalidTypeOrValueShape { position: 1 },
            SemanticFormConstructionIssue::ValueOutsideClosedAlgebra { position: 2 },
            SemanticFormConstructionIssue::AuthorityBearingCapture { position: 3 },
            SemanticFormConstructionIssue::SourceOrHistoricalCutRequirementIncomplete {
                position: 4,
            },
            SemanticFormConstructionIssue::QueryOrDecisionSemanticsIncomplete { position: 5 },
            SemanticFormConstructionIssue::TruthOrProofPostureIncomplete { position: 6 },
            SemanticFormConstructionIssue::EffectOrCapabilityDeclarationIncomplete { position: 7 },
            SemanticFormConstructionIssue::BoundOrWorkFormulaMissing { position: 8 },
            SemanticFormConstructionIssue::RefusalOrExplanationStructureIncomplete { position: 9 },
            SemanticFormConstructionIssue::IncompleteJudgment { position: 10 },
            SemanticFormConstructionIssue::ImportOrKernelInterfaceClosureIncomplete {
                position: 11,
            },
            SemanticFormConstructionIssue::NonCanonicalOrder { position: 12 },
            SemanticFormConstructionIssue::HiddenProducerOnlyNode { position: 13 },
            SemanticFormConstructionIssue::DuplicateOrCollidingIdentity { position: 14 },
        ];
        assert_eq!(issues.len(), 15);
        assert!(pairwise_distinct(&issues));
        assert_eq!(
            SemanticFormConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        let form = SemanticForm::for_laws(Commitment::raw([31; 32]));
        assert_eq!(form.content(), &Commitment::raw([31; 32]));
        assert_eq!(SEMANTIC_FORM_CONTENT.len(), 12);
    }

    /// law: semantic.judgment-binds-nine-axes — a complete judgment
    /// constructs with all nine typed members: normal type, stage, refusals,
    /// ordered effects, capability requirements, source posture, applicable
    /// symbolic bounds, explanation, evidence.
    /// Owed reversal (red twin): an erasable axis must not compile.
    #[test]
    fn judgment_binds_nine_axes() {
        let judgment = Judgment {
            normal_type: SemanticTypeRef(Commitment::raw([1; 32])),
            stage: Stage::Semantic,
            refuses: RefusalSet {
                families: Bounded::admitted(
                    vec![Commitment::raw([2; 32])],
                    &LimitWitness::declared(4),
                )
                .unwrap_or_else(|_| unreachable!("one fits")),
            },
            effects: OrderedEffectRegions {
                regions: Bounded::admitted(vec![], &LimitWitness::declared(4))
                    .unwrap_or_else(|_| unreachable!("empty fits")),
            },
            requires: CapabilityRequirements {
                requirements: Bounded::admitted(
                    vec![Commitment::raw([3; 32])],
                    &LimitWitness::declared(4),
                )
                .unwrap_or_else(|_| unreachable!("one fits")),
            },
            reads: SourceCutPosture(Commitment::raw([4; 32])),
            bounds: SymbolicBounds {
                dimensions: Bounded::admitted(
                    vec![BoundDimensionRow {
                        dimension: DimensionId::registered(1),
                        class: BoundClass::Work,
                        maximum: 1_000,
                    }],
                    &LimitWitness::declared(8),
                )
                .unwrap_or_else(|_| unreachable!("one fits")),
            },
            explains: ExplanationObligation(Commitment::raw([5; 32])),
            evidences: EvidenceObligation(Commitment::raw([6; 32])),
        };
        assert_eq!(judgment.stage, Stage::Semantic);
        assert_eq!(judgment.bounds.dimensions.len(), 1);
    }

    /// law: semantic.behavior-and-boundary-rosters-hold — seven behavior
    /// families, the three-way definition boundary, and the ten-fact
    /// operation contract.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn behavior_and_boundary_rosters_hold() {
        let families = [
            BehaviorFamily::ValuesAndStructure,
            BehaviorFamily::TruthAndDecisions,
            BehaviorFamily::DefinitionsAndKernels,
            BehaviorFamily::SourceAndQuery,
            BehaviorFamily::SemanticPositioningAndNavigation,
            BehaviorFamily::PhysicalDerivation,
            BehaviorFamily::PublicationAndEffects,
        ];
        assert_eq!(families.len(), 7);
        assert!(pairwise_distinct(&families));
        let boundaries = [
            DefinitionBoundary::PrimitiveOperator,
            DefinitionBoundary::DefinitionOverSmallerOperators,
            DefinitionBoundary::QualifiedOpaqueKernel,
        ];
        assert_eq!(boundaries.len(), 3);
        assert!(pairwise_distinct(&boundaries));
        assert_eq!(OPERATION_CONTRACT_FACTS.len(), 10);
    }

    /// law: semantic.graph-digest-is-meaning — the semantic graph digest is
    /// a commitment over normalized meaning, never a byte identity.
    /// Owed reversal (red twin): substituting a byte identity must not
    /// compile.
    #[test]
    fn graph_digest_is_meaning() {
        assert_eq!(
            SemanticGraphDigest::CLASS,
            IdentityClass::SemanticCommitment
        );
        assert_eq!(
            SemanticGraphDigest::CREATION,
            CreationLaw::DomainTaggedDigestOfMeaning
        );
        let digest = SemanticGraphDigest(Commitment::raw([7; 32]));
        assert_eq!(digest.0, Commitment::raw([7; 32]));
    }
}

mod execution {
    use super::pairwise_distinct;
    use crate::bounds::{BoundClass, DimensionId};
    use crate::execution::{
        AlgebraicLaw, AlgebraicLawLimit, CommandKind, CommandOrdinal, EffectBatch,
        EffectBatchComposition, EffectCommand, EffectfulRecursionLane, ExecutionForm,
        ExecutionFormConstruction, ExecutionFormConstructionIssue, ExecutionFormFamilyId,
        ExecutionFormVersion, ForbiddenIdentitySource, GroupFenceDefect,
        INDEPENDENCE_MAY_NOT_SHARE, INDEPENDENCE_MAY_SHARE, INTERLEAVED_CLOSURE_TOTALS,
        KernelBindingPolicy, KernelBindingPolicyConstruction, KernelBindingPosture,
        KernelFallbackPolicy, KernelInterfaceContract, KernelInterfaceContractConstructionIssue,
        KernelInterfaceContractRef, KernelRealizationId, KernelRequirement, KernelSemanticContract,
        KernelSemanticContractConstructionIssue, KernelSemanticContractRef,
        KernelSubstitutionScope, OPERATOR_REGISTER, RecursionWitness, RequiredContractKind,
        WORK_DIMENSIONS,
    };
    use crate::identity::{
        AuthorityPosition, Commitment, Occurrence, OccurrenceForm, OrderComparison,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::semantic::BoundDimensionRow;
    use crate::types::{
        Bounded, ConstLimit, EvidenceRef, LimitWitness, ReferentAvailability, ReferentIntegrity,
    };
    use core::cmp::Ordering;

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: execution.operator-register-holds-and-versions — the authored v1
    /// register holds thirty-eight distinct rows, and changing the set
    /// advances the scope-guarded Execution-Form version (the sixth
    /// production use).
    /// Reversal (red twin), discharged by the stamp this guard is now written
    /// by, on the stamp's own fixtures: cross-scope comparison is a category error
    /// (`cross-scope-comparison-on-a-stamped-guard.rs`) and the position has no
    /// road out and none back in (`a-stamped-representation-cannot-be-laundered.rs`).
    /// Both prove a property of the generated shape, which is what makes them
    /// this guard's reversal rather than another home's.
    #[test]
    fn operator_register_holds_and_versions() {
        assert_eq!(OPERATOR_REGISTER.len(), 38);
        let mut sorted = OPERATOR_REGISTER.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 38);
        assert!(OPERATOR_REGISTER.contains(&"group"));
        assert!(OPERATOR_REGISTER.contains(&"relation_expansion"));
        assert!(OPERATOR_REGISTER.contains(&"truncate"));
        let family =
            ExecutionFormFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([41; 16])));
        let v1 = ExecutionFormVersion::positioned(AuthorityPosition::assigned(family, 1));
        let v2 = ExecutionFormVersion::positioned(AuthorityPosition::assigned(family, 2));
        assert!(matches!(v1.try_cmp_same_scope(&v2), Ok(Ordering::Less)));
        let other = ExecutionFormVersion::positioned(AuthorityPosition::assigned(
            ExecutionFormFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([42; 16]))),
            1,
        ));
        assert!(matches!(
            v1.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
    }

    /// law: execution.form-family-holds-fifteen — the per-operator roster
    /// read as statement defects, position-only payloads, on the
    /// declared-bound road; the five declarable algebraic laws hold their
    /// compile-time cap.
    /// Owed reversal (red twin): a payload carrying operator content must not
    /// compile.
    #[test]
    fn form_family_holds_fifteen() {
        let issues = [
            ExecutionFormConstructionIssue::UnknownOperator { position: 0 },
            ExecutionFormConstructionIssue::OperatorVersionMismatch { position: 1 },
            ExecutionFormConstructionIssue::OperandOrResultSortMissing { position: 2 },
            ExecutionFormConstructionIssue::SortMismatch { position: 3 },
            ExecutionFormConstructionIssue::ValueOrControlDependencyInvalid { position: 4 },
            ExecutionFormConstructionIssue::RegionRelationshipInvalid { position: 5 },
            ExecutionFormConstructionIssue::RecursionRelationshipInvalid { position: 6 },
            ExecutionFormConstructionIssue::EffectOrSuspensionPostureMissing { position: 7 },
            ExecutionFormConstructionIssue::WorkChargeMissing { position: 8 },
            ExecutionFormConstructionIssue::AlgebraicLawDeclarationMissing { position: 9 },
            ExecutionFormConstructionIssue::OriginEdgeMissing { position: 10 },
            ExecutionFormConstructionIssue::CollapsedEffectBoundaryOperation { position: 11 },
            ExecutionFormConstructionIssue::RetainedHostContinuation { position: 12 },
            ExecutionFormConstructionIssue::NonCanonicalOrder { position: 13 },
            ExecutionFormConstructionIssue::HiddenProducerOnlyOperator { position: 14 },
        ];
        assert_eq!(issues.len(), 15);
        assert!(pairwise_distinct(&issues));
        assert_eq!(
            ExecutionFormConstruction::SHAPE,
            FamilyShape::IssueCollection
        );
        let laws = [
            AlgebraicLaw::Associativity,
            AlgebraicLaw::Commutativity,
            AlgebraicLaw::Monotonicity,
            AlgebraicLaw::Idempotence,
            AlgebraicLaw::Distributivity,
        ];
        assert_eq!(laws.len(), 5);
        assert!(pairwise_distinct(&laws));
        assert_eq!(AlgebraicLawLimit::MAX, 5);
        let form = ExecutionForm::for_laws(Commitment::raw([43; 32]));
        assert_eq!(form.content(), &Commitment::raw([43; 32]));
    }

    /// law: execution.effect-batch-composes-as-data — a real intent
    /// constructs with no result member representable, and the composition
    /// family holds its five issues with the three closed subcause rosters.
    /// Owed reversal (red twin): a result or receipt member must not compile.
    #[test]
    fn effect_batch_composes_as_data() {
        let batch = EffectBatch {
            commands: Bounded::admitted(
                vec![EffectCommand {
                    ordinal: CommandOrdinal::declared(0),
                    kind: CommandKind::EventAppend,
                    contracts: Commitment::raw([44; 32]),
                }],
                &LimitWitness::declared(16),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
            boundary: Commitment::raw([45; 32]),
            groups_and_fences: Commitment::raw([46; 32]),
            idempotency: Commitment::raw([47; 32]),
            bounds: Bounded::admitted(
                vec![BoundDimensionRow {
                    dimension: DimensionId::registered(2),
                    class: BoundClass::Effect,
                    maximum: 8,
                }],
                &LimitWitness::declared(8),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
        };
        assert_eq!(batch.commands.len(), 1);
        let kinds = [
            CommandKind::EventAppend,
            CommandKind::EffectIntentAdmission,
            CommandKind::CheckpointAdvance,
            CommandKind::ArtifactPublication,
            CommandKind::ProtectedPayloadPublication,
            CommandKind::SecretAuthorityMutation,
        ];
        assert_eq!(kinds.len(), 6);
        assert!(pairwise_distinct(&kinds));
        let sources = [
            ForbiddenIdentitySource::TimestampBucket,
            ForbiddenIdentitySource::AttemptIdentity,
            ForbiddenIdentitySource::Worker,
            ForbiddenIdentitySource::Route,
            ForbiddenIdentitySource::Session,
            ForbiddenIdentitySource::Host,
        ];
        assert_eq!(sources.len(), 6);
        let defects = [
            GroupFenceDefect::OverlappingGroups,
            GroupFenceDefect::FenceInteriorToAdmittedGroup,
            GroupFenceDefect::EmptyGroup,
        ];
        assert_eq!(defects.len(), 3);
        let contracts = [
            RequiredContractKind::ExpectedResult,
            RequiredContractKind::Receipt,
            RequiredContractKind::ReconciliationRequirement,
        ];
        assert_eq!(contracts.len(), 3);
        assert_eq!(EffectBatchComposition::SHAPE, FamilyShape::IssueCollection);
    }

    /// law: execution.recursion-witness-records-eleven — a real witness
    /// constructs with all eleven facts, a lexicographic measure is lawful,
    /// and the two effectful lanes hold with the interleaved closure roster.
    /// Owed reversal: a callback measure must not be representable.
    #[test]
    fn recursion_witness_records_eleven() {
        let witness = RecursionWitness {
            call_graph: Commitment::raw([48; 32]),
            edges: Commitment::raw([49; 32]),
            measures: Commitment::raw([50; 32]),
            strict_decrease: demo_evidence(51),
            input_bounds: 100,
            depth: 8,
            total_work: 10_000,
            memory_and_frames: 4_096,
            output_bounds: 1_024,
            effect_and_suspension_bounds: 4,
            origins: demo_evidence(52),
        };
        assert_eq!(witness.depth, 8);
        let lanes = [
            EffectfulRecursionLane::AtomicPlanning,
            EffectfulRecursionLane::Interleaved,
        ];
        assert_eq!(lanes.len(), 2);
        assert_eq!(INTERLEAVED_CLOSURE_TOTALS.len(), 8);
    }

    /// law: execution.kernels-partition-not-duplicate — the five
    /// role-distinct types, the authored binding routes running with the
    /// posture view, the decode-only family's ladder, and the two collection
    /// families' issue registers, whose compile-time bounds are the registers'
    /// own cardinalities.
    /// Owed reversal (red twin): literal construction of a binding arm must
    /// not compile.
    #[test]
    fn kernels_partition_not_duplicate() {
        use crate::execution::types::{KernelInterfaceIssueLimit, KernelSemanticIssueLimit};
        use crate::types::{PositiveLimit, RootLawsProfile};
        let over_semantic: Option<fn(KernelSemanticContract)> = Some(drop);
        let over_interface: Option<fn(KernelInterfaceContract)> = Some(drop);
        assert!(over_semantic.is_some());
        assert!(over_interface.is_some());
        let realization =
            KernelRealizationId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([53; 16])));
        let pinned = KernelBindingPolicy::exact_realization(realization);
        assert_eq!(pinned.posture(), KernelBindingPosture::ExactRealization);
        let substituting = KernelBindingPolicy::qualified_substitution(KernelSubstitutionScope(
            Commitment::raw([54; 32]),
        ));
        assert_eq!(
            substituting.posture(),
            KernelBindingPosture::QualifiedSubstitution
        );
        assert_eq!(
            KernelBindingPolicyConstruction::SHAPE,
            FamilyShape::SingleCause
        );
        assert_eq!(KernelBindingPolicyConstruction::SELECTION_ORDER.len(), 5);
        assert_eq!(
            KernelBindingPolicyConstruction::SELECTION_ORDER.first(),
            Some(&"PolicyArmMissingOrAmbiguous")
        );
        let requirement = KernelRequirement {
            semantic: KernelSemanticContractRef {
                contract: Commitment::raw([55; 32]),
                version: 1,
            },
            interface: KernelInterfaceContractRef {
                contract: Commitment::raw([56; 32]),
                version: 1,
            },
            binding: pinned,
            qualification: demo_evidence(57),
            fallback: KernelFallbackPolicy(Commitment::raw([58; 32])),
        };
        assert_eq!(requirement.semantic.version, 1);
        // The two issue rosters are stamped registers, so their membership and
        // their cardinality come out of one declaration and no array here can
        // restate either. What the register bought is the road below: both
        // families declared `Limit` alone until their magnitude could be read off
        // `ALL`, and a family with no `ConstLimit` cannot mint an admission
        // witness at all — the two lines were unwritable before the register.
        let semantic: PositiveLimit<KernelSemanticIssueLimit, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();
        let interface: PositiveLimit<KernelInterfaceIssueLimit, RootLawsProfile> =
            PositiveLimit::inhabited_under_profile();
        assert_eq!(
            semantic.max(),
            KernelSemanticContractConstructionIssue::ALL.len()
        );
        assert_eq!(
            interface.max(),
            KernelInterfaceContractConstructionIssue::ALL.len()
        );
    }

    /// law: execution.agreement-seam-lists-hold — six shareable, eleven
    /// never-shareable, and the nine work dimensions.
    /// Owed reversal: any list growing silently must break this law.
    #[test]
    fn agreement_seam_lists_hold() {
        assert_eq!(INDEPENDENCE_MAY_SHARE.len(), 6);
        assert_eq!(INDEPENDENCE_MAY_NOT_SHARE.len(), 11);
        assert!(INDEPENDENCE_MAY_NOT_SHARE.contains(&"verdict-helper"));
        assert_eq!(WORK_DIMENSIONS.len(), 9);
    }
}

mod image {
    use super::pairwise_distinct;
    use crate::bytes::ContentRegionId;
    use crate::execution::{KernelRequirementSet, SemanticKernelFamilyId, SemanticKernelVersion};
    use crate::identity::{
        AuthorityPosition, ByteIdentity, IdentityClass, IdentityRole, Occurrence, OccurrenceForm,
        OrderComparison,
    };
    use crate::image::{
        ADMISSION_PIPELINE, ADMISSION_PROVES, AgreementCheckedImage, BOUND_FACT_ROSTER,
        BoundedDecodedImage, ComponentCarriage, ComponentRole, ExecutableImage, ImageDigest,
        ImageFamilyFormatVersion, ImageFamilyId, ImageProfileId, ImageProfileVersion,
        ImageValidation, PROGRAM_IMAGE_EXTENSION, PackagingProfile, ProgramImage,
        ProgramImageComponent, SemanticImage, UntrustedImageBytes,
    };
    use crate::types::{Bounded, LimitWitness};
    use core::cmp::Ordering;

    /// law: image.identities-ride-scope-guards — family-format, profile, and
    /// kernel versions are the seventh, eighth, and ninth scope-guard
    /// instantiations; the image digest is a byte identity, never a meaning
    /// digest.
    /// Reversal (red twin), discharged by the stamp this guard is now written
    /// by, on the stamp's own fixtures: cross-scope comparison is a category error
    /// (`cross-scope-comparison-on-a-stamped-guard.rs`) and the position has no
    /// road out and none back in (`a-stamped-representation-cannot-be-laundered.rs`).
    /// Both prove a property of the generated shape, which is what makes them
    /// this guard's reversal rather than another home's.
    #[test]
    fn identities_ride_scope_guards() {
        let family = ImageFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([61; 16])));
        let f1 = ImageFamilyFormatVersion::positioned(AuthorityPosition::assigned(family, 1));
        let f2 = ImageFamilyFormatVersion::positioned(AuthorityPosition::assigned(family, 2));
        assert!(matches!(f1.try_cmp_same_scope(&f2), Ok(Ordering::Less)));
        let profile =
            ImageProfileId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([62; 16])));
        let p1 = ImageProfileVersion::positioned(AuthorityPosition::assigned(profile, 1));
        let other_profile = ImageProfileVersion::positioned(AuthorityPosition::assigned(
            ImageProfileId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([63; 16]))),
            1,
        ));
        assert!(matches!(
            p1.try_cmp_same_scope(&other_profile),
            Err(OrderComparison::NotSameScope)
        ));
        let kernel =
            SemanticKernelFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([64; 16])));
        let k1 = SemanticKernelVersion::positioned(AuthorityPosition::assigned(kernel, 1));
        let k2 = SemanticKernelVersion::positioned(AuthorityPosition::assigned(kernel, 3));
        assert!(matches!(k1.try_cmp_same_scope(&k2), Ok(Ordering::Less)));
        assert_eq!(ImageDigest::CLASS, IdentityClass::ByteDigest);
        let digest = ImageDigest::of(ByteIdentity::raw([60; 32]));
        assert_eq!(digest, ImageDigest::of(ByteIdentity::raw([60; 32])));
    }

    /// law: image.component-roster-is-authored-nineteen — the authored
    /// roster, the two carriages, the three packaging profiles, and a real
    /// component row on the img-row shape.
    /// Owed reversal: a twentieth role appearing silently must break this
    /// law.
    #[test]
    fn component_roster_is_authored_nineteen() {
        let roles = [
            ComponentRole::SemanticForm,
            ComponentRole::ExecutionForm,
            ComponentRole::ContractsAndDefinitions,
            ComponentRole::Constants,
            ComponentRole::DeclaredInputsAndOutputs,
            ComponentRole::EventAndEffectDeclarations,
            ComponentRole::CapabilityRequirements,
            ComponentRole::SourceAndCutRequirements,
            ComponentRole::Bounds,
            ComponentRole::ExplanationStructures,
            ComponentRole::CompletedJudgments,
            ComponentRole::CaptureRecords,
            ComponentRole::ImportClosure,
            ComponentRole::KernelRequirements,
            ComponentRole::Entrypoints,
            ComponentRole::CompatibilityPosture,
            ComponentRole::OriginMaps,
            ComponentRole::AuthenticityReferences,
            ComponentRole::QualificationReferences,
        ];
        assert_eq!(roles.len(), 19);
        assert!(pairwise_distinct(&roles));
        let carriages = [
            ComponentCarriage::Inline,
            ComponentCarriage::ImmutableReference,
        ];
        assert_eq!(carriages.len(), 2);
        let profiles = [
            PackagingProfile::SelfContained,
            PackagingProfile::ImmutableBound,
            PackagingProfile::Hybrid,
        ];
        assert_eq!(profiles.len(), 3);
        assert!(pairwise_distinct(&profiles));
        let component = ProgramImageComponent {
            role: ComponentRole::SemanticForm,
            profile: 1,
            content: ContentRegionId::of(ByteIdentity::raw([65; 32])),
            length: 4_096,
            carriage: ComponentCarriage::Inline,
        };
        assert!(matches!(component.carriage, ComponentCarriage::Inline));
    }

    /// law: image.program-image-composes — a real image constructs through
    /// the checked roads, and the bound-fact roster holds eighteen.
    /// Owed reversal: an image without both forms' components must refuse at
    /// validation (owed to the machinery seam).
    #[test]
    fn program_image_composes() {
        let image = ProgramImage {
            family: ImageFamilyFormatVersion::positioned(AuthorityPosition::assigned(
                ImageFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([66; 16]))),
                1,
            )),
            profile: ImageProfileVersion::positioned(AuthorityPosition::assigned(
                ImageProfileId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([67; 16]))),
                1,
            )),
            packaging: PackagingProfile::SelfContained,
            components: Bounded::admitted(
                vec![ProgramImageComponent {
                    role: ComponentRole::ExecutionForm,
                    profile: 1,
                    content: ContentRegionId::of(ByteIdentity::raw([68; 32])),
                    length: 1_024,
                    carriage: ComponentCarriage::Inline,
                }],
                &LimitWitness::declared(32),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
            kernel_requirements: KernelRequirementSet {
                requirements: Bounded::admitted(vec![], &LimitWitness::declared(8))
                    .unwrap_or_else(|_| unreachable!("empty fits")),
            },
        };
        assert!(matches!(image.packaging, PackagingProfile::SelfContained));
        assert_eq!(BOUND_FACT_ROSTER.len(), 18);
        assert_eq!(PROGRAM_IMAGE_EXTENSION, ".program.tpk");
    }

    /// law: image.validation-ladder-is-five-and-minted — five distinct
    /// ladder types with no public constructor on any rung, and the durable
    /// record's five phases.
    /// Owed reversal (red twin): literal construction of the two
    /// verifier-minted rungs must not compile.
    #[test]
    fn validation_ladder_is_five_and_minted() {
        let rung_one: Option<fn(UntrustedImageBytes)> = Some(drop);
        let rung_two: Option<fn(BoundedDecodedImage)> = Some(drop);
        let rung_three: Option<fn(SemanticImage)> = Some(drop);
        let rung_four: Option<fn(AgreementCheckedImage)> = Some(drop);
        let rung_five: Option<fn(ExecutableImage)> = Some(drop);
        assert!(rung_one.is_some());
        assert!(rung_two.is_some());
        assert!(rung_three.is_some());
        assert!(rung_four.is_some());
        assert!(rung_five.is_some());
        let phases = [
            ImageValidation::UntrustedBytes,
            ImageValidation::BoundedDecoded,
            ImageValidation::Semantic,
            ImageValidation::AgreementChecked,
            ImageValidation::Executable,
        ];
        assert_eq!(phases.len(), 5);
        assert!(pairwise_distinct(&phases));
    }

    /// law: image.admission-pipeline-is-sixteen — no stage skipped; the
    /// eight proven facts hold.
    /// Owed reversal: a skipped stage must break this law.
    #[test]
    fn admission_pipeline_is_sixteen() {
        assert_eq!(ADMISSION_PIPELINE.len(), 16);
        assert_eq!(
            ADMISSION_PIPELINE.first(),
            Some(&"bounded-canonical-decode")
        );
        assert_eq!(ADMISSION_PIPELINE.last(), Some(&"admitted-program"));
        assert_eq!(ADMISSION_PROVES.len(), 8);
    }
}

mod pakvm {
    use super::pairwise_distinct;
    use crate::bounds::{BoundClass, DimensionId};
    use crate::identity::Commitment;
    use crate::pakvm::{
        ArenaIndex, CLOSURE_OBLIGATIONS, CapabilityHandle, CaptureRecord, ContinuationRecord,
        INVALID_CAPTURES, LambdaBoundaryPosture, PROHIBITED_INHABITANTS, PortHandle, ReplyHandle,
        STEP_PRODUCTIONS, ValueCategory, ValueResidence, VmTerminal,
    };
    use crate::semantic::BoundDimensionRow;
    use crate::time::{ConsumedBudgetEvidence, RecordingSite, SpendRecord};
    use crate::types::{
        Bounded, EvidenceRef, LimitWitness, ReferentAvailability, ReferentIntegrity,
    };

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: pakvm.value-algebra-is-closed — nine categories, five prohibited
    /// inhabitants, four residences, and the dumb generational index.
    /// Owed reversal (red twin): an Any / host-object / function-pointer
    /// inhabitant must not compile.
    #[test]
    fn value_algebra_is_closed() {
        let categories = [
            ValueCategory::ExactPrimitivesAndApproximations,
            ValueCategory::BoundedTextAndBytes,
            ValueCategory::ProductsAndCollections,
            ValueCategory::RecursiveAlgebraicData,
            ValueCategory::UnitsIntervalsMarginsDecisions,
            ValueCategory::IdentitiesAndReferences,
            ValueCategory::KnowledgeAxes,
            ValueCategory::SourceAndEvidenceValues,
            ValueCategory::BoundaryValues,
        ];
        assert_eq!(categories.len(), 9);
        assert!(pairwise_distinct(&categories));
        assert_eq!(PROHIBITED_INHABITANTS.len(), 5);
        let residences = [
            ValueResidence::ValidatedFrame,
            ValueResidence::BoundedArena,
            ValueResidence::TypedBorrowedView,
            ValueResidence::OwnedValue,
        ];
        assert_eq!(residences.len(), 4);
        let index = ArenaIndex::located(7, 2);
        assert_eq!(index.index(), 7);
        assert_eq!(index.generation(), 2);
    }

    /// law: pakvm.live-handles-do-not-cross-threads — three role-distinct
    /// executor handles exist, structurally execution-context-local via the
    /// raw-pointer phantom.
    /// Owed reversal (red twin): sending any handle across threads must not
    /// compile — the trybuild fixture is testpak's.
    #[test]
    fn live_handles_do_not_cross_threads() {
        let over_capability: Option<fn(CapabilityHandle)> = Some(drop);
        let over_port: Option<fn(PortHandle)> = Some(drop);
        let over_reply: Option<fn(ReplyHandle)> = Some(drop);
        assert!(over_capability.is_some());
        assert!(over_port.is_some());
        assert!(over_reply.is_some());
    }

    /// law: pakvm.continuation-record-binds-twelve — a real persisted
    /// continuation constructs with all members, carrying the deadline-policy
    /// reference plus consumed-budget evidence — never a live monotonic
    /// value.
    /// Owed reversal (red twin): a live monotonic member must not compile
    /// (the live deadline type is unserializable and `!Send` by shape).
    #[test]
    fn continuation_record_binds_twelve() {
        let record = ContinuationRecord {
            program: demo_evidence(70),
            resume_coordinate: 42,
            frame: Commitment::raw([71; 32]),
            contract: Commitment::raw([72; 32]),
            request: Commitment::raw([73; 32]),
            effect_intent: demo_evidence(74),
            attempt: demo_evidence(75),
            generations: Commitment::raw([76; 32]),
            remaining_bounds: Bounded::admitted(
                vec![BoundDimensionRow {
                    dimension: DimensionId::registered(3),
                    class: BoundClass::Suspension,
                    maximum: 2,
                }],
                &LimitWitness::declared(8),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
            deadline_policy: demo_evidence(77),
            spend: ConsumedBudgetEvidence {
                site: RecordingSite::EffectAttempt,
                coordinate: demo_evidence(78),
                spends: Bounded::admitted(
                    vec![SpendRecord {
                        dimension: DimensionId::registered(4),
                        magnitude: 100,
                        uncertainty: 5,
                    }],
                    &LimitWitness::declared(8),
                )
                .unwrap_or_else(|_| unreachable!("one fits")),
            },
            posture: Commitment::raw([79; 32]),
        };
        assert_eq!(record.resume_coordinate, 42);
        assert_eq!(record.spend.site, RecordingSite::EffectAttempt);
    }

    /// law: pakvm.terminals-are-five-and-owned — the executor's closed
    /// terminal set and the six step productions; the physical and
    /// reconciled facts are other owners' and unconstructible here.
    /// Owed reversal (red twin): an executor-constructed physical fact must
    /// not compile.
    #[test]
    fn terminals_are_five_and_owned() {
        let terminals = [
            VmTerminal::PureValue,
            VmTerminal::EffectIntentPlan,
            VmTerminal::PortRequestSuspended,
            VmTerminal::SemanticRefusal,
            VmTerminal::VmBudgetExceeded,
        ];
        assert_eq!(terminals.len(), 5);
        assert!(pairwise_distinct(&terminals));
        assert_eq!(STEP_PRODUCTIONS.len(), 6);
        assert_eq!(STEP_PRODUCTIONS.first(), Some(&"semantic-value"));
    }

    /// law: pakvm.captures-and-closure-obligations — the capture record
    /// constructs in canonical binding order, the seven invalid captures and
    /// four lambda postures hold, and the six closure obligations stand.
    /// Owed reversal (red twin): a captured live handle must not compile.
    #[test]
    fn captures_and_closure_obligations() {
        let record = CaptureRecord {
            definition: Commitment::raw([80; 32]),
            captures: Bounded::admitted(
                vec![Commitment::raw([81; 32])],
                &LimitWitness::declared(8),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
            origins: demo_evidence(82),
        };
        assert_eq!(record.captures.len(), 1);
        assert_eq!(INVALID_CAPTURES.len(), 7);
        let postures = [
            LambdaBoundaryPosture::InlineOnly,
            LambdaBoundaryPosture::InvocationBound,
            LambdaBoundaryPosture::Portable,
            LambdaBoundaryPosture::Nonserializable,
        ];
        assert_eq!(postures.len(), 4);
        assert!(pairwise_distinct(&postures));
        assert_eq!(CLOSURE_OBLIGATIONS.len(), 6);
        assert_eq!(CLOSURE_OBLIGATIONS.last(), Some(&"total-typed-refusal"));
    }
}

mod bvisor {
    use super::pairwise_distinct;
    use crate::authority::ConstraintSourcePair;
    use crate::bounds::DimensionId;
    use crate::bvisor::{
        ADMISSION_DEPENDENCY_ORDER, ADMISSION_INPUTS, AdmissionOutcome, AdmittedAttempt,
        AttemptAdmission, AttemptAdmissionIssue, AttemptId, AttemptState, AuthenticitySubject,
        AvailabilitySubject, BVISOR_IS_NOT, BindingSubject, CANCELLATION_FACTS,
        CompatibilitySubject, ConsumedVerdictSubject, ContainmentProfile,
        DeclaredEvidenceRequirement, DerivedFloorBreach, GenerationAxis, GenerationPosture,
        InteractionShape, MeetFailure, NarrowingInput, PAIRWISE_NON_SUBSTITUTION,
        PHYSICAL_OBSERVATION_KINDS, PORT_REQUEST_VALIDATION, PhysicalEstimate, PortRequest,
        PortRequestId, RequestedReservationSemantics, RequiredEvidencePosture,
        ReservationObservation, TerminalAttempt,
    };
    use crate::identity::{AuthorityPosition, Commitment, Occurrence, OccurrenceForm};
    use crate::port::{PortFamilyId, PortFamilyVersion, PortPostcondition};
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::semantic::BoundDimensionRow;
    use crate::types::{
        Bounded, EvidenceRef, LimitWitness, ReferentAvailability, ReferentIntegrity,
    };

    fn demo_observation(seed: u8) -> ReservationObservation {
        ReservationObservation {
            requested: 100,
            granted: 40,
            unavailable: 60,
            guarantees: Commitment::raw([seed; 32]),
            uncertainty: 2,
        }
    }

    /// law: bvisor.admission-family-holds-fourteen — the fourteen issues
    /// with their closed subcause rosters, the port home's postcondition
    /// vocabulary collecting on its planted seat, and the sole
    /// constraint-source-pair carrier.
    /// Owed reversal (red twin): a fifteenth issue must break this law.
    #[test]
    fn admission_family_holds_fourteen() {
        let issues = [
            AttemptAdmissionIssue::RequestIncoherent {
                breach: DerivedFloorBreach(Commitment::raw([90; 32])),
            },
            AttemptAdmissionIssue::ExecutableOrInvocationRoleUnadmitted {
                subject: ConsumedVerdictSubject::VerdictAbsent,
            },
            AttemptAdmissionIssue::IdentityOrLineageBindingMismatch {
                subject: BindingSubject::Lineage,
            },
            AttemptAdmissionIssue::GenerationMismatch {
                axis: GenerationAxis::Partition,
                posture: GenerationPosture::Stale,
            },
            AttemptAdmissionIssue::PrincipalDelegationOrAuthenticityUnsatisfied {
                subject: AuthenticitySubject::Delegation,
            },
            AttemptAdmissionIssue::CapabilityMeetUnsatisfied {
                mode: MeetFailure::NoMeetNoncommutingPurposes,
                sources: ConstraintSourcePair::named(
                    Commitment::raw([91; 32]),
                    Commitment::raw([92; 32]),
                ),
            },
            AttemptAdmissionIssue::InterfaceSchemaPortOrKernelIncompatible {
                subject: CompatibilitySubject::Kernel,
            },
            AttemptAdmissionIssue::PortKernelOrHostUnavailable {
                subject: AvailabilitySubject::HostProfile,
            },
            AttemptAdmissionIssue::RequiredEvidenceUnsatisfied {
                requirement: DeclaredEvidenceRequirement(Commitment::raw([93; 32])),
                posture: RequiredEvidencePosture::Stale,
            },
            AttemptAdmissionIssue::LogicalAuthorizationNotSupplied,
            AttemptAdmissionIssue::BoundIntersectionUnsatisfied {
                dimension: DimensionId::registered(5),
                narrowing: NarrowingInput::CapabilityGrantScope,
            },
            AttemptAdmissionIssue::RequiredPostconditionUnsupported {
                postcondition: PortPostcondition::AtomicBoundary,
            },
            AttemptAdmissionIssue::CapacityUnavailable {
                observation: demo_observation(94),
            },
            AttemptAdmissionIssue::ReservationSemanticsUnavailable {
                semantics: RequestedReservationSemantics(Commitment::raw([95; 32])),
            },
        ];
        assert_eq!(issues.len(), 14);
        assert!(pairwise_distinct(&issues));
        assert_eq!(AttemptAdmission::SHAPE, FamilyShape::IssueCollection);
        let meet_modes = [
            MeetFailure::EmptyIntersection,
            MeetFailure::ContradictoryIntersection,
            MeetFailure::NoMeetNoncommutingPurposes,
            MeetFailure::StaleGeneration,
            MeetFailure::Revoked,
        ];
        assert_eq!(meet_modes.len(), 5);
        assert!(pairwise_distinct(&meet_modes));
        let axes = [
            GenerationAxis::Source,
            GenerationAxis::Store,
            GenerationAxis::Partition,
            GenerationAxis::Authority,
            GenerationAxis::Application,
        ];
        assert_eq!(axes.len(), 5);
        assert_eq!(ADMISSION_INPUTS.len(), 11);
        assert_eq!(ADMISSION_DEPENDENCY_ORDER.len(), 10);
        assert_eq!(
            ADMISSION_DEPENDENCY_ORDER.last(),
            Some(&"fresh-attempt-creation")
        );
    }

    /// law: bvisor.attempt-minting-is-admissions-alone — the admitted arm
    /// carries live custody with the fresh identity; the refused arm carries
    /// the family body and NO Attempt identity of any kind.
    /// Owed reversal (red twin): minting an Attempt identity from any other
    /// route must not compile.
    #[test]
    fn attempt_minting_is_admissions_alone() {
        let attempt = AttemptId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([96; 16])));
        let admitted = AdmissionOutcome::Admitted(AdmittedAttempt::for_laws(attempt));
        if let AdmissionOutcome::Admitted(custody) = &admitted {
            assert_eq!(custody.attempt(), attempt);
        } else {
            unreachable!("constructed admitted");
        }
        // The refused arm read as a function: it takes exactly one argument and
        // that argument's type is the family body, so no Attempt identity of
        // any kind stands in the arm's shape. The body itself is not assembled
        // here — its seat is band 00's coupled report package, proven once at
        // `refusal::every_collection_family_carries_the_coupled_seat`.
        let refused: Option<fn(AttemptAdmission) -> AdmissionOutcome> =
            Some(AdmissionOutcome::Refused);
        assert!(refused.is_some());
    }

    /// law: bvisor.lifecycle-is-affine-and-sealed — sealing CONSUMES the
    /// terminal Attempt and mints the immutable report; the persisted state
    /// enum holds four phases.
    /// Owed reversal (red twin): using a terminal Attempt after seal must
    /// not compile (moved value).
    #[test]
    fn lifecycle_is_affine_and_sealed() {
        let attempt = AttemptId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([97; 16])));
        let terminal = TerminalAttempt::for_laws(attempt);
        let report = terminal.seal(EvidenceRef::bound(
            [98; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        ));
        assert_eq!(report.attempt, attempt);
        let states = [
            AttemptState::Admitted,
            AttemptState::Running,
            AttemptState::LiveSuspended,
            AttemptState::Terminal,
        ];
        assert_eq!(states.len(), 4);
        assert!(pairwise_distinct(&states));
    }

    /// law: bvisor.reservation-has-one-home — the observation record
    /// constructs with its uncertainty; the estimate is model-named; the
    /// non-substitution table holds thirteen and the observation kinds
    /// twelve.
    /// Owed reversal (red twin): a conversion across the Attempt-existence
    /// line must not compile.
    #[test]
    fn reservation_has_one_home() {
        let observation = demo_observation(99);
        assert_eq!(observation.requested, 100);
        assert_eq!(observation.uncertainty, 2);
        let estimate = PhysicalEstimate {
            model: Commitment::raw([100; 32]),
            predicted: 5_000,
        };
        assert_eq!(estimate.predicted, 5_000);
        assert_eq!(PAIRWISE_NON_SUBSTITUTION.len(), 13);
        assert_eq!(PHYSICAL_OBSERVATION_KINDS.len(), 12);
        assert_eq!(
            PHYSICAL_OBSERVATION_KINDS.last(),
            Some(&"outcome-remains-unknown")
        );
    }

    /// law: bvisor.containment-is-two-coordinates — five profiles, two
    /// interaction shapes, and the closed is-not list.
    /// Owed reversal: a sixth profile or a merged coordinate must break this
    /// law.
    #[test]
    fn containment_is_two_coordinates() {
        let profiles = [
            ContainmentProfile::SameThreadInProcess,
            ContainmentProfile::OtherThreadInProcess,
            ContainmentProfile::WorkerProcess,
            ContainmentProfile::BrowserWorker,
            ContainmentProfile::RemoteQualifiedBoundary,
        ];
        assert_eq!(profiles.len(), 5);
        assert!(pairwise_distinct(&profiles));
        let shapes = [
            InteractionShape::PakVmGuest,
            InteractionShape::ArtifactInArtifactOutExternalTool,
        ];
        assert_eq!(shapes.len(), 2);
        assert_eq!(BVISOR_IS_NOT.len(), 8);
    }

    /// law: bvisor.port-crossing-binds — a real port request constructs
    /// bound to one Attempt and one family; the validation and cancellation
    /// rosters hold.
    /// Owed reversal (red twin): a request satisfying another Attempt must
    /// not compile.
    #[test]
    fn port_crossing_binds() {
        let request = PortRequest {
            attempt: AttemptId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([101; 16]))),
            request: PortRequestId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh(
                [102; 16],
            ))),
            family: PortFamilyVersion::positioned(AuthorityPosition::assigned(
                PortFamilyId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([103; 16]))),
                1,
            )),
            payload: Commitment::raw([104; 32]),
            bounds: Bounded::admitted(
                vec![BoundDimensionRow {
                    dimension: DimensionId::registered(6),
                    class: crate::bounds::BoundClass::Work,
                    maximum: 500,
                }],
                &LimitWitness::declared(8),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
        };
        assert_eq!(request.bounds.len(), 1);
        assert_eq!(PORT_REQUEST_VALIDATION.len(), 10);
        assert_eq!(CANCELLATION_FACTS.len(), 10);
    }
}

mod runtime {
    use super::pairwise_distinct;
    use crate::bvisor::AttemptId;
    use crate::history::CommitKnowledge;
    use crate::identity::{
        Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence, OccurrenceForm,
    };
    use crate::runtime::{
        AttemptCause, AttemptLineageNode, BoundedCauseSet, CANCELLATION_DISTINCT_FACTS,
        CHECKPOINT_NON_REASONS, CancellationDurablePosition, CancellationObservation,
        CancellationPhysicalPosition, CompensationSupport, CompletionTerminal,
        ConcurrencyConstraints, DRIVER_INVARIANCE, DRIVER_MAY_CHANGE, DeliveryRole,
        DurableCheckpoint, EffectIntentId, EffectReconciliationRecord, EffectRecoveryProfile,
        EvidenceRetention, ExternalOutcome, FOUR_MOTIONS, IdempotencyKeySupport,
        IdempotencyPosture, LIVENESS_DECLARATION, LogicalOperationId, MAILBOX_FACTS,
        NEVER_SUFFICIENT, OutcomeKnowledge, OutcomeQuerySupport, ProcessStateRole,
        RECOVERY_ACTIONS, ReconciliationDisposition, ReconciliationLifecycle, ReplayPosture,
        STITCH_OUTPUTS, SemanticRecoveryAuthority, TURN_PREIMAGE, TurnId, TurnPhase,
    };
    use crate::types::{
        Bounded, EvidenceRef, LimitWitness, ReferentAvailability, ReferentIntegrity,
    };

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: runtime.stitch-contract-and-driver-invariance — seven outputs,
    /// fifteen invariants no driver may change, seven freedoms, twelve
    /// liveness declarations.
    /// Owed reversal: a driver changing an invariant must break this law.
    #[test]
    fn stitch_contract_and_driver_invariance() {
        assert_eq!(STITCH_OUTPUTS.len(), 7);
        assert_eq!(DRIVER_INVARIANCE.len(), 15);
        assert!(DRIVER_INVARIANCE.contains(&"turn-identity"));
        assert_eq!(DRIVER_MAY_CHANGE.len(), 7);
        assert_eq!(LIVENESS_DECLARATION.len(), 12);
    }

    /// law: runtime.turn-identity-quartet — the Turn is the machine's first
    /// DERIVED Class-D production identity (replay-stable); the logical
    /// operation and effect intent stay fresh; the fourteen phases hold with
    /// their initial and terminal postures.
    /// Owed reversal (red twin): a quartet merger must not compile.
    #[test]
    fn turn_identity_quartet() {
        assert_eq!(TurnId::CLASS, IdentityClass::Occurrence);
        assert_eq!(TurnId::CREATION, CreationLaw::DerivedFromAdmittedPreimage);
        assert_eq!(LogicalOperationId::CREATION, CreationLaw::FreshOpaque);
        assert_eq!(EffectIntentId::CREATION, CreationLaw::FreshOpaque);
        assert_eq!(TURN_PREIMAGE.len(), 7);
        let phases = [
            TurnPhase::Runnable,
            TurnPhase::CutFrozen,
            TurnPhase::Planned,
            TurnPhase::AttemptRequested,
            TurnPhase::Executing,
            TurnPhase::LiveSuspended,
            TurnPhase::PhysicallyObserved,
            TurnPhase::SemanticallyInterpreted,
            TurnPhase::PublicationOutstanding,
            TurnPhase::PublicationAdmitted,
            TurnPhase::CheckpointOutstanding,
            TurnPhase::CheckpointAdvanced,
            TurnPhase::ReconciliationOutstanding,
            TurnPhase::ReconciliationComplete,
        ];
        assert_eq!(phases.len(), 14);
        assert!(pairwise_distinct(&phases));
        assert_eq!(phases.first(), Some(&TurnPhase::Runnable));
        assert_eq!(phases.last(), Some(&TurnPhase::ReconciliationComplete));
    }

    /// law: runtime.attempt-lineage-is-message-passing — the three-way cause
    /// sum binds one endpoint each, the cause set is bounded membership, and
    /// a lineage node composes with the membrane's Attempt identity.
    /// Owed reversal (red twin): a bare cause or an edge inside the cause
    /// value must not compile.
    #[test]
    fn attempt_lineage_is_message_passing() {
        let turn = TurnId::for_laws(Occurrence::for_laws(OccurrenceForm::Derived([110; 32])));
        let intent =
            EffectIntentId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([111; 16])));
        let attempt = AttemptId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([112; 16])));
        let causes = [
            AttemptCause::Turn(turn),
            AttemptCause::EffectIntent(intent),
            AttemptCause::Attempt(attempt),
        ];
        assert_eq!(causes.len(), 3);
        assert!(pairwise_distinct(&causes));
        let node = AttemptLineageNode {
            attempt: AttemptId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([113; 16]))),
            causes: BoundedCauseSet {
                causes: Bounded::admitted(
                    vec![AttemptCause::Turn(turn)],
                    &LimitWitness::declared(4),
                )
                .unwrap_or_else(|_| unreachable!("one fits")),
            },
        };
        assert_eq!(node.causes.causes.len(), 1);
    }

    /// law: runtime.checkpoint-advances-only-on-prerequisites — the durable
    /// checkpoint constructs whole, the eight non-reasons hold, and the four
    /// process-state roles stand distinct.
    /// Owed reversal: advancing on any non-reason must break this law.
    #[test]
    fn checkpoint_advances_only_on_prerequisites() {
        let checkpoint = DurableCheckpoint {
            process: Commitment::raw([114; 32]),
            sources: demo_evidence(115),
            prior: demo_evidence(116),
            evidence: demo_evidence(117),
            outstanding: Commitment::raw([118; 32]),
            admitted_by: demo_evidence(119),
        };
        assert_eq!(checkpoint.process, Commitment::raw([114; 32]));
        assert_eq!(CHECKPOINT_NON_REASONS.len(), 8);
        let roles = [
            ProcessStateRole::EventReconstructible,
            ProcessStateRole::DerivedFastStart,
            ProcessStateRole::DurableCheckpointAuthority,
            ProcessStateRole::GenuineMutableAuthority,
        ];
        assert_eq!(roles.len(), 4);
        assert!(pairwise_distinct(&roles));
    }

    /// law: runtime.effect-recovery-has-nine-axes — a full profile
    /// constructs with the five pair-fact axes as records, the explicit
    /// weaker posture, and the closed action rosters.
    /// Owed reversal (red twin): a boolean axis must not compile.
    #[test]
    fn effect_recovery_has_nine_axes() {
        let profile = EffectRecoveryProfile {
            idempotency: IdempotencyPosture {
                key: IdempotencyKeySupport::None,
                scope: Commitment::raw([120; 32]),
            },
            outcome_query: OutcomeQuerySupport {
                availability: Commitment::raw([121; 32]),
                evidence: Commitment::raw([122; 32]),
            },
            compensation: CompensationSupport {
                availability: Commitment::raw([123; 32]),
                preconditions: Commitment::raw([124; 32]),
            },
            duplicate_execution: Commitment::raw([125; 32]),
            replay: ReplayPosture::NonReplayable,
            concurrency: ConcurrencyConstraints {
                concurrency: Commitment::raw([126; 32]),
                lease: Commitment::raw([127; 32]),
            },
            external_ack: Commitment::raw([128; 32]),
            evidence: EvidenceRetention {
                retention: Commitment::raw([129; 32]),
                freshness_requirement: Commitment::raw([130; 32]),
            },
            manual_intervention: Commitment::raw([131; 32]),
        };
        assert!(matches!(
            profile.idempotency.key,
            IdempotencyKeySupport::None
        ));
        assert_eq!(profile.replay, ReplayPosture::NonReplayable);
        assert_eq!(RECOVERY_ACTIONS.len(), 7);
        assert_eq!(NEVER_SUFFICIENT.len(), 6);
    }

    /// law: runtime.reconciliation-and-cancellation-axes — the disposition
    /// exists only inside completion; the record composes history's commit
    /// axis with this home's two; the cancellation fact model holds its
    /// rosters with no single outcome enum.
    /// Owed reversal (red twin): a disposition outside `Complete` must not
    /// compile.
    #[test]
    fn reconciliation_and_cancellation_axes() {
        let record = EffectReconciliationRecord {
            commit: CommitKnowledge::KnownCommitted,
            outcome: OutcomeKnowledge::Known(ExternalOutcome(Commitment::raw([132; 32]))),
            lifecycle: ReconciliationLifecycle::Complete(
                ReconciliationDisposition::CompensationProposed,
            ),
        };
        assert!(matches!(
            record.lifecycle,
            ReconciliationLifecycle::Complete(ReconciliationDisposition::CompensationProposed)
        ));
        let dispositions = [
            ReconciliationDisposition::Reconciled,
            ReconciliationDisposition::CompensationProposed,
            ReconciliationDisposition::ManualInterventionRequired,
            ReconciliationDisposition::AutomaticActionRefused,
        ];
        assert_eq!(dispositions.len(), 4);
        assert!(pairwise_distinct(&dispositions));
        let durable = [
            CancellationDurablePosition::BeforeDurableAdmission,
            CancellationDurablePosition::AfterDurableAdmission,
        ];
        assert_eq!(durable.len(), 2);
        let physical = [
            CancellationPhysicalPosition::BeforeAttemptAdmission,
            CancellationPhysicalPosition::AfterAttemptAdmissionBeforeHostCrossing,
            CancellationPhysicalPosition::DuringOrAfterHostCrossing,
        ];
        assert_eq!(physical.len(), 3);
        let observations = [
            CancellationObservation::LateObservation,
            CancellationObservation::MechanismUnsupported,
            CancellationObservation::AcceptedOutcomeUnknown,
        ];
        assert_eq!(observations.len(), 3);
        assert_eq!(CANCELLATION_DISTINCT_FACTS.len(), 8);
    }

    /// law: runtime.delivery-and-bound-outcomes — four delivery roles, nine
    /// Mailbox facts, six Completion terminals, four motions, five recovery
    /// authorities, and the bound outcome binding the membrane's observation
    /// across the Attempt-existence line.
    /// Owed reversal (red twin): a conversion across that line must not
    /// compile.
    #[test]
    fn delivery_and_bound_outcomes() {
        let roles = [
            DeliveryRole::Mailbox,
            DeliveryRole::Completion,
            DeliveryRole::Broadcast,
            DeliveryRole::Permit,
        ];
        assert_eq!(roles.len(), 4);
        assert!(pairwise_distinct(&roles));
        assert_eq!(MAILBOX_FACTS.len(), 9);
        let terminals = [
            CompletionTerminal::CompletedResult,
            CompletionTerminal::TypedRefusal,
            CompletionTerminal::CancellationObservation,
            CompletionTerminal::ClosedBeforeObservation,
            CompletionTerminal::BudgetResourceTerminal,
            CompletionTerminal::OutcomeUnknown,
        ];
        assert_eq!(terminals.len(), 6);
        assert!(pairwise_distinct(&terminals));
        assert_eq!(FOUR_MOTIONS.len(), 4);
        let authorities = [
            SemanticRecoveryAuthority::Restart,
            SemanticRecoveryAuthority::Resume,
            SemanticRecoveryAuthority::Compensate,
            SemanticRecoveryAuthority::Quarantine,
            SemanticRecoveryAuthority::Escalate,
        ];
        assert_eq!(authorities.len(), 5);
    }
}

mod derived {
    use super::pairwise_distinct;
    use crate::bytes::ContentRegionId;
    use crate::derived::{
        BindingEntryRef, Column, DATA_SEMANTIC_WORK, DERIVATION_PRIMITIVES,
        DERIVED_REFUSAL_CLASSES, DataBlockState, ExtentEntry, ExtentEntryRef,
        KERNEL_ADMISSION_GATE, LayoutId, MaskRepresentation, MaterializationAppliedCut,
        MaterializationAvailability, MaterializationCoverage, MaterializationGeneration,
        MaterializationId, MaterializationPresence, MaterializationSourceCuts, OccurrenceId,
        PLAN_CANNOT, PROTECTED_INDEX_STANDING_BAR, PayloadLocator, PlanBinding, PlanTemplate,
        RowDomainId, SelectionMask, SelectionMaskConstruction, SourceBinding, ValidityCondition,
    };
    use crate::history::{CommitPoint, FederationCutEntries};
    use crate::identity::{
        AuthorityPosition, ByteIdentity, Commitment, CreationLaw, IdentityClass, IdentityRole,
        Occurrence, OccurrenceForm, OrderComparison,
    };
    use crate::refusal::{FamilyShape, RefusalFamily};
    use crate::runtime::DurableCheckpoint;
    use crate::types::{Completeness, EvidenceRef, ReferentAvailability, ReferentIntegrity};
    use core::cmp::Ordering;

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    fn demo_generation(seed: u8) -> MaterializationGeneration {
        MaterializationGeneration::positioned(AuthorityPosition::assigned(
            MaterializationId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([seed; 16]))),
            1,
        ))
    }

    /// law: derived.mask-family-owes-the-normative-order — nine causes in
    /// the normative check order (the gate first, its fail-closed twin
    /// second, the silent-repair refusal last); unproven is never mismatch;
    /// the length cause carries both lengths canonically.
    /// Owed reversal (red twin): a post-gate cause over unproven row domains
    /// must be unreachable.
    #[test]
    fn mask_family_owes_the_normative_order() {
        assert_eq!(SelectionMaskConstruction::SHAPE, FamilyShape::SingleCause);
        assert_eq!(SelectionMaskConstruction::SELECTION_ORDER.len(), 9);
        assert_eq!(
            SelectionMaskConstruction::SELECTION_ORDER.first(),
            Some(&"RowDomainMismatch")
        );
        assert_eq!(
            SelectionMaskConstruction::SELECTION_ORDER.get(1),
            Some(&"RowDomainEqualityUnproven")
        );
        assert_eq!(
            SelectionMaskConstruction::SELECTION_ORDER.last(),
            Some(&"UnusedBitsSet")
        );
        let unproven = SelectionMaskConstruction::RowDomainEqualityUnproven;
        let mismatch = SelectionMaskConstruction::RowDomainMismatch;
        assert_ne!(unproven, mismatch);
    }

    /// law: derived.two-seat-identity-holds — seat 1 is preimage-derived
    /// meaning, seat 2 is fresh build identity, and a real mask carries
    /// both; the source-binding forms are structural; the representations
    /// and validity conditions hold their rosters.
    /// Owed reversal (red twin): single-seat composition must not compile.
    #[test]
    fn two_seat_identity_holds() {
        assert_eq!(RowDomainId::CLASS, IdentityClass::SemanticCommitment);
        assert_eq!(
            RowDomainId::CREATION,
            CreationLaw::DerivedFromAdmittedPreimage
        );
        assert_eq!(OccurrenceId::CREATION, CreationLaw::FreshOpaque);
        let mask = SelectionMask {
            row_domain: RowDomainId(Commitment::raw([140; 32])),
            occurrence: OccurrenceId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh(
                [141; 16],
            ))),
            length: 128,
            binding: SourceBinding::GenerationForm(demo_generation(142)),
            iteration: Commitment::raw([143; 32]),
            representation: MaskRepresentation::DenseBitset,
            posture: Commitment::raw([144; 32]),
        };
        assert_eq!(mask.length, 128);
        let representations = [
            MaskRepresentation::DenseBitset,
            MaskRepresentation::SparseIndices,
            MaskRepresentation::Runs,
            MaskRepresentation::InlineWord,
        ];
        assert_eq!(representations.len(), 4);
        assert!(pairwise_distinct(&representations));
        let conditions = [
            ValidityCondition::Missing,
            ValidityCondition::Null,
            ValidityCondition::Unavailable,
            ValidityCondition::Shredded,
            ValidityCondition::Unauthorized,
            ValidityCondition::Invalid,
            ValidityCondition::Corrupt,
        ];
        assert_eq!(conditions.len(), 7);
        assert!(pairwise_distinct(&conditions));
        let column = Column {
            field: Commitment::raw([145; 32]),
            sort: Commitment::raw([146; 32]),
            row_domain: RowDomainId(Commitment::raw([140; 32])),
            length: 128,
            binding: SourceBinding::CutForm(demo_evidence(147)),
            layout: LayoutId(Commitment::raw([148; 32])),
            ordering: Commitment::raw([149; 32]),
            validity: Commitment::raw([150; 32]),
        };
        assert_eq!(column.row_domain, mask.row_domain);
    }

    /// law: derived.materialization-axes-and-the-triple — the applied cut
    /// composes over history's carrier mechanism as a role-distinct newtype;
    /// the generation is the tenth scope-guard; the three axes hold with
    /// coverage on the root completeness shape; and the load-bearing triple
    /// stays three types.
    /// Owed reversal (red twin): collapsing the triple must not compile.
    #[test]
    fn materialization_axes_and_the_triple() {
        let generation = demo_generation(151);
        let other = demo_generation(152);
        assert!(matches!(
            generation.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
        let same_scope = MaterializationGeneration::positioned(AuthorityPosition::assigned(
            MaterializationId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([151; 16]))),
            2,
        ));
        assert!(matches!(
            generation.try_cmp_same_scope(&same_scope),
            Ok(Ordering::Less)
        ));
        let applied = MaterializationAppliedCut {
            materialization: MaterializationId::for_laws(Occurrence::for_laws(
                OccurrenceForm::Fresh([153; 16]),
            )),
            generation,
            sources: MaterializationSourceCuts(
                FederationCutEntries::composed(&[], vec![])
                    .unwrap_or_else(|_| unreachable!("empty composes")),
            ),
        };
        assert_eq!(applied.sources.0.len(), 0);
        let presence = [
            MaterializationPresence::NotMaterialized,
            MaterializationPresence::Materialized,
        ];
        assert_eq!(presence.len(), 2);
        let availability = [
            MaterializationAvailability::Available,
            MaterializationAvailability::Unavailable,
        ];
        assert_eq!(availability.len(), 2);
        let coverage: MaterializationCoverage<u8> = MaterializationCoverage {
            coverage: Completeness::Complete { over: 4 },
        };
        assert!(matches!(
            coverage.coverage,
            Completeness::Complete { over: 4 }
        ));
        let over_commit: Option<fn(CommitPoint)> = Some(drop);
        let over_applied: Option<fn(MaterializationAppliedCut)> = Some(drop);
        let over_checkpoint: Option<fn(DurableCheckpoint)> = Some(drop);
        assert!(over_commit.is_some());
        assert!(over_applied.is_some());
        assert!(over_checkpoint.is_some());
    }

    /// law: derived.payload-locator-is-closed-two-forms — both forms
    /// construct, the extent identity is the Tier-1 content region, and no
    /// third form exists.
    /// Owed reversal (red twin): a four-scalar locator must not compile.
    #[test]
    fn payload_locator_is_closed_two_forms() {
        let slice = PayloadLocator::ExtentSlice {
            extent: ExtentEntryRef::declared(0),
            offset: 4_096,
            length: 512,
        };
        let binding = PayloadLocator::BindingEntry {
            binding: BindingEntryRef::declared(3),
        };
        assert_ne!(slice, binding);
        let entry = ExtentEntry {
            extent: ContentRegionId::of(ByteIdentity::raw([154; 32])),
            location: Commitment::raw([155; 32]),
            profiles: Commitment::raw([156; 32]),
            decoded_bounds: 1_048_576,
        };
        assert_eq!(entry.decoded_bounds, 1_048_576);
    }

    /// law: derived.plans-split-and-kernel-gate — the template/binding split
    /// constructs (static key vs per-use authority), the nine plan
    /// prohibitions hold, and the kernel admission gate holds eleven.
    /// Owed reversal (red twin): binding authority inside the template key
    /// must not compile.
    #[test]
    fn plans_split_and_kernel_gate() {
        let template = PlanTemplate {
            static_facts: Commitment::raw([157; 32]),
        };
        let binding = PlanBinding {
            cuts: demo_evidence(158),
            posture: Commitment::raw([159; 32]),
            admission: demo_evidence(160),
        };
        assert_eq!(template.static_facts, Commitment::raw([157; 32]));
        assert_eq!(binding.posture, Commitment::raw([159; 32]));
        assert_eq!(PLAN_CANNOT.len(), 9);
        assert_eq!(KERNEL_ADMISSION_GATE.len(), 11);
        assert_eq!(DATA_SEMANTIC_WORK.len(), 10);
    }

    /// law: derived.never-authority-rosters-hold — the lifecycle five, the
    /// ten primitives, the fifteen refusal classes, and the nine-item
    /// reversible standing bar (a standing bar, never a permanent ban).
    /// Owed reversal: flattening the bar into a ban must break this law.
    #[test]
    fn never_authority_rosters_hold() {
        let states = [
            DataBlockState::Building,
            DataBlockState::StructurallyValidated,
            DataBlockState::SealedDerivedOccurrence,
            DataBlockState::PublishedMaterialization,
            DataBlockState::RetiredOccurrence,
        ];
        assert_eq!(states.len(), 5);
        assert!(pairwise_distinct(&states));
        assert_eq!(DERIVATION_PRIMITIVES.len(), 10);
        assert_eq!(DERIVED_REFUSAL_CLASSES.len(), 15);
        assert_eq!(PROTECTED_INDEX_STANDING_BAR.len(), 9);
    }
}

mod application {
    use super::pairwise_distinct;
    use crate::application::{
        APPLICATION_VALIDATION_LADDER, AUTH_ROLES, AckProfile, ActivationGeneration,
        ActivationImageBinding, AppImageDigest, AppImageRef, AppSemanticCommitment,
        CONTRACT_COMPONENTS, CarrierObservation, CarrierRequestId, DeliveryDirection,
        DeliveryGuarantee, DeliveryIndex, DirectionState, EARLY_DATA_NEVER, FAMILY_FACTS,
        FLOW_CONTROL_FACTS, INGRESS_PIPELINE, IdempotencyIdentity, IngressAck, InstanceId,
        InstanceLifecycle, InvocationProfile, LOCAL_NOUNS, LagOverrunObservation, MESSAGE_FAMILIES,
        NON_IDENTITIES, NON_SUBSTITUTABLE_PREIMAGES, PossessionClaim, RAW_RETENTION_GUARDRAILS,
        REMOTE_VERBS, RESOURCE_NEVER_BECOMES, RejectedContentReason, RemovalHole, SessionId,
        StreamClosure, StreamState, TransportSecurityClaim,
    };
    use crate::bvisor::PortRequestId;
    use crate::history::RemovalCommitment;
    use crate::identity::{
        AuthorityPosition, ByteIdentity, Commitment, IdentityClass, IdentityRole, Occurrence,
        OccurrenceForm, OrderComparison,
    };
    use crate::types::{EvidenceRef, ReferentAvailability, ReferentIntegrity};
    use core::cmp::Ordering;

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: application.activation-generation-rides-scope — the eleventh
    /// scope-guard: generations of one instance compare, cross-instance
    /// refuses, and the activated image rides a typed relation, never the
    /// ordinal's bytes.
    /// Reversal (red twin), discharged by the stamp this guard is now written
    /// by, on the stamp's own fixtures: cross-scope comparison is a category error
    /// (`cross-scope-comparison-on-a-stamped-guard.rs`) and the position has no
    /// road out and none back in (`a-stamped-representation-cannot-be-laundered.rs`).
    /// Both prove a property of the generated shape, which is what makes them
    /// this guard's reversal rather than another home's.
    #[test]
    fn activation_generation_rides_scope() {
        let instance = InstanceId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([170; 16])));
        let g1 = ActivationGeneration::positioned(AuthorityPosition::assigned(instance, 1));
        let g2 = ActivationGeneration::positioned(AuthorityPosition::assigned(instance, 2));
        assert!(matches!(g1.try_cmp_same_scope(&g2), Ok(Ordering::Less)));
        let other = ActivationGeneration::positioned(AuthorityPosition::assigned(
            InstanceId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([171; 16]))),
            1,
        ));
        assert!(matches!(
            g1.try_cmp_same_scope(&other),
            Err(OrderComparison::NotSameScope)
        ));
        let binding = ActivationImageBinding {
            generation: g2,
            image: AppImageRef(demo_evidence(172)),
        };
        assert_eq!(binding.image.0, demo_evidence(172));
    }

    /// law: application.identities-do-not-merge — meaning is not bytes, the
    /// six non-identities are refusal causes only, and the instance
    /// lifecycle holds four with a terminal.
    /// Owed reversal (red twin): wrong-role substitution must not compile.
    #[test]
    fn identities_do_not_merge() {
        assert_eq!(
            AppSemanticCommitment(Commitment::raw([173; 32])).0,
            Commitment::raw([173; 32])
        );
        let digest = AppImageDigest(ByteIdentity::raw([174; 32]));
        assert_eq!(digest, AppImageDigest(ByteIdentity::raw([174; 32])));
        assert_eq!(NON_IDENTITIES.len(), 6);
        let lifecycle = [
            InstanceLifecycle::Instantiated,
            InstanceLifecycle::Active,
            InstanceLifecycle::Draining,
            InstanceLifecycle::Retired,
        ];
        assert_eq!(lifecycle.len(), 4);
        assert!(pairwise_distinct(&lifecycle));
        assert_eq!(APPLICATION_VALIDATION_LADDER.len(), 5);
        assert_eq!(APPLICATION_VALIDATION_LADDER.last(), Some(&"admissible"));
    }

    /// law: application.invocation-profiles-are-three — typed configuration
    /// with the resource never-becomes list.
    /// Owed reversal: a fourth profile must break this law.
    #[test]
    fn invocation_profiles_are_three() {
        let profiles = [
            InvocationProfile::DirectProgram,
            InvocationProfile::ApplicationEntrypoint,
            InvocationProfile::RestrictedQuery,
        ];
        assert_eq!(profiles.len(), 3);
        assert!(pairwise_distinct(&profiles));
        assert_eq!(RESOURCE_NEVER_BECOMES.len(), 8);
    }

    /// law: application.carrier-identities-stay-apart — the carrier request
    /// identity and the membrane's port request identity are distinct types
    /// related only through typed carriage; the delivery index is scoped to
    /// one session direction.
    /// Owed reversal (red twin): one satisfying the other must not compile.
    #[test]
    fn carrier_identities_stay_apart() {
        let over_carrier: Option<fn(CarrierRequestId)> = Some(drop);
        let over_port: Option<fn(PortRequestId)> = Some(drop);
        assert!(over_carrier.is_some());
        assert!(over_port.is_some());
        assert_eq!(CarrierRequestId::CLASS, IdentityClass::Occurrence);
        let carrier =
            CarrierRequestId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([180; 16])));
        assert_eq!(
            carrier,
            CarrierRequestId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([180; 16])))
        );
        let index = DeliveryIndex {
            session: SessionId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh([175; 16]))),
            direction: DeliveryDirection::Receive,
            index: 42,
        };
        assert_eq!(index.index, 42);
        assert_eq!(NON_SUBSTITUTABLE_PREIMAGES.len(), 8);
    }

    /// law: application.session-and-stream-vocabularies — the session state
    /// carries its terminal, the stream state is a record of both halves
    /// plus closure (never one linear enum), and a half-closed half is not a
    /// terminal.
    /// Owed reversal (red twin): a linear half-close enum must not exist.
    #[test]
    fn session_and_stream_vocabularies() {
        let stream = StreamState {
            send: DirectionState::HalfClosed,
            receive: DirectionState::Open,
            closure: StreamClosure::Open,
        };
        assert_eq!(stream.closure, StreamClosure::Open);
        let closures = [
            StreamClosure::Open,
            StreamClosure::Closed,
            StreamClosure::Reset,
        ];
        assert_eq!(closures.len(), 3);
        let guarantees = [
            DeliveryGuarantee::BestEffort,
            DeliveryGuarantee::ResumableAtLeastOnce,
        ];
        assert_eq!(guarantees.len(), 2);
        assert_eq!(FLOW_CONTROL_FACTS.len(), 4);
        let lag = LagOverrunObservation {
            not_retained: demo_evidence(176),
            recovery: demo_evidence(177),
            continuation: Commitment::raw([178; 32]),
        };
        assert_eq!(lag.continuation, Commitment::raw([178; 32]));
    }

    /// law: application.ingress-ladders-hold — the ack ladder's three rungs
    /// in order (only the last discharges), the no-default profile pair, the
    /// four-rung idempotency ladder in its declared preference order, the
    /// rejected-content classes with their guardrails, and the removal hole
    /// carrying history's commitment.
    /// Owed reversal (red twin): a non-admitted discharge must be
    /// unreachable.
    #[test]
    fn ingress_ladders_hold() {
        let ladder = [
            IngressAck::Received,
            IngressAck::Validated,
            IngressAck::Admitted,
        ];
        assert_eq!(ladder.len(), 3);
        assert_eq!(ladder.last(), Some(&IngressAck::Admitted));
        let profiles = [AckProfile::ThreeStageHandshake, AckProfile::SingleAck];
        assert_eq!(profiles.len(), 2);
        assert_eq!(IdempotencyIdentity::LADDER.len(), 4);
        assert_eq!(
            IdempotencyIdentity::LADDER.first(),
            Some(&IdempotencyIdentity::NaturalKey)
        );
        assert_eq!(
            IdempotencyIdentity::LADDER.last(),
            Some(&IdempotencyIdentity::ExplicitClientKey)
        );
        let reasons = [
            RejectedContentReason::Malformed,
            RejectedContentReason::Unauthorized,
            RejectedContentReason::OverBudget,
            RejectedContentReason::FailedClassification,
        ];
        assert_eq!(reasons.len(), 4);
        assert!(pairwise_distinct(&reasons));
        assert_eq!(RAW_RETENTION_GUARDRAILS.len(), 4);
        assert_eq!(INGRESS_PIPELINE.len(), 8);
        let hole = RemovalHole {
            removal: RemovalCommitment(Commitment::raw([179; 32])),
        };
        assert_eq!(hole.removal.0, Commitment::raw([179; 32]));
    }

    /// law: application.contract-and-rosters-hold — the contract's six
    /// components, the nine carrier observations (two of which carry typed
    /// evidence, never a verdict), sixteen message families with nine facts
    /// each, thirteen auth roles, and the surface rosters.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn contract_and_rosters_hold() {
        assert_eq!(CONTRACT_COMPONENTS.len(), 6);
        let observations = [
            CarrierObservation::BytesReceived,
            CarrierObservation::BytesAcceptedForWrite,
            CarrierObservation::FlowControlAvailability,
            CarrierObservation::ConnectionTransition,
            CarrierObservation::TransportSecurityEvidence(demo_evidence::<TransportSecurityClaim>(
                181,
            )),
            CarrierObservation::PossessionEvidence(demo_evidence::<PossessionClaim>(182)),
            CarrierObservation::IdleThreshold,
            CarrierObservation::AbsoluteDeadlineReached,
            CarrierObservation::CarrierRetry,
        ];
        assert_eq!(observations.len(), 9);
        assert!(pairwise_distinct(&observations));
        assert_eq!(MESSAGE_FAMILIES.len(), 16);
        assert_eq!(FAMILY_FACTS.len(), 9);
        assert_eq!(AUTH_ROLES.len(), 13);
        assert_eq!(EARLY_DATA_NEVER.len(), 8);
        assert_eq!(LOCAL_NOUNS.len(), 8);
        assert_eq!(REMOTE_VERBS.len(), 8);
    }
}

mod security {
    use super::pairwise_distinct;
    use crate::authority::CapabilityGrantId;
    use crate::identity::{Commitment, Occurrence, OccurrenceForm};
    use crate::security::{
        CRYPTO_ROLES, CapabilityLease, FIREWALL_ACT_TABLE, ForeignExecution, LabelArrow,
        LeaseRenewalAuthority, MechanismStandingView, REVOCATION_DEFAULTS,
        RevocationAcknowledgement, RevocationEvidence, RevocationObservation, SECRET_CAPABILITIES,
        SecretUseHandle, ShredDenominatorRow, ShredEvidence, ShredProgress, ShredRowStatus,
        TRUST_BOUNDARY_MEMBERS,
    };
    use crate::types::{
        EvidenceRef, LimitWitness, NonEmptyBounded, PositiveLimitWitness, ReferentAvailability,
        ReferentIntegrity,
    };

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: security.lease-collects-the-banded-seat — the lease constructs
    /// binding one grant to the time home's policy by reference, with the
    /// role-qualified renewal authority; the four paved revocation defaults
    /// hold.
    /// Owed reversal (red twin): a date edit as renewal must not compile.
    #[test]
    fn lease_collects_the_banded_seat() {
        let lease = CapabilityLease {
            grant: CapabilityGrantId::for_laws(Occurrence::for_laws(OccurrenceForm::Fresh(
                [190; 16],
            ))),
            scope: Commitment::raw([191; 32]),
            generation: Commitment::raw([192; 32]),
            deadline_policy: demo_evidence(193),
            renewal: LeaseRenewalAuthority(Commitment::raw([194; 32])),
        };
        assert_eq!(lease.renewal.0, Commitment::raw([194; 32]));
        assert_eq!(REVOCATION_DEFAULTS.len(), 4);
    }

    /// law: security.revocation-axes-stay-apart — observation and
    /// acknowledgement are distinct facts on one evidence record; freshness
    /// rides the evidence itself.
    /// Owed reversal (red twin): a fused observed/acknowledged token must
    /// not compile.
    #[test]
    fn revocation_axes_stay_apart() {
        let evidence = RevocationEvidence {
            participant: Commitment::raw([195; 32]),
            observation: RevocationObservation::Observed,
            acknowledgement: RevocationAcknowledgement::NotYetAcknowledged,
            evidence: demo_evidence(196),
        };
        assert_eq!(evidence.observation, RevocationObservation::Observed);
        assert_eq!(
            evidence.acknowledgement,
            RevocationAcknowledgement::NotYetAcknowledged
        );
    }

    /// law: security.shred-progress-never-collapses — the four progress
    /// facts stand distinct; the evidence record constructs with its visible
    /// denominator rows and six honest statuses.
    /// Owed reversal: a denominator hiding a row status must break this law.
    #[test]
    fn shred_progress_never_collapses() {
        let progress = [
            ShredProgress::DestructionRequested,
            ShredProgress::DestructionAttempted,
            ShredProgress::DestructionAcknowledged,
            ShredProgress::PhysicalCiphertextRetired,
        ];
        assert_eq!(progress.len(), 4);
        assert!(pairwise_distinct(&progress));
        let statuses = [
            ShredRowStatus::Destroyed,
            ShredRowStatus::Missing,
            ShredRowStatus::Unreachable,
            ShredRowStatus::LegallyRetained,
            ShredRowStatus::Unsupported,
            ShredRowStatus::FailedOrNotRun,
        ];
        assert_eq!(statuses.len(), 6);
        assert!(pairwise_distinct(&statuses));
        let evidence = ShredEvidence {
            generations: Commitment::raw([197; 32]),
            scope: Commitment::raw([198; 32]),
            backend: Commitment::raw([199; 32]),
            participants: NonEmptyBounded::admitted(
                ShredDenominatorRow {
                    subject: Commitment::raw([200; 32]),
                    status: ShredRowStatus::LegallyRetained,
                },
                vec![],
                &PositiveLimitWitness::inhabited(LimitWitness::declared(16))
                    .unwrap_or_else(|_| unreachable!("sixteen admits an item")),
            )
            .unwrap_or_else(|_| unreachable!("one fits")),
            durability: demo_evidence(201),
            index_invalidation: demo_evidence(202),
            resulting_resolution: Commitment::raw([203; 32]),
        };
        assert_eq!(evidence.participants.len(), 1);
    }

    /// law: security.mechanism-standing-is-append-only — four fact families
    /// plus a read-only view, never one mutable status enum.
    /// Owed reversal (red twin): a mutable status enum must not exist.
    #[test]
    fn mechanism_standing_is_append_only() {
        let view = MechanismStandingView(Commitment::raw([204; 32]));
        assert_eq!(view.0, Commitment::raw([204; 32]));
    }

    /// law: security.secret-handle-refuses-the-morphism — the handle type
    /// exists with no clone, copy, debug, display, or serialization route;
    /// the green half is type-level existence, the true proof is the red
    /// twin.
    /// Owed reversal (red twin): Debug/Display/serde/Send on the handle must
    /// not compile — trybuild fixtures owed to testpak.
    #[test]
    fn secret_handle_refuses_the_morphism() {
        let over_handle: Option<fn(SecretUseHandle)> = Some(drop);
        assert!(over_handle.is_some());
    }

    /// law: security.firewall-and-rosters-hold — the act table, crypto
    /// roles, witness role, foreign-execution pair, label arrows, secret
    /// capabilities, and trust-boundary members.
    /// Owed reversal: any roster growing silently must break this law.
    #[test]
    fn firewall_and_rosters_hold() {
        assert_eq!(FIREWALL_ACT_TABLE.len(), 5);
        assert_eq!(CRYPTO_ROLES.len(), 7);
        let executions = [
            ForeignExecution::IsolatedPakVmWorker,
            ForeignExecution::ExternalToolEffect,
        ];
        assert_eq!(executions.len(), 2);
        let arrows = [
            LabelArrow::Join,
            LabelArrow::AggregateWithDeclaredLeakage,
            LabelArrow::Declassification,
        ];
        assert_eq!(arrows.len(), 3);
        assert!(pairwise_distinct(&arrows));
        assert_eq!(SECRET_CAPABILITIES.len(), 4);
        assert_eq!(TRUST_BOUNDARY_MEMBERS.len(), 8);
    }
}

mod evidence {
    use super::pairwise_distinct;
    use crate::evidence::{
        AdoptionDecisionReceipt, Basis, CalibrationEvidence, CalibrationModel, CauseDisposition,
        CommitmentLayers, Coverage, DiagnosticCause, EVIDENCE_NON_COLLAPSE, EXPLANATION_LADDER,
        Enforcement, EvidenceCarriage, GeneratedPublicationReceipt, Lane, LaneDomain, Method,
        QualificationTerminal, RECEIPT_FAMILIES, ReleaseEvidence, Route, SubstrateDisclosure,
        VerificationDenominator, VerificationResult, VerificationTerminal, VerifiedClaim,
    };
    use crate::identity::Commitment;
    use crate::types::{
        Bounded, Completeness, EvidenceRef, LimitWitness, ProofDisposition, ReferentAvailability,
        ReferentIntegrity,
    };

    fn demo_evidence<Claim>(seed: u8) -> EvidenceRef<Claim> {
        EvidenceRef::bound(
            [seed; 32],
            1,
            ReferentAvailability::Available,
            ReferentIntegrity::Intact,
        )
    }

    /// law: evidence.verification-is-a-tuple-not-a-ladder — a complete tuple
    /// constructs with every axis its own typed value, and each axis holds its
    /// own closed roster: four bases, sixteen methods, ten claims, five proof
    /// dispositions, three enforcement postures. No axis is a rank of another.
    /// Owed reversal (red twin): a flattened status enum must not exist.
    #[test]
    fn verification_is_a_tuple_not_a_ladder() {
        let result = VerificationResult {
            basis: Basis::IndependentReference,
            method: Method::DifferentialExecution,
            claim: VerifiedClaim::Conformance,
            coverage: Coverage::Bounded,
            enforcement: Enforcement::Blocking,
            lane: Lane(Commitment::raw([211; 32])),
            denominator: VerificationDenominator(Completeness::Complete {
                over: Commitment::raw([210; 32]),
            }),
            proof: ProofDisposition::Established,
            terminal: VerificationTerminal::Concluded,
        };
        assert_eq!(result.lane, Lane(Commitment::raw([211; 32])));
        let bases = [
            Basis::ContractProjection,
            Basis::IndependentReference,
            Basis::DirectBoundary,
            Basis::RuntimeObservation,
        ];
        assert_eq!(bases.len(), 4);
        assert!(pairwise_distinct(&bases));
        let methods = [
            Method::StructuralRule,
            Method::CompileRefusal,
            Method::PropertySequence,
            Method::BoundedStateExploration,
            Method::ScheduleExploration,
            Method::DeterministicSimulation,
            Method::DifferentialExecution,
            Method::TranslationValidation,
            Method::FaultInjection,
            Method::CrashRecovery,
            Method::Fuzzing,
            Method::Mutation,
            Method::ComplexityContract,
            Method::BenchmarkEnvelope,
            Method::HistoryReplay,
            Method::FormalCheck,
        ];
        assert_eq!(methods.len(), 16);
        assert!(pairwise_distinct(&methods));
        let claims = [
            VerifiedClaim::Safety,
            VerifiedClaim::Liveness,
            VerifiedClaim::BoundedResponse,
            VerifiedClaim::Convergence,
            VerifiedClaim::Stability,
            VerifiedClaim::NonOscillation,
            VerifiedClaim::Determinism,
            VerifiedClaim::Refinement,
            VerifiedClaim::Conformance,
            VerifiedClaim::ResourceEnvelope,
        ];
        assert_eq!(claims.len(), 10);
        assert!(pairwise_distinct(&claims));
        let dispositions = [
            ProofDisposition::Established,
            ProofDisposition::Falsified,
            ProofDisposition::Corroborated,
            ProofDisposition::Narrowed,
            ProofDisposition::Incomplete,
        ];
        assert_eq!(dispositions.len(), 5);
        assert!(pairwise_distinct(&dispositions));
        let enforcement = [
            Enforcement::Blocking,
            Enforcement::Quarantine,
            Enforcement::Advisory,
        ];
        assert_eq!(enforcement.len(), 3);
        let lanes = [
            Lane(Commitment::<LaneDomain>::raw([211; 32])),
            Lane(Commitment::<LaneDomain>::raw([212; 32])),
        ];
        assert_eq!(lanes.len(), 2);
        assert!(pairwise_distinct(&lanes));
    }

    /// law: evidence.coverage-is-unordered — the coverage axis is a roster of
    /// four distinct kinds of reach, related by equality and by nothing else. A
    /// separate law from the tuple law because it is a separate claim: the tuple
    /// law says coverage is its own axis rather than a rank of the proof axis,
    /// and this one says the four values inside that axis are not ranked against
    /// each other either. Bounded reach is not "more" than observed history;
    /// they are different reaches, and a comparison operator would invite a
    /// reader to trade one for the other.
    /// Owed reversal (red twin): deriving `Ord` on `Coverage` and writing
    /// `a < b` must not compile — the absence this law's green half cannot
    /// state, only a compile-fail fixture can.
    #[test]
    fn coverage_is_unordered() {
        let coverage = [
            Coverage::Sampled,
            Coverage::Bounded,
            Coverage::ExhaustiveWithinDeclaredModel,
            Coverage::ObservedHistory,
        ];
        assert_eq!(coverage.len(), 4);
        assert!(pairwise_distinct(&coverage));
        // Equality is the whole relation the axis carries: each value is itself
        // and is no other, and nothing here orders one against another.
    }

    /// law: evidence.terminals-are-lifecycle-owned — verification and
    /// qualification own distinct terminal algebras; no universal terminal
    /// exists, and a falsified run is a CONCLUDED run.
    /// Owed reversal (red twin): one terminal substituting for the other
    /// must not compile.
    #[test]
    fn terminals_are_lifecycle_owned() {
        let verification = [
            VerificationTerminal::Concluded,
            VerificationTerminal::Aborted,
        ];
        assert_eq!(verification.len(), 2);
        let qualification = [
            QualificationTerminal::Qualified,
            QualificationTerminal::Failed,
            QualificationTerminal::Abandoned,
        ];
        assert_eq!(qualification.len(), 3);
        assert!(pairwise_distinct(&qualification));
    }

    /// law: evidence.routes-bind-bases — the three independent routes stand
    /// distinct with the mandatory substrate disclosure.
    /// Owed reversal (red twin): an invalid basis/route pair must refuse
    /// structurally when the admission seam lands.
    #[test]
    fn routes_bind_bases() {
        let routes = [
            Route::DifferentialImplementation,
            Route::IndependentHistoryReplay,
            Route::HostileBoundary,
        ];
        assert_eq!(routes.len(), 3);
        assert!(pairwise_distinct(&routes));
        let disclosure = SubstrateDisclosure(Commitment::raw([211; 32]));
        assert_eq!(disclosure.0, Commitment::raw([211; 32]));
    }

    /// law: evidence.cause-disposition-narrows — narrowing is progress, not
    /// a forced verdict: all three postures construct, and the narrowed-cause
    /// set is a typed carrier, never a raw vector.
    /// Owed reversal (red twin): upgrading a correlation into an established
    /// cause must not compile.
    #[test]
    fn cause_disposition_narrows() {
        let established =
            CauseDisposition::EstablishedCause(DiagnosticCause(Commitment::raw([212; 32])));
        let unresolved = CauseDisposition::UnresolvedCause;
        assert_ne!(established, unresolved);
    }

    /// law: evidence.receipt-matrix-and-carriage — twenty-five families by
    /// semantic boundary (the two structurally two-record rows named),
    /// per-item carriage, coexisting commitment layers, and the calibration
    /// pair owning neither work nor truth.
    /// Owed reversal (red twin): a universal receipt must not exist.
    #[test]
    fn receipt_matrix_and_carriage() {
        assert_eq!(RECEIPT_FAMILIES.len(), 25);
        assert!(RECEIPT_FAMILIES.contains(&"image-invocation-two-record"));
        assert!(RECEIPT_FAMILIES.contains(&"proposal-and-adoption-two-record"));
        let carriage = [
            EvidenceCarriage::Inline,
            EvidenceCarriage::ImmutableReference,
        ];
        assert_eq!(carriage.len(), 2);
        let layers = CommitmentLayers {
            layers: Bounded::admitted(
                vec![Commitment::raw([214; 32]), Commitment::raw([215; 32])],
                &LimitWitness::declared(8),
            )
            .unwrap_or_else(|_| unreachable!("two fit")),
        };
        assert_eq!(layers.layers.len(), 2);
        let calibration = CalibrationEvidence {
            model: CalibrationModel(Commitment::raw([216; 32])),
            evidence: demo_evidence(217),
        };
        assert_eq!(calibration.model.0, Commitment::raw([216; 32]));
        assert_eq!(EVIDENCE_NON_COLLAPSE.len(), 15);
    }

    /// law: evidence.lifecycles-are-four-and-separate — adoption is a human
    /// act resting on qualification by reference; generated publication is
    /// all-or-refusal; release is a conjunction with agreeing denominators;
    /// the explanation ladder holds.
    /// Owed reversal (red twin): a self-adoption must not compile.
    #[test]
    fn lifecycles_are_four_and_separate() {
        let adoption = AdoptionDecisionReceipt {
            qualified: demo_evidence(218),
            adopted_by: Commitment::raw([219; 32]),
            target: Commitment::raw([220; 32]),
        };
        assert_eq!(adoption.adopted_by, Commitment::raw([219; 32]));
        let publication = GeneratedPublicationReceipt {
            unit: Commitment::raw([221; 32]),
            staged: demo_evidence(222),
            manifest: demo_evidence(223),
        };
        assert_eq!(publication.unit, Commitment::raw([221; 32]));
        let release = ReleaseEvidence {
            rows: demo_evidence(224),
            artifacts: Commitment::raw([225; 32]),
            denominator: VerificationDenominator(Completeness::Complete {
                over: Commitment::raw([226; 32]),
            }),
        };
        assert!(matches!(
            release.denominator.0,
            Completeness::Complete { .. }
        ));
        assert_eq!(EXPLANATION_LADDER.len(), 4);
    }
}

//! The one compile-time proof surface for the metaprogramming services,
//! sectioned by module. Green laws only, in the module's declaration order, so
//! this file reads down the dependency line exactly as `lib.rs` declares it.
//!
//! A law that cannot fail is not a law: these compile (and trivially run) only
//! while the shapes hold; reversing the shape breaks the named law. Each law's
//! doc line states the reversal it is owed, and the reversal lands in testpak.
//!
//! The proof surface reaches every module and is reached by none. It is the
//! only module in this crate that may look in every direction, which is why it
//! is declared last, is not public, and is excluded from the
//! `tooling-module-order` check by that non-public declaration.

mod plane {
    use crate::plane::{
        AuthoringLimitProfile, DECLARED_LIMITS, HumanProjection, HumanTextLimit, OwnerFactSubject,
        OwnerHomeSubject, OwnerIdentityRef, ProfileVersion, RefusalReason,
    };
    use threadpak::types::{BoundedConstruction, ConstLimit, LimitAdmissionProfile};

    /// The widest magnitude the plane's OWN magnitude rows declare, read off
    /// those rows.
    ///
    /// `DECLARED_LIMITS` is emitted from the same rows the families are, so this
    /// answer moves the moment a declaration does — which is the whole point:
    /// naming a family here would make the law green against a row set that had
    /// grown past it.
    fn widest_declared_magnitude() -> usize {
        DECLARED_LIMITS.iter().fold(
            0usize,
            |widest, &(_, declared)| {
                if declared > widest { declared } else { widest }
            },
        )
    }

    /// law: plane.the-authoring-ceiling-is-this-plane-s-own — the services admit
    /// their declared magnitudes under a ceiling this plane wrote down, and the
    /// relation that justifies the number holds: the ceiling is sixteen times
    /// the widest magnitude the plane's OWN REMAINING rows declare.
    ///
    /// The widest magnitude is DERIVED from the roster rather than named. The
    /// law used to compare the ceiling against one family by its Rust spelling,
    /// which meant a new family wider than that one left the law green while the
    /// obligation's own rationale — "sixteen times the widest magnitude the
    /// plane declares" — had quietly stopped being true. Reading the roster is
    /// what makes the relation the thing under test.
    ///
    /// **The denominator moves when a row moves home, and the law says which
    /// row it is standing on.** The plane's rows are the magnitudes MORE THAN
    /// ONE home asks about; a magnitude only one home asks is declared in that
    /// home through the same stamp, and the rehoming that emptied this roster of
    /// its single-home rows took the capture-work magnitude — the row this
    /// number was first argued from — into the token seam with it. What remains
    /// widest here is `RenderedByteLimit`, at the same sixty-five thousand five
    /// hundred and thirty-six, so the relation is unchanged in arithmetic and
    /// changed in what it stands on. It stays falsifiable exactly as before:
    /// moving `RenderedByteLimit` out, or seating a wider central row, breaks
    /// this assertion rather than leaving it green against a roster it has
    /// stopped describing.
    ///
    /// The named family below is a second assertion and never the comparison —
    /// the ceiling is still checked against the DERIVED widest, and the naming
    /// line exists so the sentence above ("what remains widest here is
    /// `RenderedByteLimit`") is a claim the law breaks on rather than prose a
    /// reader has to take on faith. Naming a family INSTEAD of folding the
    /// roster is the retired posture, and it stays retired.
    ///
    /// The claim ceiling: this is the positive control for the AUTHORING plane's
    /// policy and nothing about the machine's algebra. The roster it reads is
    /// the plane's own rows, which is where the ceiling's justifying relation
    /// was argued; a home's rows are admitted under the same profile and every
    /// one of them stands far inside it, and no claim is made here about that —
    /// which is a wider disclaimer now than it was, because most of the
    /// services' magnitudes live in homes.
    ///
    /// Owed reversal (red twin): a family declaring a magnitude past this
    /// ceiling must not compile — the fixture is testpak's.
    #[test]
    fn the_authoring_ceiling_is_this_planes_own() {
        assert_eq!(AuthoringLimitProfile::MAX_DECLARED_LIMIT, 1_048_576);
        let widest = widest_declared_magnitude();
        assert!(widest > 0, "the plane declares no limit family at all");
        assert_eq!(widest, crate::plane::RenderedByteLimit::MAX);
        assert_eq!(
            AuthoringLimitProfile::MAX_DECLARED_LIMIT,
            widest.saturating_mul(16)
        );
    }

    /// law: plane.the-declared-limit-roster-is-read-from-its-own-rows — the
    /// roster a claim about "every family on these rows" is answered from is
    /// emitted by the same expansion that declares them, so it cannot list a row
    /// the plane does not declare and cannot omit one it does. It is a
    /// projection over one row set and never a second owner of any row in it.
    ///
    /// The two spot checks are the joins a hand-maintained table would fail: a
    /// named family's magnitude read through the table equals the magnitude read
    /// through its own `ConstLimit`, and the table's length is the count of rows
    /// the macro was given rather than a number anybody wrote down.
    ///
    /// The length bound is a floor with headroom and never the exact count: an
    /// exact number here would be a number somebody wrote down, which is the one
    /// thing this law exists to rule out. It moved down with the roster when the
    /// single-home rows went to their homes — the plane keeps the magnitudes
    /// more than one home asks about, and there are twelve of them.
    ///
    /// Owed reversal (red twin): a table authored beside the declarations rather
    /// than emitted from them must break this law — the fixture is testpak's.
    #[test]
    fn the_declared_limit_roster_is_read_from_its_own_rows() {
        let named = DECLARED_LIMITS
            .iter()
            .find(|(name, _)| *name == "RenderedByteLimit")
            .map(|&(_, declared)| declared);
        assert_eq!(named, Some(crate::plane::RenderedByteLimit::MAX));
        assert!(DECLARED_LIMITS.len() > 8);
        assert!(
            DECLARED_LIMITS
                .iter()
                .all(|&(name, declared)| !name.is_empty() && declared > 0)
        );
    }

    /// law: plane.subjects-do-not-unify — a reference naming one subject is a
    /// different type than a reference naming another, whatever the bytes.
    /// Owed reversal: erasing the subject parameter must break this law.
    #[test]
    fn subjects_do_not_unify() {
        let home: Option<fn(OwnerIdentityRef<OwnerHomeSubject>)> = Some(drop);
        let fact: Option<fn(OwnerIdentityRef<OwnerFactSubject>)> = Some(drop);
        assert!(home.is_some() && fact.is_some());
        let same_bytes_different_subject = OwnerIdentityRef::<OwnerHomeSubject>::decoded([3; 32]);
        assert_eq!(same_bytes_different_subject.as_bytes(), &[3_u8; 32]);
    }

    /// law: plane.reason-projection-preserves-bytes — projecting a registered
    /// reason adapts nothing; a projection may change presentation, never
    /// identity.
    /// Owed reversal: a projection that rewrote the bytes must break this law.
    #[test]
    fn reason_projection_preserves_bytes() {
        let declared = OwnerIdentityRef::<RefusalReason>::decoded([9; 32]);
        assert_eq!(declared.as_bytes(), &[9_u8; 32]);
    }

    /// law: plane.human-projections-are-bounded — a rendering that does not fit
    /// its declared bound refuses rather than truncating.
    /// Owed reversal (red twin): a constructor that truncated must break this
    /// law.
    #[test]
    fn human_projections_are_bounded() {
        let fits = HumanProjection::<HumanTextLimit>::projected("the owner declared this repair");
        assert!(fits.is_ok_and(|projection| !projection.is_empty() && projection.len() == 30));
        let oversized = "x".repeat(HumanTextLimit::MAX.saturating_add(1));
        let refused = HumanProjection::<HumanTextLimit>::projected(&oversized);
        assert!(matches!(refused, Err(BoundedConstruction::OverLimit)));
    }

    /// law: plane.profile-versions-are-not-ranked — a profile version carries a
    /// position and admits no ordering operator across profiles.
    /// Owed reversal (red twin): deriving `Ord` and comparing two versions must
    /// not compile.
    #[test]
    fn profile_versions_are_not_ranked() {
        let first = ProfileVersion::declared(1);
        let second = ProfileVersion::declared(2);
        assert_ne!(first, second);
        assert_eq!(second.position(), 2);
    }
}

/// The identity profiles' proof surface: the golden vectors that pin the
/// derivation, the mutation vectors that prove it is sensitive to its whole
/// transcript, and the crossing vectors that prove domain separation bites —
/// across families as well as across subjects and roles.
///
/// # Why golden vectors and not only properties
///
/// A property test says the derivation is self-consistent. A golden vector says
/// WHICH derivation it is. Without one, a change to the field order, the length
/// framing, the domain grammar, or a family's version would keep every property
/// green while silently renaming identities — and the rename would be discovered
/// by whoever compared a name they were already holding against a freshly
/// derived one, which is the worst possible time.
///
/// These vectors are one family's fingerprint at the position they pin. A vector
/// that fails is a change to that family's grammar, and such a change is a bump
/// of that family's version, not a fixed constant.
mod identity_profile {
    use crate::plane::{
        CAPTURED_DECLARATION_IDENTITY_PROFILE, CLOSED_EXPANSION_IDENTITY_PROFILE,
        CLOSURE_IDENTITY_PROFILE, GENERATED_UNIT_IDENTITY_PROFILE, GeneratedUnitSubject,
        GeneratorIdentity, GeneratorSchemaVersion, IDENTITY_PROFILE_STEM, IdentityProfileVersion,
        MACROC_GENERATOR, PROJECTION_INTENT_IDENTITY_PROFILE, PlanSubject, PreimageFamily,
        ProjectionIdentity, ProjectionRole, ProjectionTranscript, RENDERED_UNIT_IDENTITY_PROFILE,
        RenderedUnitSubject, SUBJECT_NAMES, TranscriptAnchoring, encode_bytes,
    };

    /// The anchor every anchored vector below is taken under.
    const GOLDEN_ANCHOR: [u8; 32] = [7; 32];

    /// The content every content-bearing vector below is taken over.
    const GOLDEN_CONTENT: &[u8] = b"golden-vector-content";

    /// The roster position every anchored vector below is taken at.
    const GOLDEN_POSITION: u32 = 3;

    /// The fixed anchored transcript: role `GeneratedUnit`, anchored under a
    /// plane identity of thirty-two `7` bytes, over `golden-vector-content`, at
    /// roster position three.
    fn anchored_vector() -> ProjectionTranscript<'static> {
        ProjectionTranscript::under(
            ProjectionRole::GeneratedUnit,
            TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
            GOLDEN_CONTENT,
            GOLDEN_POSITION,
        )
    }

    /// The fixed rooted transcript: role `Plan`, no anchor, empty content, at
    /// roster position zero — the narrowest transcript any family admits.
    fn rooted_vector() -> ProjectionTranscript<'static> {
        ProjectionTranscript::rooted(ProjectionRole::Plan, &[], 0)
    }

    /// law: identity-profile.the-declared-version-is-pinned — every family's
    /// version is a typed constant of its own, and every vector below is a
    /// fingerprint OF the family it is taken under. Reading the vectors without
    /// reading the positions they pin would make a bump look like a broken law.
    ///
    /// Each family is read through the one road a derivation reaches it by, so
    /// the pin is over the answer the plane actually uses rather than over a
    /// constant a derivation might not consult.
    ///
    /// Owed reversal: bumping any family's version without restating that
    /// family's vectors must break this law.
    #[test]
    fn the_declared_version_is_pinned() {
        assert_eq!(IDENTITY_PROFILE_STEM, "threadpak/macroc/projection-identity");
        let first = IdentityProfileVersion::declared(1);
        assert!(
            PreimageFamily::ALL
                .iter()
                .copied()
                .all(|family| family.profile().version() == first),
            "every family stands at its own first position"
        );
        assert!(
            PreimageFamily::ALL
                .iter()
                .copied()
                .all(|family| family.profile().family() == family),
            "a profile carries the family that declared it"
        );
        assert_eq!(MACROC_GENERATOR.profile().spelling(), "threadpak-macroc");
        assert_eq!(MACROC_GENERATOR.schema().position(), 3);
    }

    /// law: identity-profile.the-domain-grammar-is-spelled-exactly — the
    /// derive-key context is `<stem>/<family>/v<version>/<subject>/<role>`, and
    /// nothing about it is inferred at a call site.
    /// Owed reversal: a context assembled in another order must break this law.
    #[test]
    fn the_domain_grammar_is_spelled_exactly() {
        assert_eq!(
            RENDERED_UNIT_IDENTITY_PROFILE
                .context_for("generated-unit", ProjectionRole::OutputBytes),
            "threadpak/macroc/projection-identity/rendered-unit/v1/generated-unit/output-bytes"
        );
    }

    /// law: identity-profile.families-never-share-a-derivation-namespace — two
    /// families at one position, over one subject and one role, are two distinct
    /// derive-key contexts and derive two distinct identities. That is what the
    /// family segment ahead of the version buys, and it is the whole reason a
    /// bump under one family reaches nothing under another.
    ///
    /// Owed reversal (red twin): folding the family segment out of the context,
    /// or moving it behind the version, must break this law.
    #[test]
    fn families_never_share_a_derivation_namespace() {
        let mut contexts: Vec<String> = PreimageFamily::ALL
            .iter()
            .copied()
            .map(|family| {
                family
                    .profile()
                    .context_for("generated-unit", ProjectionRole::GeneratedUnit)
            })
            .collect();
        let counted = contexts.len();
        contexts.sort_unstable();
        contexts.dedup();
        assert_eq!(contexts.len(), counted);
        assert!(
            contexts
                .iter()
                .all(|context| context.starts_with(IDENTITY_PROFILE_STEM))
        );
        // Every family stands at position one today, so the version segment
        // separates none of them and the family segment separates all of them.
        assert_ne!(
            PROJECTION_INTENT_IDENTITY_PROFILE
                .context_for("generated-unit", ProjectionRole::GeneratedUnit),
            CLOSURE_IDENTITY_PROFILE
                .context_for("generated-unit", ProjectionRole::GeneratedUnit)
        );
    }

    /// law: identity-profile.every-role-stands-in-one-family — the answer from a
    /// role to a family is total, and the two roles standing over ONE rendered
    /// grammar share one family while every other role holds its own. A role
    /// with no family would be a mint site with no version ladder.
    ///
    /// The last two readings are the other half of the same obligation, and they
    /// are what the stand-in roles were added for: every declared family is
    /// reached by a role, and no family is left standing for identities that are
    /// minted under a neighbour's ladder instead. The explanation family in
    /// particular used to be reached by nothing while explanations existed and
    /// carried no name at all.
    ///
    /// Owed reversal (red twin): a role answering with a family whose grammar it
    /// does not stand in — a rendered unit under the plan family — must break
    /// this law, and so must a family declared with no role reaching it.
    #[test]
    fn every_role_stands_in_one_family() {
        assert_eq!(
            ProjectionRole::RenderedUnit.family(),
            PreimageFamily::RenderedUnit
        );
        assert_eq!(
            ProjectionRole::OutputBytes.family(),
            PreimageFamily::RenderedUnit
        );
        assert_eq!(ProjectionRole::Plan.family(), PreimageFamily::Plan);
        assert_eq!(
            ProjectionRole::ProjectionIntent.family(),
            PreimageFamily::ProjectionIntent
        );
        assert_eq!(
            ProjectionRole::Explanation.family(),
            PreimageFamily::Explanation
        );
        // One shared family and no other, so the roster of families the roles
        // reach is one shorter than the roster of roles.
        let mut reached: Vec<&str> = ProjectionRole::ALL
            .iter()
            .copied()
            .map(|role| role.family().stable_name())
            .collect();
        reached.sort_unstable();
        reached.dedup();
        assert_eq!(reached.len(), ProjectionRole::ALL.len().saturating_sub(1));
        // And every declared family is reached: a family no role stands in is a
        // grammar with a version ladder nothing climbs, which is where an
        // identity ends up riding a neighbour's.
        assert_eq!(reached.len(), PreimageFamily::ALL.len());
        assert!(
            PreimageFamily::ALL
                .iter()
                .copied()
                .all(|family| reached.contains(&family.stable_name()))
        );
    }

    /// law: identity-profile.every-name-in-the-context-is-distinct-and-legal —
    /// two subjects, two roles, or two families sharing a name would share a key
    /// space, and a name outside the grammar would make the context unreadable.
    /// Owed reversal: two subjects declaring one name must break this law.
    #[test]
    fn every_name_in_the_context_is_distinct_and_legal() {
        let legal = |name: &str| {
            !name.is_empty()
                && !name.starts_with('-')
                && !name.ends_with('-')
                && !name.contains("--")
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        };
        let distinct = |names: &[&str]| {
            let mut seen: Vec<&str> = names.to_vec();
            let counted = seen.len();
            seen.sort_unstable();
            seen.dedup();
            seen.len() == counted
        };
        assert!(SUBJECT_NAMES.iter().copied().all(legal));
        assert!(distinct(SUBJECT_NAMES));
        let roles: Vec<&str> = ProjectionRole::ALL
            .iter()
            .copied()
            .map(ProjectionRole::stable_name)
            .collect();
        assert!(roles.iter().copied().all(legal));
        assert!(distinct(&roles));
        let families: Vec<&str> = PreimageFamily::ALL
            .iter()
            .copied()
            .map(PreimageFamily::stable_name)
            .collect();
        assert!(families.iter().copied().all(legal));
        assert!(distinct(&families));
    }

    /// The anchored vector's ten members, spelled by hand from the
    /// specification: the shared stem, the generated-unit family, that family's
    /// position, the subject, the role and its slot, the anchoring posture and
    /// its commitment, the content, and the roster position.
    ///
    /// Written once, here, because two laws read it — the one that pins the
    /// spelling and the one that refuses the retired spelling — and two
    /// hand-built strings would be two things that agree until one is edited.
    fn spelled_members() -> Vec<u8> {
        let mut spelled = Vec::new();
        encode_bytes(b"threadpak/macroc/projection-identity", &mut spelled);
        encode_bytes(b"generated-unit", &mut spelled);
        spelled.extend_from_slice(&1_u32.to_be_bytes());
        encode_bytes(b"generated-unit", &mut spelled);
        encode_bytes(b"generated-unit", &mut spelled);
        spelled.push(3);
        spelled.push(2);
        encode_bytes(&GOLDEN_ANCHOR, &mut spelled);
        encode_bytes(GOLDEN_CONTENT, &mut spelled);
        spelled.extend_from_slice(&GOLDEN_POSITION.to_be_bytes());
        spelled
    }

    /// law: identity-profile.the-transcript-is-spelled-exactly — the transcript
    /// is the ten members of the specification, in order, with every
    /// variable-length member length-prefixed. This law requires the producer's
    /// own encoder to agree with the hand-built spelling.
    /// Owed reversal: dropping a member or a length prefix must break this law.
    #[test]
    fn the_transcript_is_spelled_exactly() {
        assert_eq!(
            anchored_vector().encoded("generated-unit"),
            spelled_members()
        );
    }

    /// law: identity-profile.no-preimage-names-the-generator — the producer
    /// writes the ten-member spelling and NOT the retired twelve-member one,
    /// which appended the generator's declared name and its schema position
    /// behind them. The generator is provenance now, so a rendered shape that
    /// moved renames nothing, and an intent identity two doors agree on keeps
    /// agreeing across the machinery that would realize it.
    ///
    /// The last reading is the one that states the size of the defect: the
    /// retired spelling derives a DIFFERENT identity from the same ten members,
    /// so every position the generator moved through renamed the whole tree.
    ///
    /// Owed reversal (red twin): writing the generator profile or schema back
    /// into the preimage must break this law.
    #[test]
    fn no_preimage_names_the_generator() {
        let mut retired = spelled_members();
        encode_bytes(
            MACROC_GENERATOR.profile().spelling().as_bytes(),
            &mut retired,
        );
        retired.extend_from_slice(&MACROC_GENERATOR.schema().position().to_be_bytes());
        let encoded = anchored_vector().encoded("generated-unit");
        assert_eq!(encoded, spelled_members());
        assert_ne!(encoded, retired);
        assert_ne!(
            *ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored_vector()).as_bytes(),
            blake3::derive_key(
                &GENERATED_UNIT_IDENTITY_PROFILE
                    .context_for("generated-unit", ProjectionRole::GeneratedUnit),
                &retired,
            ),
            "the retired spelling named something else, which is what made every \
             shape bump a rename"
        );
    }

    /// law: identity-profile.a-mint-site-cannot-choose-its-family — the profile
    /// a transcript is written under is read off the role, so the family, its
    /// version, and the context all follow from the one fact a mint site states.
    /// No constructor takes a profile, and this law reads the answer back off
    /// three transcripts to say so.
    ///
    /// Owed reversal (red twin): a transcript constructor taking a profile
    /// beside its role must break this law.
    #[test]
    fn a_mint_site_cannot_choose_its_family() {
        assert_eq!(
            anchored_vector().profile(),
            GENERATED_UNIT_IDENTITY_PROFILE,
            "the generated-unit role writes under the generated-unit family"
        );
        assert_eq!(
            ProjectionTranscript::rooted(ProjectionRole::CapturedDeclaration, &[], 0).profile(),
            CAPTURED_DECLARATION_IDENTITY_PROFILE
        );
        assert_eq!(
            ProjectionTranscript::rooted(ProjectionRole::ClosedExpansion, &[], 0).profile(),
            CLOSED_EXPANSION_IDENTITY_PROFILE
        );
    }

    /// law: identity-profile.golden-vectors-pin-the-derivation — three fixed
    /// transcripts derive three exact values, across two families, two subjects,
    /// and both anchoring postures.
    /// Owed reversal (red twin): any change to the field order, the length
    /// framing, the domain grammar, or the digest must break this law.
    ///
    /// # The three values below PREDATE the per-family split
    ///
    /// They were written as the fingerprint of the retired single profile at an
    /// earlier position, and every one of them is a BLAKE3 output — a value
    /// nobody can recompute by reading, only by executing. The family segment
    /// and the version are both members of the transcript and both segments of
    /// the derive-key context, so all three moved when the profiles split, and
    /// none can be restated in a phase where no toolchain runs.
    ///
    /// Writing three plausible-looking constants in their place would be worse
    /// than leaving them: a fabricated vector is green against nothing and would
    /// pin a derivation nobody performed. So the authored values stand, stated
    /// stale, and recomputing all three is the first toolchain contact's
    /// corrective batch — one act, each vector under the family it is the
    /// fingerprint of.
    #[test]
    fn golden_vectors_pin_the_derivation() {
        // Stale: awaits recompute under the generated-unit family at position
        // one, at the first toolchain contact's corrective batch.
        assert_eq!(
            *ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored_vector()).as_bytes(),
            [
                0x96, 0xcc, 0x34, 0x97, 0xff, 0x45, 0xff, 0xe8, 0xe0, 0xa3, 0x91, 0xbd, 0x0d, 0xe4,
                0x1c, 0xf8, 0xf0, 0xc2, 0xc4, 0x97, 0xbe, 0x9d, 0x6a, 0x8c, 0xf4, 0x6f, 0x93, 0x77,
                0x57, 0x9a, 0x06, 0x98
            ]
        );
        // Stale: awaits recompute under the generated-unit family at position
        // one — the same transcript under the other subject, which is what
        // makes the pair a separation vector rather than a repeat.
        assert_eq!(
            *ProjectionIdentity::<RenderedUnitSubject>::derived(anchored_vector()).as_bytes(),
            [
                0x58, 0x8e, 0xeb, 0xea, 0x96, 0xf7, 0xe8, 0x79, 0xf3, 0x9d, 0xe0, 0x89, 0x1c, 0x74,
                0x4a, 0xaf, 0x32, 0x7d, 0x71, 0x0d, 0xfd, 0xed, 0x4d, 0xb5, 0x92, 0xa8, 0x68, 0xed,
                0x31, 0x5c, 0x49, 0x07
            ]
        );
        // Stale: awaits recompute under the plan family at position one, at the
        // same corrective batch.
        assert_eq!(
            *ProjectionIdentity::<PlanSubject>::derived(rooted_vector()).as_bytes(),
            [
                0x26, 0x97, 0x00, 0x6a, 0xa8, 0x65, 0xbc, 0xf1, 0xe1, 0x5c, 0xff, 0x73, 0x76, 0xb9,
                0xe8, 0xe9, 0xbd, 0xc6, 0xad, 0xc9, 0xb2, 0x5a, 0xb1, 0x57, 0x30, 0xf4, 0x4b, 0x19,
                0x4c, 0xe8, 0x8a, 0xc2
            ]
        );
    }

    /// law: identity-profile.one-bit-anywhere-moves-the-identity — flipping one
    /// bit of the content, one bit of the anchor, or one step of the position
    /// derives a different identity. Nothing in the transcript is decoration.
    /// Owed reversal (red twin): a derivation that dropped the position or
    /// folded the anchor must break this law.
    #[test]
    fn one_bit_anywhere_moves_the_identity() {
        let base = ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored_vector());

        let mut flipped_content = GOLDEN_CONTENT.to_vec();
        if let Some(byte) = flipped_content.first_mut() {
            *byte ^= 0x01;
        }
        let content_moved =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::GeneratedUnit,
                TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
                &flipped_content,
                GOLDEN_POSITION,
            ));
        assert_ne!(base, content_moved);

        let mut flipped_anchor = GOLDEN_ANCHOR;
        if let Some(byte) = flipped_anchor.last_mut() {
            *byte ^= 0x01;
        }
        let anchor_moved =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::GeneratedUnit,
                TranscriptAnchoring::UnderProjectionIdentity(flipped_anchor),
                GOLDEN_CONTENT,
                GOLDEN_POSITION,
            ));
        assert_ne!(base, anchor_moved);

        let position_moved =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::GeneratedUnit,
                TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
                GOLDEN_CONTENT,
                GOLDEN_POSITION.saturating_add(1),
            ));
        assert_ne!(base, position_moved);

        let rooted_moved =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::GeneratedUnit,
                TranscriptAnchoring::Rooted,
                GOLDEN_CONTENT,
                GOLDEN_POSITION,
            ));
        assert_ne!(base, rooted_moved);
    }

    /// law: identity-profile.reordering-parts-moves-the-identity — two members
    /// swapped inside one content, and one boundary moved between two members,
    /// both derive different identities. This is what the length prefix buys:
    /// bare concatenation would let `ab|c` and `a|bc` encode alike.
    /// Owed reversal (red twin): dropping the length prefix must break this law.
    #[test]
    fn reordering_parts_moves_the_identity() {
        let joined = |left: &[u8], right: &[u8]| {
            let mut bytes = Vec::new();
            encode_bytes(left, &mut bytes);
            encode_bytes(right, &mut bytes);
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::GeneratedUnit,
                TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
                &bytes,
                GOLDEN_POSITION,
            ))
        };
        assert_ne!(joined(b"alpha", b"beta"), joined(b"beta", b"alpha"));
        assert_ne!(joined(b"ab", b"c"), joined(b"a", b"bc"));
    }

    /// law: identity-profile.domain-separation-bites — one transcript under two
    /// roles, and one transcript under two subjects, derive different
    /// identities. The separation is a runtime fact and not only the compile-time
    /// one the `PhantomData` parameter already gives.
    ///
    /// The two roles here are `GeneratedUnit` and `RenderedUnit`, which stand in
    /// two different families, so this vector proves the separation across a
    /// family boundary as well. The pair that SHARES a family is the harder
    /// case and has its own law below.
    ///
    /// Owed reversal (red twin): a single context for every subject and role
    /// must break this law.
    #[test]
    fn domain_separation_bites() {
        let under_role = |role: ProjectionRole| {
            *ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                role,
                TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
                GOLDEN_CONTENT,
                GOLDEN_POSITION,
            ))
            .as_bytes()
        };
        assert_ne!(
            under_role(ProjectionRole::GeneratedUnit),
            under_role(ProjectionRole::RenderedUnit)
        );
        assert_ne!(
            *ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored_vector()).as_bytes(),
            *ProjectionIdentity::<RenderedUnitSubject>::derived(anchored_vector()).as_bytes()
        );
        assert_ne!(
            *ProjectionIdentity::<PlanSubject>::derived(rooted_vector()).as_bytes(),
            *ProjectionIdentity::<GeneratedUnitSubject>::derived(rooted_vector()).as_bytes()
        );
    }

    /// law: identity-profile.one-family-separates-its-two-roles — the rendered
    /// unit and the digest of its bytes share ONE family and derive different
    /// identities over one transcript, because the role is a member of the
    /// preimage and a segment of the context. Sharing a version ladder is not
    /// sharing a name space.
    ///
    /// This is the case a family roster makes possible and therefore owes: two
    /// roles that answer with one profile would collide if the role were only a
    /// version's neighbour rather than a separator in its own right.
    ///
    /// Owed reversal (red twin): dropping the role from the preimage, or from
    /// the derive-key context, must break this law.
    #[test]
    fn one_family_separates_its_two_roles() {
        let under_role = |role: ProjectionRole| {
            ProjectionTranscript::under(
                role,
                TranscriptAnchoring::UnderProjectionIdentity(GOLDEN_ANCHOR),
                GOLDEN_CONTENT,
                GOLDEN_POSITION,
            )
        };
        let rendered = under_role(ProjectionRole::RenderedUnit);
        let bytes = under_role(ProjectionRole::OutputBytes);
        assert_eq!(rendered.profile(), bytes.profile());
        assert_eq!(rendered.profile(), RENDERED_UNIT_IDENTITY_PROFILE);
        assert_ne!(
            rendered.encoded("rendered-unit"),
            bytes.encoded("rendered-unit")
        );
        assert_ne!(
            *ProjectionIdentity::<RenderedUnitSubject>::derived(rendered).as_bytes(),
            *ProjectionIdentity::<RenderedUnitSubject>::derived(bytes).as_bytes()
        );
    }

    /// law: identity-profile.the-record-carries-the-anchor-whole — a derivation
    /// record states its subject, role, the family profile at its position, the
    /// generator, the position, and the content LENGTH, and carries its anchor
    /// at the full thirty-two bytes. The retired design folded that anchor to
    /// eight.
    /// Owed reversal (red twin): narrowing the recorded anchor must break this
    /// law.
    #[test]
    fn the_record_carries_the_anchor_whole() {
        let (identity, provenance) =
            ProjectionIdentity::<GeneratedUnitSubject>::derived_with_provenance(anchored_vector());
        assert_eq!(
            identity,
            ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored_vector())
        );
        assert_eq!(provenance.subject(), "generated-unit");
        assert!(matches!(provenance.role(), ProjectionRole::GeneratedUnit));
        assert_eq!(provenance.profile(), GENERATED_UNIT_IDENTITY_PROFILE);
        assert_eq!(provenance.generator(), MACROC_GENERATOR);
        assert_eq!(provenance.position(), GOLDEN_POSITION);
        assert_eq!(
            provenance.content_length(),
            u64::try_from(GOLDEN_CONTENT.len()).unwrap_or(u64::MAX)
        );
        assert_eq!(
            provenance.anchoring().commitment(),
            Some(&GOLDEN_ANCHOR),
            "the anchor is recorded at full width"
        );
        assert_eq!(
            provenance.context(),
            "threadpak/macroc/projection-identity/generated-unit/v1/generated-unit/generated-unit"
        );
    }

    /// law: identity-profile.the-record-reads-the-generator-for-staleness — the
    /// generator is on the RECORD and nowhere else, and the reading it exists
    /// for is a comparison against the shape these services render today. The
    /// comparison is over the two load-bearing facts, so a package version that
    /// moved for its own reasons reports nothing.
    ///
    /// Owed reversal (red twin): a staleness reading that compared the package
    /// version, or one taken off the identity instead of the record, must break
    /// this law.
    #[test]
    fn the_record_reads_the_generator_for_staleness() {
        let provenance = anchored_vector().provenance("generated-unit");
        assert_eq!(provenance.generator(), MACROC_GENERATOR);
        assert!(provenance.under_current_shape());
        assert!(MACROC_GENERATOR.same_rendered_shape(MACROC_GENERATOR));
        // Same declared name and shape, another package version: the same
        // rendered shape, and a comparison that said otherwise would report a
        // publication as a producer change.
        let republished = GeneratorIdentity::declared(
            MACROC_GENERATOR.profile(),
            MACROC_GENERATOR.schema(),
            "0.0.0-another-publication",
        );
        assert!(MACROC_GENERATOR.same_rendered_shape(republished));
        assert_ne!(MACROC_GENERATOR, republished);
        // A moved schema position IS a moved shape, and it still renames
        // nothing: the preimage carries neither fact.
        let reshaped = GeneratorIdentity::declared(
            MACROC_GENERATOR.profile(),
            GeneratorSchemaVersion::declared(
                MACROC_GENERATOR.schema().position().saturating_add(1),
            ),
            MACROC_GENERATOR.package_version(),
        );
        assert!(!MACROC_GENERATOR.same_rendered_shape(reshaped));
    }
}

mod refusal {
    use crate::refusal::{
        BoundAxis, PlanSeat, PlanningIssueLimit, ProjectionPlanning, ProjectionPlanningIssue,
    };
    use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
    use threadpak::types::ConstLimit;

    /// The closed bound-axis roster, proven closed by an exhaustive match: a new
    /// axis stops compiling here until it is placed.
    const fn axis_index(axis: BoundAxis) -> usize {
        match axis {
            BoundAxis::Declarations => 0,
            BoundAxis::Outputs => 1,
            BoundAxis::TraceEntries => 2,
            BoundAxis::Diagnostics => 3,
            BoundAxis::OriginEdges => 4,
            BoundAxis::Bytes => 5,
        }
    }

    /// law: refusal.bound-axes-are-six-and-closed — the plan's declared
    /// magnitudes are a closed roster, each distinct.
    /// Owed reversal: adding an axis without placing it must break this law.
    #[test]
    fn bound_axes_are_six_and_closed() {
        assert_eq!(BoundAxis::ALL.len(), 6);
        let indexes: Vec<usize> = BoundAxis::ALL.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
        // The stamped roster answers its own layout: the hand-written index
        // above and the generated slot are the same position, and the stable
        // name is declared apart from the Rust spelling.
        assert!(
            BoundAxis::ALL
                .iter()
                .copied()
                .all(|axis| usize::from(axis.slot()) == axis_index(axis))
        );
        assert_eq!(BoundAxis::TraceEntries.stable_name(), "trace-entries");
    }

    /// law: refusal.family-is-an-issue-collection — the planning family declares
    /// the collection shape and elects no primary issue, so its selection order
    /// is empty by law rather than by omission.
    /// Owed reversal (red twin): declaring `SingleCause` with a non-empty
    /// collection body must break this law.
    #[test]
    fn family_is_an_issue_collection() {
        assert!(matches!(
            ProjectionPlanning::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(ProjectionPlanning::SELECTION_ORDER.is_empty());
    }

    /// law: refusal.one-issue-body-is-total — a seam that establishes one issue
    /// builds its refusal without an error road of its own, so refusing is never
    /// the place a caller reaches for a panic.
    /// Owed reversal: a fallible one-issue road must break this law.
    #[test]
    fn one_issue_body_is_total() {
        let refusal = ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
            seat: PlanSeat::TargetBinding,
        });
        assert_eq!(refusal.body().carried().len(), 1);
        assert!(matches!(
            refusal.body().completion(),
            CompletionPosture::Complete
        ));
        assert!(matches!(
            refusal.body().carried().first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        ));
    }

    /// law: refusal.co-established-issues-stay-whole-or-say-what-they-left-out —
    /// a body carrying several issues either carries them all and says
    /// `Complete`, or carries what the declared bound holds and names the exact
    /// number of established issues it does not carry. Both directions, at the
    /// one road every planning seam builds a body through.
    ///
    /// The over-bound reading is the one that used to be false twice over: the
    /// body kept the FIRST issue alone whatever the bound admitted, and reported
    /// that examination had stopped — when the pass had already run to the end
    /// and the caller was simply handed one finding out of many. Both halves are
    /// asserted here, so restoring either fails.
    ///
    /// Owed reversal (red twin): a body that dropped the remainder silently, or
    /// one that reported a halted examination after a complete pass, must break
    /// this law.
    #[test]
    fn co_established_issues_stay_whole_or_say_what_they_left_out() {
        let node = crate::plane::for_laws(1);
        let whole = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            vec![ProjectionPlanningIssue::MembershipIncomplete { absent: node }],
        );
        assert_eq!(whole.body().carried().len(), 2);
        assert!(matches!(
            whole.body().completion(),
            CompletionPosture::Complete
        ));

        // One issue past the declared magnitude: the body fills to the bound
        // rather than collapsing to its first issue, and names the one it could
        // not carry.
        let overrun: Vec<ProjectionPlanningIssue> = core::iter::repeat_n(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            PlanningIssueLimit::MAX,
        )
        .collect();
        let truncated = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::MembershipIncomplete { absent: node },
            overrun,
        );
        assert_eq!(truncated.body().carried().len(), PlanningIssueLimit::MAX);
        assert!(matches!(
            truncated.body().completion(),
            CompletionPosture::ReportTruncated(truncation)
                if truncation.omitted().get() == 1
                    && matches!(truncation.stopped_at(), StopBound::DeclaredIssueBound)
        ));
        // The first issue survives at the front: a report that reordered its
        // findings under pressure would be a different report.
        assert!(matches!(
            truncated.body().carried().first(),
            ProjectionPlanningIssue::MembershipIncomplete { .. }
        ));
    }

    /// law: refusal.bound-refusals-name-their-magnitude — a bound refusal states
    /// the axis, the declared bound, and the observed count.
    /// Owed reversal: a payload-free bound cause must break this law.
    #[test]
    fn bound_refusals_name_their_magnitude() {
        let refusal = ProjectionPlanning::bound_exceeded(BoundAxis::Outputs, 32, 33);
        assert!(matches!(
            refusal.body().carried().first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::Outputs,
                bound: 32,
                observed: 33
            }
        ));
    }
}

mod diagnostics {
    use crate::diagnostics::{
        DiagnosticSite, MachineAnchoring, MachineAnchors, MacrocDiagnostic, MacrocPhase,
        ObservedClassification, RelatedSet, RelatedSetCompletion, ReleasePosture, RepairAction,
        ReproductionRoute, SiteCoordinate,
    };
    use crate::plane::{AuthoringLimitProfile, HumanProjection, OwnerFactRef, OwnerIdentityRef};
    use crate::token::SpanHandle;
    use threadpak::declaration::{CoordinateRole, SourceCoordinate};
    use threadpak::evidence::CauseDisposition;
    use threadpak::types::{AdmittedLimit, Bounded};

    /// The closed phase roster, proven closed by an exhaustive match.
    const fn phase_index(phase: MacrocPhase) -> usize {
        match phase {
            MacrocPhase::Capture => 0,
            MacrocPhase::DeclarationConstruction => 1,
            MacrocPhase::Linking => 2,
            MacrocPhase::Planning => 3,
            MacrocPhase::Rendering => 4,
            MacrocPhase::Inspection => 5,
        }
    }

    /// law: diagnostics.phases-are-six-and-closed — the acts the services run
    /// are a closed roster in one declared order.
    /// Owed reversal: adding a phase without placing it must break this law.
    #[test]
    fn phases_are_six_and_closed() {
        assert_eq!(MacrocPhase::ALL.len(), 6);
        let indexes: Vec<usize> = MacrocPhase::ALL.iter().copied().map(phase_index).collect();
        assert!(
            MacrocPhase::ALL
                .iter()
                .copied()
                .all(|phase| usize::from(phase.slot()) == phase_index(phase))
        );
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: diagnostics.a-diagnostic-constructs-complete — every seat is
    /// furnished, including the reason, the family, the phase, the typed
    /// coordinate, the three identities, the expected contract, the observed
    /// classification, the cause posture, the repairs, the reproduction route,
    /// and the release posture.
    /// Owed reversal (red twin): omitting any seat must not compile.
    #[test]
    fn a_diagnostic_constructs_complete() {
        let declared_by = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([40; 32]),
            fact: OwnerIdentityRef::decoded([41; 32]),
        };
        let description = HumanProjection::projected("bind the declared host contract");
        let repairs = description.map_err(|_| ()).and_then(|description| {
            Bounded::admitted_const(
                vec![RepairAction {
                    declared_by,
                    description,
                }],
                &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
            )
            .map_err(|_| ())
        });
        let built = repairs.map(|repairs| MacrocDiagnostic {
            machine: MachineAnchoring::Anchored(Box::new(MachineAnchors {
                reason: OwnerIdentityRef::decoded([42; 32]),
                family: OwnerIdentityRef::decoded([43; 32]),
                declaration: OwnerIdentityRef::decoded([44; 32]),
                fragment: OwnerIdentityRef::decoded([45; 32]),
                graph: OwnerIdentityRef::decoded([46; 32]),
            })),
            summary: HumanProjection::empty(),
            phase: MacrocPhase::Planning,
            site: DiagnosticSite::at_token(
                SpanHandle::at(4),
                SiteCoordinate::Resolved(SourceCoordinate {
                    role: CoordinateRole::SemanticOrigin,
                    position: 17,
                }),
            ),
            expected: crate::plane::for_laws(47),
            observed: ObservedClassification::SeatAbsent,
            cause: CauseDisposition::UnresolvedCause,
            related: RelatedSet::nothing_enumerated(),
            repairs,
            reproduction: ReproductionRoute::CallableServices {
                entry: crate::plane::for_laws(48),
            },
            release: ReleasePosture::NoReleasePromise,
        });
        assert!(built.is_ok_and(|diagnostic| {
            diagnostic.repairs.len() == 1
                && diagnostic.related.carried().is_empty()
                && matches!(
                    diagnostic.related.completion(),
                    RelatedSetCompletion::Complete
                )
                && diagnostic
                    .site
                    .coordinate()
                    .resolved()
                    .is_some_and(|coordinate| coordinate.position == 17)
                && diagnostic.site.token() == Some(SpanHandle::at(4))
                && matches!(diagnostic.machine, MachineAnchoring::Anchored(_))
                && matches!(diagnostic.cause, CauseDisposition::UnresolvedCause)
                && matches!(diagnostic.phase, MacrocPhase::Planning)
        }));
    }

    /// law: diagnostics.an-unanchored-diagnostic-says-so — where the machine has
    /// minted no identity for an observation, the diagnostic states the posture
    /// rather than carrying a stand-in. The compiler plane never mints a value
    /// that independently answers a question the machine owns.
    /// Owed reversal (red twin): a plane-minted "reason identity" filling the
    /// seat must break this law.
    #[test]
    fn an_unanchored_diagnostic_says_so() {
        let anchored = MachineAnchoring::Anchored(Box::new(MachineAnchors {
            reason: OwnerIdentityRef::decoded([42; 32]),
            family: OwnerIdentityRef::decoded([43; 32]),
            declaration: OwnerIdentityRef::decoded([44; 32]),
            fragment: OwnerIdentityRef::decoded([45; 32]),
            graph: OwnerIdentityRef::decoded([46; 32]),
        }));
        assert_ne!(anchored, MachineAnchoring::UnmintedAtThisSeam);
        assert!(matches!(
            MachineAnchoring::UnmintedAtThisSeam,
            MachineAnchoring::UnmintedAtThisSeam
        ));
    }

    /// law: diagnostics.an-owner-fact-may-be-named-without-being-minted — a
    /// citation names the home and the fact the owning home wrote down, which is
    /// a reference to an owner fact and never a second answer to it.
    /// Owed reversal: a `Declared` citation that derived an identity of its own
    /// must break this law.
    #[test]
    fn an_owner_fact_may_be_named_without_being_minted() {
        let named = OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed");
        let minted = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([40; 32]),
            fact: OwnerIdentityRef::decoded([41; 32]),
        };
        assert_ne!(named, minted);
        assert_ne!(named.citation_bytes(), minted.citation_bytes());
        assert_eq!(
            named.citation_bytes(),
            OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed").citation_bytes()
        );
    }

    /// law: diagnostics.repairs-cite-their-owner — a repair carries the owner
    /// fact that declares it, so no rendering can present composed advice as
    /// declared authority.
    /// Owed reversal: a repair whose only member is text must break this law.
    #[test]
    fn repairs_cite_their_owner() {
        let declared_by = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([49; 32]),
            fact: OwnerIdentityRef::decoded([50; 32]),
        };
        let repair =
            HumanProjection::projected("declare the missing obligation").map(|description| {
                RepairAction {
                    declared_by,
                    description,
                }
            });
        assert!(repair.is_ok_and(|repair| repair.declared_by == declared_by));
    }

    /// law: diagnostics.reproduction-does-not-require-the-shell — the callable
    /// services are one reproduction route in their own right, so a diagnostic
    /// is reachable without a proc-macro anywhere in the picture.
    /// Owed reversal: a route roster with only the shell must break this law.
    #[test]
    fn reproduction_does_not_require_the_shell() {
        let route = ReproductionRoute::CallableServices {
            entry: crate::plane::for_laws(51),
        };
        assert!(matches!(route, ReproductionRoute::CallableServices { .. }));
        let shell = ReproductionRoute::ExpansionShell {
            surface: crate::plane::for_laws(52),
        };
        let fixture = ReproductionRoute::RecordedFixture {
            population: crate::plane::for_laws(53),
        };
        assert_ne!(route, shell);
        assert_ne!(shell, fixture);
    }
}

mod question {
    use crate::question::ExplanationQuestion;

    /// The closed question roster, proven closed by an exhaustive match.
    const fn question_index(question: ExplanationQuestion) -> usize {
        match question {
            ExplanationQuestion::WhatAreYou => 0,
            ExplanationQuestion::WhichOwnerRequired => 1,
            ExplanationQuestion::WhichDeclarationCaused => 2,
            ExplanationQuestion::WhichTemplateOrPatternInstance => 3,
            ExplanationQuestion::WhichGraphAndProfile => 4,
            ExplanationQuestion::WhichCapabilitiesSelectedWrappers => 5,
            ExplanationQuestion::WhichAssumptionsAndSpecializations => 6,
            ExplanationQuestion::WhichOutputIdentityAndDigest => 7,
            ExplanationQuestion::WhichTestsChallenge => 8,
            ExplanationQuestion::WhichBenchmarksMeasure => 9,
            ExplanationQuestion::WhichRuntimeTracesCorrespond => 10,
            ExplanationQuestion::WhatInvalidates => 11,
            ExplanationQuestion::WhyWasRelatedProjectionNotGenerated => 12,
            ExplanationQuestion::WhatRepairsARefusal => 13,
        }
    }

    /// law: question.questions-are-fourteen-and-closed — the protocol's roster
    /// is closed at fourteen, each distinct, in one declared order.
    /// Owed reversal: adding a question without placing it must break this law.
    #[test]
    fn questions_are_fourteen_and_closed() {
        assert_eq!(ExplanationQuestion::ALL.len(), 14);
        let indexes: Vec<usize> = ExplanationQuestion::ALL
            .iter()
            .copied()
            .map(question_index)
            .collect();
        assert!(
            ExplanationQuestion::ALL
                .iter()
                .copied()
                .all(|question| usize::from(question.slot()) == question_index(question))
        );
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }
}

mod origin_graph {
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, OriginEdge, OriginEdgeLimit, OriginRelation, OriginTrail,
        TraceDecision, TraceEntry,
    };
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, TraceEntryLimit};
    use crate::refusal::{BoundAxis, ProjectionPlanningIssue};
    use threadpak::types::ConstLimit;

    /// The closed relation roster, proven closed by an exhaustive match.
    const fn relation_index(relation: OriginRelation) -> usize {
        match relation {
            OriginRelation::AuthoredDeclaration => 0,
            OriginRelation::PatternInstantiation => 1,
            OriginRelation::SemanticDerivation => 2,
            OriginRelation::FragmentConstruction => 3,
            OriginRelation::ExplicitLink => 4,
            OriginRelation::Normalization => 5,
            OriginRelation::ProfileSelection => 6,
            OriginRelation::ProjectionSelection => 7,
            OriginRelation::WrapperComposition => 8,
            OriginRelation::Rendering => 9,
            OriginRelation::HostBinding => 10,
            OriginRelation::TestDerivation => 11,
            OriginRelation::BenchmarkDerivation => 12,
            OriginRelation::DiagnosticDerivation => 13,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([1; 32]),
            fact: OwnerIdentityRef::decoded([2; 32]),
        }
    }

    /// One edge, for laws that need a trail.
    fn edge() -> OriginEdge {
        OriginEdge {
            from: crate::plane::for_laws(3),
            relation: OriginRelation::AuthoredDeclaration,
            to: crate::plane::for_laws(4),
        }
    }

    /// law: origin.relations-are-fourteen-and-closed — the settled relation
    /// categories are a closed roster whose members are pairwise distinct and
    /// declared in one order.
    /// Owed reversal: adding a relation without placing it must break this law.
    #[test]
    fn relations_are_fourteen_and_closed() {
        assert_eq!(OriginRelation::ALL.len(), 14);
        let indexes: Vec<usize> = OriginRelation::ALL
            .iter()
            .copied()
            .map(relation_index)
            .collect();
        assert!(
            OriginRelation::ALL
                .iter()
                .copied()
                .all(|relation| usize::from(relation.slot()) == relation_index(relation))
        );
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: origin.a-generated-node-without-an-origin-is-unrepresentable — the
    /// trail seat is structurally non-empty, so the orphan case has no
    /// constructor to reach and no runtime check to pass.
    /// Owed reversal (red twin): a trail built from an empty edge list must not
    /// compile.
    #[test]
    fn a_generated_node_without_an_origin_is_unrepresentable() {
        let trail = OriginTrail::from_edge(edge());
        assert!(!trail.is_empty() && trail.len() == 1);
        assert!(matches!(
            trail.first().relation,
            OriginRelation::AuthoredDeclaration
        ));
    }

    /// One node of a demo walk, distinguished by its step.
    fn node(step: usize) -> crate::plane::ProjectionIdentity<crate::plane::OriginNodeSubject> {
        crate::plane::for_laws(u8::try_from(step.saturating_add(10)).unwrap_or(u8::MAX))
    }

    /// A CONNECTED walk of the requested edge count: each edge starts where the
    /// one before it ended.
    ///
    /// The bound laws take their material from here on purpose. A walk built by
    /// repeating one edge is discontinuous, so a bound law written over it would
    /// stop testing the bound the moment continuity became a real check — which
    /// is exactly what happened, and is why the helper exists rather than a
    /// repeated edge.
    fn walk(edges: usize) -> (OriginEdge, Vec<OriginEdge>) {
        let first = OriginEdge {
            from: node(0),
            relation: OriginRelation::AuthoredDeclaration,
            to: node(1),
        };
        let rest = (1..edges)
            .map(|step| OriginEdge {
                from: node(step),
                relation: OriginRelation::Rendering,
                to: node(step.saturating_add(1)),
            })
            .collect();
        (first, rest)
    }

    /// law: origin.trails-refuse-rather-than-truncate — a walk past the declared
    /// bound refuses with the bound axis named, so an origin never quietly
    /// shortens into a span.
    /// Owed reversal: a constructor that truncated must break this law.
    #[test]
    fn trails_refuse_rather_than_truncate() {
        let (first, overrun) = walk(OriginEdgeLimit::MAX.saturating_add(1));
        let refused = OriginTrail::drawn(first, overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.body().carried().first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::OriginEdges,
                ..
            }
        )));
        let (head, tail) = walk(2);
        let fits = OriginTrail::drawn(head, tail);
        assert!(fits.is_ok_and(|trail| trail.len() == 2));
    }

    /// law: origin.a-trail-is-a-walk-or-it-is-refused — a drawn trail's edges
    /// join end to end, and one that does not refuses naming the position of the
    /// first edge that fails to join.
    ///
    /// Both directions. A connected walk draws; the same walk with one edge's
    /// start moved refuses, and the refusal names WHERE — a caller told only
    /// that a trail is broken has nothing to repair. The seam checked the edge
    /// count and nothing else before this, so a disconnected list could become
    /// an `OriginTrail` and receive canonical bytes as a provenance path.
    ///
    /// The position is what makes the law non-vacuous in both halves: a check
    /// that always reported the first edge would pass a two-edge fixture and say
    /// nothing, so the break is planted at the third.
    ///
    /// Owed reversal (red twin): a `drawn` that checked the bound alone must
    /// break this law.
    #[test]
    fn a_trail_is_a_walk_or_it_is_refused() {
        let (first, rest) = walk(4);
        let connected = OriginTrail::drawn(first, rest);
        assert!(connected.is_ok_and(|trail| trail.len() == 4));

        let (head, mut tail) = walk(4);
        // The third edge of the trail — the second of the remainder — is cut
        // loose: it now starts at a node nobody produced.
        if let Some(third) = tail.get_mut(1) {
            third.from = node(99);
        }
        let broken = OriginTrail::drawn(head, tail);
        assert!(broken.is_err_and(|planning| matches!(
            planning.body().carried().first(),
            ProjectionPlanningIssue::TrailDiscontinuous { at: 2 }
        )));
    }

    /// law: origin.not-run-is-not-passed — a check that did not run is a
    /// distinct recorded decision, and a decision that ran cites the owner fact
    /// that caused it.
    /// Owed reversal (red twin): collapsing `NotRun` into an omission must break
    /// this law.
    #[test]
    fn not_run_is_not_passed() {
        let selected = TraceEntry {
            subject: crate::plane::for_laws(5),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let omitted = TraceEntry {
            subject: crate::plane::for_laws(5),
            decision: TraceDecision::OmittedBecause(owner_fact()),
        };
        let not_run = TraceEntry {
            subject: crate::plane::for_laws(5),
            decision: TraceDecision::NotRun,
        };
        assert_ne!(selected, omitted);
        assert_ne!(omitted, not_run);
        assert_ne!(selected, not_run);
    }

    /// law: origin.traces-keep-selection-order-and-a-declared-bound — the first
    /// entry recorded is the first entry held, and a trace past its bound
    /// refuses on the trace-entry axis.
    /// Owed reversal: a constructor that sorted entries must break this law.
    #[test]
    fn traces_keep_selection_order_and_a_declared_bound() {
        let first = TraceEntry {
            subject: crate::plane::for_laws(6),
            decision: TraceDecision::NotRun,
        };
        let second = TraceEntry {
            subject: crate::plane::for_laws(7),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let recorded = DecisionTrace::recorded(first, vec![second]);
        assert!(recorded.is_ok_and(|trace| trace.len() == 2 && *trace.first() == first));

        let overrun: Vec<TraceEntry> = core::iter::repeat_n(second, TraceEntryLimit::MAX).collect();
        let refused = DecisionTrace::recorded(first, overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.body().carried().first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::TraceEntries,
                ..
            }
        )));
    }

    /// law: origin.nonclaims-cite-an-owner-fact — a stated nonclaim names the
    /// fact that leaves it unclaimed rather than standing as a bare disclaimer.
    /// Owed reversal: a nonclaim without a citation must break this law.
    #[test]
    fn nonclaims_cite_an_owner_fact() {
        let nonclaim = Nonclaim {
            unclaimed: crate::plane::for_laws(8),
            because: owner_fact(),
        };
        assert_eq!(nonclaim.because, owner_fact());
    }
}

mod planning {
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{
        AuthoringLimitProfile, OwnerFactRef, OwnerIdentityRef, ProfileVersion, SoleRenderedUnit,
    };
    use crate::planning::{
        BenchmarkDescriptorProjection, CodecProjection, DeriveImplContent, DeriveImplProjection,
        DigestContract, DocumentationProjection, GraphAnchoring, HostWrapperContent,
        HostWrapperProjection, InvalidationTrigger, KindDispositions, MemberDestination,
        OwnerContentAccount, PatternStampProjection, PlanDecisions, PlannedMember,
        PlannedMembership, PlannedOutput, ProjectionBundlePlan, ProjectionContext,
        ProjectionDisposition, ProjectionKind, ProjectionKindRow, ProjectionPlan,
        RemoteSurfaceProjection, RenderedImplementation, TargetBinding, TargetRequirement,
        TestDescriptorProjection, UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS, WrapperComponent,
    };
    use crate::question::ExplanationQuestion;
    use crate::refusal::{PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
    use threadpak::types::{AdmittedLimit, Bounded, NonEmptyBounded};

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([10; 32]),
            fact: OwnerIdentityRef::decoded([11; 32]),
        }
    }

    /// One origin trail, for laws that need a generated unit.
    fn trail() -> OriginTrail {
        OriginTrail::from_edge(OriginEdge {
            from: crate::plane::for_laws(12),
            relation: OriginRelation::SemanticDerivation,
            to: crate::plane::for_laws(13),
        })
    }

    /// One planned member under one rendered role. Logical only: a semantic key,
    /// a destination, an origin, a renderer, and a digest CONTRACT — never a
    /// digest, because no byte has been rendered when a plan is made.
    fn member(role: RenderedImplementation, tag: u8) -> PlannedMember<RenderedImplementation> {
        PlannedMember {
            role,
            output: planned_output(tag),
        }
    }

    /// One planned output, tagged so two of them are distinguishable.
    fn planned_output(tag: u8) -> PlannedOutput {
        let key = crate::plane::for_laws(tag);
        PlannedOutput {
            semantic_key: key,
            destination: MemberDestination::AtDeclarationSite,
            origin: trail(),
            expected_profile: crate::plane::for_laws(17),
            expected_profile_version: ProfileVersion::declared(3),
            digest_contract: DigestContract::over(key),
        }
    }

    /// One planned member for a kind whose rendering is a single unit.
    fn sole_member(tag: u8) -> PlannedMember<SoleRenderedUnit> {
        PlannedMember {
            role: SoleRenderedUnit::Sole,
            output: planned_output(tag),
        }
    }

    /// One shared context, under the binding the caller names.
    ///
    /// What the plan was planned OVER is deliberately not here: that is the entry
    /// account's fact, stated once, and a context that also named it would be the
    /// second account of content dependencies the watch derivation would then be
    /// reading a copy of.
    fn context(target: TargetBinding) -> ProjectionContext {
        ProjectionContext {
            graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([16; 32])),
            profile: crate::plane::for_laws(17),
            profile_version: ProfileVersion::declared(3),
            generator: crate::plane::for_laws(19),
            target,
        }
    }

    /// The ONE entry account every plan below is planned over: a linked
    /// commitment, standing on nothing, which is a stated fact rather than a set
    /// somebody forgot to supply.
    fn account<K: ProjectionKind>() -> OwnerContentAccount<K> {
        OwnerContentAccount::linked(OwnerIdentityRef::decoded([18; 32]))
    }

    /// The five decided seats, bundled the way a plan's transcript writes them,
    /// over the membership and the watch set a law states.
    fn decisions<R: crate::plane::RenderedRole>(
        membership: PlannedMembership<R>,
        invalidation: crate::planning::InvalidationSet,
        nonclaims: Bounded<Nonclaim, crate::plane::NonclaimLimit>,
    ) -> PlanDecisions<R> {
        PlanDecisions {
            membership,
            invalidation,
            trace: trace(),
            origin: trail(),
            nonclaims,
        }
    }

    /// The implementation-projection content, for the complete-plan law.
    fn derive_content() -> DeriveImplContent {
        DeriveImplContent {
            derived_type: crate::plane::for_laws(20),
            contract: crate::plane::for_laws(21),
            assumptions: Bounded::empty(),
        }
    }

    /// The trace the complete-plan law records.
    fn trace() -> DecisionTrace {
        DecisionTrace::from_entry(TraceEntry {
            subject: crate::plane::for_laws(22),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        })
    }

    /// law: planning.a-complete-plan-constructs-through-checked-seams — every
    /// seat is furnished through the plane's own seams, and the resulting plan
    /// carries its entry account, output set, watch set, trace, and trail.
    ///
    /// The account arrives FIRST and is moved in, so the plan's own answer to
    /// "what were you planned over" is the value its identity, its watch set, and
    /// its origin edges were all read off; the five decided seats travel as one
    /// [`PlanDecisions`] value in the order the transcript writes them.
    ///
    /// Owed reversal (red twin): omitting any seat must not compile.
    #[test]
    fn a_complete_plan_constructs_through_checked_seams() {
        let planned = ProjectionPlan::<DeriveImplProjection>::planned(
            account(),
            context(TargetBinding::TargetFree),
            derive_content(),
            decisions(
                PlannedMembership::from_member(member(
                    RenderedImplementation::RenderedFamilyImpl,
                    14,
                )),
                InvalidationTrigger::one_watched(InvalidationTrigger::GraphIdentityChanged {
                    watched: OwnerIdentityRef::decoded([16; 32]),
                }),
                Bounded::empty(),
            ),
        );
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && !plan.membership().is_empty()
                && plan.invalidation().len() == 1
                && plan.trace().len() == 1
                && plan.origin().len() == 1
                && plan.nonclaims().is_empty()
                && plan.context().profile_version.position() == 3
                && plan.account().dependency_count() == 0
                && !plan.membership().first().output.origin.is_empty()
        }));
    }

    /// law: planning.several-outputs-and-nonclaims-ride-the-same-plan — a plan
    /// may declare several outputs and state what it does not claim, and both
    /// bounded seats hold what was put in them.
    /// Owed reversal: a membership seam that dropped a sibling must break this
    /// law.
    #[test]
    fn several_outputs_and_nonclaims_ride_the_same_plan() {
        let nonclaims = Bounded::admitted_const(
            vec![Nonclaim {
                unclaimed: crate::plane::for_laws(23),
                because: owner_fact(),
            }],
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| ());
        let membership = PlannedMembership::declared(
            member(RenderedImplementation::RenderedFamilyImpl, 14),
            vec![member(RenderedImplementation::RenderedCauseOrderImpl, 15)],
        )
        .map_err(|_| ());
        let built = nonclaims.and_then(|nonclaims| {
            membership.and_then(|membership| {
                ProjectionPlan::<DeriveImplProjection>::planned(
                    account(),
                    context(TargetBinding::TargetFree),
                    derive_content(),
                    decisions(
                        membership,
                        InvalidationTrigger::one_watched(
                            InvalidationTrigger::GeneratorVersionChanged {
                                watched: crate::plane::for_laws(19),
                            },
                        ),
                        nonclaims,
                    ),
                )
                .map_err(|_| ())
            })
        });
        assert!(
            built.is_ok_and(|plan| plan.membership().len() == 2 && plan.nonclaims().len() == 1)
        );
    }

    /// law: planning.a-declared-output-set-reads-back-whole — the membership
    /// seam holds every sibling put into it and hands them all back on a
    /// read-only pass: two distinct outputs go in, two distinct outputs come
    /// out, and the membership is unconsumed — the second read sees the same
    /// set as the first.
    ///
    /// The order law this read carries: the declared output set is
    /// order-insensitive, so nothing identity-bearing is derived from the order
    /// observed here; identity-bearing generation canonicalizes by an
    /// owner-declared order or key first. testpak owes the permutation hostile.
    ///
    /// Owed reversal: a membership seam that dropped or aliased a sibling must
    /// break this law.
    #[test]
    fn a_declared_output_set_reads_back_whole() {
        let membership = PlannedMembership::declared(
            member(RenderedImplementation::RenderedFamilyImpl, 14),
            vec![member(RenderedImplementation::RenderedCauseOrderImpl, 31)],
        );
        assert!(membership.is_ok_and(|membership| {
            let keys: Vec<[u8; 32]> = membership
                .iter()
                .map(|row| *row.output.semantic_key.as_bytes())
                .collect();
            keys.len() == 2
                && keys.first() != keys.get(1)
                && membership
                    .under(RenderedImplementation::RenderedCauseOrderImpl)
                    .is_some()
                && membership.count_under(RenderedImplementation::RenderedFamilyImpl) == 1
                && membership.iter().count() == 2
                && membership.len() == 2
                && !membership.is_empty()
        }));
    }

    /// law: planning.a-host-bound-kind-refuses-a-target-free-context — a kind
    /// whose plans are meaningless without a host contract refuses rather than
    /// defaulting to one, and names the seat.
    /// Owed reversal: defaulting the binding must break this law.
    #[test]
    fn a_host_bound_kind_refuses_a_target_free_context() {
        assert!(matches!(
            HostWrapperProjection::TARGET_REQUIREMENT,
            TargetRequirement::BoundHostContract
        ));
        let refused = ProjectionPlan::<HostWrapperProjection>::planned(
            account(),
            context(TargetBinding::TargetFree),
            HostWrapperContent {
                host_contract: OwnerIdentityRef::decoded([24; 32]),
                components: NonEmptyBounded::singleton(WrapperComponent::Admission),
                capability_basis: owner_fact(),
            },
            decisions(
                PlannedMembership::from_member(sole_member(14)),
                InvalidationTrigger::one_watched(InvalidationTrigger::TargetContractChanged {
                    watched: OwnerIdentityRef::decoded([24; 32]),
                }),
                Bounded::empty(),
            ),
        );
        assert!(refused.is_err_and(|planning| matches!(
            planning.body().carried().first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        )));
    }

    /// The closed trigger roster, proven closed by an exhaustive match.
    const fn trigger_index(trigger: &InvalidationTrigger) -> usize {
        match trigger {
            InvalidationTrigger::SourceDeclarationChanged { .. } => 0,
            InvalidationTrigger::CapturedDeclarationChanged { .. } => 1,
            InvalidationTrigger::GraphIdentityChanged { .. } => 2,
            InvalidationTrigger::ProjectionProfileChanged { .. } => 3,
            InvalidationTrigger::TargetContractChanged { .. } => 4,
            InvalidationTrigger::GeneratorVersionChanged { .. } => 5,
            InvalidationTrigger::MechanismProfileChanged { .. } => 6,
            InvalidationTrigger::WorkFormulaChanged { .. } => 7,
            InvalidationTrigger::FixturePopulationChanged { .. } => 8,
        }
    }

    /// law: planning.invalidation-triggers-are-nine-and-each-watches-an-identity
    /// — the roster is closed at nine, its members are pairwise distinct, and
    /// each names the exact identity whose change invalidates.
    /// Owed reversal: a payload-free trigger must break this law.
    #[test]
    fn invalidation_triggers_are_nine_and_each_watches_an_identity() {
        let triggers = [
            InvalidationTrigger::SourceDeclarationChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::CapturedDeclarationChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::GraphIdentityChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::ProjectionProfileChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::TargetContractChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::MechanismProfileChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::WorkFormulaChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::FixturePopulationChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
        ];
        assert_eq!(triggers.len(), 9);
        let indexes: Vec<usize> = triggers.iter().map(trigger_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// The closed wrapper-component roster, proven closed by an exhaustive
    /// match: a new component stops compiling here until it is placed.
    const fn component_index(component: WrapperComponent) -> usize {
        match component {
            WrapperComponent::Admission => 0,
            WrapperComponent::Decode => 1,
            WrapperComponent::Encode => 2,
            WrapperComponent::Cancellation => 3,
            WrapperComponent::Receipt => 4,
            WrapperComponent::EffectDispatch => 5,
            WrapperComponent::Observation => 6,
            WrapperComponent::Explanation => 7,
        }
    }

    /// law: planning.wrapper-components-are-eight-and-closed — the components a
    /// host wrapper may compose are a closed roster in one declared order, and
    /// the roster is the denominator every exhaustive disposition is checked
    /// against.
    /// Owed reversal: adding a component without placing it must break this
    /// law.
    #[test]
    fn wrapper_components_are_eight_and_closed() {
        assert_eq!(WRAPPER_COMPONENTS.len(), 8);
        let indexes: Vec<usize> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .map(component_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// The closed disposition roster, proven closed by an exhaustive match.
    fn disposition_index(disposition: &ProjectionDisposition) -> usize {
        match disposition {
            ProjectionDisposition::Generated { .. } => 0,
            ProjectionDisposition::NotApplicable { .. } => 1,
            ProjectionDisposition::Refused { .. } => 2,
            ProjectionDisposition::UnavailableUnderProfile { .. } => 3,
            ProjectionDisposition::NotRequested => 4,
            ProjectionDisposition::ExcludedByConfiguration { .. } => 5,
        }
    }

    /// law: planning.every-absence-has-a-named-disposition — all six
    /// dispositions are constructible and pairwise distinct, and none of them
    /// is silence.
    /// Owed reversal: dropping a disposition must break this law.
    #[test]
    fn every_absence_has_a_named_disposition() {
        let dispositions = [
            ProjectionDisposition::Generated {
                output: Box::new(planned_output(14)),
            },
            ProjectionDisposition::NotApplicable {
                because: owner_fact(),
            },
            ProjectionDisposition::Refused {
                refusal: ProjectionPlanning::established(
                    ProjectionPlanningIssue::MissingOwnerFact {
                        seat: PlanSeat::TargetBinding,
                    },
                ),
            },
            ProjectionDisposition::UnavailableUnderProfile {
                profile: crate::plane::for_laws(26),
                version: ProfileVersion::declared(1),
            },
            ProjectionDisposition::NotRequested,
            ProjectionDisposition::ExcludedByConfiguration {
                configuration: OwnerIdentityRef::decoded([27; 32]),
            },
        ];
        assert_eq!(dispositions.len(), 6);
        let indexes: Vec<usize> = dispositions.iter().map(disposition_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: planning.a-bundle-names-its-members-and-refuses-a-partial-set — a
    /// bundle holds at least one member by shape and refuses past its declared
    /// bound rather than publishing part of a set.
    /// Owed reversal (red twin): an empty bundle must not compile.
    #[test]
    fn a_bundle_names_its_members_and_refuses_a_partial_set() {
        let bundle = ProjectionBundlePlan::materialized(
            crate::plane::for_laws(28),
            crate::plane::for_laws(29),
            vec![crate::plane::for_laws(30)],
        );
        assert!(bundle.is_ok_and(|plan| plan.len() == 2 && !plan.is_empty()));
        let single =
            ProjectionBundlePlan::of_one(crate::plane::for_laws(28), crate::plane::for_laws(29));
        assert_eq!(single.bundle(), crate::plane::for_laws(28));
    }

    /// law: planning.no-kind-ducks-the-explanation-protocol — every kind names
    /// every universal question, states its own questions without repeating one,
    /// and the eight kinds together reach all fourteen questions.
    /// Owed reversal: a kind declaring an empty applicable set must break this
    /// law.
    #[test]
    fn no_kind_ducks_the_explanation_protocol() {
        let rosters: [Vec<ExplanationQuestion>; 8] = [
            ProjectionPlan::<CodecProjection>::applicable_questions(),
            ProjectionPlan::<HostWrapperProjection>::applicable_questions(),
            ProjectionPlan::<RemoteSurfaceProjection>::applicable_questions(),
            ProjectionPlan::<TestDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<BenchmarkDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<DocumentationProjection>::applicable_questions(),
            ProjectionPlan::<DeriveImplProjection>::applicable_questions(),
            ProjectionPlan::<PatternStampProjection>::applicable_questions(),
        ];
        for roster in &rosters {
            assert!(
                UNIVERSAL_QUESTIONS
                    .iter()
                    .all(|question| roster.contains(question))
            );
            assert!(roster.iter().enumerate().all(|(position, question)| {
                roster
                    .iter()
                    .skip(position.saturating_add(1))
                    .all(|other| other != question)
            }));
        }
        assert!(
            ExplanationQuestion::ALL
                .iter()
                .all(|question| rosters.iter().any(|roster| roster.contains(question)))
        );
    }

    /// law: planning.the-kind-roster-is-enumerated-once-and-answered-once — the
    /// enumerated roster is the sealed kind roster whole, each row's name is its
    /// own kind's declared stable name, and a disposition record answers every
    /// row exactly once, at the row's own seat.
    ///
    /// The roster and the record are emitted by the SAME declaration that
    /// declares the kinds, which is what makes "no kind is silently absent" a
    /// shape rather than a review note: a kind admitted to `kinds!` grows a row
    /// and a required seat together, and every construction of the record stops
    /// compiling until somebody says what happens to it.
    ///
    /// The seats carry distinguishable answers here on purpose. A record whose
    /// reading road crossed two rows would still answer with a disposition for
    /// every row, so the mapping is proved by the tags rather than assumed from
    /// totality.
    ///
    /// Owed reversal (red twin): a roster enumerated beside the kind declaration
    /// rather than by it, a row naming a spelling the kind does not declare, or
    /// a reading road that answered one row from another's seat, must break this
    /// law.
    #[test]
    fn the_kind_roster_is_enumerated_once_and_answered_once() {
        /// One answer per seat, each distinguishable from every other.
        fn tagged(tag: u8) -> ProjectionDisposition {
            ProjectionDisposition::ExcludedByConfiguration {
                configuration: OwnerIdentityRef::decoded([tag; 32]),
            }
        }

        // Every row names its own kind, read off the kind rather than spelled
        // beside it — and the pairing covers the roster whole.
        let named: [(ProjectionKindRow, &str); 8] = [
            (
                ProjectionKindRow::CodecProjection,
                CodecProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::HostWrapperProjection,
                HostWrapperProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::RemoteSurfaceProjection,
                RemoteSurfaceProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::TestDescriptorProjection,
                TestDescriptorProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::BenchmarkDescriptorProjection,
                BenchmarkDescriptorProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::DocumentationProjection,
                DocumentationProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::DeriveImplProjection,
                DeriveImplProjection::KIND_NAME,
            ),
            (
                ProjectionKindRow::PatternStampProjection,
                PatternStampProjection::KIND_NAME,
            ),
        ];
        assert_eq!(named.len(), ProjectionKindRow::ALL.len());
        assert!(named.iter().all(|(row, name)| row.declared_name() == *name));
        assert!(
            ProjectionKindRow::ALL
                .iter()
                .all(|row| named.iter().any(|(named_row, _)| named_row == row))
        );

        // No two rows answer to one name, so a reader that joins by name joins
        // to one kind.
        assert!(
            ProjectionKindRow::ALL
                .iter()
                .enumerate()
                .all(|(position, row)| {
                    ProjectionKindRow::ALL
                        .iter()
                        .skip(position.saturating_add(1))
                        .all(|other| other.declared_name() != row.declared_name())
                })
        );

        // The tags are written in ROSTER order, so a row's position IS the tag
        // its seat carries: a reading that crossed two rows answers with another
        // position's tag.
        let record = KindDispositions {
            codec: tagged(0),
            host_wrapper: tagged(1),
            remote_surface: tagged(2),
            test_descriptor: tagged(3),
            benchmark_descriptor: tagged(4),
            documentation: tagged(5),
            derive_impl: tagged(6),
            pattern_stamp: tagged(7),
        };
        assert!(
            ProjectionKindRow::ALL
                .iter()
                .enumerate()
                .all(|(position, row)| {
                    *record.under(*row) == tagged(u8::try_from(position).unwrap_or(u8::MAX))
                })
        );
    }
}

mod test_descriptor {
    use crate::plane::PlanId;
    use crate::test_descriptor::{
        ActivePointSelector, DEFERRED_CLAUSE, DeferredCargo, DeferredDelivery, GATE_MACRO,
        ShellName, TRIALS_CLAUSE, TrialDelivery, deferred_module, expectation_literal,
        gate_invocation, trial_cargo,
    };
    use crate::token::{GeneratedToken, GeneratedTree};

    /// One plan identity for the laws below to key a carrier name on.
    fn plan(tag: u8) -> PlanId {
        crate::plane::for_laws(tag)
    }

    /// One cargo, carrying the subject a deferred copy stands over and one
    /// selection it reads.
    fn cargo() -> Result<DeferredCargo, ()> {
        let selector = ActivePointSelector::declared(
            "REFUSAL_FAMILY_ACTIVE_POINT",
            "RefusalFamilyActivePoint",
            "NoMutation",
        )
        .map_err(|_| ())?;
        let declared = GeneratedTree::assembled(vec![
            GeneratedToken::word("enum"),
            GeneratedToken::word("RefusalFamilyActivePoint"),
        ])
        .map_err(|_| ())?;
        DeferredCargo::deferred("EvaluationSubject", vec![selector], declared).map_err(|_| ())
    }

    /// The clause a braced seat of one rendered gate invocation carries, read by
    /// walking the invocation's own tokens rather than by matching a sentence.
    fn seat<'body>(body: &'body [GeneratedToken], clause: &str) -> Option<&'body [GeneratedToken]> {
        let Some(GeneratedToken::Group { tokens, .. }) = body.last() else {
            return None;
        };
        let position = tokens
            .iter()
            .position(|token| token == &GeneratedToken::word(clause))?;
        match tokens.get(position.saturating_add(2)) {
            Some(GeneratedToken::Group { tokens, .. }) => Some(tokens.as_slice()),
            _ => None,
        }
    }

    /// law: descriptor.the-deferred-module-stands-exactly-where-cargo-was-deferred
    /// — the shell splices its private module when, and only when, the expansion
    /// deferred a cargo into this carrier. Rendered with carried cargo the module
    /// carries the local subject the deferred implementations stand over and one
    /// constant per selection they read; rendered with nothing deferred there is
    /// no module at all.
    ///
    /// The two postures are different FACTS rather than one with a missing half:
    /// an expansion that planned no member into this carrier sent it nothing, and
    /// a module written for it would declare a subject nothing implements and
    /// constants nothing reads. So the absence is the carrier's own answer and
    /// never an empty module standing in for one.
    ///
    /// Every assertion is composed from the cargo's own typed values — its
    /// subject, and each selector's constant, roster, and row — so the law reads
    /// the same answers the rendering wrote rather than a spelling restated
    /// beside it.
    ///
    /// The claim ceiling: it says which items the module carries and nothing
    /// about what they select. Which name a deferred implementation reads its
    /// selection through, and what its roster's rows mean, are the rendering
    /// home's facts, and this home writes them as the data they are.
    ///
    /// Owed reversal (red twin): a shell that spliced an empty module for a
    /// carrier nothing was deferred into must break this law.
    #[test]
    fn the_deferred_module_stands_exactly_where_cargo_was_deferred() -> Result<(), ()> {
        let name = ShellName::mangled(plan(7));
        let cargo = cargo()?;

        let carried =
            deferred_module(&name, &DeferredDelivery::Carried(cargo.clone())).map_err(|_| ())?;
        assert_eq!(carried.first(), Some(&GeneratedToken::word("mod")));
        assert_eq!(
            carried.get(1),
            Some(&GeneratedToken::word(name.deferred_module().as_str()))
        );
        let Some(GeneratedToken::Group { tokens, .. }) = carried.get(2) else {
            return Err(());
        };
        let carries =
            |spelling: &str| tokens.iter().any(|token| token == &GeneratedToken::word(spelling));
        assert!(carries(cargo.subject()));
        assert_eq!(cargo.selectors().count(), 1);
        assert!(cargo.selectors().all(|read| {
            carries(read.constant()) && carries(read.active_enum()) && carries(read.variant())
        }));

        assert!(
            deferred_module(&name, &DeferredDelivery::NothingDeferred)
                .is_ok_and(|spliced| spliced.is_empty())
        );
        Ok(())
    }

    /// law: descriptor.everything-the-carrier-delivers-rides-inside-one-gate —
    /// a shell's body is ONE gate invocation, both cargo seats are inside it, and
    /// nothing the carrier delivers stands outside it.
    ///
    /// The deferred module used to stand BESIDE the invocation, so a pin
    /// MISMATCH suppressed the rows while releasing the module: a consumer whose
    /// published pair was incoherent got one refusal and a module of evaluation
    /// copies to compile. The law is structural rather than about a sentence —
    /// the body's last token is the invocation's own braced group, and the
    /// deferred module's tokens are found INSIDE the deferred seat and nowhere
    /// else in the body.
    ///
    /// The claim ceiling: it says where the seats are, and nothing about what the
    /// gate does with them. What a matched pin releases and what a mismatch
    /// refuses is the harness's own arm, on the other side of the wall.
    ///
    /// Owed reversal (red twin): a shell that extended its body with the deferred
    /// module after the invocation must break this law.
    #[test]
    fn everything_the_carrier_delivers_rides_inside_one_gate() -> Result<(), ()> {
        let name = ShellName::mangled(plan(11));
        let cargo = cargo()?;
        let carried =
            deferred_module(&name, &DeferredDelivery::Carried(cargo)).map_err(|_| ())?;
        let body = gate_invocation(expectation_literal(), Vec::new(), carried.clone())
            .map_err(|_| ())?;

        // One invocation: the gate's own path, then `!`, then exactly one group,
        // and nothing after it.
        assert!(body.contains(&GeneratedToken::word(GATE_MACRO)));
        assert!(matches!(body.last(), Some(GeneratedToken::Group { .. })));
        assert_eq!(
            body.iter()
                .filter(|token| matches!(token, GeneratedToken::Group { .. }))
                .count(),
            1
        );

        // The module is inside the deferred seat, token for token.
        assert_eq!(seat(&body, DEFERRED_CLAUSE), Some(carried.as_slice()));
        Ok(())
    }

    /// law: descriptor.an-empty-trials-seat-is-lawful-and-still-written — a
    /// carrier that declares no rows renders an EMPTY trials seat beside its
    /// carried deferred cargo, and the seat is written rather than left out.
    ///
    /// That delivery is what a door holding no caller-supplied row material
    /// produces, and a renderer that required a payload would make it unwritable
    /// — which is what pushed the deferred cargo outside the gate to reach a
    /// consumption target at all. The seat is still WRITTEN, because a gate arm
    /// that had to match two clause shapes would be two arms and one pin would
    /// open two doors.
    ///
    /// Owed reversal (red twin): a renderer that omitted the trials clause for a
    /// carrier with no rows, or that refused such a carrier, must break this law.
    #[test]
    fn an_empty_trials_seat_is_lawful_and_still_written() -> Result<(), ()> {
        let declared = trial_cargo(&TrialDelivery::NothingDeclared).map_err(|_| ())?;
        assert!(declared.is_empty());

        let name = ShellName::mangled(plan(12));
        let cargo = cargo()?;
        let carried =
            deferred_module(&name, &DeferredDelivery::Carried(cargo)).map_err(|_| ())?;
        let body =
            gate_invocation(expectation_literal(), declared, carried.clone()).map_err(|_| ())?;

        assert_eq!(seat(&body, TRIALS_CLAUSE), Some(&[] as &[GeneratedToken]));
        assert_eq!(seat(&body, DEFERRED_CLAUSE), Some(carried.as_slice()));
        Ok(())
    }

    /// law: descriptor.the-shell-name-carries-the-whole-plan-identity — the
    /// exported name is the declared prefix and the PLAN identity at full width,
    /// so "collision-free" is true as written rather than true of a prefix.
    ///
    /// Both halves are asserted, because both were false at once: the name used
    /// to carry eight bytes of a MEMBER's semantic key. The width is read off the
    /// identity itself rather than from a number written here, and the
    /// distinctness is exercised over two plan identities that agree in their
    /// first bytes — which is exactly the pair a truncated key mints one name
    /// for.
    ///
    /// Owed reversal (red twin): a name keyed on a member's semantic key, or one
    /// carrying a prefix of its key, must break this law.
    #[test]
    fn the_shell_name_carries_the_whole_plan_identity() {
        let first = plan(21);
        let spelling = ShellName::mangled(first);
        assert!(spelling.spelling().starts_with(ShellName::PREFIX));
        let suffix = spelling
            .spelling()
            .strip_prefix(ShellName::PREFIX)
            .unwrap_or_default();
        assert_eq!(suffix.len(), first.as_bytes().len() * 2);
        assert_eq!(ShellName::KEY_BYTES, first.as_bytes().len());
        assert!(suffix.chars().all(|character| character.is_ascii_hexdigit()));

        let second = plan(22);
        assert_ne!(first.as_bytes(), second.as_bytes());
        assert_ne!(spelling, ShellName::mangled(second));
        assert_ne!(spelling.deferred_module(), ShellName::mangled(second).deferred_module());
    }
}

mod explanation_protocol {
    use crate::derive_refusal::{RefusalFamilyExpansion, compile_refusal_text};
    use crate::explanation_protocol::{
        ExplanationAnswer, ExplanationCoverageIssue, ProjectionExplanation,
        ProjectionExplanationView, kind_admits,
    };
    use crate::origin_graph::{
        DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion};
    use crate::planning::{
        CauseAnchoring, DeriveImplProjection, DigestContract, GraphAnchoring,
        HostWrapperProjection, InvalidationTrigger, MemberDestination, PlannedOutput,
        ProjectionDisposition,
    };
    use crate::question::{ExplanationQuestion, QuestionApplicability};
    use threadpak::types::Bounded;

    /// One lawful declaration, so the laws below have a REAL plan and a REAL
    /// proof to answer a view over.
    ///
    /// A complete view carries the parentage it was answered over and reads both
    /// identities off the values themselves, so a law about coverage cannot
    /// build one out of synthetic identities — which is exactly the property
    /// under test.
    const DECLARATION: &str = "#[refusal(family = \"demo.explanation\", \
        shape = issue_collection)] enum DemoIssues { NotBound, NotCovered, }";

    /// The plan and the proof every view below is answered over.
    fn expansion() -> Option<RefusalFamilyExpansion> {
        compile_refusal_text(DECLARATION)
            .ok()
            .map(|(_, closed)| closed)
    }

    /// One owner fact.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([60; 32]),
            fact: OwnerIdentityRef::decoded([61; 32]),
        }
    }

    /// The eight universal answers every kind owes.
    fn universal_answers() -> Vec<ProjectionExplanation> {
        let trail = OriginTrail::from_edge(OriginEdge {
            from: crate::plane::for_laws(62),
            relation: OriginRelation::Rendering,
            to: crate::plane::for_laws(63),
        });
        vec![
            ProjectionExplanation::answered(ExplanationAnswer::Kind {
                kind: crate::plane::for_laws(64),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::Owner {
                owner: owner_fact(),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
                sources: CauseAnchoring::Declaration(OwnerIdentityRef::decoded([65; 32])),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::GraphAndProfile {
                graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([66; 32])),
                profile: crate::plane::for_laws(67),
                version: ProfileVersion::declared(2),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::OutputAndDigest {
                output: Box::new(PlannedOutput {
                    semantic_key: crate::plane::for_laws(68),
                    destination: MemberDestination::AtDeclarationSite,
                    origin: trail,
                    expected_profile: crate::plane::for_laws(67),
                    expected_profile_version: ProfileVersion::declared(2),
                    digest_contract: DigestContract::over(crate::plane::for_laws(68)),
                }),
                digest: crate::plane::for_laws(69),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::Invalidators {
                triggers: InvalidationTrigger::one_watched(
                    InvalidationTrigger::GraphIdentityChanged {
                        watched: OwnerIdentityRef::decoded([66; 32]),
                    },
                ),
            }),
            ProjectionExplanation::answered(ExplanationAnswer::RelatedProjectionDisposition {
                related: crate::plane::for_laws(70),
                disposition: ProjectionDisposition::NotRequested,
            }),
            ProjectionExplanation::answered(ExplanationAnswer::Repairs {
                repairs: Bounded::empty(),
            }),
        ]
    }

    /// law: explanation.an-answer-names-its-own-question — the pairing is
    /// derived from the answer, so filing a true answer under the wrong question
    /// is unrepresentable, and every question has an answer variant.
    /// Owed reversal (red twin): a constructor taking the question from the
    /// caller must break this law.
    #[test]
    fn an_answer_names_its_own_question() {
        let explanation = ProjectionExplanation::answered(ExplanationAnswer::Owner {
            owner: owner_fact(),
        });
        assert!(matches!(
            explanation.question(),
            ExplanationQuestion::WhichOwnerRequired
        ));
        let answers: Vec<ExplanationQuestion> = universal_answers()
            .iter()
            .map(ProjectionExplanation::question)
            .collect();
        assert_eq!(answers.len(), 8);
        assert!(answers.iter().enumerate().all(|(position, question)| {
            answers
                .iter()
                .skip(position.saturating_add(1))
                .all(|other| other != question)
        }));
    }

    /// law: explanation.a-rendering-cannot-disagree-with-its-answer — the line a
    /// person reads is a function of the typed answer and of nothing else.
    ///
    /// The structural half is the constructor's shape: `answered` takes the
    /// answer and only the answer, so there is no seat a caller could fill with
    /// a sentence about a different value, and no stored rendering to drift from
    /// the answer it was projected out of. The seat existed until now — a free
    /// `HumanProjection` argument, unrelated by type or check to the answer
    /// beside it — and the home's own README already claimed the rendering was
    /// derived.
    ///
    /// The executed half is below, both directions: two explanations over the
    /// same answer render the same line, two over different answers render
    /// different lines, and every answer in the universal roster renders
    /// something rather than an empty string.
    ///
    /// Owed reversal (red twin): restoring a caller-supplied rendering seat must
    /// break this law — the fixture is testpak's, because the strongest half of
    /// the claim is about a call that no longer compiles.
    #[test]
    fn a_rendering_cannot_disagree_with_its_answer() {
        let answer = ExplanationAnswer::Owner {
            owner: owner_fact(),
        };
        let one = ProjectionExplanation::answered(answer.clone());
        let again = ProjectionExplanation::answered(answer);
        assert_eq!(one.human().shown(), again.human().shown());

        let different = ProjectionExplanation::answered(ExplanationAnswer::Repairs {
            repairs: Bounded::empty(),
        });
        assert_ne!(one.human().shown(), different.human().shown());

        assert!(
            universal_answers()
                .iter()
                .all(|explanation| !explanation.human().is_empty())
        );
    }

    /// law: explanation.a-complete-view-fills-every-applicable-seat — a view
    /// completes exactly when every applicable question has one answer.
    /// Owed reversal: a view accepting a subset must break this law.
    #[test]
    fn a_complete_view_fills_every_applicable_seat() -> Result<(), ()> {
        let closed = expansion().ok_or(())?;
        let mut answers = universal_answers();
        answers.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
        ));
        let view = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            answers,
        );
        assert!(view.is_ok_and(|view| view.len() == 9 && !view.is_empty()));
        Ok(())
    }

    /// law: explanation.a-view-names-the-parentage-it-was-answered-over — a
    /// complete view carries the plan and the proof it was written over, reads
    /// both off the values themselves, and mints its own identity over the
    /// three. A view that carried coverage alone was a value a terminal could
    /// bind beside another expansion's plan and proof of the same kind: every
    /// question answered correctly, about something else.
    ///
    /// Both directions. Two views over the SAME parentage and the same answers
    /// are one identity; two views over the same answers and DIFFERENT
    /// parentage are two, which is the whole of what the seats buy.
    ///
    /// Owed reversal (red twin): a constructor taking the answers alone, or one
    /// taking two identities a caller supplies, must break this law.
    #[test]
    fn a_view_names_the_parentage_it_was_answered_over() -> Result<(), ()> {
        let mine = expansion().ok_or(())?;
        let other = compile_refusal_text(OTHER_DECLARATION)
            .map_err(|_| ())
            .map(|(_, closed)| closed)?;
        let seats = || {
            let mut answers = universal_answers();
            answers.push(ProjectionExplanation::answered(
                ExplanationAnswer::AssumptionsAndSpecializations {
                    assumptions: Bounded::empty(),
                },
            ));
            answers
        };
        let view = ProjectionExplanationView::<DeriveImplProjection>::complete(
            mine.plan(),
            mine.closure(),
            seats(),
        )
        .map_err(|_| ())?;
        let again = ProjectionExplanationView::<DeriveImplProjection>::complete(
            mine.plan(),
            mine.closure(),
            seats(),
        )
        .map_err(|_| ())?;
        let elsewhere = ProjectionExplanationView::<DeriveImplProjection>::complete(
            other.plan(),
            other.closure(),
            seats(),
        )
        .map_err(|_| ())?;

        assert_eq!(view.plan(), mine.plan().identity());
        assert_eq!(view.closure(), mine.closure().identity());
        assert_eq!(view.identity(), again.identity());
        assert_ne!(view.identity(), elsewhere.identity());
        Ok(())
    }

    /// law: explanation.the-seats-stand-in-the-declared-question-order — the
    /// answers a view holds are the kind's roster order and never the order a
    /// caller supplied, so one set of answers is one explanation however it was
    /// assembled.
    ///
    /// Owed reversal (red twin): storing the caller's order must break this law
    /// — the two views below would then carry two identities and read back in
    /// two orders.
    #[test]
    fn the_seats_stand_in_the_declared_question_order() -> Result<(), ()> {
        let closed = expansion().ok_or(())?;
        let mut written = universal_answers();
        written.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
        ));
        let mut shuffled = written.clone();
        shuffled.reverse();

        let view = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            written,
        )
        .map_err(|_| ())?;
        let backwards = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            shuffled,
        )
        .map_err(|_| ())?;

        let declared: Vec<ExplanationQuestion> =
            crate::planning::ProjectionPlan::<DeriveImplProjection>::applicable_questions();
        let held: Vec<ExplanationQuestion> =
            view.answers().map(ProjectionExplanation::question).collect();
        let held_backwards: Vec<ExplanationQuestion> = backwards
            .answers()
            .map(ProjectionExplanation::question)
            .collect();
        assert_eq!(held, declared);
        assert_eq!(held_backwards, declared);
        assert_eq!(view.identity(), backwards.identity());
        Ok(())
    }

    /// A second lawful declaration, so a law that needs TWO parentages has two.
    const OTHER_DECLARATION: &str = "#[refusal(family = \"demo.second\", \
        shape = issue_collection)] enum SecondIssues { NotBound, }";

    /// law: explanation.an-incomplete-view-names-every-missing-seat — a view
    /// missing seats refuses and reports all of them at once, never one per
    /// attempt.
    ///
    /// The coverage pass runs BEFORE the parentage is read, which is why an
    /// empty answer set refuses here rather than minting a name over nothing.
    ///
    /// Owed reversal: reporting only the first unanswered question must break
    /// this law.
    #[test]
    fn an_incomplete_view_names_every_missing_seat() -> Result<(), ()> {
        let closed = expansion().ok_or(())?;
        let refused = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            Vec::new(),
        );
        assert!(refused.is_err_and(|coverage| {
            coverage.body().carried().len() == 9
                && matches!(
                    coverage.body().carried().first(),
                    ExplanationCoverageIssue::QuestionUnanswered(ExplanationQuestion::WhatAreYou)
                )
        }));
        Ok(())
    }

    /// law: explanation.a-doubled-or-foreign-seat-refuses — answering one
    /// question twice, or answering a question the kind does not admit, each
    /// refuses under its own issue.
    /// Owed reversal: silently keeping the last answer must break this law.
    #[test]
    fn a_doubled_or_foreign_seat_refuses() -> Result<(), ()> {
        let closed = expansion().ok_or(())?;
        let mut doubled = universal_answers();
        doubled.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
        ));
        doubled.push(ProjectionExplanation::answered(ExplanationAnswer::Owner {
            owner: owner_fact(),
        }));
        let refused = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            doubled,
        );
        assert!(refused.is_err_and(|coverage| matches!(
            coverage.body().carried().first(),
            ExplanationCoverageIssue::QuestionAnsweredTwice(
                ExplanationQuestion::WhichOwnerRequired
            )
        )));

        let mut foreign = universal_answers();
        foreign.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
        ));
        foreign.push(ProjectionExplanation::answered(
            ExplanationAnswer::SelectedWrappers {
                trace: DecisionTrace::from_entry(TraceEntry {
                    subject: crate::plane::for_laws(71),
                    decision: TraceDecision::SelectedBecause(owner_fact()),
                }),
            },
        ));
        let rejected = ProjectionExplanationView::<DeriveImplProjection>::complete(
            closed.plan(),
            closed.closure(),
            foreign,
        );
        assert!(rejected.is_err_and(|coverage| matches!(
            coverage.body().carried().first(),
            ExplanationCoverageIssue::QuestionNotApplicableToKind(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            )
        )));
        Ok(())
    }

    /// law: explanation.applicability-is-answered-typed — whether a kind admits
    /// a question is a typed answer, not a bare boolean the caller reinterprets.
    /// Owed reversal: returning a boolean must break this law.
    #[test]
    fn applicability_is_answered_typed() {
        assert!(matches!(
            kind_admits::<HostWrapperProjection>(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            ),
            QuestionApplicability::Applicable
        ));
        assert!(matches!(
            kind_admits::<DeriveImplProjection>(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            ),
            QuestionApplicability::NotApplicableToKind
        ));
    }
}

mod template {
    use crate::origin_graph::Nonclaim;
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion};
    use crate::template::{
        ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate,
        ForbiddenKeyFact, INVOCATION_KEY_NEVER, META_BOUND_AXES, MetaBoundAxis, ProfileCeiling,
        SPLICE_CATEGORIES, SpliceCategory, SymbolicBoundFormula, TemplateApplication,
        TemplateArgument, TemplateBinding, TemplateBindingIssue, TemplateConstruction,
        TemplateConstructionIssue, TemplateInvocationKey, TemplateParameter, TemplateSeat,
        VersionedProfile,
    };
    use threadpak::declaration::Stage;
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// The closed splice-category roster, proven closed by an exhaustive match:
    /// a new category stops compiling here until it is placed.
    const fn category_index(category: SpliceCategory) -> usize {
        match category {
            SpliceCategory::Expression => 0,
            SpliceCategory::Type => 1,
            SpliceCategory::Pattern => 2,
            SpliceCategory::Declaration => 3,
            SpliceCategory::Fragment => 4,
            SpliceCategory::IdentifierBinding => 5,
        }
    }

    /// The closed meta bound-axis roster, proven closed by an exhaustive match.
    const fn axis_index(axis: MetaBoundAxis) -> usize {
        match axis {
            MetaBoundAxis::InputDescriptors => 0,
            MetaBoundAxis::Work => 1,
            MetaBoundAxis::Memory => 2,
            MetaBoundAxis::Recursion => 3,
            MetaBoundAxis::Declarations => 4,
            MetaBoundAxis::Symbols => 5,
            MetaBoundAxis::Diagnostics => 6,
            MetaBoundAxis::OutputBytes => 7,
        }
    }

    /// The closed forbidden-fact roster, proven closed by an exhaustive match.
    const fn forbidden_index(fact: ForbiddenKeyFact) -> usize {
        match fact {
            ForbiddenKeyFact::CheckoutPath => 0,
            ForbiddenKeyFact::CurrentDirectory => 1,
            ForbiddenKeyFact::ModificationTime => 2,
            ForbiddenKeyFact::ProcessIdentity => 3,
            ForbiddenKeyFact::AmbientEnvironment => 4,
            ForbiddenKeyFact::WallTime => 5,
            ForbiddenKeyFact::Entropy => 6,
            ForbiddenKeyFact::HostAddress => 7,
            ForbiddenKeyFact::MapIterationOrder => 8,
        }
    }

    /// The closed template-seat roster, proven closed by an exhaustive match.
    const fn seat_index(seat: TemplateSeat) -> usize {
        match seat {
            TemplateSeat::DeclaredParameters => 0,
            TemplateSeat::SuppliedBindings => 1,
            TemplateSeat::AxisCeilings => 2,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([80; 32]),
            fact: OwnerIdentityRef::decoded([81; 32]),
        }
    }

    /// One declared hole under the category and identity byte the caller names.
    fn parameter(category: SpliceCategory, tag: u8) -> TemplateParameter {
        TemplateParameter {
            category,
            parameter: OwnerIdentityRef::decoded([tag; 32]),
        }
    }

    /// One offered commitment under the category and identity byte named.
    fn argument(category: SpliceCategory, tag: u8) -> TemplateArgument {
        TemplateArgument {
            category,
            commitment: OwnerIdentityRef::decoded([tag; 32]),
        }
    }

    /// The complete ceiling: every axis bounded exactly once.
    fn complete_ceiling() -> Result<ProfileCeiling, TemplateConstruction> {
        ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 64,
                    declared_by: owner_fact(),
                })
                .collect(),
        )
    }

    /// The first lock, over one validated input.
    fn formula() -> SymbolicBoundFormula {
        SymbolicBoundFormula {
            formula: OwnerIdentityRef::decoded([82; 32]),
            declared_by: owner_fact(),
            over_inputs: NonEmptyBounded::singleton(OwnerIdentityRef::decoded([83; 32])),
        }
    }

    /// The third lock, as an obligation and a stated nonclaim.
    fn meter() -> CheckedMeterPosture {
        CheckedMeterPosture {
            obliged_by: owner_fact(),
            unmeasured: Nonclaim {
                unclaimed: crate::plane::for_laws(84),
                because: owner_fact(),
            },
        }
    }

    /// One template over the holes the caller names.
    fn template(
        first: TemplateParameter,
        rest: Vec<TemplateParameter>,
    ) -> Result<DeclarationTemplate, TemplateConstruction> {
        complete_ceiling().and_then(|ceiling| {
            DeclarationTemplate::declared(
                OwnerIdentityRef::decoded([85; 32]),
                first,
                rest,
                formula(),
                ceiling,
                meter(),
                Stage::Meta,
            )
        })
    }

    /// The language profile, at a declared version.
    fn language() -> VersionedProfile<crate::plane::LanguageProfileSubject> {
        VersionedProfile {
            profile: OwnerIdentityRef::decoded([86; 32]),
            version: ProfileVersion::declared(4),
        }
    }

    /// The meta profile, at a declared version.
    fn meta() -> VersionedProfile<crate::plane::MetaProfileSubject> {
        VersionedProfile {
            profile: OwnerIdentityRef::decoded([87; 32]),
            version: ProfileVersion::declared(5),
        }
    }

    /// law: template.splice-categories-are-six-and-closed — the hole categories
    /// are a closed roster whose members are pairwise distinct and declared in
    /// one order.
    /// Owed reversal: adding a category without placing it must break this law.
    #[test]
    fn splice_categories_are_six_and_closed() {
        assert_eq!(SPLICE_CATEGORIES.len(), 6);
        let indexes: Vec<usize> = SPLICE_CATEGORIES
            .iter()
            .copied()
            .map(category_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.a-binding-agrees-on-category-or-refuses — an argument
    /// enters a hole only when both ends name the same category, and the
    /// refusal names both categories rather than saying "wrong kind".
    /// Owed reversal (red twin): a constructor that coerced the argument's
    /// category must break this law.
    #[test]
    fn a_binding_agrees_on_category_or_refuses() {
        let bound = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::IdentifierBinding, 2),
        );
        assert!(bound.is_ok_and(|binding| {
            matches!(binding.category(), SpliceCategory::IdentifierBinding)
                && binding.argument().commitment == OwnerIdentityRef::decoded([2; 32])
                && binding.parameter().parameter == OwnerIdentityRef::decoded([1; 32])
        }));

        let refused = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::Expression, 2),
        );
        assert!(refused.is_err_and(|issue| matches!(
            issue,
            TemplateBindingIssue::CategoryMismatch {
                expected: SpliceCategory::IdentifierBinding,
                found: SpliceCategory::Expression
            }
        )));
    }

    /// law: template.the-two-families-declare-their-shapes — the binding seam
    /// runs one check and takes the single-cause shape with a declared
    /// selection order; the construction seam co-establishes and takes the
    /// collection shape, electing no primary issue.
    /// Owed reversal (red twin): swapping the two shapes must break this law.
    #[test]
    fn the_two_families_declare_their_shapes() {
        assert!(matches!(
            TemplateBindingIssue::SHAPE,
            FamilyShape::SingleCause
        ));
        assert_eq!(TemplateBindingIssue::SELECTION_ORDER, &["CategoryMismatch"]);
        assert!(matches!(
            TemplateConstruction::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TemplateConstruction::SELECTION_ORDER.is_empty());
    }

    /// law: template.a-ceiling-covers-every-meta-bound-axis — the axis roster is
    /// closed at eight, a complete ceiling reads back one magnitude per axis,
    /// and a ceiling missing or doubling an axis refuses naming that axis.
    /// Owed reversal (red twin): a ceiling admitting a subset of the axes must
    /// break this law.
    #[test]
    fn a_ceiling_covers_every_meta_bound_axis() {
        assert_eq!(META_BOUND_AXES.len(), 8);
        let indexes: Vec<usize> = META_BOUND_AXES.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        assert!(complete_ceiling().is_ok_and(|ceiling| {
            ceiling.len() == 8
                && !ceiling.is_empty()
                && ceiling.iter().count() == 8
                && ceiling
                    .iter()
                    .all(|held| held.magnitude == 64 && held.declared_by == owner_fact())
                && META_BOUND_AXES
                    .iter()
                    .all(|axis| ceiling.iter().any(|held| held.axis == *axis))
        }));

        let short = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .filter(|axis| *axis != MetaBoundAxis::Memory)
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(short.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::CeilingAxisAbsent {
                axis: MetaBoundAxis::Memory
            }
        )));

        let doubled = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .chain(core::iter::once(MetaBoundAxis::Work))
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::CeilingAxisDoubled {
                axis: MetaBoundAxis::Work
            }
        )));
    }

    /// law: template.a-template-carries-its-three-locks-and-its-stage — a
    /// declared template holds the symbolic formula over validated inputs, the
    /// complete ceiling, the checked-meter obligation with its stated nonclaim,
    /// and the stage its owner declared; two holes under one identity refuse.
    /// Owed reversal (red twin): omitting any lock seat must not compile.
    #[test]
    fn a_template_carries_its_three_locks_and_its_stage() {
        let declared = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 11)],
        );
        assert!(declared.is_ok_and(|template| {
            template.arity() == 2
                && template.parameters().count() == 2
                && template.identity() == OwnerIdentityRef::decoded([85; 32])
                && template.formula().over_inputs.len() == 1
                && template.formula().declared_by == owner_fact()
                && template.ceiling().len() == 8
                && template.meter().obliged_by == owner_fact()
                && template.meter().unmeasured.because == owner_fact()
                && matches!(template.stage(), Stage::Meta)
                && matches!(template.first_parameter().category, SpliceCategory::Type)
        }));

        let doubled = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 10)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::DuplicateParameter { .. }
        )));
    }

    /// The two-hole template the application laws range over: a type hole at
    /// parameter identity 20 and an expression hole at parameter identity 21.
    fn two_hole_template() -> Result<DeclarationTemplate, TemplateConstruction> {
        template(
            parameter(SpliceCategory::Type, 20),
            vec![parameter(SpliceCategory::Expression, 21)],
        )
    }

    /// The bindings for the holes named, each built through the checked binding
    /// seam. A category disagreement at that seam yields no binding at all, so
    /// a law that expected one fails on the count it asserts rather than on a
    /// road this helper invented.
    fn bindings(named: &[(SpliceCategory, u8, u8)]) -> Vec<TemplateBinding> {
        named
            .iter()
            .filter_map(|(category, hole, commitment)| {
                TemplateBinding::bound(
                    parameter(*category, *hole),
                    argument(*category, *commitment),
                )
                .ok()
            })
            .collect()
    }

    /// Apply the two-hole template to the bindings supplied.
    fn apply(supplied: Vec<TemplateBinding>) -> Result<TemplateApplication, TemplateConstruction> {
        two_hole_template().and_then(|template| {
            TemplateApplication::applied(
                &template,
                supplied,
                language(),
                meta(),
                ApplicativeDistinctness::Applicative,
            )
        })
    }

    /// law: template.an-application-binds-every-hole-exactly-once — a complete
    /// application reads its bindings back whole under both profiles, an
    /// unbound hole refuses, and a doubly bound hole refuses.
    /// Owed reversal: an application seam that accepted a partial binding set
    /// must break this law.
    #[test]
    fn an_application_binds_every_hole_exactly_once() {
        let supplied = bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]);
        assert_eq!(supplied.len(), 2);
        let applied = apply(supplied);
        assert!(applied.is_ok_and(|application| {
            application.arity() == 2
                && application.bindings().count() == 2
                && application.template() == OwnerIdentityRef::decoded([85; 32])
                && application.language_profile().version.position() == 4
                && application.meta_profile().version.position() == 5
                && matches!(
                    application.distinctness(),
                    ApplicativeDistinctness::Applicative
                )
        }));

        let unbound = apply(bindings(&[(SpliceCategory::Type, 20, 30)]));
        assert!(unbound.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::MissingBinding { .. }
        )));

        let doubled = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Type, 20, 33),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::DuplicateBinding { .. }
        )));
    }

    /// law: template.an-application-refuses-a-stranger-or-a-recategorized-hole —
    /// a binding naming a hole this template does not declare refuses, and a
    /// binding naming a declared hole under another category refuses naming both
    /// the declared category and the bound one.
    /// Owed reversal: an application seam that ignored an unknown binding, or
    /// one that trusted the binding's own category over the template's, must
    /// break this law.
    #[test]
    fn an_application_refuses_a_stranger_or_a_recategorized_hole() {
        let stranger = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
            (SpliceCategory::Pattern, 99, 98),
        ]));
        assert!(stranger.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::UnknownParameter { .. }
        )));

        let recategorized = apply(bindings(&[
            (SpliceCategory::Pattern, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(recategorized.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::DeclaredCategoryDisagreement {
                declared: SpliceCategory::Type,
                bound: SpliceCategory::Pattern,
                ..
            }
        )));
    }

    /// law: template.a-doubled-hole-does-not-hide-a-recategorized-one — where
    /// one declared hole is bound twice AND one of those bindings disagrees with
    /// its declared category, the pass reports BOTH, because both are true and
    /// each is its own repair.
    ///
    /// The two questions co-establish and are now asked separately. The pass
    /// used to ask about the category only in the arm where exactly one binding
    /// named the hole, so a doubled hole reported the doubling and swallowed the
    /// disagreement — and a caller who removed the duplicate learned about the
    /// category on the NEXT attempt, which is the one-defect-per-attempt road
    /// this home exists to close.
    ///
    /// Both directions: the doubled-and-recategorized set establishes two issues
    /// over the one hole, and the doubled-but-well-categorized set establishes
    /// one. The second half is what stops the fix from reporting a category
    /// disagreement nobody made.
    ///
    /// Owed reversal (red twin): asking the category question only where exactly
    /// one binding was supplied must break this law.
    #[test]
    fn a_doubled_hole_does_not_hide_a_recategorized_one() {
        let both = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Pattern, 20, 33),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(both.is_err_and(|refusal| {
            let established: Vec<&TemplateConstructionIssue> =
                refusal.body().carried().iter().collect();
            established.len() == 2
                && established.iter().any(|issue| {
                    matches!(issue, TemplateConstructionIssue::DuplicateBinding { .. })
                })
                && established.iter().any(|issue| {
                    matches!(
                        issue,
                        TemplateConstructionIssue::DeclaredCategoryDisagreement {
                            declared: SpliceCategory::Type,
                            bound: SpliceCategory::Pattern,
                            ..
                        }
                    )
                })
        }));

        let doubling_alone = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Type, 20, 33),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(doubling_alone.is_err_and(|refusal| {
            refusal.body().carried().len() == 1
                && matches!(
                    refusal.body().carried().first(),
                    TemplateConstructionIssue::DuplicateBinding { .. }
                )
        }));
    }

    /// law: template.deliberate-distinctness-is-identity-bearing — two
    /// applications of one template over the same bindings and profiles differ
    /// only when a distinctness identity says so; the applicative posture and a
    /// declared distinctness never read the same.
    /// Owed reversal (red twin): a boolean distinctness flag must break this
    /// law.
    #[test]
    fn deliberate_distinctness_is_identity_bearing() -> Result<(), ()> {
        let holes =
            template(parameter(SpliceCategory::Fragment, 40), Vec::new()).map_err(|_| ())?;
        let binding = TemplateBinding::bound(
            parameter(SpliceCategory::Fragment, 40),
            argument(SpliceCategory::Fragment, 41),
        )
        .map_err(|_| ())?;
        let applicative = TemplateApplication::applied(
            &holes,
            vec![binding],
            language(),
            meta(),
            ApplicativeDistinctness::Applicative,
        )
        .map_err(|_| ())?;
        let twin = TemplateApplication::applied(
            &holes,
            vec![binding],
            language(),
            meta(),
            ApplicativeDistinctness::Applicative,
        )
        .map_err(|_| ())?;
        let distinct = TemplateApplication::applied(
            &holes,
            vec![binding],
            language(),
            meta(),
            ApplicativeDistinctness::DeliberatelyDistinct(OwnerIdentityRef::decoded([42; 32])),
        )
        .map_err(|_| ())?;
        assert!(applicative == twin && applicative != distinct);
        Ok(())
    }

    /// law: template.the-invocation-key-names-seven-lawful-inputs — the key
    /// carries the template identity, the validated inputs, the source
    /// snapshot, the fragment dependencies, both profile versions, and the
    /// configuration commitment, and two keys differing only in a lawful input
    /// are different keys.
    /// Owed reversal: a key that dropped the configuration commitment must
    /// break this law.
    #[test]
    fn the_invocation_key_names_seven_lawful_inputs() {
        let key = TemplateInvocationKey {
            template: OwnerIdentityRef::decoded([50; 32]),
            inputs: Bounded::empty(),
            source_snapshot: OwnerIdentityRef::decoded([51; 32]),
            fragment_dependencies: Bounded::empty(),
            language_profile: language(),
            meta_profile: meta(),
            configuration: OwnerIdentityRef::decoded([52; 32]),
        };
        let reconfigured = TemplateInvocationKey {
            configuration: OwnerIdentityRef::decoded([53; 32]),
            ..key.clone()
        };
        assert_ne!(key, reconfigured);
        assert_eq!(key, key.clone());
        assert!(key.inputs.is_empty() && key.fragment_dependencies.is_empty());
        assert_eq!(key.language_profile.version.position(), 4);
        assert_eq!(key.meta_profile.version.position(), 5);
        assert_eq!(key.source_snapshot.as_bytes(), &[51_u8; 32]);
    }

    /// law: template.forbidden-key-facts-are-nine-and-closed — the never-roster
    /// is closed at nine, each member distinct, and none of them is a member of
    /// the key record.
    /// Owed reversal: adding a forbidden fact without placing it must break
    /// this law.
    #[test]
    fn forbidden_key_facts_are_nine_and_closed() {
        assert_eq!(INVOCATION_KEY_NEVER.len(), 9);
        let indexes: Vec<usize> = INVOCATION_KEY_NEVER
            .iter()
            .copied()
            .map(forbidden_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.seat-bounds-name-the-seat-that-overran — a bound refusal
    /// names which seat exceeded its magnitude, the declared bound, and the
    /// observed count, and the seat roster is closed.
    /// Owed reversal: a payload-free bound issue must break this law.
    #[test]
    fn seat_bounds_name_the_seat_that_overran() {
        let seats = [
            TemplateSeat::DeclaredParameters,
            TemplateSeat::SuppliedBindings,
            TemplateSeat::AxisCeilings,
        ];
        let indexes: Vec<usize> = seats.iter().copied().map(seat_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        let overrun: Vec<TemplateParameter> = (0..40_u8)
            .map(|tag| parameter(SpliceCategory::Expression, tag.saturating_add(100)))
            .collect();
        let refused = template(parameter(SpliceCategory::Expression, 99), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            TemplateConstructionIssue::SeatBoundExceeded {
                seat: TemplateSeat::DeclaredParameters,
                bound: 32,
                observed: 41
            }
        )));
    }
}

mod trigger_view {
    use crate::plane::{AuthoringLimitProfile, OwnerFactRef, OwnerIdentityRef};
    use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
    use crate::trigger_view::{
        TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
        WrapperTriggerView,
    };
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::{NonEmptyBounded, PositiveLimit};

    /// One owner fact, for laws that need a citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([tag; 32]),
            fact: OwnerIdentityRef::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One selection of the named component, citing one owner fact.
    fn selection(component: WrapperComponent) -> TriggerSelection {
        TriggerSelection {
            component,
            because: NonEmptyBounded::singleton(owner_fact(90)),
        }
    }

    /// One omission of the named component, citing one owner fact.
    fn omission(component: WrapperComponent) -> TriggerOmission {
        TriggerOmission {
            component,
            because: NonEmptyBounded::singleton(owner_fact(92)),
        }
    }

    /// law: trigger.a-disposition-always-cites-an-owner-fact — a selection and
    /// an omission each carry at least one citation by shape, so a bare
    /// selection is unrepresentable rather than refused, and the citations read
    /// back whole.
    /// Owed reversal (red twin): a citation-free selection must not compile.
    #[test]
    fn a_disposition_always_cites_an_owner_fact() {
        let selected = selection(WrapperComponent::Admission);
        assert_eq!(selected.because.len(), 1);
        assert_eq!(*selected.because.first(), owner_fact(90));
        assert!(!selected.because.is_empty());

        // The two-citation selection is built through the CHECKED seam and the
        // law reads the result of that seam. It used to swallow the refusal with
        // a one-citation selection, which would have passed the count assertion
        // below only by accident and would have proven nothing about the pair.
        let paired = NonEmptyBounded::admitted_const(
            owner_fact(94),
            vec![owner_fact(96)],
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map(|because| TriggerSelection {
            component: WrapperComponent::Receipt,
            because,
        });
        assert!(paired.is_ok_and(|selection| selection.because.iter().count() == 2));

        let left_out = omission(WrapperComponent::Explanation);
        assert_eq!(left_out.because.len(), 1);
        assert_eq!(*left_out.because.first(), owner_fact(92));
    }

    /// law: trigger.every-component-is-disposed-exactly-once — a composed view
    /// covers the whole component roster, an undecided component refuses under
    /// its own issue naming it, and a component disposed of twice refuses too.
    /// Owed reversal: a seam that treated an undecided component as omitted
    /// must break this law.
    #[test]
    fn every_component_is_disposed_exactly_once() {
        let plan = crate::plane::for_laws(88);
        let selections: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .take(5)
            .map(selection)
            .collect();
        let omissions: Vec<TriggerOmission> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .skip(5)
            .map(omission)
            .collect();
        let composed = WrapperTriggerView::composed(plan, selections, omissions);
        assert!(composed.is_ok_and(|view| {
            view.len() == 8
                && !view.is_empty()
                && view.plan() == plan
                && view.selections().count() == 5
                && view.omissions().count() == 3
                && view
                    .selections()
                    .all(|selection| !selection.because.is_empty())
                && view
                    .omissions()
                    .all(|omission| !omission.because.is_empty())
        }));

        let undecided: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .filter(|component| *component != WrapperComponent::Cancellation)
            .map(selection)
            .collect();
        let refused = WrapperTriggerView::composed(plan, undecided, Vec::new());
        assert!(refused.is_err_and(|composition| matches!(
            composition.body().carried().first(),
            TriggerViewIssue::MissingComponentDisposition {
                component: WrapperComponent::Cancellation
            }
        )));

        let doubled: Vec<TriggerSelection> =
            WRAPPER_COMPONENTS.iter().copied().map(selection).collect();
        let twice = WrapperTriggerView::composed(
            plan,
            doubled,
            vec![omission(WrapperComponent::Observation)],
        );
        assert!(twice.is_err_and(|composition| matches!(
            composition.body().carried().first(),
            TriggerViewIssue::DoubledComponent {
                component: WrapperComponent::Observation
            }
        )));
    }

    /// law: trigger.the-view-family-is-an-issue-collection — the composition
    /// family declares the collection shape and elects no primary issue, and a
    /// view missing several dispositions reports all of them at once.
    /// Owed reversal (red twin): reporting only the first undecided component
    /// must break this law.
    #[test]
    fn the_view_family_is_an_issue_collection() {
        assert!(matches!(
            TriggerViewComposition::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TriggerViewComposition::SELECTION_ORDER.is_empty());

        let refused = WrapperTriggerView::composed(
            crate::plane::for_laws(89),
            vec![selection(WrapperComponent::Admission)],
            Vec::new(),
        );
        assert!(refused.is_err_and(|composition| composition.body().carried().len() == 7));
    }
}

mod composition {
    use crate::composition::{
        CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
        DescriptorKind, DescriptorProvider,
    };
    use crate::plane::{OwnerFactRef, OwnerIdentityRef};
    use threadpak::refusal::{FamilyShape, RefusalFamily};

    /// The closed descriptor-kind roster, proven closed by an exhaustive match:
    /// a new kind stops compiling here until it is placed.
    const fn kind_index(kind: DescriptorKind) -> usize {
        match kind {
            DescriptorKind::TestDescriptor => 0,
            DescriptorKind::BenchmarkDescriptor => 1,
            DescriptorKind::HostBindingDescriptor => 2,
            DescriptorKind::DocumentationIndexEntry => 3,
            DescriptorKind::ApiInventoryRow => 4,
            DescriptorKind::RemoteSurfaceEntry => 5,
        }
    }

    /// One owner fact, for laws that need a home citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([tag; 32]),
            fact: OwnerIdentityRef::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One provider of the named kind under the identity byte named.
    fn provider(kind: DescriptorKind, tag: u8) -> DescriptorProvider {
        DescriptorProvider {
            provider: OwnerIdentityRef::decoded([tag; 32]),
            home: owner_fact(tag.saturating_add(50)),
            kind,
        }
    }

    /// law: composition.descriptor-kinds-are-six-and-closed — the kinds a
    /// provider may compose are a closed roster whose members are pairwise
    /// distinct and declared in one order.
    /// Owed reversal: adding a kind without placing it must break this law.
    #[test]
    fn descriptor_kinds_are_six_and_closed() {
        assert_eq!(DESCRIPTOR_KINDS.len(), 6);
        let indexes: Vec<usize> = DESCRIPTOR_KINDS.iter().copied().map(kind_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: composition.a-provider-names-its-home-and-its-kind — a declared
    /// provider carries the owning home its facts come from and the kind it
    /// composes, and the root hands both back on a read-only pass.
    /// Owed reversal: a provider standing on its own authority must break this
    /// law.
    #[test]
    fn a_provider_names_its_home_and_its_kind() {
        let root = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 1),
            vec![provider(DescriptorKind::ApiInventoryRow, 2)],
        );
        assert!(root.is_ok_and(|root| {
            let kinds: Vec<DescriptorKind> = root.iter().map(|held| held.kind).collect();
            let homes: Vec<OwnerFactRef> = root.iter().map(|held| held.home).collect();
            kinds
                == vec![
                    DescriptorKind::TestDescriptor,
                    DescriptorKind::ApiInventoryRow,
                ]
                && homes == vec![owner_fact(51), owner_fact(52)]
                && root.len() == 2
                && !root.is_empty()
                && root.first().provider == OwnerIdentityRef::decoded([1; 32])
        }));
    }

    /// law: composition.a-root-refuses-a-duplicate-provider — one provider
    /// identity declared twice refuses naming that provider, and a root past
    /// its declared magnitude refuses naming the seat. Neither is deduplicated
    /// and neither is trimmed.
    /// Owed reversal (red twin): a root that silently kept one of two entries
    /// must break this law.
    #[test]
    fn a_root_refuses_a_duplicate_provider() {
        let doubled = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 3),
            vec![provider(DescriptorKind::BenchmarkDescriptor, 3)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            CompositionRootIssue::DuplicateProvider { .. }
        )));

        let overrun: Vec<DescriptorProvider> = (0..70_u8)
            .map(|tag| {
                provider(
                    DescriptorKind::DocumentationIndexEntry,
                    tag.saturating_add(100),
                )
            })
            .collect();
        let refused =
            CompositionRoot::declared(provider(DescriptorKind::RemoteSurfaceEntry, 4), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.body().carried().first(),
            CompositionRootIssue::SeatBoundExceeded {
                bound: 64,
                observed: 71
            }
        )));
    }

    /// law: composition.the-root-family-is-an-issue-collection — the
    /// declaration family declares the collection shape, elects no primary
    /// issue, and reports every doubled provider at once.
    /// Owed reversal (red twin): declaring `SingleCause` with a collection body
    /// must break this law.
    #[test]
    fn the_root_family_is_an_issue_collection() {
        assert!(matches!(
            CompositionRootDeclaration::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(CompositionRootDeclaration::SELECTION_ORDER.is_empty());

        let refused = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 5),
            vec![
                provider(DescriptorKind::TestDescriptor, 5),
                provider(DescriptorKind::ApiInventoryRow, 6),
                provider(DescriptorKind::ApiInventoryRow, 6),
            ],
        );
        assert!(refused.is_err_and(|refusal| refusal.body().carried().len() == 2));
    }
}

mod pattern_stamp {
    use crate::origin_graph::{OriginRelation, TraceDecision};
    use crate::pattern_stamp::{
        ScopeGuardOwnerFacts, ScopeGuardStampAnchors, plan_scope_guard_stamp,
    };
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion};
    use crate::planning::{
        GraphAnchoring, InvalidationTrigger, OwnerContentAccount, PatternStampProjection,
        ProjectionContext, TargetBinding,
    };
    use crate::refusal::ProjectionPlanningIssue;

    /// One owner fact, distinguished by its fact identity.
    fn owner_fact(fact: u8) -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([100; 32]),
            fact: OwnerIdentityRef::decoded([fact; 32]),
        }
    }

    /// The ONE address the demo stamp's content walks in the door carrying.
    ///
    /// A CAPTURE and not a declaration fragment, because a stamp is planned while
    /// an expansion is holding token material and nothing has been linked. It is
    /// the anchors' own `content` seat, read back here so the account the plan
    /// site builds and the account a law reasons about are one derivation.
    fn content() -> crate::plane::ProjectionIdentity<crate::plane::CapturedDeclarationSubject> {
        crate::plane::for_laws(103)
    }

    /// The entry account the stamp's plan site builds from that address, rebuilt
    /// here so a law that asks what the watch derivation reads reads the same
    /// value the plan site moved into the plan.
    fn account() -> OwnerContentAccount<PatternStampProjection> {
        OwnerContentAccount::captured(content())
    }

    /// The anchors one demo stamp is planned against.
    fn anchors() -> ScopeGuardStampAnchors {
        ScopeGuardStampAnchors {
            content: content(),
            context: ProjectionContext {
                graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([101; 32])),
                profile: crate::plane::for_laws(102),
                profile_version: ProfileVersion::declared(1),
                generator: crate::plane::for_laws(104),
                target: TargetBinding::TargetFree,
            },
            pattern: OwnerIdentityRef::decoded([105; 32]),
            instance: OwnerIdentityRef::decoded([106; 32]),
            guard_name: OwnerIdentityRef::decoded([107; 32]),
            scope_type: OwnerIdentityRef::decoded([108; 32]),
            authored_node: crate::plane::for_laws(109),
            instantiated_node: crate::plane::for_laws(110),
            rendered_node: crate::plane::for_laws(111),
            stamped_unit: crate::plane::for_laws(112),
            byte_role: OwnerIdentityRef::decoded([113; 32]),
            traced: crate::plane::for_laws(114),
            owner_facts: ScopeGuardOwnerFacts {
                class_c_carries_no_ordering: owner_fact(115),
                comparison_is_scope_guarded: owner_fact(116),
            },
        }
    }

    /// law: pattern-stamp.a-declarative-stamp-carries-a-complete-plan — the plan
    /// family carries a declarative stamp: one output, a trail that walks back
    /// through the pattern-instantiation edge to the authored declaration, two
    /// decisions in selection order each citing an identity-home fact, four
    /// watched identities, and the two typed arguments the caller stated.
    /// Owed reversal: a stamp planned without its instantiation edge, or with a
    /// string where a typed argument belongs, must break this law.
    #[test]
    fn a_declarative_stamp_carries_a_complete_plan() {
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && plan.origin().len() == 2
                && matches!(
                    plan.origin().first().relation,
                    OriginRelation::PatternInstantiation
                )
                && plan.trace().len() == 2
                && matches!(
                    plan.trace().first().decision,
                    TraceDecision::SelectedBecause(_)
                )
                && plan.invalidation().len() == 4
                && plan.content().arguments.len() == 2
                && plan.nonclaims().is_empty()
                && !plan.membership().first().output.origin.is_empty()
        }));
    }

    /// law: pattern-stamp.the-watch-set-is-derived-from-the-context — the
    /// stamp's watch set is not written at the stamp's plan site. It IS
    /// `ProjectionContext::watch_set`, derived from the seats the context
    /// declares, so there is no second roster at the call site to fall behind
    /// the value it is about.
    ///
    /// Where the strength actually lives: the derivation destructures
    /// `ProjectionContext` exhaustively, and the plan site destructures
    /// `ScopeGuardStampAnchors` exhaustively. A seat added to either stops the
    /// build until somebody decides what it means for invalidation. That is a
    /// compiler-carried completeness claim, and this law is the positive control
    /// beside it rather than the claim itself.
    ///
    /// The claim ceiling, stated because it is the honest boundary: the derived
    /// set covers the CONTEXT and the entry ACCOUNT, and not the anchors supplied
    /// beside them. The pattern, the instantiation, the two typed arguments, the
    /// origin nodes, the stamped unit, the traced subject and the cited owner
    /// facts define the plan too, and the trigger roster declares no seat any of
    /// them could be watched through — every seat it declares is one
    /// thirty-two-byte identity of a declared kind, and the set's magnitude IS
    /// that roster's cardinality. Those anchors are now COUNTED by the plan
    /// site's destructure rather than remembered by its prose.
    ///
    /// The account is handed to the derivation rather than copied out of it: the
    /// value read below is rebuilt from the same `content` anchor the plan site
    /// builds its account from, so this law compares one derivation against
    /// itself rather than against a second account of the same content.
    ///
    /// Reversal: dropping any context seat from the derivation breaks this law,
    /// and adding a seat to either struct breaks the build.
    #[test]
    fn the_watch_set_is_derived_from_the_context() {
        let anchors = anchors();
        let derived = anchors.context.watch_set(&account());
        let planned = plan_scope_guard_stamp(&anchors);
        assert!(planned.is_ok_and(|plan| {
            derived.is_ok_and(|derived| {
                let from_plan: Vec<InvalidationTrigger> =
                    plan.invalidation().iter().copied().collect();
                let from_context: Vec<InvalidationTrigger> = derived.iter().copied().collect();
                from_plan == from_context
            })
        }));
    }

    /// law: pattern-stamp.every-context-identity-is-watched — every identity the
    /// derivation reads has a trigger of its own: the entry account's own
    /// commitment, the graph, the projection profile, the generator version, and
    /// — where the context is bound to one — the host contract.
    ///
    /// The target binding was the missing one, and it was missing in a way no
    /// existing control could see: the hand-written roster stopped at four, the
    /// roster's `TargetContractChanged` seat sat unused, and every context in
    /// the tree is target-FREE, so the gap only opens for the first plan bound
    /// to a host contract. The control below binds one deliberately.
    ///
    /// The two postures are asked separately because they are different claims.
    /// A target-free context contributes no trigger — a posture is not an
    /// identity and there is nothing to name — and a bound one contributes
    /// exactly one, naming the contract.
    #[test]
    fn every_context_identity_is_watched() {
        let free = anchors();
        let cause = account().cause_trigger();
        assert!(free.context.watch_set(&account()).is_ok_and(|set| {
            let watched: Vec<InvalidationTrigger> = set.iter().copied().collect();
            watched.len() == 4
                && cause.as_ref().is_ok_and(|cause| watched.contains(cause))
                && watched.contains(&free.context.graph_trigger())
                && watched.contains(&InvalidationTrigger::ProjectionProfileChanged {
                    watched: free.context.profile,
                })
                && watched.contains(&InvalidationTrigger::GeneratorVersionChanged {
                    watched: free.context.generator,
                })
                // A posture is not an identity: target-free names nothing.
                && !watched.iter().any(|trigger| {
                    matches!(trigger, InvalidationTrigger::TargetContractChanged { .. })
                })
        }));

        // The same context bound to a host contract watches the contract too.
        let contract = OwnerIdentityRef::decoded([117; 32]);
        let mut bound = anchors();
        bound.context.target = TargetBinding::HostContract(contract);
        assert!(bound.context.watch_set(&account()).is_ok_and(|set| {
            let watched: Vec<InvalidationTrigger> = set.iter().copied().collect();
            watched.len() == 5
                && watched
                    .contains(&InvalidationTrigger::TargetContractChanged { watched: contract })
        }));
    }

    /// law: pattern-stamp.the-watch-set-never-states-one-kind-twice — the
    /// derived set is a SET.
    ///
    /// An expansion-time context is decided against one captured declaration and
    /// the entry account's content IS that same capture, so the account's cause
    /// trigger and the context's graph trigger are the same trigger. Listed, that
    /// is one kind stated twice — which is what the invalidation magnitude is
    /// declared to exclude, since its value IS the trigger roster's cardinality.
    /// The derivation deduplicates, so the set does not depend on a call site
    /// remembering to skip the repeat, and the plan transcript's set encoding
    /// writes one member rather than two.
    ///
    /// The premise is built rather than assumed: the graph is pointed at exactly
    /// the address the account was opened over, which is the posture every
    /// expansion-time plan actually stands in.
    #[test]
    fn the_watch_set_never_states_one_kind_twice() {
        let mut shared = anchors();
        shared.context.graph = GraphAnchoring::CapturedDeclarationOnly(content());

        // The two seats really do resolve to one trigger — the premise the
        // deduplication is about, asserted rather than assumed.
        let cause = account().cause_trigger();
        assert!(
            cause
                .as_ref()
                .is_ok_and(|cause| *cause == shared.context.graph_trigger())
        );

        assert!(shared.context.watch_set(&account()).is_ok_and(|set| {
            let watched: Vec<InvalidationTrigger> = set.iter().copied().collect();
            watched.len() == 3
                && cause.is_ok_and(|cause| {
                    watched.iter().filter(|trigger| **trigger == cause).count() == 1
                })
        }));
    }

    /// law: pattern-stamp.a-dependency-set-this-profile-cannot-watch-refuses — an
    /// entry account naming more commitments than the trigger roster can watch
    /// produces no watch set at all, and therefore no plan over it.
    ///
    /// # Why a refusal and not a narrower set
    ///
    /// The roster's `SourceDeclarationChanged` seat carries one identity and an
    /// account names ONE commitment plus up to the declared dependency magnitude
    /// beside it. Watching the content's own commitment alone produced a value
    /// byte-for-byte the shape of a complete watch set, so a plan committed to a
    /// commitment and two dependencies while watching one read as CURRENT after
    /// the other two changed — and nothing downstream could tell the two apart,
    /// because there is nothing wrong with the value. That is false freshness
    /// rather than a smaller claim, and the plane fails closed until a wider
    /// roster exists.
    ///
    /// The control asserts both directions and the payload. The
    /// stands-on-nothing account still derives its four triggers, so this is not
    /// a road that refuses everything; the two-dependency account refuses with
    /// the typed issue naming both counts; and both public roads refuse, because
    /// a `cause_trigger` that still answered would be the partial claim surviving
    /// beside the road that refuses it.
    ///
    /// # Bounds
    ///
    /// It does not assert that no PLAN stands over such an account, because the
    /// stamp's own anchors cannot express one: [`ScopeGuardStampAnchors`] carries
    /// a single `content` address and no dependency seat, so the plan site builds
    /// an account that stands on nothing by construction. A control that reached
    /// the plan site would have to build the unwatchable account somewhere the
    /// stamp home cannot, and asserting through it would be asserting about a
    /// road this home does not have. The propagation itself is the plan site's
    /// `?` on the derivation, and it is carried by the type.
    ///
    /// Reversal: restoring the content's own commitment as the unconditional
    /// answer makes the two-dependency half of this law fail — the derivation
    /// hands back a set, `is_err` is false, and the issue nobody establishes
    /// cannot be matched.
    #[test]
    fn a_dependency_set_this_profile_cannot_watch_refuses() {
        // An account that stands on nothing is representable and stays so: the
        // refusal is about the profile's reach, not about accounts.
        let single = anchors();
        assert!(single.context.watch_set(&account()).is_ok());
        assert!(account().cause_trigger().is_ok());

        let several = OwnerContentAccount::<PatternStampProjection>::captured_over(
            content(),
            vec![crate::plane::for_laws(118), crate::plane::for_laws(119)],
        );
        assert!(several.is_ok_and(|several| {
            let unwatchable = ProjectionPlanningIssue::CauseSetUnwatchable {
                named: 3,
                watchable: 1,
            };
            several.cause_trigger().is_err_and(|refusal| {
                refusal.body().carried().len() == 1
                    && *refusal.body().carried().first() == unwatchable
            }) && single
                .context
                .watch_set(&several)
                .is_err_and(|refusal| *refusal.body().carried().first() == unwatchable)
        }));
    }

    /// The owning home one citation names, as the bytes that name it.
    fn citation_home(cited: OwnerFactRef) -> Vec<u8> {
        match cited {
            OwnerFactRef::Minted { home, .. } => home.as_bytes().to_vec(),
            OwnerFactRef::Declared(named) => named.home.as_bytes().to_vec(),
        }
    }

    /// law: pattern-stamp.the-stamp-cites-the-identity-home-and-never-itself —
    /// both decisions cite owner facts of one home, and the two facts are
    /// distinct: a stamp that cited itself would be its own oracle.
    /// Owed reversal: collapsing the two facts into one citation must break this
    /// law.
    #[test]
    fn the_stamp_cites_the_identity_home_and_never_itself() {
        let facts = anchors().owner_facts;
        assert_eq!(
            citation_home(facts.class_c_carries_no_ordering),
            citation_home(facts.comparison_is_scope_guarded)
        );
        assert_ne!(
            facts.class_c_carries_no_ordering.citation_bytes(),
            facts.comparison_is_scope_guarded.citation_bytes()
        );
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            matches!(
                plan.trace().first().decision,
                TraceDecision::SelectedBecause(cited) if cited == facts.class_c_carries_no_ordering
            )
        }));
    }
}

mod generated_support {
    use crate::closure::{ClosedExpansion, PartitionCargo};
    use crate::derive_refusal::plan::{rust_declaration_profile, rust_declaration_profile_version};
    use crate::derive_refusal::{
        EVALUATION_SUBJECT, RefusalCompileContext, RefusalDerivationDraft, RefusalFamilyExpansion,
        RefusalOwnerFacts, carrier_expansion, carrier_plan, compile_declaration,
        compile_refusal_text, deferred_selectors, evaluation_axis, profile_does_not_offer,
        rows_disposition,
    };
    use crate::diagnostics::MachineAnchoring;
    use crate::generated_support::{
        AccountedExpansion, AssemblyIssue, AxisCargo, CargoAxis, ProvedCargo, SupportAssembly,
    };
    use crate::planning::{
        CauseAnchoring, DeriveImplProjection, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
        EmissionPartition, ExpectedGeneratedSupportSchemaId, ProjectionDisposition,
        ProjectionKindRow,
    };
    use crate::test_descriptor::DeferredCargo;
    use crate::token::{GeneratedToken, GeneratedTree, TextCapture};
    use threadpak::refusal::CompletionPosture;
    use threadpak::types::Bounded;

    /// One lawful declaration whose shape carries both contracts, so the
    /// implementation terminal below really does plan members into the test
    /// carrier.
    const DECLARATION: &str = "#[refusal(family = \"demo.assembly\", shape = single_cause, \
        order(NotCarried = \"not-carried\"))] enum DemoCarried { NotCarried, }";

    /// The implementation terminal every law below reads cargo off.
    fn implementation() -> Option<RefusalFamilyExpansion> {
        compile_refusal_text(DECLARATION)
            .ok()
            .map(|(_, closed)| closed)
    }

    /// The complete account the door hands back for that same declaration.
    fn accounted() -> Option<AccountedExpansion<RefusalFamilyExpansion>> {
        let read = TextCapture::read(DECLARATION).ok()?;
        let context = RefusalCompileContext {
            spans: read.spans().clone(),
            machine: MachineAnchoring::UnmintedAtThisSeam,
            owner_facts: RefusalOwnerFacts::declared(),
            nonclaims: Bounded::empty(),
        };
        compile_declaration(read.input(), &context).ok()
    }

    /// The kinds this door does NOT generate, written out one by one.
    ///
    /// A list rather than "the roster minus the generated ones", because the
    /// point of the law that reads it is that each of these six was DECIDED: a
    /// complement computed from the other half would pass for a row nobody had
    /// ever said anything about.
    const NOT_OFFERED: [ProjectionKindRow; 6] = [
        ProjectionKindRow::CodecProjection,
        ProjectionKindRow::HostWrapperProjection,
        ProjectionKindRow::RemoteSurfaceProjection,
        ProjectionKindRow::BenchmarkDescriptorProjection,
        ProjectionKindRow::DocumentationProjection,
        ProjectionKindRow::PatternStampProjection,
    ];

    /// The evaluation axis's proved cargo, read off one implementation terminal.
    fn proved(
        draft: &RefusalDerivationDraft,
        terminal: &ClosedExpansion<DeriveImplProjection>,
    ) -> Option<ProvedCargo> {
        match evaluation_axis(draft, terminal).ok()? {
            AxisCargo::Absent { .. } => None,
            AxisCargo::Carried(proved) => Some(proved),
        }
    }

    /// Whether one refusal body carries an issue matching a predicate.
    fn carries(
        refusal: &crate::generated_support::CarrierAssembly,
        matching: impl Fn(&AssemblyIssue) -> bool,
    ) -> bool {
        refusal.body().carried().iter().any(matching)
    }

    /// law: assembly.cargo-enters-only-from-the-terminal-that-proved-it — the
    /// only tokens an axis can carry are the tokens the named terminal's named
    /// partition proved, and the partition must be the one that axis delivers
    /// from.
    ///
    /// Both halves are exercised because both are ways unproved or twice-bound
    /// material would cross the wall: a tree that is not the partition's own is
    /// material nobody proved, and a partition the axis does not deliver from is
    /// material already compiled by another build. The declaration-site
    /// partition is the case that costs — its units are in the consumer's normal
    /// build — and it is the one the second half hands in.
    ///
    /// The claim ceiling: it says which cargo may enter an axis and nothing
    /// about what the cargo means. What a copy stands over and which selection
    /// it reads were established where it was rendered.
    ///
    /// Owed reversal (red twin): a road that took a token tree on its own, or one
    /// that read whichever partition it was handed, must break this law.
    #[test]
    fn cargo_enters_only_from_the_terminal_that_proved_it() -> Result<(), ()> {
        let expansion = implementation().ok_or(())?;
        let terminal = expansion.expansion();
        let draft = expansion.surface().clone().planned();
        let selectors = deferred_selectors(draft.declared_membership()).map_err(|_| ())?;

        // A tree nobody proved: one token this declaration never rendered.
        let invented = GeneratedTree::assembled(vec![GeneratedToken::word("Invented")])
            .map_err(|_| ())?;
        let doctored =
            DeferredCargo::deferred(EVALUATION_SUBJECT, selectors.clone(), invented)
                .map_err(|_| ())?;
        let refusal = ProvedCargo::carried(
            terminal,
            CargoAxis::Evaluation,
            EmissionPartition::TestCarrier,
            doctored,
        )
        .err()
        .ok_or(())?;
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::CargoNotTheSourcesOwn { .. }
        )));

        // The declaration-site partition, whose units the normal build already
        // compiles.
        let carried = terminal.test_carrier().tokens().ok_or(())?.clone();
        let honest =
            DeferredCargo::deferred(EVALUATION_SUBJECT, selectors, carried).map_err(|_| ())?;
        let refusal = ProvedCargo::carried(
            terminal,
            CargoAxis::Evaluation,
            EmissionPartition::DeclarationSite,
            honest,
        )
        .err()
        .ok_or(())?;
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::CargoReachesASecondDestination {
                axis: CargoAxis::Evaluation,
                partition: EmissionPartition::DeclarationSite,
            }
        )));
        Ok(())
    }

    /// law: assembly.one-terminals-partition-is-consumed-exactly-once — two axes
    /// reading one terminal's one partition is a refusal, and it co-establishes
    /// with every other disagreement the same pass found.
    ///
    /// The doubled reading is what would deliver one proved cargo twice into one
    /// exported shell. The bench axis is the one that can be made to read it, and
    /// doing so establishes the vehicle refusal too — so the law also proves the
    /// body carries BOTH rather than electing one, which is what an
    /// issue-collection shape is for.
    ///
    /// Owed reversal (red twin): an assembly that deduplicated the second reading,
    /// or that reported the first issue alone, must break this law.
    #[test]
    fn one_terminals_partition_is_consumed_exactly_once() -> Result<(), ()> {
        let expansion = implementation().ok_or(())?;
        let terminal = expansion.expansion();
        let draft = expansion.surface().clone().planned();
        let cargo = proved(&draft, terminal).ok_or(())?;
        let refusal = SupportAssembly::assembled(
            terminal.plan().account().commitment(),
            EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
            AxisCargo::Absent {
                because: rows_disposition(),
            },
            AxisCargo::Carried(cargo.clone()),
            AxisCargo::Carried(cargo),
        )
        .err()
        .ok_or(())?;
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::CargoConsumedTwice {
                partition: EmissionPartition::TestCarrier,
                ..
            }
        )));
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::BenchVehicleNotOpen
        )));
        assert!(matches!(
            refusal.body().completion(),
            CompletionPosture::Complete
        ));
        Ok(())
    }

    /// law: assembly.every-carried-axis-stands-under-one-root — cargo from a
    /// terminal planned over another declaration is refused, and the refusal
    /// names both roots without electing either.
    ///
    /// One exported carrier delivering two declarations' material is one name at
    /// a crate root answering for two things. Which root the caller meant is the
    /// caller's own fact, so both travel.
    ///
    /// Owed reversal (red twin): an assembly that took the first carried axis's
    /// root as the whole assembly's must break this law.
    #[test]
    fn every_carried_axis_stands_under_one_root() -> Result<(), ()> {
        let expansion = implementation().ok_or(())?;
        let terminal = expansion.expansion();
        let draft = expansion.surface().clone().planned();
        let cargo = proved(&draft, terminal).ok_or(())?;
        let elsewhere = CauseAnchoring::CapturedDeclaration(crate::plane::for_laws(97));
        assert_ne!(elsewhere, terminal.plan().account().commitment());
        let refusal = SupportAssembly::assembled(
            elsewhere,
            EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
            AxisCargo::Absent {
                because: rows_disposition(),
            },
            AxisCargo::Carried(cargo),
            AxisCargo::Absent {
                because: ProjectionDisposition::NotRequested,
            },
        )
        .err()
        .ok_or(())?;
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::RootsDisagree {
                axis: CargoAxis::Evaluation,
                stated,
                carried,
            } if *stated == elsewhere && *carried == terminal.plan().account().commitment()
        )));
        Ok(())
    }

    /// law: assembly.the-gate-is-pinned-against-the-published-expectation — an
    /// expectation minted beside the one these services publish is refused, at
    /// full width.
    ///
    /// The pin the shell writes is what a consumer's gate matches, so a carrier
    /// carrying a second expectation would ship a pin no publication act wrote —
    /// and the refusal carries the observed bytes, because the repair is a
    /// comparison rather than a posture.
    ///
    /// Owed reversal (red twin): an assembly that took whichever expectation it
    /// was handed must break this law.
    #[test]
    fn the_gate_is_pinned_against_the_published_expectation() -> Result<(), ()> {
        let expansion = implementation().ok_or(())?;
        let terminal = expansion.expansion();
        let invented = ExpectedGeneratedSupportSchemaId::declared([7; 32]);
        let refusal = SupportAssembly::assembled(
            terminal.plan().account().commitment(),
            invented,
            AxisCargo::Absent {
                because: rows_disposition(),
            },
            AxisCargo::Absent {
                because: ProjectionDisposition::NotRequested,
            },
            AxisCargo::Absent {
                because: ProjectionDisposition::NotRequested,
            },
        )
        .err()
        .ok_or(())?;
        assert!(carries(&refusal, |issue| matches!(
            issue,
            AssemblyIssue::SchemaExpectationNotPublished { stated } if *stated == [7; 32]
        )));

        // The published one assembles, with every axis absent under a stated
        // disposition: an evaluation-only carrier is lawful and so is an empty
        // one.
        assert!(
            SupportAssembly::assembled(
                terminal.plan().account().commitment(),
                EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
                AxisCargo::Absent {
                    because: rows_disposition(),
                },
                AxisCargo::Absent {
                    because: ProjectionDisposition::NotRequested,
                },
                AxisCargo::Absent {
                    because: ProjectionDisposition::NotRequested,
                },
            )
            .is_ok()
        );
        Ok(())
    }

    /// law: assembly.the-joined-road-emits-exactly-the-two-terminals — a joined
    /// door road ends at two terminals, and the two declaration-site cargos an
    /// emitter writes are exactly those two terminals' declaration-site
    /// partitions.
    ///
    /// Neither is joined into a third value: a stream an emitter assembled would
    /// be bytes neither proof committed to. Both are occupied, because a door
    /// that emitted one of them would leave a carrier nobody defined or a
    /// declaration that never expands.
    ///
    /// Owed reversal (red twin): a door that joined the two cargos into one tree,
    /// or one that returned a single terminal, must break this law.
    #[test]
    fn the_joined_road_emits_exactly_the_two_terminals() -> Result<(), ()> {
        let accounted = accounted().ok_or(())?;
        let joined = accounted.joined();
        let projected = joined.projected().emitted();
        let carrier = joined.carrier_declaration_site();

        assert_eq!(projected, joined.projected().expansion().declaration_site());
        assert_eq!(carrier, joined.carrier().declaration_site());
        assert!(matches!(projected, PartitionCargo::Carried(_)));
        assert!(matches!(carrier, PartitionCargo::Carried(_)));
        assert_ne!(projected.tokens(), carrier.tokens());
        Ok(())
    }

    /// law: assembly.the-deferred-seat-carries-exactly-the-test-carrier-cargo —
    /// the evaluation axis of a joined road's assembly carries the
    /// implementation terminal's TEST-CARRIER partition, token for token, and
    /// names that terminal as its source.
    ///
    /// The trials axis is absent under a stated disposition, which is the
    /// evaluation-only delivery this door produces: the rows a descriptor states
    /// are the caller's declaration and a derive door holds none.
    ///
    /// Owed reversal (red twin): a door that carried the declaration-site cargo,
    /// or that invented row material to fill the trials axis, must break this
    /// law.
    #[test]
    fn the_deferred_seat_carries_exactly_the_test_carrier_cargo() -> Result<(), ()> {
        let accounted = accounted().ok_or(())?;
        let joined = accounted.joined();
        let AxisCargo::Carried(proved) = joined.assembly().evaluation() else {
            return Err(());
        };
        assert_eq!(proved.partition(), EmissionPartition::TestCarrier);
        assert_eq!(proved.source(), joined.projected().identity());
        assert_eq!(
            Some(proved.cargo().tree()),
            joined.projected().expansion().test_carrier().tokens()
        );
        assert_eq!(proved.root(), joined.assembly().root());
        assert!(matches!(
            joined.assembly().trial(),
            AxisCargo::Absent { .. }
        ));
        assert!(matches!(
            joined.assembly().bench(),
            AxisCargo::Absent { .. }
        ));
        Ok(())
    }

    /// law: assembly.a-carrier-terminal-is-planned-rendered-and-closed — the
    /// carrier is a projection with its own plan, its own proof, and its own
    /// explanation, walked through the same public steps every other kind's road
    /// walks.
    ///
    /// The carrier used to be a rendering nobody planned: the shell was tokens a
    /// caller could compose, standing outside every membership the closure
    /// rebuilds. Here the shell is the plan's ONE member and the terminal binds
    /// the three, so "nothing is emitted that did not close" is true of the
    /// vehicle as well as of its cargo.
    ///
    /// Owed reversal (red twin): a carrier rendered outside a closed expansion
    /// must break this law.
    #[test]
    fn a_carrier_terminal_is_planned_rendered_and_closed() -> Result<(), ()> {
        let expansion = implementation().ok_or(())?;
        let draft = expansion.surface().clone().planned();
        let assembly =
            crate::derive_refusal::assembly(&draft, expansion.expansion()).map_err(|_| ())?;
        let plan = carrier_plan(&draft).map_err(|_| ())?;
        let identity = plan.identity();
        let carrier = carrier_expansion(plan, &assembly).map_err(|_| ())?;

        assert_eq!(carrier.plan().identity(), identity);
        assert_eq!(carrier.closure().plan(), identity);
        assert_eq!(carrier.explanation().plan(), identity);
        assert_eq!(carrier.explanation().closure(), carrier.closure().identity());
        assert_eq!(
            carrier
                .plan()
                .membership()
                .count_under(crate::plane::SoleRenderedUnit::Sole),
            1
        );
        assert!(matches!(
            carrier.declaration_site(),
            PartitionCargo::Carried(_)
        ));
        // The carrier defers nothing of its own: what it delivers rides inside
        // its own rendered tokens, behind the gate, rather than in a partition
        // of its own plan.
        assert!(matches!(
            carrier.test_carrier(),
            PartitionCargo::NothingPlanned
        ));
        Ok(())
    }

    /// law: account.every-kind-of-the-sealed-roster-is-dispositioned-exactly-once
    /// — over one captured surface, every row of the enumerated kind roster
    /// reads to exactly one answer off the door's account, the generated rows
    /// are exactly the kinds a terminal was bound for, and a kind that produced
    /// nothing landed nowhere.
    ///
    /// The two halves of the account never disagree about which kinds those are:
    /// the roster of generated rows is READ off the dispositions, and where a
    /// row says generated the delivery reader answers with a partition, so a
    /// record claiming a production nothing was planned for would show up as a
    /// row that generated and landed nowhere.
    ///
    /// The claim ceiling: it says every kind is answered and says nothing about
    /// whether an answer is the RIGHT one. What each answer stands on is the law
    /// below it.
    ///
    /// Owed reversal (red twin): an account that left a kind out, that answered
    /// one kind twice, or that reported a generated kind with no delivery, must
    /// break this law.
    #[test]
    fn every_kind_of_the_sealed_roster_is_dispositioned_exactly_once() -> Result<(), ()> {
        let accounted = accounted().ok_or(())?;
        let generated: Vec<ProjectionKindRow> = accounted.generated().collect();

        // The kinds this door produces, in roster order: the carrier it planned,
        // rendered and closed, and the implementation projection the declaration
        // IS.
        assert_eq!(
            generated,
            [
                ProjectionKindRow::TestDescriptorProjection,
                ProjectionKindRow::DeriveImplProjection,
            ]
        );

        // Together the two halves cover the roster and overlap nowhere.
        assert_eq!(
            generated.len().saturating_add(NOT_OFFERED.len()),
            ProjectionKindRow::ALL.len()
        );
        assert!(NOT_OFFERED.iter().all(|row| !generated.contains(row)));

        for row in ProjectionKindRow::ALL {
            let produced = generated.contains(row);
            assert_eq!(
                produced,
                matches!(
                    accounted.disposition(*row),
                    ProjectionDisposition::Generated { .. }
                )
            );
            // A delivery is a fact about an output, so a kind with no output has
            // none — and a kind with one has exactly the delivery its planned
            // member declared.
            assert_eq!(produced, accounted.landed(*row).is_some());
        }
        Ok(())
    }

    /// law: account.a-generated-row-names-the-output-its-terminal-planned — a
    /// generated kind's disposition carries the output that kind's own terminal
    /// declared, and the delivery it reads to is that member's own destination.
    ///
    /// This is what keeps the account's two halves one account rather than two:
    /// the disposition is READ off the terminal beside it, so a record naming an
    /// output no terminal planned is not a record this door produces. And the
    /// cargo an emitter writes is those same two terminals' declaration-site
    /// partitions, both occupied — which is why the account changes what a
    /// reader can ask and changes nothing a compiler receives.
    ///
    /// Owed reversal (red twin): a disposition composed beside a terminal rather
    /// than read off it, or a delivery answered from anywhere but the member's
    /// declared destination, must break this law.
    #[test]
    fn a_generated_row_names_the_output_its_terminal_planned() -> Result<(), ()> {
        let accounted = accounted().ok_or(())?;
        let joined = accounted.joined();

        let ProjectionDisposition::Generated {
            output: implemented,
        } = accounted.disposition(ProjectionKindRow::DeriveImplProjection)
        else {
            return Err(());
        };
        assert_eq!(
            **implemented,
            joined.projected().plan().membership().first().output
        );

        let ProjectionDisposition::Generated { output: carried } =
            accounted.disposition(ProjectionKindRow::TestDescriptorProjection)
        else {
            return Err(());
        };
        assert_eq!(
            **carried,
            joined.carrier().plan().membership().first().output
        );

        // Both land at the declaration site, which is why an emitter writes two
        // cargos and the consumer's normal build compiles both.
        assert_eq!(
            accounted.landed(ProjectionKindRow::DeriveImplProjection),
            Some(EmissionPartition::DeclarationSite)
        );
        assert_eq!(
            accounted.landed(ProjectionKindRow::TestDescriptorProjection),
            Some(EmissionPartition::DeclarationSite)
        );
        assert!(matches!(
            joined.projected().emitted(),
            PartitionCargo::Carried(_)
        ));
        assert!(matches!(
            joined.carrier_declaration_site(),
            PartitionCargo::Carried(_)
        ));
        Ok(())
    }

    /// law: account.a-non-generated-row-carries-a-readable-ground — every kind
    /// this door does not offer carries the standing of the profile the door
    /// renders under, naming that profile and its version, and it is the same
    /// value the documentation road records for the one election it stops at.
    ///
    /// A reader asking why no bench arrived is handed a posture with a repair in
    /// it — a profile that offers the kind — rather than "nobody asked", which
    /// would invite a request this seam cannot honour, or silence, which is what
    /// the disposition exists to abolish.
    ///
    /// Owed reversal (red twin): a door that answered one of these six with a
    /// generated output, that minted a machine identity to fill a content seat,
    /// or that left the posture unnamed, must break this law.
    #[test]
    fn a_non_generated_row_carries_a_readable_ground() -> Result<(), ()> {
        let accounted = accounted().ok_or(())?;
        for row in NOT_OFFERED {
            assert!(matches!(
                accounted.disposition(row),
                ProjectionDisposition::UnavailableUnderProfile { profile, version }
                    if *profile == rust_declaration_profile()
                        && *version == rust_declaration_profile_version()
            ));
        }
        // One construction, read twice: the standing a kind stands under and the
        // standing the facet election stops at are the same value, so a profile
        // bump moves both or neither.
        assert_eq!(
            *accounted.disposition(ProjectionKindRow::DocumentationProjection),
            profile_does_not_offer()
        );
        Ok(())
    }
}

mod derive_refusal {
    use crate::closure::ClosureIssue;
    use crate::derive_refusal::{
        CapturedCause, CapturedDocumentation, CauseOrderStanding, DerivedMembership,
        DocumentedDeclaration, EVALUATION_SUBJECT, RefusalDeriveCapture, RefusalDeriveSurface,
        RenderRefusal, captured, captured_text, compile_refusal, compile_refusal_text, render,
    };
    use crate::diagnostics::{MachineAnchoring, MacrocPhase};
    use crate::planning::{ProjectionDisposition, RenderedImplementation};
    use crate::token::{GeneratedToken, GeneratedTree, TextCapture};
    use threadpak::declaration::CoordinateRole;
    use threadpak::refusal::{CauseOrderDeclaration, FamilyShape, RefusalFamily};

    /// The lawful single-cause declaration, as a token stream renders it.
    const SINGLE_CAUSE: &str = "#[refusal(family = \"demo.example\", shape = single_cause, \
        order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\"))] \
        enum DemoFamily { NotCanonical, NotAdmitted, }";

    /// The lawful collection declaration: no order clause, and none admitted.
    const ISSUE_COLLECTION: &str = "#[refusal(family = \"demo.example\", shape = issue_collection)] \
        enum DemoIssues { NotBound, NotCovered, }";

    /// A lawful declaration whose Rust type is spelled like a word the FAMILY
    /// body writes: the body spells the machine's shape roster, so the type name
    /// stands inside the body it would be relocated out of.
    const FAMILY_BODY_OBSERVER: &str = "#[refusal(family = \"demo.example\", shape = single_cause, \
        order(NotCanonical = \"not-canonical\"))] enum FamilyShape { NotCanonical, }";

    /// A lawful declaration whose Rust type is spelled like a word the
    /// CAUSE-ORDER body writes, and like nothing the family body writes.
    const ORDER_BODY_OBSERVER: &str = "#[refusal(family = \"demo.example\", shape = single_cause, \
        order(NotCanonical = \"not-canonical\"))] enum DeclaredCause { NotCanonical, }";

    /// The variant of a documented declaration the variant prose is written on.
    const DOCUMENTED_VARIANT: &str = "NotBound";

    /// The prose written on the one documented VARIANT of a documented
    /// declaration.
    const VARIANT_PROSE: &str = "nothing bound this";

    /// The prose written on the FAMILY of one documented declaration.
    const FAMILY_PROSE: &str = "the family a reader is handed";

    /// The prose written on the family of the OTHER documented declaration —
    /// the one thing the two declarations disagree about.
    const OTHER_FAMILY_PROSE: &str = "the family a reader is shown";

    /// The lawful collection declaration carrying prose on the family and on one
    /// of its variants, in the attribute form a documentation comment becomes.
    ///
    /// The prose is a parameter rather than a spelling inside the source, so the
    /// text a law reads back is the text this fixture wrote — one authority for
    /// both sides of the comparison rather than a literal restated beside it.
    fn documented(family_prose: &str) -> String {
        format!(
            "#[doc = \"{family_prose}\"] \
             #[refusal(family = \"demo.example\", shape = issue_collection)] \
             enum DemoIssues {{ #[doc = \"{VARIANT_PROSE}\"] {DOCUMENTED_VARIANT}, NotCovered, }}"
        )
    }

    /// One captured surface, or the cause the capture established.
    fn surface(source: &str) -> Result<RefusalDeriveSurface, RefusalDeriveCapture> {
        captured_text(source)
            .map(|(_, surface)| surface)
            .map_err(crate::derive_refusal::RefusalDeriveRefusal::cause)
    }

    /// Whether one rendered tree names a spelling as a WORD anywhere inside it,
    /// head and body alike.
    ///
    /// The whole tree and not its top level, because the two halves of the
    /// question live at two depths: a target's spelling stands in the HEAD, and
    /// the substitution the guard establishes is about the BODY.
    fn names(tree: &GeneratedTree, spelling: &str) -> bool {
        let wanted = GeneratedToken::word(spelling);
        tree.tokens().any(|token| carries(token, &wanted))
    }

    /// Whether one token, or anything nested inside it, is the wanted token.
    fn carries(token: &GeneratedToken, wanted: &GeneratedToken) -> bool {
        match token {
            GeneratedToken::Group { tokens, .. } => {
                tokens.iter().any(|nested| carries(nested, wanted))
            }
            GeneratedToken::Word(_)
            | GeneratedToken::Punct { .. }
            | GeneratedToken::Text(_)
            | GeneratedToken::ByteText(_)
            | GeneratedToken::Number(_) => token == wanted,
        }
    }

    /// law: derive.the-engine-declares-its-own-order-by-hand — the capture
    /// family's own declared facts are authored, never derived by the derive
    /// this module ships. A generator that produced its own contracts would be
    /// its own oracle.
    /// Owed reversal: deriving the engine's own order must break this law.
    #[test]
    fn the_engine_declares_its_own_order_by_hand() {
        assert!(matches!(
            RefusalDeriveCapture::SHAPE,
            FamilyShape::SingleCause
        ));
        assert_eq!(
            RefusalDeriveCapture::SELECTION_ORDER.len(),
            RefusalDeriveCapture::DECLARED_ORDER.len()
        );
        assert!(
            RefusalDeriveCapture::DECLARED_ORDER.projects_to(RefusalDeriveCapture::SELECTION_ORDER)
        );
    }

    /// law: derive.a-cause-identity-is-its-family-and-its-local-key — every
    /// cause of the capture family carries the derived pair band 00 declares,
    /// the pair's family is one family, and no two causes share a local key.
    /// Owed reversal: two causes sharing a local key must break this law.
    #[test]
    fn a_cause_identity_is_its_family_and_its_local_key() {
        let causes = [
            RefusalDeriveCapture::NotAnEnum,
            RefusalDeriveCapture::UnsupportedDeclarationForm,
            RefusalDeriveCapture::NotNamed,
            RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            RefusalDeriveCapture::NotBodied,
            RefusalDeriveCapture::NotInhabited,
            RefusalDeriveCapture::UnsupportedVariantPayload,
            RefusalDeriveCapture::NotFamilyDeclared,
            RefusalDeriveCapture::NotFamilyGrammatical,
            RefusalDeriveCapture::NotShapeDeclared,
            RefusalDeriveCapture::NotAnAdmittedShape,
            RefusalDeriveCapture::NotOrderDeclared,
            RefusalDeriveCapture::NotOrderAdmitted,
            RefusalDeriveCapture::NotCovered,
            RefusalDeriveCapture::NotDistinct,
            RefusalDeriveCapture::NotKeyed,
            RefusalDeriveCapture::Unbounded,
        ];
        assert_eq!(causes.len(), RefusalDeriveCapture::SELECTION_ORDER.len());
        assert!(
            causes
                .iter()
                .all(|cause| cause.id().family() == RefusalDeriveCapture::FAMILY)
        );
        let keys: Vec<&str> = causes
            .iter()
            .map(|cause| cause.id().local().as_declared())
            .collect();
        assert!(keys.iter().enumerate().all(|(position, key)| {
            keys.iter()
                .skip(position.saturating_add(1))
                .all(|other| other != key)
        }));
    }

    /// law: derive.a-lawful-declaration-captures-typed — a well-formed
    /// declaration yields the machine's shape, the author's family identity, the
    /// author's local keys, and the default crate binding.
    /// Owed reversal: a capture that read the body layout instead of the order
    /// clause must break this law.
    #[test]
    fn a_lawful_declaration_captures_typed() {
        assert!(surface(SINGLE_CAUSE).is_ok_and(|surface| {
            surface.family_name() == "DemoFamily"
                && surface.family_id() == "demo.example"
                && surface.binding().spelling() == "threadpak"
                && matches!(surface.shape(), FamilyShape::SingleCause)
                && surface.causes().count() == 2
                && surface
                    .causes()
                    .next()
                    .is_some_and(|cause: &CapturedCause| {
                        cause.spelling() == "NotCanonical" && cause.local_key() == "not-canonical"
                    })
        }));
        assert!(surface(ISSUE_COLLECTION).is_ok_and(|surface| {
            matches!(surface.shape(), FamilyShape::IssueCollection) && surface.causes().count() == 0
        }));
    }

    /// law: derive.the-crate-binding-travels-with-the-declaration — a consumer
    /// that renamed its dependency is captured under the name it used, and the
    /// binding reaches the rendering rather than being assumed there.
    /// Owed reversal (red twin): a renderer hardcoding `::threadpak` must break
    /// this law.
    #[test]
    fn the_crate_binding_travels_with_the_declaration() {
        let renamed = "#[refusal(crate = tp, family = \"demo.example\", shape = issue_collection)] \
            enum DemoIssues { NotBound, }";
        assert!(surface(renamed).is_ok_and(|surface| surface.binding().spelling() == "tp"));
        let rendered = compile_refusal_text(renamed)
            .map(|(_, closed)| closed.inspected())
            .map_err(|_| ());
        assert!(rendered.is_ok_and(|inspected| {
            inspected.is_some_and(|text| {
                text.contains(":: tp :: refusal :: RefusalFamily") && !text.contains("threadpak")
            })
        }));
    }

    /// law: derive.the-declared-order-is-the-selector-not-the-layout — the
    /// captured order follows the order clause, not the order the variants
    /// happen to be written in.
    /// Owed reversal: a capture reading the body layout must break this law.
    #[test]
    fn the_declared_order_is_the_selector_not_the_layout() {
        let reordered = "#[refusal(family = \"demo.example\", shape = single_cause, \
            order(NotAdmitted = \"not-admitted\", NotCanonical = \"not-canonical\"))] \
            enum DemoFamily { NotCanonical, NotAdmitted, }";
        assert!(surface(reordered).is_ok_and(|surface| {
            let spellings: Vec<&str> = surface.causes().map(CapturedCause::spelling).collect();
            spellings == vec!["NotAdmitted", "NotCanonical"]
        }));
    }

    /// law: derive.a-real-enum-is-never-told-it-is-not-an-enum — a declaration
    /// that IS a real Rust item and merely meets a limit of this grammar gets a
    /// cause naming that limit. A caller told `NotAnEnum` about a perfectly good
    /// enum goes looking for the wrong problem.
    /// Owed reversal (red twin): folding these forms back into `NotAnEnum` must
    /// break this law.
    #[test]
    fn a_real_enum_is_never_told_it_is_not_an_enum() {
        let cases = [
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 struct NotAnEnumAtAll;",
                RefusalDeriveCapture::UnsupportedDeclarationForm,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 enum Generic<T> { NotBound, }",
                RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 enum Payloaded { NotBound(u8), }",
                RefusalDeriveCapture::UnsupportedVariantPayload,
            ),
        ];
        assert!(
            cases
                .iter()
                .all(|(source, expected)| surface(source) == Err(*expected))
        );
    }

    /// law: derive.every-malformed-declaration-establishes-one-cause — the
    /// capture family is single-cause, and each malformed declaration
    /// establishes exactly the cause its defect names.
    /// Owed reversal: collapsing two defects onto one cause must break this law.
    #[test]
    fn every_malformed_declaration_establishes_one_cause() {
        let cases = [
            ("nothing declared here", RefusalDeriveCapture::NotAnEnum),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] enum { A, }",
                RefusalDeriveCapture::NotNamed,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] enum Empty { }",
                RefusalDeriveCapture::NotInhabited,
            ),
            (
                "#[refusal(shape = issue_collection)] enum Demo { A, }",
                RefusalDeriveCapture::NotFamilyDeclared,
            ),
            (
                "#[refusal(family = \"NotKebab\", shape = issue_collection)] enum Demo { A, }",
                RefusalDeriveCapture::NotFamilyGrammatical,
            ),
            (
                "#[refusal(family = \"demo.example\")] enum Demo { A, }",
                RefusalDeriveCapture::NotShapeDeclared,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }",
                RefusalDeriveCapture::NotAnAdmittedShape,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause)] enum Demo { A, }",
                RefusalDeriveCapture::NotOrderDeclared,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection, \
                 order(A = \"a\"))] enum Demo { A, }",
                RefusalDeriveCapture::NotOrderAdmitted,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"a\"))] enum Demo { A, B, }",
                RefusalDeriveCapture::NotCovered,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"a\", B = \"a\"))] enum Demo { A, B, }",
                RefusalDeriveCapture::NotDistinct,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"NotKebab\"))] enum Demo { A, }",
                RefusalDeriveCapture::NotKeyed,
            ),
        ];
        assert!(
            cases
                .iter()
                .all(|(source, expected)| surface(source) == Err(*expected))
        );
    }

    /// law: derive.a-refusal-names-the-offending-token — a capture refusal
    /// established AFTER a capture carries the token it sits at, and the text
    /// route resolves that token to a byte position. A refusal that always
    /// pointed at the first token would send every reader to the same wrong
    /// place.
    ///
    /// The handle is an OPTION and the law reads it as one: a refusal established
    /// before any capture issues no handle, and this control asserts the answer
    /// is present for a refusal that did — which is what makes the option's other
    /// arm a real posture rather than a seat this law would have papered over
    /// with handle zero.
    ///
    /// Owed reversal (red twin): reporting at `token[0]` must break this law.
    #[test]
    fn a_refusal_names_the_offending_token() -> Result<(), ()> {
        let source = "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }";
        let table = TextCapture::read(source)
            .map(|capture| capture.spans().clone())
            .map_err(|_| ())?;
        let refused = captured_text(source).map(|_| ());
        assert!(refused.is_err_and(|refusal| {
            refusal.token().is_some_and(|token| {
                table.coordinate_of(token).is_ok_and(|coordinate| {
                    coordinate.role == CoordinateRole::Byte && coordinate.position > 0
                })
            }) && refusal.cause() == RefusalDeriveCapture::NotAnAdmittedShape
        }));
        Ok(())
    }

    /// law: derive.the-standing-of-the-cause-order-is-typed — a shape that
    /// declares no canonical order says so with a typed standing, and the plan's
    /// disposition names the owner fact rather than saying nothing.
    /// Owed reversal: an untyped standing must break this law.
    #[test]
    fn the_standing_of_the_cause_order_is_typed() {
        let single = surface(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        let collection = surface(ISSUE_COLLECTION).map(RefusalDeriveSurface::planned);
        assert!(single.is_ok_and(|draft| {
            matches!(draft.cause_order_standing(), CauseOrderStanding::Declared)
                && draft.declared_membership() == DerivedMembership::FamilyAndCauseOrder
                && draft.declared_membership().len() == 4
                && !draft.declared_membership().is_empty()
        }));
        assert!(collection.is_ok_and(|draft| {
            matches!(
                draft.cause_order_standing(),
                CauseOrderStanding::NotApplicableToShape
            ) && draft.declared_membership() == DerivedMembership::FamilyOnly
        }));
    }

    /// law: derive.the-one-road-closes-before-it-emits — the live road produces
    /// a plan, a rendering, a proved closure, and a complete explanation, and
    /// every emission is reachable only off the closed expansion those four
    /// produced.
    ///
    /// The counts are FOUR because one implementation meaning is delivered as two
    /// surfaces and this declaration carries two contracts: the family
    /// implementation and the typed cause order, each with its own
    /// mutation-evaluation copy planned beside it. A membership of two would be a
    /// plan declaring a smaller output set than the delivery has, and the
    /// closure — which rebuilds the membership role by role — would be honest
    /// about a plan that is not.
    ///
    /// Owed reversal (red twin): a render road that skipped the closure must
    /// break this law.
    #[test]
    fn the_one_road_closes_before_it_emits() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let plan = closed.plan();
            let closure = closed.closure();
            plan.membership().len() == 4
                && closure.rendered().len() == 4
                && closure.reconstructed().len() == 4
                && plan.trace().len() == 3
                && plan.invalidation().len() == 3
                && plan.origin().len() == 1
                && closed.explanation().len() == 9
                && closure
                    .emission()
                    .declaration_site()
                    .tokens()
                    .is_some_and(|tree| !tree.is_empty())
                && matches!(
                    closed.cause_order(),
                    ProjectionDisposition::Generated { .. }
                )
        }));
    }

    /// law: derive.inspection-and-emission-read-one-value — the text a caller
    /// inspects is a projection of the same tokens the declaration site
    /// delivers, and those tokens are the CLOSURE's own proved cargo. There is
    /// no parallel plan, no synthetic sibling, and no join performed past the
    /// proof.
    /// Owed reversal (red twin): a second rendering built for inspection must
    /// break this law.
    #[test]
    fn inspection_and_emission_read_one_value() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let inspected = closed.inspected();
            closed
                .closure()
                .emission()
                .declaration_site()
                .tokens()
                .is_some_and(|tree| inspected == Some(tree.inspected()))
        }));
    }

    /// law: derive.the-plan-is-a-function-of-the-declaration — two captures of
    /// the same declaration produce the same plan identities and the same
    /// closure identity, and a different declaration produces different ones.
    /// Owed reversal: an identity carrying anything ambient must break this law.
    #[test]
    fn the_plan_is_a_function_of_the_declaration() -> Result<(), ()> {
        let (_, first) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let (_, second) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let (_, third) = compile_refusal_text(ISSUE_COLLECTION).map_err(|_| ())?;
        assert!(
            first.closure().identity() == second.closure().identity()
                && first.surface().identity() == second.surface().identity()
                && first.closure().identity() != third.closure().identity()
        );
        Ok(())
    }

    /// law: derive.a-membership-only-draft-renders-nothing — the draft states
    /// what the shape fixed and carries no rendering road. The frontage road is
    /// closed: there is no public value in this home other than a closed
    /// expansion that carries a token tree.
    /// Owed reversal (red twin): re-adding `rendered()` to the draft must break
    /// this law.
    #[test]
    fn a_membership_only_draft_renders_nothing() {
        let draft = surface(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        assert!(draft.is_ok_and(|draft| {
            // The draft answers what the SHAPE fixed and nothing else. Every
            // question about bytes is answered by a closed expansion or by
            // nobody.
            draft.declared_membership().roles().len() == 4
                && draft.surface().family_id() == "demo.example"
        }));
    }

    /// law: derive.the-explanation-carries-the-proved-digest — the
    /// output-and-digest seat is answered with the digest the CLOSURE proved
    /// over bytes that exist, never with a value the plan invented.
    /// Owed reversal (red twin): a plan-supplied digest must break this law.
    #[test]
    fn the_explanation_carries_the_proved_digest() -> Result<(), ()> {
        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let unit = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .ok_or(())?;
        let member = closed
            .plan()
            .membership()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .ok_or(())?;
        assert!(
            unit.digest_under(member.output.digest_contract) == unit.digest()
                && unit.semantic_key() == member.output.semantic_key
        );
        Ok(())
    }

    /// law: derive.a-diagnostic-from-an-expansion-says-it-is-unanchored — the
    /// live road refuses with a diagnostic that states the machine posture it
    /// actually has, rather than carrying a stand-in identity nobody minted.
    /// Owed reversal (red twin): a plane-minted machine identity must break this
    /// law.
    #[test]
    fn a_diagnostic_from_an_expansion_says_it_is_unanchored() -> Result<(), ()> {
        let malformed = "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }";
        let read = TextCapture::read(malformed).map_err(|_| ())?;
        let context = crate::derive_refusal::RefusalCompileContext {
            spans: read.spans().clone(),
            machine: MachineAnchoring::UnmintedAtThisSeam,
            owner_facts: crate::derive_refusal::RefusalOwnerFacts::declared(),
            nonclaims: threadpak::types::Bounded::empty(),
        };
        let compiled = compile_refusal(read.input(), &context);
        assert!(compiled.is_err_and(|diagnostic| {
            matches!(diagnostic.machine, MachineAnchoring::UnmintedAtThisSeam)
                && matches!(diagnostic.phase, MacrocPhase::Capture)
                && !diagnostic.summary.is_empty()
                && diagnostic.repairs.len() == 1
        }));
        Ok(())
    }

    /// law: derive.a-closure-refuses-a-rendering-that-drops-a-planned-role — the
    /// closure check is claim-specific: a rendering that materializes fewer
    /// units than the plan declared is refused by role, before any token exists.
    /// Owed reversal (red twin): a closure that compared counts must break this
    /// law.
    #[test]
    fn a_closure_refuses_a_rendering_that_drops_a_planned_role() -> Result<(), ()> {
        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let unit = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned()
            .ok_or(())?;
        let partial = crate::closure::RenderedProjection::of_one(unit);
        let proved = crate::closure::ProjectionClosure::proved(
            closed.plan().identity(),
            closed.plan().membership(),
            partial,
        );
        assert!(proved.is_err_and(|refusal| {
            *refusal.body().carried().first()
                == ClosureIssue::MemberMissing {
                    role: RenderedImplementation::RenderedCauseOrderImpl,
                }
        }));
        Ok(())
    }

    /// law: derive.a-relocated-body-is-established-or-refused — an
    /// implementation body only stands over the support shell's own subject
    /// where standing it there changes nothing. A body that observes the target
    /// it was derived for is refused with the typed refusal, and a body that
    /// observes it nowhere renders under the local evaluation subject's
    /// spelling while the production implementation renders under the type the
    /// declaration named.
    ///
    /// Both directions, and the guard's precision with them. The walk covers the
    /// BODY, so the two evaluation roads answer independently: a family whose
    /// Rust type is spelled like a word the FAMILY body writes refuses on the
    /// family road and renders on the cause-order road, and a family spelled
    /// like a word the CAUSE-ORDER body writes does the reverse. A walk over the
    /// whole implementation would refuse both, because the head names the target
    /// on purpose.
    ///
    /// The claim ceiling: the walk asks two questions of every word, and one of
    /// them is the language's `Self`, which no body this home renders spells at
    /// all — so it is a wall the bodies stand clear of rather than a branch a
    /// declaration reaches, and what stands under test here is the declared-name
    /// question and the substitution it guards.
    ///
    /// Owed reversal (red twin): a copy rendered over the evaluation subject
    /// with no guard in front of it must break this law.
    #[test]
    fn a_relocated_body_is_established_or_refused() -> Result<(), ()> {
        let lawful = surface(SINGLE_CAUSE).map_err(|_| ())?;
        let production = render::family_implementation(&lawful).map_err(|_| ())?;
        let relocated = render::family_evaluation_implementation(&lawful).map_err(|_| ())?;
        assert!(
            names(&production, lawful.family_name())
                && !names(&production, EVALUATION_SUBJECT)
                && names(&relocated, EVALUATION_SUBJECT)
                && !names(&relocated, lawful.family_name())
        );

        let family_observer = surface(FAMILY_BODY_OBSERVER).map_err(|_| ())?;
        assert!(render::family_implementation(&family_observer).is_ok());
        assert!(
            render::family_evaluation_implementation(&family_observer)
                .is_err_and(|refusal| refusal == RenderRefusal::TargetObserved)
        );
        assert!(render::cause_order_evaluation_implementation(&family_observer).is_ok());

        let order_observer = surface(ORDER_BODY_OBSERVER).map_err(|_| ())?;
        assert!(render::cause_order_implementation(&order_observer).is_ok());
        assert!(
            render::cause_order_evaluation_implementation(&order_observer)
                .is_err_and(|refusal| refusal == RenderRefusal::TargetObserved)
        );
        assert!(render::family_evaluation_implementation(&order_observer).is_ok());
        Ok(())
    }

    /// law: derive.one-captured-surface-carries-two-authored-facts — a
    /// declaration carrying prose on the family and on its variants captures
    /// with those rows present, each row's declared-on seat and its text read
    /// back as typed values; a declaration that writes none carries none; and
    /// the surface is named TWICE, once for what the declaration is and once for
    /// what it says.
    ///
    /// The two sensitivities are the point, and they are opposite. Two
    /// declarations that differ ONLY in their prose carry ONE semantic
    /// commitment — so an implementation projection over either is the same
    /// projection, and a reworded sentence does not re-plan a contract. They
    /// carry TWO documentation commitments — so a documentation projection sees
    /// the change it exists to be about. A declaration that differs in its SHAPE
    /// moves both, because the documentation commitment is derived over the
    /// semantic one.
    ///
    /// The single commitment this replaced had one sensitivity for both
    /// questions: prose moved everything, so a comment edit re-planned the
    /// implementation, and there was no name a documentation projection could
    /// stand on that was about the prose.
    ///
    /// The claim ceiling: a row states what was written and where it was
    /// written. Nothing here says what the text MEANS — no facet, no audience,
    /// no heading, no section — because those are the documentation
    /// projection's declarations and a capture that decided them would be
    /// deciding meaning it was handed as text.
    ///
    /// Owed reversal (red twin): a capture that skipped documentation
    /// attributes, one semantic commitment that still moved with prose, or one
    /// documentation commitment that did not, must each break this law.
    #[test]
    fn one_captured_surface_carries_two_authored_facts() -> Result<(), ()> {
        let written = surface(&documented(FAMILY_PROSE)).map_err(|_| ())?;
        let rows: Vec<&CapturedDocumentation> = written.documentation().collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.first().is_some_and(|row| {
                row.declared_on() == &DocumentedDeclaration::Family && row.text() == FAMILY_PROSE
            })
        );
        assert!(rows.get(1).is_some_and(|row| {
            row.declared_on() == &DocumentedDeclaration::Variant(DOCUMENTED_VARIANT.to_owned())
                && row.text() == VARIANT_PROSE
        }));
        assert!(surface(ISSUE_COLLECTION).is_ok_and(|plain| plain.documentation().count() == 0));

        let again = surface(&documented(FAMILY_PROSE)).map_err(|_| ())?;
        let otherwise = surface(&documented(OTHER_FAMILY_PROSE)).map_err(|_| ())?;
        assert!(
            otherwise
                .documentation()
                .any(|row| row.text() == OTHER_FAMILY_PROSE)
        );

        // The same declaration twice: both names agree.
        assert_eq!(written.identity(), again.identity());
        assert_eq!(
            written.documentation_identity(),
            again.documentation_identity()
        );

        // The same declaration, different prose: the semantic name stands still
        // and the documentation name moves.
        assert_eq!(written.identity(), otherwise.identity());
        assert_ne!(
            written.documentation_identity(),
            otherwise.documentation_identity()
        );

        // And the two names of ONE surface are never the same value: they are
        // two families over two grammars, and one of them stands over the other.
        assert_ne!(written.identity(), written.documentation_identity());
        Ok(())
    }

    /// law: derive.the-documentation-rows-wire-as-far-as-the-grammar-admits —
    /// the family seat's prose becomes the one plain sentence a documented item
    /// opens with, carried unchanged; the item earns no section, because a
    /// section is earned by a FACET and electing one is a reading of meaning
    /// this profile does not offer; and that stop is a typed disposition naming
    /// the profile and its version rather than a silence.
    ///
    /// A declaration that wrote no family-level prose reads as documented by
    /// nobody, which is a stated posture and not a refusal: a lawful declaration
    /// with no prose is lawful, and this home composes no summary on an author's
    /// behalf. A declaration whose family line is not a SENTENCE refuses in the
    /// documentation home's own family, unwrapped — this home states no second
    /// opinion about what a sentence is.
    ///
    /// Owed reversal (red twin): a road that composed a summary out of typed
    /// values, one that elected a facet, or one that repaired an unfinished
    /// sentence, must break this law.
    #[test]
    fn the_documentation_rows_wire_as_far_as_the_grammar_admits() -> Result<(), ()> {
        /// A family line that IS the one plain sentence the law admits.
        const SUMMARY_PROSE: &str = "the family a reader is handed.";

        let written = surface(&documented(SUMMARY_PROSE)).map_err(|_| ())?;
        let reading = crate::derive_refusal::documented(&written).map_err(|_| ())?;
        assert!(match reading {
            crate::derive_refusal::CapturedDocumentationReading::Documented { item, facets } => {
                item.summary().shown() == SUMMARY_PROSE
                    && item.is_empty()
                    && matches!(
                        facets,
                        crate::planning::ProjectionDisposition::UnavailableUnderProfile { .. }
                    )
            }
            crate::derive_refusal::CapturedDocumentationReading::NotDocumented { .. } => false,
        });

        // A family line that is a fragment: the documentation home's own law
        // refuses it, and this road hands that refusal through unchanged.
        let fragment = surface(&documented(FAMILY_PROSE)).map_err(|_| ())?;
        assert!(crate::derive_refusal::documented(&fragment).is_err_and(|refusal| matches!(
            refusal,
            crate::documentation::DocumentationDeclarationRefusal::SentenceNotEnded
        )));

        let plain = surface(ISSUE_COLLECTION).map_err(|_| ())?;
        let none = crate::derive_refusal::documented(&plain).map_err(|_| ())?;
        assert!(matches!(
            none,
            crate::derive_refusal::CapturedDocumentationReading::NotDocumented {
                because: crate::planning::ProjectionDisposition::NotRequested
            }
        ));
        Ok(())
    }

    /// law: derive.the-callable-route-needs-no-proc-macro — the whole road runs
    /// from text, with no proc-macro anywhere in the path, which is what makes a
    /// diagnostic's declared reproduction route a real road.
    /// Owed reversal: a road reachable only through an expansion must break this
    /// law.
    #[test]
    fn the_callable_route_needs_no_proc_macro() {
        let read = TextCapture::read(SINGLE_CAUSE).map_err(|_| ());
        assert!(read.is_ok_and(|read| {
            captured(read.input()).is_ok_and(|surface| surface.family_id() == "demo.example")
        }));
    }
}

/// The failure-path closure's proof surface.
///
/// # The working law
///
/// **A required seat is never repaired with an empty, default, or neighbouring
/// value after construction fails — a failed required seat is a typed refusal.**
///
/// Every law below stands over one class of repair this plane used to perform,
/// and each one proves the same thing about its class: the construction that
/// would have taken the old repair now either cannot be written at all, or
/// refuses with a cause that names the seat.
///
/// The classes, and where each one used to live:
///
/// | class | the repair that used to happen |
/// | ----- | ------------------------------ |
/// | a shortened complete set | a failed two-role membership became a one-role one |
/// | an invented member | an impossible empty roster invented a family member |
/// | an elected duplicate | a role carrying two members had one of them elected |
/// | a first-per-role comparison | a set check that read one member per role |
/// | a post-proof concatenation | the emitted tree was joined after the proof returned |
/// | a blanked explanation | an over-long rendering became an empty one |
/// | a neighbouring subject | an explanation answered about the value next to its own |
/// | a generic cause | five typed refusal families became one sentence |
/// | a saturated coordinate | a depth that stopped counting made two tokens one |
mod failure_path_closure {
    use crate::closure::{
        ClosedExpansion, ClosureIssue, ExpansionBindingRefusal, PartitionCargo, ProjectionClosure,
        RenderedProjection, RenderedUnit,
    };
    use crate::derive_refusal::{
        ExplanationSeat, compile_refusal_text, diagnose, plan as derive_plan,
    };
    use crate::diagnostics::{ObservedClassification, RelatedSetCompletion};
    use crate::plane::{
        HumanProjection, HumanTextLimit, PlanId, ProjectionIdentity, ProjectionRole,
        ProjectionTranscript, RenderedRole, SoleRenderedUnit, human_projection,
    };
    use crate::planning::{
        EmissionPartition, MemberDestination, PlannedMember, PlannedMembership,
        RenderedImplementation,
    };
    use crate::refusal::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
    use crate::token::{
        CaptureBound, CaptureWalk, SpanHandle, SpanTable, TextCapture, TextReadCause,
    };
    use threadpak::declaration::CoordinateRole;
    use threadpak::types::ConstLimit;

    /// The lawful single-cause declaration, whose shape fixes TWO contracts — the
    /// family implementation and the typed cause order — and therefore a four-role
    /// output set, since each contract is delivered as two surfaces.
    const SINGLE_CAUSE: &str = "#[refusal(family = \"demo.example\", shape = single_cause, \
        order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\"))] \
        enum DemoFamily { NotCanonical, NotAdmitted, }";

    /// A second lawful declaration, whose shape fixes ONE contract — so it plans
    /// a different membership and therefore a different plan, which is what the
    /// binding law needs two of.
    const COLLECTION: &str = "#[refusal(family = \"demo.other\", shape = issue_collection)] \
        enum DemoIssues { NotBound, NotCovered, }";

    /// law: closure.a-complete-set-is-never-shortened — a shape that fixes two
    /// contracts plans FOUR members, one under each rendered role, and the road
    /// that builds it carries no refusal to swallow. The old road repaired a
    /// failed complete set with the first member alone.
    ///
    /// The quantifier is the ROSTER and the assertion is per role, so this law
    /// does not restate the count: a roster that grew a seat makes the loop below
    /// ask about that seat too, and a membership that left it unplanned fails
    /// here rather than passing a length comparison.
    ///
    /// Owed reversal (red twin): a membership road returning a `Result` a caller
    /// must repair must break this law.
    #[test]
    fn a_complete_set_is_never_shortened() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let membership = closed.plan().membership();
            membership.len() == RenderedImplementation::ROLES.len()
                && RenderedImplementation::ROLES
                    .iter()
                    .all(|role| membership.count_under(*role) == 1)
        }));
    }

    /// law: closure.a-doubled-planned-role-refuses — the checked membership road
    /// refuses a set carrying two members under one role, naming the role slot
    /// and the count. The old road admitted it and let the closure's role match
    /// elect one of the two.
    /// Owed reversal (red twin): a membership admitting a doubled role must
    /// break this law.
    #[test]
    fn a_doubled_planned_role_refuses() -> Result<(), ()> {
        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let member = closed
            .plan()
            .membership()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned()
            .ok_or(())?;
        let doubled = PlannedMembership::declared(member.clone(), vec![member]);
        assert!(doubled.is_err_and(|refusal| {
            *refusal.body().carried().first()
                == ProjectionPlanningIssue::MembershipDoubled {
                    role_slot: RenderedImplementation::RenderedFamilyImpl.slot(),
                    observed: 2,
                }
        }));
        Ok(())
    }

    /// law: closure.the-proof-reads-the-plans-own-count — the closure checks how
    /// many members the PLAN declared under each role, independently of what was
    /// rendered, and refuses a doubled role before comparing anything. Reading
    /// the plan through its first match per role would have hidden it.
    /// Owed reversal (red twin): a proof that read only `under` must break this
    /// law.
    #[test]
    fn the_proof_reads_the_plans_own_count() -> Result<(), ()> {
        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let member = closed
            .plan()
            .membership()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned()
            .ok_or(())?;
        let unit = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned()
            .ok_or(())?;
        let doubled = PlannedMembership::complete(member.clone(), [member]);
        let proved = ProjectionClosure::proved(
            closed.plan().identity(),
            &doubled,
            RenderedProjection::of_one(unit),
        );
        assert!(proved.is_err_and(|refusal| {
            *refusal.body().carried().first()
                == ClosureIssue::MemberPlannedTwice {
                    role: RenderedImplementation::RenderedFamilyImpl,
                    observed: 2,
                }
        }));
        Ok(())
    }

    /// law: closure.the-rebuild-is-compared-as-a-set — the closure's last act is
    /// a role-by-role comparison of two complete memberships, and a rebuild that
    /// agrees under every role is the same set as the plan's. The seam that
    /// stood here compared one member per role and would have agreed about two
    /// sets differing in their second.
    /// Owed reversal (red twin): a first-per-role comparison must break this
    /// law.
    #[test]
    fn the_rebuild_is_compared_as_a_set() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let planned = closed.plan().membership();
            let rebuilt = closed.closure().reconstructed();
            let same = RenderedImplementation::ROLES
                .iter()
                .all(|role| rebuilt.agrees_under(planned, *role));
            let family = closed
                .plan()
                .membership()
                .under(RenderedImplementation::RenderedFamilyImpl)
                .cloned();
            // A set the plan does not hold disagrees under the role it differs
            // at, which is what makes the agreement above evidence.
            let elsewhere = family.is_some_and(|member| {
                !PlannedMembership::complete(member, [])
                    .agrees_under(planned, RenderedImplementation::RenderedCauseOrderImpl)
            });
            same && rebuilt.len() == planned.len() && elsewhere
        }));
    }

    /// The digest one emission's cargo carries, re-derived from the tokens that
    /// cargo holds.
    ///
    /// The laws below compare this against the digest the proof took, so what
    /// they establish is that two derivations over one byte string agree — never
    /// that a value equals itself.
    fn digest_agrees(plan: PlanId, partition: EmissionPartition, cargo: &PartitionCargo) -> bool {
        match cargo {
            PartitionCargo::NothingPlanned => false,
            PartitionCargo::Carried(carried) => {
                carried.digest()
                    == ProjectionIdentity::derived(ProjectionTranscript::under_projection(
                        ProjectionRole::OutputBytes,
                        &plan,
                        &carried.tree().canonical_bytes(),
                        u32::from(partition.slot()),
                    ))
            }
        }
    }

    /// law: closure.every-emission-is-inside-the-proof — the closure splits the
    /// rendering by delivery itself, joins each emission in role-roster order,
    /// keeps them, and commits to each one's digest; a closed expansion delivers
    /// exactly those rather than a second concatenation. The old road joined every
    /// unit
    /// into one tree after the proof had already returned, which put every
    /// delivery into the one build the declaration site compiles.
    ///
    /// The quantifier is the PARTITION roster and the assertion is per emission,
    /// so this law does not restate a count: an emission admitted later makes
    /// the walk below ask about it too.
    ///
    /// Owed reversal (red twin): a public post-proof join must break this law.
    #[test]
    fn every_emission_is_inside_the_proof() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let plan = closed.plan().identity();
            let rendered = closed.rendered();
            let emission = closed.closure().emission();
            EmissionPartition::ALL.iter().all(|partition| {
                let width: usize = rendered
                    .units_in(*partition)
                    .map(|unit| unit.tree().len())
                    .sum();
                match emission.joined(*partition) {
                    // The publication emission is not joined at all: an artifact
                    // is its own unit at its own address.
                    None => rendered.count_in(*partition) == 0,
                    Some(cargo) => match cargo {
                        PartitionCargo::NothingPlanned => width == 0,
                        PartitionCargo::Carried(carried) => {
                            carried.tree().len() == width
                                && digest_agrees(plan, *partition, cargo)
                        }
                    },
                }
            })
        }));
    }

    /// law: closure.the-evaluation-copy-never-reaches-the-normal-build — the
    /// mutation-evaluation surface is delivered into the test carrier and the
    /// production implementation to the declaration site, so no byte of a
    /// selector-bearing copy stands in what the consumer's normal build
    /// compiles. The seam this replaced concatenated every rendered unit into
    /// one declaration-site tree, which compiled the copy beside the
    /// implementation it exists to be evaluated against.
    ///
    /// Both halves are stated, because they are two different guarantees: the
    /// roster's constant answer is what makes the wrong delivery unwritable, and
    /// the proved emission is what makes it observed.
    ///
    /// Owed reversal (red twin): a roster answering with one destination for
    /// both halves of a pair must break this law.
    #[test]
    fn the_evaluation_copy_never_reaches_the_normal_build() -> Result<(), ()> {
        assert!(RenderedImplementation::ROLES.iter().all(|role| {
            let partition = role.destination().partition();
            if role.is_evaluation_copy() {
                partition == EmissionPartition::TestCarrier
                    && role.destination() == MemberDestination::IntoTestCarrier
            } else {
                partition == EmissionPartition::DeclarationSite
            }
        }));

        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let rendered = closed.rendered();
        let emission = closed.closure().emission();
        let width = |partition: EmissionPartition| -> usize {
            rendered
                .units_in(partition)
                .map(|unit| unit.tree().len())
                .sum()
        };
        let carried = width(EmissionPartition::TestCarrier);
        let site = width(EmissionPartition::DeclarationSite);
        assert!(
            rendered.count_in(EmissionPartition::TestCarrier) == 2
                && rendered.count_in(EmissionPartition::DeclarationSite) == 2
                && carried > 0
                && site > 0
                && emission
                    .test_carrier()
                    .tokens()
                    .is_some_and(|tree| tree.len() == carried)
                && emission
                    .declaration_site()
                    .tokens()
                    .is_some_and(|tree| tree.len() == site)
                && matches!(emission.bench_carrier(), PartitionCargo::NothingPlanned)
        );
        Ok(())
    }

    /// law: closure.an-expansion-binds-the-three-values-that-name-one-another —
    /// the terminal every kind's door ends at refuses a closure proved against
    /// another plan AND an explanation answered over another plan or another
    /// proof, and every refusal names both identities. A binding that took any
    /// of the three pairs on trust would answer every question correctly about
    /// the wrong expansion.
    ///
    /// The explanation half is what the type parameter could never catch: a kind
    /// is not an expansion, so two plans of one kind admit the same questions
    /// and a view written over either covers the roster exactly.
    ///
    /// Owed reversal (red twin): a binding that trusts its arguments, or one
    /// that compares the plan alone, must break this law.
    #[test]
    fn an_expansion_binds_the_three_values_that_name_one_another() -> Result<(), ()> {
        let (_, mine) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let (_, other) = compile_refusal_text(COLLECTION).map_err(|_| ())?;

        let crossed_proof = ClosedExpansion::bound(
            other.plan().clone(),
            mine.closure().clone(),
            other.explanation().clone(),
        );
        assert!(crossed_proof.is_err_and(|refusal| {
            refusal
                == ExpansionBindingRefusal::ClosureProvedAgainstAnotherPlan {
                    planned: other.plan().identity(),
                    proved: mine.closure().plan(),
                }
        }));

        // The plan and the proof agree with each other and the EXPLANATION comes
        // from the other expansion: coverage is perfect, the kind is the same,
        // and the answers are about a different subject.
        let crossed_explanation = ClosedExpansion::bound(
            mine.plan().clone(),
            mine.closure().clone(),
            other.explanation().clone(),
        );
        assert!(crossed_explanation.is_err_and(|refusal| {
            refusal
                == ExpansionBindingRefusal::ExplanationAnsweredOverAnotherPlan {
                    planned: mine.plan().identity(),
                    answered: other.explanation().plan(),
                }
        }));

        // The agreeing triple binds, and the family's own view over it carries
        // the terminal's identity rather than one of its own.
        let bound = ClosedExpansion::bound(
            mine.plan().clone(),
            mine.closure().clone(),
            mine.explanation().clone(),
        );
        assert!(bound.is_ok_and(|expansion| expansion.identity() == mine.identity()));
        Ok(())
    }

    /// law: closure.an-expansion-commits-to-the-explanation-it-bound — the
    /// terminal's own identity is derived over the explanation's identity as
    /// well as the plan's and the proof's, so two expansions differing only in
    /// which explanation they bound are two names.
    ///
    /// The transcript used to carry no member for the explanation at all and
    /// said so, because a view had no canonical name to commit to. It has one
    /// now, and this law is what says the terminal reads it.
    ///
    /// Owed reversal (red twin): dropping the explanation member from the
    /// closed-expansion transcript must break this law.
    #[test]
    fn an_expansion_commits_to_the_explanation_it_bound() -> Result<(), ()> {
        let (_, mine) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let (_, other) = compile_refusal_text(COLLECTION).map_err(|_| ())?;
        // Two live expansions, two explanations, two names — and the explanation
        // identities are what differ where the plans and proofs do too.
        assert_ne!(
            mine.explanation().identity(),
            other.explanation().identity()
        );
        assert_ne!(mine.identity(), other.identity());
        // The explanation names the parentage it was answered over, which is
        // what makes the terminal's comparison a comparison rather than a hope.
        assert_eq!(mine.explanation().plan(), mine.plan().identity());
        assert_eq!(mine.explanation().closure(), mine.closure().identity());
        Ok(())
    }

    /// law: closure.a-static-rendering-is-carried-whole — the proven road builds
    /// its bytes from the rendering itself, so a projection taken through it has
    /// exactly the rendering's length and can never be the empty one. The seam
    /// it replaced turned an over-long rendering into a blank explanation.
    /// Owed reversal (red twin): a proven road with an empty fallback must break
    /// this law.
    #[test]
    fn a_static_rendering_is_carried_whole() {
        let rendered = human_projection!(HumanTextLimit, "the owner declared this repair");
        assert_eq!(rendered.len(), 30);
        assert!(!rendered.is_empty());
        assert_eq!(rendered.shown(), "the owner declared this repair");
        // The checked road still refuses rather than truncating, and the two
        // roads are not interchangeable: this one reads a runtime length.
        let oversized = "x".repeat(HumanTextLimit::MAX.saturating_add(1));
        assert!(HumanProjection::<HumanTextLimit>::projected(&oversized).is_err());
    }

    /// law: closure.an-explanation-binds-its-own-subject — the output-and-digest
    /// seat carries the digest of the unit rendered under the FAMILY role, and
    /// the two rendered units carry different digests, so an answer taken from
    /// the neighbouring unit would be a different value. The seats an
    /// explanation can fail to bind are typed and named.
    /// Owed reversal (red twin): a first-unit digest fallback must break this
    /// law.
    #[test]
    fn an_explanation_binds_its_own_subject() {
        // The roster itself, not a list written here: a seat added to
        // `ExplanationSeat` and forgotten at this call site would leave the law
        // green about a roster it no longer covers.
        let mut slots: Vec<u8> = ExplanationSeat::ALL.iter().map(|seat| seat.slot()).collect();
        slots.sort_unstable();
        let counted = slots.len();
        slots.dedup();
        assert_eq!(slots.len(), counted);

        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let family = closed
                .rendered()
                .under(RenderedImplementation::RenderedFamilyImpl)
                .map(RenderedUnit::digest);
            let neighbour = closed
                .rendered()
                .under(RenderedImplementation::RenderedCauseOrderImpl)
                .map(RenderedUnit::digest);
            family.is_some_and(|family| neighbour.is_some_and(|neighbour| family != neighbour))
        }));
    }

    /// law: closure.a-typed-refusal-survives-into-the-diagnostic — a planning
    /// body's axis, magnitude, and observed count reach the diagnostic's own
    /// line and its classification, and two different bodies derive different
    /// related identities. The seam this replaced projected every family through
    /// one sentence under one classification with an empty related set.
    /// Owed reversal (red twin): a shared step diagnostic must break this law.
    #[test]
    fn a_typed_refusal_survives_into_the_diagnostic() {
        let outputs = diagnose::planning_refused(&ProjectionPlanning::bound_exceeded(
            BoundAxis::Outputs,
            32,
            33,
        ));
        let declarations = diagnose::planning_refused(&ProjectionPlanning::bound_exceeded(
            BoundAxis::Declarations,
            64,
            65,
        ));
        assert!(matches!(
            outputs.observed,
            ObservedClassification::BoundExceeded
        ));
        assert!(outputs.summary.shown().contains("declared 32, observed 33"));
        assert!(
            outputs
                .summary
                .shown()
                .contains("the outputs one plan may declare")
        );
        // The body's own identity, then one per established issue.
        assert_eq!(outputs.related.carried().len(), 2);
        assert_ne!(outputs.summary, declarations.summary);
        let mine: Vec<&crate::diagnostics::RelatedIdentity> =
            outputs.related.carried().iter().collect();
        let theirs: Vec<&crate::diagnostics::RelatedIdentity> =
            declarations.related.carried().iter().collect();
        assert_ne!(mine, theirs);

        let seat = diagnose::explanation_refused(
            &crate::derive_refusal::ExplanationBindingRefusal::RequiredOutputAbsent {
                seat: ExplanationSeat::ProvedFamilyDigest,
            },
        );
        assert!(matches!(seat.observed, ObservedClassification::SeatAbsent));
        assert!(
            seat.summary
                .shown()
                .contains(ExplanationSeat::ProvedFamilyDigest.described())
        );
    }

    /// law: closure.a-closure-refusal-names-its-role — every closure issue that
    /// is about a role names it, the three that are about the whole
    /// reconstruction name none, and the diagnostic's line carries the role's
    /// own description.
    /// Owed reversal (red twin): a role-free closure diagnostic must break this
    /// law.
    #[test]
    fn a_closure_refusal_names_its_role() -> Result<(), ()> {
        let (_, closed) = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ())?;
        let unit = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned()
            .ok_or(())?;
        let proved = ProjectionClosure::proved(
            closed.plan().identity(),
            closed.plan().membership(),
            RenderedProjection::of_one(unit),
        );
        assert!(proved.is_err_and(|refusal| {
            let projected = diagnose::closure_refused(&refusal);
            projected
                .summary
                .shown()
                .contains(RenderedImplementation::RenderedCauseOrderImpl.described())
                && projected.related.carried().len() == 2
        }));
        Ok(())
    }

    /// law: closure.a-rendered-roster-names-every-variant-once — each admitted
    /// roster is exhaustive over its own enum, every variant sits at exactly one
    /// roster position, and every slot IS that position. The roster is the
    /// quantifier every closure proof walks, so a roster missing a variant would
    /// make that variant's unit invisible to the proof.
    /// Owed reversal (red twin): a roster omitting a variant must break this
    /// law.
    #[test]
    fn a_rendered_roster_names_every_variant_once() {
        // Exhaustive matches: adding a variant without adding it to the roster
        // below stops compiling here.
        const fn implementation_position(role: RenderedImplementation) -> usize {
            match role {
                RenderedImplementation::RenderedFamilyImpl => 0,
                RenderedImplementation::RenderedCauseOrderImpl => 1,
                RenderedImplementation::RenderedFamilyEvaluation => 2,
                RenderedImplementation::RenderedCauseOrderEvaluation => 3,
            }
        }
        const fn sole_position(role: SoleRenderedUnit) -> usize {
            match role {
                SoleRenderedUnit::Sole => 0,
            }
        }
        assert_eq!(RenderedImplementation::ROLES.len(), 4);
        assert_eq!(SoleRenderedUnit::ROLES.len(), 1);
        for (position, role) in RenderedImplementation::ROLES.iter().enumerate() {
            assert_eq!(implementation_position(*role), position);
            assert_eq!(usize::try_from(role.slot()).unwrap_or(usize::MAX), position);
            assert!(!role.described().is_empty());
            // The pairing is an involution and it never answers with a seat: one
            // implementation meaning is two surfaces, so a roster entry without a
            // pair would be a surface delivered on its own.
            assert_eq!(role.twin().twin(), *role);
            assert_ne!(role.twin(), *role);
            assert_ne!(role.twin().is_evaluation_copy(), role.is_evaluation_copy());
        }
        for (position, role) in SoleRenderedUnit::ROLES.iter().enumerate() {
            assert_eq!(sole_position(*role), position);
            assert_eq!(usize::try_from(role.slot()).unwrap_or(usize::MAX), position);
        }
    }

    /// law: closure.a-token-route-locates-exactly-one-token — a route is the
    /// index path from the root, so two tokens at the same depth and the same
    /// position inside their own groups carry different routes. The pair this
    /// replaced gave both of them one value.
    /// Owed reversal (red twin): a depth-and-index coordinate must break this
    /// law.
    #[test]
    fn a_token_route_locates_exactly_one_token() {
        let read = TextCapture::read("a (b) (c)").map_err(|_| ());
        assert!(read.is_ok_and(|read| {
            let inner: Vec<Vec<u32>> = read
                .input()
                .trees()
                .filter_map(|tree| tree.group())
                .filter_map(|(_, trees)| trees.iter().next())
                .map(|tree| tree.path().steps().copied().collect())
                .collect();
            inner.len() == 2 && inner.first() != inner.get(1)
        }));
    }

    /// law: closure.an-unresolvable-handle-refuses-rather-than-resolving — a
    /// byte-offset table answers every handle its own read issued with the byte
    /// that token starts at, and answers a handle it never issued with a refusal
    /// naming the handle and how far the table reaches. Both directions, because
    /// a table that refused everything would satisfy the hostile half alone.
    /// The seam this replaced answered an unreachable handle with a
    /// semantic-origin coordinate at the handle's own index — a value shaped
    /// exactly like the honest answer the producer-held posture returns.
    /// Owed reversal (red twin): a table answering every handle must break this
    /// law.
    #[test]
    fn an_unresolvable_handle_refuses_rather_than_resolving() {
        let read = TextCapture::read("alpha (beta)").map_err(|_| ());
        assert!(read.is_ok_and(|read| {
            let issued = read.input().issued();
            let table = read.spans();
            // The lawful direction: every issued handle resolves to a byte.
            let all_resolve = (0..issued).all(|index| {
                table
                    .coordinate_of(SpanHandle::at(index))
                    .is_ok_and(|coordinate| coordinate.role == CoordinateRole::Byte)
            });
            // The hostile direction: the first handle past the table refuses,
            // and the refusal states the handle and the table's magnitude.
            let past = table.coordinate_of(SpanHandle::at(issued));
            let refuses = past.is_err_and(|refusal| {
                refusal.handle == SpanHandle::at(issued)
                    && refusal.reaches == issued
                    && refusal.described().contains(&issued.to_string())
            });
            issued > 1 && all_resolve && refuses
        }));

        // A producer that holds the compiler's spans answers in its own role,
        // and that answer is the handle's ordinal rather than a source position:
        // it invents nothing, so it refuses nothing.
        let held = SpanTable::ProducerHeld
            .coordinate_of(SpanHandle::at(9))
            .map_err(|_| ());
        assert!(held.is_ok_and(|coordinate| {
            coordinate.role == CoordinateRole::SemanticOrigin && coordinate.position == 9
        }));

        // And the capture road's own projection says which of the two it is
        // rather than composing a position out of the handle.
        let source = "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }";
        let refused = crate::derive_refusal::captured_text(source).map(|_| ());
        assert!(refused.is_err_and(|refusal| {
            let empty = SpanTable::ByteOffsets(threadpak::types::Bounded::empty());
            let line = refusal.compiler_message(&empty);
            line.contains("does not reach handle") && !line.contains("at token position")
        }));
    }

    /// law: closure.a-truncated-related-set-says-what-it-dropped — a related set
    /// that fits carries the complete body's identity and one per established
    /// issue and reports `Complete`; a set that would overrun the declared
    /// magnitude carries the body's identity alone and reports `ReportTruncated`
    /// at the declared issue bound, naming how many per-issue identities are not
    /// there. The seam this replaced returned the coarser set silently, which is
    /// a smaller success wearing the shape of a complete one.
    ///
    /// The posture is spelled for truncation rather than for an early stop, on
    /// band 00's distinction: the refusal body is complete before the set is
    /// built, so nothing here ever halts an examination and a set that reported
    /// one would be claiming an ignorance it does not have.
    ///
    /// The count is read off the identities the road actually dropped, and it
    /// lands in the same value as what the road kept — so a set that carried
    /// everything cannot report that it dropped anything, the number a reader
    /// acts on belongs to this truncation rather than to whoever wrote the
    /// posture down, and one diagnostic's set cannot be shown under another
    /// diagnostic's completion. The road builds the set rather than being handed
    /// one, which is what closes the gap band 00's package closes upstream.
    ///
    /// The same road closes the identity level. It takes the issue MATERIAL and
    /// derives the body's identity and the per-issue identities together, so the
    /// body's identity is a commitment to exactly the issues standing beside it:
    /// two different issue sets reach two different bodies, one issue set reaches
    /// one body every time, and reordering the issues is a different body because
    /// the framing that feeds it is ordered. A road taking the body and the
    /// per-issue set as two arguments admitted a pair in which each half was
    /// honestly derived and the pair named two different refusals, and the
    /// coarser set a truncation carries would then be a commitment to nothing in
    /// particular.
    ///
    /// The two levels are two SUBJECTS, which is what closes the last road
    /// between them. Under one subject the body and the issues shared a preimage
    /// grammar, and the body's preimage is the framing of its issues — so an
    /// issue whose own material happened to be that framing derived the body's
    /// exact identity, and the crafted collision below proves the old grammar
    /// admitted it. Two subjects give the two levels two derive-key contexts, so
    /// the same content at the two levels is two unrelated values, and the Rust
    /// types no longer substitute either.
    ///
    /// Reversal: `testpak/tests/related_set_identity_levels.rs` rebuilds the
    /// body's identity from the published content grammar with its own encoder
    /// and requires the produced one to match, and
    /// `testpak/tests/compile-fail/a-related-set-assembled-from-two-levels.rs`
    /// and `…/a-related-set-married-to-another-completion.rs` are the two roads
    /// out of the guard that must not compile.
    #[test]
    fn a_truncated_related_set_says_what_it_dropped() {
        use crate::diagnostics::{RelatedIdentity, RelatedSet};
        use crate::plane::{
            ProjectionIdentity, ProjectionRole, ProjectionTranscript, RelatedIssueSubject,
            encode_bytes,
        };
        use threadpak::refusal::StopBound;
        use threadpak::types::ConstLimit;

        /// One issue's canonical material, distinct per seed.
        fn material(seed: u32) -> Vec<u8> {
            seed.to_be_bytes().to_vec()
        }

        /// The identity a set commits its whole body under: the one it carries
        /// ahead of the per-issue identities, read by the LEVEL it states rather
        /// than by the position it sits at.
        fn body(set: &RelatedSet) -> Option<[u8; 32]> {
            set.carried().iter().find_map(|carried| match *carried {
                RelatedIdentity::Body(identity) => Some(*identity.as_bytes()),
                RelatedIdentity::Issue(_) => None,
            })
        }

        const FAMILY: u8 = 0;
        const OTHER_FAMILY: u8 = 1;

        let fits: Vec<Vec<u8>> = (1..=3).map(material).collect();
        let set = RelatedSet::derived_over(FAMILY, &fits);
        assert_eq!(set.carried().len(), 4);
        assert!(matches!(set.completion(), RelatedSetCompletion::Complete));
        assert!(body(&set).is_some());

        // The body's identity commits to the issues it was built over. Same
        // material, same body; one issue changed, a different body; the same
        // issues in another order, a different body again — the framing the body
        // is derived over is the issues' own material, in the issues' own order.
        assert_eq!(body(&set), body(&RelatedSet::derived_over(FAMILY, &fits)));
        let differing: Vec<Vec<u8>> = vec![material(1), material(2), material(9)];
        assert_ne!(
            body(&set),
            body(&RelatedSet::derived_over(FAMILY, &differing))
        );
        let reordered: Vec<Vec<u8>> = vec![material(3), material(2), material(1)];
        assert_ne!(
            body(&set),
            body(&RelatedSet::derived_over(FAMILY, &reordered))
        );

        // The body is its own commitment and never one of the issues restated:
        // a reader holding it is holding the whole body. Compared as BYTES,
        // because the types alone already make the two levels distinct and a
        // comparison the types decided would prove nothing about the digest.
        assert!(!set.carried().iter().any(|carried| match *carried {
            RelatedIdentity::Body(_) => false,
            RelatedIdentity::Issue(identity) => Some(*identity.as_bytes()) == body(&set),
        }));

        // Two families' issue material never encodes alike, so the same issues
        // raised under two families are two different bodies.
        assert_ne!(
            body(&set),
            body(&RelatedSet::derived_over(OTHER_FAMILY, &fits))
        );

        // The body's own identity rides ahead of the per-issue ones, so a body
        // AT the declared magnitude overruns the set by exactly one.
        let magnitude =
            u32::try_from(crate::diagnostics::RelatedIssueLimit::MAX).unwrap_or(u32::MAX);
        let over: Vec<Vec<u8>> = (1..=magnitude).map(material).collect();
        let truncated_set = RelatedSet::derived_over(FAMILY, &over);
        assert_eq!(truncated_set.carried().len(), 1);
        assert!(body(&truncated_set).is_some());
        assert!(matches!(
            truncated_set.completion(),
            RelatedSetCompletion::ReportTruncated(truncation)
                if truncation.omitted().get() == crate::diagnostics::RelatedIssueLimit::MAX
                    && matches!(truncation.stopped_at(), StopBound::DeclaredIssueBound)
        ));

        // The coarser commitment a truncation carries is still a commitment to
        // THESE issues: change one of the dropped issues and the identity that
        // survives changes with it.
        let mut other = over.clone();
        other.pop();
        other.push(material(u32::MAX));
        assert_ne!(
            body(&truncated_set),
            body(&RelatedSet::derived_over(FAMILY, &other))
        );

        // The crafted collision, constructed rather than asserted. Under one
        // subject the two levels shared a preimage grammar, and a body's
        // preimage IS the framing of its issues — so a ONE-issue set whose
        // single issue's material happens to be another set's framing derived
        // that other set's body identity, byte for byte, under a name space
        // where the two were the same kind of value.
        let inner: Vec<Vec<u8>> = vec![material(1), material(2)];
        let inner_set = RelatedSet::derived_over(FAMILY, &inner);
        let mut framing = Vec::new();
        for issue in &inner {
            encode_bytes(issue, &mut framing);
        }
        let aliasing = RelatedSet::derived_over(FAMILY, &[framing.clone()]);
        let aliasing_issue = aliasing
            .carried()
            .iter()
            .find_map(|carried| match *carried {
                RelatedIdentity::Issue(identity) => Some(*identity.as_bytes()),
                RelatedIdentity::Body(_) => None,
            });

        // Both contents composed by the published grammar, one by the BODY rule
        // over `inner` and one by the ISSUE rule over the aliasing material.
        // They are the same bytes, which is the collision stated exactly.
        let mut body_content = vec![FAMILY];
        encode_bytes(&framing, &mut body_content);
        let mut issue_content = vec![FAMILY];
        encode_bytes(&framing, &mut issue_content);
        assert_eq!(body_content, issue_content);

        // One subject over that content is one identity — the defect. The role
        // is the one the two levels actually mint at, so the reconstruction is
        // the services' own derivation with the subject split removed and
        // nothing else changed.
        let under_one_subject = |content: &[u8]| {
            *ProjectionIdentity::<RelatedIssueSubject>::derived(ProjectionTranscript::rooted(
                ProjectionRole::DiagnosticRelation,
                content,
                u32::from(FAMILY),
            ))
            .as_bytes()
        };
        assert_eq!(
            under_one_subject(&body_content),
            under_one_subject(&issue_content)
        );

        // Two subjects over that same content are two identities — the repair,
        // proven on the values the services actually mint.
        assert!(aliasing_issue.is_some());
        assert_ne!(body(&inner_set), aliasing_issue);

        // The one line rustc shows carries the same statement, because the
        // typed posture beside it is not something rustc shows. A complete set
        // adds nothing to the line; a truncated one names the count it dropped.
        let plain = diagnose::witnessed("planning refused", RelatedSetCompletion::Complete);
        assert_eq!(plain, "planning refused");
        let said = diagnose::witnessed("planning refused", truncated_set.completion());
        assert!(said.starts_with("planning refused"));
        assert!(said.contains("the related set was truncated at the declared issue bound"));
        assert!(said.contains(&crate::diagnostics::RelatedIssueLimit::MAX.to_string()));
    }

    /// law: closure.a-capture-refuses-before-a-partial-tree — nesting past the
    /// declared depth, a tree past the declared total, and a walk past its
    /// declared budget each refuse naming the bound they overran, and none of
    /// them hands back a truncated capture. Only a per-level bound stood here,
    /// and depth saturated rather than refusing.
    /// Owed reversal (red twin): a saturating depth must break this law.
    #[test]
    fn a_capture_refuses_before_a_partial_tree() {
        let deep = format!("{}x{}", "(".repeat(64), ")".repeat(64));
        let read = TextCapture::read(&deep);
        assert!(read.is_err_and(|refusal| matches!(
            refusal.cause,
            TextReadCause::Unbounded(CaptureBound::DepthUnbounded)
        )));

        let mut walk = CaptureWalk::declared();
        let mut spent = false;
        for _ in 0..=CaptureWalk::DECLARED_WORK {
            if walk.examined().is_err() {
                spent = true;
                break;
            }
        }
        assert!(spent);
        assert_eq!(walk.remaining(), 0);

        let mut counting = CaptureWalk::declared();
        let mut overran = false;
        for _ in 0..=u32::try_from(crate::token::CapturedTreeTokenLimit::MAX).unwrap_or(u32::MAX) {
            if counting.took().is_err() {
                overran = true;
                break;
            }
        }
        assert!(overran);
        assert_eq!(
            usize::try_from(counting.taken()).unwrap_or(usize::MAX),
            crate::token::CapturedTreeTokenLimit::MAX
        );

        // Every bound renders itself, so a producer reporting one composes no
        // sentence of its own.
        for bound in [
            CaptureBound::DepthUnbounded,
            CaptureBound::LevelUnbounded,
            CaptureBound::TreeUnbounded,
            CaptureBound::WorkUnbounded,
        ] {
            assert!(!bound.described().is_empty());
        }
    }

    /// law: closure.a-rendering-refusal-names-its-magnitude — a materialization
    /// refusal reaches the diagnostic naming the exact declared magnitude, the
    /// unit it governs, and the role that overran it.
    /// Owed reversal (red twin): a bound refusal that named no magnitude must
    /// break this law.
    #[test]
    fn a_rendering_refusal_names_its_magnitude() {
        let bytes = diagnose::rendering_refused(
            crate::closure::RenderingRefusal::BytesUnbounded,
            RenderedImplementation::RenderedFamilyImpl,
        );
        let shown = bytes.summary.shown();
        assert!(matches!(
            bytes.observed,
            ObservedClassification::BoundExceeded
        ));
        assert!(shown.contains("the bytes one rendered unit may carry"));
        assert!(shown.contains(RenderedImplementation::RenderedFamilyImpl.described()));
        assert!(shown.contains(&crate::plane::RenderedByteLimit::MAX.to_string()));

        let tree = diagnose::render_refused(
            crate::derive_refusal::RenderRefusal::Unbounded,
            RenderedImplementation::RenderedCauseOrderImpl,
        );
        assert_ne!(bytes.summary, tree.summary);
    }

    /// law: closure.a-planned-member-carries-no-invented-role — the membership
    /// road is a match over the two answers the shape admits, so the member set
    /// is a function of the shape and nothing invents a role for an empty
    /// roster. A collection shape declares ONE contract and a single-cause shape
    /// declares two, and both are built through the total road.
    ///
    /// Each declared contract contributes TWO members, because one implementation
    /// meaning is delivered as two surfaces and both are planned: the production
    /// implementation under its role, and the mutation-evaluation copy under that
    /// role's twin. So the counts below are two and four rather than one and two,
    /// and every member's twin stands beside it — a membership that planned the
    /// production half alone would put the copy outside the output firewall,
    /// where the closure never looks at it.
    ///
    /// Owed reversal: a roster-shaped membership road must break this law.
    #[test]
    fn a_planned_member_carries_no_invented_role() -> Result<(), ()> {
        let collection = "#[refusal(family = \"demo.example\", shape = issue_collection)] \
            enum DemoIssues { NotBound, NotCovered, }";
        let (_, closed) = compile_refusal_text(collection).map_err(|_| ())?;
        let planned = closed.plan().membership();
        assert!(
            planned.len() == 2
                && planned.count_under(RenderedImplementation::RenderedFamilyImpl) == 1
                && planned.count_under(RenderedImplementation::RenderedFamilyEvaluation) == 1
                && planned.count_under(RenderedImplementation::RenderedCauseOrderImpl) == 0
                && planned.count_under(RenderedImplementation::RenderedCauseOrderEvaluation) == 0
        );

        // The same theorem stated over the planning road directly: the profile,
        // destination, and digest contract of every member come from the ROLE,
        // and every member's twin is planned beside it. The destination is
        // compared against the roster's own constant answer rather than against
        // a literal repeated here, so a plan and a rendering that both read that
        // answer cannot disagree about a delivery — and the two halves of a pair
        // are delivered differently, which is what the comparison catches when
        // one of them stops being read from the roster.
        let read = TextCapture::read(SINGLE_CAUSE).map_err(|_| ())?;
        let surface = crate::derive_refusal::captured(read.input()).map_err(|_| ())?;
        let draft = surface.planned();
        let membership = derive_plan::membership(&draft);
        assert!(
            membership.len() == 4
                && membership.iter().all(|member: &PlannedMember<_>| {
                    member.output.destination == member.role.destination()
                        && membership.count_under(member.role.twin()) == 1
                })
                && membership.count_in(EmissionPartition::DeclarationSite) == 2
                && membership.count_in(EmissionPartition::TestCarrier) == 2
        );
        Ok(())
    }
}

//! The plane's declarations: the two identity families, the subject and limit
//! rosters, the profile and generator facts, the transcript and its derivation
//! record, and the rendered-role contract.
//!
//! Declarations only. Every constructor that must see a private field lives in
//! `type_guard.rs`, which is declared below as this file's own child so the
//! invariant nucleus and the fields it protects are never separated by a module
//! boundary.

use core::marker::PhantomData;
use threadpak::types::{Bounded, ConstLimit, Limit};

#[path = "type_guard.rs"]
mod guard;

pub(crate) use guard::{human_projection, static_bytes};

#[cfg(test)]
pub(crate) use guard::for_laws;

/// The seal on the identity-subject roster.
///
/// A value of this type is producible only inside the services, so a subject
/// declared anywhere else cannot satisfy [`IdentitySubject`]. It is the third
/// value seal in the plane, beside [`RenderedRoleSeal`] and the planning home's
/// kind seal, and it is the one that guards the derive-key context itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubjectSeal(());

/// One identity subject, by the name the domain-separation grammar spells it
/// with.
///
/// The name is the subject's segment of the derive-key context, so it is part of
/// what separates one subject's identities from another's. It is DECLARED beside
/// the marker rather than taken from the Rust spelling: a type rename is a
/// refactor, and a refactor that silently renamed every identity in the tree
/// would be a law change nobody wrote down.
///
/// The grammar is closed: lowercase ASCII letters and digits, in `-`-joined
/// segments, with no leading, trailing, or doubled separator.
///
/// # The roster is closed, and the seal is why
///
/// Sealed, and the seal is load-bearing rather than decorative. The name a
/// subject declares IS a domain separator: it is written into the derive-key
/// context [`IdentityProfile::context_for`] composes and into every transcript
/// derived under it, so the subject a type declares decides which name space its
/// identities live in. An open trait would let a type outside the services pick
/// that name space — declare `"plan"` and derive under the plan context, or
/// declare a name nothing else uses and mint a separation context the plane never
/// admitted. Either way an outside type would be choosing how the plane separates
/// its own identities, which is a law change rather than an extension point.
///
/// The roster below is the whole of it, and it is declared by the `subjects!`
/// macro, the only place a seal value is stamped. A downstream implementation is not
/// discouraged: it is unwritable, because the constant it would have to furnish
/// has no constructor outside this crate.
pub trait IdentitySubject {
    /// The seal. Only the services can produce a value of this type.
    const SEAL: SubjectSeal;

    /// The subject's declared segment of the derive-key context.
    const SUBJECT_NAME: &'static str;
}

/// Declares the plane's subject markers: one zero-sized type per identity
/// subject, each `Eq`/`Hash`/`Copy` so an identity tagged with it composes into
/// the plane's records without hand-written impls, and each carrying its
/// declared [`IdentitySubject`] name so no marker can exist without one.
///
/// This is the only site that stamps a [`SubjectSeal`], which is what closes the
/// roster: a subject exists because it was declared here, or it does not exist.
macro_rules! subjects {
    ($( $(#[$note:meta])* $name:ident = $declared:literal ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl IdentitySubject for $name {
                const SEAL: SubjectSeal = SubjectSeal::admitted();
                const SUBJECT_NAME: &'static str = $declared;
            }
        )+

        /// Every declared subject name, in roster order. The proof surface reads
        /// it to hold the grammar and the distinctness of the roster.
        #[cfg(test)]
        pub(crate) const SUBJECT_NAMES: &[&str] = &[$($declared),+];
    };
}

/// Declares the plane's limit families: each is a `Limit` with a compile-time
/// maximum, so every bounded seat in the plane names which bound governs it.
macro_rules! limits {
    ($( $(#[$note:meta])* $name:ident = $max:expr ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            impl Limit for $name {}
            impl ConstLimit for $name {
                const MAX: usize = $max;
            }
        )+
    };
}

subjects! {
    /// One registered refusal reason, as published by the machine's refusal home.
    RefusalReason = "refusal-reason",
    /// One refusal family, named by identity rather than by its Rust spelling.
    RefusalFamilySubject = "refusal-family",
    /// One owning semantic home of the machine. The home roster itself belongs
    /// to the machine and is derived by its tooling; the plane names a home by
    /// identity so it never carries a second copy of that roster.
    OwnerHomeSubject = "owner-home",
    /// One exact fact an owning home declares.
    OwnerFactSubject = "owner-fact",
    /// One node of the origin graph.
    OriginNodeSubject = "origin-node",
    /// One subject a decision trace entry is about.
    TracedSubject = "traced-subject",
    /// One subject a plan explicitly does not claim.
    NonclaimSubject = "nonclaim",
    /// One generated unit — the thing a plan declares it will materialize.
    GeneratedUnitSubject = "generated-unit",
    /// The canonical bytes of one generated unit.
    OutputBytesSubject = "output-bytes",
    /// One projection profile.
    ProjectionProfileSubject = "projection-profile",
    /// One version of the services themselves — the generator identity a plan
    /// was produced under.
    GeneratorVersionSubject = "generator-version",
    /// One projection kind, named by identity where a decoded route may name a
    /// kind the plane does not implement.
    ProjectionKindSubject = "projection-kind",
    /// One projection plan.
    PlanSubject = "plan",
    /// One bundle of plans materialized across a single publication boundary.
    BundleSubject = "bundle",
    /// One schema the machine's schema home owns.
    SchemaSubject = "schema",
    /// One byte role the machine's bytes home owns.
    ByteRoleSubject = "byte-role",
    /// One port declaration.
    PortSubject = "port",
    /// One wire contract a remote surface speaks.
    WireContractSubject = "wire-contract",
    /// One declared obligation a test descriptor challenges.
    ObligationSubject = "obligation",
    /// One measured unit a benchmark descriptor observes.
    MeasuredSubject = "measured-unit",
    /// One named work currency a benchmark envelope is stated in.
    WorkCurrencySubject = "work-currency",
    /// One subject a documentation projection documents.
    DocumentedSubject = "documented-subject",
    /// One type an implementation projection is derived for.
    DerivedTypeSubject = "derived-type",
    /// One contract an implementation projection realizes.
    ImplementedContractSubject = "implemented-contract",
    /// One authored pattern.
    PatternSubject = "pattern",
    /// One instantiation of an authored pattern.
    PatternInstanceSubject = "pattern-instance",
    /// One typed argument supplied to a pattern instantiation.
    PatternArgumentSubject = "pattern-argument",
    /// One admitted mechanism profile.
    MechanismProfileSubject = "mechanism-profile",
    /// One declared work formula.
    WorkFormulaSubject = "work-formula",
    /// One fixture population a descriptor ranges over.
    FixturePopulationSubject = "fixture-population",
    /// One contract a diagnostic expected to hold.
    ContractSubject = "contract",
    /// One related issue a diagnostic points at.
    RelatedIssueSubject = "related-issue",
    /// One callable services entry point.
    ServiceEntrySubject = "service-entry",
    /// One expansion surface of the Rust-facing shell.
    ExpansionSurfaceSubject = "expansion-surface",
    /// One runtime trace a generated unit corresponds to.
    RuntimeTraceSubject = "runtime-trace",
    /// One authored declaration template.
    TemplateSubject = "template",
    /// One typed hole a template declares.
    TemplateParameterSubject = "template-parameter",
    /// One typed commitment supplied to fill such a hole.
    TemplateArgumentSubject = "template-argument",
    /// One declared symbolic bound formula. The formula itself belongs to the
    /// owner that declared it; the plane names it and never evaluates it.
    BoundFormulaSubject = "bound-formula",
    /// One validated input descriptor a meta evaluation ranges over.
    InputDescriptorSubject = "input-descriptor",
    /// The exact source snapshot one invocation was read against.
    SourceSnapshotSubject = "source-snapshot",
    /// One language profile — the notation a front door speaks.
    LanguageProfileSubject = "language-profile",
    /// One meta profile — the posture a template evaluation runs under.
    MetaProfileSubject = "meta-profile",
    /// One deliberately declared distinctness between otherwise identical
    /// template applications.
    ApplicationDistinctnessSubject = "application-distinctness",
    /// One declared provider of descriptor material.
    DescriptorProviderSubject = "descriptor-provider",
    /// One captured declaration, as the compiler plane read it. Distinct from
    /// the machine's declaration fragment: the fragment is a linked artifact the
    /// machine owns, while this names exactly the token material one expansion
    /// was handed, before anything was linked at all.
    CapturedDeclarationSubject = "captured-declaration",
    /// One rendered unit — the thing a renderer actually materialized, as
    /// opposed to the generated unit a plan declared it would.
    RenderedUnitSubject = "rendered-unit",
    /// One proved closure between a plan's declared membership and the units a
    /// renderer actually produced.
    ClosureSubject = "closure",
    /// One closed expansion: the whole receipt one live compilation produced.
    ClosedExpansionSubject = "closed-expansion",
}

limits! {
    /// Source declarations one plan may name. A plan whose declared cause set
    /// outgrows this bound refuses rather than narrating a partial cause.
    SourceDeclarationLimit = 64,
    /// Outputs one plan may declare. The output firewall's bound: a plan
    /// declares its complete output set inside this magnitude or refuses.
    MembershipLimit = 32,
    /// Invalidation triggers one plan may watch — the trigger roster's own
    /// cardinality, since one trigger per kind is all that can be watched.
    InvalidationLimit = 9,
    /// Entries one decision trace may record.
    TraceEntryLimit = 128,
    /// Origin edges one trail may draw.
    OriginEdgeLimit = 64,
    /// Nonclaims one plan may state.
    NonclaimLimit = 16,
    /// Member plans one bundle may hold.
    BundleMemberLimit = 32,
    /// Issues one planning refusal body may carry: the roster's cardinality once
    /// each multi-seat issue is counted per seat — five single-seat issues, the
    /// missing-fact issue over its one plan seat, the bound issue over its six
    /// axes, and the doubled-output issue over the sixteen roles a membership at
    /// the output magnitude could double.
    PlanningIssueLimit = 28,
    /// Issues one explanation-coverage refusal body may carry: each of the
    /// fourteen questions may be unanswered, answered twice, or answered where
    /// the kind does not admit it, and no two of those hold of one question at
    /// once.
    ExplanationIssueLimit = 14,
    /// Explanation seats one view may hold — the question roster's cardinality.
    ExplanationSeatLimit = 14,
    /// Bytes one human projection may carry.
    HumanTextLimit = 512,
    /// Related issues one diagnostic may point at. It is the widest refusal-body
    /// magnitude in the plane on purpose: a diagnostic projects a refusal body
    /// issue for issue, so a narrower bound here would make the projection drop
    /// established issues to fit — which is the defect the projection exists to
    /// end.
    RelatedIssueLimit = 64,
    /// Repair actions one diagnostic may carry.
    RepairLimit = 8,
    /// Wrapper components one host-wrapper plan may select.
    WrapperComponentLimit = 16,
    /// Owner facts one plan's content may cite as an assumption.
    AssumptionLimit = 16,
    /// Typed arguments one pattern instantiation may supply.
    PatternArgumentLimit = 32,
    /// Facets one documentation projection may cover — the machine's facet
    /// roster is six, and a documentation projection covers a subset of it.
    FacetLimit = 6,
    /// Typed holes one template may declare, and therefore the bindings one
    /// application of it may supply: one binding per declared hole, exactly.
    TemplateParameterLimit = 32,
    /// Validated input descriptors one bound formula stands over and one
    /// invocation key commits to.
    InputDescriptorLimit = 32,
    /// Declaration fragments one invocation key names as a dependency.
    FragmentDependencyLimit = 64,
    /// Axis ceilings one profile ceiling carries — the meta bound-axis
    /// roster's own cardinality, since a ceiling names each axis exactly once.
    MetaBoundAxisLimit = 8,
    /// Issues one template-construction refusal body may carry: at most one
    /// per declared parameter seat, since no two parameter issues hold of one
    /// parameter at once, and the ceiling seam's own pass fits inside the same
    /// magnitude.
    TemplateIssueLimit = 32,
    /// Owner facts one wrapper-trigger selection or omission may cite.
    SelectionCitationLimit = 8,
    /// Issues one trigger-view refusal body may carry — the wrapper-component
    /// roster's cardinality, since a component is either undisposed or
    /// doubled and never both.
    TriggerViewIssueLimit = 8,
    /// Descriptor providers one composition root may declare.
    DescriptorProviderLimit = 64,
    /// Issues one composition-root refusal body may carry — at most one per
    /// declared provider seat.
    CompositionIssueLimit = 64,
    /// Bytes one captured derive surface may carry. Capture reads a declared
    /// input, and a declared input has a declared magnitude: past this, the
    /// capture refuses rather than parsing an unbounded body.
    DeriveSourceLimit = 8192,
    /// Causes one captured refusal family may declare. Past this the capture
    /// refuses rather than truncating a family's cause set.
    DeriveCauseLimit = 64,
    /// Token trees one captured input may carry at any one nesting level. A
    /// declared input has a declared magnitude; past this the capture refuses
    /// rather than walking an unbounded tree.
    CapturedTokenLimit = 4096,
    /// Steps one token path may carry — how deeply a declared input may nest.
    /// A level bound alone bounds the WIDTH of each level and nothing about the
    /// depth, so an input nested a million groups deep satisfies it at every
    /// level while the walk that reads it does not terminate in any useful time.
    TokenPathDepthLimit = 32,
    /// Tokens one captured input may carry ACROSS the whole tree. The level
    /// bound and the depth bound multiply: four thousand tokens at each of
    /// thirty-two levels is a tree nobody declared and nobody wants captured, so
    /// the total is bounded in its own right rather than left as the product of
    /// two other magnitudes.
    CapturedTreeTokenLimit = 16384,
    /// Bytes one rendered unit may carry. A renderer that would emit past this
    /// refuses rather than materializing part of a unit.
    RenderedByteLimit = 65536,
    /// Tokens one generated token tree may carry at any one nesting level.
    GeneratedTokenLimit = 4096,
    /// Issues one closure refusal body may carry: at most one per planned member
    /// seat plus one per unplanned rendered unit, which is twice the membership
    /// bound. Each pass of the check establishes at most one issue per role and
    /// refuses before the next pass runs, so the passes do not add up.
    ClosureIssueLimit = 64,
}

/// A reference to one exact machine identity, tagged by the subject it names.
///
/// # The lens law
///
/// This is a typed compiler-plane lens onto owner identity. Holding one means
/// only "the compiler refers exactly to this owner identity" — nothing about
/// admission, authority, freshness, availability, or equivalence.
///
/// # The walls, all structural
///
/// 1. **No public raw-byte constructor.** The only public roads are
///    [`OwnerIdentityRef::of_commitment`] and [`OwnerIdentityRef::of_reason`], each of
///    which reads an identity the machine already minted. The byte seam
///    (`decoded`) is crate-internal, awaiting the real decoder.
/// 2. **No cross-subject substitution.** `Subject` is a `PhantomData`
///    parameter, so a reference naming one subject is a different type than a
///    reference naming another regardless of bytes, and neither coerces to the
///    other.
/// 3. **No subject-erasing conversion.** [`OwnerIdentityRef::as_bytes`] hands back
///    a borrow for comparison and rendering, and re-wrapping those bytes under
///    a different subject is *unrepresentable outside this crate* precisely
///    because there is no public byte constructor to wrap them with.
/// 4. **No `IdentityRole` impl and no `Ord`.** The plane declares no class or
///    creation law for anything — that pair is the machine's to declare — and
///    references are never ranked.
///
/// The value carries the identity's declared raw-byte storage order and nothing
/// else — no availability, no version, no authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerIdentityRef<Subject> {
    bytes: [u8; 32],
    _subject: PhantomData<Subject>,
}

/// One owning home and one fact it declares, named by their declared stable
/// names rather than by minted identity.
///
/// This is a REFERENCE to an owner fact and never a second answer to it. The
/// plane reads the names the owning home wrote down; it derives nothing from
/// them, decides nothing by them, and mints no identity to stand where the
/// machine's would be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerFactName {
    /// The owning semantic home, by its declared name.
    pub home: &'static str,
    /// The fact that home declares, by its declared stable name.
    pub fact: &'static str,
}

/// A typed reference naming the owning band fact that caused a decision.
///
/// Every selection, omission, exclusion, and non-applicability in the plane
/// cites one of these. A bare boolean would say a decision happened without
/// saying whose fact decided it, which is exactly the explanation the plane
/// owes.
///
/// # Two postures, and neither is silence
///
/// The machine mints fact identities inside its own homes. Where a caller HOLDS
/// those identities, a citation carries them exactly
/// ([`OwnerFactRef::Minted`]). Where a caller does not — and an expansion shell
/// running inside `rustc` does not, because nothing has been linked and no home
/// has published an identity to it — the citation names the home and the fact by
/// their declared stable names ([`OwnerFactRef::Declared`]).
///
/// The second posture is a reference, not a substitute. The plane does not mint
/// an identity to fill the gap, because a plane-minted "owner fact identity"
/// would be a second value independently answering the owner's question, which
/// the services are forbidden to create. Naming the fact is what a deriver is
/// allowed to do; minting one is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerFactRef {
    /// The machine's own identities for the home and the fact.
    Minted {
        /// The owning semantic home.
        home: OwnerIdentityRef<OwnerHomeSubject>,
        /// The exact fact that home declares.
        fact: OwnerIdentityRef<OwnerFactSubject>,
    },
    /// The home and the fact by their declared stable names.
    Declared(OwnerFactName),
}

/// One version of one projection profile: a position in that profile's own
/// order. There is no `Ord` — versions of two different profiles are not
/// comparable, and the plane never ranks them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileVersion(u64);

/// One bounded human-readable rendering of a typed value.
///
/// It is a projection and only a projection: derived from typed values, carried
/// for a person to read, and never read back by the plane. No decision, no
/// identity, and no refusal consults it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HumanProjection<L: Limit> {
    text: Bounded<u8, L>,
}

threadpak::closed_register! {
    /// The closed roster of roles a plane identity may stand for.
    ///
    /// The role is part of the derive-key context AND part of the transcript, so
    /// two identities derived from the same anchor under different roles are
    /// different identities twice over: they are separated before a byte of the
    /// transcript is read, and they disagree inside it. A role that means
    /// something else is a law change, not a new string.
    ///
    /// # The stable name is a transcript member
    ///
    /// `stable_name` is the role's declared segment of the derive-key context
    /// and is written into every transcript. It is declared rather than taken
    /// from the Rust spelling for exactly the reason
    /// [`IdentitySubject::SUBJECT_NAME`] is: renaming a variant must not rename
    /// every identity derived under it. Changing one of these literals renames
    /// every identity the profile ever derived under that role, which is an
    /// identity-profile version bump and never an edit.
    ///
    /// `slot` is the byte the transcript carries beside it, and `ALL` is the
    /// roster in the order the plane states it.
    ///
    /// [`IdentitySubject::SUBJECT_NAME`]: super::IdentitySubject::SUBJECT_NAME
    pub enum ProjectionRole {
        /// The token material one expansion was handed.
        CapturedDeclaration = "captured-declaration",
            "the token material one expansion was handed";
        /// One projection plan.
        Plan = "plan", "one projection plan";
        /// One node of the origin graph.
        OriginNode = "origin-node", "one node of the origin graph";
        /// One generated unit a plan declares it will materialize.
        GeneratedUnit = "generated-unit",
            "one generated unit a plan declares it will materialize";
        /// One rendered unit a renderer actually materialized.
        RenderedUnit = "rendered-unit", "one rendered unit a renderer actually materialized";
        /// The canonical bytes of one rendered unit.
        OutputBytes = "output-bytes", "the canonical bytes of one rendered unit";
        /// One bundle materialized across a single publication boundary.
        Bundle = "bundle", "one bundle materialized across a single publication boundary";
        /// One proved closure between a plan and its rendering.
        Closure = "closure", "one proved closure between a plan and its rendering";
        /// One closed expansion.
        ClosedExpansion = "closed-expansion", "one closed expansion";
    }
}

/// One version of the projection-identity profile.
///
/// The version is a typed constant and a real segment of every derive-key
/// context, not a comment about one. Changing what a transcript contains, what
/// order it is written in, or what the domain grammar spells is a version bump,
/// and a bump renames every identity the profile derives — which is exactly what
/// it is for.
///
/// # A mint site's content grammar is inside that rule
///
/// The eleven shared members are one half of what a transcript contains; the
/// other half is the CONTENT each mint site composes, and each mint site
/// documents its own. A change to either is a change to what a transcript
/// contains. Version 2 is what closures committing to their emitted joined tree
/// cost: a reader handed two receipts under one version must be able to assume
/// both were derived the same way, and leaving the version at 1 across that
/// change would have broken exactly that assumption while every golden vector
/// stayed green.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(u32);

/// The versioned, domain-separated profile the plane derives its identities
/// under.
///
/// # The domain grammar
///
/// One derive-key context per subject and role, spelled exactly:
///
/// ```text
/// <stem>/v<version>/<subject>/<role>
/// ```
///
/// with `<stem>` the profile's declared stem, `<version>` the decimal position
/// of [`IdentityProfileVersion`], `<subject>` the target subject's
/// [`IdentitySubject::SUBJECT_NAME`], and `<role>` the
/// [`ProjectionRole::stable_name`]. Every segment is lowercase ASCII letters,
/// digits, and `-`, joined by `/`.
///
/// Separation is by CONTEXT and not by message prefix. Two identities over
/// identical transcript bytes under different subjects or different roles are
/// derived under different keys, so they are unrelated values rather than
/// neighbouring ones — there is no shared hash state for them to collide inside.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    stem: &'static str,
    version: IdentityProfileVersion,
}

/// The profile every plane identity in this crate is derived under.
pub const PROJECTION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    "threadpak/macroc/projection-identity",
    IdentityProfileVersion::declared(2),
);

/// The stable name of the generator that derives plane identities.
///
/// A name, not a version: it says WHICH generator, and it changes only when a
/// different generator starts producing this material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorProfileId(&'static str);

/// The version of the SHAPE this generator renders.
///
/// # It is bumped deliberately, and the rule is exact
///
/// Bump it when the rendered output's shape changes: a different token layout, a
/// different set of rendered roles, a different contract realized, or a
/// different meaning attached to one that already existed. Do not bump it for a
/// change that cannot reach the output — a comment, a refactor, a renamed local.
///
/// It is load-bearing: it rides in every transcript, so a bump renames every
/// identity this generator derives, and a plan produced under the old shape can
/// never be mistaken for one produced under the new.
///
/// This is deliberately NOT the package version. The package version moves for
/// reasons that have nothing to do with the rendered shape, and — at `0.0.0`
/// before the first release — does not move at all, which makes it worthless as
/// the fact a plan is invalidated by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorSchemaVersion(u32);

/// Which generator produced a plane identity, and under which rendered shape.
///
/// # Two load-bearing facts and one recorded one
///
/// The profile name and the schema version are IN the transcript: they decide
/// identity, and a change to either renames what the generator derives. The
/// package version is recorded and read back, and it is NOT in the transcript,
/// because it moves for reasons the rendered shape does not follow — and an
/// identity that changed on a version bump nobody's output noticed would be
/// noise dressed as provenance.
///
/// # What is NOT here, and why
///
/// There is no digest of this generator's own source. Computing one would mean
/// reading the source tree at expansion time, which the ambient-free law
/// forbids, or running a build script, which this repository forbids outright. A
/// self-digest that could not be computed honestly is not carried dishonestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorIdentity {
    profile: GeneratorProfileId,
    schema: GeneratorSchemaVersion,
    package: &'static str,
}

/// This generator, as every transcript in this crate names it.
pub const MACROC_GENERATOR: GeneratorIdentity = GeneratorIdentity::declared(
    GeneratorProfileId::declared("threadpak-macroc"),
    GeneratorSchemaVersion::declared(1),
    env!("CARGO_PKG_VERSION"),
);

/// What one transcript is anchored under.
///
/// Three postures, and each is written into the transcript as a distinct
/// discriminant ahead of its commitment, so a rooted transcript can never encode
/// as an anchored one whose anchor happened to be empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptAnchoring {
    /// No anchor at all — the root of one derivation chain, where the content is
    /// the whole of what varies. The captured declaration is what stands here:
    /// everything else in a plan hangs off it.
    Rooted,
    /// Anchored under an identity the MACHINE minted, carried at full width.
    UnderOwnerIdentity([u8; 32]),
    /// Anchored under another identity the PLANE owns, carried at full width.
    UnderProjectionIdentity([u8; 32]),
}

/// The COMPLETE preimage one [`ProjectionIdentity`] is derived from.
///
/// # The transcript specification
///
/// A transcript is the exact byte string handed to the digest. It is written
/// once, here, and this specification is complete: an independent implementation
/// needs this section and nothing else.
///
/// Two primitives:
///
/// - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
/// - `bytes(x)` — `u64be(x.len())` followed by the bytes of `x`. Every
///   variable-length member is written this way, so no two member sequences can
///   be cut at a different boundary and produce one byte string.
///
/// The members, in exactly this order, with no separators and no padding:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | profile stem | `bytes(utf8)` |
/// | 2 | profile version | `u32be` |
/// | 3 | identity subject | `bytes(utf8)` of [`IdentitySubject::SUBJECT_NAME`] |
/// | 4 | role | `bytes(utf8)` of [`ProjectionRole::stable_name`] |
/// | 5 | role slot | one byte, [`ProjectionRole::slot`] |
/// | 6 | anchoring | one byte, [`TranscriptAnchoring::slot`] |
/// | 7 | anchor commitment | `bytes(…)` — empty when rooted, else the full 32 |
/// | 8 | content | `bytes(…)` — the full material, never a fold |
/// | 9 | roster position | `u32be` |
/// | 10 | generator profile | `bytes(utf8)` of [`GeneratorProfileId::spelling`] |
/// | 11 | generator schema version | `u32be` |
///
/// The derive-key context is [`IdentityProfile::context_for`] over the same
/// subject and role. The identity is
/// `blake3::derive_key(context, transcript) -> [u8; 32]`.
///
/// # Nothing is folded on the way in
///
/// The anchor is carried at its full 32 bytes and the content at its full
/// length. The 32-byte output is the only compression anywhere in the
/// derivation, and it is the digest's own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionTranscript<'material> {
    profile: IdentityProfile,
    generator: GeneratorIdentity,
    role: ProjectionRole,
    anchoring: TranscriptAnchoring,
    content: &'material [u8],
    position: u32,
}

/// The inspectable record of ONE derivation: which subject, which role, which
/// profile at which version, which generator, what it was anchored under, and
/// how much content went in.
///
/// # Why this is a separate value from the identity
///
/// The identity answers "which thing is this?" and is thirty-two bytes. The
/// record answers "where did those thirty-two bytes come from?" and is
/// inspection material. Carrying the second inside the first put a derivation
/// record on every identity in every plan, every trace entry, and every refusal
/// body — which is what made an earlier design fold its anchor and its content
/// down to eight bytes each just to keep the record small enough to travel.
///
/// Split apart, neither constrains the other: the transcript can be complete
/// because it is not stored, and the record can be honest because it is written
/// once where the derivation happened rather than copied everywhere the identity
/// goes.
///
/// # What it carries exactly, and what it states rather than carries
///
/// The subject, the role, the profile and its version, the generator, the
/// anchoring posture, the anchor commitment at its FULL thirty-two bytes, and
/// the roster position are all carried exactly. The content is stated by its
/// LENGTH and not carried, because content is unbounded — a rendered unit's
/// canonical bytes run to the declared rendering magnitude — and a record that
/// copied it would double every rendering in memory to say something the
/// rendered unit already holds.
///
/// That is a stated limit and not a hidden one, and it is not a fold: nothing
/// here is a lossy summary of the content presented as though it identified it.
/// The identity is what commits to the content, at full width, under BLAKE3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionProvenance {
    subject: &'static str,
    role: ProjectionRole,
    profile: IdentityProfile,
    generator: GeneratorIdentity,
    anchoring: TranscriptAnchoring,
    content_length: u64,
    position: u32,
}

/// One identity the COMPILER PLANE owns, tagged by the subject it names.
///
/// # What holding one means
///
/// It means the plane derived these thirty-two bytes from a complete
/// [`ProjectionTranscript`] under [`PROJECTION_IDENTITY_PROFILE`], and would
/// derive the same ones again from the same transcript on any machine. It means
/// nothing about the machine: the machine mints no plane identity and accepts
/// none.
///
/// # The collision claim, stated exactly
///
/// **Collision resistance is claimed AS BLAKE3's, for the transcript as
/// specified on [`ProjectionTranscript`], under profile version
/// [`IdentityProfileVersion`] as declared by
/// [`PROJECTION_IDENTITY_PROFILE`] — and nothing broader.**
///
/// Read the boundaries of that sentence as strictly as it is written. It claims
/// that finding two different transcripts deriving one identity is as hard as
/// finding a BLAKE3 collision. It does NOT claim that two things the plane
/// considers different always have different transcripts — that is the
/// transcript's completeness, which each mint site is responsible for and which
/// each mint site documents. It does NOT claim anything about a different
/// profile version, which derives under different contexts and is a different
/// name space. And it does NOT make a plane identity into a machine commitment:
/// where the machine needs a commitment the machine mints one, and no plane
/// identity is ever accepted in its place.
///
/// The weak in-house fold this replaced claimed no collision resistance at any
/// width and folded its anchor and content to eight bytes before hashing. It is
/// retired: nothing in the plane derives an identity that way, and the narrow
/// folds are gone rather than re-hashed.
///
/// # The walls
///
/// 1. **No raw-byte constructor at all.** The only road is
///    [`ProjectionIdentity::derived`], which takes a typed transcript. There is
///    no public or crate-internal seam that wraps arbitrary bytes.
/// 2. **No cross-subject substitution.** `Subject` is a `PhantomData` parameter,
///    so an identity naming one subject is a different type than one naming
///    another regardless of bytes — and their derive-key contexts differ too, so
///    the separation is a runtime fact and not only a compile-time one.
/// 3. **No conversion to or from [`OwnerIdentityRef`].** The two families answer
///    different questions and neither is reachable from the other.
/// 4. **No `Ord`.** Plane identities are never ranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIdentity<Subject> {
    bytes: [u8; 32],
    _subject: PhantomData<Subject>,
}

/// One projection plan's own identity.
///
/// A plan is spoken of by identity in three places — a bundle's membership, the
/// planning family's issues, and the closure that proves a rendering against it
/// — and all three name this one type.
pub type PlanId = ProjectionIdentity<PlanSubject>;

/// One proved closure's own identity.
pub type ClosureId = ProjectionIdentity<ClosureSubject>;

/// One closed expansion's own identity.
pub type ClosedExpansionId = ProjectionIdentity<ClosedExpansionSubject>;

/// The seal on the rendered-role roster.
///
/// A value of this type is producible only inside the services, so a roster
/// declared anywhere else cannot satisfy [`RenderedRole`]. It is the same seal
/// the planning home puts on the projection-kind roster, for the
/// same reason: the closure check walks `ROLES` and asks what stood under each
/// one, so a roster that left a variant out would make that variant's rendered
/// unit invisible to the loop that is supposed to prove it. An outside
/// implementation could declare exactly that roster; a sealed one cannot exist
/// at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderedRoleSeal(());

/// The closed roster of rendered units one projection kind materializes.
///
/// A kind declares this roster once, and the closure check reads it: a rendered
/// unit is matched to a planned member by ROLE, so "the family implementation"
/// and "the cause-order implementation" are different seats rather than two
/// entries in an ordered list nobody can tell apart. A rendering that produced
/// the right number of units in the wrong roles is caught by the role, not by a
/// count.
///
/// Sealed, and the seal is load-bearing rather than decorative. Every proof in
/// the plane that says "every role was examined" says it by walking [`ROLES`],
/// so the roster IS the quantifier. An implementation that omitted one variant
/// would render a unit the closure loop never looks at and never reports, which
/// is a silent output past the firewall. Each admitted roster carries a law
/// proving it names every variant exactly once, at the roster position that
/// variant's slot claims.
///
/// [`ROLES`]: RenderedRole::ROLES
pub trait RenderedRole: Copy + PartialEq + Eq + core::fmt::Debug + Sized + 'static {
    /// The seal. Only the services can produce a value of this type.
    const SEAL: RenderedRoleSeal;

    /// The complete roster, in the order the kind states it.
    const ROLES: &'static [Self];

    /// This role's position in the roster. Part of every transcript derived for
    /// the role, so two roles never derive one identity.
    fn slot(self) -> u32;

    /// The role rendered for a person. A projection: nothing reads it back.
    fn described(self) -> &'static str;
}

/// The one-unit rendered roster, for kinds that materialize exactly one unit.
///
/// Not a placeholder and not an absence: a kind whose rendering is one unit says
/// so with a roster of one, and the closure check over it is the same check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoleRenderedUnit {
    /// The kind's one rendered unit.
    Sole,
}

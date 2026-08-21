//! The plane's declarations: the two identity families, the subject roster, the
//! magnitude stamp and the plane's own magnitude rows, the preimage-family
//! roster with one profile constant per family, the generator facts, the
//! transcript and its derivation record, and the rendered-role contract.
//!
//! Declarations only.
//! Every constructor that must see a private field lives in `type_guard.rs`,
//! declared below as this file's own child so the invariant nucleus and the
//! fields it protects are never separated by a module boundary.
//!
//! # The stamp is published, and the rows are not a central register
//!
//! `limits!` is the plane's own road and is invoked from any home in the crate.
//! The rows below it are not everybody's bounds gathered in one place: a
//! magnitude belongs here when more than ONE home asks its question, and is
//! declared in a semantic home — through this same stamp — when only that home
//! asks it.

use core::marker::PhantomData;
use threadpak::types::{Bounded, Limit, LimitAdmissionProfile};

#[path = "type_guard.rs"]
mod guard;

pub(crate) use guard::{human_projection, names_are_separating, static_bytes};

/// The seal on the identity-subject roster.
///
/// A value of this type is producible only inside the services, so a subject
/// declared anywhere else cannot satisfy [`IdentitySubject`] — which is what
/// keeps the derive-key context under this crate's own authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubjectSeal(());

/// One identity subject, by the name the domain-separation grammar spells it
/// with.
///
/// The name is the subject's segment of the derive-key context, so it is part of
/// what separates one subject's identities from another's.
/// It is DECLARED beside the marker rather than taken from the Rust spelling: a
/// type rename is a refactor, and a refactor that silently renamed every
/// identity in the tree would be a law change nobody wrote down.
///
/// The grammar is closed: lowercase ASCII letters and digits, in `-`-joined
/// segments, with no leading, trailing, or doubled separator.
///
/// # Authority
///
/// The name a subject declares IS a domain separator: it is written into the
/// derive-key context [`IdentityProfile::context_for`] composes and into every
/// transcript derived under it, so the subject a type declares decides which
/// name space its identities live in.
/// An open trait would let a type outside the services pick that name space —
/// declare `"plan"` and derive under the plan context, or declare a name
/// nothing else uses and mint a separation context the plane never admitted.
/// Either way an outside type would be choosing how the plane separates its own
/// identities, which is a law change rather than an extension point.
/// The `subjects!` macro below is the whole roster and the only place a
/// [`SubjectSeal`] is stamped, so an outside implementation is unwritable
/// rather than discouraged.
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
/// A subject exists because it was declared here, or it does not exist.
///
/// # The roster separates, or it does not compile
///
/// A declared name IS a derive-key domain separator, so the two things that
/// would break the separation are settled in the expansion rather than checked
/// afterwards: a name outside the closed grammar makes the context unreadable,
/// and a name two subjects declare gives two semantic LEVELS one name space —
/// the exact collision [`RelatedBodySubject`] was split out of
/// [`RelatedIssueSubject`] to close. The `const` item below asks both questions
/// of the whole roster at compile time, so neither is a mistake this file can
/// hold.
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

        const _: () = ::core::assert!(
            $crate::plane::names_are_separating(&[$($declared),+]),
            "a subject name outside the derive-key context grammar, or one two subjects declare",
        );
    };
}

/// The AUTHORING plane's admissible ceiling: the widest magnitude any limit
/// family the services declare may state.
///
/// The machine owns the admission algebra — which witnesses exist and what each
/// establishes — and deliberately declares no production ceiling, because a
/// number seated there for convenience becomes the ceiling every plane inherits
/// without deciding anything.
/// This is where the services decide theirs, beside the stamp every family they
/// declare is stamped by.
///
/// # Bounds
///
/// `1_048_576`, a number this plane CHOSE rather than one taken from a machine
/// width: it leaves room far above the widest magnitude the services declare
/// and stops well short of a number that has stopped meaning anything.
/// What it rules out is a "bound" that bounds nothing: a magnitude no declared
/// input could reach makes its checked constructor unfalsifiable, and a
/// constructor that cannot refuse is not a checked constructor.
/// Every bounded construction in the services is admitted against it — the
/// plane's own rows and every home's alike — so moving it moves what the whole
/// plane will accept.
///
/// # Nonclaims
///
/// A family admitted here is admitted nowhere else: the witness carries the
/// admitting profile as a type parameter, so this admission never stands in for
/// a qualification profile's ceiling or a host's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthoringLimitProfile;

impl LimitAdmissionProfile for AuthoringLimitProfile {
    const MAX_DECLARED_LIMIT: usize = 1_048_576;
}

/// The magnitude stamp: the one mechanism that turns a magnitude ROW into every
/// form its readers hold it in, wherever the row is declared.
///
/// # The mechanism and the meaning are two different ownerships
///
/// **This stamp is the plane's, and every row it stamps belongs to the home that
/// owns the question the row answers.** What a magnitude MEANS — which capacity
/// it governs, what number it states, and why that number and not another — is
/// the semantic home's declaration, and it is written in that home beside the
/// capacity it bounds. What a magnitude has to BECOME to be usable — a capacity
/// authority, a compile-time ladder, and the two integer widths its readers hold
/// it at — is a mechanical fact that is the same for every row in the services,
/// and a home that spelled it out again would be a second copy of one mechanism
/// rather than a second opinion about anything.
///
/// So the stamp is published in-crate and INVOKED LOCALLY: a home writes
/// `crate::plane::limits! { … }` in its own `types.rs`, its rows stay in the
/// home that owns them, and nothing about the emission is written twice.
///
/// # What one row emits
///
/// The authority and the magnitude come out of ONE row, in one expansion, so a
/// family cannot be declared on the compile-time ladder while wearing another
/// road's authority: the transcriber writes `DeclaredMagnitude` and `ConstLimit`
/// together or writes neither.
///
/// The magnitude is emitted at BOTH widths its readers hold it in — a
/// collection's on the ladder, a counter's beside it — from that same row, for
/// the reason stated on the counter-width constant itself.
///
/// Every path in the expansion is absolute, because the expansion lands in
/// whatever module invoked it and a relative path would make the stamp depend on
/// what that module happened to import.
///
/// # One form
///
/// There is one form, and it stamps rows. A reader with a question about a row
/// SET rather than about one family would need a second emission reading the
/// rows back, and no reader in the services has one: what a magnitude admits is
/// asked of the family that declares it, through its own `ConstLimit`, at the
/// seam that is bounded by it.
macro_rules! limits {
    ( $( $(#[$note:meta])* $name:ident = $max:expr ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            impl ::threadpak::types::Limit for $name {
                type Authority = ::threadpak::types::DeclaredMagnitude;
            }
            impl ::threadpak::types::ConstLimit for $name {
                const MAX: usize = $max;
            }

            impl $name {
                /// The same declared magnitude at the width a COUNTER holds it.
                ///
                /// [`ConstLimit::MAX`] is a collection's width, and a seat that
                /// counts rather than collects — a work budget, a token tally —
                /// holds thirty-two bits. Nothing converts the one into the
                /// other where a magnitude is read: no narrowing conversion is
                /// callable in a `const`, and the as-conversion road is barred
                /// outright, so a counter reading the ladder would have to
                /// narrow at RUNTIME and carry a refusal branch for a case its
                /// own declaration rules out.
                ///
                /// The magnitude is therefore stated once and emitted twice,
                /// from the SAME row in the SAME expansion: the two widths are
                /// one number read two ways and cannot drift, and a row past
                /// thirty-two bits stops the compiler here rather than
                /// narrowing anything silently.
                ///
                /// [`ConstLimit::MAX`]: threadpak::types::ConstLimit::MAX
                pub const MAX_U32: u32 = $max;
            }
        )+
    };
}

pub(crate) use limits;

subjects! {
    /// One registered refusal reason, as published by the machine's refusal home.
    RefusalReason = "refusal-reason",
    /// One refusal family, named by identity rather than by its Rust spelling.
    RefusalFamilySubject = "refusal-family",
    /// One owning semantic home of the machine, named by identity so the plane
    /// never carries a second copy of the machine's home roster.
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
    /// The whole refusal body one diagnostic's related set commits to, as
    /// opposed to any single issue inside it.
    ///
    /// A separate subject from [`RelatedIssueSubject`] because the two are
    /// separate semantic LEVELS over one material, and one name space holding
    /// two levels collides by construction: a body's preimage is the framing of
    /// its issues, so an issue whose own material happened to be that framing
    /// would derive the identity of the body it aliased.
    /// Two subjects give the two levels two derive-key contexts, so identical
    /// preimage bytes at different levels are unrelated values.
    RelatedBodySubject = "related-body",
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
    /// One closed expansion: the whole account one live compilation produced.
    ClosedExpansionSubject = "closed-expansion",
    /// One projection intent — WHAT a door meant, ahead of anything decided
    /// about it: the kind and the owner content commitment it was meant over.
    ///
    /// A separate subject from [`PlanSubject`] because the two are separate
    /// semantic LEVELS over one material, on the same terms
    /// [`RelatedBodySubject`] states.
    /// A plan's identity commits to everything the plan decided, its origin
    /// included, so two doors that meant the same thing are REQUIRED to derive
    /// different plan identities; the intent level is the one that is allowed to
    /// agree, and one name space holding both levels would let an intent's
    /// preimage alias a plan's.
    ProjectionIntentSubject = "projection-intent",
    /// One explanation: the answers one projection wrote over the plan it was
    /// planned from and the closure that proved its rendering.
    ///
    /// A separate subject from [`ClosedExpansionSubject`] because the two name
    /// different values: an explanation is one of the three things a closed
    /// expansion binds, and a terminal that carried its explanation's name under
    /// its own subject would give one expansion two identities in one name
    /// space.
    ExplanationSubject = "explanation",
}

limits! {
    /// Outputs one plan may declare. The output firewall's bound: a plan
    /// declares its complete output set inside this magnitude or refuses.
    MembershipLimit = 32,
    /// Entries one decision trace may record.
    TraceEntryLimit = 128,
    /// Nonclaims one plan may state.
    NonclaimLimit = 16,
    /// Bytes one human projection may carry.
    HumanTextLimit = 512,
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
    /// Token trees one captured input may carry at any one nesting level. A
    /// declared input has a declared magnitude; past this the capture refuses
    /// rather than walking an unbounded tree.
    CapturedTokenLimit = 4096,
    /// Bytes one rendered unit may carry. A renderer that would emit past this
    /// refuses rather than materializing part of a unit.
    RenderedByteLimit = 65536,
    /// Tokens one generated token tree may carry at any one nesting level.
    GeneratedTokenLimit = 4096,
}

/// A reference to one exact machine identity, tagged by the subject it names.
///
/// The machine's identity home mints; this is the compiler plane's typed lens
/// onto what it minted, carrying the identity's declared raw-byte storage order
/// and nothing else — no availability, no version, no authority.
///
/// # Construction
///
/// The public roads are [`OwnerIdentityRef::of_commitment`] and
/// [`OwnerIdentityRef::of_reason`], each reading an identity the machine already
/// minted; there is no public raw-byte constructor.
/// `Subject` is a `PhantomData` parameter, so a reference naming one subject is
/// a different type than a reference naming another regardless of bytes, and
/// neither coerces to the other.
/// [`OwnerIdentityRef::as_bytes`] hands back a borrow for comparison and
/// rendering, and re-wrapping those bytes under a different subject is
/// unrepresentable outside this crate precisely because there is no public byte
/// constructor to wrap them with.
/// There is no `IdentityRole` impl and no `Ord`: the plane declares no class or
/// creation law for anything, and references are never ranked.
///
/// # Nonclaims
///
/// Holding one means only that the compiler refers exactly to this owner
/// identity — nothing about admission, authority, freshness, availability, or
/// equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerIdentityRef<Subject> {
    bytes: [u8; 32],
    _subject: PhantomData<Subject>,
}

/// One owning home and one fact it declares, named by their declared stable
/// names rather than by minted identity.
///
/// The plane reads the names the owning home wrote down; it derives nothing
/// from them and mints no identity to stand where the machine's would be.
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
/// cites one of these.
/// A bare boolean would say a decision happened without saying whose fact
/// decided it, which is exactly the explanation the plane owes.
///
/// Where a caller HOLDS the machine's minted fact identities, a citation carries
/// them exactly ([`OwnerFactRef::Minted`]).
/// Where a caller does not — and an expansion shell running inside `rustc` does
/// not, because nothing has been linked and no home has published an identity to
/// it — the citation names the home and the fact by their declared stable names
/// ([`OwnerFactRef::Declared`]).
/// The second posture is a reference and not a substitute: a plane-minted
/// "owner fact identity" would be a second value independently answering the
/// owner's question, which the services are forbidden to create.
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
/// order.
/// There is no `Ord` — versions of two different profiles are not comparable,
/// and the plane never ranks them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileVersion(u64);

/// One bounded human-readable rendering of a typed value.
///
/// It is a projection and only a projection: derived from typed values, carried
/// for a person to read, and never read back.
/// No decision, no identity, and no refusal in the plane consults it.
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
    /// transcript is read, and they disagree inside it.
    /// Giving a role a different meaning is a law change, not a new string.
    ///
    /// # Authority
    ///
    /// A role's stable name is its declared segment of the derive-key context
    /// and is written into every transcript, declared rather than taken from the
    /// Rust spelling for exactly the reason
    /// [`IdentitySubject::SUBJECT_NAME`] is.
    /// Changing one of these literals renames every identity the profile ever
    /// derived under that role, which is an identity-profile version bump and
    /// never an edit.
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
        /// One projection intent — what a door meant, ahead of what it decided.
        ProjectionIntent = "projection-intent",
            "one projection intent, ahead of anything decided about it";
        /// One explanation, answered over a plan and the closure that proved its
        /// rendering.
        Explanation = "explanation",
            "one explanation answered over a plan and its closure";
        /// The documentation one captured declaration carries, read as a second
        /// fact over the surface its semantic commitment already names.
        DeclarationDocumentation = "declaration-documentation",
            "the documentation rows one captured declaration carries";
        /// One declared stable name this crate wrote down, standing for a value
        /// the crate itself declares.
        DeclaredName = "declared-name",
            "one declared stable name this crate wrote down";
        /// The generator's own declared name and the shape version it renders.
        GeneratorVersion = "generator-version",
            "the generator's declared name and its schema position";
        /// One refusal body, or one issue inside it, as a diagnostic points at
        /// it.
        DiagnosticRelation = "diagnostic-relation",
            "one refusal body or one established issue a diagnostic points at";
    }
}

threadpak::closed_register! {
    /// The closed roster of PREIMAGE FAMILIES the plane derives identities
    /// under.
    ///
    /// A family is one canonical preimage GRAMMAR: which members a mint site
    /// writes, in what order, carrying what material. Every family declares its
    /// own profile version ([`PreimageFamily::profile`]), and that version
    /// moves when THAT family's grammar moves and at no other time.
    ///
    /// # Authority
    ///
    /// **One version per grammar, because a version is what renames.** A single
    /// position shared by every family renames every identity in the tree the
    /// moment any one grammar widens — a rendering-shape change renames the
    /// intent identity whose meaning did not move, and the equivalence a door
    /// compares stops answering the question it was asked. Splitting the
    /// position is what makes a bump say exactly what it moved, and it is what
    /// makes the promise on [`IdentityProfileVersion`] — two identities under
    /// one position were derived the same way — a promise about one grammar
    /// rather than about every grammar at once.
    ///
    /// A family's declared name is its segment of the derive-key context
    /// ([`IdentityProfile::context_for`]) and a member of every transcript
    /// written under it, so it is DECLARED here rather than taken from the Rust
    /// spelling, for the reason [`IdentitySubject::SUBJECT_NAME`] is.
    ///
    /// # Bounds
    ///
    /// A family exists because a preimage grammar is genuinely its own, never
    /// because a type is. Two roles standing over ONE grammar share one family
    /// and are separated inside it by the role, which is a member of the
    /// transcript and a segment of the context: a rendered unit and the bytes
    /// it carries stand over the same rendered material, so
    /// [`ProjectionRole::RenderedUnit`] and [`ProjectionRole::OutputBytes`]
    /// both read to [`PreimageFamily::RenderedUnit`].
    ///
    /// # The five positions the retired single version moved through
    ///
    /// The plane ran ONE position for every family until this roster existed,
    /// and it moved five times. The archaeology is kept because each move was a
    /// real decision about a real grammar; every family's own `Versions` prose
    /// states which of these would have moved IT and which would not.
    ///
    /// - **1** — the single profile as first declared.
    /// - **2** — the closure transcript grew the joined-tree digest, so a
    ///   closure committed to the tree it proved rather than to the units it
    ///   counted.
    /// - **3** — the entry account entered the plan transcript's content: a
    ///   plan is derived over the ONE account of the owner content it was
    ///   planned over — the commitment and the dependency set it declares it
    ///   stands on — where the content it stood on had been named nowhere in
    ///   its own preimage.
    /// - **4** — the generated-token roster grew the byte-string and numeric
    ///   literal arms, so the grammar a reader must hold to read a generated
    ///   tree's canonical bytes widened.
    /// - **5** — two grammars widened at once: the member-delivery encoding
    ///   grew the two carrier slots, and the closure transcript's emission
    ///   member became the partitioned-emission encoding where it had been one
    ///   joined-tree digest.
    ///
    /// Positions 2 through 5 each moved ONE or TWO grammars and renamed every
    /// identity in the tree. That is the arithmetic this roster ends.
    pub enum PreimageFamily {
        /// The canonical tree encoding of the token material one expansion was
        /// handed.
        CapturedDeclaration = "captured-declaration",
            "the canonical bytes of one captured declaration";
        /// What a door MEANT — the projection kind and the owner content
        /// commitment — and nothing about the machinery that realizes it.
        ProjectionIntent = "projection-intent",
            "one projection kind over one owner content commitment";
        /// The account a plan was planned over and everything the plan decided
        /// beside it.
        Plan = "plan", "one plan's account, context, and decided seats";
        /// The declared material one origin node stands at.
        OriginNode = "origin-node", "one origin node's declared material";
        /// The declared material one planned member answers to, ahead of any
        /// bytes.
        GeneratedUnit = "generated-unit", "one planned member's declared material";
        /// The exact bytes a renderer materialized, under the semantic key they
        /// answer to.
        RenderedUnit = "rendered-unit",
            "the exact canonical bytes of one rendered unit";
        /// The member plans one bundle names.
        Bundle = "bundle", "the member plans one bundle names";
        /// A plan's identity, the membership rebuilt from a rendering, and the
        /// digests of what each emission carries.
        Closure = "closure",
            "one closure's plan, reconstructed membership, and partition digests";
        /// The seats one explanation answers over a plan and its closure.
        Explanation = "explanation", "one explanation's answered seats";
        /// The whole account one live compilation produced.
        ClosedExpansion = "closed-expansion",
            "one closed expansion's plan and explanation, under its anchoring closure";
        /// The documentation rows one captured declaration carries, over the
        /// semantic commitment they were cut from.
        DeclarationDocumentation = "declaration-documentation",
            "one captured declaration's ordered documentation rows over its semantic commitment";
        /// One declared stable name, as this crate wrote it down.
        DeclaredName = "declared-name", "one declared stable name, as this crate wrote it";
        /// The generator's two load-bearing facts, framed.
        GeneratorVersion = "generator-version",
            "one generator's declared name and schema position";
        /// One refusal family's tag and the framed material a diagnostic points
        /// at under it.
        DiagnosticRelation = "diagnostic-relation",
            "one refusal family's tag and the framed material it points at";
    }
}

/// The one stem every family's derive-key context opens with.
///
/// Shared rather than chosen per family, and the family segment carries the
/// separation instead. A stem a family picked for itself would let two families
/// be declared into one name space by a literal nobody compared; a segment
/// taken from the closed [`PreimageFamily`] roster cannot be, because no two
/// rows of that roster carry one name.
pub const IDENTITY_PROFILE_STEM: &str = "threadpak/macroc/projection-identity";

/// One version of ONE preimage family's profile: a position in that family's
/// own order.
///
/// The version is a typed constant and a real segment of every derive-key
/// context, not a comment about one.
/// Changing what a transcript under a family contains, what order it is written
/// in, or what the domain grammar spells is a bump OF THAT FAMILY, and a bump
/// renames every identity the family derives — which is exactly what it is for.
///
/// The members [`ProjectionTranscript`] specifies are one half of what a
/// transcript contains; the other half is the CONTENT each mint site composes
/// and documents.
/// A change to either is a change to what a transcript contains, so a reader
/// handed two identities of one family under one position may assume both were
/// derived the same way.
///
/// A position belongs to one family and to no other. Two families at position
/// one are two key spaces rather than one reached twice, and a bump under
/// either renames nothing under the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(u32);

/// The versioned, domain-separated profile ONE preimage family derives under.
///
/// One derive-key context per family, subject, and role, spelled exactly:
///
/// ```text
/// <stem>/<family>/v<version>/<subject>/<role>
/// ```
///
/// with `<stem>` [`IDENTITY_PROFILE_STEM`], `<family>` the family's declared
/// [`PreimageFamily::stable_name`], `<version>` the decimal position of THAT
/// family's [`IdentityProfileVersion`], `<subject>` the target subject's
/// [`IdentitySubject::SUBJECT_NAME`], and `<role>` the
/// [`ProjectionRole::stable_name`].
/// Every segment is lowercase ASCII letters, digits, and `-`, joined by `/`.
///
/// # Authority
///
/// **Two families can never share a derivation namespace.** The family segment
/// sits AHEAD of the version, so position one of one family and position one of
/// another are different key spaces rather than the same space reached twice,
/// and the segment is a row of a closed roster where no two rows carry one
/// name. A bump therefore moves exactly one family's key space, and every
/// identity outside that family keeps the name it had.
///
/// Separation is by CONTEXT and not by message prefix.
/// Two identities over identical transcript bytes under different families,
/// subjects, or roles are derived under different keys, so they are unrelated
/// values rather than neighbouring ones — there is no shared hash state for them
/// to collide inside.
/// The family, the version, the subject, and the role are members of the
/// transcript as well, so the separation is stated twice and a reader holding
/// the bytes can see it rather than having to be told.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    family: PreimageFamily,
    version: IdentityProfileVersion,
}

/// The profile one captured declaration's SEMANTIC commitment is derived under.
///
/// # Preimage
///
/// The captured token tree's own canonical encoding, rooted, with every
/// documentation attribute dropped from the walk: a capture is the root of a
/// derivation chain, and the material IS the whole of what varies.
///
/// **Documentation is captured and is committed to under its own family.** A
/// `#[doc = "…"]` attribute is declaration material like any other token, and it
/// is exactly the material whose meaning is a second reading rather than the
/// declaration's own semantics — so it is dropped here and enters
/// [`DECLARATION_DOCUMENTATION_IDENTITY_PROFILE`] instead. One captured surface,
/// two authored facts, and neither is a fold of the other: a declaration whose
/// prose changed keeps its semantic name and takes a new documentation name,
/// which is what lets an implementation projection and a documentation
/// projection stand on the fact each of them is actually about.
///
/// Spans enter neither. A handle is the producer's own table index, two
/// producers reading one declaration issue different ones, and the diagnostic
/// rail is where a handle belongs.
///
/// # Versions
///
/// - **1** — the family as first declared. Of the five positions the retired
///   single version moved through ([`PreimageFamily`]), none touched this
///   grammar: position 4 widened the GENERATED token roster, which is the
///   rendered side, and the captured table stood at the five rows it was first
///   declared with.
/// - **2** — the captured token roster gained the literal forms it had been
///   answering with a neighbour's row. A byte string, a raw text, a character,
///   and a byte were every one of them lawful declaration material before, and
///   every one of them was encoded under the NUMERIC row carrying its own
///   spelling as the framed content; a text carrying an escape was encoded with
///   the escape's characters rather than the value they name. Those are
///   declarations that already existed, so their content moves and the names
///   derived over them move with it — which is the case this position is bumped
///   for, and the reason the appended rows on the GENERATED side did not bump
///   theirs.
pub const CAPTURED_DECLARATION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::CapturedDeclaration,
    IdentityProfileVersion::declared(2),
);

/// The profile one captured declaration's DOCUMENTATION commitment is derived
/// under.
///
/// # Preimage
///
/// The SEMANTIC commitment at the anchor, at its full thirty-two bytes, and over
/// it the captured documentation rows in the order the walk read them — the
/// family's own rows ahead of the variants', and each variant's in the order its
/// lines were written. Each row is written as the declared-on seat's
/// discriminant, the variant spelling where the seat names one, and the row's
/// text; the roster's own length rides ahead of them all.
///
/// So the documentation commitment IS the pair the ruling names — the semantic
/// identity and the ordered rows — and a reader holding the semantic commitment
/// and the rows re-derives it and needs nothing else.
///
/// # Authority
///
/// **A second READING of one surface, never a second account of it.** The rows
/// are cut from the same token material the semantic commitment stands over, so
/// nothing here is a fact the capture did not already read; what this family
/// adds is a name for the reading a documentation projection is about, so that
/// projection's account can carry the fact it actually stands on rather than
/// borrowing the implementation side's.
///
/// # Bounds
///
/// It shares [`CapturedDeclarationSubject`] with the semantic commitment, and
/// the sharing is safe rather than convenient: two families are two segments of
/// the derive-key context AND two members of every transcript written under
/// them, so identical preimage bytes at the two levels are unrelated values
/// before a byte of content is read. That is the separation
/// [`RelatedBodySubject`] needed a second SUBJECT for, because the two levels
/// there stood under one family at one role and had nothing else to separate
/// them with.
///
/// # Versions
///
/// - **1** — the family as first declared. It did not exist for any of the five
///   positions the retired single version moved through.
pub const DECLARATION_DOCUMENTATION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::DeclarationDocumentation,
    IdentityProfileVersion::declared(1),
);

/// The profile one DECLARED STABLE NAME's identity is derived under.
///
/// # Preimage
///
/// The declared name's own bytes, exactly as this crate wrote them down, rooted,
/// at the roster position the declaring seat states.
///
/// # Authority
///
/// **A declared name is its own grammar, and it stops riding a neighbour's.**
/// The projection profile a Rust-declaration expansion runs under, the
/// projection kind the derive plans, the compiler-plane contract a diagnostic
/// expected to hold, and the callable entry point a diagnostic names for
/// reproduction are four values with one preimage shape: a constant this crate
/// declares, hashed. They stood under the plan role and the closed-expansion
/// role, which meant a bump to the PLAN grammar or to the CLOSED-EXPANSION
/// grammar renamed all four for a reason none of their preimages moved by.
///
/// The four are separated by their SUBJECTS, each of which is a segment of the
/// derive-key context and a member of every transcript — and by their content,
/// which is a different declared name in every case.
///
/// # Versions
///
/// - **1** — the family as first declared.
pub const DECLARED_NAME_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::DeclaredName,
    IdentityProfileVersion::declared(1),
);

/// The profile the GENERATOR VERSION identity is derived under.
///
/// # Preimage
///
/// The generator's declared name, length-framed, followed by its schema position
/// as four big-endian bytes — rooted, at position zero. The package version is
/// deliberately absent, for the reason [`GeneratorSchemaVersion`] states: it
/// moves for reasons no output noticed.
///
/// # Authority
///
/// **Its own family, because its grammar is genuinely its own.** Two framed
/// members are not one declared name, so it does not share
/// [`DECLARED_NAME_IDENTITY_PROFILE`] — and it stood under the PLAN role, whose
/// grammar it holds no member of.
///
/// # Bounds
///
/// This is the identity a plan's context NAMES as the generator it was produced
/// under. It is not the provenance record's generator ([`MACROC_GENERATOR`]),
/// which no preimage anywhere carries.
///
/// # Versions
///
/// - **1** — the family as first declared.
pub const GENERATOR_VERSION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::GeneratorVersion,
    IdentityProfileVersion::declared(1),
);

/// The profile a diagnostic's related identities are derived under, at both
/// levels.
///
/// # Preimage
///
/// The refusal family's own tag byte, then the framed material the level stands
/// over — the issue's own canonical bytes at the issue level, and the framing of
/// every issue in order at the body level — rooted, at the family's tag as the
/// roster position.
///
/// The two levels are separated by their SUBJECTS ([`RelatedIssueSubject`] and
/// [`RelatedBodySubject`]), which is what keeps a body's preimage from being
/// reachable as an issue's.
///
/// # Authority
///
/// **Its own family, because a diagnostic's relation is not a closed
/// expansion.** Both levels stood under [`ProjectionRole::ClosedExpansion`],
/// which put them on the closed-expansion family's version ladder — so a bump to
/// what a TERMINAL commits to would have renamed every related identity in every
/// diagnostic, and neither level holds a member of that grammar.
///
/// # Versions
///
/// - **1** — the family as first declared.
pub const DIAGNOSTIC_RELATION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::DiagnosticRelation,
    IdentityProfileVersion::declared(1),
);

/// The profile one projection intent's identity is derived under.
///
/// # Preimage
///
/// The content is the projection kind's declared name and the owner content
/// commitment it was meant over, rooted at position zero. **Nothing else enters
/// it** — no generator identity, no schema version, no delivery shape, no token
/// grammar — and the frame around it is the separation every transcript carries
/// and nothing about the machinery.
/// An intent therefore survives upgrading the machinery that realizes it, which
/// is the whole reason the intent layer exists: it is the layer two distinct
/// doors are ALLOWED to agree at, and the equivalence a builder door compares is
/// equality of these thirty-two bytes.
///
/// # Versions
///
/// - **1** — the family as first declared. None of positions 2 through 5 would
///   have moved it: the closure's tree digest, the entry account entering the
///   plan's content, the generated-token arms, and the two carrier slots beside
///   the partitioned emission are every one of them outside this preimage. The
///   single version renamed this family four times for nothing, and that is the
///   defect the split repairs.
pub const PROJECTION_INTENT_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::ProjectionIntent,
    IdentityProfileVersion::declared(1),
);

/// The profile one plan's identity is derived under.
///
/// # Preimage
///
/// The intent, the dependency set the account declares beside it, the shared
/// context, the complete membership in role-roster order, the watch set, the
/// decision trace, the origin trail, and the nonclaims — anchored on the
/// address the content walked in the door carrying.
///
/// A plan's own context names the generator version it was produced under, so
/// the generator reaches a plan's identity through the seat the plan DECLARED
/// it at, and never through a transcript member every family would have carried.
///
/// # Versions
///
/// - **1** — the family as first declared. Position 3 would have moved it: the
///   entry account entered this preimage, so a plan commits to the content it
///   was planned over rather than naming it nowhere. The first half of position
///   5 would have moved it too: the member-delivery encoding grew the two
///   carrier slots, and the membership writes every member's destination.
///   Positions 2 and 4 would not have — the closure's tree digest and the
///   generated-token roster are grammars a plan holds no member of.
pub const PLAN_IDENTITY_PROFILE: IdentityProfile =
    IdentityProfile::declared(PreimageFamily::Plan, IdentityProfileVersion::declared(1));

/// The profile one origin node's identity is derived under.
///
/// # Preimage
///
/// The declared material the node stands for, anchored on the address it is a
/// node of — so one piece of content is one node wherever it is reached from.
///
/// # Versions
///
/// - **1** — the family as first declared. None of positions 2 through 5 would
///   have moved it: an origin node's material is a declared constant and an
///   anchor, and no grammar any of those positions touched appears in it.
pub const ORIGIN_NODE_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::OriginNode,
    IdentityProfileVersion::declared(1),
);

/// The profile one generated unit's semantic key is derived under.
///
/// # Preimage
///
/// The declared material one planned member answers to and the roster position
/// of the role it stands under, anchored on what the plan hangs off — the
/// LOGICAL identity of a member, fixed before a byte of it exists.
///
/// # Versions
///
/// - **1** — the family as first declared. None of positions 2 through 5 would
///   have moved it. Position 4 in particular is the near miss and is worth
///   stating: it widened what a RENDERED tree's canonical bytes may contain,
///   and a semantic key carries no rendered byte at all.
pub const GENERATED_UNIT_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::GeneratedUnit,
    IdentityProfileVersion::declared(1),
);

/// The profile one rendered unit's identity and its output-bytes digest are
/// both derived under.
///
/// # Preimage
///
/// The exact rendered bytes, under the semantic key they answer to, at the
/// roster position of the role they were rendered under. Two roles read to this
/// one family — [`ProjectionRole::RenderedUnit`] names the unit and
/// [`ProjectionRole::OutputBytes`] names the digest of exactly those bytes —
/// because they stand over ONE grammar and are separated by the role rather
/// than by a second version ladder.
///
/// **A generator version is provenance recorded beside a rendered unit, never
/// entropy inside it.** The same exact bytes are the same artifact whichever
/// generator emitted them; which generator did is
/// [`ProjectionProvenance::generator`], and a rendered unit that changed its
/// name because the producer's package moved would be a rendered unit nobody
/// could match against the one they already hold.
///
/// # Versions
///
/// - **1** — the family as first declared. Position 4 would have moved it: the
///   generated-token roster grew the byte-string and numeric literal arms, so
///   the grammar a reader must hold to read a rendered tree's canonical bytes
///   widened, and a reader holding the earlier grammar cannot read a tree
///   carrying either arm. Positions 2, 3, and 5 would not have.
pub const RENDERED_UNIT_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::RenderedUnit,
    IdentityProfileVersion::declared(1),
);

/// The profile one bundle's identity is derived under.
///
/// # Preimage
///
/// The member plans a bundle names, as the set it publishes as one unit.
///
/// # Bounds
///
/// No road in this crate derives one yet: the plan family's bundle seat is
/// handed a bundle identity by the caller that owns the publication boundary.
/// The family is declared here so the mint, when it is written, lands under a
/// version of its own rather than borrowing a neighbour's.
///
/// # Versions
///
/// - **1** — the family as first declared. None of positions 2 through 5 would
///   have moved it.
pub const BUNDLE_IDENTITY_PROFILE: IdentityProfile =
    IdentityProfile::declared(PreimageFamily::Bundle, IdentityProfileVersion::declared(1));

/// The profile one proved closure's identity is derived under.
///
/// # Preimage
///
/// The plan's identity at the anchor, and over it: the explanation protocol
/// version, the complete planned membership in role-roster order, the role
/// roster's own length, the unit that stood under each role, and the
/// partitioned emission's digests — the whole agreement rather than a sample of
/// it.
///
/// # Versions
///
/// - **1** — the family as first declared. Position 2 would have moved it: the
///   joined-tree digest entered this preimage, so a closure commits to the tree
///   it proved. The second half of position 5 would have moved it too: the
///   emission member became the partitioned encoding where it had been that one
///   digest. Positions 3 and 4 would not have — the entry account is the plan's
///   member and reaches a closure only through the plan identity at its anchor,
///   and the generated-token arms widen the material a digest is taken over
///   without widening this grammar.
pub const CLOSURE_IDENTITY_PROFILE: IdentityProfile =
    IdentityProfile::declared(PreimageFamily::Closure, IdentityProfileVersion::declared(1));

/// The profile one explanation's identity is derived under.
///
/// # Preimage
///
/// The CLOSURE's identity at the anchor, at its full thirty-two bytes — an
/// explanation is written after a closure and over it — and over it, in this
/// order:
///
/// 1. the PLAN's identity, at full width, which is the other half of the
///    parentage a complete view carries;
/// 2. the number of answered seats;
/// 3. for every seat, in the KIND's declared question order: the question's own
///    roster slot, the typed answer's discriminant, and that answer's typed
///    material — identities at full width, typed rosters written length-framed
///    in the order the answer carries them, and typed postures written as their
///    own discriminants ahead of whatever they carry.
///
/// The declared question order is the roster's, never the caller's, so two views
/// answering one kind's questions with one set of answers derive one identity
/// whichever order the answers were supplied in.
///
/// So the three things the ruling names — the plan identity, the closure
/// identity, and the canonical typed answers — are all committed to, and an
/// explanation of one expansion can no longer stand where another expansion's
/// explanation of the same kind stands.
///
/// # Authority
///
/// **Human prose is excluded from this preimage, and the exclusion is the
/// point.** A rendered line is a projection of a typed answer, composed when it
/// is asked for and never stored
/// ([`ProjectionExplanation::human`](crate::explanation_protocol::ProjectionExplanation::human)),
/// so a preimage that carried one would commit to a rendering rather than to
/// what was answered — and would rename every explanation in the tree the day a
/// sentence was reworded. A repair's citation enters; the sentence beside it,
/// which is that citation's own projection, does not.
///
/// # Nonclaims
///
/// One typed posture is written narrower than it reads, and it is named where it
/// happens: a related projection's disposition enters as its posture and as the
/// plane-typed values that posture carries, and the arm carrying a PLANNING
/// REFUSAL enters as the posture alone. A planning body is the refusal home's
/// value and the plane declares no canonical encoding for it, so writing one
/// here would be this side legislating another home's encoding. Two explanations
/// differing only in which planning body a related projection was refused with
/// therefore carry one identity, and the refusal's own detail reaches a caller
/// through the diagnostic that projects it.
///
/// # Versions
///
/// - **1** — the family as first declared. It did not exist for any of the five
///   positions the retired single version moved through.
/// - **2** — the RELATED-PROJECTION seat's disposition grammar widened. The
///   profile-unavailable posture gained the owner-fact citation naming what the
///   profile could not furnish, and that citation is written into the
///   disposition's canonical bytes
///   ([`ProjectionDisposition::encode_into`](crate::planning::ProjectionDisposition::encode_into))
///   — so a seat answering
///   "why was the related projection not generated" with that posture now
///   carries a member this preimage did not carry at position 1, and a reader
///   holding the earlier grammar cannot read the seat's material to its end.
///   The widening is inside the third member above (the typed answers), which is
///   why it moves this family and no other: the plan identity at member one and
///   the closure identity at the anchor commit to no disposition, so neither of
///   those families moves for it.
pub const EXPLANATION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::Explanation,
    IdentityProfileVersion::declared(2),
);

/// The profile one closed expansion's identity is derived under.
///
/// # Preimage
///
/// **The three identities a terminal binds, and nothing twice.** The CLOSURE's
/// identity stands at the anchor, at its full thirty-two bytes, and over it a
/// content of exactly two members, in this order:
///
/// 1. the PLAN's identity, which already commits to the entry account (and
///    through it the kind), the context, and the complete declared membership;
/// 2. the EXPLANATION's identity, which commits to the plan and the closure it
///    was answered over and to every typed answer it carries.
///
/// Every other candidate member is already inside one of the three: the
/// partitioned emission is committed by the anchor (a closure's identity is
/// derived over its partition digests), and the kind by the plan's intent — so
/// a member for either here would spell one fact twice, and two spellings of
/// one fact are how a preimage drifts.
///
/// The explanation member is why this family exists at the shape it does: a
/// terminal binding plan A, closure A, and a DIFFERENT expansion's explanation
/// of the same kind used to derive the identity of the honest one, because the
/// explanation reached no member of this preimage at all.
///
/// # Versions
///
/// - **1** — the family as first declared. Every retired position reaches this
///   preimage only through the identities it commits to: position 2 and the
///   second half of position 5 moved the closure's own encoding and would have
///   moved the ANCHOR's value, not this grammar; positions 3 and 4 reach it
///   through the plan and rendered-unit families the same way. A version here
///   moves only when the member list above is recut.
pub const CLOSED_EXPANSION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    PreimageFamily::ClosedExpansion,
    IdentityProfileVersion::declared(1),
);

/// The stable name of the generator that derives plane identities.
///
/// A name, not a version: it says WHICH generator, and it changes only when a
/// different generator starts producing this material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorProfileId(&'static str);

/// The version of the SHAPE this generator renders.
///
/// Bump it when the rendered output's shape changes: a different token layout, a
/// different set of rendered roles, a different contract realized, or a
/// different meaning attached to one that already existed.
/// Do not bump it for a change that cannot reach the output — a comment, a
/// refactor, a renamed local.
///
/// This is deliberately NOT the package version, which moves for reasons that
/// have nothing to do with the rendered shape and is therefore worthless as the
/// fact a reader judges staleness by.
///
/// # Bounds
///
/// **It is not a segment of any preimage.** A bump here renames no identity in
/// the tree: which generator rendered a thing is a fact ABOUT the derivation and
/// rides [`ProjectionProvenance`], while what the thing IS rides the preimage
/// its family declares. Where a shape change genuinely changes what something
/// is, it says so where the change lands — a plan whose rendered-role roster
/// grew declares a different membership, and the membership is a plan's own
/// transcript member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorSchemaVersion(u32);

/// Which generator produced a plane identity, and under which rendered shape.
///
/// # Authority
///
/// **It is provenance, whole.** No preimage family names it, so no transcript
/// writes it: the record of a derivation carries it
/// ([`ProjectionProvenance::generator`]), a reader compares it against
/// [`MACROC_GENERATOR`] to judge staleness
/// ([`GeneratorIdentity::same_rendered_shape`]), and nothing derived anywhere in
/// the plane moves when it moves.
/// The retired posture wrote the name and the schema version into every
/// transcript, which made a rendering-shape change rename the intent identity
/// of a door whose meaning had not moved — and the intent identity is exactly
/// what door equivalence compares.
///
/// The profile name and the schema version are the two LOAD-BEARING facts a
/// staleness comparison reads. The package version is recorded and read back but
/// is compared by nothing, because a report of "a different generator" on a
/// version bump nobody's output noticed is noise dressed as provenance.
///
/// # Nonclaims
///
/// It carries no digest of this generator's own source, and holding one is no
/// evidence about the source: computing such a digest would mean reading the
/// source tree at expansion time, which the ambient-free law forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorIdentity {
    profile: GeneratorProfileId,
    schema: GeneratorSchemaVersion,
    package: &'static str,
}

/// This generator, as every derivation record in this crate names it.
///
/// # Versions
///
/// Each position below is the change to the RENDERED SHAPE that moved it,
/// because that is the only thing this position may move for
/// ([`GeneratorSchemaVersion`]).
/// A position is never reused and never edited.
///
/// - **1** — the shape as first declared.
/// - **2** — the derive-implementation projection's rendered-role roster gained
///   the two mutation-evaluation roles, so one implementation meaning is
///   delivered under four rendered roles where it was delivered under two. "A
///   different set of rendered roles" is exactly what this position exists to
///   move for. It reaches identity where it belongs and nowhere else: a plan's
///   transcript writes its membership in ROLE-ROSTER order over the whole
///   roster, so plans of that kind derive different identities because they
///   declare a different output set — which is the point, since a plan produced
///   before the evaluation copy was a declared member declared a smaller output
///   set than the delivery actually has.
/// - **3** — the delivery shape changed: members whose meaning is evaluation
///   ride a carrier into the consumption target where they were emitted into
///   the declaration-site tree, and the emitted output is a partitioned value
///   rather than one joined tree. "A different meaning attached to a token
///   layout that already existed" is what this position exists to move for, and
///   it too reaches identity through the seats that state it — a member's
///   destination inside a plan's membership, and the emission member of a
///   closure's own preimage.
pub const MACROC_GENERATOR: GeneratorIdentity = GeneratorIdentity::declared(
    GeneratorProfileId::declared("threadpak-macroc"),
    GeneratorSchemaVersion::declared(3),
    env!("CARGO_PKG_VERSION"),
);

/// What one transcript is anchored under.
///
/// Each posture is written into the transcript as a distinct discriminant ahead
/// of its commitment, so a rooted transcript can never encode as an anchored one
/// whose anchor happened to be empty.
///
/// # The published discriminants
///
/// Member 7 of every transcript ([`ProjectionTranscript`]) is one byte, and it
/// is this byte. The values are DECLARED here, beside the postures they stand
/// for, because an independent reader re-deriving a transcript needs them and
/// must not have to read an encoder body to find them:
///
/// | posture | byte |
/// | ------- | ---- |
/// | [`TranscriptAnchoring::Rooted`] | `0` |
/// | [`TranscriptAnchoring::UnderOwnerIdentity`] | `1` |
/// | [`TranscriptAnchoring::UnderProjectionIdentity`] | `2` |
///
/// [`TranscriptAnchoring::slot`] answers with exactly these, and the encoder
/// reads that answer rather than spelling a second table — so the declaration
/// owns the fact and the encoder body is the enforcement of it.
///
/// A value here is APPENDED and never renumbered, on the terms every slot table
/// in the services stands under: renumbering an occupied position re-encodes
/// transcripts that were already encoded, which renames every identity derived
/// from them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptAnchoring {
    /// No anchor at all — the root of one derivation chain, where the content is
    /// the whole of what varies.
    /// The captured declaration is what stands here: everything else in a plan
    /// hangs off it.
    Rooted,
    /// Anchored under an identity the MACHINE minted, carried at full width.
    UnderOwnerIdentity([u8; 32]),
    /// Anchored under another identity the PLANE owns, carried at full width.
    UnderProjectionIdentity([u8; 32]),
}

/// The COMPLETE preimage one [`ProjectionIdentity`] is derived from.
///
/// A transcript is the exact byte string handed to the digest, and this
/// specification is complete: an independent implementation needs what follows
/// and nothing else.
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
/// | 1 | profile stem | `bytes(utf8)` of [`IDENTITY_PROFILE_STEM`] |
/// | 2 | preimage family | `bytes(utf8)` of [`PreimageFamily::stable_name`] |
/// | 3 | profile version | `u32be`, that family's own position |
/// | 4 | identity subject | `bytes(utf8)` of [`IdentitySubject::SUBJECT_NAME`] |
/// | 5 | role | `bytes(utf8)` of [`ProjectionRole::stable_name`] |
/// | 6 | role slot | one byte, [`ProjectionRole::slot`] |
/// | 7 | anchoring | one byte, [`TranscriptAnchoring::slot`] |
/// | 8 | anchor commitment | `bytes(…)` — empty when rooted, else the full 32 |
/// | 9 | content | `bytes(…)` — the full material, never a fold |
/// | 10 | roster position | `u32be` |
///
/// The derive-key context is [`IdentityProfile::context_for`] over the same
/// subject and role, and the identity is
/// `blake3::derive_key(context, transcript) -> [u8; 32]`.
///
/// Nothing is folded on the way in: the anchor is carried at its full 32 bytes
/// and the content at its full length, so the 32-byte output is the only
/// compression anywhere in the derivation.
///
/// # The family is read off the role
///
/// The profile is not a parameter of a mint site. The role a mint site names
/// answers which family it stands in ([`ProjectionRole::family`]), and the
/// family answers which profile and which version
/// ([`PreimageFamily::profile`]), so a call site cannot derive a rendered unit
/// under the plan family's ladder and no seam anywhere carries a second opinion
/// about which grammar a preimage belongs to.
///
/// # Nonclaims
///
/// **The generator is not a member.** The generator this transcript names is
/// carried for the derivation RECORD ([`ProjectionTranscript::provenance`]) and
/// is written into no preimage: no family's grammar names it, so a shape bump
/// renames nothing, and a rendered unit's bytes keep their name across the
/// producers that emitted them.
/// Where a generator's shape genuinely changes what something is, the change
/// lands in that thing's own declared seats — a membership, a destination, an
/// emission — and reaches identity there.
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
/// family's profile at which version, which generator, what it was anchored
/// under, and how much content went in.
///
/// The identity answers "which thing is this?" and is thirty-two bytes; the
/// record answers "where did those thirty-two bytes come from?" and is
/// inspection material.
/// They are separate values so neither constrains the other: the transcript can
/// be complete because it is not stored, and the record can be honest because it
/// is written once where the derivation happened rather than copied everywhere
/// the identity goes.
///
/// The subject, the role, the profile and its version, the generator, the
/// anchoring posture, the anchor commitment at its FULL thirty-two bytes, and
/// the roster position are all carried exactly.
/// The content is stated by its LENGTH and not carried, because content is
/// unbounded — a rendered unit's canonical bytes run to the declared rendering
/// magnitude — and a record that copied it would double every rendering in
/// memory to say something the rendered unit already holds.
///
/// # The generator is here and only here
///
/// This is the seat the generator identity occupies in the plane.
/// It is recorded, read back ([`ProjectionProvenance::generator`]), and compared
/// for staleness ([`ProjectionProvenance::under_current_shape`]) — and it is
/// written into no preimage, so a reader learns which generator produced a value
/// without every value's name depending on the answer.
///
/// # Nonclaims
///
/// The stated length is not a fold and identifies nothing: reading it as a
/// summary of the content reads a claim nobody made.
/// The identity is what commits to the content, at full width, under BLAKE3.
///
/// A recorded generator says which producer ran, never whether the value is
/// current, correct, or comparable: a staleness reading is a fact about the
/// PRODUCER and says nothing about whether the material moved.
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
/// Holding one means the plane derived these thirty-two bytes from a complete
/// [`ProjectionTranscript`], under the profile the transcript's own preimage
/// family declares, and would derive the same ones again from the same
/// transcript on any machine.
///
/// # Authority
///
/// **Collision resistance is claimed AS BLAKE3's, for the transcript as
/// specified on [`ProjectionTranscript`], under the [`IdentityProfileVersion`]
/// the deriving [`PreimageFamily`] declares — and nothing broader.**
/// Finding two different transcripts that derive one identity is as hard as
/// finding a BLAKE3 collision.
///
/// # Construction
///
/// The only road is [`ProjectionIdentity::derived`], which takes a typed
/// transcript; no seam, public or crate-internal, wraps arbitrary bytes.
/// `Subject` is a `PhantomData` parameter, so an identity naming one subject is
/// a different type than one naming another regardless of bytes — and their
/// derive-key contexts differ too, so the separation is a runtime fact and not
/// only a compile-time one.
/// Neither identity family is reachable from the other, and plane identities are
/// never ranked, so there is no `Ord`.
///
/// # Nonclaims
///
/// It does NOT claim that two things the plane considers different always have
/// different transcripts — that is the transcript's completeness, which each
/// mint site is responsible for and documents.
/// It does NOT claim anything about a different profile version, which derives
/// under different contexts and is a different name space.
/// And it is never a machine commitment: where the machine needs one the machine
/// mints it, and no plane identity is accepted in its place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIdentity<Subject> {
    bytes: [u8; 32],
    _subject: PhantomData<Subject>,
}

/// One projection plan's own identity — the one type anything that speaks of a
/// plan by identity names.
pub type PlanId = ProjectionIdentity<PlanSubject>;

/// One proved closure's own identity.
pub type ClosureId = ProjectionIdentity<ClosureSubject>;

/// One complete explanation's own identity — the name a terminal binds its
/// explanation under, and commits to.
pub type ExplanationId = ProjectionIdentity<ExplanationSubject>;

/// One closed expansion's own identity.
pub type ClosedExpansionId = ProjectionIdentity<ClosedExpansionSubject>;

/// The seal on the rendered-role roster.
///
/// A value of this type is producible only inside the services, so a roster
/// declared anywhere else cannot satisfy [`RenderedRole`].
/// The closure check walks [`RenderedRole::ROLES`] and asks what stood under
/// each one, so a roster that left a variant out would make that variant's
/// rendered unit invisible to the loop that is supposed to prove it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderedRoleSeal(());

/// The closed roster of rendered units one projection kind materializes.
///
/// A kind declares this roster once, and the closure check reads it: a rendered
/// unit is matched to a planned member by ROLE, so "the family implementation"
/// and "the cause-order implementation" are different seats rather than two
/// entries in an ordered list nobody can tell apart.
/// A rendering that produced the right number of units in the wrong roles is
/// caught by the role, not by a count.
///
/// # Authority
///
/// Every claim in the plane that "every role was examined" is made by walking
/// [`ROLES`], so the roster IS the quantifier — which is why it is sealed.
/// An implementation that omitted one variant would render a unit the closure
/// loop never looks at and never reports, which is a silent output past the
/// firewall.
///
/// [`ROLES`]: RenderedRole::ROLES
pub trait RenderedRole: Copy + PartialEq + Eq + core::fmt::Debug + Sized + 'static {
    /// The seal. Only the services can produce a value of this type.
    const SEAL: RenderedRoleSeal;

    /// The complete roster, in the order the kind states it.
    const ROLES: &'static [Self];

    /// This role's position in the roster.
    /// Part of every transcript derived for the role, so two roles never derive
    /// one identity.
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

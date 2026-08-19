//! The plane's declarations: the two identity families, the subject roster, the
//! magnitude stamp and the plane's own magnitude rows, the profile and generator
//! facts, the transcript and its derivation record, and the rendered-role
//! contract.
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

pub(crate) use guard::{human_projection, static_bytes};

#[cfg(test)]
pub(crate) use guard::for_laws;

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

        /// Every declared subject name, in roster order, for the proof surface.
        #[cfg(test)]
        pub(crate) const SUBJECT_NAMES: &[&str] = &[$($declared),+];
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
/// # The two forms
///
/// The plain form stamps rows. The ROSTERED form stamps the same rows and emits
/// one constant reading them back — `limits! { roster NAME; … }` — for a row set
/// that has a reader with a question about the set as a whole rather than about
/// one family. The rostered form delegates the rows to the plain one rather than
/// spelling the emission twice, so there is exactly one transcriber for a
/// family however it is declared.
macro_rules! limits {
    (
        roster $roster:ident;
        $( $(#[$note:meta])* $name:ident = $max:expr ),+ $(,)?
    ) => {
        $crate::plane::limits! { $( $(#[$note])* $name = $max ),+ }

        /// Every limit family THIS row set declares, as its Rust spelling and
        /// the magnitude it declares.
        ///
        /// Emitted from the SAME rows as the families themselves, in one
        /// expansion, so it is not an inventory of the declarations — it is the
        /// declarations, read a second way, with no row anybody could forget to
        /// add or leave stale.
        /// It exists for the proof surface, the one reader with a question about
        /// this row set as a whole rather than about one family.
        ///
        /// It is a projection over one row set and never a second owner of any
        /// row in it: a magnitude read here and a magnitude read through the
        /// family's own `ConstLimit` are one number.
        #[cfg(test)]
        pub(crate) const $roster: &[(&str, usize)] = &[
            $( (stringify!($name), <$name as ::threadpak::types::ConstLimit>::MAX) ),+
        ];
    };
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
    /// One closed expansion: the whole receipt one live compilation produced.
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
}

limits! {
    roster DECLARED_LIMITS;
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
    /// missing-fact issue over its one plan seat, the discontinuity issue over
    /// the one break a trail is refused at, the bound issue over its six axes,
    /// and the doubled-output issue over the sixteen roles a membership at the
    /// output magnitude could double.
    PlanningIssueLimit = 29,
    /// Issues one explanation-coverage refusal body may carry: each of the
    /// fourteen questions may be unanswered, answered twice, or answered where
    /// the kind does not admit it, and no two of those hold of one question at
    /// once.
    ExplanationIssueLimit = 14,
    /// Explanation seats one view may hold — the question roster's cardinality.
    ExplanationSeatLimit = 14,
    /// Bytes one human projection may carry.
    HumanTextLimit = 512,
    /// Related issues one diagnostic may point at. A diagnostic projects a
    /// refusal body issue for issue, so a narrower bound here would make the
    /// projection drop established issues to fit.
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
    /// Issues one template-construction refusal body may carry, sized by the
    /// widest of the three passes rather than by the narrowest.
    ///
    /// The binding pass is that one, and it asks two independent questions per
    /// declared hole: how many bindings name it, and whether one of them
    /// disagrees with its declared category. Both can hold of one hole at once,
    /// so the pass establishes up to two issues per declared parameter, plus one
    /// unknown-parameter issue per supplied binding — three times the parameter
    /// magnitude. The hole pass and the ceiling pass are narrower and fit
    /// inside it.
    TemplateIssueLimit = 96,
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
    /// Units of capture work one walk may spend, one unit per examined token.
    ///
    /// Wider than the whole-tree magnitude, because a walk may LOOK at more than
    /// it keeps: a budget at the tree magnitude exactly would refuse a lawful
    /// input the moment its producer looked twice at anything.
    /// Four units for every token [`CapturedTreeTokenLimit`] admits, which is
    /// the room a producer that backtracks over an alternative or skips trivia
    /// needs and no more.
    /// That magnitude is the one this number stands over, so the two are moved
    /// together or not at all: a wider tree under this budget would refuse
    /// lawful declarations naming a bound they never approached, and this is the
    /// number that would have to move to keep the tree magnitude reachable.
    CaptureWorkLimit = 65536,
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
    /// Mutation points one evaluation surface may admit.
    ///
    /// Sixty-four. What it rules out is a compile-once evaluation copy whose
    /// selection roster has stopped being reviewable: every point is an arm in
    /// every other point's `match`, so the rendered copy grows with the square
    /// of the roster and a reader auditing which damages a surface admits is
    /// reading a page rather than a list.
    MutationPointLimit = 64,
    /// Alternatives one mutation point may admit.
    ///
    /// Eight. A point names the admitted damages of ONE operation, and a point
    /// offering more than eight has stopped being about one operation — the
    /// repair is a second point at a second activation site, not a wider roster
    /// at this one.
    MutationAlternativeLimit = 8,
    /// Issues one surface-composition refusal body may carry.
    ///
    /// Twice the mutation-point magnitude, sized by the widest pass rather than
    /// the narrowest. That pass is the naming pass, and it asks TWO independent
    /// questions of every admitted point — whether the point claims the
    /// no-mutation control's reserved name, and whether it is the second point
    /// under its own — and both can hold of one point at once. The passes
    /// themselves do not add up: they are dependent, and each refuses before the
    /// next one runs.
    ///
    /// Written as the number rather than as a product of the family above it: a
    /// magnitude derived from another magnitude reads as a fact when it is a
    /// choice, and this number would still be owed if the point magnitude moved
    /// for its own reasons.
    SurfaceIssueLimit = 128,
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
    }
}

/// One version of the projection-identity profile.
///
/// The version is a typed constant and a real segment of every derive-key
/// context, not a comment about one.
/// Changing what a transcript contains, what order it is written in, or what the
/// domain grammar spells is a version bump, and a bump renames every identity
/// the profile derives — which is exactly what it is for.
///
/// The members [`ProjectionTranscript`] specifies are one half of what a
/// transcript contains; the other half is the CONTENT each mint site composes
/// and documents.
/// A change to either is a change to what a transcript contains, so a reader
/// handed two identities under one version may assume both were derived the
/// same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfileVersion(u32);

/// The versioned, domain-separated profile the plane derives its identities
/// under.
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
/// [`ProjectionRole::stable_name`].
/// Every segment is lowercase ASCII letters, digits, and `-`, joined by `/`.
///
/// Separation is by CONTEXT and not by message prefix.
/// Two identities over identical transcript bytes under different subjects or
/// different roles are derived under different keys, so they are unrelated
/// values rather than neighbouring ones — there is no shared hash state for them
/// to collide inside.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    stem: &'static str,
    version: IdentityProfileVersion,
}

/// The profile every plane identity in this crate is derived under.
///
/// # Versions
///
/// Each position below is the change to what a transcript CONTAINS that moved
/// it, because that is the only thing a position may move for.
/// A position is never reused and never edited: the identities derived under an
/// earlier one keep their names, in a name space nothing later can reach.
///
/// - **1** — the profile as first declared.
/// - **2** — the closure transcript grew the joined-tree digest, so a closure
///   commits to the tree it proved rather than to the units it counted.
/// - **3** — the entry account entered the plan transcript's content: a plan is
///   derived over the ONE account of the owner content it was planned over —
///   the commitment and the dependency set it declares it stands on — where the
///   content it stood on had been named nowhere in its own preimage.
/// - **4** — the generated-token roster grew the byte-string and numeric
///   literal arms (slots 5 and 6), so the grammar a reader must hold to read
///   a generated tree's canonical bytes widened; every tree spellable before
///   the arms encodes byte for byte as it did, and the bump keeps the
///   profile's one promise — two identities under one version were derived
///   the same way — true of the widened grammar.
/// - **5** — two transcript grammars widened at once: the member-delivery
///   encoding grew the two carrier slots, and the closure transcript's
///   emission member became the partitioned-emission encoding where it had
///   been one joined-tree digest. A reader holding the earlier grammar cannot
///   read either transcript, so the version moved for the same reason
///   position 4 did.
pub const PROJECTION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    "threadpak/macroc/projection-identity",
    IdentityProfileVersion::declared(5),
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
/// It rides in every transcript, so a bump renames every identity this generator
/// derives, and a plan produced under the old shape can never be mistaken for
/// one produced under the new.
///
/// This is deliberately NOT the package version, which moves for reasons that
/// have nothing to do with the rendered shape and is therefore worthless as the
/// fact a plan is invalidated by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorSchemaVersion(u32);

/// Which generator produced a plane identity, and under which rendered shape.
///
/// The profile name and the schema version are IN the transcript: they decide
/// identity, and a change to either renames what the generator derives.
/// The package version is recorded and read back but is NOT in the transcript,
/// because an identity that changed on a version bump nobody's output noticed
/// would be noise dressed as provenance.
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

/// This generator, as every transcript in this crate names it.
///
/// # Versions
///
/// Each position below is the change to the RENDERED SHAPE that moved it,
/// because that is the only thing this position may move for
/// ([`GeneratorSchemaVersion`]).
/// A position is never reused and never edited: the identities derived under an
/// earlier one keep their names, in a name space nothing later can reach.
///
/// - **1** — the shape as first declared.
/// - **2** — the derive-implementation projection's rendered-role roster gained
///   the two mutation-evaluation roles, so one implementation meaning is
///   delivered under four rendered roles where it was delivered under two. "A
///   different set of rendered roles" is exactly what this position exists to
///   move for, and it reaches identity through the membership: a plan's
///   transcript writes its membership in ROLE-ROSTER order over the whole
///   roster, so plans of that kind derive different identities under the new
///   shape — which is the point, since a plan produced before the evaluation
///   copy was a declared member declared a smaller output set than the delivery
///   actually has.
/// - **3** — the delivery shape changed: members whose meaning is evaluation
///   ride a carrier into the consumption target where they were emitted into
///   the declaration-site tree, and the emitted output is a partitioned value
///   rather than one joined tree. "A different meaning attached to a token
///   layout that already existed" is what this position exists to move for — a
///   plan produced under the joined shape declared a delivery the output no
///   longer has.
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
/// subject and role, and the identity is
/// `blake3::derive_key(context, transcript) -> [u8; 32]`.
///
/// Nothing is folded on the way in: the anchor is carried at its full 32 bytes
/// and the content at its full length, so the 32-byte output is the only
/// compression anywhere in the derivation.
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
/// # Nonclaims
///
/// The stated length is not a fold and identifies nothing: reading it as a
/// summary of the content reads a claim nobody made.
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
/// Holding one means the plane derived these thirty-two bytes from a complete
/// [`ProjectionTranscript`] under [`PROJECTION_IDENTITY_PROFILE`], and would
/// derive the same ones again from the same transcript on any machine.
///
/// # Authority
///
/// **Collision resistance is claimed AS BLAKE3's, for the transcript as
/// specified on [`ProjectionTranscript`], under profile version
/// [`IdentityProfileVersion`] as declared by
/// [`PROJECTION_IDENTITY_PROFILE`] — and nothing broader.**
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

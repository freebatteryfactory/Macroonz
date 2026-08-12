//! The metaprogramming plane's shared carriers: the two identity families,
//! owner-fact references, profile versions, bounded human projections, and the
//! plane's declared limit families.
//!
//! # Two identity families, and neither can stand in for the other
//!
//! **[`OwnerIdentityRef`] is a read-only lens on an identity the MACHINE
//! minted.** The machine's identity home mints; the services never do. A lens
//! arrives through [`OwnerIdentityRef::of_commitment`], which reads a machine
//! commitment's published bytes and adapts nothing: identity, schema, authority,
//! bounds, and meaning cross unchanged, which is exactly what a projection is
//! allowed to do. Holding one means only "the compiler refers exactly to this
//! owner identity" — nothing about admission, authority, freshness, or
//! equivalence. There is no public raw-byte road at all.
//!
//! **[`ProjectionIdentity`] is an identity the COMPILER PLANE owns.** Plans,
//! origin nodes, rendered units, generated units, closures, and bundles are the
//! plane's own material: the machine has no opinion about them and mints nothing
//! for them, so the plane names them itself. Every one is derived
//! deterministically from a COMPLETE [`ProjectionTranscript`] under the
//! versioned, domain-separated profile [`PROJECTION_IDENTITY_PROFILE`], and the
//! derivation record — which subject, which role, which profile version, which
//! transcript members — is a separate inspectable value,
//! [`ProjectionProvenance`], carried once where the derivation happened rather
//! than inside every identity.
//!
//! The two families are different types over different subject markers and
//! neither converts to the other. A plane identity is never accepted by the
//! machine as a mint, and an owner lens is never derived by the plane.
//!
//! # Human text is never load-bearing
//!
//! [`HumanProjection`] carries bytes a caller may show a person. Nothing in the
//! plane reads it back, matches on it, or decides from it — every decision cites
//! an [`OwnerFactRef`] or a typed value instead.

use core::marker::PhantomData;
use threadpak::identity::Commitment;
use threadpak::refusal::ReasonId;
use threadpak::types::{Bounded, BoundedConstruction, ConstLimit, Limit};

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
pub trait IdentitySubject {
    /// The subject's declared segment of the derive-key context.
    const SUBJECT_NAME: &'static str;
}

/// Declares the plane's subject markers: one zero-sized type per identity
/// subject, each `Eq`/`Hash`/`Copy` so an identity tagged with it composes into
/// the plane's records without hand-written impls, and each carrying its
/// declared [`IdentitySubject`] name so no marker can exist without one.
macro_rules! subjects {
    ($( $(#[$note:meta])* $name:ident = $declared:literal ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl IdentitySubject for $name {
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

impl<Subject> OwnerIdentityRef<Subject> {
    /// The production road: project one machine commitment into the plane. The
    /// commitment's domain is the reference's subject, so a commitment over one
    /// domain cannot become a reference naming another. Nothing is adapted —
    /// the bytes cross unchanged.
    #[must_use]
    pub fn of_commitment(commitment: &Commitment<Subject>) -> Self {
        Self {
            bytes: *commitment.as_bytes(),
            _subject: PhantomData,
        }
    }

    /// The decode-route seam, awaiting a real decoder.
    ///
    /// It is crate-internal on purpose: an identity that arrived already in its
    /// declared byte order comes from an artifact somebody decoded, and the
    /// decoder that will own this route does not exist yet. Until it does, this
    /// is the single byte road in the plane and no caller outside the services
    /// can reach it. It mints nothing and admits nothing; the machine never
    /// accepts a plane reference as an identity mint.
    #[must_use]
    pub(crate) const fn decoded(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _subject: PhantomData,
        }
    }

    /// The identity's declared raw-byte storage order, borrowed for comparison
    /// and for rendering.
    ///
    /// This is not a subject-erasing conversion. Reading the bytes out and
    /// re-wrapping them under a different subject is unrepresentable outside
    /// this crate because no public byte constructor exists to wrap them with —
    /// the accessor is one-way by the absence of its inverse, not by a runtime
    /// check.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl OwnerIdentityRef<RefusalReason> {
    /// Project one registered refusal reason into the plane. A diagnostic names
    /// the reason the machine registered; it never registers one.
    #[must_use]
    pub fn of_reason(reason: ReasonId) -> Self {
        Self::decoded(*reason.as_bytes())
    }
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

impl OwnerFactRef {
    /// Cite one owner fact by the declared names its home wrote down.
    #[must_use]
    pub const fn named(home: &'static str, fact: &'static str) -> Self {
        Self::Declared(OwnerFactName { home, fact })
    }

    /// The canonical bytes of this citation, for a transcript to be taken over.
    #[must_use]
    pub fn citation_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Self::Minted { home, fact } => {
                bytes.push(0);
                bytes.extend_from_slice(home.as_bytes());
                bytes.extend_from_slice(fact.as_bytes());
            }
            Self::Declared(named) => {
                bytes.push(1);
                bytes.extend_from_slice(named.home.as_bytes());
                bytes.push(b'.');
                bytes.extend_from_slice(named.fact.as_bytes());
            }
        }
        bytes
    }
}

/// One version of one projection profile: a position in that profile's own
/// order. There is no `Ord` — versions of two different profiles are not
/// comparable, and the plane never ranks them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileVersion(u64);

impl ProfileVersion {
    /// The version the profile's authority assigned.
    #[must_use]
    pub const fn declared(position: u64) -> Self {
        Self(position)
    }

    /// The assigned position.
    #[must_use]
    pub const fn position(self) -> u64 {
        self.0
    }
}

/// One bounded human-readable rendering of a typed value.
///
/// It is a projection and only a projection: derived from typed values, carried
/// for a person to read, and never read back by the plane. No decision, no
/// identity, and no refusal consults it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HumanProjection<L: Limit> {
    text: Bounded<u8, L>,
}

impl<L: ConstLimit> HumanProjection<L> {
    /// Render one bounded human projection.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the rendering exceeds the
    /// family's declared byte maximum. A projection that does not fit refuses
    /// rather than truncating: a silently cut explanation is a false one.
    pub fn projected(text: &str) -> Result<Self, BoundedConstruction> {
        Bounded::admitted_const(text.as_bytes().to_vec()).map(|text| Self { text })
    }

    /// The seam behind [`human_projection!`], which is the only road to it.
    ///
    /// # There is no length to check here, so there is no branch to fall down
    ///
    /// The rendering arrives as a fixed-width byte array, and the width is the
    /// array's own TYPE. So this road carries no runtime count, returns no
    /// refusal, and has no branch where a rendering that did not fit becomes an
    /// empty one — the earlier seam had exactly that branch, and an oversized
    /// explanation silently became a blank one.
    ///
    /// The width cannot be chosen independently of the material either: the
    /// caller does not pass a length, it passes the array, and
    /// [`human_projection!`] builds that array in a `const` item out of the
    /// rendering itself. A rendering the width does not cover stops the
    /// compiler during that const evaluation.
    #[must_use]
    pub(crate) fn proven<const N: usize>(rendered: [u8; N]) -> Self {
        Self {
            text: Bounded::from_array(rendered),
        }
    }
}

/// One static rendering's bytes, at the fixed width the caller declared.
///
/// Written for the `const` item [`human_projection!`] builds. Evaluated at
/// compile time, where a width the rendering does not reach is a compile error
/// rather than a padded or cut projection handed to a reader.
#[expect(
    clippy::indexing_slicing,
    reason = "the walk is a const evaluation over the declared width, so an index past the rendering stops the compiler instead of reading at runtime"
)]
#[must_use]
pub(crate) const fn static_bytes<const N: usize>(text: &str) -> [u8; N] {
    let source = text.as_bytes();
    let mut rendered = [0u8; N];
    let mut at = 0usize;
    while at < N {
        rendered[at] = source[at];
        at = at.saturating_add(1);
    }
    rendered
}

/// Projects one STATIC rendering, proving at COMPILE TIME that it fits the named
/// limit family.
///
/// This is the total road, and it is the only road to
/// [`HumanProjection::proven`]. `HumanProjection::projected` reads a runtime
/// length and may refuse, and a caller that swallowed that refusal with an empty
/// fallback would be silently deleting an explanation — which is exactly the
/// defect this macro exists to make unrepresentable. Where the material is
/// static, the length is a compile-time fact: the `const` block below settles
/// the bound, the `const` item below carries the rendering at its own width, and
/// no refusal road appears anywhere between them.
macro_rules! human_projection {
    ($limit:ty, $text:literal) => {{
        const RENDERED: [u8; $text.len()] = $crate::plane::static_bytes($text);
        const {
            ::core::assert!(
                $text.len() <= <$limit as ::threadpak::types::ConstLimit>::MAX,
                "a static human projection longer than its limit family admits",
            );
        }
        $crate::plane::HumanProjection::<$limit>::proven(RENDERED)
    }};
}

pub(crate) use human_projection;

impl<L: Limit> HumanProjection<L> {
    /// The empty rendering. Total: nothing exceeds any bound, and a caller with
    /// nothing to say for a person still owes a value rather than a hole.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            text: Bounded::empty(),
        }
    }

    /// The rendering's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Whether the rendering carries no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The rendering, for a caller to SHOW a person.
    ///
    /// This is the one lawful use of the bytes and it is a one-way road out of
    /// the plane. Nothing inside the plane calls it: no decision, no identity,
    /// and no refusal consults a human projection, and none ever will. A
    /// frontend that must put a sentence in front of somebody calls this, and
    /// that is what the type exists for.
    #[must_use]
    pub fn shown(&self) -> String {
        let bytes: Vec<u8> = self.text.iter().copied().collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

// ---------------------------------------------------------------------------
// The compiler plane's own identities.
// ---------------------------------------------------------------------------

/// Append one length as eight big-endian bytes.
///
/// The plane's one length framing, used by every canonical encoding it writes.
/// Eight bytes at a fixed width rather than a varint, because a canonical
/// encoding that admitted two spellings of one length would admit two preimages
/// for one value.
pub fn encode_length(length: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(length).unwrap_or(u64::MAX).to_be_bytes());
}

/// Append one length-prefixed byte string: the eight-byte length, then the
/// bytes.
///
/// Every variable-length member of every canonical encoding in the plane is
/// written this way. Without the prefix, two members could be split at a
/// different boundary and encode identically — the classic concatenation
/// collision, which the length prefix removes outright.
pub fn encode_bytes(material: &[u8], into: &mut Vec<u8>) {
    encode_length(material.len(), into);
    into.extend_from_slice(material);
}

/// The closed roster of roles a plane identity may stand for.
///
/// The role is part of the derive-key context AND part of the transcript, so two
/// identities derived from the same anchor under different roles are different
/// identities twice over: they are separated before a byte of the transcript is
/// read, and they disagree inside it. A role that means something else is a law
/// change, not a new string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionRole {
    /// The token material one expansion was handed.
    CapturedDeclaration,
    /// One projection plan.
    Plan,
    /// One node of the origin graph.
    OriginNode,
    /// One generated unit a plan declares it will materialize.
    GeneratedUnit,
    /// One rendered unit a renderer actually materialized.
    RenderedUnit,
    /// The canonical bytes of one rendered unit.
    OutputBytes,
    /// One bundle materialized across a single publication boundary.
    Bundle,
    /// One proved closure between a plan and its rendering.
    Closure,
    /// One closed expansion.
    ClosedExpansion,
}

/// The declared role roster, in the order the plane states it.
pub const PROJECTION_ROLES: [ProjectionRole; 9] = [
    ProjectionRole::CapturedDeclaration,
    ProjectionRole::Plan,
    ProjectionRole::OriginNode,
    ProjectionRole::GeneratedUnit,
    ProjectionRole::RenderedUnit,
    ProjectionRole::OutputBytes,
    ProjectionRole::Bundle,
    ProjectionRole::Closure,
    ProjectionRole::ClosedExpansion,
];

impl ProjectionRole {
    /// The role's position in the declared roster — the byte the transcript
    /// carries for it.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::CapturedDeclaration => 0,
            Self::Plan => 1,
            Self::OriginNode => 2,
            Self::GeneratedUnit => 3,
            Self::RenderedUnit => 4,
            Self::OutputBytes => 5,
            Self::Bundle => 6,
            Self::Closure => 7,
            Self::ClosedExpansion => 8,
        }
    }

    /// The role's declared segment of the derive-key context.
    ///
    /// Declared rather than taken from the Rust spelling, for the same reason
    /// [`IdentitySubject::SUBJECT_NAME`] is: renaming a variant must not rename
    /// every identity derived under it.
    #[must_use]
    pub const fn context_name(self) -> &'static str {
        match self {
            Self::CapturedDeclaration => "captured-declaration",
            Self::Plan => "plan",
            Self::OriginNode => "origin-node",
            Self::GeneratedUnit => "generated-unit",
            Self::RenderedUnit => "rendered-unit",
            Self::OutputBytes => "output-bytes",
            Self::Bundle => "bundle",
            Self::Closure => "closure",
            Self::ClosedExpansion => "closed-expansion",
        }
    }
}

// ---------------------------------------------------------------------------
// The versioned identity profile.
// ---------------------------------------------------------------------------

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

impl IdentityProfileVersion {
    /// The version the profile's authority assigned.
    #[must_use]
    pub const fn declared(position: u32) -> Self {
        Self(position)
    }

    /// The assigned position.
    #[must_use]
    pub const fn position(self) -> u32 {
        self.0
    }
}

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
/// [`ProjectionRole::context_name`]. Every segment is lowercase ASCII letters,
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

impl IdentityProfile {
    /// The profile at one stem and one version.
    #[must_use]
    pub const fn declared(stem: &'static str, version: IdentityProfileVersion) -> Self {
        Self { stem, version }
    }

    /// The declared stem — everything of the context ahead of the version.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        self.stem
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> IdentityProfileVersion {
        self.version
    }

    /// The derive-key context for one subject under one role, spelled by the
    /// grammar above.
    #[must_use]
    pub fn context_for(self, subject: &str, role: ProjectionRole) -> String {
        let version = self.version.position();
        let role = role.context_name();
        format!("{}/v{version}/{subject}/{role}", self.stem)
    }
}

/// The profile every plane identity in this crate is derived under.
pub const PROJECTION_IDENTITY_PROFILE: IdentityProfile = IdentityProfile::declared(
    "threadpak/macroc/projection-identity",
    IdentityProfileVersion::declared(2),
);

// ---------------------------------------------------------------------------
// The generator identity.
// ---------------------------------------------------------------------------

/// The stable name of the generator that derives plane identities.
///
/// A name, not a version: it says WHICH generator, and it changes only when a
/// different generator starts producing this material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorProfileId(&'static str);

impl GeneratorProfileId {
    /// The generator under its declared stable name.
    #[must_use]
    pub const fn declared(spelling: &'static str) -> Self {
        Self(spelling)
    }

    /// The declared name.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.0
    }
}

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

impl GeneratorSchemaVersion {
    /// The schema version the generator's authority assigned.
    #[must_use]
    pub const fn declared(position: u32) -> Self {
        Self(position)
    }

    /// The assigned position.
    #[must_use]
    pub const fn position(self) -> u32 {
        self.0
    }
}

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

impl GeneratorIdentity {
    /// The generator under its declared name, rendered shape, and recorded
    /// package version.
    #[must_use]
    pub const fn declared(
        profile: GeneratorProfileId,
        schema: GeneratorSchemaVersion,
        package: &'static str,
    ) -> Self {
        Self {
            profile,
            schema,
            package,
        }
    }

    /// The generator's stable name. Load-bearing: it is in every transcript.
    #[must_use]
    pub const fn profile(self) -> GeneratorProfileId {
        self.profile
    }

    /// The rendered shape's version. Load-bearing: it is in every transcript.
    #[must_use]
    pub const fn schema(self) -> GeneratorSchemaVersion {
        self.schema
    }

    /// The package version, recorded for a reader and load-bearing nowhere.
    #[must_use]
    pub const fn package_version(self) -> &'static str {
        self.package
    }
}

/// This generator, as every transcript in this crate names it.
pub const MACROC_GENERATOR: GeneratorIdentity = GeneratorIdentity::declared(
    GeneratorProfileId::declared("threadpak-macroc"),
    GeneratorSchemaVersion::declared(1),
    env!("CARGO_PKG_VERSION"),
);

// ---------------------------------------------------------------------------
// The transcript.
// ---------------------------------------------------------------------------

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

impl TranscriptAnchoring {
    /// The discriminant byte the transcript carries for this posture.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Rooted => 0,
            Self::UnderOwnerIdentity(_) => 1,
            Self::UnderProjectionIdentity(_) => 2,
        }
    }

    /// The anchor commitment at full width, where there is one.
    #[must_use]
    pub const fn commitment(&self) -> Option<&[u8; 32]> {
        match self {
            Self::Rooted => None,
            Self::UnderOwnerIdentity(anchor) | Self::UnderProjectionIdentity(anchor) => {
                Some(anchor)
            }
        }
    }
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
/// | 4 | role | `bytes(utf8)` of [`ProjectionRole::context_name`] |
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

impl<'material> ProjectionTranscript<'material> {
    /// Derive under an identity the MACHINE minted.
    #[must_use]
    pub fn under_owner<Subject>(
        role: ProjectionRole,
        anchor: &OwnerIdentityRef<Subject>,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(
            role,
            TranscriptAnchoring::UnderOwnerIdentity(*anchor.as_bytes()),
            content,
            position,
        )
    }

    /// Derive under another identity the PLANE owns.
    #[must_use]
    pub fn under_projection<Subject>(
        role: ProjectionRole,
        anchor: &ProjectionIdentity<Subject>,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(
            role,
            TranscriptAnchoring::UnderProjectionIdentity(*anchor.as_bytes()),
            content,
            position,
        )
    }

    /// Derive under no anchor at all — the root of one derivation chain.
    #[must_use]
    pub fn rooted(role: ProjectionRole, content: &'material [u8], position: u32) -> Self {
        Self::anchored(role, TranscriptAnchoring::Rooted, content, position)
    }

    /// Derive under an anchoring the caller already decided.
    ///
    /// The road for a mint site whose anchor depends on a typed posture rather
    /// than on which of two identity families it holds — a plan hangs off
    /// whatever caused it, and what caused it is a sum type.
    #[must_use]
    pub fn under(
        role: ProjectionRole,
        anchoring: TranscriptAnchoring,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self::anchored(role, anchoring, content, position)
    }

    /// The shared constructor: every transcript names the one declared profile
    /// and the one declared generator, so neither can be varied per call site.
    #[must_use]
    fn anchored(
        role: ProjectionRole,
        anchoring: TranscriptAnchoring,
        content: &'material [u8],
        position: u32,
    ) -> Self {
        Self {
            profile: PROJECTION_IDENTITY_PROFILE,
            generator: MACROC_GENERATOR,
            role,
            anchoring,
            content,
            position,
        }
    }

    /// The profile this transcript is written under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator this transcript names.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// The role this transcript stands for.
    #[must_use]
    pub const fn role(&self) -> ProjectionRole {
        self.role
    }

    /// What this transcript is anchored under.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        self.anchoring
    }

    /// The varying material, at full length.
    #[must_use]
    pub const fn content(&self) -> &'material [u8] {
        self.content
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The transcript's bytes for one identity subject, exactly as the
    /// specification above states them.
    #[must_use]
    pub fn encoded(&self, subject: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(self.profile.stem().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&self.profile.version().position().to_be_bytes());
        encode_bytes(subject.as_bytes(), &mut bytes);
        encode_bytes(self.role.context_name().as_bytes(), &mut bytes);
        bytes.push(self.role.slot());
        bytes.push(self.anchoring.slot());
        match self.anchoring.commitment() {
            Some(anchor) => encode_bytes(anchor, &mut bytes),
            None => encode_bytes(&[], &mut bytes),
        }
        encode_bytes(self.content, &mut bytes);
        bytes.extend_from_slice(&self.position.to_be_bytes());
        encode_bytes(self.generator.profile().spelling().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&self.generator.schema().position().to_be_bytes());
        bytes
    }

    /// The derivation record this transcript leaves for one identity subject.
    #[must_use]
    pub fn provenance(&self, subject: &'static str) -> ProjectionProvenance {
        ProjectionProvenance {
            subject,
            role: self.role,
            profile: self.profile,
            generator: self.generator,
            anchoring: self.anchoring,
            content_length: u64::try_from(self.content.len()).unwrap_or(u64::MAX),
            position: self.position,
        }
    }
}

// ---------------------------------------------------------------------------
// The derivation record.
// ---------------------------------------------------------------------------

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

impl ProjectionProvenance {
    /// The identity subject this derivation named.
    #[must_use]
    pub const fn subject(&self) -> &'static str {
        self.subject
    }

    /// The role it stood for.
    #[must_use]
    pub const fn role(&self) -> ProjectionRole {
        self.role
    }

    /// The profile and version it was derived under.
    #[must_use]
    pub const fn profile(&self) -> IdentityProfile {
        self.profile
    }

    /// The generator that derived it.
    #[must_use]
    pub const fn generator(&self) -> GeneratorIdentity {
        self.generator
    }

    /// What it was anchored under, anchor commitment included.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        self.anchoring
    }

    /// How many bytes of content went into the transcript.
    #[must_use]
    pub const fn content_length(&self) -> u64 {
        self.content_length
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The derive-key context this derivation ran under, rendered by the domain
    /// grammar.
    #[must_use]
    pub fn context(&self) -> String {
        self.profile.context_for(self.subject, self.role)
    }
}

// ---------------------------------------------------------------------------
// The identity.
// ---------------------------------------------------------------------------

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

impl<Subject: IdentitySubject> ProjectionIdentity<Subject> {
    /// Derive one plane identity from its complete transcript. Deterministic
    /// and total: every transcript names an identity.
    #[must_use]
    pub fn derived(transcript: ProjectionTranscript<'_>) -> Self {
        let context = transcript
            .profile()
            .context_for(Subject::SUBJECT_NAME, transcript.role());
        Self {
            bytes: blake3::derive_key(&context, &transcript.encoded(Subject::SUBJECT_NAME)),
            _subject: PhantomData,
        }
    }

    /// Derive one plane identity and the record of how it was derived.
    ///
    /// The record is for whoever is going to keep it. Three values keep theirs:
    /// a plan, a proved closure, and a closed expansion — the three whose
    /// identity a reader is most likely to be handed on its own and asked to
    /// account for. A caller with nowhere to put one takes
    /// [`ProjectionIdentity::derived`] instead, and the record is simply not
    /// made rather than made and carried by everything.
    #[must_use]
    pub fn derived_with_provenance(
        transcript: ProjectionTranscript<'_>,
    ) -> (Self, ProjectionProvenance) {
        (
            Self::derived(transcript),
            transcript.provenance(Subject::SUBJECT_NAME),
        )
    }
}

impl<Subject> ProjectionIdentity<Subject> {
    /// The identity's thirty-two bytes, borrowed for comparison and for
    /// rendering.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
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

// ---------------------------------------------------------------------------
// Rendered roles.
// ---------------------------------------------------------------------------

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

impl RenderedRoleSeal {
    /// The seal, admitted only within the services.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

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

impl RenderedRole for SoleRenderedUnit {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[Self::Sole];

    fn slot(self) -> u32 {
        0
    }

    fn described(self) -> &'static str {
        "the kind's one rendered unit"
    }
}

/// One plane identity minted for the proof surface alone.
///
/// Test-gated on purpose. The laws need distinguishable identities without
/// having a captured declaration to derive them from, and this road exists
/// nowhere else: a production caller derives from a real transcript or has no
/// identity at all.
#[cfg(test)]
pub(crate) fn for_laws<Subject: IdentitySubject>(tag: u8) -> ProjectionIdentity<Subject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        &[tag],
        u32::from(tag),
    ))
}

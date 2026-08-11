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
//! origin nodes, rendered units, generated units, and bundles are the plane's
//! own material: the machine has no opinion about them and mints nothing for
//! them, so the plane names them itself. Every one is derived deterministically
//! from an explicit typed [`ProjectionPreimage`] that is recorded inside the
//! identity, so the question "where did this identity come from?" is answered by
//! reading the value rather than by trusting the producer.
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

/// Declares the plane's subject markers: one zero-sized type per identity
/// subject, each `Eq`/`Hash`/`Copy` so an identity tagged with it composes into
/// the plane's records without hand-written impls.
macro_rules! subjects {
    ($( $(#[$note:meta])* $name:ident ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
        )+
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
    RefusalReason,
    /// One refusal family, named by identity rather than by its Rust spelling.
    RefusalFamilySubject,
    /// One owning semantic home of the machine. The home roster itself belongs
    /// to the machine and is derived by its tooling; the plane names a home by
    /// identity so it never carries a second copy of that roster.
    OwnerHomeSubject,
    /// One exact fact an owning home declares.
    OwnerFactSubject,
    /// One node of the origin graph.
    OriginNodeSubject,
    /// One subject a decision trace entry is about.
    TracedSubject,
    /// One subject a plan explicitly does not claim.
    NonclaimSubject,
    /// One generated unit — the thing a plan declares it will materialize.
    GeneratedUnitSubject,
    /// The canonical bytes of one generated unit.
    OutputBytesSubject,
    /// One projection profile.
    ProjectionProfileSubject,
    /// One version of the services themselves — the generator identity a plan
    /// was produced under.
    GeneratorVersionSubject,
    /// One projection kind, named by identity where a decoded route may name a
    /// kind the plane does not implement.
    ProjectionKindSubject,
    /// One projection plan.
    PlanSubject,
    /// One bundle of plans materialized across a single publication boundary.
    BundleSubject,
    /// One schema the machine's schema home owns.
    SchemaSubject,
    /// One byte role the machine's bytes home owns.
    ByteRoleSubject,
    /// One port declaration.
    PortSubject,
    /// One wire contract a remote surface speaks.
    WireContractSubject,
    /// One declared obligation a test descriptor challenges.
    ObligationSubject,
    /// One measured unit a benchmark descriptor observes.
    MeasuredSubject,
    /// One named work currency a benchmark envelope is stated in.
    WorkCurrencySubject,
    /// One subject a documentation projection documents.
    DocumentedSubject,
    /// One type an implementation projection is derived for.
    DerivedTypeSubject,
    /// One contract an implementation projection realizes.
    ImplementedContractSubject,
    /// One authored pattern.
    PatternSubject,
    /// One instantiation of an authored pattern.
    PatternInstanceSubject,
    /// One typed argument supplied to a pattern instantiation.
    PatternArgumentSubject,
    /// One admitted mechanism profile.
    MechanismProfileSubject,
    /// One declared work formula.
    WorkFormulaSubject,
    /// One fixture population a descriptor ranges over.
    FixturePopulationSubject,
    /// One contract a diagnostic expected to hold.
    ContractSubject,
    /// One related issue a diagnostic points at.
    RelatedIssueSubject,
    /// One callable services entry point.
    ServiceEntrySubject,
    /// One expansion surface of the Rust-facing shell.
    ExpansionSurfaceSubject,
    /// One runtime trace a generated unit corresponds to.
    RuntimeTraceSubject,
    /// One authored declaration template.
    TemplateSubject,
    /// One typed hole a template declares.
    TemplateParameterSubject,
    /// One typed commitment supplied to fill such a hole.
    TemplateArgumentSubject,
    /// One declared symbolic bound formula. The formula itself belongs to the
    /// owner that declared it; the plane names it and never evaluates it.
    BoundFormulaSubject,
    /// One validated input descriptor a meta evaluation ranges over.
    InputDescriptorSubject,
    /// The exact source snapshot one invocation was read against.
    SourceSnapshotSubject,
    /// One language profile — the notation a front door speaks.
    LanguageProfileSubject,
    /// One meta profile — the posture a template evaluation runs under.
    MetaProfileSubject,
    /// One deliberately declared distinctness between otherwise identical
    /// template applications.
    ApplicationDistinctnessSubject,
    /// One declared provider of descriptor material.
    DescriptorProviderSubject,
    /// One captured declaration, as the compiler plane read it. Distinct from
    /// the machine's declaration fragment: the fragment is a linked artifact the
    /// machine owns, while this names exactly the token material one expansion
    /// was handed, before anything was linked at all.
    CapturedDeclarationSubject,
    /// One rendered unit — the thing a renderer actually materialized, as
    /// opposed to the generated unit a plan declared it would.
    RenderedUnitSubject,
    /// One proved closure between a plan's declared membership and the units a
    /// renderer actually produced.
    ClosureSubject,
    /// One closed expansion: the whole receipt one live compilation produced.
    ClosedExpansionSubject,
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
    /// missing-fact issue over its one plan seat, and the bound issue over its
    /// six axes.
    PlanningIssueLimit = 12,
    /// Issues one explanation-coverage refusal body may carry: each of the
    /// fourteen questions may be unanswered, answered twice, or answered where
    /// the kind does not admit it, and no two of those hold of one question at
    /// once.
    ExplanationIssueLimit = 14,
    /// Explanation seats one view may hold — the question roster's cardinality.
    ExplanationSeatLimit = 14,
    /// Bytes one human projection may carry.
    HumanTextLimit = 512,
    /// Related issues one diagnostic may point at.
    RelatedIssueLimit = 16,
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
    /// Bytes one rendered unit may carry. A renderer that would emit past this
    /// refuses rather than materializing part of a unit.
    RenderedByteLimit = 65536,
    /// Tokens one generated token tree may carry at any one nesting level.
    GeneratedTokenLimit = 4096,
    /// Issues one closure refusal body may carry: at most one per planned member
    /// seat plus one per unplanned rendered unit, which is twice the membership
    /// bound.
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

    /// The canonical bytes of this citation, for a preimage to be taken over.
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
    /// It takes text whose length the CALLER already proved against `L::MAX` in
    /// a `const` block, so there is no runtime check here and no refusal to
    /// return. Reaching it without that proof is the one thing the macro exists
    /// to prevent, which is why the seam is crate-internal.
    #[must_use]
    pub(crate) fn proven(text: &'static str) -> Self {
        Self {
            text: Bounded::admitted_const(text.as_bytes().to_vec()).unwrap_or_else(|_| {
                // Unreachable by the macro's compile-time proof; the total road
                // is taken rather than a panic, because a projection that
                // somehow did not fit must still not stop the compiler.
                Bounded::empty()
            }),
        }
    }
}

/// Projects one STATIC rendering, proving at COMPILE TIME that it fits the named
/// limit family.
///
/// This is the total road. `HumanProjection::projected` reads a runtime length
/// and may refuse, and a caller that swallowed that refusal with an empty
/// fallback would be silently deleting an explanation — which is exactly the
/// defect this macro exists to make unrepresentable. Where the material is
/// static, the length is a compile-time fact, so it is proven at compile time
/// and the refusal road never appears.
macro_rules! human_projection {
    ($limit:ty, $text:literal) => {{
        const {
            ::core::assert!(
                $text.len() <= <$limit as ::threadpak::types::ConstLimit>::MAX,
                "a static human projection longer than its limit family admits",
            );
        }
        $crate::plane::HumanProjection::<$limit>::proven($text)
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

/// The closed roster of roles a plane identity may stand for.
///
/// The role is part of the preimage, so two identities derived from the same
/// anchor under different roles are different identities by construction rather
/// than by convention. A role that means something else is a law change, not a
/// new string.
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
    /// The role's position in the declared roster — the byte the preimage
    /// encoding carries for it.
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
}

/// The typed compiler-plane preimage one [`ProjectionIdentity`] is derived from.
///
/// Four recorded facts, and every one of them is readable back off the identity:
///
/// 1. the **role** the identity stands for;
/// 2. the **anchor fold** — the plane's fold over the declared bytes of the
///    identity this one is derived under, whether that is an owner lens or
///    another plane identity;
/// 3. the **content fold** — the plane's fold over the varying material (a
///    family name, a rendered byte sequence, a role word);
/// 4. the **position** the identity holds inside its anchor's declared sequence.
///
/// # What the record does and does not carry
///
/// The role and the position are carried exactly. The anchor and the content are
/// carried as FOLDS and not as themselves, because a preimage rides inside every
/// plan and every refusal body in the plane and neither of those may carry an
/// unbounded value. That is a stated limit of the provenance this record leaves,
/// not a hidden one: reading a preimage back tells you what role, at which
/// position, under a fold of which anchor, over a fold of which content — it
/// hands back neither the anchor nor the content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionPreimage {
    role: ProjectionRole,
    anchor: [u8; 8],
    content: [u8; 8],
    position: u32,
}

impl ProjectionPreimage {
    /// Derive under an identity the MACHINE minted.
    #[must_use]
    pub fn under_owner<Subject>(
        role: ProjectionRole,
        anchor: &OwnerIdentityRef<Subject>,
        content: &[u8],
        position: u32,
    ) -> Self {
        Self {
            role,
            anchor: folded(anchor.as_bytes()),
            content: folded(content),
            position,
        }
    }

    /// Derive under another identity the PLANE owns.
    #[must_use]
    pub fn under_projection<Subject>(
        role: ProjectionRole,
        anchor: &ProjectionIdentity<Subject>,
        content: &[u8],
        position: u32,
    ) -> Self {
        Self {
            role,
            anchor: folded(anchor.as_bytes()),
            content: folded(content),
            position,
        }
    }

    /// Derive under no anchor at all — the root of one plane's derivation chain,
    /// where the content IS the whole preimage. The captured declaration is the
    /// only thing that stands here: everything else in a plan hangs off it.
    #[must_use]
    pub fn rooted(role: ProjectionRole, content: &[u8], position: u32) -> Self {
        Self {
            role,
            anchor: [0u8; 8],
            content: folded(content),
            position,
        }
    }

    /// The role this preimage stands for.
    #[must_use]
    pub const fn role(&self) -> ProjectionRole {
        self.role
    }

    /// The fold over the declared bytes of the identity this preimage is derived
    /// under.
    #[must_use]
    pub const fn anchor(&self) -> &[u8; 8] {
        &self.anchor
    }

    /// The fold over the varying material.
    #[must_use]
    pub const fn content(&self) -> &[u8; 8] {
        &self.content
    }

    /// The position inside the anchor's declared sequence.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// The preimage's canonical byte encoding — role slot, anchor, content fold,
    /// and position, each at a fixed width and in a fixed order.
    fn encoded(&self) -> [u8; 21] {
        let mut encoded = [0u8; 21];
        if let Some(window) = encoded.get_mut(0..1) {
            window.copy_from_slice(&[self.role.slot()]);
        }
        if let Some(window) = encoded.get_mut(1..9) {
            window.copy_from_slice(&self.anchor);
        }
        if let Some(window) = encoded.get_mut(9..17) {
            window.copy_from_slice(&self.content);
        }
        if let Some(window) = encoded.get_mut(17..21) {
            window.copy_from_slice(&self.position.to_be_bytes());
        }
        encoded
    }
}

/// The eight-byte fold of one byte sequence — the width a preimage records an
/// anchor and a content at.
///
/// Eight bytes rather than thirty-two because a preimage rides inside every
/// refusal body in the plane, and the rarest issue must not set the size of
/// every seam. The nonclaim is the fold's own and is not weakened by the
/// narrower width: collision resistance was never claimed at any width.
fn folded(material: &[u8]) -> [u8; 8] {
    let tag = provenance_tag(&[material]);
    let mut narrow = [0u8; 8];
    if let Some(window) = tag.get(0..8) {
        narrow.copy_from_slice(window);
    }
    narrow
}

/// The plane's in-house deterministic byte fold.
///
/// # This is a PROVENANCE TAG, and it is NOT a cryptographic commitment
///
/// The nonclaim is stated here rather than implied: **collision resistance is
/// not claimed.** Two different preimages may in principle fold to one tag, and
/// nothing in the plane treats a tag as evidence that two things are the same
/// thing. A tag exists so that a plane identity is DETERMINISTIC and traceable —
/// the same declared input yields the same identities on every machine and every
/// run — and for nothing else. Where the machine needs a commitment, the machine
/// mints one; the plane never substitutes this for it.
///
/// A real digest for tooling is a mechanism admission the repository owner has
/// not made. Until that admission exists this fold stands, self-contained, with
/// no dependency edge bought for it.
///
/// The fold is four independent lanes of an FNV-1a-shaped mix, each seeded
/// differently and each length-prefixed per part, so a tag is stable under
/// nothing but the exact parts it was taken over.
#[must_use]
pub fn provenance_tag(parts: &[&[u8]]) -> [u8; 32] {
    /// The lane seed.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    /// The lane multiplier.
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    /// The per-lane separation constant.
    const LANE_STEP: u64 = 0x9e37_79b9_7f4a_7c15;

    let mut tag = [0u8; 32];
    for (lane, window) in tag.chunks_exact_mut(8).enumerate() {
        let seed = u64::try_from(lane).unwrap_or(0).wrapping_mul(LANE_STEP);
        let mut state = OFFSET ^ seed;
        for part in parts {
            let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
            state = (state ^ length).wrapping_mul(PRIME);
            for byte in *part {
                state = (state ^ u64::from(*byte)).wrapping_mul(PRIME);
            }
        }
        window.copy_from_slice(&state.to_be_bytes());
    }
    tag
}

/// One identity the COMPILER PLANE owns, tagged by the subject it names.
///
/// # What holding one means
///
/// It means the plane derived this identity from the recorded
/// [`ProjectionPreimage`], deterministically, and would derive the same one
/// again from the same preimage. It means nothing about the machine: the machine
/// mints no plane identity and accepts none.
///
/// # The walls
///
/// 1. **No raw-byte constructor at all.** The only road is
///    [`ProjectionIdentity::derived`], which takes a typed preimage. There is no
///    public or crate-internal seam that wraps arbitrary bytes.
/// 2. **No cross-subject substitution.** `Subject` is a `PhantomData` parameter,
///    so an identity naming one subject is a different type than one naming
///    another regardless of bytes.
/// 3. **No conversion to or from [`OwnerIdentityRef`].** The two families answer
///    different questions and neither is reachable from the other.
/// 4. **No `Ord`.** Plane identities are never ranked.
///
/// # The provenance nonclaim
///
/// The bytes are a [`provenance_tag`], not a commitment. Collision resistance is
/// not claimed — see the fold's own documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionIdentity<Subject> {
    tag: [u8; 32],
    preimage: ProjectionPreimage,
    _subject: PhantomData<Subject>,
}

impl<Subject> ProjectionIdentity<Subject> {
    /// Derive one plane identity from its typed preimage. Deterministic and
    /// total: every preimage names an identity.
    #[must_use]
    pub fn derived(preimage: ProjectionPreimage) -> Self {
        Self {
            tag: provenance_tag(&[&preimage.encoded()]),
            preimage,
            _subject: PhantomData,
        }
    }

    /// The provenance tag. Not a commitment — see [`provenance_tag`].
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.tag
    }

    /// The recorded preimage this identity was derived from.
    #[must_use]
    pub const fn preimage(&self) -> &ProjectionPreimage {
        &self.preimage
    }
}

// ---------------------------------------------------------------------------
// Rendered roles.
// ---------------------------------------------------------------------------

/// The closed roster of rendered units one projection kind materializes.
///
/// A kind declares this roster once, and the closure check reads it: a rendered
/// unit is matched to a planned member by ROLE, so "the family implementation"
/// and "the cause-order implementation" are different seats rather than two
/// entries in an ordered list nobody can tell apart. A rendering that produced
/// the right number of units in the wrong roles is caught by the role, not by a
/// count.
pub trait RenderedRole: Copy + PartialEq + Eq + core::fmt::Debug + Sized + 'static {
    /// The complete roster, in the order the kind states it.
    const ROLES: &'static [Self];

    /// This role's position in the roster. Part of every preimage derived for
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
/// nowhere else: a production caller derives from a real preimage or has no
/// identity at all.
#[cfg(test)]
pub(crate) fn for_laws<Subject>(tag: u8) -> ProjectionIdentity<Subject> {
    ProjectionIdentity::derived(ProjectionPreimage::rooted(
        ProjectionRole::Plan,
        &[tag],
        u32::from(tag),
    ))
}

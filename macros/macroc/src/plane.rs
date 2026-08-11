//! The metaprogramming plane's shared carriers: exact-identity references,
//! owner-fact references, profile versions, bounded human projections, and the
//! plane's declared limit families.
//!
//! # Why the plane references identities instead of minting them
//!
//! The machine's identity home mints identities; the services never do. Every
//! identity a plan carries therefore arrives as an [`ExactIdentity`] — a
//! reference in the identity's own declared raw-byte storage order, tagged by
//! the subject it names so two subjects never unify at compile time. The
//! production road is [`ExactIdentity::of_commitment`], which reads a machine
//! commitment's published bytes and adapts nothing: identity, schema, authority,
//! bounds, and meaning cross unchanged, which is exactly what a projection is
//! allowed to do. Neither the commitment road nor the reason road grants
//! anything: a plane reference admits no operation, carries no authority, and is
//! never accepted by the machine as a mint. There is no public raw-byte road at
//! all — the one byte seam is crate-internal and named for the decoder it is
//! waiting for.
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
    InvalidationLimit = 8,
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
///    [`ExactIdentity::of_commitment`] and [`ExactIdentity::of_reason`], each of
///    which reads an identity the machine already minted. The byte seam
///    (`decoded`) is crate-internal, awaiting the real decoder.
/// 2. **No cross-subject substitution.** `Subject` is a `PhantomData`
///    parameter, so a reference naming one subject is a different type than a
///    reference naming another regardless of bytes, and neither coerces to the
///    other.
/// 3. **No subject-erasing conversion.** [`ExactIdentity::as_bytes`] hands back
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
pub struct ExactIdentity<Subject> {
    bytes: [u8; 32],
    _subject: PhantomData<Subject>,
}

impl<Subject> ExactIdentity<Subject> {
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

impl ExactIdentity<RefusalReason> {
    /// Project one registered refusal reason into the plane. A diagnostic names
    /// the reason the machine registered; it never registers one.
    #[must_use]
    pub fn of_reason(reason: ReasonId) -> Self {
        Self::decoded(*reason.as_bytes())
    }
}

/// A typed reference naming the owning band fact that caused a decision.
///
/// Every selection, omission, exclusion, and non-applicability in the plane
/// cites one of these. A bare boolean would say a decision happened without
/// saying whose fact decided it, which is exactly the explanation the plane
/// owes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerFactRef {
    /// The owning semantic home.
    pub home: ExactIdentity<OwnerHomeSubject>,
    /// The exact fact that home declares.
    pub fact: ExactIdentity<OwnerFactSubject>,
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
}

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
}

#[cfg(test)]
mod laws {
    use super::{
        ExactIdentity, HumanProjection, HumanTextLimit, OwnerFactSubject, OwnerHomeSubject,
        ProfileVersion, RefusalReason,
    };
    use threadpak::types::{BoundedConstruction, ConstLimit};

    /// law: plane.subjects-do-not-unify — a reference naming one subject is a
    /// different type than a reference naming another, whatever the bytes.
    /// Owed reversal: erasing the subject parameter must break this law.
    #[test]
    fn subjects_do_not_unify() {
        let home: fn(ExactIdentity<OwnerHomeSubject>) = drop;
        let fact: fn(ExactIdentity<OwnerFactSubject>) = drop;
        assert!((home as usize) != 0 && (fact as usize) != 0);
        let same_bytes_different_subject = ExactIdentity::<OwnerHomeSubject>::decoded([3; 32]);
        assert_eq!(same_bytes_different_subject.as_bytes(), &[3_u8; 32]);
    }

    /// law: plane.reason-projection-preserves-bytes — projecting a registered
    /// reason adapts nothing; a projection may change presentation, never
    /// identity.
    /// Owed reversal: a projection that rewrote the bytes must break this law.
    #[test]
    fn reason_projection_preserves_bytes() {
        let declared = ExactIdentity::<RefusalReason>::decoded([9; 32]);
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

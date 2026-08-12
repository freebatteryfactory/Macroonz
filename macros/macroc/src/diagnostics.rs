//! What the services say when something disagrees.
//!
//! # One value, many faithful projections
//!
//! A diagnostic is one typed value. The compiler-facing rendering, the
//! machine-readable rendering, and the rendering an agent is handed are
//! projections of that one value — they may differ in shape, ordering, and
//! verbosity, and they may never differ in what they claim. A projection that
//! upgrades a narrowed suspect into an established cause, or a suggestion into
//! an authority, has changed the claim and is not a projection of it.
//!
//! # Repairs are owner-declared, never invented
//!
//! Every [`RepairAction`] cites the owner fact that declares the repair. The
//! services do not compose advice: they report which declared repair applies.
//! And the standing prohibition: no repair ever suggests deleting a declared
//! capability so that generation compiles. Making the machine smaller until the
//! services stop complaining is not a repair, it is a silent narrowing of what
//! the program promised.

use crate::plane::{
    ContractSubject, ExpansionSurfaceSubject, FixturePopulationSubject, HumanProjection,
    HumanTextLimit, OwnerFactRef, OwnerIdentityRef, ProjectionIdentity, RefusalFamilySubject,
    RefusalReason, RelatedIssueLimit, RelatedIssueSubject, RepairLimit, ServiceEntrySubject,
};
use crate::token::{SpanHandle, SpanResolutionRefusal};
use threadpak::declaration::SourceCoordinate;
use threadpak::declaration::types::{FragmentIdentityDomain, LinkedGraphDomain, SymbolDomain};
use threadpak::evidence::CauseDisposition;
use threadpak::evidence::types::ReleaseArtifactDomain;
use threadpak::refusal::StopBound;
use threadpak::types::Bounded;

/// Which act of the services was running when the disagreement was observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacrocPhase {
    /// Capturing the caller's declared input.
    Capture,
    /// Constructing declaration material from it.
    DeclarationConstruction,
    /// Linking that material into a closed graph.
    Linking,
    /// Planning a projection over the closed graph.
    Planning,
    /// Rendering a planned output.
    Rendering,
    /// Inspecting or explaining an existing plan or output.
    Inspection,
}

/// The declared phase roster, in the order the services run them.
pub const MACROC_PHASES: [MacrocPhase; 6] = [
    MacrocPhase::Capture,
    MacrocPhase::DeclarationConstruction,
    MacrocPhase::Linking,
    MacrocPhase::Planning,
    MacrocPhase::Rendering,
    MacrocPhase::Inspection,
];

/// How what was observed differs from the contract that was expected. A typed
/// classification, never a sentence: the sentence is a projection of this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservedClassification {
    /// A required seat was unfurnished.
    SeatAbsent,
    /// What was present disagrees with the expected contract.
    ContractDisagreement,
    /// An identity that had to match did not.
    IdentityDisagreement,
    /// The material was presented under a profile that does not admit it.
    ProfileDisagreement,
    /// A declared magnitude was exceeded.
    BoundExceeded,
    /// Generated material arrived with no origin.
    OriginAbsent,
    /// The route depends on a mechanism no admission covers.
    MechanismUnadmitted,
}

/// One repair the owner declared, projected for a person to read.
///
/// The citation is the load-bearing member. The text is a projection of it, and
/// the services never compose a repair the owner did not declare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepairAction {
    /// The owner fact that declares this repair.
    pub declared_by: OwnerFactRef,
    /// The repair rendered for a person.
    pub description: HumanProjection<HumanTextLimit>,
}

/// How to reach the same observation again. A diagnostic that cannot be
/// reproduced is a report about one run, not about the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReproductionRoute {
    /// Call the services directly at this entry point — the route that needs no
    /// proc-macro at all.
    CallableServices {
        /// The entry point.
        entry: ProjectionIdentity<ServiceEntrySubject>,
    },
    /// Expand through the Rust-facing shell's surface.
    ExpansionShell {
        /// The expansion surface.
        surface: ProjectionIdentity<ExpansionSurfaceSubject>,
    },
    /// Replay against a recorded fixture population.
    RecordedFixture {
        /// The recorded population.
        population: ProjectionIdentity<FixturePopulationSubject>,
    },
}

/// Whether the subject of a diagnostic stands under a release promise.
///
/// Not a rank and not a maturity score: either an artifact was released and is
/// named, or no release promise covers this at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReleasePosture {
    /// A released artifact covers this subject.
    UnderReleasePromise {
        /// The exact released artifact.
        artifact: OwnerIdentityRef<ReleaseArtifactDomain>,
    },
    /// No release promise covers this subject.
    NoReleasePromise,
}

/// The machine's own identities for one observation.
///
/// Every seat here names something the machine minted. A caller that holds them
/// supplies them; nothing in the plane derives one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineAnchors {
    /// The registered reason, as the machine's refusal home published it.
    pub reason: OwnerIdentityRef<RefusalReason>,
    /// The refusal family that owns the reason.
    pub family: OwnerIdentityRef<RefusalFamilySubject>,
    /// The declaring symbol.
    pub declaration: OwnerIdentityRef<SymbolDomain>,
    /// The declaration fragment involved.
    pub fragment: OwnerIdentityRef<FragmentIdentityDomain>,
    /// The closed graph the observation was made against.
    pub graph: OwnerIdentityRef<LinkedGraphDomain>,
}

/// Whether one diagnostic is anchored in the machine's own identities.
///
/// # Not a hole, and not an optional seat
///
/// A diagnostic raised where the caller holds the machine's identities carries
/// them exactly. A diagnostic raised INSIDE AN EXPANSION does not: at that seam
/// nothing has been linked, no fragment exists, no graph exists, and no reason
/// has been registered to the compiler plane. The honest answer is to say so.
///
/// The plane refuses the alternative. Minting a stand-in "reason identity" or a
/// stand-in "graph identity" would be creating a second value that independently
/// answers a question the machine owns — exactly what a deriver may never do.
/// So the seat states the posture instead, and a reader can tell an anchored
/// diagnostic from an unanchored one without reading anything else.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MachineAnchoring {
    /// The machine's identities, as the caller held them. Boxed because a
    /// diagnostic travels by value and the anchored posture must not set the
    /// size of the unanchored one.
    Anchored(Box<MachineAnchors>),
    /// No machine identity stands here, because none exists at this seam yet.
    /// The plane names the posture and mints nothing to fill the seat.
    UnmintedAtThisSeam,
}

/// Where one diagnostic's token sits, or why the producer's table could not
/// say.
///
/// Two postures, exactly as [`MachineAnchoring`] carries two, and for the same
/// reason: a seat that cannot be furnished states the posture rather than being
/// filled with a stand-in. A coordinate written where a table did not reach
/// would read exactly like a coordinate the table resolved, and the reader has
/// no third value to compare it against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SiteCoordinate {
    /// The position the producer's table resolved, in the role it speaks.
    Resolved(SourceCoordinate),
    /// The producer's table does not reach this handle. The refusal states the
    /// handle and how far the table reaches, so a reader can tell a mismatched
    /// table from a truncated one.
    NotReached(SpanResolutionRefusal),
}

impl SiteCoordinate {
    /// The posture one span table's answer takes.
    #[must_use]
    pub const fn answered(answer: Result<SourceCoordinate, SpanResolutionRefusal>) -> Self {
        match answer {
            Ok(coordinate) => Self::Resolved(coordinate),
            Err(refusal) => Self::NotReached(refusal),
        }
    }

    /// The resolved coordinate, where the table reached the handle.
    #[must_use]
    pub const fn resolved(self) -> Option<SourceCoordinate> {
        match self {
            Self::Resolved(coordinate) => Some(coordinate),
            Self::NotReached(_) => None,
        }
    }
}

/// Where one diagnostic points.
///
/// Two seats, and the first is the load-bearing one. The **token handle** names
/// the offending token in the producer's own span table, so whoever produced the
/// input can put a compiler error on exactly that token rather than on the first
/// token of the declaration. The **coordinate** is that position rendered in
/// whatever coordinate role the producer speaks — a byte offset where the input
/// was read from text, and the handle's own index where the producer holds the
/// compiler's spans itself — or the typed statement that this producer's table
/// does not reach the handle at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticSite {
    /// The offending token.
    pub token: SpanHandle,
    /// Where that token sits, in the producer's coordinate role.
    pub coordinate: SiteCoordinate,
}

/// Whether one diagnostic's related set carries every established issue's own
/// identity, or stopped at the declared related-issue magnitude.
///
/// The vocabulary is band 00's, the one every collection-shaped refusal body in
/// the plane already reports its enumeration with: `EarlyStopped`, naming the
/// [`StopBound`] that stopped it. What a SET adds to an enumeration is the count
/// — a reader holding the complete body's identity alone otherwise cannot tell a
/// one-issue refusal from a sixty-issue one, and a set that carried the body's
/// identity silently would be a coarser commitment wearing the shape of a
/// complete one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelatedSetCompletion {
    /// The complete body's identity, then one per established issue, all of
    /// them.
    Complete,
    /// The set stopped at a declared bound. The complete body's identity is
    /// carried — it commits to every issue at once — and this many per-issue
    /// identities are not.
    EarlyStopped {
        /// The declared bound that stopped the set.
        stopped_at: StopBound,
        /// How many per-issue identities the set does not carry.
        omitted: usize,
    },
}

/// One diagnostic from the services.
///
/// Every seat is required. A diagnostic that could omit its phase, its site, its
/// expected contract, or its cause posture would be a diagnostic that sometimes
/// says less than it knows, and the shape forbids it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacrocDiagnostic {
    /// Whether the machine's own identities stand behind this observation.
    pub machine: MachineAnchoring,
    /// The act that was running.
    pub phase: MacrocPhase,
    /// Where the observation sits, and which token it is about.
    pub site: DiagnosticSite,
    /// The one line this diagnostic projects for a person.
    ///
    /// Composed inside the services, where the typed value it projects lives,
    /// so no frontend ever writes a sentence of its own. It is a projection and
    /// only a projection: nothing in the plane reads it back, and a frontend
    /// SHOWS it rather than deciding from it.
    pub summary: HumanProjection<HumanTextLimit>,
    /// The contract that was expected to hold. A compiler-plane contract: the
    /// plane states what it expected of the material it read.
    pub expected: ProjectionIdentity<ContractSubject>,
    /// How what was found differs from it.
    pub observed: ObservedClassification,
    /// The machine's cause posture: an established cause, narrowed suspects, or
    /// unresolved. Narrowing is progress, never a forced verdict.
    pub cause: CauseDisposition,
    /// Other issues this one points at.
    pub related: Bounded<ProjectionIdentity<RelatedIssueSubject>, RelatedIssueLimit>,
    /// Whether that set names every established issue, or stopped at a declared
    /// bound and says how many it does not name.
    pub related_completion: RelatedSetCompletion,
    /// The owner-declared repairs that apply.
    pub repairs: Bounded<RepairAction, RepairLimit>,
    /// How to reach this observation again.
    pub reproduction: ReproductionRoute,
    /// Whether a release promise covers the subject.
    pub release: ReleasePosture,
}

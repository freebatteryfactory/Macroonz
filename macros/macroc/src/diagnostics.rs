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
    ContractSubject, ExactIdentity, ExpansionSurfaceSubject, FixturePopulationSubject,
    HumanProjection, HumanTextLimit, OwnerFactRef, RefusalFamilySubject, RefusalReason,
    RelatedIssueLimit, RelatedIssueSubject, RepairLimit, ServiceEntrySubject,
};
use threadpak::declaration::SourceCoordinate;
use threadpak::declaration::types::{FragmentIdentityDomain, LinkedGraphDomain, SymbolDomain};
use threadpak::evidence::CauseDisposition;
use threadpak::evidence::types::ReleaseArtifactDomain;
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
        entry: ExactIdentity<ServiceEntrySubject>,
    },
    /// Expand through the Rust-facing shell's surface.
    ExpansionShell {
        /// The expansion surface.
        surface: ExactIdentity<ExpansionSurfaceSubject>,
    },
    /// Replay against a recorded fixture population.
    RecordedFixture {
        /// The recorded population.
        population: ExactIdentity<FixturePopulationSubject>,
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
        artifact: ExactIdentity<ReleaseArtifactDomain>,
    },
    /// No release promise covers this subject.
    NoReleasePromise,
}

/// One diagnostic from the services.
///
/// Every seat is required. A diagnostic that could omit its phase, its
/// coordinate, its expected contract, or its cause posture would be a
/// diagnostic that sometimes says less than it knows, and the shape forbids it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacrocDiagnostic {
    /// The registered reason, as the machine's refusal home published it.
    pub reason: ExactIdentity<RefusalReason>,
    /// The refusal family that owns the reason.
    pub family: ExactIdentity<RefusalFamilySubject>,
    /// The act that was running.
    pub phase: MacrocPhase,
    /// Where in the source the observation sits, under its declared coordinate
    /// role.
    pub coordinate: SourceCoordinate,
    /// The declaring symbol.
    pub declaration: ExactIdentity<SymbolDomain>,
    /// The declaration fragment involved.
    pub fragment: ExactIdentity<FragmentIdentityDomain>,
    /// The closed graph the observation was made against.
    pub graph: ExactIdentity<LinkedGraphDomain>,
    /// The contract that was expected to hold.
    pub expected: ExactIdentity<ContractSubject>,
    /// How what was found differs from it.
    pub observed: ObservedClassification,
    /// The machine's cause posture: an established cause, narrowed suspects, or
    /// unresolved. Narrowing is progress, never a forced verdict.
    pub cause: CauseDisposition,
    /// Other issues this one points at.
    pub related: Bounded<ExactIdentity<RelatedIssueSubject>, RelatedIssueLimit>,
    /// The owner-declared repairs that apply.
    pub repairs: Bounded<RepairAction, RepairLimit>,
    /// How to reach this observation again.
    pub reproduction: ReproductionRoute,
    /// Whether a release promise covers the subject.
    pub release: ReleasePosture,
}

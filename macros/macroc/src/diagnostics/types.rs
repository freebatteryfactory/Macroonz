//! The diagnostics home's declarations: the act that was running, how what was
//! observed differs from what was expected, where it sits, what repairs it, and
//! how to reach it again.
//!
//! Declarations only. The home's README is the one place the readable-but-not-
//! writable distinction is argued and the one place the guard's ownership is
//! stated; this file is where that ruling lands, in the two private seats below
//! and in the child declaration on the last line. Each of those seats carries its
//! own reason at its own declaration, which is the reason for that seat rather
//! than a second copy of the home's narrative.

use crate::plane::{
    ContractSubject, ExpansionSurfaceSubject, FixturePopulationSubject, HumanProjection,
    HumanTextLimit, OwnerFactRef, OwnerIdentityRef, ProjectionIdentity, RefusalFamilySubject,
    RefusalReason, RelatedIssueLimit, RelatedIssueSubject, RepairLimit, ServiceEntrySubject,
};
use crate::token::{SpanHandle, SpanResolutionRefusal};
use core::num::NonZeroUsize;
use threadpak::declaration::SourceCoordinate;
use threadpak::declaration::types::{FragmentIdentityDomain, LinkedGraphDomain, SymbolDomain};
use threadpak::evidence::CauseDisposition;
use threadpak::evidence::types::ReleaseArtifactDomain;
use threadpak::refusal::StopBound;
use threadpak::types::Bounded;

#[path = "type_guard.rs"]
mod guard;

threadpak::closed_register! {
    /// Which act of the services was running when the disagreement was
    /// observed.
    ///
    /// `ALL` is the roster in the order the services run the phases, and `slot`
    /// is that order read back as a position.
    pub enum MacrocPhase {
        /// Capturing the caller's declared input.
        Capture = "capture", "capturing the caller's declared input";
        /// Constructing declaration material from it.
        DeclarationConstruction = "declaration-construction",
            "constructing declaration material from the captured input";
        /// Linking that material into a closed graph.
        Linking = "linking", "linking declaration material into a closed graph";
        /// Planning a projection over the closed graph.
        Planning = "planning", "planning a projection over the closed graph";
        /// Rendering a planned output.
        Rendering = "rendering", "rendering a planned output";
        /// Inspecting or explaining an existing plan or output.
        Inspection = "inspection", "inspecting or explaining an existing plan or output";
    }
}

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

/// How many per-issue identities one truncated related set does not carry.
///
/// Opaque, with public readers and no public mint. The count is a fact about an
/// act — the set-building road ran out of declared magnitude and left identities
/// behind — and a seat a caller could write would be a count with no act behind
/// it. The one road that mints this reads the number off the material it
/// actually dropped, so what a reader acts on is what happened rather than what
/// somebody asserted.
///
/// It takes band 00's shape for the same reason it takes band 00's vocabulary:
/// `ReportTruncation` there is opaque for exactly this reason, and a tooling
/// value reporting the same kind of fact under a weaker discipline would be the
/// weaker statement that keeps passing after the stronger one is gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelatedSetTruncation {
    stopped_at: StopBound,
    omitted: NonZeroUsize,
}

/// Whether one diagnostic's related set carries every established issue's own
/// identity, or was truncated at the declared related-issue magnitude.
///
/// The vocabulary is band 00's, the one every collection-shaped refusal body in
/// the plane reports its coverage with, and it takes band 00's distinction
/// exactly: nothing here ever HALTS an examination. The body is complete before
/// the set is built, so what a short set reports is a truncated REPORT and never
/// an unexamined remainder — which is why the truncation spelling stands here
/// and `EarlyStopped` does not. What a SET adds is the count: a reader holding
/// the complete body's identity alone otherwise cannot tell a one-issue refusal
/// from a sixty-issue one, and a set that carried the body's identity silently
/// would be a coarser commitment wearing the shape of a complete one.
///
/// The truncated posture carries an opaque [`RelatedSetTruncation`] rather than
/// a bound and a number, because the pair written as fields is a pair a caller
/// can write. A set that carried every identity could then report that sixty
/// were dropped, and nothing in the type would notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelatedSetCompletion {
    /// The complete body's identity, then one per established issue, all of
    /// them.
    Complete,
    /// The set was truncated at a declared bound. The complete body's identity
    /// is carried — it commits to every issue at once — and the truncation names
    /// the bound and how many per-issue identities are not there.
    ReportTruncated(RelatedSetTruncation),
}

/// One diagnostic's related set: the identities it carries, married to whether
/// that is all of them.
///
/// The two are one value for band 00's reason. A completion is a claim ABOUT a
/// set, and a claim about a set that can be carried away from it is a claim that
/// can be told about a different one — a diagnostic could then wear the coarser
/// set of one refusal under the completion of another, with both halves honest
/// on their own and the pair a lie. So the set-building road is the only road
/// in, the seats are private, and there is no road back out to a loose pair.
///
/// The identities inside take the same rule one level down. The road is handed
/// the issue MATERIAL and derives the body's identity and the per-issue
/// identities together, so the body's identity is a commitment to exactly the
/// issues beside it. Taking the two levels as two arguments would be the same
/// lie in smaller print: each identity derives honestly, and the pair names one
/// refusal's body over another refusal's issues.
///
/// It takes the same shape band 00's [`threadpak::refusal::AdmittedPrefix`]
/// takes, because it reports the same kind of fact about the same kind of act. A
/// tooling value reporting a truncation under a weaker discipline would be the
/// weaker statement that keeps passing after the stronger one is gone.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelatedSet {
    carried: Bounded<ProjectionIdentity<RelatedIssueSubject>, RelatedIssueLimit>,
    completion: RelatedSetCompletion,
}

/// One diagnostic from the services.
///
/// Every seat is required. A diagnostic that could omit its phase, its site, its
/// expected contract, or its cause posture would be a diagnostic that sometimes
/// says less than it knows, and the shape forbids it.
#[must_use = "a diagnostic carries the observation, its site, and the owner-declared repair"]
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
    /// Other issues this one points at, and whether that set names every
    /// established issue or stopped at a declared bound and says how many it
    /// does not name. One seat rather than two: the completion belongs to the
    /// set it was built beside and to no other.
    pub related: RelatedSet,
    /// The owner-declared repairs that apply.
    pub repairs: Bounded<RepairAction, RepairLimit>,
    /// How to reach this observation again.
    pub reproduction: ReproductionRoute,
    /// Whether a release promise covers the subject.
    pub release: ReleasePosture,
}

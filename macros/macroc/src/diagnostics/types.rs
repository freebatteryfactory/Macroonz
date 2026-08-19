//! The diagnostics home's declarations: the act that was running, how what was
//! observed differs from what was expected, where it sits, what repairs it, how
//! to reach it again, and the magnitude a related set is bounded by.
//!
//! Declarations only.
//! Every seat of a diagnostic is readable; the two that a caller may not write
//! are private below, and the roads that reach them live in `type_guard.rs`,
//! this file's own child.
//! The site's two postures are declared here as one sum, and the roads that
//! build and read it live in that same child — including the one place a
//! pre-capture byte becomes an answered coordinate.

use crate::plane::{
    ContractSubject, ExpansionSurfaceSubject, FixturePopulationSubject, HumanProjection,
    HumanTextLimit, OwnerFactRef, OwnerIdentityRef, ProjectionIdentity, RefusalFamilySubject,
    RefusalReason, RelatedBodySubject, RelatedIssueSubject, RepairLimit, ServiceEntrySubject,
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

// ---------------------------------------------------------------------------
// The magnitude.
//
// This home's own row, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on the row below
// are this home's, declared beside the capacity it governs.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many related issue identities one diagnostic
    /// may point at.
    ///
    /// # Bounds
    ///
    /// Sixty-four. A diagnostic projects a refusal body ISSUE FOR ISSUE, so this
    /// magnitude is how much of a body one report enumerates before it falls
    /// back to the body's own coarser commitment — a narrower one would make the
    /// projection drop established issues out of ordinary bodies.
    ///
    /// # Nonclaims
    ///
    /// It is this home's own family and is read off no refusal family's
    /// magnitude. The services declare issue bodies wider than it, so a body
    /// that outruns this set is a case the road MEETS rather than one the
    /// magnitudes rule out: the body's own identity is carried alone with the
    /// count that stands outside, stated rather than silently shortened.
    RelatedIssueLimit = 64,
}

threadpak::closed_register! {
    /// Which act of the services was running when the disagreement was
    /// observed.
    ///
    /// # Ordering
    ///
    /// The roster is declared in the order the services run these phases.
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

/// How what was observed differs from the contract that was expected.
///
/// A typed classification, never a sentence: the sentence is a projection of
/// this.
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
/// The citation is the load-bearing member.
/// The text is a projection of it, and the services never compose a repair the
/// owner did not declare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepairAction {
    /// The owner fact that declares this repair.
    pub declared_by: OwnerFactRef,
    /// The repair rendered for a person.
    pub description: HumanProjection<HumanTextLimit>,
}

/// How to reach the same observation again.
///
/// A diagnostic that cannot be reproduced is a report about one run, not about
/// the machine.
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
/// Every seat here names something the machine minted.
/// A caller that holds them supplies them; nothing in the plane derives one.
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
/// A diagnostic raised where the caller holds the machine's identities carries
/// them exactly.
/// A diagnostic raised inside an expansion does not: at that seam nothing has
/// been linked, no fragment exists, no graph exists, and no reason has been
/// registered to the compiler plane.
///
/// # Nonclaims
///
/// The unanchored posture is a stated answer, not a hole and not an optional
/// seat.
/// Minting a stand-in reason or graph identity would create a second value
/// independently answering a question the machine owns, so the seat states the
/// posture instead and a reader tells the two apart without reading anything
/// else.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MachineAnchoring {
    /// The machine's identities, as the caller held them.
    /// Boxed because a diagnostic travels by value and the anchored posture
    /// must not set the size of the unanchored one.
    Anchored(Box<MachineAnchors>),
    /// No machine identity stands here, because none exists at this seam yet.
    /// The plane names the posture and mints nothing to fill the seat.
    UnmintedAtThisSeam,
}

/// Where one diagnostic's token sits, or why the producer's table could not
/// say.
///
/// A seat that cannot be furnished states the posture rather than being filled
/// with a stand-in, as [`MachineAnchoring`] does: a coordinate written where a
/// table did not reach would read exactly like a coordinate the table resolved,
/// and the reader has no third value to compare it against.
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
/// Two arms, and they are different observations rather than one with a missing
/// half — the same split the derive home's own refusal site states one level
/// down, landed here so a diagnostic can carry it.
///
/// A diagnostic about a CAPTURED declaration names the offending token in the
/// producer's own span table, so whoever produced the input can put a compiler
/// error on exactly that token rather than on the first token of the
/// declaration; the coordinate beside it is that position rendered in whatever
/// coordinate role the producer speaks, or the typed statement that this
/// producer's table does not reach the handle at all.
///
/// A diagnostic established BEFORE any capture has no token to name: no table
/// was built, no handle was issued, and there is nothing for a handle to index.
/// What such an observation has is the byte it was born at, and that is the
/// whole of what its arm carries.
///
/// # Nonclaims
///
/// **The pre-capture arm mints no handle, and that is the substitution this sum
/// removes.** A required handle seat forces handle zero onto an observation that
/// issued none, and handle zero reads exactly like an honest answer pointing at
/// the first token of the declaration.
///
/// It carries no [`SiteCoordinate`] either: a table that was never built cannot
/// have failed to reach anything, so the arm carries a plain
/// [`SourceCoordinate`] and [`DiagnosticSite::coordinate`] is the one place that
/// byte is lifted into the answered posture.
#[must_use = "a diagnostic site names the token it points at, or the byte it was born at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSite {
    /// One token of a captured declaration, and where the producer's table put
    /// it.
    AtToken {
        /// The offending token, as a handle into the producer's own span table.
        token: SpanHandle,
        /// Where that token sits, in the producer's coordinate role — or the
        /// typed statement that the table does not reach the handle.
        coordinate: SiteCoordinate,
    },
    /// One byte of the text a read refused on, before any capture existed to
    /// issue a handle.
    BeforeCapture {
        /// The byte the observation was born at, in the role its own text
        /// counts in.
        coordinate: SourceCoordinate,
    },
}

/// How many per-issue identities one truncated related set does not carry.
///
/// Opaque, with public readers and no public mint.
/// The count is a fact about an act — the set-building road ran out of declared
/// magnitude and left identities behind — so a seat a caller could write would
/// be a count with no act behind it.
/// The one road that mints this reads the number off the material it actually
/// dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelatedSetTruncation {
    stopped_at: StopBound,
    omitted: NonZeroUsize,
}

/// Whether one diagnostic's related set carries every established issue's own
/// identity, or was truncated at the declared related-issue magnitude.
///
/// Nothing here ever halts an examination: the body is complete before the set
/// is built, so what a short set reports is a truncated report and never an
/// unexamined remainder.
/// What a set adds is the count — a reader holding the complete body's identity
/// alone cannot tell a one-issue refusal from a sixty-issue one, and a set that
/// carried the body's identity silently would be a coarser commitment wearing
/// the shape of a complete one.
///
/// The truncated posture carries an opaque [`RelatedSetTruncation`] rather than
/// a bound and a number, because a pair written as fields is a pair a caller
/// can write: a set that carried every identity could then report that sixty
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

/// One identity a related set carries, at the level it is about.
///
/// A related set commits at two levels over one material: the whole body, which
/// is a commitment to every established issue at once, and each established
/// issue on its own.
/// They are two types rather than two positions, because position is not a fact
/// a reader can check and because one subject over two levels collides by
/// construction — the body's preimage is the framing of its issues, so an issue
/// whose own material happened to be that framing would derive the byte-for-byte
/// identity of the body it aliased.
///
/// Split, the two levels cannot substitute twice over.
/// A body identity is a different Rust type than an issue identity, so seating
/// one where the other belongs does not compile; and the two subjects declare
/// different segments of the derive-key context, so the same preimage bytes at
/// the two levels derive unrelated identities rather than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelatedIdentity {
    /// The whole refusal body, as one commitment to every issue it established.
    Body(ProjectionIdentity<RelatedBodySubject>),
    /// One established issue, on its own.
    Issue(ProjectionIdentity<RelatedIssueSubject>),
}

/// One diagnostic's related set: the identities it carries, married to whether
/// that is all of them.
///
/// A completion is a claim about a set, and a claim that can be carried away
/// from its set is a claim that can be told about a different one — a
/// diagnostic could then wear the coarser set of one refusal under the
/// completion of another, with both halves honest on their own and the pair a
/// lie.
/// So the set-building road is the only road in, the seats are private, and
/// there is no road back out to a loose pair.
///
/// The identities inside take the same rule one level down.
/// The road is handed the issue material and derives the body's identity and
/// the per-issue identities together, so the body's identity is a commitment to
/// exactly the issues beside it.
/// Taking the two levels as two arguments would be the same lie in smaller
/// print: each identity derives honestly, and the pair names one refusal's body
/// over another refusal's issues.
///
/// # Ordering
///
/// The two levels are carried as [`RelatedIdentity`], which names which level
/// each one is, and the body rides first.
/// A reader does not depend on that: an identity states its own level, so a set
/// read out of order still says which commitment is which.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelatedSet {
    carried: Bounded<RelatedIdentity, RelatedIssueLimit>,
    completion: RelatedSetCompletion,
}

/// One diagnostic from the services.
///
/// Every seat is required.
/// A diagnostic that could omit its phase, its site, its expected contract, or
/// its cause posture would be a diagnostic that sometimes says less than it
/// knows, and the shape forbids it.
#[must_use = "a diagnostic carries the observation, its site, and the owner-declared repair"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacrocDiagnostic {
    /// Whether the machine's own identities stand behind this observation.
    pub machine: MachineAnchoring,
    /// The act that was running.
    pub phase: MacrocPhase,
    /// Where the observation sits, and which token it is about where a capture
    /// issued one to be about.
    pub site: DiagnosticSite,
    /// The one line this diagnostic projects for a person.
    ///
    /// Composed inside the services, where the typed value it projects lives,
    /// so no frontend ever writes a sentence of its own.
    /// A projection and only a projection: nothing in the plane reads it back,
    /// and a frontend shows it rather than deciding from it.
    pub summary: HumanProjection<HumanTextLimit>,
    /// The contract that was expected to hold: what the plane expected of the
    /// material it read.
    pub expected: ProjectionIdentity<ContractSubject>,
    /// How what was found differs from it.
    pub observed: ObservedClassification,
    /// The machine's cause posture: an established cause, narrowed suspects, or
    /// unresolved.
    /// Narrowing is progress, never a forced verdict.
    pub cause: CauseDisposition,
    /// Other issues this one points at, and whether that set names every
    /// established issue or stopped at a declared bound.
    /// One seat rather than two: the completion belongs to the set it was built
    /// beside and to no other.
    pub related: RelatedSet,
    /// The owner-declared repairs that apply.
    pub repairs: Bounded<RepairAction, RepairLimit>,
    /// How to reach this observation again.
    pub reproduction: ReproductionRoute,
    /// Whether a release promise covers the subject.
    pub release: ReleasePosture,
}

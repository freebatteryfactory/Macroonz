//! The assembly home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! central claim structural rather than reviewed. Proved cargo is read HERE, off
//! a terminal's own partition and compared against what that partition carries,
//! so a value carrying tokens nobody proved is not a value anybody can hold —
//! and the reading road is CRATE-INTERNAL, so the envelope those tokens ride in
//! is the source-owning road's declaration rather than any caller's. An
//! assembly is built HERE, after the whole verification agreed, so there is no
//! half-verified whole for a renderer to mistake for a verified one. A joined
//! expansion is built HERE, over two terminals that were both bound, so a
//! door-level value naming a carrier it never rendered is unwritable. And the
//! complete account is built HERE, over a joined value and the roster of
//! dispositions one door decided, so an account seating another door's answers
//! beside these terminals is unwritable on the same terms.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared beside
//! the rest of this home's declarations would put all of them inside the same
//! wall. That module's entire content is the record and its inherent
//! implementations, so the module IS the complete set of roads that reach the
//! private seat.
//!
//! # Nonclaims
//!
//! A private seat excludes every SIBLING — `types.rs` above it, `establish.rs`
//! and `render.rs` beside it, anywhere else in the services, and any crate
//! downstream — and the compiler says so with `E0451`. It does not exclude
//! DESCENDANTS, so the reversal for these seats is a compile-fail fixture
//! testpak owns.

use super::super::establish::{consumption_issues, root_issues};
use super::{
    AccountedExpansion, AssemblyIssue, AxisCargo, CargoAxis, JoinedExpansion, ProvedCargo,
    SupportAssembly,
};
use crate::closure::{ClosedExpansion, PartitionCargo};
use crate::plane::{ClosedExpansionId, OutputBytesSubject, ProjectionIdentity};
use crate::planning::{
    CauseAnchoring, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, EmissionPartition,
    ExpectedGeneratedSupportSchemaId, KindDispositions, ProjectionDisposition, ProjectionKind,
    ProjectionKindRow, TestDescriptorProjection,
};
use crate::test_descriptor::{DeferredCargo, TrialTablePayload};

pub use seat::CarrierAssembly;

mod seat {
    use super::super::{AssemblyIssue, AssemblyIssueLimit};
    use crate::plane::AuthoringLimitProfile;
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The carrier-assembly refusal family body.
    ///
    /// Independent members: one set of outputs may stand under two roots AND
    /// read one terminal's partition twice in the same pass, and reporting one
    /// of them would leave a caller repairing a carrier one disagreement per
    /// attempt.
    #[must_use = "a refusal family body carries every way the outputs did not compose"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct CarrierAssembly {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue the pass
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private for the second half of the same claim: a PUBLIC seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one pass could write it into another
        /// pass's refusal. Read back through [`CarrierAssembly::body`].
        body: AdmittedPrefix<AssemblyIssue, AssemblyIssueLimit>,
    }

    impl CarrierAssembly {
        /// The one-issue body, for a road whose checks can establish exactly one
        /// issue.
        ///
        /// Total: the declared bound admits an item by compile-time proof, so
        /// refusing never needs an error road of its own.
        ///
        /// Crate-internal: a body exists only where one of this home's own
        /// roads established the issue it carries.
        pub(crate) fn established(issue: AssemblyIssue) -> Self {
            Self {
                body: AdmittedPrefix::carrying_one(issue),
            }
        }

        /// The several-issue body, for the verification pass whose checks
        /// co-establish.
        ///
        /// The caller arrives holding every issue the pass established, so the
        /// posture here is about the REPORT and never about the pass: where the
        /// issues fit the declared bound the body carries all of them, and where
        /// they do not the body carries what the bound holds and names how many
        /// established issues stand outside it. Never a silent drop.
        pub(crate) fn co_established(first: AssemblyIssue, rest: Vec<AssemblyIssue>) -> Self {
            Self {
                body: AdmittedPrefix::examined_completely(
                    first,
                    rest,
                    &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                    StopBound::DeclaredIssueBound,
                ),
            }
        }

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<AssemblyIssue, AssemblyIssueLimit> {
            &self.body
        }
    }
}

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for the verification pass, so no pass can establish issues and then
/// walk on past them.
fn refused(issues: Vec<AssemblyIssue>) -> Option<CarrierAssembly> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(CarrierAssembly::co_established(
        first,
        established.collect(),
    ))
}

impl ProvedCargo {
    /// Read one axis's cargo off the terminal that proved it.
    ///
    /// # What this road establishes, and why it takes no tree of its own
    ///
    /// The caller says WHICH terminal and WHICH partition, and hands the cargo
    /// the rendering home composed — the local subject the copies stand over,
    /// the selections they read, and the tokens. This road then reads that
    /// terminal's own partition and refuses unless the tokens are the ones it
    /// proved. So a value of this type carries a parentage it was checked
    /// against rather than one it was told, and the source identity and the
    /// proved digest are recorded by the same act that read them.
    ///
    /// The ROOT is read off the terminal's own entry account, never supplied:
    /// what a terminal was planned over is the plan's answer, and a root handed
    /// in beside the terminal would be a second account of it.
    ///
    /// # Errors
    ///
    /// Returns [`AssemblyIssue::CargoReachesASecondDestination`] where the named
    /// partition is not the carrier partition this axis delivers from — the
    /// declaration-site partition in particular, whose units the consumer's
    /// normal build already compiles.
    ///
    /// Returns [`AssemblyIssue::CargoNotTheSourcesOwn`] where that partition
    /// carries nothing at all, where the terminal has no joined cargo for it,
    /// and where the tokens handed in are not the tokens it proved.
    ///
    /// The two checks are DEPENDENT — there is no cargo to compare until the
    /// partition is the axis's own — so exactly one of them is ever established,
    /// which is why this road answers with one issue rather than a body.
    ///
    /// # Who may promote
    ///
    /// **Crate-internal, because promotion to proved cargo belongs to the road
    /// that owns the source's rendering vocabulary.** The tree this road checks
    /// is authenticated against the terminal that proved it; the ENVELOPE around
    /// it — the local subject the copies stand over, and the selectors they read
    /// their active points through — is not, and cannot be: those spellings are
    /// declarations of the home that rendered the copies, and no terminal
    /// carries them to be compared against. A public road here would therefore
    /// let any caller wrap proved tokens in an envelope of its own choosing and
    /// hand back a value whose whole claim is that its contents are one
    /// terminal's own.
    ///
    /// So the one lawful promotion point is
    /// [`evaluation_axis`](crate::derive_refusal::evaluation_axis), the road that
    /// declares the subject and the selectors it then hands in. The deferred
    /// cargo the envelope is built from stays PUBLIC and is unaffected: it is a
    /// declaration value, refused seat by seat at the carrier's own door, and
    /// holding one claims nothing about any terminal.
    ///
    /// # The opening condition
    ///
    /// A generic deferred-envelope contract is a stated OPENING CONDITION rather
    /// than a gap. It opens when a second projection family needs to transport
    /// independently declared envelope metadata around proved carrier tokens —
    /// at which point the envelope has an owner beside the rendering home, and
    /// the contract that admits it is that owner's to state. Until then the
    /// specific road is what is true, and a public promotion point would be a
    /// vehicle standing for declarations nobody makes.
    pub(crate) fn carried<K: ProjectionKind>(
        expansion: &ClosedExpansion<K>,
        axis: CargoAxis,
        partition: EmissionPartition,
        cargo: DeferredCargo,
    ) -> Result<Self, CarrierAssembly> {
        let source = expansion.identity();
        if axis.delivers_from() != Some(partition) {
            return Err(CarrierAssembly::established(
                AssemblyIssue::CargoReachesASecondDestination { axis, partition },
            ));
        }
        let proved = expansion.emission().joined(partition);
        let Some(PartitionCargo::Carried(carried)) = proved else {
            return Err(CarrierAssembly::established(
                AssemblyIssue::CargoNotTheSourcesOwn { source, partition },
            ));
        };
        if carried.tree() != cargo.tree() {
            return Err(CarrierAssembly::established(
                AssemblyIssue::CargoNotTheSourcesOwn { source, partition },
            ));
        }
        Ok(Self {
            source,
            root: expansion.plan().account().commitment(),
            partition,
            digest: carried.digest(),
            cargo,
        })
    }

    /// The terminal this cargo was proved by.
    #[must_use]
    pub const fn source(&self) -> ClosedExpansionId {
        self.source
    }

    /// The root that terminal was planned over.
    #[must_use]
    pub const fn root(&self) -> CauseAnchoring {
        self.root
    }

    /// The partition it was read from.
    #[must_use]
    pub const fn partition(&self) -> EmissionPartition {
        self.partition
    }

    /// The digest that terminal's proof committed to over exactly these bytes.
    #[must_use]
    pub const fn digest(&self) -> ProjectionIdentity<OutputBytesSubject> {
        self.digest
    }

    /// The cargo itself: the local subject, the selections, and the tokens.
    pub const fn cargo(&self) -> &DeferredCargo {
        &self.cargo
    }
}

impl SupportAssembly {
    /// Assemble one carrier out of closed outputs.
    ///
    /// # What is verified here, and what was verified before
    ///
    /// Each carried axis's cargo was already proved to be its own terminal's by
    /// this home's crate-internal promotion road (`ProvedCargo::carried`, which
    /// the source-owning road walks), which is why no tree reaches this. What
    /// remains are the facts about the WHOLE: one root under every axis, one
    /// published expectation for the gate, every terminal's partition consumed
    /// once, and a bench axis whose vehicle is not yet open.
    ///
    /// # Errors
    ///
    /// Returns [`CarrierAssembly`] carrying every disagreement the pass
    /// established, together: an assembly failing on two roots and a doubled
    /// consumption at once is repaired in one attempt rather than two.
    ///
    /// The issues are [`AssemblyIssue::SchemaExpectationNotPublished`] where the
    /// gate would be pinned against an expectation these services do not
    /// publish, [`AssemblyIssue::RootsDisagree`] naming each axis whose terminal
    /// stands elsewhere, [`AssemblyIssue::CargoConsumedTwice`] naming a
    /// terminal's partition two axes read, and
    /// [`AssemblyIssue::BenchVehicleNotOpen`] where the bench axis carries
    /// material the published grammar has no seat for.
    pub fn assembled(
        root: CauseAnchoring,
        expectation: ExpectedGeneratedSupportSchemaId,
        trial: AxisCargo<TrialTablePayload>,
        evaluation: AxisCargo<ProvedCargo>,
        bench: AxisCargo<ProvedCargo>,
    ) -> Result<Self, CarrierAssembly> {
        let mut issues: Vec<AssemblyIssue> = Vec::new();

        // One published expectation. The comparison is against the services' own
        // checked-in constant at full width rather than against a posture: the
        // pin the shell writes is what a consumer's gate matches, so an
        // expectation minted beside the published one is a shell carrying a pin
        // no publication act wrote.
        if expectation.as_bytes() != EXPECTED_GENERATED_SUPPORT_SCHEMA_ID.as_bytes() {
            issues.push(AssemblyIssue::SchemaExpectationNotPublished {
                stated: *expectation.as_bytes(),
            });
        }

        let carried: Vec<(CargoAxis, &ProvedCargo)> = [
            (CargoAxis::Evaluation, &evaluation),
            (CargoAxis::Bench, &bench),
        ]
        .into_iter()
        .filter_map(|(axis, held)| match held {
            AxisCargo::Absent { .. } => None,
            AxisCargo::Carried(proved) => Some((axis, proved)),
        })
        .collect();

        issues.extend(root_issues(root, &carried));
        issues.extend(consumption_issues(&carried));

        // The bench axis's carried arm is typed and its vehicle is a stated
        // opening condition: the published grammar writes a trials seat and a
        // deferred seat, and neither is the bench seat. Refusing here names the
        // condition; rendering it into one of the two other seats would deliver
        // bench material to a target that does not run it.
        if matches!(bench, AxisCargo::Carried(_)) {
            issues.push(AssemblyIssue::BenchVehicleNotOpen);
        }

        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }
        Ok(Self {
            root,
            expectation,
            trial,
            evaluation,
            bench,
        })
    }

    /// The one root every carried axis stands under.
    #[must_use]
    pub const fn root(&self) -> CauseAnchoring {
        self.root
    }

    /// The published expectation the carrier's gate is pinned against.
    pub const fn expectation(&self) -> &ExpectedGeneratedSupportSchemaId {
        &self.expectation
    }

    /// What the trials axis carries, or what happened to the projection that
    /// would have filled it.
    pub const fn trial(&self) -> &AxisCargo<TrialTablePayload> {
        &self.trial
    }

    /// What the evaluation axis carries, on the same terms.
    pub const fn evaluation(&self) -> &AxisCargo<ProvedCargo> {
        &self.evaluation
    }

    /// What the bench axis carries, on the same terms.
    ///
    /// A carried bench axis never reaches this reader, because the assembly it
    /// would stand in refuses: the seat exists so the day the bench seat is
    /// declared, the material has a home to arrive in rather than a shape
    /// somebody adds under pressure.
    pub const fn bench(&self) -> &AxisCargo<ProvedCargo> {
        &self.bench
    }

    /// Every terminal this assembly carries cargo from, in axis-roster order.
    ///
    /// The identities the shell's rendered bytes commit to: the bytes are
    /// rendered from THIS value and from nothing else, so what an exported
    /// carrier delivers is what these terminals proved.
    pub fn sources(&self) -> impl Iterator<Item = ClosedExpansionId> + '_ {
        [&self.evaluation, &self.bench]
            .into_iter()
            .filter_map(|held| match held {
                AxisCargo::Absent { .. } => None,
                AxisCargo::Carried(proved) => Some(proved.source()),
            })
    }
}

impl<Projected> JoinedExpansion<Projected> {
    /// Bind one joined expansion: the kind's own terminal, the carrier terminal
    /// that delivers its cargo, and the assembly that joined them.
    ///
    /// Crate-internal, because a door is what joins: a caller that could write
    /// this literal could seat a carrier beside a projected terminal it was
    /// never assembled from, and every reading downstream would answer correctly
    /// about the wrong pair.
    pub(crate) fn joined(
        projected: Projected,
        carrier: ClosedExpansion<TestDescriptorProjection>,
        assembly: SupportAssembly,
    ) -> Self {
        Self {
            projected,
            carrier,
            assembly,
        }
    }

    /// The kind's own terminal, whole.
    ///
    /// Read through, never restated: what it emitted, what it planned, and what
    /// it proved are its own answers, and a seat here that repeated one of them
    /// would be a second answer to a question this value already answers.
    #[must_use]
    pub const fn projected(&self) -> &Projected {
        &self.projected
    }

    /// The carrier terminal — the closed expansion whose one member is the
    /// exported support shell.
    pub const fn carrier(&self) -> &ClosedExpansion<TestDescriptorProjection> {
        &self.carrier
    }

    /// The verified assembly the carrier was rendered from.
    pub const fn assembly(&self) -> &SupportAssembly {
        &self.assembly
    }

    /// What the carrier terminal expands into at the declaration site: the
    /// exported shell definition.
    ///
    /// The carrier's OWN proved partition, read off the terminal. The projected
    /// terminal's declaration-site cargo is read off the projected terminal, and
    /// the two are what a door emits — never a third value joining them, which
    /// would be a stream neither proof committed to.
    pub const fn carrier_declaration_site(&self) -> &PartitionCargo {
        self.carrier.declaration_site()
    }
}

impl<Projected> AccountedExpansion<Projected> {
    /// Bind one door road's complete account: what it produced, and what it says
    /// happened to every kind of the sealed roster.
    ///
    /// Crate-internal, on the terms [`JoinedExpansion::joined`] states and for
    /// the same reason one step further out: a caller that could write this
    /// literal could seat a roster of dispositions beside terminals they were
    /// never decided over, and every reading downstream would answer correctly
    /// about the wrong door.
    pub(crate) fn accounted(
        joined: JoinedExpansion<Projected>,
        dispositions: KindDispositions,
    ) -> Self {
        Self {
            joined,
            dispositions,
        }
    }

    /// What this door PRODUCED: both terminals, and the assembly that joined
    /// them.
    ///
    /// Read through, never restated. What each terminal planned, proved, and
    /// emits, and what the carrier delivers, are that value's own answers —
    /// [`JoinedExpansion::projected`], [`JoinedExpansion::carrier`],
    /// [`JoinedExpansion::assembly`], and
    /// [`JoinedExpansion::carrier_declaration_site`] — and a seat here that
    /// repeated one of them would be a second answer to a question this value
    /// already answers.
    pub const fn joined(&self) -> &JoinedExpansion<Projected> {
        &self.joined
    }

    /// What happened to one kind's projection at this door, over this surface.
    ///
    /// Total over the sealed roster: every row has exactly one answer, and a
    /// generated row's answer names the one output a disposition names.
    pub const fn disposition(&self, kind: ProjectionKindRow) -> &ProjectionDisposition {
        self.dispositions.under(kind)
    }

    /// Which delivery one kind's cargo landed in, where that kind produced any.
    ///
    /// The planned member's own destination, read to the emission it belongs to
    /// through the one road a destination reads to
    /// ([`MemberDestination::partition`]) — so what a reader is told about where
    /// a kind's cargo went is what the join, the proof, and a consumption target
    /// were told.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing for a kind that produced nothing, and that is a
    /// stated posture rather than a missing value: a kind whose disposition is an
    /// absence landed nowhere, and the disposition beside it says which absence
    /// it was. It claims nothing about the OTHER members a generated kind's plan
    /// declared — a disposition names one output, and the membership is where a
    /// reader asking what was materialized reads.
    ///
    /// [`MemberDestination::partition`]: crate::planning::MemberDestination::partition
    #[must_use]
    pub fn landed(&self, kind: ProjectionKindRow) -> Option<EmissionPartition> {
        match self.disposition(kind) {
            ProjectionDisposition::Generated { output } => Some(output.destination.partition()),
            ProjectionDisposition::NotApplicable { .. }
            | ProjectionDisposition::Refused { .. }
            | ProjectionDisposition::UnavailableUnderProfile { .. }
            | ProjectionDisposition::NotRequested
            | ProjectionDisposition::ExcludedByConfiguration { .. } => None,
        }
    }

    /// Every kind this door generated, in roster order.
    ///
    /// The rows whose disposition says GENERATED, and nothing else: which
    /// terminal each of them ended at is read off [`AccountedExpansion::joined`],
    /// because a terminal is what a door produced and a row is what it said about
    /// it.
    pub fn generated(&self) -> impl Iterator<Item = ProjectionKindRow> + '_ {
        ProjectionKindRow::ALL.iter().copied().filter(|row| {
            matches!(
                self.disposition(*row),
                ProjectionDisposition::Generated { .. }
            )
        })
    }
}

//! The pattern-stamp home's invariant nucleus: every road that reaches a private
//! field, and the one road that composes the published artifact.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! claims structural rather than remembered. A type path is parsed HERE, so a
//! segment the machine's compiler would read as something else is not a value
//! anybody can hold. A coverage's two namespaces are closed HERE, so a published
//! stamp that would seat one family twice, or collide two seats inside one home,
//! is refused before a token exists. And the artifact is composed HERE, so there
//! is no half-rendered publication unit for a reader to mistake for a whole one.
//!
//! # One alphabet, and this home does not own it
//!
//! Every spelling this home writes as a Rust identifier is admitted by the one
//! alphabet the wall's carrier already publishes
//! ([`is_rendered_identifier`](crate::test_descriptor::is_rendered_identifier)).
//! A second copy would agree with that one until one of them was edited, and the
//! failure would surface as a name nobody can trace.

use super::super::render;
use super::{
    CoupledSeatDeclaration, InsufficiencyGround, PublishedSeatStamp, SeatDeclarationLimit,
    SeatDeclarationRefusal, SeatMint, SeatNames, SeatPath, SeatPathSegmentLimit, SeatProse,
    SeatSeating, SeatVisibility, StampCoverage, StampName, StampPublicationRecord,
    StampRenderIssue, StampedSeat, StampedUnitPlan,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, ByteRoleSubject, GeneratedUnitSubject, OwnerIdentityRef, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::DigestContract;
use crate::test_descriptor::is_rendered_identifier;
use crate::token::GeneratedTree;
use std::collections::BTreeSet;
use threadpak::types::{NonEmptyBounded, PositiveLimit};

impl SeatPath {
    /// The path one seat names, parsed from the segments the caller stated.
    ///
    /// # Errors
    ///
    /// Returns [`SeatDeclarationRefusal::PathSegmentsAbsent`] where no segment
    /// was supplied — a path naming nothing names no type —
    /// [`SeatDeclarationRefusal::SpellingNotAnIdentifier`] where a segment is not
    /// one Rust identifier, and
    /// [`SeatDeclarationRefusal::PathSegmentsUnbounded`] where the segments
    /// outgrow the declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true
    /// of any refused path.
    pub fn spelled(segments: Vec<String>) -> Result<Self, SeatDeclarationRefusal> {
        let mut supplied = segments.into_iter();
        let Some(first) = supplied.next() else {
            return Err(SeatDeclarationRefusal::PathSegmentsAbsent);
        };
        let rest: Vec<String> = supplied.collect();
        if !is_rendered_identifier(first.as_str()) {
            return Err(SeatDeclarationRefusal::SpellingNotAnIdentifier);
        }
        for segment in &rest {
            if !is_rendered_identifier(segment.as_str()) {
                return Err(SeatDeclarationRefusal::SpellingNotAnIdentifier);
            }
        }
        let admitted: NonEmptyBounded<String, SeatPathSegmentLimit> =
            NonEmptyBounded::admitted_const(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
            )
            .map_err(|_| SeatDeclarationRefusal::PathSegmentsUnbounded)?;
        Ok(Self { segments: admitted })
    }

    /// The segments, in the order they were stated; structurally at least one.
    pub fn segments(&self) -> impl Iterator<Item = &String> {
        self.segments.iter()
    }

    /// How many segments the path carries; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Always `false`: a path naming no segment is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

impl SeatNames {
    /// The two spellings one seat is written under.
    ///
    /// # Errors
    ///
    /// Returns [`SeatDeclarationRefusal::SpellingNotAnIdentifier`] where either
    /// spelling is not one Rust identifier. The family is read before the module,
    /// so exactly one cause is true of any refused pair.
    pub fn named(family: &str, home: &str) -> Result<Self, SeatDeclarationRefusal> {
        if !is_rendered_identifier(family) {
            return Err(SeatDeclarationRefusal::SpellingNotAnIdentifier);
        }
        if !is_rendered_identifier(home) {
            return Err(SeatDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            family: family.to_owned(),
            home: home.to_owned(),
        })
    }

    /// The refusal family's own spelling.
    #[must_use]
    pub fn family(&self) -> &str {
        self.family.as_str()
    }

    /// The module the seat is seated in.
    #[must_use]
    pub fn home(&self) -> &str {
        self.home.as_str()
    }
}

impl CoupledSeatDeclaration {
    /// Declare one coupled seat.
    ///
    /// Total: every argument is a value whose own road already settled what it
    /// promises — the spellings are identifiers, the paths are non-empty and
    /// bounded, the reach and the mint are closed rosters, and the prose is the
    /// caller's words. There is nothing left to read and therefore no error
    /// branch for a caller to fill.
    #[must_use]
    pub const fn declared(
        names: SeatNames,
        issue: SeatPath,
        bound: SeatPath,
        reach: SeatVisibility,
        mint: SeatMint,
        prose: SeatProse,
    ) -> Self {
        Self {
            names,
            issue,
            bound,
            reach,
            mint,
            prose,
        }
    }

    /// The two spellings this seat is written under.
    #[must_use]
    pub const fn names(&self) -> &SeatNames {
        &self.names
    }

    /// The issue roster the seat's body carries.
    #[must_use]
    pub const fn issue(&self) -> &SeatPath {
        &self.issue
    }

    /// The magnitude the roster is bounded by.
    #[must_use]
    pub const fn bound(&self) -> &SeatPath {
        &self.bound
    }

    /// The reach the seat is re-exported at.
    #[must_use]
    pub const fn reach(&self) -> SeatVisibility {
        self.reach
    }

    /// Which mint road the seat asks for.
    #[must_use]
    pub const fn mint(&self) -> &SeatMint {
        &self.mint
    }

    /// The prose the seat is documented with.
    #[must_use]
    pub const fn prose(&self) -> &SeatProse {
        &self.prose
    }
}

impl StampName {
    /// The name one published stamp is exported under.
    ///
    /// # Errors
    ///
    /// Returns [`SeatDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// spelling is not one Rust identifier.
    pub fn declared(spelling: &str) -> Result<Self, SeatDeclarationRefusal> {
        if !is_rendered_identifier(spelling) {
            return Err(SeatDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            spelling: spelling.to_owned(),
        })
    }

    /// The exported spelling a home invokes this stamp by.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl StampCoverage {
    /// Declare the complete payload one published stamp is rendered from.
    ///
    /// # Errors
    ///
    /// Returns [`SeatDeclarationRefusal::SeatsAbsent`] where no seat was
    /// supplied — a published stamp nobody invokes is an artifact with no reader
    /// — [`SeatDeclarationRefusal::FamilySpellingDoubled`] and
    /// [`SeatDeclarationRefusal::HomeSpellingDoubled`] where two seats name one
    /// family or one module, and [`SeatDeclarationRefusal::SeatsUnbounded`] where
    /// the seats outgrow the declared magnitude.
    ///
    /// The namespace checks run before the magnitude check because a collision is
    /// a defect in what was declared, and a caller repairing a magnitude first
    /// would repair the collision second.
    pub fn declared(
        stamp: StampName,
        seats: Vec<CoupledSeatDeclaration>,
    ) -> Result<Self, SeatDeclarationRefusal> {
        let mut supplied = seats.into_iter();
        let Some(first) = supplied.next() else {
            return Err(SeatDeclarationRefusal::SeatsAbsent);
        };
        let rest: Vec<CoupledSeatDeclaration> = supplied.collect();
        seat_namespaces_closed(&first, &rest)?;
        let admitted: NonEmptyBounded<CoupledSeatDeclaration, SeatDeclarationLimit> =
            NonEmptyBounded::admitted_const(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
            )
            .map_err(|_| SeatDeclarationRefusal::SeatsUnbounded)?;
        Ok(Self {
            stamp,
            seats: admitted,
        })
    }

    /// The name the stamp this coverage is for is exported under.
    #[must_use]
    pub const fn stamp(&self) -> &StampName {
        &self.stamp
    }

    /// The seats covered, in the order they were declared; structurally at least
    /// one.
    ///
    /// # Ordering
    ///
    /// This order IS meaning for the migration: one invocation is rendered per
    /// seat in the order this yields, so the same seats supplied in another order
    /// produce the same artifact and a different walk.
    pub fn seats(&self) -> impl Iterator<Item = &CoupledSeatDeclaration> {
        self.seats.iter()
    }

    /// The coverage read as manifest rows: the two spellings of each covered
    /// seat.
    ///
    /// A projection of the seats above and never a second list, so a manifest row
    /// and the seat it is about cannot disagree.
    pub fn stamped_seats(&self) -> impl Iterator<Item = StampedSeat> + '_ {
        self.seats.iter().map(StampedSeat::of)
    }

    /// How many seats this coverage declares; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.seats.len()
    }

    /// Always `false`: a coverage declaring no seat is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.seats.is_empty()
    }
}

impl StampedSeat {
    /// The manifest row one declared seat amounts to.
    ///
    /// Read off the declaration rather than stated beside it, which is what makes
    /// a manifest a projection of the coverage instead of a claim about it.
    #[must_use]
    pub fn of(declared: &CoupledSeatDeclaration) -> Self {
        Self {
            family: declared.names().family().to_owned(),
            home: declared.names().home().to_owned(),
        }
    }

    /// The refusal family the stamp seats.
    #[must_use]
    pub fn family(&self) -> &str {
        self.family.as_str()
    }

    /// The module that seat lands in.
    #[must_use]
    pub fn home(&self) -> &str {
        self.home.as_str()
    }
}

impl SeatSeating {
    /// The seat this landing is for.
    #[must_use]
    pub const fn seat(&self) -> &StampedSeat {
        &self.seat
    }

    /// The invocation the migration writes at that seat's home.
    #[must_use]
    pub const fn invocation(&self) -> &GeneratedTree {
        &self.invocation
    }
}

impl StampPublicationRecord {
    /// Why neither of the lighter roads can express this output.
    #[must_use]
    pub const fn ground(&self) -> InsufficiencyGround {
        self.ground
    }

    /// The planned member this artifact materializes — what the machine's own
    /// receipt will anchor its publication unit to.
    #[must_use]
    pub const fn unit(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.unit
    }

    /// What the eventual staged bytes' digest must satisfy.
    #[must_use]
    pub const fn staged(&self) -> DigestContract {
        self.staged
    }

    /// What the unit contains, row by row.
    pub fn manifest(&self) -> impl Iterator<Item = StampedSeat> + '_ {
        self.coverage.stamped_seats()
    }

    /// The coverage the artifact was rendered from, whole.
    #[must_use]
    pub const fn covered(&self) -> &StampCoverage {
        &self.coverage
    }
}

impl PublishedSeatStamp {
    /// Render one published stamp over what the plan decided, what the caller
    /// declared, and why the lighter roads are insufficient.
    ///
    /// The order is the road: the exported definition first, then one invocation
    /// per covered seat, then the record — and the artifact only after all three,
    /// so no half-rendered publication unit exists.
    ///
    /// # Errors
    ///
    /// Returns [`StampRenderIssue::StampTreeUnbounded`] where the definition's
    /// tokens, one invocation's tokens, or the tree either is assembled into
    /// outgrow the declared token magnitude. The whole artifact refuses rather
    /// than materializing the seats that happened to fit: a publication unit is
    /// landed whole or not at all.
    pub fn rendered(
        stated: &StampedUnitPlan,
        coverage: &StampCoverage,
        ground: InsufficiencyGround,
    ) -> Result<Self, StampRenderIssue> {
        let definition = GeneratedTree::assembled(render::stamp_definition(coverage.stamp())?)
            .map_err(|_| render::unbounded())?;
        let mut seatings: Vec<SeatSeating> = Vec::new();
        for declared in coverage.seats() {
            let invocation =
                GeneratedTree::assembled(render::seat_invocation(coverage.stamp(), declared)?)
                    .map_err(|_| render::unbounded())?;
            seatings.push(SeatSeating {
                seat: StampedSeat::of(declared),
                invocation,
            });
        }
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            byte_role: stated.byte_role,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin.clone(),
            definition,
            seatings,
            record: StampPublicationRecord {
                ground,
                unit: stated.semantic_key,
                staged: stated.digest_contract,
                coverage: coverage.clone(),
            },
        })
    }

    /// The rendered role this artifact stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
        self.role
    }

    /// The planned member's semantic key this artifact answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
    }

    /// The byte role the artifact is written under.
    #[must_use]
    pub const fn byte_role(&self) -> OwnerIdentityRef<ByteRoleSubject> {
        self.byte_role
    }

    /// The profile the plan expected to render it.
    #[must_use]
    pub const fn profile(&self) -> ProjectionIdentity<ProjectionProfileSubject> {
        self.profile
    }

    /// That profile's version.
    #[must_use]
    pub const fn profile_version(&self) -> ProfileVersion {
        self.profile_version
    }

    /// The trail this artifact walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The name the stamp is exported under, read out of the record's coverage.
    #[must_use]
    pub const fn name(&self) -> &StampName {
        self.record.covered().stamp()
    }

    /// The published definition: the exported `macro_rules!` the publication road
    /// lands in the machine as visible source.
    #[must_use]
    pub const fn definition(&self) -> &GeneratedTree {
        &self.definition
    }

    /// Every covered seat's landing, in coverage order.
    ///
    /// # Bounds
    ///
    /// Exactly as many as the coverage declared, because the road that built them
    /// walked that coverage once. The count is the coverage's own fact — already
    /// admitted under its declared magnitude — and not a second magnitude this
    /// road settles.
    pub fn seatings(&self) -> impl Iterator<Item = &SeatSeating> {
        self.seatings.iter()
    }

    /// How many seatings this artifact carries; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.seatings.len()
    }

    /// Always `false`: an artifact covering no seat is unrepresentable, because
    /// the coverage it is rendered from is structurally non-empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.seatings.is_empty()
    }

    /// This side's record of the publication act, in the machine receipt's field
    /// shape.
    #[must_use]
    pub const fn record(&self) -> &StampPublicationRecord {
        &self.record
    }
}

/// The two namespaces one published stamp's coverage closes.
///
/// A family seated twice would put one refusal family in two modules, and two
/// seats naming one module would collide inside a home as a duplicate definition.
/// Both are refused here rather than left to the machine's own compiler, which
/// would report a collision inside an expansion nobody wrote.
///
/// The families are read before the modules, so exactly one cause is true of any
/// refused coverage.
fn seat_namespaces_closed(
    first: &CoupledSeatDeclaration,
    rest: &[CoupledSeatDeclaration],
) -> Result<(), SeatDeclarationRefusal> {
    let mut families: BTreeSet<&str> = BTreeSet::new();
    for declared in core::iter::once(first).chain(rest.iter()) {
        if !families.insert(declared.names().family()) {
            return Err(SeatDeclarationRefusal::FamilySpellingDoubled);
        }
    }
    let mut homes: BTreeSet<&str> = BTreeSet::new();
    for declared in core::iter::once(first).chain(rest.iter()) {
        if !homes.insert(declared.names().home()) {
            return Err(SeatDeclarationRefusal::HomeSpellingDoubled);
        }
    }
    Ok(())
}

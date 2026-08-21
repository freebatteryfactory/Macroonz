//! The benchmark-descriptor home's invariant nucleus: every road that reaches a
//! private field, and the composition that produces a shell only where every part
//! of it was rendered.
//!
//! Declared inside `types.rs` as its own child. A work formula is encoded HERE,
//! so a formula carrying no bytes is not a value anybody can hold. An axis is
//! admitted HERE, so a growth class is never read off a single point. A backend
//! is named HERE, so the adapter's one swap point cannot be handed a spelling
//! that is not a Rust identifier. And a shell is composed HERE, so there is no
//! half-rendered carrier for a reader to mistake for a whole one.
//!
//! # The refusal family is the carrier's
//!
//! Rendering refuses in [`ShellRendering`], which the test-descriptor home
//! declares beside the carrier both crossings ride. A second body for the same
//! question would be a second answer to "what magnitude did this rendering
//! overrun", and the two crossings hit the same token roster under the same
//! declared magnitude.

use super::super::render;
use super::{
    BenchAttachment, BenchBackend, BenchDeclarationRefusal, BenchMeasurement, BenchReferences,
    BenchReporterAdapter, BenchRow, BenchRowLimit, BenchTablePayload, BenchmarkPlan,
    BenchmarkShell, InputSizeLimit, WorkFormula, WorkFormulaLimit, WorkObservationLimit,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, GeneratedUnitSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::MemberDestination;
use crate::test_descriptor::{
    BoundPath, ShellName, ShellRenderIssue, ShellRendering, WallName, expectation_literal,
    exported_shell, gate_invocation, is_rendered_identifier, unbounded,
};
use crate::token::GeneratedTree;
use std::collections::BTreeSet;
use threadpak::types::{AdmittedLimit, Bounded, NonEmptyBounded, PositiveLimit};

/// The backend this home renders as the CURRENT one-file choice.
///
/// Named here rather than compiled in anywhere: it is the value
/// [`BenchBackend::current`] hands back, the adapter reads its spelling from that
/// value alone, and a consumer swapping backends replaces the value rather than
/// editing a rendering.
const CURRENT_BACKEND: &str = "divan";

impl WorkFormula {
    /// One declared work formula, over the declaration's own encoded bytes.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::WorkFormulaEmpty`] where no bytes were
    /// supplied — an operation that declares no formula states that by carrying
    /// none, so an empty one is a formula nobody wrote — and
    /// [`BenchDeclarationRefusal::WorkFormulaUnbounded`] where the bytes outgrow
    /// the declared magnitude.
    pub fn encoded(bytes: Vec<u8>) -> Result<Self, BenchDeclarationRefusal> {
        if bytes.is_empty() {
            return Err(BenchDeclarationRefusal::WorkFormulaEmpty);
        }
        let encoded: Bounded<u8, WorkFormulaLimit> = Bounded::admitted_const(
            bytes,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| BenchDeclarationRefusal::WorkFormulaUnbounded)?;
        Ok(Self { encoded })
    }

    /// The formula's encoded bytes, in the order the declaration wrote them.
    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        self.encoded.iter()
    }

    /// How many bytes the formula carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.encoded.len()
    }
}

impl BenchAttachment {
    /// What makes one row measurable: the three callables the host order invokes,
    /// and the work observations it reads.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::WorkObservationsUnbounded`] where the
    /// observations outgrow the declared magnitude. The three callables are
    /// required by the signature, so a row that would be benchmarked without its
    /// two gates is unrepresentable rather than refused.
    pub fn measuring(
        measured: BoundPath,
        planted_worse: BoundPath,
        preflight: BoundPath,
        observations: Vec<BoundPath>,
    ) -> Result<Self, BenchDeclarationRefusal> {
        let admitted: Bounded<BoundPath, WorkObservationLimit> = Bounded::admitted_const(
            observations,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| BenchDeclarationRefusal::WorkObservationsUnbounded)?;
        Ok(Self {
            measured,
            planted_worse,
            preflight,
            observations: admitted,
        })
    }

    /// The work observations this row reads, in the order they were declared.
    pub fn observations(&self) -> impl Iterator<Item = &BoundPath> {
        self.observations.iter()
    }
}

impl BenchRow {
    /// Declare one bench row.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::SpellingNotAnIdentifier`] where the lens
    /// spelling is not one Rust identifier,
    /// [`BenchDeclarationRefusal::AxisNotACurve`] where the axis states fewer than
    /// two sizes — a growth class is read off a curve and never off a point —
    /// [`BenchDeclarationRefusal::AxisSizeDoubled`] where two positions state one
    /// size, and [`BenchDeclarationRefusal::AxisUnbounded`] where the axis
    /// outgrows the declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true of
    /// any refused row.
    pub fn declared(
        lens: &str,
        references: BenchReferences,
        axis: Vec<u64>,
        measurement: BenchMeasurement,
        attachment: BenchAttachment,
    ) -> Result<Self, BenchDeclarationRefusal> {
        if !is_rendered_identifier(lens) {
            return Err(BenchDeclarationRefusal::SpellingNotAnIdentifier);
        }
        if axis.len() < 2 {
            return Err(BenchDeclarationRefusal::AxisNotACurve);
        }
        let distinct: BTreeSet<&u64> = axis.iter().collect();
        if distinct.len() != axis.len() {
            return Err(BenchDeclarationRefusal::AxisSizeDoubled);
        }
        let admitted: Bounded<u64, InputSizeLimit> = Bounded::admitted_const(
            axis,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| BenchDeclarationRefusal::AxisUnbounded)?;
        Ok(Self {
            lens: lens.to_owned(),
            references,
            axis: admitted,
            measurement,
            attachment,
        })
    }

    /// The lens spelling the rendered adapter registers this row under.
    #[must_use]
    pub fn lens(&self) -> &str {
        self.lens.as_str()
    }

    /// The four namespaced references this row states about itself.
    #[must_use]
    pub const fn references(&self) -> &BenchReferences {
        &self.references
    }

    /// The declared input-size axis, in the order it was declared; structurally
    /// at least two distinct sizes.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: the axis is the curve a growth class is read off,
    /// and the rendering writes the sizes in exactly the order they were
    /// declared.
    pub fn axis(&self) -> impl Iterator<Item = &u64> {
        self.axis.iter()
    }

    /// How many points the axis states; structurally at least two.
    #[must_use]
    pub fn count(&self) -> usize {
        self.axis.len()
    }

    /// What this row declares about how it is measured.
    #[must_use]
    pub const fn measurement(&self) -> &BenchMeasurement {
        &self.measurement
    }

    /// What makes this row measurable.
    #[must_use]
    pub const fn attachment(&self) -> &BenchAttachment {
        &self.attachment
    }
}

impl BenchTablePayload {
    /// Declare the complete payload one bench table is written from.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// module spelling is not one Rust identifier,
    /// [`BenchDeclarationRefusal::RowsAbsent`] where no row was supplied,
    /// [`BenchDeclarationRefusal::LensSpellingDoubled`] where two rows carry one
    /// lens spelling — the rendered adapter puts every lens in one namespace, so a
    /// collision would be a duplicate definition inside an expansion nobody wrote
    /// — and [`BenchDeclarationRefusal::RowsUnbounded`] where the rows outgrow the
    /// declared magnitude.
    pub fn declared(
        module: &str,
        table: WallName,
        producer: WallName,
        rows: Vec<BenchRow>,
    ) -> Result<Self, BenchDeclarationRefusal> {
        if !is_rendered_identifier(module) {
            return Err(BenchDeclarationRefusal::SpellingNotAnIdentifier);
        }
        let mut supplied = rows.into_iter();
        let Some(first) = supplied.next() else {
            return Err(BenchDeclarationRefusal::RowsAbsent);
        };
        let rest: Vec<BenchRow> = supplied.collect();
        lens_namespace_closed(&first, &rest)?;
        let admitted: NonEmptyBounded<BenchRow, BenchRowLimit> = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| BenchDeclarationRefusal::RowsUnbounded)?;
        Ok(Self {
            module: module.to_owned(),
            table,
            producer,
            rows: admitted,
        })
    }

    /// The stamped module's spelling.
    #[must_use]
    pub fn module(&self) -> &str {
        self.module.as_str()
    }

    /// The bench table's own namespaced name.
    #[must_use]
    pub const fn table(&self) -> &WallName {
        &self.table
    }

    /// The producer that emitted this table.
    #[must_use]
    pub const fn producer(&self) -> &WallName {
        &self.producer
    }

    /// The rows, in the order they were declared; structurally at least one.
    pub fn rows(&self) -> impl Iterator<Item = &BenchRow> {
        self.rows.iter()
    }

    /// How many rows this table declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

impl BenchBackend {
    /// The backend this home renders as the current one-file choice.
    ///
    /// Total: the spelling is a constant this home declares, so there is no count
    /// to read and no refusal to return.
    ///
    /// # Nonclaims
    ///
    /// It states which backend the rendering writes TODAY and nothing about which
    /// one a consumer must use. The adapter reads its backend name from the value
    /// it is handed, so a consumer that hands it another one gets that one — which
    /// is what "backend-agnostic by construction" means here.
    #[must_use]
    pub fn current() -> Self {
        Self {
            spelling: CURRENT_BACKEND.to_owned(),
        }
    }

    /// The backend a consumer named, under whatever it reaches the dependency by.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// spelling is not one Rust identifier: it is written in path position, so a
    /// spelling that is not an identifier renders tokens the consumer's compiler
    /// reads as something else.
    pub fn named(spelling: &str) -> Result<Self, BenchDeclarationRefusal> {
        if !is_rendered_identifier(spelling) {
            return Err(BenchDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            spelling: spelling.to_owned(),
        })
    }

    /// The spelling every backend-naming token in the adapter is written from.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl BenchReporterAdapter {
    /// Declare the one-file reporter adapter.
    ///
    /// # Errors
    ///
    /// Returns [`BenchDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// module spelling is not one Rust identifier.
    pub fn declared(module: &str, backend: BenchBackend) -> Result<Self, BenchDeclarationRefusal> {
        if !is_rendered_identifier(module) {
            return Err(BenchDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            module: module.to_owned(),
            backend,
        })
    }

    /// The module the adapter is rendered as.
    #[must_use]
    pub fn module(&self) -> &str {
        self.module.as_str()
    }

    /// The one swap point: the backend the neutral table is bound to.
    #[must_use]
    pub const fn backend(&self) -> &BenchBackend {
        &self.backend
    }
}

impl BenchmarkShell {
    /// Where a benchmark shell lands, stated once as a constant rather than
    /// carried as a seat that could say something else.
    pub const DESTINATION: MemberDestination = MemberDestination::AtDeclarationSite;

    /// Render one benchmark shell over what the plan decided and what the caller
    /// declared.
    ///
    /// The order is the road: the exported name from the plan's own identity,
    /// then the bench table the gate carries and the one-file adapter that rides
    /// beside it — attempted independently, because they are independent — then
    /// the gate's expectation, which is total, and the shell only after all of
    /// them.
    ///
    /// # Errors
    ///
    /// Returns the carrier's rendering family naming
    /// [`ShellRenderIssue::ShellTreeUnbounded`] where the bench table, the
    /// one-file adapter, or the carrier that holds them outgrows the declared
    /// token magnitude. The table and the adapter are INDEPENDENT parts and are
    /// attempted both before either is given up on, so their issues are
    /// established TOGETHER: a caller repairing a seam one part per attempt is a
    /// caller this home failed, and which part overran is what the reader repairs
    /// from.
    ///
    /// The gate's expectation is not among them: the road that writes it is
    /// total, because thirty-two bytes are one literal token.
    pub fn rendered(
        stated: &BenchmarkPlan,
        payload: &BenchTablePayload,
        adapter: &BenchReporterAdapter,
    ) -> Result<Self, ShellRendering> {
        let name = ShellName::mangled(stated.plan);
        let mut issues: Vec<ShellRenderIssue> = Vec::new();
        let cargo = collected(render::bench_table(payload), &mut issues);
        let reporter = collected(render::reporter_adapter(adapter, payload), &mut issues);
        let (Some(cargo), Some(reporter)) = (cargo, reporter) else {
            return Err(established(issues));
        };
        // The bench table rides the TRIALS seat, which is the seat this
        // crossing's payload has always ridden; the DEFERRED seat is rendered
        // empty, because this crossing defers no cargo into the carrier — the
        // reporter adapter is an item beside the gate rather than cargo through
        // it.
        let mut body = gate_invocation(expectation_literal(), cargo, Vec::new()).map_err(sole)?;
        body.extend(reporter);
        let tokens = exported_shell(&name, body).map_err(sole)?;
        let tree = GeneratedTree::assembled(tokens).map_err(|_| sole(unbounded()))?;
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin.clone(),
            name,
            tree,
        })
    }

    /// The rendered role this shell stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
        self.role
    }

    /// The planned member's semantic key this shell answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
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

    /// The trail this shell walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The exported name a bench target invokes this shell by.
    #[must_use]
    pub const fn name(&self) -> &ShellName {
        &self.name
    }

    /// The rendered carrier — the exported macro definition, holding the bench
    /// table and the one-file adapter inert.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

// ---------------------------------------------------------------------------
// The passes.
// ---------------------------------------------------------------------------

/// One rendered part, or nothing where the part established an issue.
///
/// Every part is attempted before any of them is given up on, which is what makes
/// the refusal body carry the complete owed set rather than the first gap the road
/// happened to reach.
fn collected<T>(
    rendered: Result<T, ShellRenderIssue>,
    issues: &mut Vec<ShellRenderIssue>,
) -> Option<T> {
    match rendered {
        Ok(part) => Some(part),
        Err(issue) => {
            issues.push(issue);
            None
        }
    }
}

/// One established set of issues as the body a refusal carries.
///
/// The empty case cannot arise on the roads that call it — every caller pushed at
/// least one issue before reaching here — and rather than fabricate a value for a
/// case that cannot happen, the shape refuses with the one issue that is always
/// true of a rendering nobody could complete.
fn established(issues: Vec<ShellRenderIssue>) -> ShellRendering {
    match ShellRenderIssue::established(issues) {
        Some((first, rest)) => ShellRendering::established(first, rest),
        None => ShellRendering::established(unbounded(), Vec::new()),
    }
}

/// One established issue as the body a refusal carries.
fn sole(issue: ShellRenderIssue) -> ShellRendering {
    ShellRendering::established(issue, Vec::new())
}

/// The rendered adapter's ONE namespace, closed: every lens spelling across every
/// row, distinct.
///
/// Refused here rather than left to the consumer's compiler, which would report a
/// duplicate definition inside an expansion nobody wrote.
fn lens_namespace_closed(
    first: &BenchRow,
    rest: &[BenchRow],
) -> Result<(), BenchDeclarationRefusal> {
    let mut taken: BTreeSet<&str> = BTreeSet::new();
    for row in core::iter::once(first).chain(rest.iter()) {
        if !taken.insert(row.lens()) {
            return Err(BenchDeclarationRefusal::LensSpellingDoubled);
        }
    }
    Ok(())
}

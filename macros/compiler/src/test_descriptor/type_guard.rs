//! The test-descriptor home's invariant nucleus: every road that reaches a
//! private field, the mangling that makes an exported name collision-free, and
//! the one road that turns a pass's established issues into the pair a refusal
//! body is built from.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! claims structural rather than remembered.
//! A name is parsed HERE, so a reference that names nothing is not a value anybody can hold.
//! A path is rooted HERE, so a rendered expression that names no crate binding is unwritable.
//! A payload's namespace is closed HERE, so a stamped module that would declare one function twice is refused before a token exists.
//! A deferred cargo's token tree is bound HERE.
//! A shell is composed HERE, so there is no half-rendered carrier for a reader to mistake for a whole one.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared beside
//! the rest of this home's declarations would put all of them inside the same
//! wall. That module's entire content is the record and its inherent
//! implementations, so the module IS the complete set of roads that reach the
//! private seat.

use super::super::render;
use super::{
    BoundPath, CrateFacing, DeclarationDoor, DeferredCargo, DeferredDelivery, DescriptorPlan,
    DescriptorRow, GeneratedSupportShell, PathSegmentLimit, RoleLimit, RowLimit, RowReferences,
    ShellDeclarationRefusal, ShellName, ShellRenderIssue, SuiteGroup, SuiteGroupLimit,
    SupportDelivery, SupportMacroName, TagLimit, TrialDelivery, TrialLensName, TrialModuleName,
    TrialSeatName, TrialTablePayload, WallName,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, GeneratedUnitSubject, PlanId, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::MemberDestination;
use crate::token::GeneratedTree;
use macroonz::{AdmittedLimit, Bounded, NonEmptyBounded, PositiveLimit};
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// The vocabulary's nuclei.
// ---------------------------------------------------------------------------

impl WallName {
    /// This name, parsed from the owner that declares it and the spelling it
    /// carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem. The checks are dependent
    /// and in that order, so exactly one cause is true of any refused name.
    pub fn named(namespace: &str, stem: &str) -> Result<Self, ShellDeclarationRefusal> {
        if namespace.is_empty() {
            return Err(ShellDeclarationRefusal::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(ShellDeclarationRefusal::EmptyStem);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            stem: stem.to_owned(),
        })
    }

    /// The owner that declares the spelling.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    /// The spelling itself.
    #[must_use]
    pub fn stem(&self) -> &str {
        self.stem.as_str()
    }
}

impl BoundPath {
    /// The path rooted at one rename twin, over the segments that follow it.
    ///
    /// The first segment is a separate parameter, so a path naming a crate and
    /// nothing in it is unrepresentable rather than refused — but the door still
    /// carries [`ShellDeclarationRefusal::PathSegmentsAbsent`], because a caller
    /// arrives holding a list and a list can be empty.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::PathSegmentsAbsent`] where no segment
    /// was supplied, [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where a
    /// segment is not one Rust identifier, and
    /// [`ShellDeclarationRefusal::PathSegmentsUnbounded`] where the segments
    /// outgrow the declared magnitude.
    pub fn rooted(
        facing: CrateFacing,
        segments: Vec<String>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        let mut supplied = segments.into_iter();
        let Some(first) = supplied.next() else {
            return Err(ShellDeclarationRefusal::PathSegmentsAbsent);
        };
        let rest: Vec<String> = supplied.collect();
        if !is_rendered_identifier(first.as_str()) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
        for segment in &rest {
            if !is_rendered_identifier(segment.as_str()) {
                return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
            }
        }
        let admitted: NonEmptyBounded<String, PathSegmentLimit> = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| ShellDeclarationRefusal::PathSegmentsUnbounded)?;
        Ok(Self {
            facing,
            segments: admitted,
        })
    }

    /// Which rename twin this path is rooted at.
    #[must_use]
    pub const fn facing(&self) -> CrateFacing {
        self.facing
    }

    /// The segments after the crate binding, in the order they were declared;
    /// structurally at least one.
    pub fn segments(&self) -> impl Iterator<Item = &String> {
        self.segments.iter()
    }

    /// How many segments follow the crate binding; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.len()
    }
}

// ---------------------------------------------------------------------------
// The four syntax-facing identifiers.
//
// One road each, and each of them the same road: a spelling written in
// identifier position is admitted by the one alphabet every crossing renders
// through, or it is refused before a token exists. The four types are four seats
// rather than one, so a road that wants an exported support name cannot be
// handed a lens.
// ---------------------------------------------------------------------------

/// Declares one identifier newtype's checked constructor and its reading.
///
/// One expansion per row rather than four hand-written pairs: the CHECK and the
/// refusal are the same fact for all four spellings — each is written into a
/// consumer's target in identifier position — and four copies of it would be four
/// things to keep true. What differs between the rows is the seat, which is the
/// type the row declares, and the sentence a reader is shown.
macro_rules! rendered_identifiers {
    ($(
        $name:ident, $reading:literal
    );+ $(;)?) => {
        $(
            impl $name {
                #[doc = concat!("This ", $reading, ", read from the spelling an author wrote.")]
                ///
                /// # Errors
                ///
                /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`]
                /// where the spelling is not one Rust identifier: it is written
                /// into a consumer's target in identifier position, and a
                /// spelling that is not one renders tokens that compiler reads as
                /// something else.
                pub fn declared(spelling: &str) -> Result<Self, ShellDeclarationRefusal> {
                    if is_rendered_identifier(spelling) {
                        Ok(Self(spelling.to_owned()))
                    } else {
                        Err(ShellDeclarationRefusal::SpellingNotAnIdentifier)
                    }
                }

                #[doc = concat!("The spelling this ", $reading, " carries.")]
                #[must_use]
                pub fn spelling(&self) -> &str {
                    self.0.as_str()
                }
            }
        )+
    };
}

rendered_identifiers! {
    SupportMacroName, "exported support name";
    TrialModuleName, "stamped module name";
    TrialSeatName, "aggregate seat name";
    TrialLensName, "row lens name";
}

impl DescriptorRow {
    /// Declare one descriptor row.
    ///
    /// # Bounds
    ///
    /// There is no suite parameter and no origin parameter, and neither absence
    /// is a dropped fact. A row's execution suite is its GROUP's, stated once at
    /// [`SuiteGroup`] and inherited structurally; a row's origin is the
    /// producer's own act, composed inside the rendering from the payload's door
    /// and this home's declared projection spelling.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::RoleDoubled`] and
    /// [`ShellDeclarationRefusal::TagDoubled`] where a roster states one label
    /// twice — refused rather than folded away, because collapsing a duplicate
    /// silently would be this side normalizing an authoring defect the harness
    /// itself refuses — and the two unbounded causes where a roster outgrows its
    /// declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true
    /// of any refused row.
    pub fn declared(
        lens: TrialLensName,
        references: RowReferences,
        roles: Vec<WallName>,
        tags: Vec<WallName>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if names_doubled(&roles) {
            return Err(ShellDeclarationRefusal::RoleDoubled);
        }
        let admitted_roles: Bounded<WallName, RoleLimit> = Bounded::admitted_const(
            roles,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| ShellDeclarationRefusal::RolesUnbounded)?;
        if names_doubled(&tags) {
            return Err(ShellDeclarationRefusal::TagDoubled);
        }
        let admitted_tags: Bounded<WallName, TagLimit> = Bounded::admitted_const(
            tags,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| ShellDeclarationRefusal::TagsUnbounded)?;
        Ok(Self {
            lens,
            references,
            roles: admitted_roles,
            tags: admitted_tags,
        })
    }

    /// The lens the stamp declares this row's named test function under.
    #[must_use]
    pub const fn lens(&self) -> &TrialLensName {
        &self.lens
    }

    /// The four namespaced references this row states about itself.
    #[must_use]
    pub const fn references(&self) -> &RowReferences {
        &self.references
    }

    /// The roles this row carries, in the order they were declared.
    pub fn roles(&self) -> impl Iterator<Item = &WallName> {
        self.roles.iter()
    }

    /// The tags this row carries, in the order they were declared.
    pub fn tags(&self) -> impl Iterator<Item = &WallName> {
        self.tags.iter()
    }
}

impl SuiteGroup {
    /// Declare one aggregate seat's group.
    ///
    /// # Authority
    ///
    /// The suite is stated here and NOWHERE ELSE on this road: every row grouped
    /// under this seat runs under this suite by construction, so the pairing the
    /// stamp cannot check at expansion is one no declaration can get wrong.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::RowsAbsent`] where no row was supplied
    /// — a seat over no row is a seat that measures nothing — and
    /// [`ShellDeclarationRefusal::RowsUnbounded`] where the rows outgrow the
    /// declared magnitude.
    ///
    /// # Bounds
    ///
    /// Lens uniqueness is NOT checked here. The stamped module puts every group's
    /// seats and every group's lenses in ONE namespace, so the whole namespace is
    /// visible at the payload and nowhere else — and a uniqueness law standing in
    /// two homes is one law that agrees with itself until one home is edited.
    pub fn declared(
        seat: TrialSeatName,
        suite: WallName,
        rows: Vec<DescriptorRow>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        let mut supplied = rows.into_iter();
        let Some(first) = supplied.next() else {
            return Err(ShellDeclarationRefusal::RowsAbsent);
        };
        let rest: Vec<DescriptorRow> = supplied.collect();
        let admitted: NonEmptyBounded<DescriptorRow, RowLimit> = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| ShellDeclarationRefusal::RowsUnbounded)?;
        Ok(Self {
            seat,
            suite,
            rows: admitted,
        })
    }

    /// The aggregate seat this group declares.
    #[must_use]
    pub const fn seat(&self) -> &TrialSeatName {
        &self.seat
    }

    /// The execution suite this seat selects on.
    #[must_use]
    pub const fn suite(&self) -> &WallName {
        &self.suite
    }

    /// The rows declared under this seat, in the order they were declared;
    /// structurally at least one.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: the stamp writes one lens function per row in the
    /// order it reads them, and the table collects them in the same order, so the
    /// same rows supplied in another order render a different tree.
    pub fn rows(&self) -> impl Iterator<Item = &DescriptorRow> {
        self.rows.iter()
    }

    /// How many rows this seat declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

impl TrialTablePayload {
    /// Declare the complete payload one stamped trial table is written from.
    ///
    /// # Bounds
    ///
    /// There is no producer parameter and no projection parameter. Which producer
    /// emitted a table and which projection emitted its rows are facts about
    /// THESE SERVICES, so they are this home's declared spellings
    /// ([`GENERATED_TABLE_PRODUCER`](super::GENERATED_TABLE_PRODUCER),
    /// [`GENERATED_ROW_PROJECTION`](super::GENERATED_ROW_PROJECTION)) rather than
    /// seats an authored declaration could fill; the door is a row of a closed
    /// roster for the same reason.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SuiteGroupsAbsent`] where no group was
    /// supplied, [`ShellDeclarationRefusal::SeatSpellingDoubled`] and
    /// [`ShellDeclarationRefusal::LensSpellingDoubled`] where two items of the
    /// stamped module's ONE namespace carry a single spelling — seats and lenses
    /// share that namespace, so a seat colliding with a lens is caught here as
    /// well — and [`ShellDeclarationRefusal::SuiteGroupsUnbounded`] where the
    /// groups outgrow the declared magnitude.
    ///
    /// The namespace check runs before the magnitude check because a collision is
    /// a defect in what was declared, and a caller repairing a magnitude first
    /// would repair the collision second.
    pub fn declared(
        support: SupportMacroName,
        module: TrialModuleName,
        table: WallName,
        door: DeclarationDoor,
        groups: Vec<SuiteGroup>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        let mut supplied = groups.into_iter();
        let Some(first) = supplied.next() else {
            return Err(ShellDeclarationRefusal::SuiteGroupsAbsent);
        };
        let rest: Vec<SuiteGroup> = supplied.collect();
        stamped_namespace_closed(&first, &rest)?;
        let admitted: NonEmptyBounded<SuiteGroup, SuiteGroupLimit> =
            NonEmptyBounded::admitted_const(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
            )
            .map_err(|_| ShellDeclarationRefusal::SuiteGroupsUnbounded)?;
        Ok(Self {
            support,
            module,
            table,
            door,
            groups: admitted,
        })
    }

    /// The exported name a consumption target invokes this declaration's support
    /// carrier by.
    #[must_use]
    pub const fn support(&self) -> &SupportMacroName {
        &self.support
    }

    /// The module the stamp writes this table into.
    #[must_use]
    pub const fn module(&self) -> &TrialModuleName {
        &self.module
    }

    /// The authored table's own namespaced name.
    #[must_use]
    pub const fn table(&self) -> &WallName {
        &self.table
    }

    /// The declaration door these rows were authored through.
    #[must_use]
    pub const fn door(&self) -> DeclarationDoor {
        self.door
    }

    /// The aggregate seats, in the order they were declared; structurally at
    /// least one.
    pub fn groups(&self) -> impl Iterator<Item = &SuiteGroup> {
        self.groups.iter()
    }

    /// How many aggregate seats this payload declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.groups.len()
    }
}

impl DeferredCargo {
    /// Declare the cargo one carrier receives.
    ///
    /// # Errors
    ///
    /// A cargo of no tokens is admitted and remains distinct from `DeferredDelivery::NothingDeferred`, because this road never turns one posture into the other.
    pub const fn deferred(tokens: GeneratedTree) -> Self {
        Self { tokens }
    }

    /// The tokens themselves: one emission's proved cargo, handed over whole.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tokens
    }
}

impl ShellName {
    /// The fixed prefix every exported shell name carries.
    ///
    /// Two leading underscores and a namespaced stem: the name lands at the root
    /// of the consumer's own crate, so it wears the machine's name and reads as
    /// machinery rather than as anything a consumer wrote.
    pub const PREFIX: &'static str = "__threadpak_generated_support_";

    /// How many bytes of the plan identity the mangled spelling carries.
    ///
    /// THIRTY-TWO — the whole identity — spelled as sixty-four lowercase
    /// hexadecimal characters. It used to be eight, and eight bytes made every
    /// "collision-free" sentence on this road a different claim than the one it
    /// was written as: sixty-four bits of a derived identity is a width at which
    /// a collision is unlikely, and the prose beside it said two distinct plans
    /// CANNOT reach one spelling. A name at full width makes the sentence true
    /// as written, and the cost is a suffix nobody reads — which is the cost a
    /// mangled machinery name is supposed to have.
    ///
    /// # Nonclaims
    ///
    /// The spelling is not the identity, and nothing reads it back. No road
    /// parses hexadecimal into a plan identity, no decision consults the
    /// spelling, and every identity-bearing road uses the plan's own value.
    pub const KEY_BYTES: usize = 32;

    /// The mangled name one plan's shell is exported under.
    ///
    /// The key is the PLAN's own identity, which is why the parameter is one:
    /// a planned member's semantic key is a value the planning caller supplies,
    /// and two plans handed one key would mint one exported name for two
    /// declarations. A plan identity is derived by these services over the
    /// account, the context, the whole membership, the watch set, the trace, and
    /// the origin trail, so it cannot be handed in and it separates two doors
    /// over one declaration.
    ///
    /// Total: the prefix is a constant and the suffix is hexadecimal over
    /// thirty-two bytes that always exist, so there is no count to read and no
    /// refusal to return.
    #[must_use]
    #[expect(
        clippy::format_push_string,
        reason = "the lint is about an allocation per turn of a hot loop; this turns once per key byte to spell one shell's name at expansion time, and the alternative writes through a `Result` that cannot fail into a road that would then discard it"
    )]
    pub fn mangled(plan: PlanId) -> Self {
        let mut spelling = String::from(Self::PREFIX);
        for byte in plan.as_bytes().iter().take(Self::KEY_BYTES) {
            spelling.push_str(&format!("{byte:02x}"));
        }
        Self { spelling }
    }

    /// The exported spelling a consumption target invokes this shell by.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl GeneratedSupportShell {
    /// Where a generated support shell lands, stated once as a constant rather
    /// than carried as a seat that could say something else.
    ///
    /// It is the DECLARATION SITE, and that is the whole reason the carrier is a
    /// macro definition: a macro invoked in a test target sees only its own
    /// invocation tokens, so the declaration's structure has to travel as
    /// deferred tokens the consumption target then invokes.
    pub const DESTINATION: MemberDestination = MemberDestination::AtDeclarationSite;

    /// Render one generated support shell over what the plan decided, what the
    /// caller declared into the trials seat, and what an expansion deferred into
    /// the deferred seat.
    ///
    /// The order is the road: the exported name from the plan's own identity,
    /// then the trials seat's tokens, then the deferred seat's, then the ONE
    /// gate invocation that carries both, then the exported macro definition
    /// around it, then the caller-named alias that forwards to it — and the shell
    /// only after all six, so no half-rendered carrier exists.
    ///
    /// # Two definitions, one tree
    ///
    /// The physical carrier is exported under a plan-keyed spelling nobody can
    /// know before expansion, so a delivery that carries ROWS also renders the
    /// PUBLIC ALIAS its author declared: an ordinary exported `macro_rules!` whose
    /// one rule forwards its whole input to the hidden name through `$crate`. The
    /// alias is a member of THIS tree rather than a second emission appended
    /// afterwards, because everything a closed expansion hands out is inside what
    /// was proved.
    ///
    /// The one helper that owns the support address supplies the alias. Trial
    /// declarations own it when present; a mutation-only declaration owns it
    /// otherwise. A carrier with neither helper remains unaddressed.
    ///
    /// # Everything the carrier delivers rides INSIDE the gate
    ///
    /// The shell's body is one gate invocation and nothing else. Both cargo
    /// seats are inside it: `trials:` carries the descriptor rows under the
    /// harness's own grammar, and `deferred:` carries opaque token trees the
    /// gate never parses. The rendering home owns the complete cargo tree; this
    /// shell forwards it verbatim and the gate releases both seats or neither.
    ///
    /// Both seats are always rendered, whichever posture fills them.
    /// An empty trials seat beside carried deferred cargo is a MUTATION-ONLY delivery and is lawful.
    /// A seat left out would be a second shape one published arm has to match.
    ///
    /// # Construction
    ///
    /// Crate-internal, and the carrier-assembly home declared later in the
    /// module order is its one caller. A public road here would take a deferred
    /// cargo anybody can declare, so unproved tokens would cross the wall
    /// through the very carrier the assembly exists to keep them out of; the
    /// public road to a shell runs through a verified assembly, which is where
    /// the one root, the one published expectation, and the exactly-once
    /// consumption are established.
    ///
    /// # Errors
    ///
    /// Returns the rendering family naming
    /// [`ShellRenderIssue::ShellTreeUnbounded`] where the trials seat, the
    /// deferred seat, the gate invocation around both, the exported carrier
    /// around that, or the assembled tree outgrows the declared token magnitude.
    ///
    /// The gate's own expectation is not among them: thirty-two bytes are one
    /// literal token, so the road that writes it is total and there is no branch
    /// here for a case that cannot happen. What remains is a chain each part of
    /// which is refused before the next is reached, so exactly one issue is ever
    /// established on this crossing. The family's collection shape is the BENCH
    /// crossing's, which renders two independent parts — a bench table and a
    /// reporter adapter — either of which can overrun on its own.
    pub(crate) fn rendered(
        stated: &DescriptorPlan,
        trials: TrialDelivery<'_>,
        deferred: DeferredDelivery<'_>,
        support: SupportDelivery<'_>,
    ) -> Result<Self, ShellRendering> {
        let name = ShellName::mangled(stated.plan);
        let pin = render::expectation_roster().map_err(|issue| established(vec![issue]))?;
        let declared = render::trial_cargo(trials).map_err(|issue| established(vec![issue]))?;
        let carried = render::deferred_cargo(deferred);
        let body = render::gate_invocation(pin, declared, carried)
            .map_err(|issue| established(vec![issue]))?;
        let matched = render::matcher(trials).map_err(|issue| established(vec![issue]))?;
        let mut tokens = render::exported_shell(&name, matched, body)
            .map_err(|issue| established(vec![issue]))?;
        tokens.extend(
            render::public_alias(&name, support).map_err(|issue| established(vec![issue]))?,
        );
        let tree =
            GeneratedTree::assembled(tokens).map_err(|_| established(vec![render::unbounded()]))?;
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

    /// The exported name a consumption target invokes this shell by.
    #[must_use]
    pub const fn name(&self) -> &ShellName {
        &self.name
    }

    /// The rendered carrier — the exported macro definition, holding its cargo
    /// inert.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl ShellRenderIssue {
    /// One pass's established issues as the pair a refusal body is built from, or
    /// nothing where the pass established none.
    ///
    /// Seated here rather than beside a pass because the body is here: a pass
    /// hands over what it found, and the shape a body requires — a first issue and
    /// the rest — is decided once, where bodies are made.
    #[must_use]
    pub fn established(issues: Vec<Self>) -> Option<(Self, Vec<Self>)> {
        let mut walk = issues.into_iter();
        let first = walk.next()?;
        Some((first, walk.collect()))
    }
}

// ---------------------------------------------------------------------------
// The passes, and the seat itself.
// ---------------------------------------------------------------------------

/// One established set of issues as the body a refusal carries.
///
/// The empty case cannot arise on the roads that call it — every caller pushed at
/// least one issue before reaching here — and rather than fabricate a value for a
/// case that cannot happen, the shape refuses with the one issue that is always
/// true of a rendering nobody could complete.
fn established(issues: Vec<ShellRenderIssue>) -> ShellRendering {
    match ShellRenderIssue::established(issues) {
        Some((first, rest)) => ShellRendering::established(first, rest),
        None => ShellRendering::established(render::unbounded(), Vec::new()),
    }
}

/// Whether two of one roster's names carry one spelling.
///
/// Counted rather than walked with an early return, so the answer is one
/// comparison between what was supplied and what was distinct.
fn names_doubled(names: &[WallName]) -> bool {
    let distinct: BTreeSet<&WallName> = names.iter().collect();
    distinct.len() != names.len()
}

/// Whether two stamped functions would declare one constant name.
///
/// The CONSTANT alone matters because that is what the spliced module declares.
/// Two distinct seats or lenses under one constant would declare one item twice.
///
/// Counted rather than walked with an early return, on the same terms.
/// The stamped module's ONE namespace, closed: every seat spelling and every lens
/// spelling across every group, distinct.
///
/// Seats and lenses are both functions in the module the stamp writes, so they
/// share one namespace and a seat colliding with a lens is the same defect as two
/// lenses colliding. Refused here rather than left to the consumer's compiler,
/// which would report a duplicate definition inside an expansion nobody wrote.
fn stamped_namespace_closed(
    first: &SuiteGroup,
    rest: &[SuiteGroup],
) -> Result<(), ShellDeclarationRefusal> {
    let mut taken: BTreeSet<&str> = BTreeSet::new();
    for group in core::iter::once(first).chain(rest.iter()) {
        if !taken.insert(group.seat().spelling()) {
            return Err(ShellDeclarationRefusal::SeatSpellingDoubled);
        }
    }
    for group in core::iter::once(first).chain(rest.iter()) {
        for row in group.rows() {
            if !taken.insert(row.lens().spelling()) {
                return Err(ShellDeclarationRefusal::LensSpellingDoubled);
            }
        }
    }
    Ok(())
}

/// Whether one spelling is a single Rust identifier the carrier is willing to
/// render.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern rather
/// than a name.
///
/// # Authority
///
/// **One alphabet, for every spelling any crossing renders as an identifier** — a
/// path segment, a lens, a seat, a module, a backend. It is public because the
/// benchmark home renders identifiers too and rides this carrier: a second copy
/// of the alphabet would agree with this one until one of them was edited, and
/// the failure would surface in a consumer's build with no idea where the name
/// came from.
#[must_use]
pub fn is_rendered_identifier(spelling: &str) -> bool {
    let mut characters = spelling.chars();
    let Some(head) = characters.next() else {
        return false;
    };
    if !head.is_ascii_alphabetic() && head != '_' {
        return false;
    }
    if spelling == "_" {
        return false;
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

pub use seat::ShellRendering;

mod seat {
    use super::super::{ShellIssueLimit, ShellRenderIssue};
    use crate::plane::AuthoringLimitProfile;
    use macroonz::{AdmittedPrefix, PositiveLimit, StopBound};

    /// The shell-rendering refusal family body.
    ///
    /// Independent members: a crossing renders its parts independently — the
    /// carrier's expectation and its payload, and the bench crossing's payload
    /// and adapter beside them — and each can outgrow the declared token
    /// magnitude on its own, so several are true of one rendering and no primary
    /// issue is ever elected.
    #[must_use = "a refusal family body carries every gap the rendering passes established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ShellRendering {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its pass established
        /// or names how many stand outside that bound. One seat rather than two,
        /// because a coverage claim seated beside its body is a claim that can be
        /// swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record hands
        /// the whole record back as a literal, so any holder of a body built for
        /// one pass could write it into another pass's refusal. Read back through
        /// [`ShellRendering::body`].
        body: AdmittedPrefix<ShellRenderIssue, ShellIssueLimit>,
    }

    impl ShellRendering {
        /// The body a rendering pass refuses with.
        ///
        /// Each pass walks its whole subject before a body exists, so the posture
        /// here is about the REPORT rather than the pass: where every established
        /// issue fits the declared bound the body carries all of them; where it
        /// does not, the body carries what the bound holds and names how many
        /// established issues stand outside it.
        ///
        /// Crate-internal, so a body exists only where one of this crate's own
        /// rendering roads ran — the benchmark home rides the same carrier and
        /// refuses in the same family, and a body a consumer could mint would be a
        /// refusal nobody's pass established.
        pub(crate) fn established(first: ShellRenderIssue, rest: Vec<ShellRenderIssue>) -> Self {
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
        /// Borrowed and never owned, for the reason band 00 borrows its carry: an
        /// owned body is a value a caller can seat under another refusal, which is
        /// the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<ShellRenderIssue, ShellIssueLimit> {
            &self.body
        }
    }
}

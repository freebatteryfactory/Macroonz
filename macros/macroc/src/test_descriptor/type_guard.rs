//! The test-descriptor home's invariant nucleus: every road that reaches a
//! private field, the mangling that makes an exported name collision-free, and
//! the one road that turns a pass's established issues into the pair a refusal
//! body is built from.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! claims structural rather than remembered. A name is parsed HERE, so a
//! reference that names nothing is not a value anybody can hold. A path is rooted
//! HERE, so a rendered expression that names no crate binding is unwritable. A
//! payload's namespace is closed HERE, so a stamped module that would declare one
//! function twice is refused before a token exists. A deferred cargo's subject
//! and selectors are declared HERE, so a module the shell splices can never
//! declare one constant twice. And a shell is composed HERE, so there is no
//! half-rendered carrier for a reader to mistake for a whole one.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared beside
//! the rest of this home's declarations would put all of them inside the same
//! wall. That module's entire content is the record and its inherent
//! implementations, so the module IS the complete set of roads that reach the
//! private seat.

use super::super::render;
use super::{
    ActivePointSelector, BoundPath, CrateFacing, DeferredCargo, DeferredDelivery, DescriptorPlan,
    DescriptorRow, GeneratedSupportShell, PathSegmentLimit, ProducerOrigin, RoleLimit,
    RowAttachment, RowLimit, RowReferences, SelectorLimit, ShellDeclarationRefusal, ShellName,
    ShellRenderIssue, SuiteGroup, SuiteGroupLimit, TagLimit, TrialTablePayload, WallName,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, GeneratedUnitSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::MemberDestination;
use crate::token::GeneratedTree;
use std::collections::BTreeSet;
use threadpak::types::{AdmittedLimit, Bounded, NonEmptyBounded, PositiveLimit};

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
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Always `false`: a path naming no segment is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

impl DescriptorRow {
    /// Declare one descriptor row.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// lens spelling is not one Rust identifier,
    /// [`ShellDeclarationRefusal::RoleDoubled`] and
    /// [`ShellDeclarationRefusal::TagDoubled`] where a roster states one label
    /// twice — refused rather than folded away, because collapsing a duplicate
    /// silently would be this side normalizing an authoring defect the harness
    /// itself refuses — and the two unbounded causes where a roster outgrows its
    /// declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true
    /// of any refused row.
    pub fn declared(
        lens: &str,
        references: RowReferences,
        roles: Vec<WallName>,
        tags: Vec<WallName>,
        origin: ProducerOrigin,
        attachment: RowAttachment,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(lens) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
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
            lens: lens.to_owned(),
            references,
            roles: admitted_roles,
            tags: admitted_tags,
            origin,
            attachment,
        })
    }

    /// The lens spelling the stamp declares this row's named test function under.
    #[must_use]
    pub fn lens(&self) -> &str {
        self.lens.as_str()
    }

    /// The five namespaced references this row states about itself.
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

    /// What this producer's own act contributed to this row.
    #[must_use]
    pub const fn origin(&self) -> &ProducerOrigin {
        &self.origin
    }

    /// What makes this row executable.
    #[must_use]
    pub const fn attachment(&self) -> &RowAttachment {
        &self.attachment
    }
}

impl SuiteGroup {
    /// Declare one aggregate seat's group.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// seat spelling is not one Rust identifier,
    /// [`ShellDeclarationRefusal::RowsAbsent`] where no row was supplied — a seat
    /// over no row is a seat that measures nothing — and
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
        seat: &str,
        suite: WallName,
        rows: Vec<DescriptorRow>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(seat) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
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
            seat: seat.to_owned(),
            suite,
            rows: admitted,
        })
    }

    /// The aggregate seat's spelling.
    #[must_use]
    pub fn seat(&self) -> &str {
        self.seat.as_str()
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
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Always `false`: a seat over no row is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl TrialTablePayload {
    /// Declare the complete payload one stamped trial table is written from.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// module spelling is not one Rust identifier,
    /// [`ShellDeclarationRefusal::SuiteGroupsAbsent`] where no group was
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
        module: &str,
        table: WallName,
        producer: WallName,
        groups: Vec<SuiteGroup>,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(module) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
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
            module: module.to_owned(),
            table,
            producer,
            groups: admitted,
        })
    }

    /// The stamped module's spelling.
    #[must_use]
    pub fn module(&self) -> &str {
        self.module.as_str()
    }

    /// The authored table's own namespaced name.
    #[must_use]
    pub const fn table(&self) -> &WallName {
        &self.table
    }

    /// The producer that emitted this table.
    #[must_use]
    pub const fn producer(&self) -> &WallName {
        &self.producer
    }

    /// The aggregate seats, in the order they were declared; structurally at
    /// least one.
    pub fn groups(&self) -> impl Iterator<Item = &SuiteGroup> {
        self.groups.iter()
    }

    /// How many aggregate seats this payload declares; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Always `false`: a payload declaring no seat is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

impl ActivePointSelector {
    /// Declare one active-point selector.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// constant, the roster it stands on, or the row it stands at is not one
    /// Rust identifier. All three are written as identifiers into the module the
    /// cargo is spliced into, and a spelling that is not one renders tokens the
    /// consumer's compiler reads as something else.
    pub fn declared(
        constant: &str,
        active_enum: &str,
        variant: &str,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(constant)
            || !is_rendered_identifier(active_enum)
            || !is_rendered_identifier(variant)
        {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self {
            constant: constant.to_owned(),
            active_enum: active_enum.to_owned(),
            variant: variant.to_owned(),
        })
    }

    /// The constant every activation site reads the selection through.
    #[must_use]
    pub fn constant(&self) -> &str {
        self.constant.as_str()
    }

    /// The active-point roster that constant stands on.
    #[must_use]
    pub fn active_enum(&self) -> &str {
        self.active_enum.as_str()
    }

    /// The row of that roster the constant stands at.
    #[must_use]
    pub fn variant(&self) -> &str {
        self.variant.as_str()
    }
}

impl DeferredCargo {
    /// Declare the cargo one carrier receives.
    ///
    /// # Errors
    ///
    /// Returns [`ShellDeclarationRefusal::SpellingNotAnIdentifier`] where the
    /// subject's spelling is not one Rust identifier,
    /// [`ShellDeclarationRefusal::SelectorConstantDoubled`] where two selectors
    /// are read through one constant — the module the cargo is spliced into
    /// would declare that constant twice — and
    /// [`ShellDeclarationRefusal::SelectorsUnbounded`] where the selectors
    /// outgrow the declared magnitude.
    ///
    /// # Bounds
    ///
    /// An EMPTY selector roster is admitted and is a stated fact: a cargo whose
    /// implementations read no selection still stands over the subject, and the
    /// module the shell splices carries the subject either way.
    ///
    /// A cargo of no TOKENS is admitted too, and is a different fact from a
    /// carrier nothing was deferred into — that one is
    /// [`DeferredDelivery::NothingDeferred`], and this road never turns one into
    /// the other.
    pub fn deferred(
        subject: &str,
        selectors: Vec<ActivePointSelector>,
        tokens: GeneratedTree,
    ) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(subject) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
        if constants_doubled(&selectors) {
            return Err(ShellDeclarationRefusal::SelectorConstantDoubled);
        }
        let admitted: Bounded<ActivePointSelector, SelectorLimit> = Bounded::admitted_const(
            selectors,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| ShellDeclarationRefusal::SelectorsUnbounded)?;
        Ok(Self {
            subject: subject.to_owned(),
            selectors: admitted,
            tokens,
        })
    }

    /// The local subject the deferred implementations stand over.
    #[must_use]
    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    /// The selectors the deferred implementations read, in the order they were
    /// declared.
    pub fn selectors(&self) -> impl Iterator<Item = &ActivePointSelector> {
        self.selectors.iter()
    }

    /// How many selectors the cargo declares.
    #[must_use]
    pub fn len(&self) -> usize {
        self.selectors.len()
    }

    /// Whether the cargo declares no selector at all — a lawful, stated posture.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selectors.is_empty()
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

    /// How many bytes of the semantic key the mangled spelling carries.
    ///
    /// EIGHT, spelled as sixteen lowercase hexadecimal characters. The whole key
    /// is thirty-two bytes and a sixty-four character suffix is a name nobody
    /// reads; eight bytes is sixty-four bits of a content-addressed identity,
    /// which is the width at which a collision inside one crate's macro namespace
    /// stops being a thing anybody can arrange. The prefix carries the rest of
    /// the distinctness: nothing else in a consumer's crate is spelled this way.
    ///
    /// # Nonclaims
    ///
    /// A prefix of a derived identity is not the identity. Nothing derives
    /// anything from this spelling, nothing reads it back into a key, and the
    /// planned member's own semantic key is what every identity-bearing road
    /// uses.
    pub const KEY_BYTES: usize = 8;

    /// The mangled name one planned member's shell is exported under.
    ///
    /// Total: the prefix is a constant and the suffix is hexadecimal over bytes
    /// that always exist, so there is no count to read and no refusal to return.
    #[must_use]
    pub fn mangled(semantic_key: &ProjectionIdentity<GeneratedUnitSubject>) -> Self {
        let mut spelling = String::from(Self::PREFIX);
        for byte in semantic_key.as_bytes().iter().take(Self::KEY_BYTES) {
            spelling.push_str(&format!("{byte:02x}"));
        }
        Self { spelling }
    }

    /// The exported spelling a consumption target invokes this shell by.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }

    /// The suffix the shell's own deferred module wears.
    pub const DEFERRED_SUFFIX: &'static str = "_deferred";

    /// The private module this shell splices its deferred cargo into.
    ///
    /// The shell's own hex-keyed spelling and one suffix, so the module is
    /// collision-free on exactly the terms the exported name is: the key is
    /// content-addressed, so two distinct planned members reach two distinct
    /// modules without this home keeping a register of what it has already
    /// written.
    ///
    /// A macro and a module stand in two of Rust's namespaces and could share
    /// one spelling, but a reader who trips over both at one crate root should
    /// not have to know which namespace resolved which — so the module says
    /// which of the two it is.
    #[must_use]
    pub fn deferred_module(&self) -> String {
        let mut spelling = self.spelling.clone();
        spelling.push_str(Self::DEFERRED_SUFFIX);
        spelling
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
    /// caller declared, and what the expansion deferred into this carrier.
    ///
    /// The order is the road: the exported name from the plan's own semantic key,
    /// then the payload's tokens, then the gate invocation the payload rides
    /// inside, then the deferred module beside it, then the exported macro
    /// definition that carries both — and the shell only after all five, so no
    /// half-rendered carrier exists.
    ///
    /// # What each part of the body is
    ///
    /// The GATE INVOCATION carries the descriptor rows, and its grammar is the
    /// harness's. The DEFERRED MODULE stands beside it rather than inside it,
    /// because what rides through the gate is the harness's own cargo and this
    /// module is not: it is the local subject the deferred implementations stand
    /// over, those implementations, and one constant per selection they read.
    /// Both are items at the invocation site and both are expanded by the
    /// consumption target, which is what makes them one delivery.
    ///
    /// A delivery that deferred nothing splices no module: an expansion that
    /// planned no member into this carrier and one that sent it a cargo of no
    /// tokens are different facts, and only the second has a module to write.
    ///
    /// # Errors
    ///
    /// Returns the rendering family naming
    /// [`ShellRenderIssue::ShellTreeUnbounded`] where the stamped payload, the
    /// gate invocation around it, the deferred module beside it, the exported
    /// carrier around both, or the assembled tree outgrows the declared token
    /// magnitude.
    ///
    /// The gate's own expectation is not among them: thirty-two bytes are one
    /// literal token, so the road that writes it is total and there is no branch
    /// here for a case that cannot happen. What remains is a chain each part of
    /// which is refused before the next is reached, so exactly one issue is ever
    /// established on this crossing. The family's collection shape is the BENCH
    /// crossing's, which renders two independent parts — a bench table and a
    /// reporter adapter — either of which can overrun on its own.
    pub fn rendered(
        stated: &DescriptorPlan,
        payload: &TrialTablePayload,
        deferred: &DeferredDelivery,
    ) -> Result<Self, ShellRendering> {
        let name = ShellName::mangled(&stated.semantic_key);
        let pin = render::expectation_literal();
        let cargo = render::stamped_module(payload).map_err(|issue| established(vec![issue]))?;
        let mut body =
            render::gate_invocation(pin, cargo).map_err(|issue| established(vec![issue]))?;
        body.extend(
            render::deferred_module(&name, deferred).map_err(|issue| established(vec![issue]))?,
        );
        let tokens = render::exported_shell(&name, body).map_err(|issue| established(vec![issue]))?;
        let tree = GeneratedTree::assembled(tokens)
            .map_err(|_| established(vec![render::unbounded()]))?;
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

/// Whether two of one cargo's selectors are read through one constant.
///
/// The CONSTANT alone, because that is what the spliced module declares: two
/// selectors standing on one roster at two rows is an ordinary cargo, and two
/// standing under one constant is a module declaring one item twice.
///
/// Counted rather than walked with an early return, on the same terms.
fn constants_doubled(selectors: &[ActivePointSelector]) -> bool {
    let distinct: BTreeSet<&str> = selectors.iter().map(ActivePointSelector::constant).collect();
    distinct.len() != selectors.len()
}

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
        if !taken.insert(group.seat()) {
            return Err(ShellDeclarationRefusal::SeatSpellingDoubled);
        }
    }
    for group in core::iter::once(first).chain(rest.iter()) {
        for row in group.rows() {
            if !taken.insert(row.lens()) {
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
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

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
        pub(crate) fn established(
            first: ShellRenderIssue,
            rest: Vec<ShellRenderIssue>,
        ) -> Self {
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

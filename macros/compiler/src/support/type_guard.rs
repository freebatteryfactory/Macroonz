//! The support home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural rather than reviewed.
//! A name is parsed HERE, so a reference that names nothing is not a value anybody can hold.
//! A path is rooted HERE, so a rendered expression naming no crate is unwritable.
//! Proved cargo is promoted HERE, off a terminal's own delivery and compared against what that delivery carries, so a carrier holding tokens nobody proved is unwritable.
//! An assembly is built HERE, after the whole verification agreed, so there is no half-verified whole for a renderer to mistake for a verified one.
//! And a carrier is composed HERE, in one act, so its bytes are a function of the assembly the caller holds and of nothing beside it.

use super::super::establish::{carried_axes, consumption_issues, form_issues, root_issues};
use super::super::render;
use super::{
    ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, AxisCargo, BoundPath, CargoAxis,
    CrateFacing, DeclarationError, DeclaredCargo, DeferredCargo, DeliveryForm, EXPECTED_SCHEMA_ID,
    PATH_SEGMENT_LIMIT, ProvedCargo, SchemaId, ShellError, ShellName, SupportAssembly, SupportAxes,
    SupportName, SupportShell, WallName,
};
use crate::bounded::{Capped, Capping, NonEmpty, NonEmptyError};
use crate::closure::PartitionCargo;
use crate::expansion::Expansion;
use crate::identity::{self, ClosedExpansionId, Identity, PlanId};
use crate::kind::{Destination, Kind};
use crate::plan::Plan;
use crate::request::Door;
use crate::token::{GeneratedToken, GeneratedTree};

/// One lowercase hexadecimal digit for the low four bits of a byte.
///
/// The mask is the proof: four bits always name a digit, so the road is total and the fallback is a value no input reaches.
fn digit(nibble: u8) -> char {
    char::from_digit(u32::from(nibble & 0x0f), 16).unwrap_or('0')
}

impl SchemaId {
    /// The address these thirty-two bytes are.
    ///
    /// A `const fn`, because a producer writes its expectation down once beside the door it renders through rather than composing one per expansion.
    #[must_use]
    pub const fn pinned(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The bytes themselves, at full width.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl WallName {
    /// This name, parsed from the owner that declares a spelling and the spelling it carries.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::EmptyNamespace`], then [`DeclarationError::EmptyStem`].
    /// The checks are dependent and in that order, so exactly one cause is true of any refused name.
    pub fn named(namespace: &str, stem: &str) -> Result<Self, DeclarationError> {
        if namespace.is_empty() {
            return Err(DeclarationError::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(DeclarationError::EmptyStem);
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
    /// The path rooted at one crate, over the segments that follow it.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::SpellingNotAnIdentifier`] where a segment cannot name a rendered item — the root is typed as the facing, so every segment is an item step, and one outside the alphabet or on the keyword roster renders a path no consumer's compiler reads — [`DeclarationError::PathSegmentsAbsent`] where no segment was supplied — a path naming a crate and nothing in it names no item — and [`DeclarationError::PathSegmentsUnbounded`] where the segments outgrow the declared magnitude.
    pub fn rooted(facing: CrateFacing, segments: Vec<String>) -> Result<Self, DeclarationError> {
        for segment in &segments {
            if !rendered_name(segment.as_str()) {
                return Err(DeclarationError::SpellingNotAnIdentifier);
            }
        }
        let admitted: NonEmpty<String, PATH_SEGMENT_LIMIT> =
            NonEmpty::new(segments).map_err(|refusal| match refusal {
                NonEmptyError::Empty(_) => DeclarationError::PathSegmentsAbsent,
                NonEmptyError::Overflow(_) => DeclarationError::PathSegmentsUnbounded,
            })?;
        Ok(Self {
            facing,
            segments: admitted,
        })
    }

    /// Which crate this path is rooted at.
    #[must_use]
    pub const fn facing(&self) -> CrateFacing {
        self.facing
    }

    /// The segments after that crate, in the order they were declared; structurally at least one.
    #[must_use]
    pub fn segments(&self) -> &NonEmpty<String, PATH_SEGMENT_LIMIT> {
        &self.segments
    }

    /// How many segments follow the crate; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.count()
    }
}

impl SupportName {
    /// The exported address, read from the spelling an author wrote.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::SpellingNotAnIdentifier`] where the spelling cannot name a rendered item — not one Rust identifier, or a keyword the language already took: it is written into a consumer's target in identifier position, and either disagreement renders tokens that compiler reads as something else.
    pub fn declared(spelling: &str) -> Result<Self, DeclarationError> {
        if rendered_name(spelling) {
            Ok(Self(spelling.to_owned()))
        } else {
            Err(DeclarationError::SpellingNotAnIdentifier)
        }
    }

    /// The spelling this address carries.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.0.as_str()
    }
}

impl ShellName {
    /// The fixed prefix every exported carrier name carries.
    ///
    /// Two leading underscores and a namespaced stem: the name lands at the root of the consumer's own crate, so it reads as machinery rather than as anything a consumer wrote.
    pub const PREFIX: &'static str = "__macroonz_support_";

    /// How many bytes of the plan identity the mangled spelling carries.
    ///
    /// The whole identity, spelled as lowercase hexadecimal, which is what makes the collision-free sentence beside it true as written rather than true of a prefix.
    ///
    /// # Nonclaims
    ///
    /// The spelling is not the identity and nothing reads it back: no road parses hexadecimal into a plan identity, and every identity-bearing road uses the plan's own value.
    pub const KEY_BYTES: usize = 32;

    /// The mangled name one plan's carrier is exported under.
    ///
    /// Total: the prefix is a constant and the suffix is hexadecimal over bytes that always exist, so there is no count to read and no refusal to return.
    #[must_use]
    pub fn mangled(plan: PlanId) -> Self {
        let mut spelling = String::from(Self::PREFIX);
        for byte in plan.as_bytes().iter().take(Self::KEY_BYTES) {
            spelling.push(digit(byte.wrapping_shr(4)));
            spelling.push(digit(*byte));
        }
        Self { spelling }
    }

    /// The exported spelling a consumption target invokes this carrier by.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl DeferredCargo {
    /// Declare the tokens one opaque seat receives.
    ///
    /// A cargo of no tokens is admitted and stays distinct from an axis that carries none, because this road never turns one posture into the other.
    pub const fn deferred(tokens: GeneratedTree) -> Self {
        Self { tokens }
    }

    /// The tokens themselves.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tokens
    }
}

impl DeclaredCargo {
    /// Bind one stamped body to the matcher clauses it consumes.
    pub const fn declared(matched: GeneratedTree, stamped: GeneratedTree) -> Self {
        Self { matched, stamped }
    }

    /// Read one stamped body off the terminal that proved it, bound to the matcher clauses that body consumes.
    ///
    /// The declared axis takes a body somebody wrote, and [`DeclaredCargo::declared`] is that road; this one is for the body a descriptor terminal RENDERED, where the honest source is the terminal's own declaration-site delivery rather than a tree a door recomposed beside it.
    /// The matcher clauses stay the caller's, because the grammar that spells the body's metavariables is the one that knows which clauses bind them.
    ///
    /// # Errors
    ///
    /// Returns [`AssemblyIssue::CargoNotTheSourcesOwn`] where the terminal's declaration-site delivery carries nothing: a terminal that planned no member there proved no stamped body anybody could carry.
    pub fn stamped_from<K: Kind>(
        expansion: &Expansion<K>,
        matched: GeneratedTree,
    ) -> Result<Self, AssemblyError> {
        let source = expansion.identity();
        let Some(PartitionCargo::Carried(proved)) =
            expansion.emission().joined(Destination::DeclarationSite)
        else {
            return Err(AssemblyError::of(AssemblyIssue::CargoNotTheSourcesOwn {
                source,
                destination: Destination::DeclarationSite,
            }));
        };
        Ok(Self {
            matched,
            stamped: proved.tree().clone(),
        })
    }

    /// The clauses this delivery's invocation must supply.
    #[must_use]
    pub const fn matched(&self) -> &GeneratedTree {
        &self.matched
    }

    /// The body the gate's stamped seat carries.
    #[must_use]
    pub const fn stamped(&self) -> &GeneratedTree {
        &self.stamped
    }
}

impl ProvedCargo {
    /// Read one axis's cargo off the terminal that proved it.
    ///
    /// The caller says WHICH terminal and WHICH delivery and hands over the cargo it holds; this road reads that terminal's own delivery and refuses unless the cargo is exactly what it proved.
    /// A value of this type therefore carries parentage it was checked against rather than parentage it was told, and the declaration it stands over is read off the terminal's own plan rather than supplied beside it.
    ///
    /// # Errors
    ///
    /// Returns [`AssemblyIssue::CargoReachesASecondDestination`] where the named delivery is not the one this axis reads from, and [`AssemblyIssue::CargoNotTheSourcesOwn`] where that delivery carries nothing at all or carries other tokens.
    ///
    /// The two checks are dependent — there is no cargo to compare until the delivery is the axis's own — so exactly one of them is ever established.
    pub fn carried<K: Kind>(
        expansion: &Expansion<K>,
        axis: CargoAxis,
        destination: Destination,
        cargo: DeferredCargo,
    ) -> Result<Self, AssemblyError> {
        let source = expansion.identity();
        if axis.reads_from() != Some(destination) {
            return Err(AssemblyError::of(
                AssemblyIssue::CargoReachesASecondDestination { axis, destination },
            ));
        }
        let Some(PartitionCargo::Carried(proved)) = expansion.emission().joined(destination) else {
            return Err(AssemblyError::of(AssemblyIssue::CargoNotTheSourcesOwn {
                source,
                destination,
            }));
        };
        if proved.tree() != cargo.tree() {
            return Err(AssemblyError::of(AssemblyIssue::CargoNotTheSourcesOwn {
                source,
                destination,
            }));
        }
        Ok(Self {
            source,
            root: expansion.plan().account().commitment(),
            destination,
            digest: proved.digest(),
            cargo,
        })
    }

    /// The terminal this cargo was proved by.
    #[must_use]
    pub const fn source(&self) -> ClosedExpansionId {
        self.source
    }

    /// The declaration that terminal was planned over.
    #[must_use]
    pub const fn root(&self) -> Identity<identity::CapturedDeclaration> {
        self.root
    }

    /// The delivery it was read from.
    #[must_use]
    pub const fn destination(&self) -> Destination {
        self.destination
    }

    /// The digest that terminal's proof committed to over exactly these bytes.
    #[must_use]
    pub const fn digest(&self) -> Identity<identity::OutputBytes> {
        self.digest
    }

    /// The cargo whose tokens that delivery proved.
    pub const fn cargo(&self) -> &DeferredCargo {
        &self.cargo
    }
}

impl AssemblyError {
    /// The refusal one established issue makes.
    pub fn of(issue: AssemblyIssue) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }

    /// The refusal a pass whose checks co-establish makes.
    ///
    /// The caller arrives holding every issue its pass established, so the posture the body writes is about the REPORT and never about the pass: where the issues fit it carries all of them, and where they do not it carries what fits and counts the rest.
    pub fn over(first: AssemblyIssue, rest: Vec<AssemblyIssue>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }

    /// The first issue the pass established, which every refusal has.
    #[must_use]
    pub fn first_issue(&self) -> &AssemblyIssue {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the pass established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<AssemblyIssue, ASSEMBLY_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether this refusal carries every issue its pass established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

impl SupportAssembly {
    /// Compose one carrier out of closed outputs.
    ///
    /// # What is established here, and what was established before
    ///
    /// Each carried axis's cargo was already proved to be its own terminal's by [`ProvedCargo::carried`], which is why no loose tree reaches this.
    /// What remains are the facts about the WHOLE: one declaration under every axis, every terminal's delivery consumed once, and one delivery form with the seats that form requires.
    /// The gate pin is these services' own published constant, written here so no caller can offer another.
    ///
    /// # Errors
    ///
    /// Returns [`AssemblyError`] carrying every disagreement the pass established, together: an assembly failing on two declarations and a doubled consumption at once is repaired in one attempt rather than two.
    pub fn assembled(
        root: Identity<identity::CapturedDeclaration>,
        address: Option<SupportName>,
        axes: SupportAxes,
    ) -> Result<Self, AssemblyError> {
        let mut issues: Vec<AssemblyIssue> = Vec::new();

        // The carried set borrows the axes, and the axes move into the assembly
        // below, so the pass over them is closed before either happens.
        {
            let carried = carried_axes(&axes);
            issues.extend(root_issues(root, &carried));
            issues.extend(consumption_issues(&carried));
        }
        issues.extend(form_issues(&axes));

        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }
        Ok(Self {
            root,
            expectation: EXPECTED_SCHEMA_ID,
            address,
            declared: axes.declared,
            deferred: axes.deferred,
            bench: axes.bench,
        })
    }

    /// The declaration every axis of this assembly stands over.
    #[must_use]
    pub const fn root(&self) -> Identity<identity::CapturedDeclaration> {
        self.root
    }

    /// The published expectation the carrier's gate is matched against.
    #[must_use]
    pub const fn expectation(&self) -> SchemaId {
        self.expectation
    }

    /// The address a person invokes this carrier by, where an author declared one.
    #[must_use]
    pub const fn address(&self) -> Option<&SupportName> {
        self.address.as_ref()
    }

    /// What the declared axis carries, or what happened to whatever would have filled it.
    pub const fn declared(&self) -> &AxisCargo<DeclaredCargo> {
        &self.declared
    }

    /// What the deferred axis carries, on the same terms.
    pub const fn deferred(&self) -> &AxisCargo<ProvedCargo> {
        &self.deferred
    }

    /// What the bench axis carries, on the same terms.
    pub const fn bench(&self) -> &AxisCargo<ProvedCargo> {
        &self.bench
    }

    /// Which coupled pair of seats this assembly's gate invocation writes.
    ///
    /// Read off the axes rather than stated beside them, so a form naming one pair while the cargo fills the other is unrepresentable.
    #[must_use]
    pub const fn form(&self) -> DeliveryForm {
        match &self.bench {
            AxisCargo::Carried(_) => DeliveryForm::Benches,
            AxisCargo::Absent { .. } => DeliveryForm::Trials,
        }
    }

    /// Every terminal this assembly carries cargo from, in axis-roster order.
    ///
    /// The identities a rendered carrier's bytes stand over: the bytes are rendered from THIS value and from nothing else, so what an exported carrier delivers is what these terminals proved.
    pub fn sources(&self) -> impl Iterator<Item = ClosedExpansionId> {
        [proved_source(&self.deferred), proved_source(&self.bench)]
            .into_iter()
            .flatten()
    }
}

impl SupportShell {
    /// Render one carrier over what a carrier plan decided and what an assembly established.
    ///
    /// # The join, first
    ///
    /// The plan's declaration and the assembly's are compared before a seat is read.
    /// This is the seam at which "one carrier delivers one declaration's proved cargo" stops being a claim about the assembly alone: the assembly compared each carried axis against the declaration it was handed, and a carrier plan is not an axis — it is the vehicle, and nothing before this call has compared the vehicle's declaration against the cargo's.
    ///
    /// The comparison is made HERE rather than in whichever door joins the two, because this is the public road: a check in a wrapper would leave the road itself open to any caller holding a plan and somebody else's assembly.
    ///
    /// # One act
    ///
    /// The name, both seats, the gate around them, the exported definition around that, and the forwarding address are composed in one call, so an exported carrier's bytes are a function of the assembly the caller holds.
    ///
    /// # Errors
    ///
    /// Returns [`ShellError::NotOneDeclaration`] naming both declarations and electing neither, and [`ShellError::TreeUnbounded`] where any part of the composition outgrows the declared token magnitude.
    pub fn assembled<C: Kind>(
        carrier: &Plan<C>,
        assembly: &SupportAssembly,
        door: &Door,
    ) -> Result<Self, ShellError> {
        let planned = carrier.account().commitment();
        let stated = assembly.root();
        if planned != stated {
            return Err(ShellError::NotOneDeclaration { stated, planned });
        }
        let name = ShellName::mangled(carrier.identity());
        let form = assembly.form();
        let pin = render::expectation_roster(assembly.expectation())?;
        let body = render::gate_invocation(
            form,
            pin,
            stamped_cargo(assembly),
            opaque_cargo(assembly, form),
        )?;
        let matched = render::matcher(assembly.declared());
        let mut tokens =
            render::exported_shell(&name, &render::shell_sentence(door), matched, body)?;
        if let Some(address) = assembly.address() {
            tokens.extend(render::public_alias(
                &name,
                address,
                &render::alias_sentence(door),
            )?);
        }
        let tree = GeneratedTree::assembled(tokens)?;
        Ok(Self { name, tree })
    }

    /// The exported name a consumption target invokes this carrier by.
    #[must_use]
    pub const fn name(&self) -> &ShellName {
        &self.name
    }

    /// The rendered carrier, holding its cargo inert.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }

    /// The rendered carrier, taken whole.
    ///
    /// The road a renderer materializes this into a unit through, so the tree moves rather than being copied out from behind a reference.
    #[must_use]
    pub fn into_tree(self) -> GeneratedTree {
        self.tree
    }
}

/// The refusal one established issue list amounts to, or nothing where the list is empty.
///
/// One road for the whole verification, so no pass can establish issues and then walk on past them.
fn refused(issues: Vec<AssemblyIssue>) -> Option<AssemblyError> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(AssemblyError::over(first, established.collect()))
}

/// The terminal one proved axis reads from, where the axis carries anything.
fn proved_source(axis: &AxisCargo<ProvedCargo>) -> Option<ClosedExpansionId> {
    match axis {
        AxisCargo::Absent { .. } => None,
        AxisCargo::Carried(proved) => Some(proved.source()),
    }
}

/// What the gate's stamped seat is written from.
///
/// An axis nothing declared writes the seat EMPTY rather than leaving it out, because a gate arm that had to match two clause shapes would be two arms and one pin would open two doors.
fn stamped_cargo(assembly: &SupportAssembly) -> Vec<GeneratedToken> {
    match assembly.declared() {
        AxisCargo::Absent { .. } => Vec::new(),
        AxisCargo::Carried(cargo) => cargo.stamped().tokens().to_vec(),
    }
}

/// What the gate's opaque seat is written from, read off the axis this form's seat is filled by.
fn opaque_cargo(assembly: &SupportAssembly, form: DeliveryForm) -> Vec<GeneratedToken> {
    let axis = match form {
        DeliveryForm::Trials => assembly.deferred(),
        DeliveryForm::Benches => assembly.bench(),
    };
    match axis {
        AxisCargo::Absent { .. } => Vec::new(),
        AxisCargo::Carried(proved) => proved.cargo().tree().tokens().to_vec(),
    }
}

pub use crate::token::{rendered_identifier, rendered_name};

//! The diagnostic home's invariant nucleus: the one road that builds a related set, the two placement-safe roads that build diagnostics, and the roads its own values are read through.
//!
//! Declared inside `types.rs` as its own child, so the seats a caller may not write are reachable here and nowhere else.
//!
//! What lands here is what is about an ACT rather than about a value.
//! The related-set road takes the issue material — not a count, and not identities somebody else already derived — and builds the set, so the capping and the identities are two readings of one act.
//! The caller-placed road takes a refusal, a door, and the public placement vocabulary; the intrinsic road takes a refusal projection whose own type supplies its site.
//! Both end at the same private composition, so a line naming one position beside a seat holding another is unrepresentable.

use super::{
    Diagnostic, DiagnosticName, DiagnosticNameRefusal, DiagnosticSeats, Family, IntrinsicRefused,
    Line, LineSite, Observed, Phase, Placement, Refused, RelatedIdentity, RelatedSet, Repair,
    Route, Site, SiteCoordinate,
};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::project::{composed, witnessed};
use crate::identity::{
    Contract, Identity, RelatedBody, RelatedIssue, Role, ServiceEntry, Transcript, encode_bytes,
};
use crate::request::Door;
use crate::token::{SourceCoordinate, SpanHandle};

impl Family {
    /// Declare one family under its owner's namespace.
    ///
    /// The shape is `namespace/stem`, checked where the constant is written: a name that carries no namespace is a name two crates could write.
    ///
    /// # Panics
    ///
    /// Stops const evaluation — the build, for the `const` items this is written for — on a name without an interior `/`.
    #[must_use]
    pub const fn declared(name: &'static str) -> Self {
        assert!(
            interior_separator(name.as_bytes()),
            "a family name is namespace/stem, with material on both sides"
        );
        Self(name)
    }

    /// The name, exactly as declared.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.0
    }
}

impl DiagnosticName {
    /// Declares one lowercase ASCII kebab-case diagnostic name without normalizing it.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticNameRefusal::Empty`] where no name was supplied and [`DiagnosticNameRefusal::NotKebabCase`] where the spelling contains another character or a leading, trailing, or doubled separator.
    pub const fn declared(name: &'static str) -> Result<Self, DiagnosticNameRefusal> {
        if name.is_empty() {
            return Err(DiagnosticNameRefusal::Empty);
        }
        if !crate::identity::name_is_grammatical(name) {
            return Err(DiagnosticNameRefusal::NotKebabCase);
        }
        Ok(Self(name))
    }

    /// The name, exactly as declared.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.0
    }
}

/// Whether the first `/` in the bytes has material on both sides.
const fn interior_separator(name: &[u8]) -> bool {
    let mut rest = name;
    let mut ahead = 0_usize;
    while let Some((byte, remaining)) = rest.split_first() {
        if *byte == b'/' {
            return ahead != 0 && !remaining.is_empty();
        }
        ahead = ahead.saturating_add(1);
        rest = remaining;
    }
    false
}

/// The content one related identity is derived over, at either level.
///
/// The family name separates two spaces so the same bytes raised under two families never encode alike, and both members are framed so no two compositions share a preimage.
/// One composition serves both levels deliberately: what separates them is the subject, which is a segment of the derive-key context rather than a discriminant somebody could forget to write into a preimage.
fn related_content(family: Family, material: &[u8]) -> Vec<u8> {
    let mut content = Vec::new();
    encode_bytes(family.name().as_bytes(), &mut content);
    encode_bytes(material, &mut content);
    content
}

/// One related-issue identity over one established issue's material.
///
/// Private on purpose: an identity of this subject exists only as part of a set this file built.
/// The position is zero for every family: the family rides inside the content, framed, where a name fits.
fn issue_identity(family: Family, material: &[u8]) -> Identity<RelatedIssue> {
    Identity::derived(Transcript::rooted(
        Role::DiagnosticRelation,
        &related_content(family, material),
        0,
    ))
}

/// One related-body identity over the framing of a whole body's issues.
///
/// The same private discipline and the same seat, under the other subject.
/// The separation is deliberately not a byte inside the preimage: the subject rides in the derive-key context, so two levels over identical content are separated before a byte of that content is read.
fn body_identity(family: Family, material: &[u8]) -> Identity<RelatedBody> {
    Identity::derived(Transcript::rooted(
        Role::DiagnosticRelation,
        &related_content(family, material),
        0,
    ))
}

impl RelatedSet {
    /// The related set one refusal body amounts to, derived over that body's own issue material.
    ///
    /// A road taking a body identity and a set of per-issue identities as two arguments takes two halves that do not check each other: each is honestly derived on its own, so the pair can name one refusal's body over another refusal's issues and still read exactly like a set that belongs together.
    /// Deriving here removes the pairing instead of policing it.
    ///
    /// # Construction
    ///
    /// This is a mint site, so its grammar is stated in full.
    /// Both levels derive at [`Role::DiagnosticRelation`], rooted, at position zero, over
    ///
    /// ```text
    /// content = u64be(len(family)) || family || u64be(material.len()) || material
    /// ```
    ///
    /// where the family is its declared name's UTF-8 bytes, the material of an issue is that issue's own canonical bytes, and the material of the body is `u64be(issue.len()) || issue` for every issue in order, concatenated.
    /// The per-issue identities are derived first and the body's identity over exactly that material in exactly that order, so two different issue sets cannot reach one body identity and one issue set cannot reach two.
    /// The two subjects are `related-issue` and `related-body`; everything else an independent reader needs is on [`Transcript`].
    ///
    /// # Bounds
    ///
    /// The set overruns [`RELATED_ISSUE_LIMIT`](super::RELATED_ISSUE_LIMIT) at the boundary by exactly one, because the body's own identity sits ahead of the per-issue ones.
    /// Where that happens the body's identity is carried alone — a coarser commitment to the same refusal, never a shorter commitment to a different one — and the capping states how many per-issue identities are not there.
    ///
    /// Handed no issues, this road answers with [`RelatedSet::nothing_enumerated`] rather than deriving a body identity over empty material, so "nothing was enumerated" has one representation and two diagnostics that enumerated nothing compare equal.
    #[must_use]
    pub fn derived_over(family: Family, issues: &[Vec<u8>]) -> Self {
        if issues.is_empty() {
            return Self::nothing_enumerated();
        }
        let mut body_material = Vec::new();
        let mut per_issue = Vec::with_capacity(issues.len());
        for issue in issues {
            per_issue.push(RelatedIdentity::Issue(issue_identity(family, issue)));
            encode_bytes(issue, &mut body_material);
        }
        let body = RelatedIdentity::Body(body_identity(family, &body_material));
        let mut all = Vec::with_capacity(per_issue.len().saturating_add(1));
        all.push(body);
        all.append(&mut per_issue);
        match Bounded::new(all) {
            Ok(carried) => Self {
                carried,
                capping: Capping::Complete,
            },
            Err(_) => Self {
                carried: Bounded::from_array([body]),
                capping: Capping::Truncated {
                    omitted: issues.len(),
                },
            },
        }
    }

    /// The canonical empty relation: what a road that enumerated nothing amounts to.
    ///
    /// A single-cause refusal establishes one cause and enumerates nothing, so there is no per-issue set to fall short of: zero identities are carried and zero were dropped.
    ///
    /// # Nonclaims
    ///
    /// Emptiness here is a stated posture about an act that ran — the road looked, and there was nothing to enumerate.
    /// It is not an absent set, not a set that failed to build, and not a capping that dropped everything.
    #[must_use]
    pub const fn nothing_enumerated() -> Self {
        Self {
            carried: Bounded::empty(),
            capping: Capping::Complete,
        }
    }

    /// The identities the set carries, body first.
    #[must_use]
    pub fn carried(&self) -> &[RelatedIdentity] {
        self.carried.as_slice()
    }

    /// Whether that set names every established issue.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.capping
    }
}

impl Site {
    /// The site of an observation about one token of a captured declaration.
    ///
    /// The handle is the load-bearing half: the producer resolves it to the exact compiler span, and this compiler never does.
    /// The coordinate beside it is whatever that producer's table answered, including the typed statement that the table does not reach the handle — which is a fact about the TABLE and leaves the observation itself standing.
    pub const fn at_token(token: SpanHandle, coordinate: SiteCoordinate) -> Self {
        Self::AtToken { token, coordinate }
    }

    /// The site of an observation established before any capture existed to issue a handle.
    ///
    /// It takes no [`SpanHandle`], which is the whole point of the road: there is no seat here for a caller to fill with handle zero, and no branch in which one is invented.
    pub const fn before_capture(coordinate: SourceCoordinate) -> Self {
        Self::BeforeCapture { coordinate }
    }

    /// The site of an observation about the declaration as a whole.
    ///
    /// It takes nothing, which is the statement: there is no narrower token, so no handle and no coordinate exist to be answered — and none is invented for a machine reader to mistake for a resolved one.
    pub const fn whole_declaration() -> Self {
        Self::WholeDeclaration
    }

    /// The token this diagnostic points at, where a capture issued one.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing for a site established BEFORE a capture, because no table was built and no handle was issued, and with nothing for a WHOLE-DECLARATION site, because the refusal is not at a token.
    /// Each is a stated posture rather than a missing value: a handle answered here would index a table that never existed or name a spot the refusal is not about, and either would read exactly like an honest handle.
    #[must_use]
    pub const fn token(self) -> Option<SpanHandle> {
        match self {
            Self::AtToken { token, .. } => Some(token),
            Self::BeforeCapture { .. } | Self::WholeDeclaration => None,
        }
    }

    /// Where this diagnostic sits, where it sits at a place at all.
    ///
    /// The ONE place a pre-capture byte is lifted into the answered posture, and it lifts honestly: nothing was resolved because nothing needed resolving, and the coordinate's own role says which text the position counts into.
    /// A whole-declaration site answers with nothing — the refusal has no position inside the declaration, and a manufactured one would read exactly like a resolved coordinate.
    #[must_use]
    pub const fn coordinate(self) -> Option<SiteCoordinate> {
        match self {
            Self::AtToken { coordinate, .. } => Some(coordinate),
            Self::BeforeCapture { coordinate } => Some(SiteCoordinate::Resolved(coordinate)),
            Self::WholeDeclaration => None,
        }
    }
}

impl Route {
    /// The reproduction road one door offers.
    pub(crate) const fn through(entry: Identity<ServiceEntry>) -> Self {
        Self { entry }
    }

    /// The callable entry point that reaches this observation again.
    #[must_use]
    pub const fn entry(self) -> Identity<ServiceEntry> {
        self.entry
    }
}

impl Diagnostic {
    /// Project one caller-placed refused step into the diagnostic its door hands back.
    ///
    /// Every seat that could be written two ways is written once on this road: the line through [`composed`], the expected contract and the reproduction route off the door, and the site through the placement the caller states.
    /// The site is built once and read twice — the prose and the seat are projections of the same value.
    /// Refusal types whose site is intrinsic expose their own diagnostic road and do not implement this method's public bound.
    pub fn refused<E: Refused>(refusal: &E, door: &Door, placement: &Placement<'_>) -> Self {
        compose_diagnostic(refusal, door, placement_site(placement))
    }

    /// The step that was running.
    #[must_use]
    pub const fn phase(&self) -> Phase {
        self.phase
    }

    /// Where the observation sits.
    pub const fn site(&self) -> Site {
        self.site
    }

    /// The one line this diagnostic projects for a person.
    ///
    /// A projection and only a projection: nothing reads it back, and a frontend shows it rather than deciding from it.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.carried.summary
    }

    /// The contract that was expected to hold.
    #[must_use]
    pub fn expected(&self) -> Identity<Contract> {
        self.carried.expected
    }

    /// How what was found differs from it.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        self.observed
    }

    /// The other issues this one points at, and how that set was capped.
    #[must_use]
    pub fn related(&self) -> &RelatedSet {
        &self.carried.related
    }

    /// The owner-declared repairs that apply.
    #[must_use]
    pub fn repairs(&self) -> &[Repair] {
        self.carried.repairs.as_slice()
    }

    /// How to reach this observation again.
    #[must_use]
    pub fn route(&self) -> Route {
        self.carried.route
    }
}

/// Compose one intrinsically placed refusal without accepting a second site beside it.
pub(crate) fn intrinsic_diagnostic<E: IntrinsicRefused>(refusal: &E, door: &Door) -> Diagnostic {
    compose_diagnostic(refusal, door, refusal.site())
}

/// Compose one refusal over the site its lawful road established.
fn compose_diagnostic<E: Refused>(refusal: &E, door: &Door, site: Site) -> Diagnostic {
    let related = RelatedSet::derived_over(E::FAMILY, &refusal.related());
    let first = refusal.first();
    let line = Line {
        class: refusal.class(),
        first: &first,
        body: refusal.body(),
    };
    let composed_line = composed(door, &line, placement_line_site(site));
    Diagnostic {
        phase: E::PHASE,
        site,
        observed: refusal.observed(),
        carried: Box::new(DiagnosticSeats {
            summary: witnessed(&composed_line, related.capping()),
            expected: door.grammar(),
            related,
            repairs: refusal.repairs(),
            route: Route::through(door.entry()),
        }),
    }
}

/// The site one placement amounts to.
///
/// The whole-declaration placement answers with the whole-declaration site: the placement says there is nowhere narrower to point, and the site says exactly the same thing — no token, no coordinate, no manufactured first-token stand-in a machine reader could mistake for a resolved position.
fn placement_site(placement: &Placement<'_>) -> Site {
    match *placement {
        Placement::WholeDeclaration => Site::whole_declaration(),
        Placement::AtToken { token, spans } => {
            Site::at_token(token, SiteCoordinate::answered(spans.coordinate_of(token)))
        }
    }
}

/// What the composed line says about where the refusal sits — a projection of the site, so the prose and the seat cannot disagree.
fn placement_line_site(site: Site) -> LineSite {
    match site {
        Site::WholeDeclaration => LineSite::WholeDeclaration,
        Site::AtToken { coordinate, .. } => LineSite::At(coordinate),
        Site::BeforeCapture { coordinate } => LineSite::At(SiteCoordinate::Resolved(coordinate)),
    }
}

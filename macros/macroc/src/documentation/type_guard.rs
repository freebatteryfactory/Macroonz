//! The documentation home's invariant nucleus: every road that reaches a private
//! field, the coverage pass the facet roster quantifies, and the one road that
//! turns a pass's established issues into the pair a refusal body is built from.
//!
//! Declared inside `types.rs` as its own child, which is what makes the
//! one-plain-sentence law structural: the ONLY road to a summary refuses an empty
//! one, a multi-line one, an unfinished one, and one past the declared magnitude,
//! and there is no second road that seats one — so an item carrying a paragraph
//! where its summary belongs is a value nobody can write rather than a state a
//! reader has to notice.
//!
//! The surface is built here for the same reason. Documentation exists only where
//! the plan declared a member at the declaration site under its role, only where
//! every covered facet earned exactly one section and every section's facet was
//! covered, and only where every line could actually be written — so there is no
//! half-composed delivery for a reader to mistake for a whole one.
//!
//! The refusal BODY is DECLARED in the `seat` module below rather than in
//! `types.rs`, because Rust's privacy is MODULE-scoped and a seat declared beside
//! the rest of this home's declarations would put all of them inside the wall.

use super::super::plan::documentation_plan;
use super::super::render;
use super::{
    AuthoredLine, DocumentationDeclarationRefusal, DocumentationIssue, DocumentationTextLimit,
    DocumentedItem, DocumentedSection, DocumentedSurface, PlainSentence, SectionLine,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, FacetLimit, GeneratedUnitSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::{DocumentationProjection, MemberDestination, ProjectionPlan};
use crate::token::GeneratedTree;
use threadpak::declaration::Facet;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

/// The full stop the one plain sentence ends on.
///
/// One character, stated here rather than written into a comparison, so the law
/// and the sentence that states it read the same value.
const SENTENCE_END: char = '.';

// ---------------------------------------------------------------------------
// The owner's own doc text.
// ---------------------------------------------------------------------------

impl PlainSentence {
    /// The one plain sentence, as the owner wrote it.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentationDeclarationRefusal::EmptyText`] where the sentence
    /// carries no character, [`DocumentationDeclarationRefusal::TextCarriesLineBreak`]
    /// where it carries a line break,
    /// [`DocumentationDeclarationRefusal::TextUnbounded`] where it outgrows the
    /// declared magnitude, and
    /// [`DocumentationDeclarationRefusal::SentenceNotEnded`] where it does not end
    /// in a full stop.
    ///
    /// The checks are dependent and in that order — there is no ending to read
    /// until there are characters — so exactly one cause is true of any refused
    /// sentence.
    pub fn stated(text: &str) -> Result<Self, DocumentationDeclarationRefusal> {
        admitted_text(text)?;
        if !text.ends_with(SENTENCE_END) {
            return Err(DocumentationDeclarationRefusal::SentenceNotEnded);
        }
        Ok(Self {
            text: text.to_owned(),
        })
    }

    /// The sentence, for a rendering to write out unchanged.
    #[must_use]
    pub fn shown(&self) -> &str {
        self.text.as_str()
    }
}

impl AuthoredLine {
    /// One line of the owner's own doc text.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentationDeclarationRefusal::EmptyText`],
    /// [`DocumentationDeclarationRefusal::TextCarriesLineBreak`], and
    /// [`DocumentationDeclarationRefusal::TextUnbounded`] on exactly
    /// [`PlainSentence::stated`]'s terms.
    ///
    /// It carries no full-stop law: a line inside a section may lawfully be a
    /// list item, a table row, or a fenced code line, and none of those is a
    /// sentence.
    pub fn written(text: &str) -> Result<Self, DocumentationDeclarationRefusal> {
        admitted_text(text)?;
        Ok(Self {
            text: text.to_owned(),
        })
    }

    /// The line, for a rendering to write out unchanged.
    #[must_use]
    pub fn shown(&self) -> &str {
        self.text.as_str()
    }
}

impl DocumentedSection {
    /// One earned section.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentationDeclarationRefusal::LinesAbsent`] where no line was
    /// supplied — a heading over nothing explains nothing — and
    /// [`DocumentationDeclarationRefusal::LinesUnbounded`] where the lines outgrow
    /// the declared magnitude.
    ///
    /// The door reads a runtime count because a caller arrives holding a list;
    /// the VALUE cannot be empty at all, because the seat behind this road carries
    /// a first line by signature.
    pub fn written(
        facet: Facet,
        heading: AuthoredLine,
        lines: Vec<SectionLine>,
    ) -> Result<Self, DocumentationDeclarationRefusal> {
        let mut supplied = lines.into_iter();
        let Some(first) = supplied.next() else {
            return Err(DocumentationDeclarationRefusal::LinesAbsent);
        };
        let rest: Vec<SectionLine> = supplied.collect();
        let lines = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| DocumentationDeclarationRefusal::LinesUnbounded)?;
        Ok(Self {
            facet,
            heading,
            lines,
        })
    }

    /// The facet that earns this section.
    #[must_use]
    pub const fn facet(&self) -> Facet {
        self.facet
    }

    /// The owner's own heading for it.
    #[must_use]
    pub const fn heading(&self) -> &AuthoredLine {
        &self.heading
    }

    /// The lines under it, in the order the owner wrote them; structurally at
    /// least one.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: it is the order a reader reads, and prose reordered
    /// is prose rewritten.
    pub fn lines(&self) -> impl Iterator<Item = &SectionLine> {
        self.lines.iter()
    }

    /// How many lines the section carries; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Always `false`: a section with no line is unrepresentable.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl DocumentedItem {
    /// The complete documentation material one item carries.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentationDeclarationRefusal::SectionsUnbounded`] where the
    /// sections outgrow the declared magnitude.
    ///
    /// An EMPTY roster is admitted and is a stated fact: an item whose plan covers
    /// no facet owes exactly its one plain sentence, and that item is fully
    /// documented. Whether the roster AGREES with the plan is the coverage pass's
    /// question, not this door's.
    pub fn documented(
        summary: PlainSentence,
        sections: Vec<DocumentedSection>,
    ) -> Result<Self, DocumentationDeclarationRefusal> {
        let sections = Bounded::admitted_const(
            sections,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| DocumentationDeclarationRefusal::SectionsUnbounded)?;
        Ok(Self { summary, sections })
    }

    /// The one plain sentence this item opens with.
    #[must_use]
    pub const fn summary(&self) -> &PlainSentence {
        &self.summary
    }

    /// The earned sections, in the order the owner wrote them.
    pub fn sections(&self) -> impl Iterator<Item = &DocumentedSection> {
        self.sections.iter()
    }

    /// How many sections the item carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Whether the item carries no section at all — a lawful, stated posture.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

impl DocumentedSurface {
    /// Where documentation material lands, stated once as a constant rather than
    /// carried as a seat that could say something else.
    ///
    /// The material is an attribute run spliced ahead of the owner's own item, so
    /// it lands exactly where that item does.
    pub const DESTINATION: MemberDestination = MemberDestination::AtDeclarationSite;

    /// Compose one item's documentation.
    ///
    /// The order is the road: what the plan decided, then the coverage pass over
    /// the plan's facet roster, then the rendering — so material never exists that
    /// the passes did not agree on.
    ///
    /// # Errors
    ///
    /// Returns the coverage family naming the plan's disagreement (the role was
    /// not planned, or its member lands somewhere other than the declaration
    /// site), the coverage pass's (a covered facet nobody wrote, a section on a
    /// facet nobody covered, or two sections under one facet), or the rendering's
    /// (a facet name the machine's roster does not declare, or material past the
    /// declared token magnitude).
    ///
    /// The plan pass, the coverage pass, and the rendering are DEPENDENT — there
    /// is nothing to cover until the plan has been read and nothing to render
    /// until the coverage agrees — so a plan issue never co-establishes with a
    /// coverage one. Coverage issues co-establish freely with each other, which is
    /// why the body is a collection.
    pub fn composed(
        plan: &ProjectionPlan<DocumentationProjection>,
        item: &DocumentedItem,
    ) -> Result<Self, DocumentationCoverage> {
        let stated = documentation_plan(plan).map_err(sole)?;
        if let Some((first, rest)) =
            DocumentationIssue::established(coverage_issues(&stated.facets, item))
        {
            return Err(DocumentationCoverage::established(first, rest));
        }
        let tree = render::documented_item(item).map_err(sole)?;
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin,
            tree,
        })
    }

    /// The rendered role this material stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
        self.role
    }

    /// The planned member's semantic key this material answers to.
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

    /// The trail this material walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// The rendered attribute run — the doc material the owner's item carries.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

impl DocumentationIssue {
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
// The passes that reach a private seat, and the seat itself.
// ---------------------------------------------------------------------------

/// The three checks every piece of owner doc text passes, in the order a refusal
/// establishes them.
fn admitted_text(text: &str) -> Result<(), DocumentationDeclarationRefusal> {
    if text.is_empty() {
        return Err(DocumentationDeclarationRefusal::EmptyText);
    }
    if text.chars().any(|character| character == '\n' || character == '\r') {
        return Err(DocumentationDeclarationRefusal::TextCarriesLineBreak);
    }
    if text.len() > DocumentationTextLimit::MAX {
        return Err(DocumentationDeclarationRefusal::TextUnbounded);
    }
    Ok(())
}

/// The coverage pass: what the plan's facet roster and the item's sections say
/// about each other.
///
/// The roster is the QUANTIFIER in both directions — every covered facet is asked
/// whether exactly one section was earned by it, and every section is asked
/// whether its facet was covered — so neither side can quietly be the whole
/// answer.
fn coverage_issues(
    facets: &Bounded<Facet, FacetLimit>,
    item: &DocumentedItem,
) -> Vec<DocumentationIssue> {
    let mut issues: Vec<DocumentationIssue> = Vec::new();
    for facet in facets.iter() {
        let written = item
            .sections()
            .filter(|section| section.facet() == *facet)
            .count();
        if written == 0 {
            issues.push(DocumentationIssue::CoveredFacetNotWritten { facet: *facet });
        } else if written > 1 {
            issues.push(DocumentationIssue::FacetSectionDoubled { facet: *facet });
        }
    }
    for section in item.sections() {
        if !facets.iter().any(|facet| *facet == section.facet()) {
            issues.push(DocumentationIssue::SectionFacetNotCovered {
                facet: section.facet(),
            });
        }
    }
    issues
}

/// One established issue as the body a refusal is built from.
fn sole(issue: DocumentationIssue) -> DocumentationCoverage {
    DocumentationCoverage::established(issue, Vec::new())
}

pub use seat::DocumentationCoverage;

mod seat {
    use super::super::{DocumentationIssue, DocumentationIssueLimit};
    use crate::plane::AuthoringLimitProfile;
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The documentation-coverage refusal family body.
    ///
    /// Independent members: a plan may cover several facets nobody wrote while an
    /// item writes several sections nobody covered, so no primary issue is ever
    /// elected.
    #[must_use = "a refusal family body carries every disagreement the coverage passes established"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct DocumentationCoverage {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue its pass established
        /// or names how many stand outside that bound. One seat rather than two,
        /// because a coverage claim seated beside its body is a claim that can be
        /// swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record hands
        /// the whole record back as a literal, so any holder of a body built for
        /// one pass could write it into another pass's refusal. Read back through
        /// [`DocumentationCoverage::body`].
        body: AdmittedPrefix<DocumentationIssue, DocumentationIssueLimit>,
    }

    impl DocumentationCoverage {
        /// The body a coverage pass refuses with.
        ///
        /// Each pass walks its whole subject before a body exists, so the posture
        /// here is about the REPORT rather than the pass: where every established
        /// issue fits the declared bound the body carries all of them; where it
        /// does not, the body carries what the bound holds and names how many
        /// established issues stand outside it.
        ///
        /// Reaches the guard file and no further, so a body exists only where one
        /// of the passes beside it ran.
        pub(super) fn established(
            first: DocumentationIssue,
            rest: Vec<DocumentationIssue>,
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
        /// Borrowed and never owned, for the reason the machine's refusal home
        /// borrows its carry: an owned body is a value a caller can seat under
        /// another refusal, which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<DocumentationIssue, DocumentationIssueLimit> {
            &self.body
        }
    }
}

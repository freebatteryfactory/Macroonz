//! Reading one captured declaration's documentation rows into the documentation
//! home's own material.
//!
//! # Whose vocabulary this is
//!
//! Every value this file produces is the DOCUMENTATION home's, built through
//! that home's own doors: a [`PlainSentence`] through the door that enforces the
//! one-plain-sentence law, a [`DocumentedItem`] through the door that admits its
//! sections, and a [`ProjectionDisposition`] for the election this profile does
//! not offer. Nothing here declares a new documentation word, and nothing here
//! composes prose: the summary an item opens with is the OWNER's own family-seat
//! line, carried unchanged, and a sentence this home composed would trace to
//! neither of the two sources the documentation home admits.
//!
//! # How far the road goes, and where it stops
//!
//! A captured documentation row states three things: which declaration it was
//! written on, the text it carries, and the token it sits at. A
//! [`DocumentedSection`] wants three others: the FACET that earns it, a HEADING,
//! and the lines under that heading. Two of those a row simply lacks, and the
//! third — the facet — is an ELECTION about what a sentence means, which this
//! home is forbidden to make: a capture reads a declaration and writes down what
//! it already said.
//!
//! So the road wires exactly the mapping the existing grammar admits and states
//! the rest as a typed posture:
//!
//! - the FAMILY seat's prose is the item's one plain sentence, where it satisfies
//!   that law;
//! - the item carries NO earned section, which the documentation home admits as
//!   a stated fact — an item whose plan covers no facet owes exactly its one
//!   plain sentence, and that item is fully documented;
//! - facet election is answered with
//!   [`ProjectionDisposition::UnavailableUnderProfile`] over the declared
//!   Rust-declaration profile, because that is what is true: this derive reads a
//!   declaration under its declared compiler profile, and that profile reads
//!   tokens rather than what prose MEANS.
//!
//! The VARIANT rows are read and are not written into the item, and that is the
//! same statement: a variant's prose is about one cause of the family, and the
//! documentation home has no seat for a per-cause section that is not earned by
//! a facet. They stay on the captured surface, where a reader that wants them
//! reads them, and they are inside the documentation commitment either way.

use super::account::profile_does_not_offer;
use super::types::{DocumentedDeclaration, RefusalDeriveSurface};
use crate::documentation::{
    DocumentationDeclarationRefusal, DocumentedItem, PlainSentence,
};
use crate::planning::ProjectionDisposition;

/// What one captured declaration's documentation rows read as.
///
/// Two postures, and they are different observations rather than one with a
/// missing half. A declaration whose FAMILY seat carries an admissible sentence
/// has documentation material a projection can be planned over; one that carries
/// no family-seat row at all has none, and this home composes none — a summary
/// invented here would be a claim about the owner's declaration the owner did
/// not make.
///
/// Neither posture is a refusal. A declaration that documented nothing is a
/// lawful declaration, and the derive's own road does not stop for it.
#[must_use = "a documentation reading is either material a projection is planned over or the stated absence of it"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapturedDocumentationReading {
    /// The family seat carried a line the one-plain-sentence law admits, so the
    /// item is that sentence and no earned section.
    Documented {
        /// The documentation material, ready for a documentation projection's
        /// composition.
        item: DocumentedItem,
        /// What happened to facet election, which decides what a section is
        /// earned by.
        facets: ProjectionDisposition,
    },
    /// The declaration carries no family-seat row, so there is no owner sentence
    /// for an item to open with.
    NotDocumented {
        /// Why no item was read. Nobody asked for one: the author wrote no
        /// family-level prose, and this home writes none on an author's behalf.
        because: ProjectionDisposition,
    },
}

/// Read one captured surface's documentation rows.
///
/// # The summary road
///
/// The FIRST family-seat row is the item's summary. First rather than joined,
/// because a summary is one LINE and each captured row is one written line: a
/// road that joined several rows into one sentence would be composing prose out
/// of an author's paragraph, and a road that picked a later row would be
/// electing which of an author's lines opens their item.
///
/// # Errors
///
/// Returns [`DocumentationDeclarationRefusal`] where the family seat's line is
/// not the one plain sentence that home's law admits — empty, carrying a line
/// break, past the declared magnitude, or not ending in a full stop. It is the
/// documentation home's own refusal, unwrapped and unwrapped only: this home
/// states no second opinion about what a sentence is.
pub fn documented(
    surface: &RefusalDeriveSurface,
) -> Result<CapturedDocumentationReading, DocumentationDeclarationRefusal> {
    let Some(written) = surface
        .documentation()
        .find(|row| row.declared_on() == &DocumentedDeclaration::Family)
    else {
        return Ok(CapturedDocumentationReading::NotDocumented {
            because: ProjectionDisposition::NotRequested,
        });
    };
    let summary = PlainSentence::stated(written.text())?;
    // No section, and the empty roster is the documentation home's own stated
    // posture rather than a shorter item: what a section is EARNED by is a
    // facet, and electing one is the meaning decision below.
    let item = DocumentedItem::documented(summary, Vec::new())?;
    Ok(CapturedDocumentationReading::Documented {
        item,
        facets: facet_election(),
    })
}

/// What happened to facet election, as a typed posture with its ground stated.
///
/// # The ground
///
/// The derive reads a declaration under its DECLARED COMPILER PROFILE, and that
/// profile reads tokens: an attribute, its `doc` word, and the text inside it.
/// Which of the machine's semantic facets a piece of that text COVERS is a fact
/// about what the text means, and this profile offers no reading of meaning at
/// all — so the honest answer is that the selected profile does not offer the
/// projection, named at that profile and at its version.
///
/// It is not a refusal, and it is not an absence. A refusal would stop a lawful
/// derivation over a lawful declaration, and an absence would read as though
/// somebody had forgotten to decide. This is a decision, recorded.
///
/// The seat that closes it is a facet the owner DECLARES on the declaration
/// rather than one this home reads out of prose — at which point the election
/// stops being an election.
///
/// # One construction, two readings
///
/// The posture itself is [`profile_does_not_offer`] — the standing this door's
/// declared compiler profile has for everything it does not offer — read rather
/// than spelled a second time here. The two questions are genuinely different:
/// that road answers what happened to a whole projection kind, and this one
/// answers what happened to the election a section is earned by. The ANSWER is
/// the same profile at the same version in both cases, and a profile bump that
/// moved one of them and not the other would be two answers to one fact.
fn facet_election() -> ProjectionDisposition {
    profile_does_not_offer()
}

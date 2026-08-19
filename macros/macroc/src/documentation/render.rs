//! The token half of the road: the summary attribute, the earned sections, and
//! the one sentence each typed fact composes to.
//!
//! # Tokens, not text
//!
//! Doc material is rendered as `#[doc = "…"]` ATTRIBUTES rather than as `///`
//! comment characters, because a comment is trivia a token stream does not carry
//! and an attribute is an item a token stream does. Nothing here composes Rust
//! source; the Rust a person reads is
//! [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected), a
//! projection of what is emitted.
//!
//! The text literal is the one literal arm the generated-token roster carries, and
//! it is exactly the arm doc material needs — so this home renders its whole
//! delivery without ever reaching for an arm that is not there.
//!
//! # Never invented prose
//!
//! Every sentence this file writes comes from one of exactly two places: the
//! owner's own text, written out unchanged, or one typed fact composed from typed
//! values at the moment the line is asked for. There is no third road, and
//! [`SectionLine`] is the shape that says so.
//!
//! A composed sentence is never STORED. It is written when the material is
//! rendered and read back by nobody, so a documented item whose prose contradicts
//! its typed fact is not a value anybody can hold — the same discipline the
//! explanation station's own human projection stands under.
//!
//! # The one thing this file cannot spell
//!
//! [`DocumentedFact::CoveredFacet`] asks for a facet's NAME, and the machine's
//! facet roster declares none — it is a plain six-arm enum with no stable name and
//! no described projection. Rendering the Rust variant's spelling instead is NOT
//! the repair: a spelling taken from a variant renames the prose whenever somebody
//! refactors, which is exactly what a declared name exists to prevent. So the fact
//! refuses, naming the facet, and `FACT_ROSTER` in `type_contract.rs` names the
//! seat that closes it.
//!
//! # The item is not rendered here
//!
//! What this file writes is the doc material an item CARRIES. The item is the
//! owner's, and a projection that emitted it would be a second declaration of
//! something the owner already declared once.

use super::{
    AuthoredLine, DocumentationIssue, DocumentedFact, DocumentedItem, DocumentedSection,
    SectionLine,
};
use crate::plane::{GeneratedTokenLimit, OwnerFactRef};
use crate::planning::CauseAnchoring;
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::types::ConstLimit;

/// The mark one section heading opens with.
///
/// One `#` and a space: a top-level heading inside an item's own documentation,
/// which is the level rustdoc's own conventions put a section at.
pub const HEADING_MARK: &str = "# ";

/// The blank documentation line that separates a section from what came before
/// it.
///
/// An empty attribute rather than an absent one, because rustdoc reads a blank
/// line as a paragraph break and a missing one as a continuation — so the
/// separation has to be WRITTEN, not left out.
pub const BLANK_LINE: &str = "";

/// The issue a tree that outgrew the declared token magnitude amounts to.
#[must_use]
pub fn unbounded() -> DocumentationIssue {
    DocumentationIssue::DocumentationTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`DocumentationIssue::DocumentationTreeUnbounded`] where the group
/// carries more tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, DocumentationIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One `#[doc = "…"]` attribute, as the tokens that spell it.
///
/// # Errors
///
/// Returns [`DocumentationIssue::DocumentationTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn doc_attribute(text: &str) -> Result<Vec<GeneratedToken>, DocumentationIssue> {
    let body = vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(text),
    ];
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// The blank documentation line, as the attribute that spells it.
///
/// # Errors
///
/// Returns [`DocumentationIssue::DocumentationTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn blank() -> Result<Vec<GeneratedToken>, DocumentationIssue> {
    doc_attribute(BLANK_LINE)
}

/// One section's heading line, as the owner wrote it under the heading mark.
///
/// The words are the owner's and the mark is this home's: a heading composed here
/// would be a sentence tracing to neither of the two admitted sources.
#[must_use]
pub fn heading_line(heading: &AuthoredLine) -> String {
    let mut rendered = String::from(HEADING_MARK);
    rendered.push_str(heading.shown());
    rendered
}

/// Thirty-two bytes as lowercase hexadecimal, for a sentence that names an
/// identity.
///
/// A rendering and only a rendering: nothing reads it back, and every road that
/// compares identities compares the identities.
#[must_use]
pub fn hex(material: &[u8; 32]) -> String {
    let mut rendered = String::new();
    for byte in material {
        rendered.push_str(&format!("{byte:02x}"));
    }
    rendered
}

/// The sentence one typed fact composes to.
///
/// Composed from the fact's own typed values at the moment it is asked for, and
/// never stored.
///
/// # Errors
///
/// Returns [`DocumentationIssue::FacetNameNotDeclared`] for
/// [`DocumentedFact::CoveredFacet`] under the vocabularies as they stand: the
/// machine's facet roster declares no stable name, so a sentence naming a facet
/// would be these services legislating a spelling inside a vocabulary the machine
/// owns.
pub fn fact_sentence(fact: &DocumentedFact) -> Result<String, DocumentationIssue> {
    match fact {
        DocumentedFact::ProjectionKindName { name } => {
            Ok(format!("The projection kind is `{name}`."))
        }
        DocumentedFact::CausingDeclaration { anchoring } => Ok(causing_sentence(*anchoring)),
        DocumentedFact::OutputIdentity { key } => {
            let identity = hex(key.as_bytes());
            Ok(format!("The planned output identity is `{identity}`."))
        }
        DocumentedFact::Assumption { fact: cited } => Ok(assumption_sentence(*cited)),
        DocumentedFact::CoveredFacet { facet } => {
            Err(DocumentationIssue::FacetNameNotDeclared { facet: *facet })
        }
    }
}

/// The sentence one anchored cause address composes to.
///
/// The two postures never read alike, because they are not the same claim: a
/// linked fragment is an address the machine minted, and a captured declaration is
/// the token material one expansion was handed.
#[must_use]
pub fn causing_sentence(anchoring: CauseAnchoring) -> String {
    match anchoring {
        CauseAnchoring::Declaration(fragment) => {
            let identity = hex(fragment.as_bytes());
            format!("The declaration fragment that caused this is `{identity}`.")
        }
        CauseAnchoring::CapturedDeclaration(captured) => {
            let identity = hex(captured.as_bytes());
            format!("The captured declaration that caused this is `{identity}`.")
        }
    }
}

/// The sentence one cited owner fact composes to.
///
/// A DECLARED citation names the home and the fact by the names their owner wrote
/// down, and a MINTED one names the identities the machine minted. Neither is
/// derived here, and neither stands in for the other.
#[must_use]
pub fn assumption_sentence(cited: OwnerFactRef) -> String {
    match cited {
        OwnerFactRef::Declared(named) => {
            let home = named.home;
            let fact = named.fact;
            format!("This rests on the `{home}` home's declared fact `{fact}`.")
        }
        OwnerFactRef::Minted { home, fact } => {
            let owner = hex(home.as_bytes());
            let declared = hex(fact.as_bytes());
            format!("This rests on owner fact `{declared}` of home `{owner}`.")
        }
    }
}

/// The line one section line renders to: the owner's own text, or one typed
/// fact's sentence.
///
/// # Errors
///
/// Returns [`DocumentationIssue::FacetNameNotDeclared`] on exactly
/// [`fact_sentence`]'s terms.
pub fn section_line(line: &SectionLine) -> Result<String, DocumentationIssue> {
    match line {
        SectionLine::Authored(authored) => Ok(authored.shown().to_owned()),
        SectionLine::Fact(fact) => fact_sentence(fact),
    }
}

/// One earned section: a blank line, the heading, a blank line, then each line.
///
/// # Errors
///
/// Returns [`DocumentationIssue::FacetNameNotDeclared`] where a line asks for a
/// facet's name, and [`DocumentationIssue::DocumentationTreeUnbounded`] where the
/// section outgrows the declared token magnitude.
pub fn section(written: &DocumentedSection) -> Result<Vec<GeneratedToken>, DocumentationIssue> {
    let mut tokens = blank()?;
    tokens.extend(doc_attribute(&heading_line(written.heading()))?);
    tokens.extend(blank()?);
    for line in written.lines() {
        tokens.extend(doc_attribute(&section_line(line)?)?);
    }
    Ok(tokens)
}

/// The complete doc material one item carries: the one plain sentence, then every
/// earned section in the order the owner wrote them.
///
/// # Errors
///
/// Returns [`DocumentationIssue::FacetNameNotDeclared`] where a line asks for a
/// facet's name, and [`DocumentationIssue::DocumentationTreeUnbounded`] where the
/// material outgrows the declared token magnitude.
///
/// The walk STOPS at the first line it cannot write, and does not gather the rest:
/// a partially rendered attribute run is doc material nobody wrote, and handing
/// one back would put prose on an item that says less than the owner declared.
pub fn documented_item(item: &DocumentedItem) -> Result<GeneratedTree, DocumentationIssue> {
    let mut tokens = doc_attribute(item.summary().shown())?;
    for written in item.sections() {
        tokens.extend(section(written)?);
    }
    GeneratedTree::assembled(tokens).map_err(|_| unbounded())
}

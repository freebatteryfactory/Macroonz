#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::{documentation_plan, explanation_answers, explanation_view};
pub use render::{
    BLANK_LINE, HEADING_MARK, assumption_sentence, blank, causing_sentence, doc_attribute,
    documented_item, fact_sentence, group, heading_line, hex, section, section_line, unbounded,
};
pub use type_contract::{FACT_ROSTER, FactSource, FactSpelling};
pub use types::{
    AuthoredLine, DocumentationCoverage, DocumentationDeclarationRefusal,
    DocumentationExplanationAnchors, DocumentationIssue, DocumentationIssueLimit,
    DocumentationLineLimit, DocumentationPlan, DocumentationSectionLimit, DocumentationTextLimit,
    DocumentedFact, DocumentedItem, DocumentedSection, DocumentedSurface, PlainSentence,
    SectionLine,
};

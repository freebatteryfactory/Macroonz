//! Constructors and readers for the network declaration vocabulary.

use super::{
    DisciplineRow, FaultRow, LinkRow, NetworkCaptureError, NetworkDeclaration, ScheduleRow,
};
use crate::descriptor::{CaptureCause, Grammar, HelperRefusal};
use crate::token::SpanHandle;

impl LinkRow {
    /// One drawn link, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn drawn(name: String, from: String, to: String) -> Self {
        Self { name, from, to }
    }

    /// The spelling this link is referred to by.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The sending node's spelling.
    #[must_use]
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The receiving node's spelling.
    #[must_use]
    pub fn to(&self) -> &str {
        &self.to
    }
}

impl DisciplineRow {
    /// One link's gathered phrases, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn gathered(link: LinkRow, faults: Vec<FaultRow>) -> Self {
        Self { link, faults }
    }

    /// The link this discipline governs.
    #[must_use]
    pub const fn link(&self) -> &LinkRow {
        &self.link
    }

    /// The phrases, in authored order.
    #[must_use]
    pub fn faults(&self) -> &[FaultRow] {
        &self.faults
    }

    /// One more phrase, gathered in authored order.
    pub(crate) fn push(&mut self, fault: FaultRow) {
        self.faults.push(fault);
    }
}

impl ScheduleRow {
    /// One declared schedule, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn declared(name: String, disciplines: Vec<DisciplineRow>) -> Self {
        Self { name, disciplines }
    }

    /// The spelling this schedule's builder is named by.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The disciplines, in first-mention link order.
    #[must_use]
    pub fn disciplines(&self) -> &[DisciplineRow] {
        &self.disciplines
    }
}

impl NetworkDeclaration {
    /// The complete payload, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn read(
        module: String,
        namespace: String,
        nodes: Vec<String>,
        links: Vec<LinkRow>,
        schedules: Vec<ScheduleRow>,
    ) -> Self {
        Self {
            module,
            namespace,
            nodes,
            links,
            schedules,
        }
    }

    /// The module the builders land in.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// The namespace every declared name is owned under.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The node spellings, in authored order.
    #[must_use]
    pub fn nodes(&self) -> &[String] {
        &self.nodes
    }

    /// The links, in authored order.
    #[must_use]
    pub fn links(&self) -> &[LinkRow] {
        &self.links
    }

    /// The schedules, in authored order.
    #[must_use]
    pub fn schedules(&self) -> &[ScheduleRow] {
        &self.schedules
    }
}

impl NetworkCaptureError {
    /// One refusal the network grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}

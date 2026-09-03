//! The table roads: how a binding is married, and how the authored world and a staged view are closed.

use crate::descriptor::types::{
    AuthoredTable, AuthoredTableName, AuthoredTableRefusal, Binding, BindingRefusal,
    ExecutableAttachment, Origin, Provenance, Row, StagedTableRefusal, StagedTableView,
    TablePosture, TableView, TrialKey,
};
use std::collections::BTreeSet;

impl<Invocation, Conclusion> Binding<Invocation, Conclusion> {
    /// One row married to the attachment that executes it.
    ///
    /// # Errors
    ///
    /// Refuses a row and an attachment naming different subjects, then different checks — the marriage is what closes the seam a hidden row-to-function registry would open.
    /// Refuses, last, a row carrying producer facts inside a binding that names no schema the producer emitted against.
    pub fn bound(
        row: Row,
        attachment: ExecutableAttachment<Invocation, Conclusion>,
        provenance: Provenance,
    ) -> Result<Self, BindingRefusal> {
        if row.subject() != attachment.subject() {
            return Err(BindingRefusal::SubjectMismatch {
                row: row.subject(),
                attachment: attachment.subject(),
            });
        }
        if row.check() != attachment.check() {
            return Err(BindingRefusal::CheckMismatch {
                row: row.check(),
                attachment: attachment.check(),
            });
        }
        if let (Origin::Generated(_), Provenance::Unproduced) = (row.origin(), provenance) {
            return Err(BindingRefusal::GeneratedWithoutSchemaPin);
        }
        Ok(Self {
            row,
            attachment,
            provenance,
        })
    }

    /// The row this binding carries.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The attachment that executes it.
    #[must_use]
    pub const fn attachment(&self) -> &ExecutableAttachment<Invocation, Conclusion> {
        &self.attachment
    }

    /// Whether a producer stands behind this binding, and which schema it emitted against.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// The identity that decides whether two bindings are one trial.
    #[must_use]
    pub const fn trial_key(&self) -> TrialKey {
        self.row.trial_key()
    }
}

/// Cloning copies the row, the attachment, and the provenance, for the reason the attachment's realization states.
impl<Invocation, Conclusion> Clone for Binding<Invocation, Conclusion> {
    fn clone(&self) -> Self {
        Self {
            row: self.row.clone(),
            attachment: self.attachment.clone(),
            provenance: self.provenance,
        }
    }
}

impl<Invocation, Conclusion> AuthoredTable<Invocation, Conclusion> {
    /// The complete authored world, over the bindings authored into it.
    ///
    /// # Errors
    ///
    /// Refuses a binding carrying the candidate origin arm, so a candidate joins the authored world only through a human's admission.
    /// Refuses two bindings stating one trial, so a denominator can never read two where one thing is measured.
    pub fn authored(
        name: AuthoredTableName,
        provenance: Provenance,
        bindings: Vec<Binding<Invocation, Conclusion>>,
    ) -> Result<Self, AuthoredTableRefusal> {
        let mut trials = BTreeSet::new();
        for binding in &bindings {
            let key = binding.trial_key();
            if let Origin::Candidate(_) = binding.row().origin() {
                return Err(AuthoredTableRefusal::CandidateOrigin(key));
            }
            if !trials.insert(key) {
                return Err(AuthoredTableRefusal::DuplicateTrial(key));
            }
        }
        Ok(Self {
            name,
            provenance,
            bindings,
        })
    }

    /// The name this world is known by.
    #[must_use]
    pub const fn name(&self) -> AuthoredTableName {
        self.name
    }

    /// Whether a producer stands behind this table, and which schema it emitted against.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Every binding this world holds, in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[Binding<Invocation, Conclusion>] {
        &self.bindings
    }

    /// This world as the one sealed read surface.
    #[must_use]
    pub const fn view(&self) -> TableView<'_, Invocation, Conclusion> {
        TableView::Authored(self)
    }
}

impl<'parent, Invocation, Conclusion> StagedTableView<'parent, Invocation, Conclusion> {
    /// A complete authored world with candidates overlaid on it.
    ///
    /// # Errors
    ///
    /// Refuses an overlaid binding that does not carry the candidate origin arm, so the staging door cannot be an authoring door.
    /// Refuses a candidate stating a trial the parent or another candidate already states, so uniqueness holds across both worlds at once.
    pub fn staged(
        parent: &'parent AuthoredTable<Invocation, Conclusion>,
        candidates: Vec<Binding<Invocation, Conclusion>>,
    ) -> Result<Self, StagedTableRefusal> {
        let mut trials: BTreeSet<TrialKey> =
            parent.bindings().iter().map(Binding::trial_key).collect();
        for candidate in &candidates {
            let key = candidate.trial_key();
            match candidate.row().origin() {
                Origin::Candidate(_) => {}
                Origin::HandWritten
                | Origin::Generated(_)
                | Origin::AdmittedReplay(_)
                | Origin::AdmittedDischarge(_) => {
                    return Err(StagedTableRefusal::NotACandidate(key));
                }
            }
            if !trials.insert(key) {
                return Err(StagedTableRefusal::DuplicateTrial(key));
            }
        }
        Ok(Self { parent, candidates })
    }

    /// The authored world this view is overlaid on.
    #[must_use]
    pub const fn parent(&self) -> &'parent AuthoredTable<Invocation, Conclusion> {
        self.parent
    }

    /// The candidates overlaid, in staged order.
    #[must_use]
    pub fn candidates(&self) -> &[Binding<Invocation, Conclusion>] {
        &self.candidates
    }

    /// This staged world as the one sealed read surface.
    #[must_use]
    pub const fn view(&self) -> TableView<'_, Invocation, Conclusion> {
        TableView::Staged(self)
    }
}

impl<Invocation, Conclusion> TableView<'_, Invocation, Conclusion> {
    /// Every binding this view presents: the authored world in authored order, then the overlay in staged order.
    pub fn bindings(&self) -> impl Iterator<Item = &Binding<Invocation, Conclusion>> {
        let (authored, overlay) = match self {
            Self::Authored(table) => (table.bindings(), &[][..]),
            Self::Staged(staged) => (staged.parent().bindings(), staged.candidates()),
        };
        authored.iter().chain(overlay.iter())
    }

    /// Which world this view presents, and — when it is staged — the authored parent it was overlaid on.
    #[must_use]
    pub fn posture(&self) -> TablePosture {
        match self {
            Self::Authored(_) => TablePosture::Authored,
            Self::Staged(staged) => TablePosture::Staged {
                parent: staged.parent().name(),
            },
        }
    }
}

//! The trial home's invariant nucleus: every road that reaches a private field.
//!
//! A row's rosters are admitted here, so a row classified as something nobody declared does not exist.
//! A group's rows are admitted here, so a seat that measures nothing does not exist.
//! A payload's module namespace is closed here, so a stamped module that would declare one function twice is refused before a token exists.

use super::{
    ROLE_LIMIT, ROW_LIMIT, References, Row, SUITE_GROUP_LIMIT, SuiteGroup, TAG_LIMIT,
    TrialCaptureError, Trials,
};
use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{
    CaptureCause, DeclarationError, FunctionName, Grammar, HelperRefusal, ModuleName, Name, Seat,
    SupportName,
};
use crate::token::SpanHandle;
use std::collections::BTreeSet;

impl Row {
    /// Declare one descriptor row.
    ///
    /// There is no suite parameter and no origin parameter, and neither absence is a dropped fact: a row's execution suite is its group's, and a row's origin is the producer's own act.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Doubled`] where a roster states one label twice — refused rather than folded away, because collapsing a duplicate silently would be this side normalizing an authoring defect the harness itself refuses — and [`DeclarationError::Unbounded`] where a roster outgrows its declared magnitude.
    /// The checks are dependent and in that order, so exactly one cause is true of any refused row.
    pub fn declared(
        lens: FunctionName,
        references: References,
        roles: Vec<Name>,
        tags: Vec<Name>,
    ) -> Result<Self, DeclarationError> {
        if names_doubled(&roles) {
            return Err(DeclarationError::Doubled { seat: Seat::Role });
        }
        let offered_roles = roles.len();
        let admitted_roles: Bounded<Name, ROLE_LIMIT> = Bounded::new(roles)
            .map_err(|_| DeclarationError::unbounded(Seat::Role, ROLE_LIMIT, offered_roles))?;
        if names_doubled(&tags) {
            return Err(DeclarationError::Doubled { seat: Seat::Tag });
        }
        let offered_tags = tags.len();
        let admitted_tags: Bounded<Name, TAG_LIMIT> = Bounded::new(tags)
            .map_err(|_| DeclarationError::unbounded(Seat::Tag, TAG_LIMIT, offered_tags))?;
        Ok(Self {
            lens,
            references,
            roles: admitted_roles,
            tags: admitted_tags,
        })
    }

    /// The lens the stamp declares this row's named test function under.
    #[must_use]
    pub const fn lens(&self) -> &FunctionName {
        &self.lens
    }

    /// The four namespaced references this row states about itself.
    #[must_use]
    pub const fn references(&self) -> &References {
        &self.references
    }

    /// The roles this row carries, in the order they were declared.
    #[must_use]
    pub fn roles(&self) -> &[Name] {
        self.roles.as_slice()
    }

    /// The tags this row carries, in the order they were declared.
    #[must_use]
    pub fn tags(&self) -> &[Name] {
        self.tags.as_slice()
    }
}

impl SuiteGroup {
    /// Declare one aggregate seat's group.
    ///
    /// The suite is stated here and nowhere else on this road: every row grouped under this seat runs under this suite by construction.
    ///
    /// Lens uniqueness is not checked here. The stamped module puts every group's seats and every group's lenses in ONE namespace, so the whole namespace is visible at the payload and nowhere else, and a uniqueness law standing in two homes is one law that agrees with itself until one home is edited.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no row was supplied — a seat over no row is a seat that measures nothing — and [`DeclarationError::Unbounded`] where the rows outgrow [`ROW_LIMIT`].
    pub fn declared(
        seat: FunctionName,
        suite: Name,
        rows: Vec<Row>,
    ) -> Result<Self, DeclarationError> {
        if rows.is_empty() {
            return Err(DeclarationError::Absent { seat: Seat::Row });
        }
        let offered = rows.len();
        let admitted = NonEmpty::new(rows)
            .map_err(|_| DeclarationError::unbounded(Seat::Row, ROW_LIMIT, offered))?;
        Ok(Self {
            seat,
            suite,
            rows: admitted,
        })
    }

    /// The aggregate seat this group declares.
    #[must_use]
    pub const fn seat(&self) -> &FunctionName {
        &self.seat
    }

    /// The execution suite this seat selects on.
    #[must_use]
    pub const fn suite(&self) -> &Name {
        &self.suite
    }

    /// The rows declared under this seat; structurally at least one.
    ///
    /// # Ordering
    ///
    /// This order is meaning: the stamp writes one lens function per row in the order it reads them, so the same rows supplied in another order render a different tree.
    #[must_use]
    pub fn rows(&self) -> &NonEmpty<Row, ROW_LIMIT> {
        &self.rows
    }
}

impl Trials {
    /// Declare the complete payload one stamped trial table is written from.
    ///
    /// There is no producer parameter and no door parameter: which producer emitted a table and which door it was authored through are facts about these services, stated by the emitter the caller declares rather than by an authored declaration.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no group was supplied, [`DeclarationError::Doubled`] where two items of the stamped module's ONE namespace carry a single spelling — seats and lenses share it, so a seat colliding with a lens is caught here as well — and [`DeclarationError::Unbounded`] where the groups outgrow [`SUITE_GROUP_LIMIT`].
    ///
    /// The namespace check runs before the magnitude check, because a collision is a defect in what was declared and a caller repairing a magnitude first would repair the collision second.
    pub fn declared(
        support: SupportName,
        module: ModuleName,
        table: Name,
        groups: Vec<SuiteGroup>,
    ) -> Result<Self, DeclarationError> {
        if groups.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::SuiteGroup,
            });
        }
        stamped_namespace_closed(&groups)?;
        let offered = groups.len();
        let admitted = NonEmpty::new(groups).map_err(|_| {
            DeclarationError::unbounded(Seat::SuiteGroup, SUITE_GROUP_LIMIT, offered)
        })?;
        Ok(Self {
            support,
            module,
            table,
            groups: admitted,
        })
    }

    /// The exported name a consumption target invokes this declaration's carrier by.
    #[must_use]
    pub const fn support(&self) -> &SupportName {
        &self.support
    }

    /// The module the stamp writes this table into.
    #[must_use]
    pub const fn module(&self) -> &ModuleName {
        &self.module
    }

    /// The authored table's own namespaced name.
    #[must_use]
    pub const fn table(&self) -> &Name {
        &self.table
    }

    /// The aggregate seats, in the order they were declared; structurally at least one.
    #[must_use]
    pub fn groups(&self) -> &NonEmpty<SuiteGroup, SUITE_GROUP_LIMIT> {
        &self.groups
    }

    /// How many rows stand under this table, across every aggregate seat.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.groups().iter().fold(0usize, |total, group| {
            total.saturating_add(group.rows().count())
        })
    }
}

impl TrialCaptureError {
    /// One refusal the trial grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// One refusal the vocabulary established over a value this grammar read.
    pub const fn vocabulary_refused(
        grammar: Grammar,
        refusal: DeclarationError,
        at: SpanHandle,
    ) -> Self {
        Self(HelperRefusal::vocabulary_refused(grammar, refusal, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}

/// Whether two of one roster's names carry one spelling.
fn names_doubled(names: &[Name]) -> bool {
    let distinct: BTreeSet<&Name> = names.iter().collect();
    distinct.len() != names.len()
}

/// The stamped module's ONE namespace, closed: every seat spelling and every lens spelling across every group, distinct.
///
/// Seats and lenses are both functions in the module the stamp writes, so they share one namespace and a seat colliding with a lens is the same defect as two lenses colliding.
/// Refused here rather than left to the consumer's compiler, which would report a duplicate definition inside an expansion nobody wrote.
fn stamped_namespace_closed(groups: &[SuiteGroup]) -> Result<(), DeclarationError> {
    let mut taken: BTreeSet<&str> = BTreeSet::new();
    for group in groups {
        if !taken.insert(group.seat().spelling()) {
            return Err(DeclarationError::Doubled {
                seat: Seat::Aggregate,
            });
        }
    }
    for group in groups {
        for row in group.rows() {
            if !taken.insert(row.lens().spelling()) {
                return Err(DeclarationError::Doubled { seat: Seat::Lens });
            }
        }
    }
    Ok(())
}

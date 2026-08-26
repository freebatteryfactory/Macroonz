//! The kind home's invariant nucleus: the only road from a consumer-owned disposition record to a complete set witness.

use super::{Disposition, DispositionRecord, DispositionSet, DispositionSetError, KindSet};
use core::marker::PhantomData;

impl<Set: KindSet> DispositionSet<Set> {
    /// Check one set's disposition record and seal it as complete.
    ///
    /// # Errors
    ///
    /// Returns [`DispositionSetError::CountMismatch`] when the record surrenders fewer or more dispositions than [`KindSet::NAMES`] declares.
    /// Nothing is truncated, padded, or inferred: silence remains unable to enter an account.
    pub fn complete(record: Set::Dispositions) -> Result<Self, DispositionSetError> {
        let rows: Vec<_> = record.into_dispositions().collect();
        let expected = Set::NAMES.len();
        let observed = rows.len();
        if expected != observed {
            return Err(DispositionSetError::CountMismatch { expected, observed });
        }
        Ok(Self {
            dispositions: rows,
            kind_set: PhantomData,
        })
    }

    /// The number of disposition rows, equal to this set's declared-name count by construction.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.dispositions.len()
    }

    /// Whether this complete set has no declared kinds and therefore no disposition rows.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.dispositions.is_empty()
    }

    /// Every declared kind name paired with its disposition, in declaration order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&'static str, &Disposition)> {
        Set::NAMES.iter().copied().zip(self.dispositions.iter())
    }
}

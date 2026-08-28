//! Construction and readings for an authored operator family.

use super::OperatorFamily;

impl OperatorFamily {
    /// Declare one row from inside the home that owns the bank.
    pub(in crate::depot) const fn declared(slug: &'static str, attacks: &'static str) -> Self {
        Self { slug, attacks }
    }

    /// The family's stable slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        self.slug
    }

    /// What an operator in this family damages.
    #[must_use]
    pub const fn attacks(self) -> &'static str {
        self.attacks
    }
}

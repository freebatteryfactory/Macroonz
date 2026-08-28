//! The authored shape of one mutation-operator family.

#[path = "type_guard.rs"]
mod guard;

/// One family of mutation operators, and what an operator of it damages in a subject.
///
/// Family-bearing mutation identities carry the stable slug, while the account of what the family attacks does not enter those preimages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamily {
    /// The family's stable slug, declared rather than read off the Rust spelling beside it.
    slug: &'static str,
    /// What one operator of this family damages.
    attacks: &'static str,
}

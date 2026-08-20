//! The bank's declarations: the shape of one mutation-operator-family row, and
//! the shape of one anti-substitution swap-pair row.
//!
//! Declarations only. The entries themselves are the family files beside this
//! one, and what the bank is for is this home's README.

/// One family of mutation operators the proof-pressure engine draws from.
///
/// A row states which family it is and what one of that family's operators
/// damages in a subject. It is a fact and not a mutation: nothing here selects,
/// applies, or scores anything — planning and application are
/// [`crate::muterprater`]'s, and reading this bank never runs a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamily {
    /// The family's stable slug — its identity to anything naming a family from
    /// outside the file that declares it.
    ///
    /// Declared rather than taken from the Rust spelling beside it, so renaming
    /// the constant moves the spelling and moves nothing named under the slug.
    pub slug: &'static str,
    /// What one operator of this family damages in the subject it is applied
    /// to.
    pub attacks: &'static str,
}

/// One anti-substitution swap pair: the type a position requires, the type that
/// must never be accepted in its place, and the boundary the two stand across.
///
/// A row is material, not a case. The generator reads a row and emits the
/// compile-refusal case that offers the substitute where the seat's type is
/// required; the case's evidence is the compiler's own refusal, and the fixture
/// it lands in is `tests/`'s.
///
/// # Ordering
///
/// The pair is directional and the two seats never trade places: the row says
/// that offering [`substitute`](Self::substitute) where [`seat`](Self::seat) is
/// required does not typecheck, and says nothing at all about the other
/// direction — two role-distinct types may be separated for one reason in one
/// direction and for another reason, or none, in the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapPair {
    /// The type the position under test requires.
    pub seat: &'static str,
    /// The type that must never be accepted where the seat's type is required.
    pub substitute: &'static str,
    /// The separation the two types stand on opposite sides of, named as the
    /// instrument that owns them names it.
    pub boundary: &'static str,
}

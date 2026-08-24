//! Declarations only: the shape of one mutation-operator family row, and the shape of one anti-substitution pair.

#[path = "type_guard.rs"]
mod guard;

/// One family of mutation operators, and what an operator of it damages in a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamily {
    /// The family's stable slug, declared rather than read off the Rust spelling beside it.
    slug: &'static str,
    /// What one operator of this family damages.
    attacks: &'static str,
}

/// One directional pair: the type a position requires, the type that must never be accepted in its place, and the boundary the two stand across.
///
/// The seats never trade places — a row says that offering [`substitute`](Self::substitute) where [`seat`](Self::seat) is required does not typecheck, and says nothing at all about the other direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapPair {
    /// The type the position under test requires.
    pub seat: &'static str,
    /// The type that must never be accepted where the seat's type is required.
    pub substitute: &'static str,
    /// The separation the two stand on opposite sides of, named as the home that owns them names it.
    pub boundary: &'static str,
}

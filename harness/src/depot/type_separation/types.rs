//! The authored shape of one directional type-separation challenge.

/// One directional pair: the type a position requires, the type that must never be accepted in its place, and the boundary the two stand across.
///
/// The seats never trade places — a row says that offering [`substitute`](Self::substitute) where [`seat`](Self::seat) is required does not typecheck, and says nothing at all about the other direction.
/// Holding this value establishes the authored challenge only; the compiler's rejection is the evidence that the separation holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapPair {
    /// The type the position under test requires.
    pub seat: &'static str,
    /// The type that must never be accepted where the seat's type is required.
    pub substitute: &'static str,
    /// The separation the two stand on opposite sides of, named as the home that owns them names it.
    pub boundary: &'static str,
}

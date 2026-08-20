//! The bound classes and the affine budget shape.
//!
//! Bounds are semantic; the meter is a mechanism — runtime metering enforces the
//! admitted contract without backend instruction count ever becoming part of
//! semantic meaning. Every admitted profile declares its bounds; a projection may
//! never change them; a response stays bound to the bounds that admitted it.
//!
//! # Budgets are affine
//!
//! At every crossing, remaining budgets only shrink. A budget is a consumable
//! value, not a readable counter: charging consumes the budget and yields the
//! smaller successor, the type is deliberately neither `Copy` nor `Clone`, and
//! no widening operation exists here at all — value can be lost at a boundary,
//! never manufactured. The only reverse is a named, authority-bearing morphism
//! that consumes evidence and leaves a receipt, owned where grants live.

use crate::refusal::{FamilyShape, RefusalFamily};
use core::marker::PhantomData;

/// The closed class register — seven, closed. The first five are the
/// cross-domain minimum every admitted computation carries; `Output` and
/// `Time` complete the two-level register, whose dimension level is owned by
/// the execution home. `Time` is the durable deadline-policy budget, enforced
/// at the time home.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundClass {
    /// Bounded work — how much portable semantic computation may occur.
    Work,
    /// Bounded memory — how much live bounded state may be retained.
    Memory,
    /// Bounded result size — the semantic NORMAL result.
    Result,
    /// Bounded effect count — how much admitted effect intent may be proposed
    /// or crossed.
    Effect,
    /// Bounded suspensions — the class most easily forgotten.
    Suspension,
    /// Bounded output — non-result material emitted, rendered, generated, or
    /// packaged; artifact count and bytes are Output dimensions.
    Output,
    /// The durable deadline-policy budget — its value IS the deadline
    /// policy's; enforcement lives with the time home, riding this home's
    /// affine budget shape.
    Time,
}

/// The cross-domain minimum: no computation is admitted without enforceable
/// finite bounds in all five.
pub const CROSS_DOMAIN_MINIMUM: [BoundClass; 5] = [
    BoundClass::Work,
    BoundClass::Memory,
    BoundClass::Result,
    BoundClass::Effect,
    BoundClass::Suspension,
];

/// A bound dimension family marker — the typed hole naming which dimension a
/// budget spends in, so budgets from different dimensions never unify. Owner
/// homes declare their dimension markers (the time home's deadline dimension
/// rides this same shape).
pub trait Dimension {}

/// A registered dimension identity (u16-registered; the dimension roster —
/// the register's second level — is the execution home's, because the
/// dimensions derive from what operators charge).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionId(u16);

impl DimensionId {
    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// One portable semantic-work account: a magnitude in one registered
/// dimension — the canonical work record of the seven-record register, seated
/// here by band math (the navigation and execution homes both consume it; one
/// type, one owner). Portable-work evidence is one versioned surface;
/// mechanism diagnostics are a second, independently versioned surface —
/// diagnostics never silently become the portable budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticWork {
    /// The charged dimension.
    pub dimension: DimensionId,
    /// The magnitude.
    pub magnitude: u64,
}

/// The charge refusal: single cause. Payload owed: charged amount, remaining
/// magnitude, and the dimension identity.
#[must_use = "a charge refusal carries the lawful reason the budget was not spent"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetCharge {
    /// The charge exceeds the remaining budget.
    BoundExceeded,
}

impl RefusalFamily for BudgetCharge {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["BoundExceeded"];
}

/// An affine budget in one dimension. Deliberately neither `Copy` nor `Clone`:
/// holding it is holding the remaining capacity, charging consumes it, and no
/// operation here can ever make it larger. Minted only by admission boundaries.
#[derive(Debug)]
pub struct Budget<D: Dimension> {
    remaining: u64,
    _dimension: PhantomData<D>,
}

impl<D: Dimension> Budget<D> {
    /// The remaining magnitude. Reading is not gaining.
    #[must_use]
    pub fn remaining(&self) -> u64 {
        self.remaining
    }

    /// The one lawful operation: consume this budget, yielding the strictly
    /// smaller successor or the typed refusal. The monotone-shrink law is the
    /// signature itself — `self` is taken by value and no widening path exists.
    ///
    /// # Errors
    ///
    /// Returns the [`BudgetCharge`] family body when the charge exceeds the
    /// remaining budget; the budget is consumed either way — a refused charge
    /// does not refund certainty to a caller that lost it.
    pub fn charge(self, amount: u64) -> Result<Self, BudgetCharge> {
        match self.remaining.checked_sub(amount) {
            Some(remaining) => Ok(Self {
                remaining,
                _dimension: PhantomData,
            }),
            None => Err(BudgetCharge::BoundExceeded),
        }
    }
}

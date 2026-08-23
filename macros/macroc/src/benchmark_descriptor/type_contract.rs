//! The benchmark-descriptor home's declarative surface: the tables this home
//! states rather than computes.
//!
//! Two declarations stand here.
//!
//! The BUDGET ORDER: which position of the schema's positional budget roster each
//! named tolerance occupies. The schema declares budgets as a roster of counts and
//! this home declares them as three named seats, so the mapping between the two is
//! a stated table rather than an order a reader infers from a rendering. It is a
//! table for a reader: the rendering writes the counts as literals in exactly this
//! order and elects no name at rendering time.
//!
//! The CROSSING BILL: exactly what the harness owes before anything this home
//! renders resolves at a consumer's site. The bench row vocabulary's SCHEMA is
//! published — its field roster is declared beside the descriptor's and the
//! mutation discovery's, and the one pin covers all three — but the TYPES and
//! constructors that roster describes are not, and neither is a payload road
//! through the gate. Stated as a constant table so the join between what this home
//! writes and what the mailbox publishes is one list rather than a search.

use super::ContentionPosture;

impl ContentionPosture {
    /// The arm spelling this posture is emitted under, exactly as the harness's
    /// schema declares the closed choice.
    ///
    /// A constant answer over a closed roster, so a SECOND posture admitted
    /// later stops the compiler here until somebody says what it is called at
    /// the address — and the schema's own roster has exactly one, so a second is
    /// a change to the schema's declared choice and to the identity derived from
    /// it, not a change a rendering can absorb quietly.
    #[must_use]
    pub const fn arm(self) -> &'static str {
        match self {
            Self::NoDeclaredContention => "NoDeclaredContention",
        }
    }
}

/// The named tolerance each position of the schema's positional budget roster
/// carries, in the order the rendering writes them.
///
/// The schema declares `declared_budgets` as a roster of counts and this home
/// declares three named seats, so the mapping is a stated table rather than an
/// order a reader infers from a rendering: a reader joining this home's
/// [`DeclaredBudgets`](super::DeclaredBudgets) to the schema's positions reads
/// this, and [`budgets`](super::budgets) writes the three counts in exactly this
/// order.
///
/// # Nonclaims
///
/// It is a table for a READER and never a lookup the rendering depends on. The
/// rendered counts are literals now that the token roster spells numbers, so
/// nothing here elects a name at rendering time — which is why moving a row of
/// this table is a change to what the table SAYS and never a silent change to
/// what is emitted. The order the emission writes and the order stated here move
/// together, or this table has stopped describing the emission.
pub const BUDGET_ORDER: [&str; 3] = ["samples", "warmup", "ratio-threshold"];

/// One thing the harness owes before what this home renders resolves at a
/// consumer's site: the seat, and why the emission needs it.
///
/// # Authority
///
/// **This is a bill, not an inventory.** Every row names a seat the rendering
/// actually writes a path to — nothing is listed that the emission does not
/// depend on — and the mailbox side owns how each one is met. The producer writes
/// letters to an address; this table is the address it writes to, stated once so
/// nobody has to read the rendering to find out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrossingOwed {
    /// The seat the emission names.
    pub seat: &'static str,
    /// Why the emission names it.
    pub because: &'static str,
}

/// The complete bill for the wall's third crossing.
pub const CROSSING_OWED: [CrossingOwed; 5] = [
    CrossingOwed {
        seat: "the bench row vocabulary's types and public constructors",
        because: "the bench field roster is published and pinned, and the types that roster \
                  describes are not, so a row expression names constructors the address has not \
                  opened yet",
    },
    CrossingOwed {
        seat: "the bench binding's constructor",
        because: "a bench row is pure data and cannot measure; the binding is what closes the \
                  hidden-callable seam the descriptor's binding closes",
    },
    CrossingOwed {
        seat: "a bench payload road through the gate",
        because: "the gate's opening arm forwards its payload to the trial-table stamp \
                  unconditionally, so the third crossing has no road that reaches its own stamp \
                  while still passing the one pin",
    },
    CrossingOwed {
        seat: "the bench table stamp",
        because: "the rendered payload's grammar is a stamp's, and the harness owns its stamps \
                  beside the vocabulary they read",
    },
    CrossingOwed {
        seat: "the bench host that runs the declared order",
        because: "the preflight passes, then the planted-worse gate proves the measurement \
                  separates the declared class, and only then is the backend invoked — the \
                  rendered adapter binds and reports, because an adapter that ran the order \
                  would be a second host and a backend that skipped it would benchmark a \
                  failing operation",
    },
];

//! The descriptor home's declarative trait participation: the widening from a replay-bearing admission ground to the summary vocabulary, and the refusal conversions used by the stamped road.
//!
//! [`ReplayBearingGround`] is the narrowed vocabulary accepted by a replay admission.
//! Its `From` implementation is the sole widening into [`AdmissionGround`], so the narrowed type costs no parallel spelling at the summary boundary.
//!
//! # The discharge table
//!
//! A declared row expression builds its own parts through this home's public
//! constructors and writes `?` on each one, which is the language's own
//! conversion rather than a variant invented inside a vocabulary the writer does
//! not own. What that `?` stands on is the roster at the foot of this file: one
//! arm of [`TrialTableRefusal`] per family a construction on that road answers
//! with. The roster reads back as the bill — a family listed there has a lawful
//! discharge, and a family absent from it has none.

use super::types::{
    AdmissionGround, BindingRefusal, ClassificationRefusal, EncodeRefusal, NameRefusal,
    ReplayBearingGround, RowRefusal, SchemaRefusal, TrialTableRefusal,
};

impl From<ReplayBearingGround> for AdmissionGround {
    /// The summary vocabulary one narrowed ground widens to.
    ///
    /// The narrowing exists so an arm cannot be handed a ground it does not
    /// earn; the widening exists so the narrowing costs no second slot table, no
    /// second spelling, and no second summary. Every replay-bearing ground has
    /// exactly one image here, and it is the same word an admission act states.
    fn from(ground: ReplayBearingGround) -> Self {
        match ground {
            ReplayBearingGround::MutantKilled => Self::MutantKilled,
            ReplayBearingGround::ClaimPinned => Self::ClaimPinned,
        }
    }
}

/// The one lawful discharge each construction's refusal has into the stamped
/// road's family, written once and stamped over the roster.
///
/// Every realization is the same three lines over a different pair, and a
/// hand-copied one per family would be that single law standing in six places —
/// which is exactly the shape that lets one of them drift into naming a
/// different arm than the rest. The roster below is therefore the statement, and
/// the realizations are its transcription.
macro_rules! discharged_into_trial_table {
    ($($family:ident => $arm:ident),+ $(,)?) => {
        $(
            impl From<$family> for TrialTableRefusal {
                fn from(refusal: $family) -> Self {
                    Self::$arm(refusal)
                }
            }
        )+
    };
}

// The bill, in the order a row expression performs the constructions: the names
// it spells, the rosters it takes as authored, the row it declares, the root
// schema declaration a produced row pins against and the identity derived from
// it, and the binding that marries the row to what executes it.
//
// The encoding family appears once, for the schema identity. A row's own
// encoding refusal travels inside the row's family instead, because a row whose
// bytes could not be written is a row that was never built, and the arm a reader
// wants is the one naming which construction refused.
//
// The authored-table refusal is deliberately absent: the only construction that
// raises it stands in the stamp's tail position, where the arm is named where it
// stands, so a conversion here would be that mapping written twice.
discharged_into_trial_table!(
    NameRefusal => NameNotParsed,
    ClassificationRefusal => ClassificationNotAuthored,
    RowRefusal => RowNotDeclared,
    SchemaRefusal => SchemaNotDeclared,
    EncodeRefusal => SchemaNotEncoded,
    BindingRefusal => BindingNotBound,
);

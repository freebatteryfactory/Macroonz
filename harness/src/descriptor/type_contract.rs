//! The home's declarative trait participation: one widening, and the refusal discharges the stamped road's `?` stands on.
//!
//! The narrowing on a replay admission exists so an arm cannot be handed a ground it does not earn; the widening below is what makes that narrowing cost no second slot table and no second summary.
//!
//! A declared row expression builds its parts through this home's public constructors and writes `?` on each one, which is the language's own conversion rather than a variant invented inside a vocabulary the writer does not own.
//! The roster at the foot of this file is what that `?` stands on, and it reads back as the bill: a family listed there has a lawful discharge, and a family absent from it has none.

use super::types::{
    AdmissionGround, BindingRefusal, ClassificationRefusal, EncodeRefusal, NameRefusal,
    ReplayBearingGround, RowRefusal, SchemaRefusal, TrialTableRefusal,
};

impl From<ReplayBearingGround> for AdmissionGround {
    /// The summary vocabulary one narrowed ground widens to.
    ///
    /// Every replay-bearing ground has exactly one image here, and it is the same word an admission act states.
    fn from(ground: ReplayBearingGround) -> Self {
        match ground {
            ReplayBearingGround::MutantKilled => Self::MutantKilled,
            ReplayBearingGround::ClaimPinned => Self::ClaimPinned,
        }
    }
}

/// The one lawful discharge each construction's refusal has into the stamped road's family, written once and stamped over the roster below.
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

// The bill, in the order a row expression performs the constructions: the names it spells, the rosters it takes as
// authored, the row it declares, the root schema declaration a produced row pins against and the identity derived
// from it, and the binding that marries the row to what executes it.
//
// The encoding family appears once, for the schema identity. A row's own encoding refusal travels inside the row's
// family instead, because a row whose bytes could not be written is a row that was never built.
//
// The authored-table refusal is deliberately absent: the only construction that raises it stands in the stamp's tail
// position, where the arm is named where it stands, so a conversion here would be that mapping written twice.
discharged_into_trial_table!(
    NameRefusal => NameNotParsed,
    ClassificationRefusal => ClassificationNotAuthored,
    RowRefusal => RowNotDeclared,
    SchemaRefusal => SchemaNotDeclared,
    EncodeRefusal => SchemaNotEncoded,
    BindingRefusal => BindingNotBound,
);

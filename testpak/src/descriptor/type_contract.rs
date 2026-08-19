//! The descriptor home's declarative surface: the closed tables its arms are
//! read through, and the one discharge table every refusal on the stamped road
//! travels.
//!
//! Three tables, each a declaration rather than a computation — a constant
//! answer per arm, stated here so it is read in one place instead of inferred
//! from whichever road happened to need it. The discharge table is a declaration
//! in the same sense: which arm of the stamped road's family a given
//! construction's refusal lands in, said once.
//!
//! # The capsule table
//!
//! [`AdmissionGround::capsule_posture`] is why the admitted origin has two arms
//! rather than one arm with an optional replay seat. A ground either brings a
//! depot capsule entry with it or authors none at all, and the row's shape
//! follows the ground rather than a caller's care.
//!
//! # The slot tables
//!
//! [`FieldShape::slot`] and [`FieldCardinality::slot`] are identity-bearing:
//! the encoder writes these bytes into the generated-support schema's preimage,
//! so a changed slot renames the schema. They are declared rather than derived
//! from declaration order, because a variant reordered for readability must not
//! silently move an identity. No slot is zero, so a zeroed buffer never reads
//! back as a lawful arm.
//!
//! [`Origin::slot`], [`SynthesisFacts::slot`], and [`AdmissionGround::slot`]
//! are identity-bearing in the same way and for a second consumer: the row
//! preimage ([`crate::descriptor::encode_row`]) writes them, so a changed slot
//! renames every row revision derived under it. The origin's slots are the same
//! arms, in the same order, that the descriptor schema's `origin` field
//! declares as its closed choice — one reading order, written twice because the
//! two are different kinds of statement, and held together by the conformance
//! trial.
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
    AdmissionGround, BindingRefusal, CapsulePosture, ClassificationRefusal, EncodeRefusal,
    FieldCardinality, FieldShape, NameRefusal, Origin, RowRefusal, SchemaRefusal, SynthesisFacts,
    TrialTableRefusal,
};

impl AdmissionGround {
    /// Whether admitting on this ground authors a depot capsule entry.
    ///
    /// The two replay-bearing grounds carry a reproduction account: a mutant
    /// kill and a claim pin both stand on a run that happened, and the capsule
    /// is what a later reader replays it from. A discharge stands on the
    /// admitted row itself — rerunning the row regenerates the behavioral
    /// evidence — so there is nothing for a capsule to hold.
    #[must_use]
    pub const fn capsule_posture(self) -> CapsulePosture {
        match self {
            Self::MutantKilled | Self::ClaimPinned => CapsulePosture::ReplayBearing,
            Self::ObligationDischarged => CapsulePosture::NoCapsule,
        }
    }

    /// The byte this ground is written as in a row's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::MutantKilled => 1,
            Self::ClaimPinned => 2,
            Self::ObligationDischarged => 3,
        }
    }
}

impl Origin {
    /// The byte this arm is written as in a row's canonical preimage.
    ///
    /// The order is the descriptor schema's declared `origin` choice order:
    /// hand-written, generated, candidate, admitted-replay, admitted-discharge.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::HandWritten => 1,
            Self::Generated(_) => 2,
            Self::Candidate(_) => 3,
            Self::AdmittedReplay { .. } => 4,
            Self::AdmittedDischarge { .. } => 5,
        }
    }
}

impl SynthesisFacts {
    /// The byte this arm is written as in a row's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Survivor(_) => 1,
            Self::ProofGap => 2,
        }
    }
}

impl FieldShape {
    /// The byte this shape is written as in the schema's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::NamespacedName => 1,
            Self::ContentAddress => 2,
            Self::ClosedChoice(_) => 3,
            Self::Bytes => 4,
            Self::Count => 5,
        }
    }
}

impl FieldCardinality {
    /// The byte this cardinality is written as in the schema's canonical
    /// preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::ExactlyOne => 1,
            Self::ZeroOrOne => 2,
            Self::ZeroOrMore => 3,
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

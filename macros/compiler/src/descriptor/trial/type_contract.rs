//! The trial home's stated tables: what the kind is, where its one unit lands, the question it owes, and how its grammar refuses.

use super::{TrialAnswer, TrialCaptureError, TrialQuestion, TrialRole, TrialTable, Trials};
use crate::bounded::Bounded;
use crate::diagnostic::{
    FIRST_HELPER_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    Repair,
};
use crate::identity::encode_bytes;
use crate::kind::{Answer, Destination, Kind, Question, Role};

impl Kind for TrialTable {
    const NAME: &'static str = "trial-table";

    type Content = Trials;
    type Role = TrialRole;
    type Question = TrialQuestion;
}

impl Role for TrialRole {
    const ALL: &'static [Self] = &[Self::Table];

    fn name(self) -> &'static str {
        match self {
            Self::Table => "table",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Table => Destination::TestCarrier,
        }
    }
}

impl Question for TrialQuestion {
    const ALL: &'static [Self] = &[Self::WhichTestsChallenge];

    type Answer = TrialAnswer;

    fn name(self) -> &'static str {
        match self {
            Self::WhichTestsChallenge => "which-tests-challenge",
        }
    }
}

impl Answer for TrialAnswer {
    type Question = TrialQuestion;

    fn question(&self) -> TrialQuestion {
        match *self {
            Self::ChallengingTests { .. } => TrialQuestion::WhichTestsChallenge,
        }
    }

    fn encode_into(&self, into: &mut Vec<u8>) {
        match *self {
            Self::ChallengingTests { ref table, rows } => {
                encode_bytes(table.namespace().as_bytes(), into);
                encode_bytes(table.stem().as_bytes(), into);
                into.extend_from_slice(&rows.to_be_bytes());
            }
        }
    }

    fn human(&self) -> String {
        match *self {
            Self::ChallengingTests { ref table, rows } => {
                let namespace = table.namespace();
                let stem = table.stem();
                format!("the table `{namespace}`/`{stem}` declares {rows} rows that challenge it")
            }
        }
    }
}

impl Refused for TrialCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = FIRST_HELPER_FAMILY;

    fn class(&self) -> RefusalClass {
        self.refusal().class()
    }

    fn first(&self) -> String {
        self.refusal().first()
    }

    fn observed(&self) -> Observed {
        self.refusal().classified()
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        vec![self.refusal().canonical_bytes()]
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        self.refusal().repairs()
    }
}

impl core::fmt::Display for TrialCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for TrialCaptureError {}

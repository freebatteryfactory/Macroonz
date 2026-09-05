//! The trial home's stated tables: what the kind is, where its one unit lands, the question it owes, and how its grammar refuses.

use super::{
    Row, SuiteGroup, TrialAnswer, TrialCaptureError, TrialQuestion, TrialRole, TrialTable, Trials,
};
use crate::descriptor::Name;
use crate::diagnostic::FIRST_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{Answer, CanonicalContent, Destination, Kind, Question, Role};

impl CanonicalContent for Trials {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.support().spelling().as_bytes(), into);
        encode_bytes(self.module().spelling().as_bytes(), into);
        encode_name(self.table(), into);
        encode_length(self.groups().count(), into);
        for group in self.groups() {
            encode_group(group, into);
        }
    }
}

fn encode_group(group: &SuiteGroup, into: &mut Vec<u8>) {
    let mut encoded = Vec::new();
    encode_bytes(group.seat().spelling().as_bytes(), &mut encoded);
    encode_name(group.suite(), &mut encoded);
    encode_length(group.rows().count(), &mut encoded);
    for row in group.rows() {
        let mut member = Vec::new();
        encode_row(row, &mut member);
        encode_bytes(&member, &mut encoded);
    }
    encode_bytes(&encoded, into);
}

fn encode_row(row: &Row, into: &mut Vec<u8>) {
    encode_bytes(row.lens().spelling().as_bytes(), into);
    let references = row.references();
    encode_name(&references.claim, into);
    encode_name(&references.subject, into);
    encode_name(&references.check, into);
    encode_name(&references.population, into);
    encode_length(row.roles().len(), into);
    for role in row.roles() {
        encode_name(role, into);
    }
    encode_length(row.tags().len(), into);
    for tag in row.tags() {
        encode_name(tag, into);
    }
}

fn encode_name(name: &Name, into: &mut Vec<u8>) {
    encode_bytes(name.namespace().as_bytes(), into);
    encode_bytes(name.stem().as_bytes(), into);
}

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
            Self::Table => Destination::DeclarationSite,
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

crate::descriptor::impl_helper_capture_contract!(TrialCaptureError, FIRST_HELPER_FAMILY, canonical);

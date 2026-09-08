//! Independently framed projection envelopes with separately movable nested claims.

use super::binding_vector::name;
use super::mutation_vector::MutationVector;
use super::parity_vector::{ParityVector, TrialVector, finding};
use super::proposal_vector::TargetVector;
use super::vector::{frame, hash};

pub(super) struct ProjectionVector {
    pub(super) header: [u32; 3],
    pub(super) parity: ParityVector,
    pub(super) baseline: Vec<u8>,
    pub(super) selected: Vec<u8>,
    pub(super) baseline_id: Option<Vec<u8>>,
    pub(super) selected_id: Option<Vec<u8>>,
    pub(super) selection: Vec<u8>,
    pub(super) baseline_report: TrialVector,
    pub(super) selected_report: TrialVector,
    pub(super) mutation: MutationVector,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-compiled-projection-pressure/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

impl ProjectionVector {
    pub(super) fn declared() -> Self {
        let parity = ParityVector::declared();
        let baseline_report = ParityVector::declared().production_report;
        let mut selected_report = ParityVector::declared().production_report;
        selected_report.attempt = finding(&parity.witness.trial());
        let mut target = TargetVector::selected(2);
        let [owner, _suite, _subject, _check, _population] = parity.witness.row.names;
        target.owner = vec![1];
        name(owner, &mut target.owner);
        let mut mutation = MutationVector::demonstrated();
        mutation.target = target.body();
        mutation.activation = vec![2];
        mutation.equivalence = 0;
        mutation.outcome = vec![0, 0];
        mutation
            .outcome
            .extend(selected_report.attempt.iter().copied().skip(1));
        let mut selection = Vec::new();
        frame(&[7; 32], &mut selection);
        name((b"mutation-owner", b"point"), &mut selection);
        frame(&[11; 32], &mut selection);
        Self {
            header: [1, 1, 0],
            parity,
            baseline: Vec::new(),
            selected: vec![0, 0xff, 1],
            baseline_id: None,
            selected_id: None,
            selection,
            baseline_report,
            selected_report,
            mutation,
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in self.header {
            body.extend_from_slice(&word.to_be_bytes());
        }
        frame(&self.parity.encoded(), &mut body);
        for (source, stated) in [
            (&self.baseline, &self.baseline_id),
            (&self.selected, &self.selected_id),
        ] {
            let identity = hash("compiled-artifact-content/v1", source);
            frame(stated.as_deref().unwrap_or(&identity), &mut body);
            frame(source, &mut body);
        }
        body.extend_from_slice(&self.selection);
        frame(&self.baseline_report.encoded(), &mut body);
        frame(&self.selected_report.encoded(), &mut body);
        frame(&self.mutation.encoded(), &mut body);
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

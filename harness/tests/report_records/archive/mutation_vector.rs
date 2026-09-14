//! Independently framed complete mutation claims, without the product writer.

use super::proposal_vector::{TargetVector, activation};
use super::trial_vector;
use super::vector::{frame, hash};

pub(super) struct MutationVector {
    pub(super) header: [u32; 3],
    pub(super) target: Vec<u8>,
    pub(super) baseline: u8,
    pub(super) materialization: u8,
    pub(super) activation: Vec<u8>,
    pub(super) execution: u8,
    pub(super) outcome: Vec<u8>,
    pub(super) equivalence: u8,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-mutation-report/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

impl MutationVector {
    pub(super) fn demonstrated() -> Self {
        let foreign = trial_vector::foreign(&[b'a', 255, b'b'], &[0, 1]);
        let finding = trial_vector::refusal(&foreign);
        let mut outcome = vec![0, 0];
        outcome.extend(finding.into_iter().skip(1));
        Self {
            header: [1, 1, 0],
            target: TargetVector::selected(1).body(),
            baseline: 0,
            materialization: 0,
            activation: activation(7),
            execution: 0,
            outcome,
            equivalence: 2,
        }
    }

    pub(super) fn backend() -> Self {
        let mut vector = Self::demonstrated();
        vector.target = TargetVector::external().body();
        vector.activation = vec![2];
        vector.outcome = vec![0, 1];
        vector
            .outcome
            .extend(trial_vector::foreign(b"caught", &[0, 0]));
        vector
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for word in self.header {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        frame(&self.target, &mut bytes);
        bytes.push(self.baseline);
        bytes.push(self.materialization);
        frame(&self.activation, &mut bytes);
        bytes.push(self.execution);
        bytes.extend_from_slice(&self.outcome);
        bytes.push(self.equivalence);
        bytes
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

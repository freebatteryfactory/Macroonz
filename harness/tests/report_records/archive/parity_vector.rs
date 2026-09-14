//! Hand-authored parity and trial framing with independently movable coordinates.

use super::binding_vector::{BindingVector, name, revision};
use super::vector::{InputKind, Vector, frame, hash};

pub(super) struct TrialVector {
    pub(super) key: Vec<u8>,
    pub(super) metadata: Vec<u8>,
    pub(super) posture: u8,
    pub(super) site: Vec<u8>,
    pub(super) attempt: Vec<u8>,
    pub(super) measurement: Vec<u8>,
}

impl TrialVector {
    fn declared(trial: &[u8; 32]) -> Self {
        let source = Vector::declared(InputKind::Unit);
        let mut key = Vec::new();
        frame(trial, &mut key);
        key.extend(source.key.into_iter().skip(40));
        let mut site = Vec::new();
        frame(b"outside", &mut site);
        frame(b"subject.rs", &mut site);
        site.extend_from_slice(&11u32.to_be_bytes());
        frame(b"trial", &mut site);
        Self {
            key,
            metadata: vec![0],
            posture: 1,
            site,
            attempt: vec![0],
            measurement: vec![1],
        }
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in [1u32, 2, 0] {
            body.extend_from_slice(&word.to_be_bytes());
        }
        frame(&self.key, &mut body);
        body.extend_from_slice(&self.metadata);
        body.push(self.posture);
        body.extend_from_slice(&self.site);
        body.extend_from_slice(&self.attempt);
        body.extend_from_slice(&self.measurement);
        super::trial_vector::envelope(&body)
    }
}

pub(super) struct ParityVector {
    pub(super) header: [u32; 3],
    pub(super) pair: Vec<u8>,
    pub(super) witness: BindingVector,
    pub(super) input_convention: Vec<u8>,
    pub(super) meaning_convention: Vec<u8>,
    pub(super) input: Vec<u8>,
    pub(super) production: Vec<u8>,
    pub(super) evaluation: Vec<u8>,
    pub(super) firings: u32,
    pub(super) substrate: Vec<u8>,
    pub(super) conclusion: Vec<u8>,
    pub(super) production_report: TrialVector,
    pub(super) evaluation_report: TrialVector,
    pub(super) disposition: Vec<u8>,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-no-mutation-parity/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn finding(trial: &[u8; 32]) -> Vec<u8> {
    let mut fingerprint = Vec::new();
    frame(trial, &mut fingerprint);
    frame(b"outside", &mut fingerprint);
    frame(b"disagreement", &mut fingerprint);
    fingerprint.push(1);
    let mut bytes = vec![1];
    frame(&fingerprint, &mut bytes);
    frame(b"check.rs", &mut bytes);
    bytes.extend_from_slice(&29u32.to_be_bytes());
    bytes.push(1);
    frame(&[b'x', 0xff], &mut bytes);
    bytes.extend_from_slice(&[0, 1]);
    bytes
}

fn convention() -> Vec<u8> {
    let mut bytes = Vec::new();
    name((b"encoding", b"bytes"), &mut bytes);
    bytes.extend_from_slice(&3u32.to_be_bytes());
    frame(&[8; 32], &mut bytes);
    frame(&revision(&[9; 32], 2), &mut bytes);
    bytes
}

impl ParityVector {
    pub(super) fn declared() -> Self {
        let witness = BindingVector::declared();
        let trial = witness.trial();
        let mut pair = Vec::new();
        name((b"evaluation", b"family"), &mut pair);
        frame(&revision(&[5; 32], 1), &mut pair);
        frame(&revision(&[6; 32], 2), &mut pair);
        frame(&[7; 32], &mut pair);
        let mut substrate = vec![1];
        substrate.extend_from_slice(&2u64.to_be_bytes());
        name((b"a", b"base"), &mut substrate);
        name((b"b", b"base"), &mut substrate);
        Self {
            header: [1, 1, 0],
            pair,
            witness,
            input_convention: convention(),
            meaning_convention: convention(),
            input: vec![10],
            production: vec![11],
            evaluation: vec![12],
            firings: 0,
            substrate,
            conclusion: vec![0],
            production_report: TrialVector::declared(&trial),
            evaluation_report: TrialVector::declared(&trial),
            disposition: vec![1],
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in self.header {
            body.extend_from_slice(&word.to_be_bytes());
        }
        body.extend_from_slice(&self.pair);
        frame(&self.witness.encoded(), &mut body);
        body.extend_from_slice(&self.input_convention);
        frame(&self.input, &mut body);
        body.extend_from_slice(&self.meaning_convention);
        frame(&self.production, &mut body);
        frame(&self.evaluation, &mut body);
        body.extend_from_slice(&self.firings.to_be_bytes());
        body.extend_from_slice(&self.substrate);
        body.extend_from_slice(&self.conclusion);
        frame(&self.production_report.encoded(), &mut body);
        frame(&self.evaluation_report.encoded(), &mut body);
        body.extend_from_slice(&self.disposition);
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

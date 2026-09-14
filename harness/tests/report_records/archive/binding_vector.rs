//! Hand-framed attachment and identity controls independent of product encoders.

use super::candidate_vector::{CandidateVector, Name, Opening};
use super::vector::{frame, hash};

pub(super) struct BindingVector {
    pub(super) version: u32,
    pub(super) row: CandidateVector,
    pub(super) subject: Name,
    pub(super) check: Name,
    pub(super) subject_revision: Vec<u8>,
    pub(super) check_revision: Vec<u8>,
    pub(super) provenance: Vec<u8>,
}

pub(super) fn name((namespace, stem): Name, bytes: &mut Vec<u8>) {
    frame(namespace, bytes);
    frame(stem, bytes);
}

pub(super) fn revision(address: &[u8], posture: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    frame(address, &mut bytes);
    bytes.push(posture);
    bytes
}

impl BindingVector {
    pub(super) fn declared() -> Self {
        Self {
            version: 1,
            row: CandidateVector::declared(Opening::Gap),
            subject: (b"subject-owner", b"route"),
            check: (b"check-owner", b"law"),
            subject_revision: revision(&[2; 32], 0),
            check_revision: revision(&[3; 32], 1),
            provenance: vec![0],
        }
    }

    pub(super) fn trial(&self) -> [u8; 32] {
        let [claim, _suite, subject, check, population] = self.row.names;
        let mut key = Vec::new();
        for coordinate in [claim, subject, check, population] {
            name(coordinate, &mut key);
        }
        let mut trial = Vec::new();
        frame(&hash("trial-key/v1", &key), &mut trial);
        trial.push(0);
        hash("trial-identity/v1", &trial)
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut bytes = self.version.to_be_bytes().to_vec();
        frame(&self.row.encoded(), &mut bytes);
        name(self.subject, &mut bytes);
        name(self.check, &mut bytes);
        frame(&self.subject_revision, &mut bytes);
        frame(&self.check_revision, &mut bytes);
        bytes.extend_from_slice(&self.provenance);
        bytes
    }
}

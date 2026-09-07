//! Independent proposal bytes composed from independent descriptor and report vectors.

use super::candidate_vector::{CandidateVector, Name, Opening};
use super::run_vector::{self, RunVector};
use super::trial_vector;
use super::vector::{InputKind, Vector, frame, hash};

pub(super) struct ProposalVector {
    pub(super) version: u32,
    pub(super) kind: u32,
    pub(super) custody: u32,
    pub(super) candidate: Vec<u8>,
    pub(super) destination: Name,
    pub(super) ground: u8,
    pub(super) stated_identity: Option<[u8; 32]>,
    pub(super) members: Vec<u8>,
}

pub(super) struct TargetVector {
    pub(super) identity: Vec<u8>,
    pub(super) family: Vec<u8>,
    pub(super) site: Vec<u8>,
    pub(super) owner: Vec<u8>,
}

pub(super) struct KillVector {
    pub(super) target: TargetVector,
    pub(super) activation: Vec<u8>,
    pub(super) capsule: Vector,
    pub(super) run: RunVector,
    pub(super) trial: [u8; 32],
    pub(super) known: Vec<Vec<u8>>,
    pub(super) count: u64,
}

pub(super) fn name((namespace, stem): Name, into: &mut Vec<u8>) {
    frame(namespace, into);
    frame(stem, into);
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-proposal/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

impl ProposalVector {
    pub(super) fn declared(ground: u8, members: Vec<u8>) -> Self {
        Self {
            version: 1,
            kind: 1,
            custody: 0,
            candidate: CandidateVector::declared(Opening::Survivor).encoded(),
            destination: ("révision".as_bytes(), b"destination"),
            ground,
            stated_identity: None,
            members,
        }
    }

    pub(super) fn identity(&self) -> [u8; 32] {
        let mut bytes = 1u32.to_be_bytes().to_vec();
        frame(&self.candidate, &mut bytes);
        bytes.push(self.ground);
        name(self.destination, &mut bytes);
        hash("proposal/v1", &bytes)
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in [self.version, self.kind, self.custody] {
            body.extend_from_slice(&word.to_be_bytes());
        }
        frame(&self.candidate, &mut body);
        body.push(self.ground);
        name(self.destination, &mut body);
        frame(
            &self.stated_identity.unwrap_or_else(|| self.identity()),
            &mut body,
        );
        body.extend_from_slice(&self.members);
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

impl TargetVector {
    pub(super) fn selected(slot: u8) -> Self {
        let mut identity = vec![slot];
        name((b"mutation-owner", b"point"), &mut identity);
        frame(&[11; 32], &mut identity);
        let mut family = vec![1];
        frame(b"historical-family-outside-current-bank", &mut family);
        let mut site = vec![1];
        name((b"activation-owner", b"site"), &mut site);
        let mut owner = vec![1];
        name((b"mapped-owner", b"claim"), &mut owner);
        Self {
            identity,
            family,
            site,
            owner,
        }
    }

    pub(super) fn external() -> Self {
        let mut identity = vec![0];
        frame(&[17; 32], &mut identity);
        let mut site = vec![0];
        frame("src/étranger.rs".as_bytes(), &mut site);
        site.extend_from_slice(&43u32.to_be_bytes());
        site.extend_from_slice(&7u32.to_be_bytes());
        Self {
            identity,
            family: vec![0],
            site,
            owner: vec![0],
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        [&self.identity[..], &self.family, &self.site, &self.owner].concat()
    }
}

pub(super) fn activation(firings: u32) -> Vec<u8> {
    let mut bytes = vec![0];
    frame(&[21; 32], &mut bytes);
    name((b"mutation-owner", b"point"), &mut bytes);
    frame(&[11; 32], &mut bytes);
    frame(&[12; 32], &mut bytes);
    bytes.extend_from_slice(&firings.to_be_bytes());
    bytes
}

impl KillVector {
    pub(super) fn declared() -> Self {
        let capsule = Vector::declared(InputKind::Bound);
        let mut run = RunVector::mixed();
        let foreign = trial_vector::foreign(&[b'a', 255, b'b'], &[0, 1]);
        let trial =
            trial_vector::envelope(&trial_vector::body(&trial_vector::refusal(&foreign), &[1]));
        run.rows = vec![
            run_vector::row(9, 10, 1, &[]),
            run_vector::row(1, 8, 0, &trial),
            run_vector::row(11, 12, 2, &[]),
        ];
        let mut known = Vec::new();
        frame(&[77; 32], &mut known);
        frame(b"previous", &mut known);
        frame(b"refusal", &mut known);
        known.push(4);
        Self {
            target: TargetVector::selected(1),
            activation: activation(5),
            capsule,
            run,
            trial: [1; 32],
            known: vec![known.clone(), known],
            count: 2,
        }
    }

    pub(super) fn members(&self) -> Vec<u8> {
        let mut body = Vec::new();
        frame(&self.target.body(), &mut body);
        frame(&self.activation, &mut body);
        frame(&self.capsule.encoded(), &mut body);
        frame(&self.run.encoded(), &mut body);
        frame(&self.trial, &mut body);
        body.extend_from_slice(&self.count.to_be_bytes());
        for fingerprint in &self.known {
            frame(fingerprint, &mut body);
        }
        body
    }

    pub(super) fn proposal(&self) -> ProposalVector {
        ProposalVector::declared(1, self.members())
    }
}

pub(super) fn pin(before: u64, after: u64) -> ProposalVector {
    let mut members = Vec::new();
    name((b"another-owner", b"pinned-claim"), &mut members);
    frame(&Vector::declared(InputKind::Bound).encoded(), &mut members);
    members.extend_from_slice(&before.to_be_bytes());
    members.extend_from_slice(&after.to_be_bytes());
    ProposalVector::declared(2, members)
}

pub(super) fn discharge(opening: &[u8], lane: u8, input: InputKind) -> ProposalVector {
    let source = Vector::declared(input);
    let mut members = Vec::new();
    name((b"owed-owner", b"obligation"), &mut members);
    frame(opening, &mut members);
    members.push(lane);
    frame(&[88; 32], &mut members);
    frame(&source.key, &mut members);
    members.extend_from_slice(&source.metadata);
    ProposalVector::declared(3, members)
}

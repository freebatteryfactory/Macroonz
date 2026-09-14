//! Independently authored historical reduction bytes and deliberately contradictory accounts.

use super::vector::{InputKind, Vector, frame, hash};

#[derive(Clone)]
pub(super) struct Semantic {
    pub(super) namespace: Vec<u8>,
    pub(super) stem: Vec<u8>,
    pub(super) revision: Vec<u8>,
    pub(super) posture: u8,
    pub(super) candidates: u64,
    pub(super) probes: u64,
}

impl Semantic {
    pub(super) fn offered(stem: &[u8], candidates: u64, probes: u64) -> Self {
        Self {
            namespace: b"outside".to_vec(),
            stem: stem.to_vec(),
            revision: vec![8; 32],
            posture: 0,
            candidates,
            probes,
        }
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame(&self.namespace, &mut bytes);
        frame(&self.stem, &mut bytes);
        frame(&self.revision, &mut bytes);
        bytes.push(self.posture);
        bytes.extend_from_slice(&self.candidates.to_be_bytes());
        bytes.extend_from_slice(&self.probes.to_be_bytes());
        bytes
    }
}

pub(super) struct ReductionVector {
    pub(super) capsule: Vector,
    pub(super) report_posture: u8,
    pub(super) probe: Vec<u8>,
    pub(super) probe_posture: u8,
    pub(super) budget: u32,
    pub(super) semantic: Vec<Semantic>,
    pub(super) byte: u8,
    pub(super) census: [u32; 3],
    pub(super) halt: u8,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut bytes = hash("historical-reduction/v1", body).to_vec();
    bytes.extend_from_slice(body);
    bytes
}

impl ReductionVector {
    pub(super) fn declared(kind: InputKind) -> Self {
        Self {
            capsule: Vector::declared(kind),
            report_posture: 0,
            probe: vec![9; 32],
            probe_posture: 1,
            budget: 20,
            semantic: vec![
                Semantic::offered(b"first", 3, 3),
                Semantic::offered(b"second", 2, 2),
            ],
            byte: 0,
            census: [3, 4, 5],
            halt: 0,
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for value in [1u32, 4, 0] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        frame(&self.capsule.encoded(), &mut bytes);
        bytes.push(self.report_posture);
        frame(&self.probe, &mut bytes);
        bytes.push(self.probe_posture);
        bytes.extend_from_slice(&self.budget.to_be_bytes());
        bytes.extend_from_slice(
            &u64::try_from(self.semantic.len())
                .unwrap_or(u64::MAX)
                .to_be_bytes(),
        );
        for reducer in &self.semantic {
            frame(&reducer.encoded(), &mut bytes);
        }
        bytes.push(self.byte);
        for count in self.census {
            bytes.extend_from_slice(&count.to_be_bytes());
        }
        bytes.push(self.halt);
        bytes
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

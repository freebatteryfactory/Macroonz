//! Independently framed surface material and historical envelopes.

use super::binding_vector::name;
use super::candidate_vector::Name;
use super::vector::{frame, hash};

#[derive(Clone)]
pub(super) struct AlternativeVector {
    pub(super) family: Vec<u8>,
    pub(super) operation: Vec<u8>,
    pub(super) identity: Option<Vec<u8>>,
}

impl AlternativeVector {
    pub(super) fn address(&self, point: Name) -> [u8; 32] {
        let mut bytes = Vec::new();
        name(point, &mut bytes);
        frame(&self.family, &mut bytes);
        frame(&self.operation, &mut bytes);
        hash("mutation-alternative/v1", &bytes)
    }
}

#[derive(Clone)]
pub(super) struct PointVector {
    pub(super) name: Name,
    pub(super) owner: Name,
    pub(super) original: Vec<u8>,
    pub(super) site: Name,
    pub(super) alternatives: Vec<AlternativeVector>,
    pub(super) count: Option<u64>,
}

impl PointVector {
    pub(super) fn ordered(&mut self) {
        self.alternatives
            .sort_by_key(|alternative| alternative.address(self.name));
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        name(self.name, &mut bytes);
        name(self.owner, &mut bytes);
        frame(&self.original, &mut bytes);
        name(self.site, &mut bytes);
        count(self.count, self.alternatives.len(), &mut bytes);
        for alternative in &self.alternatives {
            let identity = alternative
                .identity
                .clone()
                .unwrap_or_else(|| alternative.address(self.name).to_vec());
            frame(&identity, &mut bytes);
            frame(&alternative.family, &mut bytes);
            frame(&alternative.operation, &mut bytes);
        }
        bytes
    }
}

#[derive(Clone)]
pub(super) struct SurfaceVector {
    pub(super) format: u32,
    pub(super) kind: u32,
    pub(super) custody: u32,
    pub(super) family: Name,
    pub(super) policy: Vec<u8>,
    pub(super) points: Vec<PointVector>,
    pub(super) count: Option<u64>,
    pub(super) identity: Option<Vec<u8>>,
}

fn count(override_count: Option<u64>, actual: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(
        &override_count
            .unwrap_or_else(|| u64::try_from(actual).unwrap_or(u64::MAX))
            .to_be_bytes(),
    );
}

impl SurfaceVector {
    pub(super) fn declared() -> Self {
        let mut points = Vec::new();
        for stem in [b"a".as_slice(), b"b"] {
            let mut point = PointVector {
                name: (b"point", stem),
                owner: (b"owner", b"law"),
                original: vec![255],
                site: (b"site", b"fires"),
                count: None,
                alternatives: vec![
                    AlternativeVector {
                        family: b"boolean-operators".to_vec(),
                        operation: vec![0],
                        identity: None,
                    },
                    AlternativeVector {
                        family: b"boolean-operators".to_vec(),
                        operation: vec![1],
                        identity: None,
                    },
                ],
            };
            point.ordered();
            points.push(point);
        }
        Self {
            format: 1,
            kind: 1,
            custody: 0,
            family: ("évaluation".as_bytes(), b"family"),
            policy: vec![7; 32],
            points,
            count: None,
            identity: None,
        }
    }

    pub(super) fn canonical(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        name(self.family, &mut bytes);
        frame(&self.policy, &mut bytes);
        count(self.count, self.points.len(), &mut bytes);
        for point in &self.points {
            bytes.extend_from_slice(&point.encoded());
        }
        bytes
    }

    pub(super) fn body_with(&self, canonical: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&self.format.to_be_bytes());
        body.extend_from_slice(&self.kind.to_be_bytes());
        body.extend_from_slice(&self.custody.to_be_bytes());
        let identity = self
            .identity
            .clone()
            .unwrap_or_else(|| hash("evaluation-surface/v1", canonical).to_vec());
        frame(&identity, &mut body);
        frame(canonical, &mut body);
        body
    }

    pub(super) fn body(&self) -> Vec<u8> {
        self.body_with(&self.canonical())
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-evaluation-surface/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

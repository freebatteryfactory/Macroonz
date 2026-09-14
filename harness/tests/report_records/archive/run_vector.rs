//! Independently authored run framing with mutable historical members.

use super::{trial_vector, vector};

pub(super) struct RunVector {
    pub(super) context: Vec<u8>,
    pub(super) coordinates: Option<Vec<u8>>,
    pub(super) metadata: Vec<u8>,
    pub(super) posture: Vec<u8>,
    pub(super) selection: u8,
    pub(super) rows: Vec<Vec<u8>>,
    pub(super) count: u64,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = vector::hash("historical-run-report/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn row(trial: u8, revision: u8, disposition: u8, nested: &[u8]) -> Vec<u8> {
    let mut row = Vec::new();
    for byte in [trial, revision, 2, 3] {
        vector::frame(&[byte; 32], &mut row);
    }
    vector::frame(b"outside", &mut row);
    vector::frame(b"retention", &mut row);
    row.push(disposition);
    if disposition == 0 {
        vector::frame(nested, &mut row);
    }
    row
}

impl RunVector {
    pub(super) fn mixed() -> Self {
        let mut context = Vec::new();
        context.extend_from_slice(&3u32.to_be_bytes());
        context.extend_from_slice(&55u64.to_be_bytes());
        context.extend_from_slice(&89u64.to_be_bytes());
        vector::frame(b"x86-test", &mut context);
        vector::frame(b"rust-test", &mut context);
        let mut coordinates = Vec::new();
        vector::frame(&[4; 32], &mut coordinates);
        vector::frame(&[5; 32], &mut coordinates);
        coordinates.push(0);
        let mut metadata = Vec::new();
        vector::frame(b"outside", &mut metadata);
        vector::frame(b"bytes", &mut metadata);
        metadata.extend_from_slice(&7u32.to_be_bytes());
        vector::frame(&[6; 32], &mut metadata);
        let mut posture = vec![1];
        vector::frame(b"outside", &mut posture);
        vector::frame(b"parent", &mut posture);
        Self {
            context,
            coordinates: Some(coordinates),
            metadata,
            posture,
            selection: 0,
            rows: vec![
                row(9, 10, 1, &[]),
                row(
                    1,
                    8,
                    0,
                    &trial_vector::envelope(&trial_vector::body(&[3], &[1])),
                ),
                row(11, 12, 2, &[]),
            ],
            count: 3,
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in [1u32, 3, 0] {
            body.extend_from_slice(&word.to_be_bytes());
        }
        vector::frame(&self.context, &mut body);
        if let Some(coordinates) = &self.coordinates {
            body.push(1);
            vector::frame(coordinates, &mut body);
            body.extend_from_slice(&self.metadata);
        } else {
            body.push(0);
        }
        body.extend_from_slice(&self.posture);
        body.push(self.selection);
        body.extend_from_slice(&self.count.to_be_bytes());
        for row in &self.rows {
            vector::frame(row, &mut body);
        }
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

//! Expected benchmark bytes use their published grammar and independently stated fixture data.

use super::types::{DeclarationVector, ReportVector, RowVector};

pub(super) fn count(value: usize, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&u64::try_from(value).unwrap_or(u64::MAX).to_be_bytes());
}

pub(super) fn frame(material: &[u8], bytes: &mut Vec<u8>) {
    count(material.len(), bytes);
    bytes.extend_from_slice(material);
}

pub(super) fn name(namespace: &[u8], stem: &[u8], bytes: &mut Vec<u8>) {
    frame(namespace, bytes);
    frame(stem, bytes);
}

pub(super) fn address(body: &[u8]) -> Vec<u8> {
    let mut bytes = blake3::derive_key(
        "macroonz/harness-identity/historical-benchmark-report/v1",
        body,
    )
    .to_vec();
    bytes.extend_from_slice(body);
    bytes
}

impl DeclarationVector {
    pub(super) fn lawful(sizes: &[u64]) -> Self {
        Self {
            sizes: sizes.to_vec(),
            samples: 2,
            warmups: 1,
            ratio: (2, 1),
            contention: 0,
            formula: Some(b"work=samples*n".to_vec()),
        }
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        name(b"harness.bench.consumer", b"linear-workload", &mut bytes);
        count(self.sizes.len(), &mut bytes);
        for size in &self.sizes {
            bytes.extend_from_slice(&size.to_be_bytes());
        }
        name(
            b"harness.bench.consumer",
            b"correctness-preflight",
            &mut bytes,
        );
        name(b"harness.bench.consumer", b"quadratic-control", &mut bytes);
        bytes.extend_from_slice(&self.samples.to_be_bytes());
        bytes.extend_from_slice(&self.warmups.to_be_bytes());
        bytes.extend_from_slice(&self.ratio.0.to_be_bytes());
        bytes.extend_from_slice(&self.ratio.1.to_be_bytes());
        bytes.push(self.contention);
        match &self.formula {
            None => bytes.push(0),
            Some(formula) => {
                bytes.push(1);
                frame(formula, &mut bytes);
            }
        }
        name(b"harness.bench.consumer", b"linear-growth", &mut bytes);
        bytes
    }
}

pub(super) fn curve(points: &[(u64, Vec<(&str, u64)>)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    count(points.len(), &mut bytes);
    for (size, observations) in points {
        bytes.extend_from_slice(&size.to_be_bytes());
        count(observations.len(), &mut bytes);
        for (observation, value) in observations {
            name(
                b"harness.bench.consumer",
                observation.as_bytes(),
                &mut bytes,
            );
            bytes.extend_from_slice(&value.to_be_bytes());
        }
    }
    bytes
}

pub(super) fn lawful_curve(sizes: &[u64], exponent: u32) -> Vec<u8> {
    assert!(sizes.iter().all(|size| *size <= 12));
    assert!((1..=2).contains(&exponent));
    curve(
        &sizes
            .iter()
            .map(|size| {
                let value = size.saturating_pow(exponent).saturating_mul(2);
                (*size, vec![("unit-work", value)])
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn judgment(causes: [Option<(&str, &str)>; 3]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for cause in causes {
        match cause {
            None => bytes.push(0),
            Some((family, local)) => {
                bytes.push(1);
                frame(family.as_bytes(), &mut bytes);
                frame(local.as_bytes(), &mut bytes);
            }
        }
    }
    bytes
}

pub(super) fn lawful_judgment() -> Vec<u8> {
    judgment([
        None,
        Some(("harness.bench.consumer", "planted-worse-refused")),
        None,
    ])
}

pub(super) fn secondary(
    curve: &[u8],
    judgment: &[u8],
    attribution: u8,
    measurements: &[Vec<u8>],
) -> Vec<u8> {
    let mut bytes = curve.to_vec();
    bytes.extend_from_slice(judgment);
    bytes.push(attribution);
    count(measurements.len(), &mut bytes);
    for measurement in measurements {
        bytes.extend_from_slice(measurement);
    }
    bytes
}

impl RowVector {
    pub(super) fn qualified(preflight: Vec<u8>, sizes: &[u64]) -> Self {
        let measured = lawful_curve(sizes, 1);
        let judgment = lawful_judgment();
        Self {
            declaration: DeclarationVector::lawful(sizes).encoded(),
            target: b"neutral-bench-target".to_vec(),
            toolchain: b"1.98.0".to_vec(),
            preflight,
            stage: 3,
            secondary: secondary(
                &measured,
                &judgment,
                0,
                &sizes
                    .iter()
                    .flat_map(|_| [vec![1], vec![1]])
                    .collect::<Vec<_>>(),
            ),
            measured,
            planted_worse: lawful_curve(sizes, 2),
            judgment,
        }
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame(&self.declaration, &mut bytes);
        frame(&self.target, &mut bytes);
        frame(&self.toolchain, &mut bytes);
        frame(&self.preflight, &mut bytes);
        bytes.push(self.stage);
        if self.stage != 0 {
            bytes.extend_from_slice(&self.measured);
            bytes.extend_from_slice(&self.planted_worse);
            bytes.extend_from_slice(&self.judgment);
            if self.stage == 3 {
                bytes.extend_from_slice(&self.secondary);
            }
        }
        bytes
    }
}

impl ReportVector {
    pub(super) fn declared(rows: Vec<RowVector>) -> Self {
        Self {
            format: 1,
            kind: 1,
            custody: 0,
            table: (
                b"harness.bench.archive".to_vec(),
                b"complete-table".to_vec(),
            ),
            provenance: vec![0],
            rows,
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for value in [self.format, self.kind, self.custody] {
            body.extend_from_slice(&value.to_be_bytes());
        }
        name(&self.table.0, &self.table.1, &mut body);
        frame(&self.provenance, &mut body);
        count(self.rows.len(), &mut body);
        for row in &self.rows {
            body.extend_from_slice(&row.encoded());
        }
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        address(&self.body())
    }
}

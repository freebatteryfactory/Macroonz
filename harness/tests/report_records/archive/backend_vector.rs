//! Independent backend envelopes and original-material joins, without product writers.

use super::mutation_vector::MutationVector;
use super::proposal_vector::TargetVector;
use super::trial_vector::foreign;
use super::vector::{frame, hash};

pub(super) const FILE: &str = "src/étranger.rs";
pub(super) const SOURCE: &[u8] = &[0, 255, b'\n', 17];
pub(super) const CONSOLE: &str = "Found 19 mutants\nok Unmutated baseline\n  caught src/étranger.rs:43:7: replace   true with false  \nsummary\n";

#[derive(Clone, Copy)]
pub(super) enum Material {
    Absent,
    Complete,
}

pub(super) struct BackendVector {
    pub(super) header: [u32; 3],
    pub(super) invocation: Vec<u8>,
    pub(super) profile: Vec<u8>,
    pub(super) output: Vec<u8>,
    pub(super) sources: Vec<u8>,
    pub(super) run: Vec<u8>,
    pub(super) announced: Vec<u8>,
    pub(super) unparsed: Vec<u8>,
    pub(super) material: Vec<u8>,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut bytes = hash("historical-backend-manifest/v1", body).to_vec();
    bytes.extend_from_slice(body);
    bytes
}

pub(super) fn invocation(
    version: &[u8],
    executable: &[u8],
    count: u64,
    arguments: &[&[u8]],
) -> Vec<u8> {
    let mut body = vec![0];
    frame(version, &mut body);
    frame(executable, &mut body);
    body.extend_from_slice(&count.to_be_bytes());
    for argument in arguments {
        frame(argument, &mut body);
    }
    frame(b"native-test", &mut body);
    frame(b"rust-test", &mut body);
    body
}

pub(super) fn profile(version: &[u8]) -> Vec<u8> {
    let mut body = vec![0, 1];
    frame(version, &mut body);
    body.push(0);
    body.extend_from_slice(&1u32.to_be_bytes());
    body
}

pub(super) fn sources(count: u64, entries: &[(&[u8], &[u8])]) -> Vec<u8> {
    let mut body = count.to_be_bytes().to_vec();
    for (file, revision) in entries {
        frame(file, &mut body);
        frame(revision, &mut body);
    }
    body
}

pub(super) fn run(records: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    for value in [1u32, 2, 0] {
        body.extend_from_slice(&value.to_be_bytes());
    }
    body.push(0);
    body.extend_from_slice(
        &u64::try_from(records.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for record in records {
        frame(record, &mut body);
    }
    let mut encoded = hash("historical-mutation-run/v1", &body).to_vec();
    encoded.extend(body);
    encoded
}

pub(super) fn record() -> MutationVector {
    let mut target = TargetVector::external();
    let mut preimage = 1u32.to_be_bytes().to_vec();
    frame(FILE.as_bytes(), &mut preimage);
    preimage.extend_from_slice(&43u32.to_be_bytes());
    preimage.extend_from_slice(&7u32.to_be_bytes());
    frame(b"replace true with false", &mut preimage);
    target.identity = vec![0];
    frame(&hash("mutation-target/v1", &preimage), &mut target.identity);
    let mut record = MutationVector::backend();
    record.target = target.body();
    record.equivalence = 0;
    record.outcome = vec![0, 1];
    record.outcome.extend(foreign(
        "  caught src/étranger.rs:43:7: replace   true with false  ".as_bytes(),
        &[0, 0],
    ));
    record
}

pub(super) fn unread(count: u64, entries: &[(u64, Vec<u8>)]) -> Vec<u8> {
    let mut bytes = count.to_be_bytes().to_vec();
    for (ordinal, text) in entries {
        bytes.extend_from_slice(&ordinal.to_be_bytes());
        bytes.extend_from_slice(text);
    }
    bytes
}

pub(super) fn material(console: &[u8], sources: &[&[u8]]) -> Vec<u8> {
    let mut bytes = vec![1];
    frame(console, &mut bytes);
    for source in sources {
        frame(source, &mut bytes);
    }
    bytes
}

impl BackendVector {
    pub(super) fn declared(originals: Material) -> Self {
        let mut output = Vec::new();
        frame(
            &hash("mutation-backend-output/v1", CONSOLE.as_bytes()),
            &mut output,
        );
        let revision = hash("mutation-source-revision/v1", SOURCE);
        Self {
            header: [1, 1, 0],
            invocation: invocation(
                b"27-test",
                b"cargo path",
                3,
                &[b"mutants", b"", b"two words"],
            ),
            profile: profile(b"27-test"),
            output,
            sources: sources(1, &[(FILE.as_bytes(), &revision)]),
            run: run(&[record().encoded()]),
            announced: [vec![1], 19u32.to_be_bytes().to_vec()].concat(),
            unparsed: unread(1, &[(3, foreign(b"summary", &[0, 0]))]),
            material: if matches!(originals, Material::Complete) {
                material(CONSOLE.as_bytes(), &[SOURCE])
            } else {
                vec![0]
            },
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in self.header {
            body.extend_from_slice(&word.to_be_bytes());
        }
        body.extend_from_slice(&self.invocation);
        body.extend_from_slice(&self.profile);
        body.extend_from_slice(&self.output);
        body.extend_from_slice(&self.sources);
        frame(&self.run, &mut body);
        body.extend_from_slice(&self.announced);
        body.extend_from_slice(&self.unparsed);
        body.extend_from_slice(&self.material);
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

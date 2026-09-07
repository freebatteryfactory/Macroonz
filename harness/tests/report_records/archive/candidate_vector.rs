//! Candidate preimages authored independently of every descriptor constructor and writer.

use super::vector::frame;

pub(super) type Name = (&'static [u8], &'static [u8]);

#[derive(Clone, Copy)]
pub(super) enum Opening {
    Gap,
    Survivor,
}

pub(super) struct CandidateVector {
    pub(super) version: u32,
    pub(super) names: [Name; 5],
    pub(super) roles: Vec<Name>,
    pub(super) tags: Vec<Name>,
    pub(super) origin: Vec<u8>,
}

fn name((namespace, stem): Name, into: &mut Vec<u8>) {
    frame(namespace, into);
    frame(stem, into);
}

impl CandidateVector {
    pub(super) fn declared(opening: Opening) -> Self {
        let origin = match opening {
            Opening::Survivor => {
                let mut origin = vec![3, 1];
                name((b"mutation-owner", b"point"), &mut origin);
                origin
            }
            Opening::Gap => vec![3, 2],
        };
        Self {
            version: 2,
            names: [
                ("données".as_bytes(), "keeps-λ".as_bytes()),
                (b"execution-owner", b"suite"),
                (b"subject-owner", b"route"),
                (b"check-owner", b"law"),
                (b"population-owner", b"bytes"),
            ],
            roles: vec![(b"a", b"z"), (b"b", b"a")],
            tags: vec![(b"n", b"a"), (b"n", b"b")],
            origin,
        }
    }

    pub(super) fn prefix(&self) -> Vec<u8> {
        let mut encoded = self.version.to_be_bytes().to_vec();
        for field in self.names.iter().take(2) {
            name(*field, &mut encoded);
        }
        encoded
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        let mut encoded = self.prefix();
        for labels in [&self.roles, &self.tags] {
            encoded.extend_from_slice(
                &u64::try_from(labels.len())
                    .unwrap_or(u64::MAX)
                    .to_be_bytes(),
            );
            for label in labels {
                name(*label, &mut encoded);
            }
        }
        for field in self.names.iter().skip(2) {
            name(*field, &mut encoded);
        }
        encoded.extend_from_slice(&self.origin);
        encoded
    }
}

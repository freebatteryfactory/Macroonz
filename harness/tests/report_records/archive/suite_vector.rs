//! Independently framed suite-pressure claims over complete backend vectors.

use super::backend_vector::{self as backend, BackendVector, Material};
use super::trial_vector::foreign;
use super::vector::{frame, hash};

pub(super) struct SuiteVector {
    pub(super) header: [u32; 3],
    pub(super) checked: u8,
    pub(super) version: Vec<u8>,
    pub(super) backend: Vec<u8>,
    pub(super) ordinal: u64,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-compiled-suite-pressure/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn missed() -> Vec<u8> {
    let mut record = backend::record();
    record.outcome = vec![2, 4];
    record.encoded()
}

pub(super) fn mixed_backend() -> BackendVector {
    let mut second_kill = backend::record();
    second_kill.outcome = vec![0, 1];
    second_kill
        .outcome
        .extend(foreign(b"a distinct later rejection", &[0, 0]));
    let mut vector = BackendVector::declared(Material::Absent);
    vector.run = backend::run(&[missed(), backend::record().encoded(), second_kill.encoded()]);
    vector
}

impl SuiteVector {
    pub(super) fn declared(material: Material) -> Self {
        Self {
            header: [1, 1, 0],
            checked: 1,
            version: b"27-test".to_vec(),
            backend: BackendVector::declared(material).encoded(),
            ordinal: 0,
        }
    }

    pub(super) fn mixed() -> Self {
        let mut vector = Self::declared(Material::Absent);
        vector.backend = mixed_backend().encoded();
        vector.ordinal = 1;
        vector
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for word in self.header {
            body.extend_from_slice(&word.to_be_bytes());
        }
        body.push(self.checked);
        frame(&self.version, &mut body);
        frame(&self.backend, &mut body);
        body.extend_from_slice(&self.ordinal.to_be_bytes());
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

//! Independently framed historical claims, without the product writer or live evidence.

#[derive(Clone, Copy)]
pub(super) enum InputKind {
    Unit,
    Bound,
}

pub(super) struct Vector {
    pub(super) format: u32,
    pub(super) kind: u32,
    pub(super) custody: u32,
    pub(super) key: Vec<u8>,
    pub(super) metadata: Vec<u8>,
    pub(super) fingerprint: Vec<u8>,
    pub(super) capsule: Vec<u8>,
}

pub(super) fn frame(bytes: &[u8], into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_be_bytes());
    into.extend_from_slice(bytes);
}

pub(super) fn hash(family: &str, bytes: &[u8]) -> [u8; 32] {
    blake3::derive_key(&format!("macroonz/harness-identity/{family}"), bytes)
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-replay-capsule/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

impl Vector {
    pub(super) fn declared(kind: InputKind) -> Self {
        let typed = matches!(kind, InputKind::Bound);
        let mut key = Vec::new();
        for byte in [1u8, 2, 3] {
            frame(&[byte; 32], &mut key);
        }
        key.extend_from_slice(&3u32.to_be_bytes());
        key.extend_from_slice(&55u64.to_be_bytes());
        key.extend_from_slice(&89u64.to_be_bytes());
        frame(b"x86-test", &mut key);
        frame(b"rust-test", &mut key);
        let mut metadata = vec![u8::from(typed)];
        if typed {
            frame(&[4; 32], &mut key);
            frame(&[5; 32], &mut key);
            key.push(0);
            frame(b"outside", &mut metadata);
            frame(b"bytes", &mut metadata);
            metadata.extend_from_slice(&7u32.to_be_bytes());
            frame(&[6; 32], &mut metadata);
        }
        let mut fingerprint = Vec::new();
        frame(&[1; 32], &mut fingerprint);
        frame(b"fixture", &mut fingerprint);
        frame(b"disagrees", &mut fingerprint);
        fingerprint.push(1);
        let mut capsule = Vec::new();
        let key_family = if typed {
            "input-execution-key/v1"
        } else {
            "execution-key/v1"
        };
        frame(&hash(key_family, &key), &mut capsule);
        frame(&[1], &mut capsule);
        frame(&hash("failure-fingerprint/v1", &fingerprint), &mut capsule);
        frame(b"profile", &mut capsule);
        capsule.extend_from_slice(&2u32.to_be_bytes());
        frame(b"reduce", &mut capsule);
        capsule.extend_from_slice(&3u32.to_be_bytes());
        frame(&[7; 32], &mut capsule);
        capsule.push(1);
        Self {
            format: 1,
            kind: 1,
            custody: 0,
            key,
            metadata,
            fingerprint,
            capsule,
        }
    }

    pub(super) fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&self.format.to_be_bytes());
        body.extend_from_slice(&self.kind.to_be_bytes());
        body.extend_from_slice(&self.custody.to_be_bytes());
        frame(&self.key, &mut body);
        body.extend_from_slice(&self.metadata);
        frame(&self.fingerprint, &mut body);
        frame(&self.capsule, &mut body);
        body
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}

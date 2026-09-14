//! Independently stated bytes and malformed inputs challenge generated codec operations.

use super::wire::{AssemblyRefusal, Child, Mode, Record, baked::RecordDecodeError};

const EXPECTED: &[u8] = &[
    0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 1, 0, 0, 0, 0, 0, 0, 0, 2, 104, 105, 0,
    0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 7,
];

pub(super) fn round_trip() {
    let expected = Record {
        count: 513,
        payload: vec![3, 4],
        label: Some("hi".to_owned()),
        modes: vec![Mode::Read, Mode::Write],
        child: Child { value: 7 },
    };
    let mut encoded = Vec::new();
    expected.encode_canonical(&mut encoded);
    assert_eq!(encoded, EXPECTED);
    assert_eq!(Record::decode_canonical(EXPECTED), Ok(expected));

    let sparse = Record {
        count: 1,
        payload: Vec::new(),
        label: None,
        modes: Vec::new(),
        child: Child { value: 0 },
    };
    let mut sparse_bytes = Vec::new();
    sparse.encode_canonical(&mut sparse_bytes);
    assert_eq!(Record::decode_canonical(&sparse_bytes), Ok(sparse));
}

pub(super) fn malformed() -> Result<(), String> {
    for (offset, value, expected) in [
        (
            18,
            2,
            RecordDecodeError::PresenceNotAdmitted { member: "label" },
        ),
        (27, 255, RecordDecodeError::TextNotUtf8 { member: "label" }),
        (
            37,
            9,
            RecordDecodeError::SlotNotAdmitted { member: "modes" },
        ),
        (
            15,
            99,
            RecordDecodeError::LengthPastRemaining { member: "payload" },
        ),
        (
            47,
            1,
            RecordDecodeError::NestedMemberRefused { member: "child" },
        ),
        (
            0,
            1,
            RecordDecodeError::CountPastDeclaredWidth { member: "count" },
        ),
    ] {
        let mut damaged = EXPECTED.to_vec();
        *damaged
            .get_mut(offset)
            .ok_or("example byte offset absent")? = value;
        assert_eq!(Record::decode_canonical(&damaged), Err(expected));
    }
    let mut trailing = EXPECTED.to_vec();
    trailing.push(0);
    assert_eq!(
        Record::decode_canonical(&trailing),
        Err(RecordDecodeError::TrailingBytes)
    );
    assert_eq!(
        Record::decode_canonical(&[0, 1, 2]),
        Err(RecordDecodeError::Truncated { member: "count" })
    );
    Ok(())
}

pub(super) fn checked_assembly() -> Result<(), String> {
    let mut zero_count = EXPECTED.to_vec();
    zero_count.get_mut(..8).ok_or("count bytes absent")?.fill(0);
    assert_eq!(
        Record::decode_canonical(&zero_count),
        Err(RecordDecodeError::NotAssembled(AssemblyRefusal::ZeroCount))
    );
    Ok(())
}

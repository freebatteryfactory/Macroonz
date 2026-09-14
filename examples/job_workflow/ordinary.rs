//! Independent finite-population expectations for ordinary Job changes.

use crate::job::{Event, Record, Stage, baked};

pub(super) fn lifecycle() {
    assert_eq!(
        baked::EVENT_VARIANTS,
        &[Event::Queue, Event::Complete, Event::Cancel]
    );
    assert_eq!(
        baked::STAGE_VARIANTS,
        &[Stage::Draft, Stage::Queued, Stage::Done]
    );
    assert_eq!(baked::apply(Stage::Draft, Event::Queue), Ok(Stage::Queued));
    assert_eq!(
        baked::apply(Stage::Queued, Event::Complete),
        Ok(Stage::Done)
    );
    assert!(baked::apply(Stage::Draft, Event::Complete).is_err());
    assert!(baked::apply(Stage::Queued, Event::Queue).is_err());
    assert_eq!(baked::apply(Stage::Done, Event::Queue), Ok(Stage::Queued));
    assert!(baked::apply(Stage::Done, Event::Complete).is_err());
    assert!(baked::apply(Stage::Draft, Event::Cancel).is_err());
    assert_eq!(baked::apply(Stage::Queued, Event::Cancel), Ok(Stage::Draft));
    assert!(baked::apply(Stage::Done, Event::Cancel).is_err());
}

pub(super) fn record_bytes() {
    let record = Record {
        count: 513,
        attempts: 7,
    };
    let mut bytes = Vec::new();
    record.encode_canonical(&mut bytes);
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 7]);
    assert_eq!(Record::decode_canonical(&bytes), Ok(record));
    assert!(Record::decode_canonical(&[0, 0, 0, 0, 0, 0, 2, 1]).is_err());
    assert!(Record::decode_canonical(&[0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 1, 0]).is_err());
    bytes.push(0);
    assert!(Record::decode_canonical(&bytes).is_err());
    assert!(Record::decode_canonical(&[]).is_err());
}

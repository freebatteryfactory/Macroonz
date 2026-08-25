//! A capture operation consumes its level, so a producer refusal leaves no partial level to finish.

use macroonz::{CaptureBuilder, CapturedAtom};

fn main() {
    let mut builder = CaptureBuilder::<u64>::declared();
    let level = builder.open();
    let _refused = level.atom(0, |_| Err::<CapturedAtom, _>("unread"));
    let _partial = level.finish();
}

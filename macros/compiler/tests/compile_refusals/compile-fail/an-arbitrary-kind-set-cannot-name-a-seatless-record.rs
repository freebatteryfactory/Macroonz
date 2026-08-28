//! A kind set's associated record must surrender dispositions, so `()` cannot stand in for the answers a multi-kind set owes.

use macroonz_compiler::KindSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SeatlessKinds;

impl KindSet for SeatlessKinds {
    type Dispositions = ();

    const NAMES: &'static [&'static str] = &["seatless.first", "seatless.second"];
}

fn main() {
    let _names = <SeatlessKinds as KindSet>::NAMES;
}

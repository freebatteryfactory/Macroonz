//! A captured token receives its path and handle from the builder rather than from sibling arguments.

use macroonz_compiler::{CapturedPayload, CapturedTokenTree, SpanHandle, TokenPath};

fn main() {
    let _forged = CapturedTokenTree::captured(
        CapturedPayload::Word(String::from("forged")),
        TokenPath::root(),
        SpanHandle::at(41),
    );
}

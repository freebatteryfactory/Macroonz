//! A compiler-diagnostic source position can be minted only through its informing constructor.

use macroonz_harness::oracle::SourcePosition;

fn bypass() {
    let _ = SourcePosition {
        line: 1,
        column: 1,
    };
}

fn main() {}

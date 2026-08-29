//! A rustc error code and relative source path can be minted only through their informing constructors.

use macroonz_harness::oracle::{RelativeSourcePath, RustcErrorCode};

fn bypass() {
    let _ = RustcErrorCode("E0308".to_owned());
    let _ = RelativeSourcePath("src/main.rs".to_owned());
}

fn main() {}

use macroonz_compiler::{CaptureCursor, CapturedTokenTree};

fn forged(tokens: &[CapturedTokenTree]) -> CaptureCursor<'_> {
    CaptureCursor {
        tokens,
        next: 1,
        end: None,
    }
}

fn main() {
    let _ = forged(&[]);
}

use macroonz_compiler::{CapturedFragment, SpanHandle};

fn main() {
    let _forged = CapturedFragment {
        tokens: &[],
        end: Some(SpanHandle::at(0)),
    };
}

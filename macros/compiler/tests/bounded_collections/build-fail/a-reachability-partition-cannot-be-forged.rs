use macroonz_compiler::{Bounded, NonEmpty, Reachability};

fn main() {
    let _forged = Reachability::<2> {
        reachable: NonEmpty::one(0),
        unreachable: Bounded::empty(),
    };
}

use macroonz_harness::input::BoundInput;

fn require(_: BoundInput<u16>) {}

fn wrong(input: BoundInput<u8>) {
    require(input);
}

fn main() {}

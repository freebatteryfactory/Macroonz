//! The generated-delimiter roster now accounts for invisible compiler groups preserved from caller-authored Rust.

use macroonz_compiler::GeneratedDelimiter;

fn old_slot(delimiter: GeneratedDelimiter) -> u8 {
    match delimiter {
        GeneratedDelimiter::Parenthesis => 0,
        GeneratedDelimiter::Brace => 1,
        GeneratedDelimiter::Bracket => 2,
    }
}

fn main() {
    let _slot = old_slot(GeneratedDelimiter::Bare);
}

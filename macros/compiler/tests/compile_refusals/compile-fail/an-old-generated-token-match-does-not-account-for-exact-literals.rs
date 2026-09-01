//! The generated-token roster now accounts for exact caller-authored literals preserved through the proc host.

use macroonz_compiler::GeneratedToken;

fn old_slot(token: GeneratedToken) -> u8 {
    match token {
        GeneratedToken::Word(_) => 0,
        GeneratedToken::Punct {
            mark: _,
            spacing: _,
        } => 1,
        GeneratedToken::Text(_) => 2,
        GeneratedToken::Group {
            delimiter: _,
            tokens: _,
        } => 3,
        GeneratedToken::ByteText(_) => 4,
        GeneratedToken::Number(_) => 5,
        GeneratedToken::RawIdentifier(_) => 6,
    }
}

fn main() {
    let _slot = old_slot(GeneratedToken::word("migration"));
}

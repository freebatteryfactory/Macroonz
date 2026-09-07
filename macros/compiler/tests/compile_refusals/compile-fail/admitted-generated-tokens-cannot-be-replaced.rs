//! Even an exclusive tree owner cannot replace its admitted tokens through the public read.

use macroonz_compiler::{GeneratedToken, GeneratedTree};

fn replace(tree: &mut GeneratedTree) {
    tree.tokens()[0] = GeneratedToken::word("#");
}

fn main() {
    if let Ok(mut tree) = GeneratedTree::assembled(vec![GeneratedToken::word("lawful")]) {
        replace(&mut tree);
    }
}

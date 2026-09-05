//! One pre-order walk over captured token trees, shared by every compiler lane that locates tokens by position.

use macroonz_compiler::CapturedTokenTree;

/// Every captured token at and below the given trees, in reading order, with each group before its children.
pub(crate) fn flattened(trees: &[CapturedTokenTree]) -> Vec<&CapturedTokenTree> {
    let mut found = Vec::new();
    collect(trees, &mut found);
    found
}

fn collect<'tree>(trees: &'tree [CapturedTokenTree], into: &mut Vec<&'tree CapturedTokenTree>) {
    for tree in trees {
        into.push(tree);
        if let Some((_delimiter, children)) = tree.group() {
            collect(children, into);
        }
    }
}

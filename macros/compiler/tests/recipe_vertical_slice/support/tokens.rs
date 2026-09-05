//! Span handles located inside one recipe source by the authored word or group a claim names.

use crate::captured_tokens::flattened;
use macroonz_compiler::{CapturedTokenTree, SpanHandle, TextCapture};

/// Which occurrence of a repeated word a claim points at.
#[derive(Clone, Copy)]
pub(crate) enum Occurrence {
    First,
    Last,
    Nth(usize),
}

fn captured(source: &str) -> Result<Vec<CapturedTokenTree>, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    Ok(read.input().trees().to_vec())
}

/// The span of one authored word, chosen by occurrence in reading order.
pub(crate) fn word_handle(
    source: &str,
    word: &str,
    occurrence: Occurrence,
) -> Result<SpanHandle, ()> {
    let trees = captured(source)?;
    let mut matching = flattened(&trees)
        .into_iter()
        .filter(|tree| tree.word() == Some(word));
    match occurrence {
        Occurrence::First => matching.next(),
        Occurrence::Last => matching.next_back(),
        Occurrence::Nth(index) => matching.nth(index),
    }
    .map(CapturedTokenTree::span)
    .ok_or(())
}

/// The span of the group that directly follows the last occurrence of one authored word.
pub(crate) fn group_after_word(source: &str, word: &str) -> Result<SpanHandle, ()> {
    let trees = captured(source)?;
    let flat = flattened(&trees);
    let position = flat
        .iter()
        .rposition(|tree| tree.word() == Some(word))
        .ok_or(())?;
    flat.get(position.saturating_add(1))
        .filter(|tree| tree.group().is_some())
        .map(|tree| tree.span())
        .ok_or(())
}

/// The span of the last group whose direct children include one authored word.
pub(crate) fn last_group_directly_containing(source: &str, word: &str) -> Result<SpanHandle, ()> {
    let trees = captured(source)?;
    flattened(&trees)
        .into_iter()
        .rfind(|tree| {
            tree.group().is_some_and(|(_delimiter, children)| {
                children.iter().any(|child| child.word() == Some(word))
            })
        })
        .map(CapturedTokenTree::span)
        .ok_or(())
}

/// The span of the narrowest group whose direct children include one authored word.
pub(crate) fn narrow_group_containing(source: &str, word: &str) -> Result<SpanHandle, ()> {
    let trees = captured(source)?;
    find_group(&trees, word).ok_or(())
}

fn find_group(trees: &[CapturedTokenTree], word: &str) -> Option<SpanHandle> {
    for tree in trees {
        let Some((_delimiter, children)) = tree.group() else {
            continue;
        };
        if let Some(found) = find_group(children, word) {
            return Some(found);
        }
        if children.iter().any(|child| child.word() == Some(word)) {
            return Some(tree.span());
        }
    }
    None
}

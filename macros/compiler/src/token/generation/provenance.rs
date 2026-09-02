//! Nonsemantic source restoration for generated trees.

use super::{GeneratedSpacing, GeneratedToken, GeneratedTree};
use crate::token::SpanHandle;
use std::collections::{BTreeMap, VecDeque};

impl GeneratedTree {
    /// Restore one exact caller-authored token run onto the first matching generated run that carries no producer spans yet.
    #[must_use]
    pub(crate) fn restored_from(&self, source: &Self) -> Self {
        let mut target_tokens = Vec::new();
        preorder_tokens(self.tokens.as_slice(), &mut target_tokens);
        let mut source_tokens = Vec::new();
        preorder_tokens(source.tokens.as_slice(), &mut source_tokens);
        let Some(opening) = restoration_opening(self, &target_tokens, source, &source_tokens)
        else {
            return self.clone();
        };
        self.restored_run(source, opening)
    }

    /// Restore one exact caller-authored function signature and every generated use of its parameter bindings.
    #[must_use]
    pub(crate) fn restored_function_from(
        &self,
        source: &Self,
        bindings: &[GeneratedToken],
    ) -> Self {
        let mut target_tokens = Vec::new();
        preorder_tokens(self.tokens.as_slice(), &mut target_tokens);
        let mut source_tokens = Vec::new();
        preorder_tokens(source.tokens.as_slice(), &mut source_tokens);
        let Some(opening) = restoration_opening(self, &target_tokens, source, &source_tokens)
        else {
            return self.clone();
        };
        let mut restored = self.restored_run(source, opening);
        let body_position = opening.saturating_add(source_tokens.len());
        let Some(GeneratedToken::Group {
            tokens: body_tokens,
            ..
        }) = target_tokens.get(body_position).copied()
        else {
            return restored;
        };
        let body_end = body_position
            .saturating_add(1)
            .saturating_add(recursive_token_count(body_tokens.as_slice()));
        for binding in bindings {
            let Some(binding_span) = source_tokens
                .iter()
                .zip(&source.source_spans)
                .find_map(|(token, span)| (*token == binding).then_some(*span).flatten())
            else {
                continue;
            };
            restore_binding_uses(
                &mut restored,
                &target_tokens,
                binding,
                binding_span,
                body_position.saturating_add(1)..body_end,
            );
        }
        restored
    }

    /// Restore one exact caller-authored body together with its nearest matching generated binding.
    #[must_use]
    pub(crate) fn restored_body_from(
        &self,
        source: &Self,
        binding: &GeneratedToken,
        binding_span: SpanHandle,
    ) -> Self {
        let mut target_tokens = Vec::new();
        preorder_tokens(self.tokens.as_slice(), &mut target_tokens);
        let mut source_tokens = Vec::new();
        preorder_tokens(source.tokens.as_slice(), &mut source_tokens);
        let Some(opening) = restoration_opening(self, &target_tokens, source, &source_tokens)
        else {
            return self.clone();
        };
        let mut restored = self.restored_run(source, opening);
        let Some(binding_position) = (0..opening).rev().find(|position| {
            target_tokens
                .get(*position)
                .is_some_and(|token| *token == binding)
        }) else {
            return restored;
        };
        if let Some(target_span) = restored.source_spans.get_mut(binding_position)
            && target_span.is_none()
        {
            *target_span = Some(binding_span);
        }
        restored
    }

    /// Restore caller-authored identifiers onto generated paths and caller-named item declarations.
    #[must_use]
    pub(crate) fn restored_references_from(&self, source: &Self) -> Self {
        let mut source_tokens = Vec::new();
        preorder_tokens(source.tokens.as_slice(), &mut source_tokens);
        let mut available = BTreeMap::<(u8, String), VecDeque<SpanHandle>>::new();
        for (token, source_span) in source_tokens.iter().zip(&source.source_spans) {
            if let (Some(key), Some(source_span)) = (identifier_source_key(token), source_span) {
                available.entry(key).or_default().push_back(*source_span);
            }
        }
        let mut target_tokens = Vec::new();
        preorder_tokens(self.tokens.as_slice(), &mut target_tokens);
        let mut restored = self.clone();
        for (position, token) in target_tokens.iter().enumerate() {
            if !self.source_spans.get(position).is_some_and(Option::is_none)
                || !is_reference_position(&target_tokens, position)
            {
                continue;
            }
            let Some(key) = identifier_source_key(token) else {
                continue;
            };
            let Some(source_spans) = available.get_mut(&key) else {
                continue;
            };
            let Some(source_span) = source_spans.pop_back() else {
                continue;
            };
            if let Some(target_span) = restored.source_spans.get_mut(position) {
                *target_span = Some(source_span);
            }
            source_spans.push_front(source_span);
        }
        restored
    }

    fn restored_run(&self, source: &Self, opening: usize) -> Self {
        let mut restored = self.clone();
        for (offset, source_span) in source.source_spans.iter().copied().enumerate() {
            if source_span.is_some()
                && let Some(target_span) = restored
                    .source_spans
                    .get_mut(opening.saturating_add(offset))
            {
                *target_span = source_span;
            }
        }
        restored
    }
}

/// Append one generated tree's tokens in the same pre-order as its source-span roster.
fn preorder_tokens<'tokens>(
    tokens: &'tokens [GeneratedToken],
    into: &mut Vec<&'tokens GeneratedToken>,
) {
    for token in tokens {
        into.push(token);
        if let GeneratedToken::Group {
            tokens: nested_tokens,
            ..
        } = token
        {
            preorder_tokens(nested_tokens.as_slice(), into);
        }
    }
}

/// The recursive token denominator one restored function body occupies.
fn recursive_token_count(tokens: &[GeneratedToken]) -> usize {
    tokens.iter().fold(0usize, |count, token| {
        let nested = match token {
            GeneratedToken::Group {
                tokens: nested_tokens,
                ..
            } => recursive_token_count(nested_tokens.as_slice()),
            GeneratedToken::Word(_)
            | GeneratedToken::Punct { .. }
            | GeneratedToken::Text(_)
            | GeneratedToken::ByteText(_)
            | GeneratedToken::Number(_)
            | GeneratedToken::RawIdentifier(_)
            | GeneratedToken::Literal(_) => 0,
        };
        count.saturating_add(1).saturating_add(nested)
    })
}

/// The ordered lookup key for one authored identifier form.
fn identifier_source_key(token: &GeneratedToken) -> Option<(u8, String)> {
    match token {
        GeneratedToken::Word(spelling) => Some((0, spelling.clone())),
        GeneratedToken::RawIdentifier(spelling) => Some((1, spelling.clone())),
        GeneratedToken::Punct { .. }
        | GeneratedToken::Text(_)
        | GeneratedToken::Group { .. }
        | GeneratedToken::ByteText(_)
        | GeneratedToken::Number(_)
        | GeneratedToken::Literal(_) => None,
    }
}

/// Find the first exact generated run that can receive one source-span roster.
fn restoration_opening(
    target: &GeneratedTree,
    target_tokens: &[&GeneratedToken],
    source: &GeneratedTree,
    source_tokens: &[&GeneratedToken],
) -> Option<usize> {
    if source_tokens.is_empty() {
        return None;
    }
    target_tokens
        .windows(source_tokens.len())
        .enumerate()
        .find_map(|(opening, candidate)| {
            let tokens_match = candidate
                .iter()
                .zip(source_tokens)
                .all(|(generated, authored)| *generated == *authored);
            let spans_are_open =
                source
                    .source_spans
                    .iter()
                    .enumerate()
                    .all(|(offset, source_span)| {
                        source_span.is_none()
                            || target
                                .source_spans
                                .get(opening.saturating_add(offset))
                                .is_some_and(Option::is_none)
                    });
            (tokens_match && spans_are_open).then_some(opening)
        })
}

/// Restore one exact parameter binding onto matching generated uses inside its function body.
fn restore_binding_uses(
    restored: &mut GeneratedTree,
    tokens: &[&GeneratedToken],
    binding: &GeneratedToken,
    binding_span: SpanHandle,
    positions: core::ops::Range<usize>,
) {
    for position in positions {
        if tokens.get(position).is_none_or(|token| *token != binding) {
            continue;
        }
        let Some(target_span) = restored.source_spans.get_mut(position) else {
            continue;
        };
        if target_span.is_none() {
            *target_span = Some(binding_span);
        }
    }
}

/// Whether one flattened generated identifier is a path segment or caller-named item seat.
fn is_reference_position(tokens: &[&GeneratedToken], position: usize) -> bool {
    let path_segment = position
        .checked_sub(2)
        .zip(position.checked_sub(1))
        .is_some_and(|(joint, alone)| {
            matches!(
                (tokens.get(joint), tokens.get(alone)),
                (
                    Some(GeneratedToken::Punct {
                        mark: ':',
                        spacing: GeneratedSpacing::Joint,
                    }),
                    Some(GeneratedToken::Punct {
                        mark: ':',
                        spacing: GeneratedSpacing::Alone,
                    })
                )
            )
        });
    let caller_named_item = position
        .checked_sub(1)
        .and_then(|previous| tokens.get(previous))
        .is_some_and(|previous| {
            matches!(
                previous,
                GeneratedToken::Word(keyword) if matches!(keyword.as_str(), "fn" | "mod")
            )
        });
    path_segment || caller_named_item
}

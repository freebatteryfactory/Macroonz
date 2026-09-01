//! A shallow structural lens into one complete caller-authored Rust item.
//!
//! The lens recognizes only the item envelope needed to preserve and augment authored material.
//! It does not parse a second Rust AST, decide what an item means, or replace Rustc's grammar judgment.

use super::{
    AuthoredItem, AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal,
    CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle,
};

/// The established coordinates used to assemble one borrowed item lens.
#[derive(Clone, Copy)]
struct ItemCoordinates {
    attributes_end: usize,
    visibility_end: usize,
    kind_index: usize,
    kind: AuthoredItemKind,
    name_index: Option<usize>,
    terminator: usize,
    body_index: Option<usize>,
}

impl CapturedInput {
    /// Read this declared item boundary into a source-coupled structural lens for one supported item family.
    ///
    /// The complete token reading remains available through [`AuthoredItem::preserved`].
    /// The lens identifies only the outer item envelope and leaves full Rust legality to Rustc.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal where the boundary is empty, has no recognized item family or required name, or does not end as one complete item boundary.
    pub fn authored_item(&self) -> Result<AuthoredItem<'_>, AuthoredItemReadRefusal> {
        read_item(self.trees())
    }
}

impl<'tokens> AuthoredItem<'tokens> {
    /// The complete authored token reading this lens stands over.
    #[must_use]
    pub const fn preserved(self) -> CapturedFragment<'tokens> {
        self.preserved
    }

    /// The leading outer attributes, in authored order.
    #[must_use]
    pub const fn attributes(self) -> CapturedFragment<'tokens> {
        self.attributes
    }

    /// The explicit visibility tokens, or an empty fragment for inherited visibility.
    #[must_use]
    pub const fn visibility(self) -> CapturedFragment<'tokens> {
        self.visibility
    }

    /// The qualifiers between visibility and the item-family keyword.
    #[must_use]
    pub const fn qualifiers(self) -> CapturedFragment<'tokens> {
        self.qualifiers
    }

    /// The recognized structural item family.
    #[must_use]
    pub const fn kind(self) -> AuthoredItemKind {
        self.kind
    }

    /// The exact token carrying the item-family keyword.
    #[must_use]
    pub const fn kind_token(self) -> &'tokens CapturedTokenTree {
        self.kind_token
    }

    /// The optional item-name token and its ordinary or raw spelling.
    #[must_use]
    pub fn name(self) -> Option<(&'tokens CapturedTokenTree, &'tokens str)> {
        self.name_token
            .and_then(|token| identifier(token).map(|spelling| (token, spelling)))
    }

    /// The exact generic-parameter run, including its angle punctuation, where one is present.
    #[must_use]
    pub const fn generics(self) -> Option<CapturedFragment<'tokens>> {
        self.generics
    }

    /// The exact where-clause run, beginning with `where`, where one is present.
    #[must_use]
    pub const fn where_clause(self) -> Option<CapturedFragment<'tokens>> {
        self.where_clause
    }

    /// The item-signature run after outer attributes and before its first body group or terminator.
    #[must_use]
    pub const fn signature(self) -> CapturedFragment<'tokens> {
        self.signature
    }

    /// The optional body group's delimiter and exact inner token fragment.
    #[must_use]
    pub fn body(self) -> Option<(CapturedDelimiter, CapturedFragment<'tokens>)> {
        self.body_delimiter.zip(self.body)
    }

    /// The explicit `unsafe` qualifier token on this item, where the caller wrote one.
    ///
    /// This is a local syntactic boundary, not a global unsafe scan or a soundness claim.
    #[must_use]
    pub const fn unsafe_token(self) -> Option<&'tokens CapturedTokenTree> {
        self.unsafe_token
    }
}

impl AuthoredItemReadRefusal {
    /// The structural item-envelope issue this read established.
    pub const fn issue(self) -> AuthoredItemReadIssue {
        self.issue
    }

    /// The exact producer span available at the refusal site.
    #[must_use]
    pub const fn token(self) -> Option<SpanHandle> {
        self.at
    }
}

/// Read one item lens after the caller or proc boundary declared the complete item run.
fn read_item(tokens: &[CapturedTokenTree]) -> Result<AuthoredItem<'_>, AuthoredItemReadRefusal> {
    if tokens.is_empty() {
        return Err(refused(AuthoredItemReadIssue::ItemMissing, None));
    }
    let attributes_end = attributes_end(tokens);
    let visibility_end = visibility_end(tokens, attributes_end);
    let (kind_index, kind) = item_kind(tokens, visibility_end)?;
    let name_index = item_name(tokens, kind_index, kind)?;
    let terminator = terminator(tokens, kind)?;
    let body_index = body_index(tokens, name_index, kind, terminator);
    assemble(
        tokens,
        ItemCoordinates {
            attributes_end,
            visibility_end,
            kind_index,
            kind,
            name_index,
            terminator,
            body_index,
        },
    )
}

/// Assemble the lens after every structural coordinate has been established.
fn assemble(
    tokens: &[CapturedTokenTree],
    coordinates: ItemCoordinates,
) -> Result<AuthoredItem<'_>, AuthoredItemReadRefusal> {
    let signature_end = coordinates.body_index.unwrap_or(coordinates.terminator);
    let generic_start = coordinates.name_index.map_or_else(
        || coordinates.kind_index.checked_add(1),
        |index| index.checked_add(1),
    );
    let generics = generic_start
        .and_then(|start| generic_range(tokens, start, signature_end))
        .map(|(start, end)| fragment(tokens, start, end, None))
        .transpose()?;
    let where_clause = word_index(
        tokens,
        "where",
        coordinates.kind_index,
        coordinates.terminator,
    )
    .map(|start| fragment(tokens, start, coordinates.terminator, None))
    .transpose()?;
    let body_token = coordinates.body_index.and_then(|index| tokens.get(index));
    let (body_delimiter, body) = body_token.and_then(CapturedTokenTree::group).map_or(
        (None, None),
        |(delimiter, members)| {
            (
                Some(delimiter),
                Some(CapturedFragment::over(
                    members,
                    body_token.map(CapturedTokenTree::span),
                )),
            )
        },
    );
    Ok(AuthoredItem {
        preserved: fragment(tokens, 0, tokens.len(), None)?,
        attributes: fragment(tokens, 0, coordinates.attributes_end, None)?,
        visibility: fragment(
            tokens,
            coordinates.attributes_end,
            coordinates.visibility_end,
            None,
        )?,
        qualifiers: fragment(
            tokens,
            coordinates.visibility_end,
            coordinates.kind_index,
            None,
        )?,
        signature: fragment(tokens, coordinates.attributes_end, signature_end, None)?,
        generics,
        where_clause,
        body,
        body_delimiter,
        kind: coordinates.kind,
        kind_token: token_at(tokens, coordinates.kind_index)?,
        name_token: coordinates.name_index.and_then(|index| tokens.get(index)),
        unsafe_token: word_token(
            tokens,
            "unsafe",
            coordinates.visibility_end,
            coordinates.kind_index,
        ),
    })
}

/// The end of the leading outer-attribute run.
fn attributes_end(tokens: &[CapturedTokenTree]) -> usize {
    let mut next = 0usize;
    loop {
        let Some(mark) = tokens.get(next) else {
            return next;
        };
        let Some(group) = next.checked_add(1).and_then(|index| tokens.get(index)) else {
            return next;
        };
        let is_outer = mark.punct() == Some('#')
            && group
                .group()
                .is_some_and(|(delimiter, _)| delimiter == CapturedDelimiter::Bracket);
        if !is_outer {
            return next;
        }
        next = next.saturating_add(2);
    }
}

/// The end of one optional `pub` visibility, including a restriction group.
fn visibility_end(tokens: &[CapturedTokenTree], start: usize) -> usize {
    let Some(token) = tokens.get(start) else {
        return start;
    };
    if token.word() != Some("pub") {
        return start;
    }
    let after_pub = start.saturating_add(1);
    tokens.get(after_pub).map_or(after_pub, |next| {
        if next
            .group()
            .is_some_and(|(delimiter, _)| delimiter == CapturedDelimiter::Parenthesis)
        {
            after_pub.saturating_add(1)
        } else {
            after_pub
        }
    })
}

/// Find the item-family keyword after the lawful qualifier vocabulary.
fn item_kind(
    tokens: &[CapturedTokenTree],
    start: usize,
) -> Result<(usize, AuthoredItemKind), AuthoredItemReadRefusal> {
    let mut next = start;
    while let Some(token) = tokens.get(next) {
        let word = token.word();
        let found = match word {
            Some("mod") => Some(AuthoredItemKind::Module),
            Some("struct") => Some(AuthoredItemKind::Structure),
            Some("enum") => Some(AuthoredItemKind::Enumeration),
            Some("union") => Some(AuthoredItemKind::Union),
            Some("trait") => Some(AuthoredItemKind::Trait),
            Some("fn") => Some(AuthoredItemKind::Function),
            Some("impl") => Some(AuthoredItemKind::Implementation),
            Some("type") => Some(AuthoredItemKind::TypeAlias),
            Some("static") => Some(AuthoredItemKind::Static),
            Some("use") => Some(AuthoredItemKind::Use),
            Some("const") if !function_follows(tokens, next.saturating_add(1)) => {
                Some(AuthoredItemKind::Constant)
            }
            Some("extern") if next_word(tokens, next) == Some("crate") => {
                Some(AuthoredItemKind::ExternalCrate)
            }
            Some("unsafe" | "async" | "default" | "auto" | "extern" | "const") => None,
            _ => {
                return Err(refused(
                    AuthoredItemReadIssue::ItemKindMissing,
                    Some(token.span()),
                ));
            }
        };
        if let Some(kind) = found {
            return Ok((next, kind));
        }
        next = next.saturating_add(1);
        if word == Some("extern")
            && tokens
                .get(next)
                .is_some_and(|candidate| candidate.text().is_some())
        {
            next = next.saturating_add(1);
        }
    }
    Err(refused(
        AuthoredItemReadIssue::ItemKindMissing,
        tokens.last().map(CapturedTokenTree::span),
    ))
}

/// Whether the qualifier run after `const` reaches a function item.
fn function_follows(tokens: &[CapturedTokenTree], start: usize) -> bool {
    let mut next = start;
    while let Some(token) = tokens.get(next) {
        match token.word() {
            Some("fn") => return true,
            Some("unsafe" | "async" | "extern") => {
                next = next.saturating_add(1);
                if token.word() == Some("extern")
                    && tokens
                        .get(next)
                        .is_some_and(|candidate| candidate.text().is_some())
                {
                    next = next.saturating_add(1);
                }
            }
            _ => return false,
        }
    }
    false
}

/// Read the required name seat for item families that carry one.
fn item_name(
    tokens: &[CapturedTokenTree],
    kind_index: usize,
    kind: AuthoredItemKind,
) -> Result<Option<usize>, AuthoredItemReadRefusal> {
    if matches!(
        kind,
        AuthoredItemKind::Implementation | AuthoredItemKind::Use
    ) {
        return Ok(None);
    }
    let mut candidate = kind_index.saturating_add(1);
    if kind == AuthoredItemKind::ExternalCrate {
        candidate = candidate.saturating_add(1);
    }
    if kind == AuthoredItemKind::Static
        && tokens.get(candidate).and_then(CapturedTokenTree::word) == Some("mut")
    {
        candidate = candidate.saturating_add(1);
    }
    let token = tokens.get(candidate).ok_or(refused(
        AuthoredItemReadIssue::ItemNameMissing(kind),
        tokens.get(kind_index).map(CapturedTokenTree::span),
    ))?;
    identifier(token).ok_or(refused(
        AuthoredItemReadIssue::ItemNameMissing(kind),
        Some(token.span()),
    ))?;
    Ok(Some(candidate))
}

/// Establish that the declared item boundary ends with a body or semicolon.
fn terminator(
    tokens: &[CapturedTokenTree],
    kind: AuthoredItemKind,
) -> Result<usize, AuthoredItemReadRefusal> {
    let index = tokens
        .len()
        .checked_sub(1)
        .ok_or(refused(AuthoredItemReadIssue::ItemMissing, None))?;
    let token = token_at(tokens, index)?;
    let finished = token.punct() == Some(';')
        || token
            .group()
            .is_some_and(|(delimiter, _)| delimiter == CapturedDelimiter::Brace);
    if finished {
        Ok(index)
    } else {
        Err(refused(
            AuthoredItemReadIssue::ItemBoundaryUnfinished(kind),
            Some(token.span()),
        ))
    }
}

/// Select the body group without interpreting its contents.
fn body_index(
    tokens: &[CapturedTokenTree],
    name: Option<usize>,
    kind: AuthoredItemKind,
    terminator: usize,
) -> Option<usize> {
    if tokens
        .get(terminator)
        .is_some_and(|token| token.group().is_some())
    {
        return Some(terminator);
    }
    if kind != AuthoredItemKind::Structure {
        return None;
    }
    let start = name.and_then(|index| index.checked_add(1))?;
    tokens
        .iter()
        .enumerate()
        .skip(start)
        .take(terminator.saturating_sub(start))
        .find_map(|(index, token)| {
            token.group().and_then(|(delimiter, _)| {
                matches!(
                    delimiter,
                    CapturedDelimiter::Parenthesis | CapturedDelimiter::Brace
                )
                .then_some(index)
            })
        })
}

/// Find an immediately seated generic-parameter run and its matching close.
fn generic_range(tokens: &[CapturedTokenTree], start: usize, end: usize) -> Option<(usize, usize)> {
    if tokens.get(start).and_then(CapturedTokenTree::punct) != Some('<') {
        return None;
    }
    let mut depth = 0usize;
    for (index, token) in tokens
        .iter()
        .enumerate()
        .skip(start)
        .take(end.saturating_sub(start))
    {
        match token.punct() {
            Some('<') => depth = depth.saturating_add(1),
            Some('>') if !arrow_close(tokens, index) => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return index.checked_add(1).map(|after| (start, after));
                }
            }
            Some(_) | None => {}
        }
    }
    None
}

/// Whether this greater-than punctuation closes a thin arrow instead of a generic roster.
fn arrow_close(tokens: &[CapturedTokenTree], index: usize) -> bool {
    index
        .checked_sub(1)
        .and_then(|previous| tokens.get(previous))
        .and_then(CapturedTokenTree::joint_punct)
        == Some('-')
}

/// Borrow one established token range as a fragment.
fn fragment(
    tokens: &[CapturedTokenTree],
    start: usize,
    end: usize,
    enclosing: Option<SpanHandle>,
) -> Result<CapturedFragment<'_>, AuthoredItemReadRefusal> {
    tokens.get(start..end).map_or_else(
        || {
            Err(refused(
                AuthoredItemReadIssue::LensRangeContradiction,
                tokens.get(start).map(CapturedTokenTree::span),
            ))
        },
        |borrowed| Ok(CapturedFragment::over(borrowed, enclosing)),
    )
}

/// Read one required token coordinate established by the lens.
fn token_at(
    tokens: &[CapturedTokenTree],
    index: usize,
) -> Result<&CapturedTokenTree, AuthoredItemReadRefusal> {
    tokens.get(index).ok_or(refused(
        AuthoredItemReadIssue::LensRangeContradiction,
        tokens.last().map(CapturedTokenTree::span),
    ))
}

/// One ordinary or raw identifier spelling.
fn identifier(token: &CapturedTokenTree) -> Option<&str> {
    token.word().or_else(|| token.raw_identifier())
}

/// The word after one token coordinate.
fn next_word(tokens: &[CapturedTokenTree], index: usize) -> Option<&str> {
    index
        .checked_add(1)
        .and_then(|next| tokens.get(next))
        .and_then(CapturedTokenTree::word)
}

/// Find one exact word in an established half-open range.
fn word_index(tokens: &[CapturedTokenTree], word: &str, start: usize, end: usize) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start)
        .take(end.saturating_sub(start))
        .find_map(|(index, token)| (token.word() == Some(word)).then_some(index))
}

/// Find one exact word token in an established half-open range.
fn word_token<'tokens>(
    tokens: &'tokens [CapturedTokenTree],
    word: &str,
    start: usize,
    end: usize,
) -> Option<&'tokens CapturedTokenTree> {
    word_index(tokens, word, start, end).and_then(|index| tokens.get(index))
}

/// Construct one typed authored-item refusal.
const fn refused(issue: AuthoredItemReadIssue, at: Option<SpanHandle>) -> AuthoredItemReadRefusal {
    AuthoredItemReadRefusal { issue, at }
}

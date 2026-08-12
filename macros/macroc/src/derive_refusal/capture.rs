//! Reading one refusal-family declaration out of a typed token tree.
//!
//! # The grammar is AUTHORED, and it is small
//!
//! ```text
//! #[refusal(
//!     crate = <binding>,                     // optional; defaults to `threadpak`
//!     family = "<domain>.<family>",
//!     shape = <shape-word>,
//!     order(<Variant> = "<local-key>", ...),
//! )]
//! enum <FamilyName> { <Variant>, ... }
//! ```
//!
//! - `<binding>` is how the consumer names the machine on its own dependency
//!   list. It is optional because most consumers do not rename it, and it is
//!   captured because some do.
//! - `family` states the family's stable identity. The causes' identities are
//!   DERIVED from it and their local keys, under band 00's canonical key
//!   grammar, so no author writes a shared prefix out by hand fifteen times.
//! - `<shape-word>` is one of `single_cause`, `issue_collection`,
//!   `inseparable_pair`. The words map onto the machine's own [`FamilyShape`]
//!   roster; this module carries the spelling of the words and not a second
//!   roster of shapes.
//! - `order(...)` states the canonical selection order, required exactly when
//!   the shape is `single_cause` and admitted only then. Its order is the
//!   *selector's* order and need not match the order the variants are written in.
//! - Variants carry nothing but their own names, and a local key is a quoted
//!   text with no escape sequence in it.
//!
//! # Reading tokens, not text
//!
//! Everything below walks [`CapturedTokenTree`] values. Groups are already
//! groups, so nothing here re-discovers balance, and every refusal names the
//! exact token it was established at rather than a byte somewhere near it.
//!
//! # Refusals are honest about what they found
//!
//! A real enum whose variant carries a payload is not "not an enum". A struct is
//! not "not an enum". A generic enum is not "not an enum". Each of those is a
//! real declaration meeting a real limit of this grammar, and each gets a cause
//! that says which limit — because a caller told `NotAnEnum` about a perfectly
//! good enum goes looking for the wrong problem.

use super::types::{
    CapturedCause, CrateBinding, RefusalDeriveCapture, RefusalDeriveRefusal, RefusalDeriveSurface,
    SHAPE_WORD_INSEPARABLE_PAIR, SHAPE_WORD_ISSUE_COLLECTION, SHAPE_WORD_SINGLE_CAUSE,
};
use crate::plane::{
    CapturedTokenLimit, DeriveCauseLimit, ProjectionIdentity, ProjectionRole, ProjectionTranscript,
};
use crate::token::{
    CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle, TextCapture, TextReadCause,
    TextReadRefusal,
};
use threadpak::refusal::FamilyShape;
use threadpak::types::{Bounded, ConstLimit};

/// Capture one refusal-family declaration from a typed token tree.
///
/// # Errors
///
/// Returns [`RefusalDeriveRefusal`] carrying the established
/// [`RefusalDeriveCapture`] cause and the token it was established at.
pub fn captured(input: &CapturedInput) -> Result<RefusalDeriveSurface, RefusalDeriveRefusal> {
    let trees: Vec<&CapturedTokenTree> = input.trees().collect();
    let identity = ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::CapturedDeclaration,
        &input.canonical_bytes(),
        0,
    ));
    if trees.len() > CapturedTokenLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, first_span(&trees)));
    }
    let declared = read_enum(&trees)?;
    let attribute = read_attribute(&trees)?;
    let causes = read_causes(&attribute, &declared)?;
    let causes = Bounded::admitted_const(causes)
        .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, declared.body_span))?;
    Ok(RefusalDeriveSurface::assembled(
        declared.family_name,
        attribute.family_id,
        attribute.binding,
        attribute.shape,
        causes,
        identity,
    ))
}

/// Read one declared input from TEXT and capture it — the callable route, which
/// is what makes a diagnostic's reproduction route a real road.
///
/// # Errors
///
/// Returns [`RefusalDeriveRefusal`] for both the read and the capture: a text
/// that cannot be cut into tokens establishes a grammar cause exactly as a
/// declaration that cuts fine and says the wrong thing does.
pub fn captured_text(
    source: &str,
) -> Result<(TextCapture, RefusalDeriveSurface), RefusalDeriveRefusal> {
    let read = TextCapture::read(source).map_err(text_refusal)?;
    let surface = captured(read.input())?;
    Ok((read, surface))
}

/// The capture cause one text-read refusal establishes.
const fn text_refusal(refusal: TextReadRefusal) -> RefusalDeriveRefusal {
    let cause = match refusal.cause {
        TextReadCause::NotTerminated | TextReadCause::NotEscapeFree => {
            RefusalDeriveCapture::NotKeyed
        }
        TextReadCause::NotBalanced | TextReadCause::NotOpened => RefusalDeriveCapture::NotAnEnum,
        TextReadCause::Unbounded => RefusalDeriveCapture::Unbounded,
    };
    RefusalDeriveRefusal::established(cause, SpanHandle::at(0))
}

/// One established capture refusal at one token.
const fn refuse(cause: RefusalDeriveCapture, token: SpanHandle) -> RefusalDeriveRefusal {
    RefusalDeriveRefusal::established(cause, token)
}

/// The span of the first token, or handle zero where there is none.
fn first_span(trees: &[&CapturedTokenTree]) -> SpanHandle {
    trees.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// The item words this grammar recognizes as real Rust declarations that are
/// nevertheless not enums. A declaration spelling one of these gets
/// [`RefusalDeriveCapture::UnsupportedDeclarationForm`], never `NotAnEnum`.
const OTHER_ITEM_FORMS: [&str; 8] = [
    "struct", "union", "trait", "fn", "impl", "type", "const", "static",
];

/// The enum declaration as it was read.
struct DeclaredEnum {
    /// The declared family's Rust name.
    family_name: String,
    /// The variant names, in the order the body writes them.
    variants: Vec<String>,
    /// The token the body opens at.
    body_span: SpanHandle,
}

/// The attribute as it was read.
struct DeclaredAttribute {
    /// How the consumer names the machine.
    binding: CrateBinding,
    /// The declared family identity.
    family_id: String,
    /// The machine's body shape the declared word names.
    shape: FamilyShape,
    /// The token the shape word sits at.
    shape_span: SpanHandle,
    /// The declared order pairs, where an order clause was declared, and the
    /// token that clause opens at.
    order: Option<(Vec<CapturedCause>, SpanHandle)>,
}

/// Read the enum declaration: the item form, the keyword, the name, the profile
/// limits, and the body's variants.
fn read_enum(trees: &[&CapturedTokenTree]) -> Result<DeclaredEnum, RefusalDeriveRefusal> {
    let words: Vec<(usize, &str)> = trees
        .iter()
        .enumerate()
        .filter_map(|(index, tree)| tree.word().map(|word| (index, word)))
        .collect();
    let Some((enum_index, _)) = words.iter().copied().find(|(_, word)| *word == "enum") else {
        if let Some((index, _)) = words
            .iter()
            .copied()
            .find(|(_, word)| OTHER_ITEM_FORMS.contains(word))
        {
            return Err(refuse(
                RefusalDeriveCapture::UnsupportedDeclarationForm,
                span_at(trees, index),
            ));
        }
        return Err(refuse(RefusalDeriveCapture::NotAnEnum, first_span(trees)));
    };
    let name_index = enum_index.saturating_add(1);
    let family_name = trees
        .get(name_index)
        .and_then(|tree| tree.word())
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotNamed, span_at(trees, enum_index)))?
        .to_owned();

    // Everything between the name and the body is a form this profile does not
    // read: generic parameters, and a `where` clause.
    let body_index = (name_index.saturating_add(1)..trees.len())
        .find(|index| {
            trees
                .get(*index)
                .and_then(|tree| tree.group())
                .is_some_and(|(delimiter, _)| delimiter == CapturedDelimiter::Brace)
        })
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotBodied, span_at(trees, name_index)))?;
    if body_index > name_index.saturating_add(1) {
        return Err(refuse(
            RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            span_at(trees, name_index.saturating_add(1)),
        ));
    }

    let body_span = span_at(trees, body_index);
    let body = trees
        .get(body_index)
        .and_then(|tree| tree.group())
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotBodied, body_span))?;
    let body_trees: Vec<&CapturedTokenTree> = body.1.iter().collect();
    let variants = read_variants(&body_trees)?;
    if variants.is_empty() {
        return Err(refuse(RefusalDeriveCapture::NotInhabited, body_span));
    }
    if variants.len() > DeriveCauseLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, body_span));
    }
    Ok(DeclaredEnum {
        family_name,
        variants,
        body_span,
    })
}

/// Read the bare variant names out of one enum body.
fn read_variants(body: &[&CapturedTokenTree]) -> Result<Vec<String>, RefusalDeriveRefusal> {
    let mut variants: Vec<String> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            close_variant(&group, &mut variants)?;
            group.clear();
        } else {
            group.push(tree);
        }
    }
    close_variant(&group, &mut variants)?;
    Ok(variants)
}

/// Close one comma-separated group. Empty groups are trailing commas, a lone
/// word is a variant, and a word followed by anything is a variant carrying a
/// payload — which is a real variant this grammar does not admit, not a
/// non-enum.
fn close_variant(
    group: &[&CapturedTokenTree],
    variants: &mut Vec<String>,
) -> Result<(), RefusalDeriveRefusal> {
    let Some((first, rest)) = group.split_first() else {
        return Ok(());
    };
    let Some(word) = first.word() else {
        return Err(refuse(RefusalDeriveCapture::NotAnEnum, first.span()));
    };
    if let Some(extra) = rest.first() {
        return Err(refuse(
            RefusalDeriveCapture::UnsupportedVariantPayload,
            extra.span(),
        ));
    }
    variants.push(word.to_owned());
    Ok(())
}

/// The span of one token, or handle zero where there is none.
fn span_at(trees: &[&CapturedTokenTree], index: usize) -> SpanHandle {
    trees
        .get(index)
        .map_or(SpanHandle::at(0), |tree| tree.span())
}

/// Read the `#[refusal(...)]` attribute: the crate binding, the family
/// identity, the shape word, and the order clause.
fn read_attribute(trees: &[&CapturedTokenTree]) -> Result<DeclaredAttribute, RefusalDeriveRefusal> {
    let body = refusal_attribute_body(trees)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotFamilyDeclared, first_span(trees)))?;
    let inner: &[&CapturedTokenTree] = &body;

    let binding = assigned_word(inner, "crate")
        .map_or_else(CrateBinding::default_binding, |(word, _)| {
            CrateBinding::declared(word)
        });

    let (family_id, family_span) = assigned_text(inner, "family")
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotFamilyDeclared, first_span(trees)))?;
    if !is_family_grammatical(family_id) {
        return Err(refuse(
            RefusalDeriveCapture::NotFamilyGrammatical,
            family_span,
        ));
    }

    let (word, shape_span) = assigned_word(inner, "shape")
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotShapeDeclared, first_span(trees)))?;
    let shape = admitted_shape(word)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAnAdmittedShape, shape_span))?;

    let order = match order_clause(inner) {
        Some((clause, span)) => Some((read_order_pairs(&clause, span)?, span)),
        None => None,
    };

    Ok(DeclaredAttribute {
        binding,
        family_id: family_id.to_owned(),
        shape,
        shape_span,
        order,
    })
}

/// The token trees inside the `#[refusal(...)]` attribute, where one is
/// declared.
fn refusal_attribute_body<'trees>(
    trees: &[&'trees CapturedTokenTree],
) -> Option<Vec<&'trees CapturedTokenTree>> {
    for (index, tree) in trees.iter().enumerate() {
        let Some((delimiter, inner)) = tree.group() else {
            continue;
        };
        if delimiter != CapturedDelimiter::Bracket
            || index
                .checked_sub(1)
                .and_then(|hash| trees.get(hash))
                .and_then(|hash| hash.punct())
                != Some('#')
        {
            continue;
        }
        let bracketed: Vec<&CapturedTokenTree> = inner.iter().collect();
        let named = bracketed
            .first()
            .and_then(|tree| tree.word())
            .is_some_and(|word| word == "refusal");
        if !named {
            continue;
        }
        if let Some((CapturedDelimiter::Parenthesis, body)) =
            bracketed.get(1).and_then(|tree| tree.group())
        {
            return Some(body.iter().collect());
        }
    }
    None
}

/// The word assigned to one key inside the attribute body, with its token.
fn assigned_word<'trees>(
    body: &[&'trees CapturedTokenTree],
    key: &str,
) -> Option<(&'trees str, SpanHandle)> {
    let index = assignment_index(body, key)?;
    let value = body.get(index.saturating_add(2))?;
    value.word().map(|word| (word, value.span()))
}

/// The text assigned to one key inside the attribute body, with its token.
fn assigned_text<'trees>(
    body: &[&'trees CapturedTokenTree],
    key: &str,
) -> Option<(&'trees str, SpanHandle)> {
    let index = assignment_index(body, key)?;
    let value = body.get(index.saturating_add(2))?;
    value.text().map(|text| (text, value.span()))
}

/// The index of one `key =` assignment inside the attribute body.
fn assignment_index(body: &[&CapturedTokenTree], key: &str) -> Option<usize> {
    (0..body.len()).find(|index| {
        body.get(*index).and_then(|tree| tree.word()) == Some(key)
            && body
                .get(index.saturating_add(1))
                .and_then(|tree| tree.punct())
                == Some('=')
    })
}

/// The tokens inside the `order(...)` clause, where one is declared, with the
/// token it opens at.
fn order_clause<'trees>(
    body: &[&'trees CapturedTokenTree],
) -> Option<(Vec<&'trees CapturedTokenTree>, SpanHandle)> {
    for (index, tree) in body.iter().enumerate() {
        if tree.word() != Some("order") {
            continue;
        }
        let group = body.get(index.saturating_add(1))?;
        if let Some((CapturedDelimiter::Parenthesis, inner)) = group.group() {
            return Some((inner.iter().collect(), group.span()));
        }
    }
    None
}

/// The `Variant = "local-key"` pairs inside an order clause.
fn read_order_pairs(
    clause: &[&CapturedTokenTree],
    span: SpanHandle,
) -> Result<Vec<CapturedCause>, RefusalDeriveRefusal> {
    let mut pairs: Vec<CapturedCause> = Vec::new();
    let mut index = 0usize;
    while index < clause.len() {
        let spelling = clause
            .get(index)
            .and_then(|tree| tree.word())
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotCovered, span))?;
        if clause
            .get(index.saturating_add(1))
            .and_then(|tree| tree.punct())
            != Some('=')
        {
            return Err(refuse(RefusalDeriveCapture::NotCovered, span));
        }
        let value = clause
            .get(index.saturating_add(2))
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotCovered, span))?;
        let local_key = value
            .text()
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotKeyed, value.span()))?;
        if !is_local_key_grammatical(local_key) {
            return Err(refuse(RefusalDeriveCapture::NotKeyed, value.span()));
        }
        pairs.push(CapturedCause::read(spelling, local_key));
        index = index.saturating_add(3);
        if index < clause.len() {
            if clause.get(index).and_then(|tree| tree.punct()) != Some(',') {
                return Err(refuse(RefusalDeriveCapture::NotCovered, span));
            }
            index = index.saturating_add(1);
        }
    }
    Ok(pairs)
}

/// Read the declared causes: the order clause where the shape carries one, and
/// then coverage and distinctness against the body.
fn read_causes(
    attribute: &DeclaredAttribute,
    declared: &DeclaredEnum,
) -> Result<Vec<CapturedCause>, RefusalDeriveRefusal> {
    let causes = match (attribute.shape, attribute.order.as_ref()) {
        (FamilyShape::SingleCause, None) => {
            return Err(refuse(
                RefusalDeriveCapture::NotOrderDeclared,
                attribute.shape_span,
            ));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, Some((_, span))) => {
            return Err(refuse(RefusalDeriveCapture::NotOrderAdmitted, *span));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, None) => {
            return Ok(Vec::new());
        }
        (FamilyShape::SingleCause, Some((pairs, _))) => pairs.clone(),
    };
    if !covers(&causes, &declared.variants) {
        return Err(refuse(RefusalDeriveCapture::NotCovered, declared.body_span));
    }
    if !distinct(&causes) {
        return Err(refuse(
            RefusalDeriveCapture::NotDistinct,
            declared.body_span,
        ));
    }
    if u16::try_from(causes.len()).is_err() {
        return Err(refuse(RefusalDeriveCapture::Unbounded, declared.body_span));
    }
    Ok(causes)
}

/// The machine's body shape one authored word names.
fn admitted_shape(word: &str) -> Option<FamilyShape> {
    match word {
        SHAPE_WORD_SINGLE_CAUSE => Some(FamilyShape::SingleCause),
        SHAPE_WORD_ISSUE_COLLECTION => Some(FamilyShape::IssueCollection),
        SHAPE_WORD_INSEPARABLE_PAIR => Some(FamilyShape::InseparablePair),
        _ => None,
    }
}

/// Whether one text is a single lowercase kebab-case segment.
fn is_local_key_grammatical(key: &str) -> bool {
    !key.is_empty()
        && !key.starts_with('-')
        && !key.ends_with('-')
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Whether one text is two lowercase kebab-case segments joined by a dot.
fn is_family_grammatical(identity: &str) -> bool {
    let mut segments = identity.split('.');
    let domain = segments.next();
    let family = segments.next();
    let extra = segments.next();
    match (domain, family, extra) {
        (Some(domain), Some(family), None) => {
            is_local_key_grammatical(domain) && is_local_key_grammatical(family)
        }
        _ => false,
    }
}

/// Whether the ordered causes and the body variants name the same set, each
/// exactly once.
fn covers(causes: &[CapturedCause], variants: &[String]) -> bool {
    causes.len() == variants.len()
        && causes.iter().all(|cause| {
            variants
                .iter()
                .filter(|variant| variant.as_str() == cause.spelling())
                .count()
                == 1
        })
        && variants.iter().all(|variant| {
            causes
                .iter()
                .filter(|cause| cause.spelling() == variant.as_str())
                .count()
                == 1
        })
}

/// Whether every declared local key is distinct — the local-uniqueness proof the
/// derive owes. Family uniqueness is a different question and is owed to the
/// composition root, which band 00's key grammar says out loud.
fn distinct(causes: &[CapturedCause]) -> bool {
    causes.iter().enumerate().all(|(index, cause)| {
        causes
            .iter()
            .skip(index.saturating_add(1))
            .all(|other| other.local_key() != cause.local_key())
    })
}

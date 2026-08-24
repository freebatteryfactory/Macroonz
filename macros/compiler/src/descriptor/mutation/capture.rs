//! Reading one authored mutation-policy declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! #[<helper>(
//!     module = <module name>,
//!     refusal = <refusal type name>,
//!     support = <exported name>,
//!     family = named("<namespace>", "<stem>"),
//!     point = named("<namespace>", "<stem>"),
//!     fact = named("<namespace>", "<stem>"),
//!
//!     map named("<namespace>", "<fact>") = named("<namespace>", "<claim>"),
//!     permit named("<namespace>", "<claim>") = ["<family slug>", ...],
//! )]
//! ```
//!
//! `support` is the one optional clause: a declaration whose carrier another helper already addressed states none.
//!
//! # Names, not a sealed roster
//!
//! An owner fact and an operator family are the CONSUMER's declarations. This reading resolves neither against a roster it owns, because a producer that knew which facts exist would be a producer that knew what the consumer's declaration means.
//! What it checks is shape: a mapping is one fact and one claim, a permission is one claim and a non-empty roster of family slugs, and nothing states one fact or one claim twice.
//!
//! # What the helper cannot state
//!
//! The site's own material — the type its alternatives are values of, the production the unchanged declaration answers with, the operation bytes, and the alternatives themselves — is computed by the door that captured the declaration this helper sits on.
//! [`Declaration::completed`] is where the two meet.

use super::{
    ALTERNATIVE_LIMIT, Address, Alternative, DECLARED_ORDER_FAMILY, Declaration, FactMapping,
    FamilySlug, MutationCaptureError, Permission, Policy, Surface,
};
use crate::bounded::Overflow;
use crate::descriptor::{
    CaptureCause, DeclarationError, Grammar, ModuleName, Name, Seat, SupportName, TypeName,
};
use crate::identity::{encode_bytes, encode_length};
use crate::token::{
    CapturedDelimiter, CapturedTokenTree, GeneratedDelimiter, GeneratedToken, SpanHandle,
};

/// The clause naming the module the surface is written as.
const MODULE: &str = "module";

/// The clause naming the refusal type the surface declares.
const REFUSAL: &str = "refusal";

/// The clause naming the exported support address, where this declaration owns it.
const SUPPORT: &str = "support";

/// The clause naming the evaluation family.
const FAMILY: &str = "family";

/// The clause naming the point the site is discovered at.
const POINT: &str = "point";

/// The clause naming the owner fact the site stands on.
const FACT: &str = "fact";

/// The word one owner-fact mapping opens with.
const MAP: &str = "map";

/// The word one permission opens with.
const PERMIT: &str = "permit";

/// The road every namespaced reference in this grammar is spelled by.
const NAMED: &str = "named";

/// The clause keys this grammar declares.
const DECLARABLE: [&str; 6] = [MODULE, REFUSAL, SUPPORT, FAMILY, POINT, FACT];

/// Read one mutation declaration out of the helper attribute's body.
///
/// # Errors
///
/// Returns [`MutationCaptureError`] where the tokens do not say a mutation declaration, and where the values they say are not a lawful declaration — each at the token the clause it was established at sits at.
pub fn captured(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<Declaration, MutationCaptureError> {
    let clauses = clauses(grammar, body)?;
    let address = Address {
        module: ModuleName::declared(identifier(grammar, &clauses, MODULE, at)?)
            .map_err(|refusal| carried(grammar, refusal, at))?,
        support: optional_support(grammar, &clauses)?,
        refusal: TypeName::declared(identifier(grammar, &clauses, REFUSAL, at)?)
            .map_err(|refusal| carried(grammar, refusal, at))?,
    };
    let family = named_reference(grammar, &clauses, FAMILY, at)?;
    let point = named_reference(grammar, &clauses, POINT, at)?;
    let fact = named_reference(grammar, &clauses, FACT, at)?;

    let mut mappings: Vec<FactMapping> = Vec::new();
    let mut permissions: Vec<Permission> = Vec::new();
    for clause in &clauses {
        match *clause {
            Clause::Mapping {
                fact: ref mapped,
                ref claim,
            } => mappings.push(FactMapping {
                fact: mapped.clone(),
                claim: claim.clone(),
            }),
            Clause::Permission {
                ref claim,
                ref families,
                at: site,
            } => permissions.push(
                Permission::permitted(claim.clone(), families.clone())
                    .map_err(|refusal| carried(grammar, refusal, site))?,
            ),
            Clause::Assigned { .. } => {}
        }
    }
    let policy = Policy::declared(family, mappings, permissions)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    Ok(Declaration::captured(address, policy, point, fact))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> MutationCaptureError {
    MutationCaptureError::grammar_refused(grammar, cause, at)
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(
    grammar: Grammar,
    refusal: DeclarationError,
    at: SpanHandle,
) -> MutationCaptureError {
    MutationCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One clause of a mutation declaration's body, as the split read it.
enum Clause<'trees> {
    /// `<key> = <value tokens>`.
    Assigned {
        /// The key the clause names.
        key: &'trees str,
        /// The tokens the value is spelled from.
        value: Vec<&'trees CapturedTokenTree>,
        /// The token the key sits at.
        at: SpanHandle,
    },
    /// `map named(…) = named(…)`.
    Mapping {
        /// The owner fact pressure is applied to.
        fact: Name,
        /// The claim that permits it.
        claim: Name,
    },
    /// `permit named(…) = [<slug>, …]`.
    Permission {
        /// The claim whose permission is stated.
        claim: Name,
        /// The operator families it permits.
        families: Vec<FamilySlug>,
        /// The token the clause opens at.
        at: SpanHandle,
    },
}

/// Cut one declaration body into its comma-separated clauses.
fn clauses<'trees>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, MutationCaptureError> {
    let mut read: Vec<Clause<'trees>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            read.push(clause(grammar, &group)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        read.push(clause(grammar, &group)?);
    }
    distinct(grammar, &read)?;
    Ok(read)
}

/// Read one comma-separated group as the clause its opening word declares.
fn clause<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
) -> Result<Clause<'trees>, MutationCaptureError> {
    let Some((head, rest)) = group.split_first() else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            SpanHandle::at(0),
        ));
    };
    match head.word() {
        Some(MAP) => mapping(grammar, rest, head.span()),
        Some(PERMIT) => permission(grammar, rest, head.span()),
        Some(key) if DECLARABLE.contains(&key) => assignment(grammar, key, rest, head.span()),
        Some(_) => Err(refused(
            grammar,
            CaptureCause::ClauseUndeclared,
            head.span(),
        )),
        None => Err(refused(grammar, CaptureCause::ClauseUnread, head.span())),
    }
}

/// Read one `<key> = <value>` assignment.
fn assignment<'trees>(
    grammar: Grammar,
    key: &'trees str,
    rest: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationCaptureError> {
    let Some((assigned_by, value)) = rest.split_first() else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    if assigned_by.punct() != Some('=') || value.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            assigned_by.span(),
        ));
    }
    Ok(Clause::Assigned {
        key,
        value: value.to_vec(),
        at,
    })
}

/// Read one `map named(…) = named(…)` clause off the trees after its opening word.
fn mapping<'trees>(
    grammar: Grammar,
    rest: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationCaptureError> {
    let [
        fact_word,
        fact_arguments,
        assigned_by,
        claim_word,
        claim_arguments,
    ] = rest
    else {
        return Err(refused(grammar, CaptureCause::MappingUnread, at));
    };
    if assigned_by.punct() != Some('=') {
        return Err(refused(
            grammar,
            CaptureCause::MappingUnread,
            assigned_by.span(),
        ));
    }
    Ok(Clause::Mapping {
        fact: named_value(grammar, fact_word, fact_arguments)?,
        claim: named_value(grammar, claim_word, claim_arguments)?,
    })
}

/// Read one `permit named(…) = [<slug>, …]` clause off the trees after its opening word.
fn permission<'trees>(
    grammar: Grammar,
    rest: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationCaptureError> {
    let [word, arguments, assigned_by, bracketed] = rest else {
        return Err(refused(grammar, CaptureCause::PermissionUnread, at));
    };
    if assigned_by.punct() != Some('=') {
        return Err(refused(
            grammar,
            CaptureCause::PermissionUnread,
            assigned_by.span(),
        ));
    }
    let claim = named_value(grammar, word, arguments)?;
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(refused(
            grammar,
            CaptureCause::PermissionUnread,
            bracketed.span(),
        ));
    };
    let mut families: Vec<FamilySlug> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            match group.as_slice() {
                [] => {
                    return Err(refused(
                        grammar,
                        CaptureCause::SeparatorDangling,
                        tree.span(),
                    ));
                }
                [only] => families.push(family(grammar, only)?),
                [first, ..] => {
                    return Err(refused(
                        grammar,
                        CaptureCause::PermissionUnread,
                        first.span(),
                    ));
                }
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    match group.as_slice() {
        [] => {}
        [only] => families.push(family(grammar, only)?),
        [first, ..] => {
            return Err(refused(
                grammar,
                CaptureCause::PermissionUnread,
                first.span(),
            ));
        }
    }
    Ok(Clause::Permission {
        claim,
        families,
        at,
    })
}

/// One operator-family slug, read off its own token.
fn family(grammar: Grammar, tree: &CapturedTokenTree) -> Result<FamilySlug, MutationCaptureError> {
    let Some(slug) = tree.text() else {
        return Err(refused(
            grammar,
            CaptureCause::PermissionUnread,
            tree.span(),
        ));
    };
    FamilySlug::declared(slug).map_err(|refusal| carried(grammar, refusal, tree.span()))
}

/// Refuse where one assigned clause key is stated twice.
///
/// Mappings and permissions are not checked here: one owner fact mapped twice and two permissions over one claim are facts about the whole POLICY, stated once where the policy is built.
fn distinct(grammar: Grammar, clauses: &[Clause<'_>]) -> Result<(), MutationCaptureError> {
    for (position, clause) in clauses.iter().enumerate() {
        let Clause::Assigned { key, at, .. } = *clause else {
            continue;
        };
        let earlier = clauses.iter().take(position).any(|other| match *other {
            Clause::Assigned { key: seen, .. } => seen == key,
            Clause::Mapping { .. } | Clause::Permission { .. } => false,
        });
        if earlier {
            return Err(refused(grammar, CaptureCause::ClauseDoubled, at));
        }
    }
    Ok(())
}

/// The value tokens one assigned clause carries, and the token its key sits at.
fn assigned<'trees, 'clauses>(
    clauses: &'clauses [Clause<'trees>],
    key: &str,
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| match *clause {
        Clause::Assigned {
            key: named,
            ref value,
            at,
        } if named == key => Some((value.as_slice(), at)),
        Clause::Assigned { .. } | Clause::Mapping { .. } | Clause::Permission { .. } => None,
    })
}

/// One identifier a clause assigns.
fn identifier<'trees>(
    grammar: Grammar,
    clauses: &[Clause<'trees>],
    key: &str,
    at: SpanHandle,
) -> Result<&'trees str, MutationCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    only.word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))
}

/// The exported support address, where this declaration owns one.
fn optional_support(
    grammar: Grammar,
    clauses: &[Clause<'_>],
) -> Result<Option<SupportName>, MutationCaptureError> {
    let Some((value, at)) = assigned(clauses, SUPPORT) else {
        return Ok(None);
    };
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    let spelling = only
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))?;
    SupportName::declared(spelling)
        .map(Some)
        .map_err(|refusal| carried(grammar, refusal, only.span()))
}

/// One `named(<namespace>, <stem>)` reference a clause assigns.
fn named_reference(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<Name, MutationCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [word, arguments] = value else {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, clause));
    };
    named_value(grammar, word, arguments)
}

/// One `named(<namespace>, <stem>)` reference, read off the word and the group that spell it.
fn named_value(
    grammar: Grammar,
    word: &CapturedTokenTree,
    arguments: &CapturedTokenTree,
) -> Result<Name, MutationCaptureError> {
    if word.word() != Some(NAMED) {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, word.span()));
    }
    let Some((CapturedDelimiter::Parenthesis, inner)) = arguments.group() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    let parts: Vec<&CapturedTokenTree> = inner.iter().collect();
    let [namespace, separator, stem] = parts.as_slice() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    if separator.punct() != Some(',') {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            separator.span(),
        ));
    }
    let (Some(owner), Some(spelling)) = (namespace.text(), stem.text()) else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    Name::named(owner, spelling).map_err(|refusal| carried(grammar, refusal, arguments.span()))
}

/// The version byte the declared-order operation encoding opens with.
const ORDER_OPERATION_VERSION: u32 = 1;

/// The label the declared-order operation encoding carries after its version.
const ORDER_OPERATION_LABEL: &[u8] = b"declared-variant-order";

/// Complete one captured declaration with the site material this door computes from the item the helper sits on.
///
/// # What the door reads, and what it refuses to
///
/// The item must be an `enum`, and its variant list — in the order the author wrote it — is the declared order the site presses.
/// The reading is structural: variant NAMES and their order, never fields, discriminants, or what any variant means.
///
/// The unchanged operation is the authored order itself.
/// Each alternative is one adjacent transposition of it, under the [`DECLARED_ORDER_FAMILY`] operator family, so what the pressure asks is exactly: would any witness notice if two neighbors of this declared order traded places?
///
/// The order type is `[&'static str; N]` and every value of it is the spellings themselves, so the rendered module is pure data and resolves in a test target that has never seen the declaring crate's module tree.
///
/// # Errors
///
/// Returns [`MutationCaptureError`] where the item states no enum body ([`CaptureCause::ItemUnread`]), where the order has fewer than two members to transpose ([`CaptureCause::OrderUnpressable`]), and where the values it states are not a lawful site — each at the token it was established at.
pub fn completed(
    declaration: Declaration,
    item: &[&CapturedTokenTree],
    grammar: Grammar,
) -> Result<Surface, MutationCaptureError> {
    let (at, order) = declared_order(grammar, item)?;
    if order.len() > ALTERNATIVE_LIMIT {
        return Err(carried(
            grammar,
            DeclarationError::unbounded(Seat::Alternative, ALTERNATIVE_LIMIT, order.len()),
            at,
        ));
    }
    let family = declaration.policy().family().clone();
    let unchanged = order_operation(&family, &order);
    let production = spelling_array(&order).map_err(|overflow| overflown(grammar, overflow, at))?;
    let order_type =
        order_type(order.len()).map_err(|overflow| overflown(grammar, overflow, at))?;

    let mut alternatives: Vec<Alternative> = Vec::new();
    for left in 0..order.len().saturating_sub(1) {
        let mut transposed = order.clone();
        transposed.swap(left, left.saturating_add(1));
        let operation = order_operation(&family, &transposed);
        let meaning =
            spelling_array(&transposed).map_err(|overflow| overflown(grammar, overflow, at))?;
        let slug = FamilySlug::declared(DECLARED_ORDER_FAMILY)
            .map_err(|refusal| carried(grammar, refusal, at))?;
        alternatives.push(
            Alternative::stated(slug, operation, meaning)
                .map_err(|refusal| carried(grammar, refusal, at))?,
        );
    }
    declaration
        .completed(order_type, production, unchanged, alternatives)
        .map_err(|refusal| carried(grammar, refusal, at))
}

/// One token-magnitude overflow, carried as the vocabulary refusal about the site's alternatives.
fn overflown(grammar: Grammar, overflow: Overflow, at: SpanHandle) -> MutationCaptureError {
    carried(
        grammar,
        DeclarationError::unbounded(Seat::Alternative, overflow.capacity, overflow.offered),
        at,
    )
}

/// The declared order the item states: the enum's variant names, in authored order, and the token the body sits at.
fn declared_order<'trees>(
    grammar: Grammar,
    item: &[&'trees CapturedTokenTree],
) -> Result<(SpanHandle, Vec<&'trees str>), MutationCaptureError> {
    let fallback = item
        .first()
        .map_or_else(|| SpanHandle::at(0), |tree| tree.span());
    let opened = item
        .iter()
        .position(|tree| tree.word() == Some("enum"))
        .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, fallback))?;
    let body = item
        .iter()
        .skip(opened)
        .find_map(|tree| match tree.group() {
            Some((CapturedDelimiter::Brace, inner)) => Some((tree.span(), inner)),
            Some(_) | None => None,
        })
        .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, fallback))?;
    let (at, inner) = body;
    let mut order: Vec<&str> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            order.push(variant(grammar, &group, at)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        order.push(variant(grammar, &group, at)?);
    }
    if order.len() < 2 {
        return Err(refused(grammar, CaptureCause::OrderUnpressable, at));
    }
    Ok((at, order))
}

/// One variant's name, read past whatever attributes stand before it.
fn variant<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<&'trees str, MutationCaptureError> {
    let mut trees = group.iter();
    while let Some(tree) = trees.next() {
        if tree.punct() == Some('#') {
            let _attribute_body = trees.next();
            continue;
        }
        return tree
            .word()
            .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, tree.span()));
    }
    Err(refused(grammar, CaptureCause::ItemUnread, at))
}

/// The semantic bytes one order is identified by: a version, a label, the evaluation family, and the members in sequence.
fn order_operation(family: &Name, order: &[&str]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&ORDER_OPERATION_VERSION.to_be_bytes());
    encode_bytes(ORDER_OPERATION_LABEL, &mut bytes);
    encode_bytes(family.namespace().as_bytes(), &mut bytes);
    encode_bytes(family.stem().as_bytes(), &mut bytes);
    encode_length(order.len(), &mut bytes);
    for spelling in order {
        encode_bytes(spelling.as_bytes(), &mut bytes);
    }
    bytes
}

/// The type every rendered order value inhabits: an array of the declared width over static text.
fn order_type(count: usize) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        vec![
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("static"),
            GeneratedToken::word("str"),
            GeneratedToken::alone(';'),
            GeneratedToken::number(u64::try_from(count).unwrap_or(u64::MAX)),
        ],
    )?])
}

/// One order as the array literal that spells it.
fn spelling_array(order: &[&str]) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut members: Vec<GeneratedToken> = Vec::new();
    for spelling in order {
        members.push(GeneratedToken::text(spelling));
        members.push(GeneratedToken::alone(','));
    }
    Ok(vec![GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        members,
    )?])
}

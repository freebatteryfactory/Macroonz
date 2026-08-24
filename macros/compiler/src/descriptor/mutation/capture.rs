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
    Address, Declaration, FactMapping, FamilySlug, MutationCaptureError, Permission, Policy,
};
use crate::descriptor::{
    CaptureCause, DeclarationError, Grammar, ModuleName, Name, SupportName, TypeName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

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
            if !group.is_empty() {
                read.push(clause(grammar, &group)?);
            }
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
    for tree in inner {
        if tree.punct() == Some(',') {
            continue;
        }
        let Some(slug) = tree.text() else {
            return Err(refused(
                grammar,
                CaptureCause::PermissionUnread,
                tree.span(),
            ));
        };
        families.push(
            FamilySlug::declared(slug).map_err(|refusal| carried(grammar, refusal, tree.span()))?,
        );
    }
    Ok(Clause::Permission {
        claim,
        families,
        at,
    })
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

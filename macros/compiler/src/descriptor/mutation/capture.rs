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
use crate::descriptor::clause::{
    Clause, assigned, declaration_clauses, identifier, named_reference, named_value,
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
    let clauses = declaration_clauses(grammar, body, &DECLARABLE, nested_clause, refused)?;
    let address = Address {
        module: ModuleName::declared(identifier(grammar, &clauses, MODULE, at, refused)?)
            .map_err(|refusal| carried(grammar, refusal, at))?,
        support: optional_support(grammar, &clauses)?,
        refusal: TypeName::declared(identifier(grammar, &clauses, REFUSAL, at, refused)?)
            .map_err(|refusal| carried(grammar, refusal, at))?,
    };
    let family = named_reference(grammar, &clauses, FAMILY, at, refused, carried)?;
    let point = named_reference(grammar, &clauses, POINT, at, refused, carried)?;
    let fact = named_reference(grammar, &clauses, FACT, at, refused, carried)?;

    let mut mappings: Vec<FactMapping> = Vec::new();
    let mut permissions: Vec<Permission> = Vec::new();
    for clause in &clauses {
        let Some(nested) = clause.nested_value() else {
            continue;
        };
        match nested {
            NestedClause::Mapping {
                fact: mapped,
                claim,
            } => mappings.push(FactMapping {
                fact: mapped.clone(),
                claim: claim.clone(),
            }),
            NestedClause::Permission {
                claim,
                families,
                at: site,
            } => permissions.push(
                Permission::permitted(claim.clone(), families.clone())
                    .map_err(|refusal| carried(grammar, refusal, *site))?,
            ),
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

/// One mutation-owned nested clause.
enum NestedClause {
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

/// Read one mutation-owned nested clause where the group states one.
fn nested_clause(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<Option<NestedClause>, MutationCaptureError> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(None);
    };
    match head.word() {
        Some(MAP) => mapping(grammar, rest, head.span()).map(Some),
        Some(PERMIT) => permission(grammar, rest, head.span()).map(Some),
        Some(key) if !DECLARABLE.contains(&key) => Err(refused(
            grammar,
            CaptureCause::ClauseUndeclared,
            head.span(),
        )),
        Some(_) | None => Ok(None),
    }
}

/// Read one `map named(…) = named(…)` clause off the trees after its opening word.
fn mapping(
    grammar: Grammar,
    rest: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<NestedClause, MutationCaptureError> {
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
    Ok(NestedClause::Mapping {
        fact: named_value(
            grammar,
            &[*fact_word, *fact_arguments],
            at,
            refused,
            carried,
        )?,
        claim: named_value(
            grammar,
            &[*claim_word, *claim_arguments],
            at,
            refused,
            carried,
        )?,
    })
}

/// Read one `permit named(…) = [<slug>, …]` clause off the trees after its opening word.
fn permission(
    grammar: Grammar,
    rest: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<NestedClause, MutationCaptureError> {
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
    let claim = named_value(grammar, &[*word, *arguments], at, refused, carried)?;
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
            families.push(separated_family(grammar, &group, tree.span())?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if let Some(last) = trailing_family(grammar, &group)? {
        families.push(last);
    }
    Ok(NestedClause::Permission {
        claim,
        families,
        at,
    })
}

/// One family group ending at a separator, which therefore cannot be empty.
fn separated_family(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<FamilySlug, MutationCaptureError> {
    match group {
        [] => Err(refused(grammar, CaptureCause::SeparatorDangling, at)),
        [only] => family(grammar, only),
        [first, ..] => Err(refused(
            grammar,
            CaptureCause::PermissionUnread,
            first.span(),
        )),
    }
}

/// The final family group, where an empty group means an ordinary trailing separator or an empty roster.
fn trailing_family(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<Option<FamilySlug>, MutationCaptureError> {
    match group {
        [] => Ok(None),
        [only] => family(grammar, only).map(Some),
        [first, ..] => Err(refused(
            grammar,
            CaptureCause::PermissionUnread,
            first.span(),
        )),
    }
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

/// The exported support address, where this declaration owns one.
fn optional_support(
    grammar: Grammar,
    clauses: &[Clause<'_, NestedClause>],
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

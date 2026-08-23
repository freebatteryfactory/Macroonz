//! Reading one closed `threadpak_mutations` helper body.

use super::{
    GeneratedMutationFamily, MutationDeclaration, MutationDeclarationCause,
    MutationDeclarationRefusal, MutationModuleName, MutationOwnerFact,
    OperatorPermissionDeclaration, OwnerClaimDeclaration,
};
use crate::test_descriptor::{ShellDeclarationRefusal, SupportMacroName, WallName};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

/// The helper attribute one mutation declaration is written in.
pub const MUTATION_ATTRIBUTE: &str = "threadpak_mutations";

const MODULE_CLAUSE: &str = "module";
const SUPPORT_CLAUSE: &str = "support";
const FAMILY_CLAUSE: &str = "family";
const MAP_WORD: &str = "map";
const PERMIT_WORD: &str = "permit";
const NAMED_ROAD: &str = "named";

enum Clause<'trees> {
    Assigned {
        key: &'trees str,
        value: Vec<&'trees CapturedTokenTree>,
        at: SpanHandle,
    },
    Mapping {
        fact: MutationOwnerFact,
        claim: WallName,
        at: SpanHandle,
    },
    Permission {
        claim: WallName,
        families: Vec<GeneratedMutationFamily>,
        at: SpanHandle,
    },
}

/// Read one mutation declaration from a helper body.
///
/// # Errors
///
/// Returns the exact grammar or carrier-vocabulary refusal at the token where
/// the body first disagrees with the closed helper grammar.
pub fn captured_mutations(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<MutationDeclaration, MutationDeclarationRefusal> {
    let clauses = clauses(body)?;
    let module_site = assigned(&clauses, MODULE_CLAUSE)
        .ok_or_else(|| grammar(MutationDeclarationCause::NotCovered, at))?;
    let module = MutationModuleName::declared(identifier_value(module_site.0, module_site.1)?)
        .map_err(|refusal| carrier(refusal, module_site.1))?;
    let support = assigned(&clauses, SUPPORT_CLAUSE)
        .map(|(value, site)| {
            SupportMacroName::declared(identifier_value(value, site)?)
                .map(|name| (name, site))
                .map_err(|refusal| carrier(refusal, site))
        })
        .transpose()?;
    let family = named_assignment(&clauses, FAMILY_CLAUSE, at)?;

    let mut mappings = Vec::new();
    let mut permissions = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Mapping { fact, claim, .. } => {
                mappings.push(OwnerClaimDeclaration::mapped(fact, claim));
            }
            Clause::Permission {
                claim, families, ..
            } => {
                let mut families = families.into_iter();
                let Some(first) = families.next() else {
                    return Err(grammar(MutationDeclarationCause::EmptyOperatorFamilies, at));
                };
                permissions.push(OperatorPermissionDeclaration::permitted(
                    claim,
                    first,
                    families.collect(),
                ));
            }
            Clause::Assigned { .. } => {}
        }
    }
    Ok(MutationDeclaration::captured(
        at,
        module,
        support,
        family,
        mappings,
        permissions,
    ))
}

const fn grammar(cause: MutationDeclarationCause, at: SpanHandle) -> MutationDeclarationRefusal {
    MutationDeclarationRefusal::Grammar { cause, at }
}

const fn carrier(refusal: ShellDeclarationRefusal, at: SpanHandle) -> MutationDeclarationRefusal {
    MutationDeclarationRefusal::Carrier { refusal, at }
}

fn clauses<'trees>(
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, MutationDeclarationRefusal> {
    let mut read = Vec::new();
    let mut group = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(grammar(MutationDeclarationCause::NotAClause, tree.span()));
            }
            read.push(clause(&group)?);
            group.clear();
        } else {
            group.push(*tree);
        }
    }
    if !group.is_empty() {
        read.push(clause(&group)?);
    }
    distinct(&read)?;
    Ok(read)
}

fn clause<'trees>(
    group: &[&'trees CapturedTokenTree],
) -> Result<Clause<'trees>, MutationDeclarationRefusal> {
    let Some((head, rest)) = group.split_first() else {
        return Err(grammar(
            MutationDeclarationCause::NotAClause,
            SpanHandle::at(0),
        ));
    };
    match head.word() {
        Some(MAP_WORD) => mapping(rest, head.span()),
        Some(PERMIT_WORD) => permission(rest, head.span()),
        Some(key @ (MODULE_CLAUSE | SUPPORT_CLAUSE | FAMILY_CLAUSE)) => {
            assignment(key, rest, head.span())
        }
        Some(_) => Err(grammar(
            MutationDeclarationCause::NotADeclarableClause,
            head.span(),
        )),
        None => Err(grammar(MutationDeclarationCause::NotAClause, head.span())),
    }
}

fn assignment<'trees>(
    key: &'trees str,
    rest: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationDeclarationRefusal> {
    let Some((assigned_by, value)) = rest.split_first() else {
        return Err(grammar(MutationDeclarationCause::NotAClause, at));
    };
    if assigned_by.punct() != Some('=') || value.is_empty() {
        return Err(grammar(
            MutationDeclarationCause::NotAClause,
            assigned_by.span(),
        ));
    }
    Ok(Clause::Assigned {
        key,
        value: value.to_vec(),
        at,
    })
}

fn mapping<'trees>(
    rest: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationDeclarationRefusal> {
    let [fact, assigned_by, word, arguments] = rest else {
        return Err(grammar(MutationDeclarationCause::NotAMapping, at));
    };
    if assigned_by.punct() != Some('=') {
        return Err(grammar(
            MutationDeclarationCause::NotAMapping,
            assigned_by.span(),
        ));
    }
    let Some(spelling) = fact.word() else {
        return Err(grammar(MutationDeclarationCause::NotAMapping, fact.span()));
    };
    let fact = MutationOwnerFact::of_spelling(spelling)
        .ok_or_else(|| grammar(MutationDeclarationCause::UnknownOwnerFact, fact.span()))?;
    let claim = named_value(word, arguments, at)?;
    Ok(Clause::Mapping { fact, claim, at })
}

fn permission<'trees>(
    rest: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<Clause<'trees>, MutationDeclarationRefusal> {
    let [word, arguments, assigned_by, roster] = rest else {
        return Err(grammar(MutationDeclarationCause::NotAPermission, at));
    };
    if assigned_by.punct() != Some('=') {
        return Err(grammar(
            MutationDeclarationCause::NotAPermission,
            assigned_by.span(),
        ));
    }
    let claim = named_value(word, arguments, at)?;
    let Some((CapturedDelimiter::Bracket, inner)) = roster.group() else {
        return Err(grammar(
            MutationDeclarationCause::NotAPermission,
            roster.span(),
        ));
    };
    let mut families = Vec::new();
    let mut group = Vec::new();
    for tree in inner.iter() {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(grammar(
                    MutationDeclarationCause::NotAPermission,
                    tree.span(),
                ));
            }
            families.push(operator_family(&group)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        families.push(operator_family(&group)?);
    }
    if families.is_empty() {
        return Err(grammar(
            MutationDeclarationCause::EmptyOperatorFamilies,
            roster.span(),
        ));
    }
    for (position, family) in families.iter().enumerate() {
        if families.iter().take(position).any(|seen| seen == family) {
            return Err(grammar(
                MutationDeclarationCause::DuplicateOperatorFamily,
                roster.span(),
            ));
        }
    }
    Ok(Clause::Permission {
        claim,
        families,
        at,
    })
}

fn operator_family(
    group: &[&CapturedTokenTree],
) -> Result<GeneratedMutationFamily, MutationDeclarationRefusal> {
    let [only] = group else {
        let at = group.first().map_or(SpanHandle::at(0), |tree| tree.span());
        return Err(grammar(MutationDeclarationCause::NotAPermission, at));
    };
    let Some(slug) = only.text() else {
        return Err(grammar(
            MutationDeclarationCause::NotAPermission,
            only.span(),
        ));
    };
    GeneratedMutationFamily::of_slug(slug)
        .ok_or_else(|| grammar(MutationDeclarationCause::UnknownOperatorFamily, only.span()))
}

fn distinct(clauses: &[Clause<'_>]) -> Result<(), MutationDeclarationRefusal> {
    for (position, clause) in clauses.iter().enumerate() {
        let duplicate = clauses
            .iter()
            .take(position)
            .any(|earlier| match (earlier, clause) {
                (Clause::Assigned { key: left, .. }, Clause::Assigned { key: right, .. }) => {
                    left == right
                }
                (Clause::Mapping { fact: left, .. }, Clause::Mapping { fact: right, .. }) => {
                    left == right
                }
                (
                    Clause::Permission { claim: left, .. },
                    Clause::Permission { claim: right, .. },
                ) => left == right,
                _ => false,
            });
        if duplicate {
            let (cause, at) = match clause {
                Clause::Assigned { at, .. } => (MutationDeclarationCause::NotDistinct, *at),
                Clause::Mapping { at, .. } => (MutationDeclarationCause::DuplicateOwnerFact, *at),
                Clause::Permission { at, .. } => {
                    (MutationDeclarationCause::DuplicatePermissionClaim, *at)
                }
            };
            return Err(grammar(cause, at));
        }
    }
    Ok(())
}

fn assigned<'trees, 'clauses>(
    clauses: &'clauses [Clause<'trees>],
    key: &str,
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| match clause {
        Clause::Assigned {
            key: found,
            value,
            at,
        } if *found == key => Some((value.as_slice(), *at)),
        Clause::Assigned { .. } | Clause::Mapping { .. } | Clause::Permission { .. } => None,
    })
}

fn identifier_value<'trees>(
    value: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<&'trees str, MutationDeclarationRefusal> {
    let [only] = value else {
        return Err(grammar(MutationDeclarationCause::NotAClause, at));
    };
    only.word()
        .ok_or_else(|| grammar(MutationDeclarationCause::NotAClause, only.span()))
}

fn named_assignment(
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<WallName, MutationDeclarationRefusal> {
    let (value, site) =
        assigned(clauses, key).ok_or_else(|| grammar(MutationDeclarationCause::NotCovered, at))?;
    let [word, arguments] = value else {
        return Err(grammar(MutationDeclarationCause::NotANamedReference, site));
    };
    named_value(word, arguments, site)
}

fn named_value(
    word: &CapturedTokenTree,
    arguments: &CapturedTokenTree,
    at: SpanHandle,
) -> Result<WallName, MutationDeclarationRefusal> {
    if word.word() != Some(NAMED_ROAD) {
        return Err(grammar(
            MutationDeclarationCause::NotANamedReference,
            word.span(),
        ));
    }
    let Some((CapturedDelimiter::Parenthesis, inner)) = arguments.group() else {
        return Err(grammar(
            MutationDeclarationCause::NotANamedReference,
            arguments.span(),
        ));
    };
    let parts: Vec<&CapturedTokenTree> = inner.iter().collect();
    let [namespace, separator, stem] = parts.as_slice() else {
        return Err(grammar(MutationDeclarationCause::NotANamedReference, at));
    };
    if separator.punct() != Some(',') {
        return Err(grammar(
            MutationDeclarationCause::NotANamedReference,
            separator.span(),
        ));
    }
    let (Some(namespace), Some(stem)) = (namespace.text(), stem.text()) else {
        return Err(grammar(
            MutationDeclarationCause::NotANamedReference,
            arguments.span(),
        ));
    };
    WallName::named(namespace, stem).map_err(|refusal| carrier(refusal, arguments.span()))
}

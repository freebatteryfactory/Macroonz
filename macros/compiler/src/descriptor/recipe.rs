//! Preparing recipe evidence through the bounded first-party descriptor adapter.

use crate::descriptor::{Emitter, Grammar};
use crate::diagnostic::{Diagnostic, Placement};
use crate::expansion::Expansion;
use crate::kind::Kind;
use crate::recipe::{
    EVIDENCE_LIMIT, EvidenceTarget, PreparedEvidence, Recipe, RecipeEvidence, RecipeRole,
};
use crate::render::RenderError;
use crate::request::Door;
use crate::token::{CapturedInput, GeneratedTree, SpanHandle};

struct PreparedNames {
    root_macros: Vec<String>,
    baked_types: Vec<String>,
}

/// Prepare every selected standard evidence projection through its existing descriptor owner.
pub(crate) fn prepared(
    capture: &CapturedInput,
    recipe: &Recipe,
    door: &Door,
    replaced: &[RecipeRole],
) -> Result<PreparedEvidence, Diagnostic> {
    let producer = door.producer();
    let emitter = Emitter {
        namespace: producer.namespace,
        producer: producer.name,
        door: "recipe",
    };
    let mut trees: [Option<GeneratedTree>; EVIDENCE_LIMIT] = core::array::from_fn(|_| None);
    let mut names = PreparedNames::over(recipe);
    for role in RecipeRole::ALL
        .iter()
        .copied()
        .filter(|role| crate::recipe::evidence_position(*role).is_some())
    {
        if replaced.contains(&role) {
            continue;
        }
        let Some(evidence) = recipe.evidence(role) else {
            continue;
        };
        let tree = prepared_tree(capture, recipe, evidence, role, emitter, door, &mut names)?;
        if let Some(position) = crate::recipe::evidence_position(role) {
            let Some(slot) = trees.get_mut(position) else {
                return Err(nothing_rendered(door));
            };
            *slot = Some(tree);
        }
    }
    Ok(PreparedEvidence::assembled(trees))
}

fn prepared_tree(
    capture: &CapturedInput,
    recipe: &Recipe,
    evidence: &RecipeEvidence,
    role: RecipeRole,
    emitter: Emitter,
    door: &Door,
    names: &mut PreparedNames,
) -> Result<GeneratedTree, Diagnostic> {
    match role {
        RecipeRole::Trials => {
            let expansion = crate::descriptor::door::trials_requiring_declaring(
                evidence.body(),
                capture,
                Grammar {
                    attribute: "trials",
                },
                emitter,
                door,
            )?;
            names.support(&expansion, evidence.at(), door)?;
            emitted(&expansion, door)
        }
        RecipeRole::Mutation => {
            let order = mutation_order(recipe, evidence.target(), door)?;
            let expansion = crate::descriptor::door::mutations_from_order_requiring_declaring(
                evidence.body(),
                capture,
                &order,
                evidence.at(),
                Grammar {
                    attribute: "mutations",
                },
                door,
            )?;
            names.support(&expansion, evidence.at(), door)?;
            emitted(&expansion, door)
        }
        RecipeRole::Benchmarks => {
            let expansion = crate::descriptor::door::bench_requiring_declaring(
                evidence.body(),
                capture,
                Grammar { attribute: "bench" },
                emitter,
                door,
            )?;
            names.support(&expansion, evidence.at(), door)?;
            emitted(&expansion, door)
        }
        RecipeRole::Network => {
            let expansion = crate::descriptor::door::network(
                evidence.body().clone(),
                Grammar {
                    attribute: "network",
                },
                door,
            )?;
            names.baked_type(expansion.plan().content().module(), evidence.at(), door)?;
            emitted(&expansion, door)
        }
        RecipeRole::Concurrency => {
            let expansion = crate::descriptor::door::concurrency(
                evidence.body().clone(),
                Grammar {
                    attribute: "concurrency",
                },
                door,
            )?;
            names.baked_type(expansion.plan().content().module(), evidence.at(), door)?;
            emitted(&expansion, door)
        }
        RecipeRole::Companions
        | RecipeRole::RelationTables
        | RecipeRole::Dispatch
        | RecipeRole::CompileContract
        | RecipeRole::DeclarationConformance
        | RecipeRole::Typestate
        | RecipeRole::Codec => Err(nothing_rendered(door)),
    }
}

impl PreparedNames {
    fn over(recipe: &Recipe) -> Self {
        let root_macros = recipe
            .support()
            .map(|support| vec![support.spelling().to_owned()])
            .unwrap_or_default();
        let baked_types = recipe.baked_type_names();
        Self {
            root_macros,
            baked_types,
        }
    }

    fn support(
        &mut self,
        expansion: &Expansion<crate::support::SupportCarrier>,
        at: SpanHandle,
        door: &Door,
    ) -> Result<(), Diagnostic> {
        let Some(address) = expansion.plan().content().address() else {
            return Err(nothing_rendered(door));
        };
        admit_name(&mut self.root_macros, address.spelling(), at, door)
    }

    fn baked_type(&mut self, name: &str, at: SpanHandle, door: &Door) -> Result<(), Diagnostic> {
        admit_name(&mut self.baked_types, name, at, door)
    }
}

fn admit_name(
    names: &mut Vec<String>,
    name: &str,
    at: SpanHandle,
    door: &Door,
) -> Result<(), Diagnostic> {
    if names
        .iter()
        .any(|occupied| identifier_key(occupied) == identifier_key(name))
    {
        return Err(crate::recipe::generated_name_collision(
            name.to_owned(),
            at,
            door,
        ));
    }
    names.push(name.to_owned());
    Ok(())
}

fn identifier_key(spelling: &str) -> &str {
    spelling.strip_prefix("r#").unwrap_or(spelling)
}

fn mutation_order(
    recipe: &Recipe,
    target: Option<&EvidenceTarget>,
    door: &Door,
) -> Result<Vec<String>, Diagnostic> {
    let vocabulary_name = target
        .map(EvidenceTarget::name)
        .ok_or_else(|| nothing_rendered(door))?;
    let vocabulary = recipe
        .vocabulary(vocabulary_name)
        .ok_or_else(|| nothing_rendered(door))?;
    let members = vocabulary.members().members().collect::<Vec<_>>();
    if members.is_empty() {
        return Err(nothing_rendered(door));
    }
    Ok(members
        .iter()
        .map(|member| member.spelling().to_owned())
        .collect())
}

fn emitted<K: Kind>(expansion: &Expansion<K>, door: &Door) -> Result<GeneratedTree, Diagnostic> {
    expansion
        .emit()
        .tokens()
        .cloned()
        .ok_or_else(|| nothing_rendered(door))
}

fn nothing_rendered(door: &Door) -> Diagnostic {
    Diagnostic::refused(
        &RenderError::NothingRendered,
        door,
        &Placement::WholeDeclaration,
    )
}

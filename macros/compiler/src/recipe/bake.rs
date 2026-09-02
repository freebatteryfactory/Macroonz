//! The paved and callable walks over one informed recipe and one projection protocol.

use super::render;
use super::types::{
    RECIPE_FACT, RecipeError, RecipeIssue, RecipeShell, RecipeShellContent, StandardProjector,
};
use super::{
    ConfiguredEvidence, EvidenceCompiler, HarnessPosture, PROJECTION_LIMIT, ProjectionSink,
    ProjectorReplacement, Recipe, RecipeBake, RecipeProjection, RecipeProjector, RecipeRole,
};
use crate::closure::PartitionCargo;
use crate::diagnostic::{Diagnostic, Placement, Refused};
use crate::kind::{Destination, Disposition, SoleRole};
use crate::request::{Door, Request};
use crate::support::{
    self, AxisCargo, CargoAxis, DeferredCargo, ProvedCargo, SupportAxes, SupportCarrier,
};
use crate::token::{
    CapturedInput, GeneratedDelimiter, GeneratedToken, GeneratedTree, SpanTable, documentation,
    group,
};

/// Bake one recipe through the standard projector catalog.
///
/// # Errors
///
/// Returns the exact capture, recipe, planning, rendering, closure, support, or final-emission diagnostic established before tokens are exposed.
pub fn bake(
    capture: &CapturedInput,
    harness: HarnessPosture,
    door: &Door,
) -> Result<RecipeBake, Diagnostic> {
    walked(capture, harness, door, &[])
}

/// Bake the facade wrapper's fixed posture envelope through the same recipe road.
///
/// This is inter-package carrier plumbing for `macroonz::recipe!`; ordinary callable hosts use [`bake`] or [`bake_with`].
///
/// # Errors
///
/// Returns the exact envelope, recipe, or downstream compiler diagnostic before any tokens are exposed.
#[doc(hidden)]
pub fn bake_wrapped(capture: &CapturedInput, door: &Door) -> Result<RecipeBake, Diagnostic> {
    let (harness, inner) =
        wrapper_input(capture).map_err(|refusal| recipe_refused(&refusal, door))?;
    walked(&inner, harness, door, &[])
}

/// Bake one recipe while replacing selected standard projections with caller-owned projectors.
///
/// Every replacement receives the same informed view, selected request, and one-use output sink as the standard projector.
/// Replacement slice order does not change invocation order; selected roles always run in the closed [`RecipeRole`] order.
///
/// # Errors
///
/// Returns the same diagnostics as [`bake`], plus a typed recipe refusal where the roster is unbounded, repeats a role, or names an unselected role.
pub fn bake_with(
    capture: &CapturedInput,
    harness: HarnessPosture,
    door: &Door,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<RecipeBake, Diagnostic> {
    walked(capture, harness, door, replacements)
}

fn walked(
    capture: &CapturedInput,
    harness: HarnessPosture,
    door: &Door,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<RecipeBake, Diagnostic> {
    let recipe =
        Recipe::read(capture, harness).map_err(|refusal| recipe_refused(&refusal, door))?;
    validate_replacements(&recipe, replacements)
        .map_err(|refusal| recipe_refused(&refusal, door))?;
    let selected = recipe.selected_roles().collect::<Vec<_>>();
    let Some((&first, rest)) = selected.split_first() else {
        return Err(recipe_refused(
            &RecipeError::at(RecipeIssue::ProjectionRequired, None),
            door,
        ));
    };
    let replaced = replacements
        .iter()
        .map(|replacement| replacement.role())
        .collect::<Vec<_>>();
    let prepared = ConfiguredEvidence::prepared(capture, &recipe, door, replaced.as_slice())?;
    let standard = StandardProjector::over(&prepared);
    let projection = Request::<RecipeProjection>::over(capture.clone(), recipe.clone(), door)
        .selecting(first, rest.to_vec())
        .assuming(vec![RECIPE_FACT])
        .render(|_, output| {
            for role in selected.iter().copied() {
                let replacement = replacements
                    .iter()
                    .copied()
                    .find(|replacement| replacement.role() == role);
                let projector: &dyn RecipeProjector = match replacement {
                    Some(replacement) => replacement.projector(),
                    None => &standard,
                };
                render::project(
                    &recipe,
                    role,
                    ProjectionSink::bound(output, role),
                    projector,
                )?;
            }
            Ok(())
        })?;
    let support = support(capture, &recipe, &projection, door)?;
    let emitted = final_emission(capture, &recipe, &projection, support.as_ref(), door)?;
    Ok(RecipeBake::baked(projection, emitted))
}

fn validate_replacements(
    recipe: &Recipe,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<(), RecipeError> {
    if replacements.len() > PROJECTION_LIMIT {
        return Err(RecipeError::at(
            RecipeIssue::ReplacementRosterUnbounded {
                observed: replacements.len(),
            },
            None,
        ));
    }
    for (position, replacement) in replacements.iter().copied().enumerate() {
        if replacements
            .iter()
            .take(position)
            .any(|earlier| earlier.role() == replacement.role())
        {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateReplacement {
                    role: replacement.role(),
                },
                None,
            ));
        }
        if !recipe
            .selected_roles()
            .any(|selected| selected == replacement.role())
        {
            return Err(RecipeError::at(
                RecipeIssue::ReplacementUnplanned {
                    role: replacement.role(),
                },
                None,
            ));
        }
    }
    Ok(())
}

fn wrapper_input(capture: &CapturedInput) -> Result<(HarnessPosture, CapturedInput), RecipeError> {
    let [facade, marker, body] = capture.trees() else {
        return Err(RecipeError::at(RecipeIssue::InlineModuleRequired, None));
    };
    let Some(facade) = facade.group_fragment(crate::token::CapturedDelimiter::Brace) else {
        return Err(RecipeError::at(
            RecipeIssue::InlineModuleRequired,
            Some(facade.span()),
        ));
    };
    if facade.is_empty() {
        return Err(RecipeError::at(
            RecipeIssue::InlineModuleRequired,
            Some(facade.enclosing_span().unwrap_or(marker.span())),
        ));
    }
    let harness = match marker.word() {
        Some("__macroonz_test_carrier_available") => HarnessPosture::Available,
        Some("__macroonz_test_carrier_unavailable") => HarnessPosture::Unavailable,
        _ => {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(marker.span()),
            ));
        }
    };
    let Some(fragment) = body.group_fragment(crate::token::CapturedDelimiter::Brace) else {
        return Err(RecipeError::at(
            RecipeIssue::InlineModuleRequired,
            Some(body.span()),
        ));
    };
    let selected = CapturedInput::selected(fragment, capture.issued()).map_err(|_| {
        RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: crate::token::CAPTURED_TOKEN_LIMIT,
            }),
            Some(body.span()),
        )
    })?;
    Ok((harness, selected))
}

fn support(
    capture: &CapturedInput,
    recipe: &Recipe,
    projection: &crate::expansion::Expansion<RecipeProjection>,
    door: &Door,
) -> Result<Option<crate::expansion::Expansion<SupportCarrier>>, Diagnostic> {
    let Some(address) = recipe.support().cloned() else {
        return Ok(None);
    };
    let deferred = proved_test_cargo(projection, door)?;
    let axes = SupportAxes {
        declared: AxisCargo::Absent {
            because: Disposition::NotRequested {
                because: RECIPE_FACT,
            },
        },
        deferred: AxisCargo::Carried(deferred),
        bench: AxisCargo::Absent {
            because: Disposition::NotApplicable {
                because: RECIPE_FACT,
            },
        },
    };
    let assembly = support::SupportAssembly::assembled_requiring_declaring(
        projection.plan().account().commitment(),
        Some(address),
        axes,
    )
    .map_err(|refusal| whole(&refusal, door))?;
    support::delivered(capture.clone(), Vec::new(), assembly, door).map(Some)
}

fn proved_test_cargo(
    projection: &crate::expansion::Expansion<RecipeProjection>,
    door: &Door,
) -> Result<ProvedCargo, Diagnostic> {
    let Some(PartitionCargo::Carried(cargo)) =
        projection.emission().joined(Destination::TestCarrier)
    else {
        return Err(recipe_refused(
            &RecipeError::at(RecipeIssue::SupportAddressUnneeded, None),
            door,
        ));
    };
    ProvedCargo::carried(
        projection,
        CargoAxis::Deferred,
        Destination::TestCarrier,
        DeferredCargo::deferred(cargo.tree().clone()),
    )
    .map_err(|refusal| whole(&refusal, door))
}

fn final_emission(
    capture: &CapturedInput,
    recipe: &Recipe,
    projection: &crate::expansion::Expansion<RecipeProjection>,
    support: Option<&crate::expansion::Expansion<SupportCarrier>>,
    door: &Door,
) -> Result<crate::expansion::Expansion<RecipeShell>, Diagnostic> {
    let tree = final_tree(recipe, projection, support).map_err(|overflow| {
        whole(
            &crate::render::RenderError::TokensUnbounded {
                bound: overflow.capacity,
                observed: overflow.offered,
            },
            door,
        )
    })?;
    let content = RecipeShellContent::composed(
        projection.identity(),
        support.map(crate::expansion::Expansion::identity),
    );
    Request::<RecipeShell>::over(capture.clone(), content, door)
        .assuming(vec![RECIPE_FACT])
        .render(|_, output| output.unit(SoleRole::Sole, tree))
}

fn final_tree(
    recipe: &Recipe,
    projection: &crate::expansion::Expansion<RecipeProjection>,
    support: Option<&crate::expansion::Expansion<SupportCarrier>>,
) -> Result<GeneratedTree, crate::bounded::Overflow> {
    let mut root = GeneratedTree::assembled(Vec::new())?;
    if let Some(support) = support
        && let Some(tree) = support.emit().tokens()
    {
        root = root.joined(tree)?;
    }
    for role in [
        RecipeRole::Trials,
        RecipeRole::Mutation,
        RecipeRole::Benchmarks,
    ] {
        if let Some(unit) = projection.closure().rendered().under(role) {
            root = root.joined(unit.tree())?;
        }
    }
    let mut body = recipe.authored_body().clone();
    let mut companions = Vec::new();
    for role in [
        RecipeRole::Companions,
        RecipeRole::RelationTables,
        RecipeRole::Codec,
        RecipeRole::Dispatch,
        RecipeRole::Typestate,
        RecipeRole::Network,
        RecipeRole::Concurrency,
    ] {
        if let Some(unit) = projection.closure().rendered().under(role) {
            companions.extend(unit.tree().tokens().iter().cloned());
        }
    }
    if !companions.is_empty() {
        let mut generated = documentation(
            "Generated companions selected by this recipe's informed projection account.",
        )?;
        generated.extend([
            GeneratedToken::word("pub"),
            GeneratedToken::word("mod"),
            GeneratedToken::word("baked"),
            group(GeneratedDelimiter::Brace, companions)?,
        ]);
        body = body.joined(&GeneratedTree::assembled(generated)?)?;
    }
    let grouped = body.grouped(GeneratedDelimiter::Brace, recipe.module_body_at())?;
    let module = recipe.module_head().joined(&grouped)?;
    root.joined(&module)
}

fn recipe_refused(refusal: &RecipeError, door: &Door) -> Diagnostic {
    match refusal.token() {
        Some(token) => Diagnostic::refused(
            refusal,
            door,
            &Placement::AtToken {
                token,
                spans: &SpanTable::ProducerHeld,
            },
        ),
        None => Diagnostic::refused(refusal, door, &Placement::WholeDeclaration),
    }
}

pub(crate) fn generated_name_collision(name: String, door: &Door) -> Diagnostic {
    recipe_refused(
        &RecipeError::at(RecipeIssue::GeneratedNameCollision { name }, None),
        door,
    )
}

fn whole<E: Refused>(refusal: &E, door: &Door) -> Diagnostic {
    Diagnostic::refused(refusal, door, &Placement::WholeDeclaration)
}

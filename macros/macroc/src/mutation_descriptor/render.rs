//! Rendering one generated mutation discovery and its directive-shaped callables.

use super::encode::declared_order_operation;
use super::{GeneratedMutationFamily, MutationDeclaration, MutationOwnerFact};
use crate::derive_refusal::{
    CapturedCause, MutationDeclarationPosture, RefusalDeriveSurface, RenderRefusal,
    render::declared_order_expression,
};
use crate::test_descriptor::{
    CrateFacing, group as shell_group, roster as shell_roster, twin_path,
};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::refusal::FamilyShape;

const REFUSAL: &str = "MutationLoweringRefusal";
const POINT_SUFFIX: &str = "declared-order";
const ORIGINAL_ORDER: &str = "ORIGINAL_ORDER";

/// One adjacent declared-order alternative, with its semantic bytes and rendered meaning inseparable.
struct DeclaredOrderAlternative<'surface> {
    operation: Vec<u8>,
    order: Vec<&'surface CapturedCause>,
}

/// The one producer plan from which discovery bytes, constants, and directive dispatch are rendered.
struct DeclaredOrderPlan<'surface> {
    original_operation: Vec<u8>,
    original_order: Vec<&'surface CapturedCause>,
    alternatives: Vec<DeclaredOrderAlternative<'surface>>,
}

/// Render the helper-named module TestPak lowers and invokes.
///
/// # Errors
///
/// Refuses when the generated tree exceeds the declared token magnitude.
pub(crate) fn generated_module(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let MutationDeclarationPosture::Declared(declared) = surface.mutations() else {
        return Err(RenderRefusal::Unbounded);
    };
    let declaration = declared.declaration();
    let plan = declared_order_plan(surface);

    let mut body = refusal_type()?;
    body.extend(order_constants(surface, &plan)?);
    body.extend(candidate_orders(surface, &plan)?);
    body.extend(lowering(surface, declaration, &plan)?);
    body.extend(production(surface)?);
    body.extend(evaluation(surface, &plan.alternatives)?);

    let tokens = vec![
        GeneratedToken::word("pub"),
        visibility_crate()?,
        GeneratedToken::word("mod"),
        GeneratedToken::word(declaration.module().spelling()),
        group(GeneratedDelimiter::Brace, body).map_err(|_| RenderRefusal::Unbounded)?,
    ];
    GeneratedTree::assembled(tokens).map_err(|_| RenderRefusal::Unbounded)
}

fn refusal_type() -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let variants = [
        ("Name", harness(&["descriptor", "NameRefusal"])),
        ("Permission", harness(&["muterprater", "PermissionRefusal"])),
        ("Policy", harness(&["muterprater", "PolicyRefusal"])),
        ("Discovery", harness(&["muterprater", "DiscoveryRefusal"])),
        (
            "Lowering",
            harness(&["muterprater", "DiscoveryLoweringRefusal"]),
        ),
    ];
    let mut body = Vec::new();
    for (name, carried) in variants {
        body.push(GeneratedToken::word(name));
        body.push(group(GeneratedDelimiter::Parenthesis, carried)?);
        body.push(GeneratedToken::alone(','));
    }
    body.push(GeneratedToken::word("OperatorFamilyNotFound"));
    body.push(GeneratedToken::alone(','));
    Ok(vec![
        GeneratedToken::alone('#'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("derive"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("Debug")],
                )?,
            ],
        )?,
        GeneratedToken::word("pub"),
        visibility_crate()?,
        GeneratedToken::word("enum"),
        GeneratedToken::word(REFUSAL),
        group(GeneratedDelimiter::Brace, body)?,
    ])
}

fn lowering(
    surface: &RefusalDeriveSurface,
    declaration: &MutationDeclaration,
    plan: &DeclaredOrderPlan<'_>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut body = Vec::new();
    body.extend(bound_local(
        "family",
        mapped(
            call(
                harness(&["muterprater", "EvaluationFamilyRef", "named"]),
                text_pair(declaration.family().namespace(), declaration.family().stem()),
            )?,
            "Name",
        )?,
    ));

    for (position, permission) in declaration.permissions().enumerate() {
        let claim = format!("permission_claim_{position}");
        body.extend(bound_local(
            &claim,
            mapped(
                call(
                    harness(&["descriptor", "ClaimRef", "named"]),
                    text_pair(permission.claim().namespace(), permission.claim().stem()),
                )?,
                "Name",
            )?,
        ));
        let mut family_locals = Vec::new();
        for (family_position, family) in permission.families().iter().enumerate() {
            let local = format!("permission_family_{position}_{family_position}");
            body.extend(bound_local(
                &local,
                option_or(
                    call(
                        harness(&["muterprater", "OperatorFamilyRef", "of_slug"]),
                        vec![GeneratedToken::text(family.slug())],
                    )?,
                    "OperatorFamilyNotFound",
                )?,
            ));
            family_locals.push(GeneratedToken::word(&local));
            family_locals.push(GeneratedToken::alone(','));
        }
        body.extend(bound_local(
            &format!("permission_{position}"),
            mapped(
                call(
                    harness(&["muterprater", "MutationPermission", "declared"]),
                    comma(vec![GeneratedToken::word(&claim)], roster(family_locals)?),
                )?,
                "Permission",
            )?,
        ));
    }

    let mut permission_locals = Vec::new();
    for position in 0..declaration.permissions().count() {
        permission_locals.push(GeneratedToken::word(format!("permission_{position}").as_str()));
        permission_locals.push(GeneratedToken::alone(','));
    }
    body.extend(bound_local(
        "policy",
        mapped(
            call(
                harness(&["muterprater", "MutationPolicy", "declared"]),
                comma(vec![GeneratedToken::word("family")], roster(permission_locals)?),
            )?,
            "Policy",
        )?,
    ));

    let mut sites = Vec::new();
    if !plan.alternatives.is_empty() {
        body.extend(site(surface, declaration, plan)?);
        sites.push(GeneratedToken::word("declared_order_site"));
        sites.push(GeneratedToken::alone(','));
    }
    let mut lowered = harness(&["muterprater", "discover", "lower_discoveries"]);
    let mut arguments = vec![GeneratedToken::alone('&'), GeneratedToken::word("policy")];
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(roster(sites)?);
    lowered.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    lowered.extend(map_err_tokens("Lowering")?);
    body.extend(lowered);

    Ok(function(
        "lowering",
        Vec::new(),
        result_type(
            harness(&["muterprater", "MutationSurfaceLowering"]),
            vec![GeneratedToken::word(REFUSAL)],
        ),
        body,
    )?)
}

fn site(
    surface: &RefusalDeriveSurface,
    declaration: &MutationDeclaration,
    plan: &DeclaredOrderPlan<'_>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let (point_namespace, point_stem) = point_name(surface);
    let mut body = Vec::new();
    body.extend(bound_local(
        "point",
        mapped(
            call(
                harness(&["descriptor", "MutationPointRef", "named"]),
                text_pair(point_namespace, &point_stem),
            )?,
            "Name",
        )?,
    ));
    body.extend(bound_local(
        "activation",
        mapped(
            call(
                harness(&["muterprater", "ActivationSite", "named"]),
                text_pair(point_namespace, &point_stem),
            )?,
            "Name",
        )?,
    ));
    body.extend(bound_local(
        "declared_order_family",
        option_or(
            call(
                harness(&["muterprater", "OperatorFamilyRef", "of_slug"]),
                vec![GeneratedToken::text(
                    GeneratedMutationFamily::DeclaredOrderPermutation.slug(),
                )],
            )?,
            "OperatorFamilyNotFound",
        )?,
    ));

    let mapping = match declaration.mapping(MutationOwnerFact::DeclaredOrder) {
        Some(claim) => {
            body.extend(bound_local(
                "owner_claim",
                mapped(
                    call(
                        harness(&["descriptor", "ClaimRef", "named"]),
                        text_pair(claim.namespace(), claim.stem()),
                    )?,
                    "Name",
                )?,
            ));
            let mut tokens = harness(&["muterprater", "OwnerClaimMapping", "Mapped"]);
            tokens.push(group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("owner_claim")],
            )?);
            tokens
        }
        None => harness(&["muterprater", "OwnerClaimMapping", "OwnerUnmapped"]),
    };

    let mut candidate_roster = Vec::new();
    for alternative in &plan.alternatives {
        candidate_roster.extend(call(
            harness(&["muterprater", "AlternativeDeclaration", "stated"]),
            comma(
                vec![GeneratedToken::word("declared_order_family")],
                vec![
                    GeneratedToken::byte_text(&alternative.operation),
                    GeneratedToken::alone('.'),
                    GeneratedToken::word("to_vec"),
                    group(GeneratedDelimiter::Parenthesis, Vec::new())?,
                ],
            ),
        )?);
        candidate_roster.push(GeneratedToken::alone(','));
    }
    let arguments = comma_many(vec![
        vec![GeneratedToken::word("point")],
        mapping,
        vec![
            GeneratedToken::byte_text(&plan.original_operation),
            GeneratedToken::alone('.'),
            GeneratedToken::word("to_vec"),
            group(GeneratedDelimiter::Parenthesis, Vec::new())?,
        ],
        roster(candidate_roster)?,
        vec![GeneratedToken::word("activation")],
    ]);
    body.extend(bound_local(
        "declared_order_site",
        mapped(
            call(
                harness(&["muterprater", "DiscoveredMutationSite", "discovered"]),
                arguments,
            )?,
            "Discovery",
        )?,
    ));
    Ok(body)
}

fn production(surface: &RefusalDeriveSurface) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let parameters = vec![
        GeneratedToken::word("_input"),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
    ];
    let result = machine_order_type(surface);
    let body = vec![GeneratedToken::word(ORIGINAL_ORDER)];
    function("production", parameters, result, body)
}

fn evaluation(
    surface: &RefusalDeriveSurface,
    alternatives: &[DeclaredOrderAlternative<'_>],
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut parameters = vec![
        GeneratedToken::word("_input"),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
        GeneratedToken::alone(','),
        GeneratedToken::word("directive"),
        GeneratedToken::alone(':'),
    ];
    parameters.extend(harness(&["muterprater", "EvaluationDirective"]));
    parameters.push(GeneratedToken::alone('<'));
    parameters.push(GeneratedToken::joint('\''));
    parameters.push(GeneratedToken::word("surface"));
    parameters.push(GeneratedToken::alone('>'));

    let mut observation = harness(&["muterprater", "EvaluationObservation"]);
    observation.push(GeneratedToken::alone('<'));
    observation.extend(machine_order_type(surface));
    observation.push(GeneratedToken::alone('>'));
    let result = result_type(
        observation,
        harness(&["muterprater", "EvaluationCallRefusal"]),
    );

    let mut body = Vec::new();
    body.extend(vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("Some"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("resolved")],
        )?,
        GeneratedToken::alone('='),
        GeneratedToken::word("directive"),
        GeneratedToken::alone('.'),
        GeneratedToken::word("resolved"),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
        GeneratedToken::word("else"),
        group(
            GeneratedDelimiter::Brace,
            returned_observation(vec![GeneratedToken::word(ORIGINAL_ORDER)], 0)?,
        )?,
        GeneratedToken::alone(';'),
    ]);

    for (left, alternative) in alternatives.iter().enumerate() {
        let condition = active_condition(surface, &alternative.operation)?;
        body.push(GeneratedToken::word("if"));
        body.extend(condition);
        body.push(group(
            GeneratedDelimiter::Brace,
            returned_observation(
                vec![GeneratedToken::word(&format!("ALTERNATIVE_ORDER_{left}"))],
                1,
            )?,
        )?);
    }
    let mut refusal = harness(&[
        "muterprater",
        "EvaluationCallRefusal",
        "ActiveSelectionNotImplemented",
    ]);
    refusal.push(group(
        GeneratedDelimiter::Parenthesis,
        method_call("resolved", "selection")?,
    )?);
    let mut returned = vec![GeneratedToken::word("Err")];
    returned.push(group(GeneratedDelimiter::Parenthesis, refusal)?);
    body.extend(returned);

    let mut tokens = vec![GeneratedToken::word("pub"), visibility_crate()?];
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word("evaluation"));
    tokens.push(GeneratedToken::alone('<'));
    tokens.push(GeneratedToken::joint('\''));
    tokens.push(GeneratedToken::word("surface"));
    tokens.push(GeneratedToken::alone('>'));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(result);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

fn order_constants(
    surface: &RefusalDeriveSurface,
    plan: &DeclaredOrderPlan<'_>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = constant(
        ORIGINAL_ORDER,
        machine_order_type(surface),
        order_expression(surface, &plan.original_order)?,
    );
    for (position, alternative) in plan.alternatives.iter().enumerate() {
        tokens.extend(constant(
            &format!("ALTERNATIVE_ORDER_{position}"),
            machine_order_type(surface),
            order_expression(surface, &alternative.order)?,
        ));
    }
    Ok(tokens)
}

fn candidate_orders(
    surface: &RefusalDeriveSurface,
    plan: &DeclaredOrderPlan<'_>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut values = Vec::new();
    for position in 0..plan.alternatives.len() {
        values.push(GeneratedToken::word(&format!("ALTERNATIVE_ORDER_{position}")));
        values.push(GeneratedToken::alone(','));
    }
    let mut kind = machine_order_type(surface);
    kind.push(GeneratedToken::alone(';'));
    kind.push(GeneratedToken::number(
        u64::try_from(plan.alternatives.len()).unwrap_or(u64::MAX),
    ));
    let result = vec![group(GeneratedDelimiter::Bracket, kind)?];
    let body = vec![group(GeneratedDelimiter::Bracket, values)?];
    function("candidate_orders", Vec::new(), result, body)
}

fn declared_order_plan(surface: &RefusalDeriveSurface) -> DeclaredOrderPlan<'_> {
    if !matches!(surface.shape(), FamilyShape::SingleCause) {
        return DeclaredOrderPlan {
            original_operation: Vec::new(),
            original_order: Vec::new(),
            alternatives: Vec::new(),
        };
    }
    let original_order: Vec<_> = surface.causes().collect();
    let original_operation = operation_for(surface.family_id(), &original_order);
    let mut alternatives = Vec::new();
    for left in 0..original_order.len().saturating_sub(1) {
        let mut order = original_order.clone();
        order.swap(left, left.saturating_add(1));
        alternatives.push(DeclaredOrderAlternative {
            operation: operation_for(surface.family_id(), &order),
            order,
        });
    }
    DeclaredOrderPlan {
        original_operation,
        original_order,
        alternatives,
    }
}

fn operation_for(family: &str, order: &[&CapturedCause]) -> Vec<u8> {
    declared_order_operation(
        family,
        order
            .iter()
            .map(|cause| (cause.local_key(), cause.spelling())),
    )
}

fn order_expression(
    surface: &RefusalDeriveSurface,
    order: &[&CapturedCause],
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    match surface.shape() {
        FamilyShape::SingleCause => declared_order_expression(surface, order.iter().copied()),
        FamilyShape::IssueCollection | FamilyShape::InseparablePair => {
            call(machine_path(surface, &["refusal", "DeclaredCauseOrder", "none"]), Vec::new())
        }
    }
}

fn constant(
    name: &str,
    kind: Vec<GeneratedToken>,
    value: Vec<GeneratedToken>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("const"), GeneratedToken::word(name)];
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(kind);
    tokens.push(GeneratedToken::alone('='));
    tokens.extend(value);
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

fn active_condition(
    surface: &RefusalDeriveSurface,
    operation: &[u8],
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let (namespace, stem) = point_name(surface);
    let comparisons = vec![
        equality(method_chain("resolved", &["point", "identity", "name", "namespace", "written"])?, vec![GeneratedToken::text(namespace)]),
        equality(method_chain("resolved", &["point", "identity", "name", "stem", "written"])?, vec![GeneratedToken::text(&stem)]),
        equality(method_chain("resolved", &["alternative", "family", "slug"])?, vec![GeneratedToken::text(GeneratedMutationFamily::DeclaredOrderPermutation.slug())]),
        equality(method_chain("resolved", &["alternative", "operation"])?, vec![GeneratedToken::byte_text(operation)]),
    ];
    Ok(and_all(comparisons))
}

fn returned_observation(
    meaning: Vec<GeneratedToken>,
    firings: u32,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut observed = call(
        harness(&["muterprater", "EvaluationObservation", "observed"]),
        comma(meaning, vec![GeneratedToken::number(u64::from(firings))]),
    )?;
    let mut ok = vec![GeneratedToken::word("Ok")];
    ok.push(group(GeneratedDelimiter::Parenthesis, observed.split_off(0))?);
    let mut returned = vec![GeneratedToken::word("return")];
    returned.extend(ok);
    returned.push(GeneratedToken::alone(';'));
    Ok(returned)
}

fn function(
    name: &str,
    parameters: Vec<GeneratedToken>,
    result: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = vec![
        GeneratedToken::word("pub"),
        visibility_crate()?,
        GeneratedToken::word("fn"),
        GeneratedToken::word(name),
        group(GeneratedDelimiter::Parenthesis, parameters)?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
    ];
    tokens.extend(result);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

fn bound_local(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(name),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

fn mapped(mut expression: Vec<GeneratedToken>, variant: &str) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    expression.extend(map_err_tokens(variant)?);
    expression.push(GeneratedToken::alone('?'));
    Ok(expression)
}

fn map_err_tokens(variant: &str) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = vec![
        GeneratedToken::alone('.'),
        GeneratedToken::word("map_err"),
    ];
    let path = vec![
        GeneratedToken::word(REFUSAL),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(variant),
    ];
    tokens.push(group(GeneratedDelimiter::Parenthesis, path)?);
    Ok(tokens)
}

fn option_or(mut expression: Vec<GeneratedToken>, variant: &str) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    expression.push(GeneratedToken::alone('.'));
    expression.push(GeneratedToken::word("ok_or"));
    expression.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word(REFUSAL),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word(variant),
        ],
    )?);
    expression.push(GeneratedToken::alone('?'));
    Ok(expression)
}

fn call(
    mut path: Vec<GeneratedToken>,
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    path.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(path)
}

fn result_type(mut ok: Vec<GeneratedToken>, error: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut result = GeneratedToken::absolute_path(&["core", "result", "Result"]);
    result.push(GeneratedToken::alone('<'));
    result.append(&mut ok);
    result.push(GeneratedToken::alone(','));
    result.extend(error);
    result.push(GeneratedToken::alone('>'));
    result
}

fn harness(segments: &[&str]) -> Vec<GeneratedToken> {
    twin_path(CrateFacing::Harness, segments)
}

fn machine_path(surface: &RefusalDeriveSurface, segments: &[&str]) -> Vec<GeneratedToken> {
    let mut path = vec![surface.binding().spelling()];
    path.extend_from_slice(segments);
    GeneratedToken::absolute_path(&path)
}

fn machine_order_type(surface: &RefusalDeriveSurface) -> Vec<GeneratedToken> {
    machine_path(surface, &["refusal", "DeclaredCauseOrder"])
}

fn text_pair(namespace: &str, stem: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::text(namespace),
        GeneratedToken::alone(','),
        GeneratedToken::text(stem),
    ]
}

fn comma(mut left: Vec<GeneratedToken>, right: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    left.push(GeneratedToken::alone(','));
    left.extend(right);
    left
}

fn comma_many(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut joined = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            joined.push(GeneratedToken::alone(','));
        }
        joined.extend(part);
    }
    joined
}

fn method_call(local: &str, method: &str) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    Ok(vec![
        GeneratedToken::word(local),
        GeneratedToken::alone('.'),
        GeneratedToken::word(method),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
    ])
}

fn method_chain(
    local: &str,
    methods: &[&str],
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = vec![GeneratedToken::word(local)];
    for method in methods {
        tokens.push(GeneratedToken::alone('.'));
        tokens.push(GeneratedToken::word(method));
        tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    }
    Ok(tokens)
}

fn equality(mut left: Vec<GeneratedToken>, right: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    left.push(GeneratedToken::joint('='));
    left.push(GeneratedToken::alone('='));
    left.extend(right);
    left
}

fn and_all(comparisons: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, comparison) in comparisons.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::joint('&'));
            tokens.push(GeneratedToken::alone('&'));
        }
        tokens.extend(comparison);
    }
    tokens
}

fn visibility_crate() -> Result<GeneratedToken, RenderRefusal> {
    group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word("crate")],
    )
    .map_err(|_| RenderRefusal::Unbounded)
}

fn point_name(surface: &RefusalDeriveSurface) -> (&str, String) {
    let (namespace, family) = surface
        .family_id()
        .split_once('.')
        .unwrap_or(("", surface.family_id()));
    (namespace, format!("{family}-{POINT_SUFFIX}"))
}

fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, RenderRefusal> {
    shell_group(delimiter, tokens).map_err(|_| RenderRefusal::Unbounded)
}

fn roster(tokens: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    shell_roster(tokens).map_err(|_| RenderRefusal::Unbounded)
}

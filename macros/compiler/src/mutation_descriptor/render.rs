//! Rendering one generated mutation discovery and its directive-shaped callables.

use super::encode::declared_order_operation;
use super::{
    GeneratedMutationFamily, MutationDeclaration, MutationOrderCause, MutationOrderDeclaration,
    MutationOwnerFact, MutationProjectionRequest, MutationRenderRefusal,
};
use crate::test_descriptor::{
    CrateFacing, group as shell_group, roster as shell_roster, twin_path,
};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};

const REFUSAL: &str = "MutationLoweringRefusal";
const POINT_SUFFIX: &str = "declared-order";

/// One adjacent declared-order alternative, with its semantic bytes and rendered meaning inseparable.
struct DeclaredOrderAlternative {
    operation: Vec<u8>,
    order: Vec<MutationOrderCause>,
}

/// The one producer plan from which discovery bytes, constants, and directive dispatch are rendered.
struct DeclaredOrderPlan {
    original_operation: Vec<u8>,
    alternatives: Vec<DeclaredOrderAlternative>,
}

/// Render the helper-named module `TestPak` lowers and invokes.
///
/// # Errors
///
/// Refuses when the generated tree exceeds the declared token magnitude.
pub(crate) fn generated_module(
    request: &MutationProjectionRequest,
) -> Result<GeneratedTree, MutationRenderRefusal> {
    let declaration = request.declaration();
    let plan = declared_order_plan(request);

    let mut body = refusal_type()?;
    body.extend(alternative_order_constants(request, &plan)?);
    body.extend(candidate_orders(request, &plan)?);
    body.extend(lowering(request, declaration, &plan)?);
    body.extend(production(request)?);
    body.extend(evaluation(request, &plan.alternatives)?);

    let tokens = vec![
        GeneratedToken::word("pub"),
        visibility_crate()?,
        GeneratedToken::word("mod"),
        GeneratedToken::word(declaration.module().spelling()),
        group(GeneratedDelimiter::Brace, body)?,
    ];
    GeneratedTree::assembled(tokens).map_err(|_| MutationRenderRefusal::Unbounded)
}

fn refusal_type() -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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
    request: &MutationProjectionRequest,
    declaration: &MutationDeclaration,
    plan: &DeclaredOrderPlan,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let mut body = Vec::new();
    body.extend(bound_local(
        "family",
        mapped(
            call(
                harness(&["muterprater", "EvaluationFamilyRef", "named"]),
                text_pair(
                    declaration.family().namespace(),
                    declaration.family().stem(),
                ),
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
        permission_locals.push(GeneratedToken::word(
            format!("permission_{position}").as_str(),
        ));
        permission_locals.push(GeneratedToken::alone(','));
    }
    body.extend(bound_local(
        "policy",
        mapped(
            call(
                harness(&["muterprater", "MutationPolicy", "declared"]),
                comma(
                    vec![GeneratedToken::word("family")],
                    roster(permission_locals)?,
                ),
            )?,
            "Policy",
        )?,
    ));

    let mut sites = Vec::new();
    if !plan.alternatives.is_empty() {
        body.extend(site(request, declaration, plan)?);
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

    function(
        "lowering",
        Vec::new(),
        result_type(
            harness(&["muterprater", "MutationSurfaceLowering"]),
            vec![GeneratedToken::word(REFUSAL)],
        ),
        body,
    )
}

fn site(
    request: &MutationProjectionRequest,
    declaration: &MutationDeclaration,
    plan: &DeclaredOrderPlan,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let point_namespace = request.point().namespace();
    let point_stem = format!("{}-{POINT_SUFFIX}", request.point().stem());
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

fn production(
    request: &MutationProjectionRequest,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let parameters = vec![
        GeneratedToken::word("_input"),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
    ];
    let result = request.order_type().to_vec();
    let body = request.production_expression().to_vec();
    function("production", parameters, result, body)
}

fn evaluation(
    request: &MutationProjectionRequest,
    alternatives: &[DeclaredOrderAlternative],
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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
    observation.extend(request.order_type().iter().cloned());
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
            returned_observation(
                call_local("production", vec![GeneratedToken::word("_input")])?,
                0,
            )?,
        )?,
        GeneratedToken::alone(';'),
    ]);

    for (left, alternative) in alternatives.iter().enumerate() {
        let condition = active_condition(request, &alternative.operation)?;
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

fn alternative_order_constants(
    request: &MutationProjectionRequest,
    plan: &DeclaredOrderPlan,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let mut tokens = Vec::new();
    for (position, alternative) in plan.alternatives.iter().enumerate() {
        tokens.extend(constant(
            &format!("ALTERNATIVE_ORDER_{position}"),
            request.order_type().to_vec(),
            order_expression(request, &alternative.order)?,
        ));
    }
    Ok(tokens)
}

fn candidate_orders(
    request: &MutationProjectionRequest,
    plan: &DeclaredOrderPlan,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let mut values = Vec::new();
    for position in 0..plan.alternatives.len() {
        values.push(GeneratedToken::word(&format!(
            "ALTERNATIVE_ORDER_{position}"
        )));
        values.push(GeneratedToken::alone(','));
    }
    let mut kind = request.order_type().to_vec();
    kind.push(GeneratedToken::alone(';'));
    kind.push(GeneratedToken::number(request.alternative_count()));
    let result = vec![group(GeneratedDelimiter::Bracket, kind)?];
    let body = vec![group(GeneratedDelimiter::Bracket, values)?];
    function("candidate_orders", Vec::new(), result, body)
}

fn declared_order_plan(request: &MutationProjectionRequest) -> DeclaredOrderPlan {
    let MutationOrderDeclaration::Declared(original_order) = request.order() else {
        return DeclaredOrderPlan {
            original_operation: Vec::new(),
            alternatives: Vec::new(),
        };
    };
    let original_operation = operation_for(request.point(), original_order);
    let mut alternatives = Vec::new();
    for left in 0..original_order.len().saturating_sub(1) {
        let mut order = original_order.clone();
        order.swap(left, left.saturating_add(1));
        alternatives.push(DeclaredOrderAlternative {
            operation: operation_for(request.point(), &order),
            order,
        });
    }
    DeclaredOrderPlan {
        original_operation,
        alternatives,
    }
}

fn operation_for(
    family: &crate::test_descriptor::WallName,
    order: &[MutationOrderCause],
) -> Vec<u8> {
    let family_id = format!("{}.{}", family.namespace(), family.stem());
    declared_order_operation(
        &family_id,
        order
            .iter()
            .map(|cause| (cause.local_key(), cause.spelling())),
    )
}

fn order_expression(
    request: &MutationProjectionRequest,
    order: &[MutationOrderCause],
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    match request.order() {
        MutationOrderDeclaration::NotApplicable { expression } => Ok(expression.clone()),
        MutationOrderDeclaration::Declared(_) => {
            let mut rows = Vec::new();
            for cause in order {
                rows.extend(cause.row().iter().cloned());
                rows.push(GeneratedToken::alone(','));
            }
            let mut borrowed = vec![GeneratedToken::alone('&')];
            borrowed.push(group(GeneratedDelimiter::Bracket, rows)?);
            call(request.order_constructor().to_vec(), borrowed)
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
    request: &MutationProjectionRequest,
    operation: &[u8],
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let namespace = request.point().namespace();
    let stem = format!("{}-{POINT_SUFFIX}", request.point().stem());
    let comparisons = vec![
        equality(
            method_chain(
                "resolved",
                &["point", "identity", "name", "namespace", "written"],
            )?,
            vec![GeneratedToken::text(namespace)],
        ),
        equality(
            method_chain(
                "resolved",
                &["point", "identity", "name", "stem", "written"],
            )?,
            vec![GeneratedToken::text(&stem)],
        ),
        equality(
            method_chain("resolved", &["alternative", "family", "slug"])?,
            vec![GeneratedToken::text(
                GeneratedMutationFamily::DeclaredOrderPermutation.slug(),
            )],
        ),
        equality(
            method_chain("resolved", &["alternative", "operation"])?,
            vec![GeneratedToken::byte_text(operation)],
        ),
    ];
    Ok(and_all(comparisons))
}

fn returned_observation(
    meaning: Vec<GeneratedToken>,
    firings: u32,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let mut observed = call(
        harness(&["muterprater", "EvaluationObservation", "observed"]),
        comma(meaning, vec![GeneratedToken::number(u64::from(firings))]),
    )?;
    let mut ok = vec![GeneratedToken::word("Ok")];
    ok.push(group(
        GeneratedDelimiter::Parenthesis,
        observed.split_off(0),
    )?);
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
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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

fn mapped(
    mut expression: Vec<GeneratedToken>,
    variant: &str,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    expression.extend(map_err_tokens(variant)?);
    expression.push(GeneratedToken::alone('?'));
    Ok(expression)
}

fn map_err_tokens(variant: &str) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    let mut tokens = vec![GeneratedToken::alone('.'), GeneratedToken::word("map_err")];
    let path = vec![
        GeneratedToken::word(REFUSAL),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(variant),
    ];
    tokens.push(group(GeneratedDelimiter::Parenthesis, path)?);
    Ok(tokens)
}

fn option_or(
    mut expression: Vec<GeneratedToken>,
    variant: &str,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    path.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(path)
}

fn call_local(
    name: &str,
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    call(vec![GeneratedToken::word(name)], arguments)
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

fn method_call(local: &str, method: &str) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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
) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
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

fn visibility_crate() -> Result<GeneratedToken, MutationRenderRefusal> {
    group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word("crate")],
    )
    .map_err(|_| MutationRenderRefusal::Unbounded)
}

fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, MutationRenderRefusal> {
    shell_group(delimiter, tokens).map_err(|_| MutationRenderRefusal::Unbounded)
}

fn roster(tokens: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, MutationRenderRefusal> {
    shell_roster(tokens).map_err(|_| MutationRenderRefusal::Unbounded)
}

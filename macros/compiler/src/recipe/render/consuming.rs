//! Opaque resource wrappers and runtime-validated methods over informed transition rows.

use super::tokens::{public, typestate_variant};
use crate::recipe::{
    ConsumingMethod, ConsumingParameter, ConsumingProjection, ProjectionError, Recipe,
    RecipeRelationRow, RecipeTransitionEffect, RecipeVocabulary,
};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, absolute_path, call, comma_many,
    consuming_receiver, decorated, documentation, function_item, function_signature,
    generic_parameters, group, implementation, method_call, named_field, named_struct, result_type,
    shared_receiver, typed_parameter,
};

const PHASE: &str = "__MacroonzPhase";
const RESOURCE: &str = "__macroonz_resource";
const ERROR: &str = "__macroonz_error";

pub(super) fn items(
    recipe: &Recipe,
    vocabulary: &RecipeVocabulary,
    config: &ConsumingProjection,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut items = wrapper(config)?;
    items.extend(readback(config)?);
    items.extend(debug(config)?);
    let Some(relation) = recipe.transition_relation() else {
        return Err(missing());
    };
    for member in vocabulary.members().members() {
        let mut methods = restore(vocabulary, member.name_token(), config)?;
        for row in relation
            .rows()
            .filter(|row| row.left() == member.spelling())
        {
            let Some(method) = config
                .methods()
                .find(|method| method.event() == row.right())
            else {
                return Err(missing());
            };
            methods.extend(transition(vocabulary, config, row, method)?);
        }
        items.extend(implementation(
            Vec::new(),
            parts(config.parameters()),
            None,
            wrapper_type(config, member.name_token().clone()),
            parts(config.predicates()),
            methods,
        )?);
    }
    Ok(items)
}

fn missing() -> ProjectionError {
    ProjectionError::Render(crate::render::RenderError::NothingRendered)
}

fn parts<'a>(values: impl Iterator<Item = &'a GeneratedTree>) -> Vec<Vec<GeneratedToken>> {
    values.map(|value| value.tokens().to_vec()).collect()
}

fn generic_declarations(config: &ConsumingProjection) -> Vec<Vec<GeneratedToken>> {
    let mut parameters = parts(config.parameters());
    parameters.push(vec![GeneratedToken::word(PHASE)]);
    parameters
}

fn wrapper_type(config: &ConsumingProjection, phase: GeneratedToken) -> Vec<GeneratedToken> {
    let mut arguments = parts(config.arguments());
    arguments.push(vec![phase]);
    let mut tokens = vec![config.wrapper_token().clone()];
    tokens.extend(generic_parameters(arguments));
    tokens
}

fn wrapper(config: &ConsumingProjection) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut phantom = absolute_path(&["core", "marker", "PhantomData"]);
    phantom.extend(generic_parameters(vec![vec![GeneratedToken::word(PHASE)]]));
    Ok(decorated(
        vec![documentation(
            "An owned resource carrying a runtime-validated phase witness.",
        )?],
        public(),
        named_struct(
            config.wrapper_token().clone(),
            generic_declarations(config),
            parts(config.predicates()),
            vec![
                named_field(
                    GeneratedToken::word(RESOURCE),
                    config.resource().kind().tokens().to_vec(),
                ),
                named_field(GeneratedToken::word("__macroonz_phase"), phantom),
            ],
        )?,
    ))
}

fn readback(config: &ConsumingProjection) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let resource = vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(RESOURCE),
    ];
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(config.resource().kind().tokens().iter().cloned());
    let mut value = vec![GeneratedToken::alone('&')];
    value.extend(resource.clone());
    let mut methods = method(
        "Reads the caller-owned resource without minting a new phase witness.",
        GeneratedToken::word("resource"),
        vec![shared_receiver(Vec::new())],
        borrowed,
        value,
    )?;
    methods.extend(method(
        "Consumes this witness and returns its caller-owned resource.",
        GeneratedToken::word("into_resource"),
        vec![consuming_receiver()],
        config.resource().kind().tokens().to_vec(),
        resource,
    )?);
    implementation(
        Vec::new(),
        generic_declarations(config),
        None,
        wrapper_type(config, GeneratedToken::word(PHASE)),
        parts(config.predicates()),
        methods,
    )
    .map_err(ProjectionError::Tokens)
}

fn debug(config: &ConsumingProjection) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut formatter = vec![GeneratedToken::alone('&'), GeneratedToken::word("mut")];
    formatter.extend(absolute_path(&["core", "fmt", "Formatter"]));
    formatter.extend(generic_parameters(vec![vec![
        GeneratedToken::joint('\''),
        GeneratedToken::word("_"),
    ]]));
    let signature = function_signature(
        Vec::new(),
        GeneratedToken::word("fmt"),
        vec![
            shared_receiver(Vec::new()),
            typed_parameter(vec![GeneratedToken::word("formatter")], formatter),
        ],
        Vec::new(),
        Some(absolute_path(&["core", "fmt", "Result"])),
        Vec::new(),
    )?;
    let body = method_call(
        vec![GeneratedToken::word("formatter")],
        "write_str",
        vec![GeneratedToken::text(config.wrapper())],
    )?;
    implementation(
        Vec::new(),
        generic_declarations(config),
        Some(absolute_path(&["core", "fmt", "Debug"])),
        wrapper_type(config, GeneratedToken::word(PHASE)),
        parts(config.predicates()),
        function_item(signature, body)?,
    )
    .map_err(ProjectionError::Tokens)
}

fn parameter(value: &ConsumingParameter) -> Vec<GeneratedToken> {
    typed_parameter(
        vec![value.name_token().clone()],
        value.kind().tokens().to_vec(),
    )
}

fn method(
    sentence: &str,
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    result: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(decorated(
        vec![documentation(sentence)?],
        public(),
        function_item(
            function_signature(
                Vec::new(),
                name,
                parameters,
                Vec::new(),
                Some(result),
                Vec::new(),
            )?,
            body,
        )?,
    ))
}

fn failure_type(config: &ConsumingProjection) -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(vec![group(
        GeneratedDelimiter::Parenthesis,
        comma_many(vec![
            config.resource().kind().tokens().to_vec(),
            config.refusal().tokens().to_vec(),
        ]),
    )?])
}

fn restored(
    config: &ConsumingProjection,
    resource: GeneratedToken,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut constructor = vec![config.wrapper_token().clone()];
    constructor.push(group(
        GeneratedDelimiter::Brace,
        comma_many(vec![
            named_field(GeneratedToken::word(RESOURCE), vec![resource]),
            named_field(
                GeneratedToken::word("__macroonz_phase"),
                absolute_path(&["core", "marker", "PhantomData"]),
            ),
        ]),
    )?);
    call(
        absolute_path(&["core", "result", "Result", "Ok"]),
        constructor,
    )
    .map_err(ProjectionError::Tokens)
}

fn validate(
    vocabulary: &RecipeVocabulary,
    phase: &GeneratedToken,
    config: &ConsumingProjection,
    resource: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    call(
        vec![group(
            GeneratedDelimiter::Parenthesis,
            config.validator().tokens().to_vec(),
        )?],
        comma_many(vec![
            vec![config.runtime().name_token().clone()],
            resource,
            typestate_variant(vocabulary.name_token(), phase),
        ]),
    )
    .map_err(ProjectionError::Tokens)
}

fn refuse(
    expression: Vec<GeneratedToken>,
    resource: GeneratedToken,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut condition = vec![GeneratedToken::word("if"), GeneratedToken::word("let")];
    condition.extend(call(
        absolute_path(&["core", "result", "Result", "Err"]),
        vec![GeneratedToken::word(ERROR)],
    )?);
    condition.push(GeneratedToken::alone('='));
    condition.extend(expression);
    let pair = group(
        GeneratedDelimiter::Parenthesis,
        comma_many(vec![vec![resource], vec![GeneratedToken::word(ERROR)]]),
    )?;
    let mut body = vec![GeneratedToken::word("return")];
    body.extend(call(
        absolute_path(&["core", "result", "Result", "Err"]),
        vec![pair],
    )?);
    body.push(GeneratedToken::alone(';'));
    condition.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(condition)
}

fn validated(
    vocabulary: &RecipeVocabulary,
    phase: &GeneratedToken,
    config: &ConsumingProjection,
    resource: Vec<GeneratedToken>,
    returned: GeneratedToken,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let expression = validate(vocabulary, phase, config, resource)?;
    let mut closure_tokens = vec![GeneratedToken::alone('|')];
    closure_tokens.extend(parameter(config.runtime()));
    closure_tokens.extend([
        GeneratedToken::alone('|'),
        group(GeneratedDelimiter::Brace, expression)?,
    ]);
    let closure = group(GeneratedDelimiter::Parenthesis, closure_tokens)?;
    let mut statement = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("__macroonz_validation"),
        GeneratedToken::alone(':'),
    ];
    statement.extend(result_type(
        vec![group(GeneratedDelimiter::Parenthesis, Vec::new())?],
        config.refusal().tokens().to_vec(),
    ));
    statement.push(GeneratedToken::alone('='));
    statement.extend(call(
        vec![closure],
        vec![config.runtime().name_token().clone()],
    )?);
    statement.push(GeneratedToken::alone(';'));
    statement.extend(refuse(
        vec![GeneratedToken::word("__macroonz_validation")],
        returned,
    )?);
    Ok(statement)
}

fn restore(
    vocabulary: &RecipeVocabulary,
    phase: &GeneratedToken,
    config: &ConsumingProjection,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let resource = config.resource().name_token();
    let mut body = validated(
        vocabulary,
        phase,
        config,
        vec![GeneratedToken::alone('&'), resource.clone()],
        resource.clone(),
    )?;
    body.extend(restored(config, resource.clone())?);
    method(
        "Restores this phase only after the caller's runtime validator admits the resource.",
        GeneratedToken::word("restore"),
        vec![parameter(config.runtime()), parameter(config.resource())],
        result_type(vec![GeneratedToken::word("Self")], failure_type(config)?),
        body,
    )
}

fn transition(
    vocabulary: &RecipeVocabulary,
    config: &ConsumingProjection,
    row: &RecipeRelationRow,
    binding: &ConsumingMethod,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let Some((_, target, effect)) = row.payload().transition_parts() else {
        return Err(missing());
    };
    let mut parameters = vec![consuming_receiver(), parameter(config.runtime())];
    if let Some(payload) = binding.payload() {
        parameters.push(parameter(payload));
    }
    let mut body = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("mut"),
        GeneratedToken::word(RESOURCE),
        GeneratedToken::alone('='),
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(RESOURCE),
        GeneratedToken::alone(';'),
    ];
    body.extend([
        GeneratedToken::word("let"),
        config.resource().name_token().clone(),
        GeneratedToken::alone('='),
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
        GeneratedToken::word(RESOURCE),
        GeneratedToken::alone(';'),
    ]);
    let input = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::alone('*'),
        config.resource().name_token().clone(),
    ];
    body.extend(validated(
        vocabulary,
        row.left_name_token(),
        config,
        input.clone(),
        GeneratedToken::word(RESOURCE),
    )?);
    body.extend(execute(vocabulary, target, config, effect)?);
    body.extend(validated(
        vocabulary,
        target,
        config,
        input,
        GeneratedToken::word(RESOURCE),
    )?);
    body.extend(restored(config, GeneratedToken::word(RESOURCE))?);
    method(
        "Consumes one admitted source witness, executes its declared effect and validates the target phase.",
        binding.name_token().clone(),
        parameters,
        result_type(wrapper_type(config, target.clone()), failure_type(config)?),
        body,
    )
}

fn execute(
    vocabulary: &RecipeVocabulary,
    target: &GeneratedToken,
    config: &ConsumingProjection,
    effect: &RecipeTransitionEffect,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let (parameters, arguments) = match effect {
        RecipeTransitionEffect::Path(_) => (Vec::new(), Vec::new()),
        RecipeTransitionEffect::ExactRust { .. } => {
            let mut resource_kind = vec![GeneratedToken::alone('&'), GeneratedToken::word("mut")];
            resource_kind.extend(config.resource().kind().tokens().iter().cloned());
            (
                comma_many(vec![
                    parameter(config.runtime()),
                    typed_parameter(vec![config.resource().name_token().clone()], resource_kind),
                ]),
                comma_many(vec![
                    vec![config.runtime().name_token().clone()],
                    vec![config.resource().name_token().clone()],
                ]),
            )
        }
    };
    let effect = match effect {
        RecipeTransitionEffect::Path(path) => call(path.tokens().to_vec(), Vec::new())?,
        RecipeTransitionEffect::ExactRust {
            target_binding,
            body,
        } => {
            let mut tokens = vec![
                GeneratedToken::word("let"),
                target_binding.clone(),
                GeneratedToken::alone('='),
            ];
            tokens.extend(typestate_variant(vocabulary.name_token(), target));
            tokens.push(GeneratedToken::alone(';'));
            tokens.extend(body.tokens().iter().cloned());
            tokens
        }
    };
    let mut closure = vec![if parameters.is_empty() {
        GeneratedToken::joint('|')
    } else {
        GeneratedToken::alone('|')
    }];
    closure.extend(parameters);
    closure.extend([
        GeneratedToken::alone('|'),
        group(GeneratedDelimiter::Brace, effect)?,
    ]);
    let invocation = call(
        vec![group(GeneratedDelimiter::Parenthesis, closure)?],
        arguments,
    )?;
    let mut statement = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("__macroonz_effect"),
        GeneratedToken::alone(':'),
    ];
    statement.extend(result_type(
        vec![GeneratedToken::word("_")],
        config.refusal().tokens().to_vec(),
    ));
    statement.push(GeneratedToken::alone('='));
    statement.extend(invocation);
    statement.push(GeneratedToken::alone(';'));
    statement.extend(refuse(
        vec![GeneratedToken::word("__macroonz_effect")],
        GeneratedToken::word(RESOURCE),
    )?);
    Ok(statement)
}

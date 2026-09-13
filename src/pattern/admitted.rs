//! Checked construction and readback composed into the existing published-pattern grammar.

use crate::compiler::stamp::{
    DECLARED_REACH, Fragment, Part, Pattern, Seat, Seating, StampError, TRANSPORTED_REACH,
};
use crate::compiler::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, attribute, call, consuming_receiver,
    decorated, documentation, function_item, function_signature, generic_parameters, group,
    implementation, metavariable, method_call, result_type, shared_receiver, tuple_struct,
    typed_parameter, use_item,
};

/// Composes the admitted-newtype stamp pattern with one caller-declared generic shape.
///
/// # Errors
///
/// Returns the stamp owner's refusal when the composed pattern exceeds token bounds or contains invalid offered tokens.
pub fn admitted_newtype(
    parameters: &[GeneratedTree],
    arguments: &[GeneratedTree],
    predicates: &[GeneratedTree],
) -> Result<Pattern, StampError> {
    Pattern::declared(
        "Declares a nominal wrapper whose construction calls the supplied admission function.",
        grammar()?,
        GeneratedTree::assembled(body(parameters, arguments, predicates)?)?,
    )
}

fn grammar() -> Result<Vec<Part>, StampError> {
    let mut parts = vec![
        Part::Reach,
        seat("name", Fragment::Identifier)?,
        literal(vec![GeneratedToken::word("in")])?,
        seat("home", Fragment::Identifier)?,
        literal(vec![GeneratedToken::alone(';')])?,
    ];
    for (clause, name, fragment) in [
        ("layout", "layout", Fragment::Attribute),
        ("value", "inner", Fragment::Type),
        ("refusal", "refusal", Fragment::Type),
        ("admit", "admit", Fragment::Path),
    ] {
        parts.extend([
            literal(vec![GeneratedToken::word(clause)])?,
            seat(name, fragment)?,
            literal(vec![GeneratedToken::alone(';')])?,
        ]);
    }
    Ok(parts)
}

fn seat(name: &str, fragment: Fragment) -> Result<Part, StampError> {
    Ok(Part::Seat(Seat::declared(name, Seating::One(fragment))?))
}

fn literal(tokens: Vec<GeneratedToken>) -> Result<Part, StampError> {
    Ok(Part::Literal(GeneratedTree::assembled(tokens)?))
}

fn runs(parts: &[GeneratedTree]) -> Vec<Vec<GeneratedToken>> {
    parts.iter().map(|part| part.tokens().to_vec()).collect()
}

fn body(
    parameters: &[GeneratedTree],
    arguments: &[GeneratedTree],
    predicates: &[GeneratedTree],
) -> Result<Vec<GeneratedToken>, StampError> {
    let layout = attribute(vec![
        GeneratedToken::word("repr"),
        group(GeneratedDelimiter::Parenthesis, metavariable("layout"))?,
    ])?;
    let mut items = decorated(
        vec![
            documentation("A nominal value admitted by the caller's constructor.")?,
            layout,
        ],
        metavariable(TRANSPORTED_REACH),
        tuple_struct(
            GeneratedToken::word("Value"),
            runs(parameters),
            vec![metavariable("inner")],
            runs(predicates),
        )?,
    );
    let mut value_type = vec![GeneratedToken::word("Value")];
    value_type.extend(generic_parameters(runs(arguments)));
    items.extend(implementation(
        Vec::new(),
        runs(parameters),
        None,
        value_type,
        runs(predicates),
        methods()?,
    )?);
    // A macro metavariable is two tokens, so this module name is not a one-token item name.
    let mut tokens = vec![GeneratedToken::word("mod")];
    tokens.extend(metavariable("home"));
    tokens.push(group(GeneratedDelimiter::Brace, items)?);
    let mut path = vec![
        GeneratedToken::word("self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
    ];
    path.extend(metavariable("home"));
    path.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("Value"),
        GeneratedToken::word("as"),
    ]);
    path.extend(metavariable("name"));
    tokens.extend(decorated(
        Vec::new(),
        metavariable(DECLARED_REACH),
        use_item(path, None),
    ));
    Ok(tokens)
}

fn method(
    note: &str,
    name: &str,
    parameters: Vec<Vec<GeneratedToken>>,
    result: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, StampError> {
    Ok(decorated(
        vec![documentation(note)?],
        vec![GeneratedToken::word("pub")],
        function_item(
            function_signature(
                Vec::new(),
                GeneratedToken::word(name),
                parameters,
                Vec::new(),
                Some(result),
                Vec::new(),
            )?,
            body,
        )?,
    ))
}

fn methods() -> Result<Vec<GeneratedToken>, StampError> {
    let input = vec![GeneratedToken::word("value")];
    let mut body = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("admitted"),
        GeneratedToken::alone(':'),
    ];
    body.extend(result_type(metavariable("inner"), metavariable("refusal")));
    body.push(GeneratedToken::alone('='));
    body.extend(call(
        vec![group(
            GeneratedDelimiter::Parenthesis,
            metavariable("admit"),
        )?],
        input.clone(),
    )?);
    body.push(GeneratedToken::alone(';'));
    body.extend(method_call(
        vec![GeneratedToken::word("admitted")],
        "map",
        vec![GeneratedToken::word("Self")],
    )?);
    let mut methods = method(
        "Calls the supplied admission function before constructing this nominal value.",
        "try_new",
        vec![typed_parameter(input, metavariable("inner"))],
        result_type(vec![GeneratedToken::word("Self")], metavariable("refusal")),
        body,
    )?;
    let field = vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::number(0),
    ];
    let mut borrowed_type = vec![GeneratedToken::alone('&')];
    borrowed_type.extend(metavariable("inner"));
    let mut borrowed_value = vec![GeneratedToken::alone('&')];
    borrowed_value.extend(field.clone());
    methods.extend(method(
        "Borrows the stored representation without supplying mutable access.",
        "as_inner",
        vec![shared_receiver(Vec::new())],
        borrowed_type,
        borrowed_value,
    )?);
    methods.extend(method(
        "Consumes this nominal value and returns its stored representation.",
        "into_inner",
        vec![consuming_receiver()],
        metavariable("inner"),
        field,
    )?);
    Ok(methods)
}

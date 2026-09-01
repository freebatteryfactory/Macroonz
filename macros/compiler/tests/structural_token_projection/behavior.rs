//! Conventional behavior shells compared with independent tokens and real Rust compilation.

use macroonz_compiler::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, attribute, bound_path, call, constant,
    consuming_receiver, decorated, enumeration, exclusive_receiver, function, function_item,
    function_signature, group, match_arm, match_expression, method_call, method_chain,
    pinned_receiver, result_type, shared_receiver, tuple_struct, typed_parameter, unit_struct,
    unit_variant,
};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);

fn name(spelling: &str) -> GeneratedToken {
    GeneratedToken::word(spelling)
}

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn lifetime(spelling: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('\''), GeneratedToken::word(spelling)]
}

fn const_parameter(spelling: &str, kind: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("const"),
        GeneratedToken::word(spelling),
        GeneratedToken::alone(':'),
        GeneratedToken::word(kind),
    ]
}

fn derived(names: &[&str]) -> Result<Vec<GeneratedToken>, ()> {
    let mut traits = Vec::new();
    for (position, item) in names.iter().enumerate() {
        if position > 0 {
            traits.push(GeneratedToken::alone(','));
        }
        traits.push(GeneratedToken::word(item));
    }
    attribute(vec![
        GeneratedToken::word("derive"),
        group(GeneratedDelimiter::Parenthesis, traits).map_err(|_refusal| ())?,
    ])
    .map_err(|_refusal| ())
}

fn method(
    qualifiers: Vec<GeneratedToken>,
    method_name: &str,
    parameters: Vec<Vec<GeneratedToken>>,
    generics: Vec<Vec<GeneratedToken>>,
    result: Option<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ()> {
    let signature = function_signature(
        qualifiers,
        name(method_name),
        parameters,
        generics,
        result,
        predicates,
    )
    .map_err(|_refusal| ())?;
    function_item(signature, body).map_err(|_refusal| ())
}

fn public_method(
    qualifiers: Vec<GeneratedToken>,
    method_name: &str,
    parameters: Vec<Vec<GeneratedToken>>,
    generics: Vec<Vec<GeneratedToken>>,
    result: Option<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ()> {
    Ok(decorated(
        Vec::new(),
        public(),
        method(
            qualifiers,
            method_name,
            parameters,
            generics,
            result,
            predicates,
            body,
        )?,
    ))
}

fn canonical_behavior() -> Result<GeneratedTree, ()> {
    let pattern_zero = vec![GeneratedToken::number(0)];
    let guard = vec![
        GeneratedToken::word("N"),
        GeneratedToken::alone('>'),
        GeneratedToken::number(0),
    ];
    let body = match_expression(
        vec![GeneratedToken::word("input")],
        vec![
            match_arm(pattern_zero, Some(guard), ok_number(1)?),
            match_arm(vec![GeneratedToken::word("_")], None, refusal("Absent")?),
        ],
    )
    .map_err(|_refusal| ())?;
    let result = result_type(
        vec![GeneratedToken::word("u8")],
        vec![GeneratedToken::word("Refusal")],
    );
    let predicate = vec![
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("u8"),
                GeneratedToken::alone(';'),
                GeneratedToken::word("N"),
            ],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::alone(':'),
        GeneratedToken::word("Sized"),
    ];
    let item = public_method(
        vec![GeneratedToken::word("async")],
        "decide",
        vec![
            exclusive_receiver(lifetime("a")),
            typed_parameter(
                vec![GeneratedToken::word("input")],
                vec![GeneratedToken::word("u8")],
            ),
        ],
        vec![lifetime("a"), const_parameter("N", "usize")],
        Some(result),
        vec![predicate],
        body,
    )?;
    GeneratedTree::assembled(item).map_err(|_refusal| ())
}

fn raw_canonical_behavior() -> Result<GeneratedTree, ()> {
    let parameters = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        GeneratedToken::word("mut"),
        GeneratedToken::word("self"),
        GeneratedToken::alone(','),
        GeneratedToken::word("input"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("u8"),
    ];
    let first_arm = vec![
        GeneratedToken::number(0),
        GeneratedToken::word("if"),
        GeneratedToken::word("N"),
        GeneratedToken::alone('>'),
        GeneratedToken::number(0),
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
        GeneratedToken::word("Ok"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::number(1)],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::alone(','),
    ];
    let second_arm = vec![
        GeneratedToken::word("_"),
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
        GeneratedToken::word("Err"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![
                GeneratedToken::word("Refusal"),
                GeneratedToken::joint(':'),
                GeneratedToken::alone(':'),
                GeneratedToken::word("Absent"),
            ],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::alone(','),
    ];
    let mut arms = first_arm;
    arms.extend(second_arm);
    let body = vec![
        GeneratedToken::word("match"),
        GeneratedToken::word("input"),
        group(GeneratedDelimiter::Brace, arms).map_err(|_refusal| ())?,
    ];
    let item = vec![
        GeneratedToken::word("pub"),
        GeneratedToken::word("async"),
        GeneratedToken::word("fn"),
        GeneratedToken::word("decide"),
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        GeneratedToken::alone(','),
        GeneratedToken::word("const"),
        GeneratedToken::word("N"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("usize"),
        GeneratedToken::alone('>'),
        group(GeneratedDelimiter::Parenthesis, parameters).map_err(|_refusal| ())?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("core"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("result"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("Result"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("u8"),
        GeneratedToken::alone(','),
        GeneratedToken::word("Refusal"),
        GeneratedToken::alone('>'),
        GeneratedToken::word("where"),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("u8"),
                GeneratedToken::alone(';'),
                GeneratedToken::word("N"),
            ],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::alone(':'),
        GeneratedToken::word("Sized"),
        group(GeneratedDelimiter::Brace, body).map_err(|_refusal| ())?,
    ];
    GeneratedTree::assembled(item).map_err(|_refusal| ())
}

fn ok_number(value: u64) -> Result<Vec<GeneratedToken>, ()> {
    call(
        vec![GeneratedToken::word("Ok")],
        vec![GeneratedToken::number(value)],
    )
    .map_err(|_refusal| ())
}

fn refusal(variant: &str) -> Result<Vec<GeneratedToken>, ()> {
    call(
        vec![GeneratedToken::word("Err")],
        vec![
            GeneratedToken::word("Refusal"),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word(variant),
        ],
    )
    .map_err(|_refusal| ())
}

fn behavior_suite() -> Result<GeneratedTree, ()> {
    let mut tokens = model_declarations()?;
    tokens.extend(model_implementation()?);
    tokens.extend(free_behavior()?);
    GeneratedTree::assembled(tokens).map_err(|_refusal| ())
}

fn model_declarations() -> Result<Vec<GeneratedToken>, ()> {
    let mut tokens = decorated(
        vec![derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?],
        public(),
        enumeration(
            name("Refusal"),
            Vec::new(),
            Vec::new(),
            vec![unit_variant(name("Absent"))],
        )
        .map_err(|_refusal| ())?,
    );
    tokens.extend(decorated(
        vec![derived(&["Debug", "PartialEq", "Eq"])?],
        public(),
        tuple_struct(
            name("Model"),
            Vec::new(),
            vec![decorated(
                Vec::new(),
                public(),
                vec![GeneratedToken::word("u8")],
            )],
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
    ));
    tokens.extend(decorated(
        vec![derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?],
        public(),
        enumeration(
            name("Flag"),
            Vec::new(),
            Vec::new(),
            vec![unit_variant(name("Off")), unit_variant(name("On"))],
        )
        .map_err(|_refusal| ())?,
    ));
    for marker in ["Closed", "Open"] {
        tokens.extend(decorated(
            vec![derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?],
            public(),
            unit_struct(name(marker), Vec::new(), Vec::new()),
        ));
    }
    tokens.extend(decorated(
        vec![derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?],
        public(),
        tuple_struct(
            name("Stage"),
            vec![vec![name("Marker")]],
            vec![decorated(Vec::new(), public(), vec![name("Marker")])],
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
    ));
    tokens.extend(decorated(
        Vec::new(),
        public(),
        constant(
            "LOOKUP",
            lookup_kind().map_err(|_refusal| ())?,
            lookup_value().map_err(|_refusal| ())?,
        ),
    ));
    Ok(tokens)
}

fn lookup_kind() -> Result<Vec<GeneratedToken>, macroonz_compiler::Overflow> {
    Ok(vec![
        GeneratedToken::alone('&'),
        group(
            GeneratedDelimiter::Bracket,
            vec![group(
                GeneratedDelimiter::Parenthesis,
                vec![
                    GeneratedToken::word("u8"),
                    GeneratedToken::alone(','),
                    GeneratedToken::word("u8"),
                ],
            )?],
        )?,
    ])
}

fn lookup_value() -> Result<Vec<GeneratedToken>, macroonz_compiler::Overflow> {
    Ok(vec![
        GeneratedToken::alone('&'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![
                        GeneratedToken::number(0),
                        GeneratedToken::alone(','),
                        GeneratedToken::number(1),
                    ],
                )?,
                GeneratedToken::alone(','),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![
                        GeneratedToken::number(1),
                        GeneratedToken::alone(','),
                        GeneratedToken::number(2),
                    ],
                )?,
            ],
        )?,
    ])
}

fn model_implementation() -> Result<Vec<GeneratedToken>, ()> {
    let mut methods = constructor_and_views()?;
    methods.extend(pinned_and_custom_receivers()?);
    methods.extend(conversions_and_async()?);
    Ok(vec![
        GeneratedToken::word("impl"),
        GeneratedToken::word("Model"),
        group(GeneratedDelimiter::Brace, methods).map_err(|_refusal| ())?,
    ])
}

fn constructor_and_views() -> Result<Vec<GeneratedToken>, ()> {
    let value = typed_parameter(
        vec![GeneratedToken::word("value")],
        vec![GeneratedToken::word("u8")],
    );
    let mut methods = public_method(
        Vec::new(),
        "new",
        vec![value.clone()],
        Vec::new(),
        Some(vec![GeneratedToken::word("Self")]),
        Vec::new(),
        vec![
            GeneratedToken::word("Self"),
            group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("value")],
            )
            .map_err(|_refusal| ())?,
        ],
    )?;
    methods.extend(public_method(
        Vec::new(),
        "into_inner",
        vec![consuming_receiver()],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        field("self", 0),
    )?);
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(lifetime("a"));
    borrowed.push(GeneratedToken::word("u8"));
    let mut borrowed_body = vec![GeneratedToken::alone('&')];
    borrowed_body.extend(field("self", 0));
    methods.extend(public_method(
        Vec::new(),
        "borrowed",
        vec![shared_receiver(lifetime("a"))],
        vec![lifetime("a")],
        Some(borrowed),
        Vec::new(),
        borrowed_body,
    )?);
    let mut set_body = field("self", 0);
    set_body.extend([
        GeneratedToken::alone('='),
        GeneratedToken::word("value"),
        GeneratedToken::alone(';'),
    ]);
    methods.extend(public_method(
        Vec::new(),
        "set",
        vec![exclusive_receiver(Vec::new()), value],
        Vec::new(),
        None,
        Vec::new(),
        set_body,
    )?);
    let mut reborrowed_kind = vec![GeneratedToken::alone('&')];
    reborrowed_kind.extend(lifetime("a"));
    reborrowed_kind.extend([GeneratedToken::word("mut"), GeneratedToken::word("u8")]);
    let mut reborrowed_body = vec![GeneratedToken::alone('&'), GeneratedToken::word("mut")];
    reborrowed_body.extend(field("self", 0));
    methods.extend(public_method(
        Vec::new(),
        "reborrowed",
        vec![exclusive_receiver(lifetime("a"))],
        vec![lifetime("a")],
        Some(reborrowed_kind),
        Vec::new(),
        reborrowed_body,
    )?);
    Ok(methods)
}

fn pinned_and_custom_receivers() -> Result<Vec<GeneratedToken>, ()> {
    let mut pinned_body = method_chain(vec![GeneratedToken::word("self")], &["as_ref", "get_ref"])
        .map_err(|_refusal| ())?;
    pinned_body.extend([GeneratedToken::alone('.'), GeneratedToken::number(0)]);
    let mut methods = public_method(
        Vec::new(),
        "pinned",
        vec![pinned_receiver(Vec::new())],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        pinned_body,
    )?;
    let custom = typed_parameter(
        vec![GeneratedToken::word("self")],
        vec![
            GeneratedToken::word("Box"),
            GeneratedToken::alone('<'),
            GeneratedToken::word("Self"),
            GeneratedToken::alone('>'),
        ],
    );
    methods.extend(public_method(
        Vec::new(),
        "boxed",
        vec![custom],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        field("self", 0),
    )?);
    Ok(methods)
}

fn conversions_and_async() -> Result<Vec<GeneratedToken>, ()> {
    let checked = match_expression(
        vec![GeneratedToken::word("value")],
        vec![
            match_arm(vec![GeneratedToken::number(0)], None, refusal("Absent")?),
            match_arm(
                vec![GeneratedToken::word("_")],
                None,
                call(
                    vec![GeneratedToken::word("Ok")],
                    vec![
                        GeneratedToken::word("Self"),
                        group(
                            GeneratedDelimiter::Parenthesis,
                            vec![GeneratedToken::word("value")],
                        )
                        .map_err(|_refusal| ())?,
                    ],
                )
                .map_err(|_refusal| ())?,
            ),
        ],
    )
    .map_err(|_refusal| ())?;
    let mut methods = public_method(
        Vec::new(),
        "checked",
        vec![typed_parameter(
            vec![GeneratedToken::word("value")],
            vec![GeneratedToken::word("u8")],
        )],
        Vec::new(),
        Some(result_type(
            vec![GeneratedToken::word("Self")],
            vec![GeneratedToken::word("Refusal")],
        )),
        Vec::new(),
        checked,
    )?;
    methods.extend(public_method(
        Vec::new(),
        "as_u16",
        vec![consuming_receiver()],
        Vec::new(),
        Some(vec![GeneratedToken::word("u16")]),
        Vec::new(),
        call(
            vec![
                GeneratedToken::word("u16"),
                GeneratedToken::joint(':'),
                GeneratedToken::alone(':'),
                GeneratedToken::word("from"),
            ],
            field("self", 0),
        )
        .map_err(|_refusal| ())?,
    )?);
    methods.extend(public_method(
        vec![GeneratedToken::word("async")],
        "ready",
        vec![shared_receiver(Vec::new())],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        field("self", 0),
    )?);
    Ok(methods)
}

fn free_behavior() -> Result<Vec<GeneratedToken>, ()> {
    let mut tokens = typestate_transition()?;
    tokens.extend(exhaustive_function()?);
    tokens.extend(sparse_function()?);
    tokens.extend(const_generic_function()?);
    tokens.extend(external_effect_function()?);
    tokens.extend(explicit_unsafe_function()?);
    Ok(tokens)
}

fn typestate_transition() -> Result<Vec<GeneratedToken>, ()> {
    let mut body = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("_"),
        GeneratedToken::alone('='),
        GeneratedToken::word("stage"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("Stage"),
    ];
    body.push(
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("Open")],
        )
        .map_err(|_refusal| ())?,
    );
    public_method(
        Vec::new(),
        "open_stage",
        vec![typed_parameter(
            vec![GeneratedToken::word("stage")],
            vec![
                GeneratedToken::word("Stage"),
                GeneratedToken::alone('<'),
                GeneratedToken::word("Closed"),
                GeneratedToken::alone('>'),
            ],
        )],
        Vec::new(),
        Some(vec![
            GeneratedToken::word("Stage"),
            GeneratedToken::alone('<'),
            GeneratedToken::word("Open"),
            GeneratedToken::alone('>'),
        ]),
        Vec::new(),
        body,
    )
}

fn exhaustive_function() -> Result<Vec<GeneratedToken>, ()> {
    let body = match_expression(
        vec![GeneratedToken::word("flag")],
        vec![
            match_arm(
                path_variant("Flag", "Off"),
                None,
                vec![GeneratedToken::number(0)],
            ),
            match_arm(
                path_variant("Flag", "On"),
                None,
                vec![GeneratedToken::number(1)],
            ),
        ],
    )
    .map_err(|_refusal| ())?;
    public_method(
        Vec::new(),
        "exhaustive",
        vec![typed_parameter(
            vec![GeneratedToken::word("flag")],
            vec![GeneratedToken::word("Flag")],
        )],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        body,
    )
}

fn sparse_function() -> Result<Vec<GeneratedToken>, ()> {
    let body = match_expression(
        vec![GeneratedToken::word("value")],
        vec![
            match_arm(vec![GeneratedToken::number(1)], None, ok_number(2)?),
            match_arm(vec![GeneratedToken::word("_")], None, refusal("Absent")?),
        ],
    )
    .map_err(|_refusal| ())?;
    public_method(
        Vec::new(),
        "sparse",
        vec![typed_parameter(
            vec![GeneratedToken::word("value")],
            vec![GeneratedToken::word("u8")],
        )],
        Vec::new(),
        Some(result_type(
            vec![GeneratedToken::word("u8")],
            vec![GeneratedToken::word("Refusal")],
        )),
        Vec::new(),
        body,
    )
}

fn const_generic_function() -> Result<Vec<GeneratedToken>, ()> {
    let array = vec![
        GeneratedToken::alone('&'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("u8"),
                GeneratedToken::alone(';'),
                GeneratedToken::word("N"),
            ],
        )
        .map_err(|_refusal| ())?,
    ];
    let body = method_call(
        method_call(
            method_call(
                vec![GeneratedToken::word("values")],
                "get",
                vec![GeneratedToken::word("index")],
            )
            .map_err(|_refusal| ())?,
            "copied",
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
        "ok_or",
        path_variant("Refusal", "Absent"),
    )
    .map_err(|_refusal| ())?;
    public_method(
        Vec::new(),
        "choose",
        vec![
            typed_parameter(
                vec![GeneratedToken::word("index")],
                vec![GeneratedToken::word("usize")],
            ),
            typed_parameter(vec![GeneratedToken::word("values")], array),
        ],
        vec![const_parameter("N", "usize")],
        Some(result_type(
            vec![GeneratedToken::word("u8")],
            vec![GeneratedToken::word("Refusal")],
        )),
        Vec::new(),
        body,
    )
}

fn external_effect_function() -> Result<Vec<GeneratedToken>, ()> {
    let body = call(
        bound_path("crate", &["external"]),
        vec![GeneratedToken::word("value")],
    )
    .map_err(|_refusal| ())?;
    public_method(
        Vec::new(),
        "effect",
        vec![typed_parameter(
            vec![GeneratedToken::word("value")],
            vec![GeneratedToken::word("u8")],
        )],
        Vec::new(),
        Some(vec![GeneratedToken::word("u8")]),
        Vec::new(),
        body,
    )
}

fn explicit_unsafe_function() -> Result<Vec<GeneratedToken>, ()> {
    let operation = method_call(vec![GeneratedToken::word("pointer")], "read", Vec::new())
        .map_err(|_refusal| ())?;
    let body = vec![
        GeneratedToken::word("unsafe"),
        group(GeneratedDelimiter::Brace, operation).map_err(|_refusal| ())?,
    ];
    Ok(decorated(
        vec![
            macroonz_compiler::documentation(
                "# Safety\n\nThe pointer must be valid for one byte read.",
            )
            .map_err(|_refusal| ())?,
        ],
        public(),
        method(
            vec![GeneratedToken::word("unsafe")],
            "read_raw",
            vec![typed_parameter(
                vec![GeneratedToken::word("pointer")],
                vec![
                    GeneratedToken::alone('*'),
                    GeneratedToken::word("const"),
                    GeneratedToken::word("u8"),
                ],
            )],
            Vec::new(),
            Some(vec![GeneratedToken::word("u8")]),
            Vec::new(),
            body,
        )?,
    ))
}

fn field(root: &str, index: u64) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(root),
        GeneratedToken::alone('.'),
        GeneratedToken::number(index),
    ]
}

fn path_variant(kind: &str, variant: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(kind),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(variant),
    ]
}

fn specimen_path(extension: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "macroonz_behavior_{}_{ordinal}{extension}",
        std::process::id()
    ))
}

fn compile(source: &str, extra: &[&str]) -> Result<std::process::Output, String> {
    let source_path = specimen_path(".rs");
    let executable = specimen_path(std::env::consts::EXE_SUFFIX);
    std::fs::create_dir_all(env!("CARGO_TARGET_TMPDIR")).map_err(|error| error.to_string())?;
    std::fs::write(&source_path, source).map_err(|error| error.to_string())?;
    let mut command = Command::new("rustup");
    command
        .arg("run")
        .arg("1.98.0")
        .arg("rustc")
        .arg(&source_path)
        .arg("--edition=2024")
        .arg("-o")
        .arg(&executable);
    command.args(extra);
    let output = command.output().map_err(|error| error.to_string())?;
    drop(std::fs::remove_file(&source_path));
    if output.status.success() {
        let executed = Command::new(&executable)
            .output()
            .map_err(|error| error.to_string())?;
        drop(std::fs::remove_file(&executable));
        if !executed.status.success() {
            return Err(String::from_utf8_lossy(&executed.stderr).into_owned());
        }
    } else {
        drop(std::fs::remove_file(&executable));
    }
    Ok(output)
}

const SUITE_ASSERTIONS: &str = r"
fn external(value: u8) -> u8 { value.saturating_add(1) }

fn main() {
    let mut model = Model::new(3);
    assert_eq!(*model.borrowed(), 3);
    model.set(4);
    *model.reborrowed() = 5;
    assert_eq!(model.into_inner(), 5);
    assert_eq!(Model::checked(5).map(Model::into_inner), Ok(5));
    assert!(matches!(Model::checked(0), Err(Refusal::Absent)));
    assert_eq!(Model::new(7).as_u16(), 7);
    let mut pinned = Box::pin(Model::new(8));
    assert_eq!(Model::pinned(pinned.as_mut()), 8);
    assert_eq!(Box::new(Model::new(9)).boxed(), 9);
    drop(Model::new(10).ready());
    let Stage(Open) = open_stage(Stage(Closed));
    assert_eq!(exhaustive(Flag::Off), 0);
    assert_eq!(exhaustive(Flag::On), 1);
    assert_eq!(sparse(1), Ok(2));
    assert!(matches!(sparse(0), Err(Refusal::Absent)));
    assert_eq!(choose(1, &[4, 6]), Ok(6));
    assert_eq!(effect(3), 4);
    assert_eq!(LOOKUP, &[(0, 1), (1, 2)]);
}
";

/// Claim: the behavior composers own conventional punctuation without changing caller material.
/// Subject: one async method carrying lifetime and const generics, an exclusive receiver, a typed parameter and result, one where predicate, and guarded and sparse match arms.
/// Population: the complete public signature and match operations.
/// Hostile control: an independently assembled raw token tree fixes every canonical token rather than calling the paved operations twice.
/// Evidence ceiling: the compiler crossing separately decides whether one complete generated behavior suite is lawful Rust.
#[test]
fn behavior_composers_match_independent_canonical_tokens() -> Result<(), ()> {
    let paved = canonical_behavior()?;
    let raw = raw_canonical_behavior()?;
    assert_eq!(paved.canonical_bytes(), raw.canonical_bytes());
    assert_eq!(paved.inspected(), raw.inspected());
    Ok(())
}

/// Claim: the pre-existing narrow function convenience is one composition over the complete behavior kernel rather than a second implementation.
/// Subject: one private function with a flattened typed parameter run, exact result and exact body.
/// Population: the complete narrow convenience surface.
/// Hostile control: the expected tree is assembled from independently written canonical tokens.
/// Evidence ceiling: qualifiers, generics, receivers and where predicates use the complete signature road instead.
#[test]
fn narrow_function_convenience_matches_independent_tokens() -> Result<(), ()> {
    let paved = function(
        "read",
        vec![name("value"), GeneratedToken::alone(':'), name("u8")],
        vec![name("u8")],
        vec![name("value")],
    )
    .map_err(|_refusal| ())?;
    let raw = vec![
        name("fn"),
        name("read"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![name("value"), GeneratedToken::alone(':'), name("u8")],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        name("u8"),
        group(GeneratedDelimiter::Brace, vec![name("value")]).map_err(|_refusal| ())?,
    ];
    assert_eq!(paved, raw);
    Ok(())
}

/// Claim: one behavior kernel expresses the complete conventional behavior family under stable Rust 1.98.
/// Subject: constructor, borrowed and exclusive views, consuming, pinned and custom receivers, sync and async methods, fallible and infallible conversions, exhaustive and sparse matches, const-generic dispatch, lookup table, external effect path, and an exact unsafe declaration plus discharge block.
/// Population: every behavior family named by this crossing in one compiled and executed source.
/// Hostile control: separate rustc refusals plant an illegal receiver, missing exhaustive arm, unresolved external effect path and missing unsafe discharge block.
/// Evidence ceiling: this establishes one generated local Windows specimen, not every caller fragment or a safety proof.
#[test]
fn behavior_composers_emit_executable_rust_1_98() -> Result<(), String> {
    let mut source = String::from("#![deny(unsafe_op_in_unsafe_fn)]\n");
    source.push_str(
        &behavior_suite()
            .map_err(|()| "the behavior suite refused".to_owned())?
            .inspected(),
    );
    source.push_str(SUITE_ASSERTIONS);
    let output = compile(&source, &[])?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// Claim: safe presets never infer an unsafe qualifier or block.
/// Subject: every receiver preset and one unqualified function signature.
/// Population: consuming, shared, exclusive and pinned presets plus the safe function head.
/// Hostile control: the executable suite separately proves an explicit caller-authored unsafe qualifier and discharge block can be carried.
/// Evidence ceiling: this is an exact generated-token reading, not a whole-program unsafe scan.
#[test]
fn safe_behavior_presets_emit_no_unsafe_boundary() -> Result<(), ()> {
    let signature = function_signature(
        Vec::new(),
        name("safe"),
        vec![
            consuming_receiver(),
            shared_receiver(Vec::new()),
            exclusive_receiver(Vec::new()),
            pinned_receiver(Vec::new()),
        ],
        Vec::new(),
        None,
        Vec::new(),
    )
    .map_err(|_refusal| ())?;
    let inspected = GeneratedTree::assembled(signature)
        .map_err(|_refusal| ())?
        .inspected();
    assert!(!inspected.contains("unsafe"));
    Ok(())
}

/// Claim: every conventional receiver preset owns one exact Rust spelling and nothing else.
/// Subject: consuming, shared, exclusive and pinned receivers carrying one explicit lifetime where the form admits one.
/// Population: the complete receiver preset vocabulary.
/// Hostile control: each expectation is an independently written generated-token run rather than another paved composition.
/// Evidence ceiling: custom receivers remain exact caller-authored typed-parameter material and are compiled in the behavior suite instead.
#[test]
fn receiver_presets_match_independent_exact_tokens() {
    let stated_lifetime = lifetime("a");
    assert_eq!(consuming_receiver(), vec![GeneratedToken::word("self")]);
    assert_eq!(
        shared_receiver(stated_lifetime.clone()),
        vec![
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("a"),
            GeneratedToken::word("self"),
        ]
    );
    assert_eq!(
        exclusive_receiver(stated_lifetime.clone()),
        vec![
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("a"),
            GeneratedToken::word("mut"),
            GeneratedToken::word("self"),
        ]
    );
    assert_eq!(
        pinned_receiver(stated_lifetime),
        vec![
            GeneratedToken::word("self"),
            GeneratedToken::alone(':'),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word("core"),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word("pin"),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word("Pin"),
            GeneratedToken::alone('<'),
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("a"),
            GeneratedToken::word("mut"),
            GeneratedToken::word("Self"),
            GeneratedToken::alone('>'),
        ]
    );
}

/// Claim: Rustc retains receiver, exhaustiveness and unsafe-discharge authority over generated behavior.
/// Subject: four complete hostile source files compiled independently of the positive suite.
/// Population: wrong receiver type, omitted enum arm, unresolved ordinary effect path, and an unsafe operation outside an explicit block.
/// Hostile control: the complete positive suite compiles and executes through the same rustc command topology.
/// Evidence ceiling: the assertion fixes stable diagnostic classes without claiming diagnostics from future compilers.
#[test]
fn rustc_refuses_illegal_behavior_contracts() -> Result<(), String> {
    for (source, arguments, anchor) in [
        (
            "struct Model; impl Model { fn wrong(self: String) {} } fn main() {}",
            &[][..],
            "E0307",
        ),
        (
            "enum Flag { Off, On } fn read(flag: Flag) -> u8 { match flag { Flag::Off => 0 } } fn main() {}",
            &[][..],
            "E0004",
        ),
        (
            "fn effect(value: u8) -> u8 { crate::missing(value) } fn main() {}",
            &[][..],
            "E0425",
        ),
        (
            "unsafe fn read(pointer: *const u8) -> u8 { pointer.read() } fn main() {}",
            &["-Dunsafe-op-in-unsafe-fn"][..],
            "E0133",
        ),
    ] {
        let output = compile(source, arguments)?;
        if output.status.success() {
            return Err(format!(
                "hostile compilation unexpectedly succeeded for {anchor}"
            ));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains(anchor) {
            return Err(format!("expected {anchor} in:\n{stderr}"));
        }
    }
    Ok(())
}

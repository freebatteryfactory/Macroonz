//! Conventional namespace and data items compared with independently assembled canonical tokens.

use crate::support::observe_rustc;
use macroonz_compiler::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, absolute_path, attribute, constant,
    decorated, documentation, enumeration, generic_parameters, group, inline_module, named_field,
    named_struct, named_variant, tuple_struct, tuple_variant, type_alias, unit_struct,
    unit_variant, use_item, where_clause,
};

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn name(spelling: &str) -> GeneratedToken {
    GeneratedToken::word(spelling)
}

fn lifetime(name: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('\''), GeneratedToken::word(name)]
}

fn bounded_type(name: &str, bound: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(name),
        GeneratedToken::alone(':'),
        GeneratedToken::word(bound),
    ]
}

fn const_parameter(name: &str, kind: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("const"),
        GeneratedToken::word(name),
        GeneratedToken::alone(':'),
        GeneratedToken::word(kind),
    ]
}

fn borrowed(name: &str, kind: &str) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('&')];
    tokens.extend(lifetime(name));
    tokens.push(GeneratedToken::word(kind));
    tokens
}

fn array(kind: &str, length: &str) -> Result<Vec<GeneratedToken>, ()> {
    group(
        GeneratedDelimiter::Bracket,
        vec![
            GeneratedToken::word(kind),
            GeneratedToken::alone(';'),
            GeneratedToken::word(length),
        ],
    )
    .map(|array| vec![array])
    .map_err(|_refusal| ())
}

fn generic(root: &str, argument: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(root),
        GeneratedToken::alone('<'),
        GeneratedToken::word(argument),
        GeneratedToken::alone('>'),
    ]
}

fn paved_module() -> Result<GeneratedTree, ()> {
    let mut items = paved_prelude()?;
    items.extend(paved_record()?);
    items.extend(paved_choice()?);
    let module = decorated(
        vec![documentation("Generated namespace and data forms.").map_err(|_refusal| ())?],
        public(),
        inline_module(name("model"), items).map_err(|_refusal| ())?,
    );
    GeneratedTree::assembled(module).map_err(|_refusal| ())
}

fn paved_prelude() -> Result<Vec<GeneratedToken>, ()> {
    let mut items = decorated(
        Vec::new(),
        public(),
        use_item(
            absolute_path(&["core", "marker", "PhantomData"]),
            Some(name("MarkerData")),
        ),
    );
    items.extend(decorated(
        Vec::new(),
        public(),
        constant(
            "LIMIT",
            vec![GeneratedToken::word("usize")],
            vec![GeneratedToken::number(4)],
        ),
    ));
    items.extend(decorated(
        Vec::new(),
        public(),
        type_alias(
            name("Borrowed"),
            vec![lifetime("a"), vec![GeneratedToken::word("T")]],
            borrowed("a", "T"),
            Vec::new(),
        ),
    ));
    items.extend(decorated(
        Vec::new(),
        public(),
        unit_struct(name("Marker"), Vec::new(), Vec::new()),
    ));
    items.extend(decorated(
        Vec::new(),
        public(),
        unit_struct(
            GeneratedToken::raw_identifier("type"),
            Vec::new(),
            Vec::new(),
        ),
    ));
    items.extend(decorated(
        Vec::new(),
        public(),
        tuple_struct(
            name("Newtype"),
            vec![vec![GeneratedToken::word("T")]],
            vec![decorated(
                Vec::new(),
                public(),
                vec![GeneratedToken::word("T")],
            )],
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
    ));
    Ok(items)
}

fn paved_record() -> Result<Vec<GeneratedToken>, ()> {
    Ok(decorated(
        vec![
            attribute(vec![
                GeneratedToken::word("derive"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("Clone")],
                )
                .map_err(|_refusal| ())?,
            ])
            .map_err(|_refusal| ())?,
        ],
        public(),
        named_struct(
            name("Record"),
            vec![
                lifetime("a"),
                bounded_type("T", "Clone"),
                const_parameter("N", "usize"),
            ],
            vec![{
                let mut predicate = vec![GeneratedToken::word("T"), GeneratedToken::alone(':')];
                predicate.extend(lifetime("a"));
                predicate
            }],
            vec![
                decorated(
                    Vec::new(),
                    public(),
                    named_field(name("borrowed"), borrowed("a", "T")),
                ),
                decorated(
                    Vec::new(),
                    public(),
                    named_field(name("bytes"), array("u8", "N")?),
                ),
                decorated(
                    Vec::new(),
                    public(),
                    named_field(name("marker"), generic("MarkerData", "T")),
                ),
            ],
        )
        .map_err(|_refusal| ())?,
    ))
}

fn paved_choice() -> Result<Vec<GeneratedToken>, ()> {
    Ok(decorated(
        Vec::new(),
        public(),
        enumeration(
            name("Choice"),
            vec![vec![GeneratedToken::word("T")]],
            Vec::new(),
            vec![
                unit_variant(name("None")),
                tuple_variant(name("One"), vec![vec![GeneratedToken::word("T")]])
                    .map_err(|_refusal| ())?,
                named_variant(
                    name("Named"),
                    vec![named_field(name("value"), vec![GeneratedToken::word("T")])],
                )
                .map_err(|_refusal| ())?,
            ],
        )
        .map_err(|_refusal| ())?,
    ))
}

fn raw_module() -> Result<GeneratedTree, ()> {
    let mut items = raw_prelude()?;
    items.extend(raw_record()?);
    items.extend(raw_choice()?);
    let module = vec![
        GeneratedToken::alone('#'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("doc"),
                GeneratedToken::alone('='),
                GeneratedToken::text("Generated namespace and data forms."),
            ],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::word("pub"),
        GeneratedToken::word("mod"),
        GeneratedToken::word("model"),
        group(GeneratedDelimiter::Brace, items).map_err(|_refusal| ())?,
    ];
    GeneratedTree::assembled(module).map_err(|_refusal| ())
}

fn raw_prelude() -> Result<Vec<GeneratedToken>, ()> {
    let mut items = vec![GeneratedToken::word("pub"), GeneratedToken::word("use")];
    items.extend(absolute_path(&["core", "marker", "PhantomData"]));
    items.extend([
        GeneratedToken::word("as"),
        GeneratedToken::word("MarkerData"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("const"),
        GeneratedToken::word("LIMIT"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("usize"),
        GeneratedToken::alone('='),
        GeneratedToken::number(4),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("type"),
        GeneratedToken::word("Borrowed"),
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        GeneratedToken::alone(','),
        GeneratedToken::word("T"),
        GeneratedToken::alone('>'),
        GeneratedToken::alone('='),
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        GeneratedToken::word("T"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("struct"),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("struct"),
        GeneratedToken::raw_identifier("type"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("struct"),
        GeneratedToken::word("Newtype"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("T"),
        GeneratedToken::alone('>'),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("pub"), GeneratedToken::word("T")],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::alone(';'),
    ]);
    Ok(items)
}

fn raw_record() -> Result<Vec<GeneratedToken>, ()> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("derive"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("Clone")],
                )
                .map_err(|_refusal| ())?,
            ],
        )
        .map_err(|_refusal| ())?,
        GeneratedToken::word("pub"),
        GeneratedToken::word("struct"),
        GeneratedToken::word("Record"),
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        GeneratedToken::alone(','),
        GeneratedToken::word("T"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("Clone"),
        GeneratedToken::alone(','),
        GeneratedToken::word("const"),
        GeneratedToken::word("N"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("usize"),
        GeneratedToken::alone('>'),
        GeneratedToken::word("where"),
        GeneratedToken::word("T"),
        GeneratedToken::alone(':'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("a"),
        group(
            GeneratedDelimiter::Brace,
            vec![
                GeneratedToken::word("pub"),
                GeneratedToken::word("borrowed"),
                GeneratedToken::alone(':'),
                GeneratedToken::alone('&'),
                GeneratedToken::joint('\''),
                GeneratedToken::word("a"),
                GeneratedToken::word("T"),
                GeneratedToken::alone(','),
                GeneratedToken::word("pub"),
                GeneratedToken::word("bytes"),
                GeneratedToken::alone(':'),
                group(
                    GeneratedDelimiter::Bracket,
                    vec![
                        GeneratedToken::word("u8"),
                        GeneratedToken::alone(';'),
                        GeneratedToken::word("N"),
                    ],
                )
                .map_err(|_refusal| ())?,
                GeneratedToken::alone(','),
                GeneratedToken::word("pub"),
                GeneratedToken::word("marker"),
                GeneratedToken::alone(':'),
                GeneratedToken::word("MarkerData"),
                GeneratedToken::alone('<'),
                GeneratedToken::word("T"),
                GeneratedToken::alone('>'),
            ],
        )
        .map_err(|_refusal| ())?,
    ])
}

fn raw_choice() -> Result<Vec<GeneratedToken>, ()> {
    Ok(vec![
        GeneratedToken::word("pub"),
        GeneratedToken::word("enum"),
        GeneratedToken::word("Choice"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("T"),
        GeneratedToken::alone('>'),
        group(
            GeneratedDelimiter::Brace,
            vec![
                GeneratedToken::word("None"),
                GeneratedToken::alone(','),
                GeneratedToken::word("One"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("T")],
                )
                .map_err(|_refusal| ())?,
                GeneratedToken::alone(','),
                GeneratedToken::word("Named"),
                group(
                    GeneratedDelimiter::Brace,
                    vec![
                        GeneratedToken::word("value"),
                        GeneratedToken::alone(':'),
                        GeneratedToken::word("T"),
                    ],
                )
                .map_err(|_refusal| ())?,
            ],
        )
        .map_err(|_refusal| ())?,
    ])
}

const SPECIMEN_ASSERTIONS: &str = r"
fn main() {
    let value = 7_u8;
    let borrowed: model::Borrowed<'_, u8> = &value;
    let record = model::Record::<u8, 2> {
        borrowed,
        bytes: [1, 2],
        marker: core::marker::PhantomData,
    };
    let newtype = model::Newtype(3_u8);
    let choice = model::Choice::Named { value: 5_u8 };
    let _marker = model::Marker;
    let _raw = model::r#type;
    assert_eq!(model::LIMIT, 4);
    assert_eq!(*record.borrowed, 7);
    assert_eq!(record.bytes, [1, 2]);
    assert_eq!(newtype.0, 3);
    assert!(matches!(choice, model::Choice::Named { value: 5 }));
}
";

/// Claim: conventional namespace and data composers own punctuation without changing caller material.
/// Subject: one module containing every Task 12 namespace and data family.
/// Population: attributes, documentation, visibility, explicit reexport, constant, alias, marker, newtype, phantom field, lifetime/type/const generics, bounds, where clause, named fields, enum, and unit/tuple/named variants.
/// Hostile control: an independently assembled raw token tree fixes every canonical token rather than calling the paved operations twice.
/// Evidence ceiling: the compiler still decides whether the resulting Rust is lawful at a real use site.
#[test]
fn namespace_and_data_composers_match_independent_canonical_tokens() -> Result<(), ()> {
    let paved = paved_module()?;
    let raw = raw_module()?;
    assert_eq!(paved.canonical_bytes(), raw.canonical_bytes());
    assert_eq!(paved.inspected(), raw.inspected());
    Ok(())
}

/// Claim: every conventional namespace and data item emitted by the paved composers is accepted and executable under stable Rust 1.98.
/// Subject: the same complete generated module fixed by the independent canonical vector.
/// Population: explicit reexport, constant, alias, raw marker, newtype, named generic record, phantom field, lifetime/type/const parameters, bound, where predicate, and all enum variant forms.
/// Hostile control: the canonical-vector lane separately catches punctuation movement before this compiler crossing can normalize it.
/// Evidence ceiling: this establishes one generated item composition on the local Windows host, not arbitrary caller fragments or later language syntax.
#[test]
fn namespace_and_data_composers_emit_executable_rust_1_98() -> Result<(), String> {
    let mut source = paved_module()
        .map_err(|()| "the paved module refused".to_owned())?
        .inspected();
    source.push_str(SPECIMEN_ASSERTIONS);
    let output = observe_rustc("namespace-data", &source, &[])?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// Claim: empty generic and where rosters add no invisible syntax.
/// Subject: both total empty composers.
/// Population: the complete empty input for each operation.
/// Hostile control: one nonempty input to each operation must produce its explicit delimiter or keyword.
/// Evidence ceiling: this fixes mechanical absence rather than a semantic default for any item.
#[test]
fn empty_generic_and_where_rosters_are_exactly_empty() {
    assert!(generic_parameters(Vec::new()).is_empty());
    assert!(where_clause(Vec::new()).is_empty());
    assert_eq!(
        generic_parameters(vec![vec![GeneratedToken::word("T")]]),
        vec![
            GeneratedToken::alone('<'),
            GeneratedToken::word("T"),
            GeneratedToken::alone('>'),
        ]
    );
    assert_eq!(
        where_clause(vec![bounded_type("T", "Clone")]),
        vec![
            GeneratedToken::word("where"),
            GeneratedToken::word("T"),
            GeneratedToken::alone(':'),
            GeneratedToken::word("Clone"),
        ]
    );
}

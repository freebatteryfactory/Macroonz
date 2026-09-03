//! Conventional trait and implementation shells compared with independent tokens and real Rust compilation.

use crate::support::observe_rustc;
use macroonz_compiler::{
    Empty, GeneratedDelimiter, GeneratedToken, GeneratedTree, KeyedRoster, NonEmptyError, Overflow,
    associated_constant, associated_function, associated_type, decorated, documentation,
    exclusive_receiver, function_signature, group, implementation, keyed_roster_items,
    trait_declaration, tuple_struct, typed_parameter,
};

const MEMBER_LIMIT: usize = 4;

fn name(spelling: &str) -> GeneratedToken {
    GeneratedToken::word(spelling)
}

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn lifetime(spelling: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('\''), GeneratedToken::word(spelling)]
}

fn generic(spelling: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::word(spelling)]
}

fn generic_path(name: &str, arguments: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone('<')];
    for (position, argument) in arguments.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone(','));
        }
        tokens.extend(argument);
    }
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

fn bound(kind: &str, trait_name: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(kind),
        GeneratedToken::alone(':'),
        GeneratedToken::word(trait_name),
    ]
}

fn trait_contract() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut items = associated_type(name("Item"), Vec::new(), Vec::new(), None, Vec::new());
    items.extend(associated_constant(
        name("LIMIT"),
        vec![name("usize")],
        None,
    ));
    items.extend(associated_function(
        function_signature(
            Vec::new(),
            name("read"),
            vec![vec![GeneratedToken::alone('&'), name("self")]],
            Vec::new(),
            Some(vec![
                name("Self"),
                GeneratedToken::joint(':'),
                GeneratedToken::alone(':'),
                name("Item"),
            ]),
            Vec::new(),
        )?,
        None,
    )?);
    Ok(decorated(
        Vec::new(),
        public(),
        trait_declaration(
            Vec::new(),
            name("Contract"),
            vec![generic("T")],
            vec![generic("Send"), generic("Sync")],
            vec![bound("T", "Copy")],
            items,
        )?,
    ))
}

fn contract_implementation() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut items = associated_type(
        name("Item"),
        Vec::new(),
        Vec::new(),
        Some(generic("T")),
        Vec::new(),
    );
    items.extend(associated_constant(
        name("LIMIT"),
        vec![name("usize")],
        Some(vec![GeneratedToken::number(1)]),
    ));
    items.extend(associated_function(
        function_signature(
            Vec::new(),
            name("read"),
            vec![vec![GeneratedToken::alone('&'), name("self")]],
            Vec::new(),
            Some(vec![
                name("Self"),
                GeneratedToken::joint(':'),
                GeneratedToken::alone(':'),
                name("Item"),
            ]),
            Vec::new(),
        )?,
        Some(vec![
            name("self"),
            GeneratedToken::alone('.'),
            GeneratedToken::number(0),
        ]),
    )?);
    implementation(
        Vec::new(),
        vec![generic("T")],
        Some(generic_path("Contract", vec![generic("T")])),
        generic_path("Packet", vec![generic("T")]),
        vec![bound("T", "Copy")],
        items,
    )
}

fn raw_contract() -> Result<Vec<GeneratedToken>, Overflow> {
    let items = vec![
        name("type"),
        name("Item"),
        GeneratedToken::alone(';'),
        name("const"),
        name("LIMIT"),
        GeneratedToken::alone(':'),
        name("usize"),
        GeneratedToken::alone(';'),
        name("fn"),
        name("read"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::alone('&'), name("self")],
        )?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        name("Self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name("Item"),
        GeneratedToken::alone(';'),
    ];
    Ok(vec![
        name("pub"),
        name("trait"),
        name("Contract"),
        GeneratedToken::alone('<'),
        name("T"),
        GeneratedToken::alone('>'),
        GeneratedToken::alone(':'),
        name("Send"),
        GeneratedToken::alone('+'),
        name("Sync"),
        name("where"),
        name("T"),
        GeneratedToken::alone(':'),
        name("Copy"),
        group(GeneratedDelimiter::Brace, items)?,
    ])
}

fn raw_contract_implementation() -> Result<Vec<GeneratedToken>, Overflow> {
    let items = vec![
        name("type"),
        name("Item"),
        GeneratedToken::alone('='),
        name("T"),
        GeneratedToken::alone(';'),
        name("const"),
        name("LIMIT"),
        GeneratedToken::alone(':'),
        name("usize"),
        GeneratedToken::alone('='),
        GeneratedToken::number(1),
        GeneratedToken::alone(';'),
        name("fn"),
        name("read"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::alone('&'), name("self")],
        )?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        name("Self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name("Item"),
        group(
            GeneratedDelimiter::Brace,
            vec![
                name("self"),
                GeneratedToken::alone('.'),
                GeneratedToken::number(0),
            ],
        )?,
    ];
    Ok(vec![
        name("impl"),
        GeneratedToken::alone('<'),
        name("T"),
        GeneratedToken::alone('>'),
        name("Contract"),
        GeneratedToken::alone('<'),
        name("T"),
        GeneratedToken::alone('>'),
        name("for"),
        name("Packet"),
        GeneratedToken::alone('<'),
        name("T"),
        GeneratedToken::alone('>'),
        name("where"),
        name("T"),
        GeneratedToken::alone(':'),
        name("Copy"),
        group(GeneratedDelimiter::Brace, items)?,
    ])
}

fn advanced_suite() -> Result<GeneratedTree, ()> {
    let mut tokens = decorated(
        Vec::new(),
        public(),
        tuple_struct(
            name("Packet"),
            vec![generic("T")],
            vec![decorated(Vec::new(), public(), generic("T"))],
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
    );
    tokens.extend(decorated(
        Vec::new(),
        public(),
        tuple_struct(
            name("Array"),
            vec![
                generic("T"),
                vec![
                    name("const"),
                    name("N"),
                    GeneratedToken::alone(':'),
                    name("usize"),
                ],
            ],
            vec![decorated(
                Vec::new(),
                public(),
                vec![
                    group(
                        GeneratedDelimiter::Bracket,
                        vec![name("T"), GeneratedToken::alone(';'), name("N")],
                    )
                    .map_err(|_refusal| ())?,
                ],
            )],
            Vec::new(),
        )
        .map_err(|_refusal| ())?,
    ));
    tokens.extend(advanced_trait().map_err(|_refusal| ())?);
    tokens.extend(advanced_trait_implementation().map_err(|_refusal| ())?);
    tokens.extend(inherent_packet().map_err(|_refusal| ())?);
    tokens.extend(blanket_adapter().map_err(|_refusal| ())?);
    tokens.extend(const_generic_inherent().map_err(|_refusal| ())?);
    GeneratedTree::assembled(tokens).map_err(|_refusal| ())
}

fn advanced_trait() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut items = associated_type(
        name("Item"),
        vec![lifetime("b")],
        vec![generic("Copy")],
        None,
        vec![
            vec![
                name("Self"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("b"),
            ],
            vec![
                name("Self"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("a"),
            ],
            vec![
                name("T"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("b"),
            ],
        ],
    );
    items.extend(associated_constant(
        name("WIDTH"),
        vec![name("usize")],
        None,
    ));
    let result = vec![
        name("Self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name("Item"),
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        name("a"),
        GeneratedToken::alone('>'),
    ];
    items.extend(associated_function(
        function_signature(
            vec![name("unsafe")],
            name("read"),
            vec![exclusive_receiver(lifetime("a"))],
            Vec::new(),
            Some(result),
            Vec::new(),
        )?,
        None,
    )?);
    Ok(decorated(
        vec![documentation(
            "# Safety\n\nImplementations must uphold the caller-declared read contract.",
        )?],
        public(),
        trait_declaration(
            vec![name("unsafe")],
            name("View"),
            vec![lifetime("a"), generic("T")],
            vec![generic("Send")],
            vec![vec![
                name("T"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("a"),
            ]],
            items,
        )?,
    ))
}

fn advanced_trait_implementation() -> Result<Vec<GeneratedToken>, Overflow> {
    let item_value = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        name("b"),
        name("T"),
    ];
    let mut items = associated_type(
        name("Item"),
        vec![lifetime("b")],
        Vec::new(),
        Some(item_value),
        vec![
            vec![
                name("Self"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("b"),
            ],
            vec![
                name("Self"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("a"),
            ],
            vec![
                name("T"),
                GeneratedToken::alone(':'),
                GeneratedToken::joint('\''),
                name("b"),
            ],
        ],
    );
    items.extend(associated_constant(
        name("WIDTH"),
        vec![name("usize")],
        Some(vec![GeneratedToken::number(1)]),
    ));
    let result = vec![
        name("Self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name("Item"),
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        name("a"),
        GeneratedToken::alone('>'),
    ];
    items.extend(associated_function(
        function_signature(
            vec![name("unsafe")],
            name("read"),
            vec![exclusive_receiver(lifetime("a"))],
            Vec::new(),
            Some(result),
            Vec::new(),
        )?,
        Some(vec![
            GeneratedToken::alone('&'),
            name("self"),
            GeneratedToken::alone('.'),
            GeneratedToken::number(0),
        ]),
    )?);
    implementation(
        vec![name("unsafe")],
        vec![lifetime("a"), generic("T")],
        Some(generic_path("View", vec![lifetime("a"), generic("T")])),
        generic_path("Packet", vec![generic("T")]),
        vec![vec![
            name("T"),
            GeneratedToken::alone(':'),
            name("Copy"),
            GeneratedToken::alone('+'),
            name("Send"),
            GeneratedToken::alone('+'),
            GeneratedToken::joint('\''),
            name("a"),
        ]],
        items,
    )
}

fn inherent_packet() -> Result<Vec<GeneratedToken>, Overflow> {
    let signature = function_signature(
        Vec::new(),
        name("new"),
        vec![typed_parameter(generic("value"), generic("T"))],
        Vec::new(),
        Some(vec![name("Self")]),
        Vec::new(),
    )?;
    let method = decorated(
        Vec::new(),
        public(),
        associated_function(
            signature,
            Some(vec![
                name("Self"),
                group(GeneratedDelimiter::Parenthesis, generic("value"))?,
            ]),
        )?,
    );
    implementation(
        Vec::new(),
        vec![generic("T")],
        None,
        generic_path("Packet", vec![generic("T")]),
        Vec::new(),
        method,
    )
}

fn blanket_adapter() -> Result<Vec<GeneratedToken>, Overflow> {
    let signature = function_signature(
        Vec::new(),
        name("from"),
        vec![typed_parameter(generic("value"), generic("T"))],
        Vec::new(),
        Some(vec![name("Self")]),
        Vec::new(),
    )?;
    let body = vec![
        name("Self"),
        group(GeneratedDelimiter::Parenthesis, generic("value"))?,
    ];
    implementation(
        Vec::new(),
        vec![generic("T")],
        Some(generic_path("From", vec![generic("T")])),
        generic_path("Packet", vec![generic("T")]),
        Vec::new(),
        associated_function(signature, Some(body))?,
    )
}

fn const_generic_inherent() -> Result<Vec<GeneratedToken>, Overflow> {
    let signature = function_signature(
        vec![name("const")],
        name("len"),
        vec![vec![GeneratedToken::alone('&'), name("self")]],
        Vec::new(),
        Some(vec![name("usize")]),
        Vec::new(),
    )?;
    implementation(
        Vec::new(),
        vec![
            generic("T"),
            vec![
                name("const"),
                name("N"),
                GeneratedToken::alone(':'),
                name("usize"),
            ],
        ],
        None,
        generic_path("Array", vec![generic("T"), generic("N")]),
        Vec::new(),
        associated_function(signature, Some(generic("N")))?,
    )
}

/// Claim: trait and implementation composers own conventional punctuation without changing caller material.
/// Subject: one generic trait and one generic implementation carrying an associated type, constant and required or provided method.
/// Population: every complete public trait and implementation operation.
/// Hostile control: independently assembled raw token trees fix each canonical token.
/// Evidence ceiling: the executable crossing separately establishes advanced Rust legality.
#[test]
fn trait_and_implementation_composers_match_independent_tokens() -> Result<(), ()> {
    let mut paved = trait_contract().map_err(|_refusal| ())?;
    paved.extend(contract_implementation().map_err(|_refusal| ())?);
    let mut raw = raw_contract().map_err(|_refusal| ())?;
    raw.extend(raw_contract_implementation().map_err(|_refusal| ())?);
    let paved = GeneratedTree::assembled(paved).map_err(|_refusal| ())?;
    let raw = GeneratedTree::assembled(raw).map_err(|_refusal| ())?;
    assert_eq!(paved.canonical_bytes(), raw.canonical_bytes());
    assert_eq!(paved.inspected(), raw.inspected());
    Ok(())
}

/// Claim: the implementation-set operation visits every informed row once in retained order and stops at the first refusal.
/// Subject: a three-member keyed roster.
/// Population: every callback coordinate supplied by the flat-item operation.
/// Hostile control: the second row refuses, and the third row would be visible if traversal continued.
/// Evidence ceiling: prior callbacks can have caller-owned effects and are not rolled back.
#[test]
fn implementation_sets_are_ordered_exact_and_short_circuiting() -> Result<(), ()> {
    let roster = KeyedRoster::<String, String, MEMBER_LIMIT>::new(
        vec!["first".to_owned(), "second".to_owned(), "third".to_owned()],
        Clone::clone,
    )
    .map_err(|_refusal| ())?;
    let expected = Overflow {
        capacity: 2,
        offered: 3,
    };
    let mut visited = Vec::new();
    let Err(refusal) = keyed_roster_items(&roster, |index, key, _member| {
        visited.push((index, key.clone()));
        if index == 1 {
            return Err(expected);
        }
        Ok(vec![name("impl")])
    }) else {
        return Err(());
    };
    assert_eq!(refusal.position(), 1);
    assert_eq!(refusal.cause(), NonEmptyError::Overflow(expected));
    assert_eq!(
        visited,
        vec![(0, "first".to_owned()), (1, "second".to_owned())]
    );

    let Err(missing) = keyed_roster_items(&roster, |_index, _key, _member| Ok(Vec::new())) else {
        return Err(());
    };
    assert_eq!(missing.position(), 0);
    assert_eq!(missing.cause(), NonEmptyError::Empty(Empty));
    Ok(())
}

/// Claim: one trait and implementation kernel expresses GATs, associated constants and methods, inherent and trait implementations, a blanket adapter, const generics, lifetimes, and explicit unsafe trait authority under stable Rust 1.98.
/// Subject: one generated package-local Rust source and its executable assertions.
/// Population: every required trait and implementation family in this crossing.
/// Hostile control: separate rustc refusals plant orphan, coherence, unconstrained-generic, missing-unsafe and missing-associated-item defects.
/// Evidence ceiling: this proves one Windows source crossing and does not prove safety obligations true.
#[test]
fn trait_and_implementation_composers_emit_executable_rust_1_98() -> Result<(), String> {
    let mut source = advanced_suite()
        .map_err(|()| "the advanced trait suite refused".to_owned())?
        .inspected();
    source.push_str(
        r"
fn main() {
    let packet = Packet::new(7_u8);
    assert_eq!(Packet::from(8_u8).0, 8);
    assert_eq!(Array([1_u8, 2_u8]).len(), 2);
    assert_eq!(<Packet<u8> as View<'_, u8>>::WIDTH, 1);
    let mut source = Packet(9);
    let read = unsafe { <Packet<u8> as View<'_, u8>>::read(&mut source) };
    assert_eq!(*read, 9);
    assert_eq!(packet.0, 7);
}
",
    );
    let output = observe_rustc("traits", &source, &[])?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// Claim: rustc retains orphan, coherence, generic-constraint, unsafe-implementation and associated-item completeness authority.
/// Subject: five independently compiled hostile Rust sources.
/// Population: every refusal family named by the trait and implementation acceptance denominator.
/// Hostile control: the generated advanced suite compiles and executes through the same rustc topology.
/// Evidence ceiling: diagnostic classes are fixed for stable Rust 1.98 rather than future compilers.
#[test]
fn rustc_refuses_illegal_trait_and_implementation_contracts() -> Result<(), String> {
    for (source, anchor) in [
        (
            "impl From<u8> for u16 { fn from(value: u8) -> Self { u16::from(value) } } fn main() {}",
            "E0117",
        ),
        (
            "trait Mark {} impl<T> Mark for T {} impl Mark for u8 {} fn main() {}",
            "E0119",
        ),
        (
            "struct Local; impl<T> Local { fn value() {} } fn main() {}",
            "E0207",
        ),
        (
            "unsafe trait Contract {} struct Local; impl Contract for Local {} fn main() {}",
            "E0200",
        ),
        (
            "trait Contract { type Item; const N: usize; fn read(&self); } struct Local; impl Contract for Local {} fn main() {}",
            "E0046",
        ),
    ] {
        let output = observe_rustc("traits", source, &[])?;
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

//! Every recipe-owned magnitude is derived where a closed roster exists and crossed at N-1, N, and N+1.

use super::{DOOR, bake, emitted_bytes, refusal_summary};
use macroonz_compiler::recipe::{
    CODEC_LIMIT, EVIDENCE_LIMIT, HarnessPosture, PROJECTION_CLAUSE_LIMIT, RELATION_LIMIT,
    RELATION_QUESTION_LIMIT, RELATION_ROW_LIMIT, RELATION_TABLE_LIMIT, TRANSITION_LIMIT,
    VOCABULARY_LIMIT,
};
use macroonz_compiler::{RENDERED_BYTE_LIMIT, TEXT_SOURCE_BYTE_LIMIT, TextCapture};

const SEQUENCE_REFUSAL: &str = "captured sequence carries more members than its declared magnitude";

fn assert_boundary(limit: usize, source: impl Fn(usize) -> String) -> Result<(), String> {
    let before = limit
        .checked_sub(1)
        .ok_or_else(|| String::from("a boundary must admit one earlier magnitude"))?;
    let beyond = limit
        .checked_add(1)
        .ok_or_else(|| String::from("a boundary must admit one later magnitude"))?;
    for count in [before, limit] {
        let candidate = source(count);
        let _baked = bake(candidate.as_str()).map_err(|()| {
            let reason = refusal_summary(candidate.as_str())
                .unwrap_or_else(|()| String::from("the refusal was not projected"));
            format!("{count} was refused at limit {limit}: {reason}")
        })?;
    }
    let summary = refusal_summary(source(beyond).as_str())
        .map_err(|()| format!("{beyond} did not produce a recipe refusal"))?;
    assert!(summary.contains(SEQUENCE_REFUSAL), "{summary}");
    assert!(summary.contains(limit.to_string().as_str()), "{summary}");
    Ok(())
}

#[test]
fn vocabulary_members_and_vocabulary_declarations_cross_their_exact_boundaries()
-> Result<(), String> {
    assert_boundary(VOCABULARY_LIMIT, vocabulary_member_source)?;
    assert_boundary(VOCABULARY_LIMIT, vocabulary_source)
}

#[test]
fn relation_rows_relations_and_relation_table_selections_cross_their_exact_boundaries()
-> Result<(), String> {
    assert_boundary(RELATION_ROW_LIMIT, relation_row_source)?;
    assert_boundary(RELATION_LIMIT, relation_source)?;
    assert_boundary(RELATION_TABLE_LIMIT, relation_table_source)
}

#[test]
fn transition_rows_and_codec_declarations_cross_their_exact_boundaries() -> Result<(), String> {
    assert_boundary(TRANSITION_LIMIT, transition_source)?;
    assert_boundary(CODEC_LIMIT, codec_source)
}

#[test]
fn posture_projection_and_evidence_rosters_cross_their_exact_boundaries() -> Result<(), String> {
    assert_boundary(RELATION_QUESTION_LIMIT, posture_source)?;
    assert_boundary(PROJECTION_CLAUSE_LIMIT, projection_source)?;
    assert_boundary(EVIDENCE_LIMIT, evidence_source)
}

#[test]
fn the_catalog_bearing_maximal_load_fits_the_derived_render_envelope_with_pinned_headroom()
-> Result<(), String> {
    const EXPECTED_CANONICAL_BYTES: usize = 4_548_871;

    let source = maximal_recipe_source();
    assert!(
        source.len() <= TEXT_SOURCE_BYTE_LIMIT,
        "the maximal recipe source carried {} bytes",
        source.len()
    );
    let captured = TextCapture::read(source.as_str()).map_err(|error| format!("{error:?}"))?;
    let baked = macroonz_compiler::recipe::bake(captured.input(), HarnessPosture::Available, &DOOR)
        .map_err(|error| error.summary().to_owned())?;
    let bytes = emitted_bytes(&baked).ok_or_else(|| String::from("the recipe emitted no unit"))?;
    assert_eq!(bytes.len(), EXPECTED_CANONICAL_BYTES);
    assert!(
        bytes.len() + TEXT_SOURCE_BYTE_LIMIT <= RENDERED_BYTE_LIMIT,
        "the maximal recipe left less than one complete input magnitude of headroom"
    );
    Ok(())
}

fn vocabulary_member_source(count: usize) -> String {
    let variants = comma_joined(count, |index| format!("V{index}"));
    format!(
        "pub mod subject {{ pub enum Stage {{ {variants} }} bake! {{ vocabularies {{ Stage; }}; projections {{ companions; }}; }} }}"
    )
}

fn vocabulary_source(count: usize) -> String {
    let declarations = joined(count, |index| format!("pub enum V{index} {{ A }}"));
    let names = joined(count, |index| format!("V{index};"));
    format!(
        "pub mod subject {{ {declarations} bake! {{ vocabularies {{ {names} }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_row_source(count: usize) -> String {
    let right_count = count.div_ceil(VOCABULARY_LIMIT);
    let left = comma_joined(VOCABULARY_LIMIT, |index| format!("L{index}"));
    let right = comma_joined(right_count, |index| format!("R{index}"));
    let rows = joined(count, |index| {
        format!(
            "(L{}, R{});",
            index % VOCABULARY_LIMIT,
            index.checked_div(VOCABULARY_LIMIT).unwrap_or_default()
        )
    });
    format!(
        "pub mod subject {{ pub enum Left {{ {left} }} pub enum Right {{ {right} }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ links(Left, Right) {{ {rows} }}; }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_source(count: usize) -> String {
    let relations = joined(count, |index| {
        format!("R{index}(Left, Right) {{ (A, B); }};")
    });
    format!(
        "pub mod subject {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_table_source(count: usize) -> String {
    let relations = joined(RELATION_LIMIT, |index| {
        format!("R{index}(Left, Right) {{ (A, B); }};")
    });
    let tables = joined(count, |index| format!("R{};", index % RELATION_LIMIT));
    format!(
        "pub mod subject {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ relation_tables {{ {tables} }}; }}; }} }}"
    )
}

fn transition_source(count: usize) -> String {
    let event_count = count.div_ceil(VOCABULARY_LIMIT);
    let states = comma_joined(VOCABULARY_LIMIT, |index| format!("S{index}"));
    let events = comma_joined(event_count, |index| format!("E{index}"));
    let rows = joined(count, |index| {
        format!(
            "(S{}, E{}) => S0 with(crate::effect);",
            index % VOCABULARY_LIMIT,
            index.checked_div(VOCABULARY_LIMIT).unwrap_or_default()
        )
    });
    format!(
        "pub mod subject {{ pub enum State {{ {states} }} pub enum Event {{ {events} }} bake! {{ vocabularies {{ State; Event; }}; transitions(State, Event) {{ {rows} }}; absence(refused); projections {{ dispatch; }}; }} }}"
    )
}

fn codec_source(count: usize) -> String {
    let records = joined(count, |index| {
        format!("pub struct Record{index} {{ pub value: u16 }}")
    });
    let codecs = joined(count, |index| {
        format!(
            "codec{index}(Record{index}) {{ direction(encode); refusal(Refusal{index}); assembly(assembled, total); members {{ value: u16 => count(required); }}; }};"
        )
    });
    format!(
        "pub mod subject {{ {records} bake! {{ codecs {{ {codecs} }}; projections {{ codec; }}; }} }}"
    )
}

fn posture_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "empty(allowed);",
        "repetition(allowed);",
        "membership(open, open);",
        "completeness(partial, partial);",
        "density(sparse);",
        "absence(allowed);",
        "self_relation(allowed);",
        "cycle(allowed);",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum Node {{ A }} bake! {{ vocabularies {{ Node; }}; relations {{ links(Node, Node) {{ (A, A); }}; }}; postures {{ links {{ {clauses} }}; }}; projections {{ companions; }}; }} }}"
    )
}

fn projection_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "companions;",
        "relation_tables { policy; };",
        "dispatch;",
        "compile_contract;",
        "declaration_conformance;",
        "typestate(State);",
        "codec;",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum State {{ Closed, Open }} pub enum Event {{ Go }} pub enum Right {{ B }} pub struct Ledger {{ pub value: u16 }} bake! {{ vocabularies {{ State; Event; Right; }}; transitions(State, Event) {{ (Closed, Go) => Open with(crate::effect); }}; relations {{ policy(State, Right) {{ (Closed, B); }}; }}; absence(refused); codecs {{ ledger(Ledger) {{ direction(encode); refusal(LedgerError); assembly(assembled, total); members {{ value: u16 => count(required); }}; }}; }}; projections {{ {clauses} }}; support(recipe_support); }} }}"
    )
}

fn evidence_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "trials unavailable;",
        "mutation unavailable;",
        "benchmarks unavailable;",
        "network unavailable;",
        "concurrency unavailable;",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum Stage {{ A }} bake! {{ vocabularies {{ Stage; }}; projections {{ companions; }}; evidence {{ {clauses} }}; }} }}"
    )
}

fn maximal_recipe_source() -> String {
    const STATES: &[&str] = &[
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
    ];
    const EVENTS: &[&str] = &["A", "B", "C", "D", "E", "F", "G", "H"];

    let state_variants = STATES.join(",");
    let event_variants = EVENTS.join(",");
    let other_vocabularies = joined(VOCABULARY_LIMIT - 2, |index| {
        format!("pub enum V{index}{{A}}")
    });
    let other_vocabulary_names = joined(VOCABULARY_LIMIT - 2, |index| format!("V{index};"));
    let transition_rows = joined(TRANSITION_LIMIT, |index| {
        let state_index = index.checked_div(EVENTS.len()).unwrap_or_default();
        let event_index = index.checked_rem(EVENTS.len()).unwrap_or_default();
        let state = STATES.get(state_index).copied().unwrap_or_default();
        let event = EVENTS.get(event_index).copied().unwrap_or_default();
        format!("({state},{event})=>A with(crate::effect);")
    });
    let generic_rows = "(A,A);".repeat(RELATION_ROW_LIMIT);
    let generic_relations = joined(RELATION_LIMIT - 1, |index| {
        format!("R{index}(S,E){{{generic_rows}}};")
    });
    let relation_tables = joined(RELATION_TABLE_LIMIT - 1, |index| format!("R{index};"));
    let records = joined(CODEC_LIMIT, |index| {
        format!("pub struct C{index}{{pub value:u16}}")
    });
    let codecs = joined(CODEC_LIMIT, |index| {
        format!(
            "c{index}(C{index}){{direction(encode);refusal(X{index});assembly(assembled,total);members{{value:u16=>count(required);}};}};"
        )
    });
    format!(
        r"pub mod maximal {{
            pub enum S{{{state_variants}}}
            pub enum E{{{event_variants}}}
            {other_vocabularies}
            {records}
            bake!{{
                vocabularies{{S;E;{other_vocabulary_names}}};
                transitions(S,E){{{transition_rows}}};
                relations{{{generic_relations}}};
                absence(refused);
                codecs{{{codecs}}};
                projections{{
                    companions;
                    relation_tables{{{relation_tables}}};
                    dispatch;
                    compile_contract;
                    declaration_conformance;
                    typestate(S);
                    codec;
                }};
                {MAXIMAL_EVIDENCE}
                support(recipe_support);
            }}
        }}"
    )
}

const MAXIMAL_EVIDENCE: &str = r#"
evidence {
    trials {
        support = recipe_trials_support,
        module = recipe_trials,
        table = named("bounds", "trials"),
        suite checks = named("bounds", "unit") {
            transition_answers {
                claim = named("bounds", "transition-answers"),
                subject = named("bounds", "dispatch"),
                check = named("bounds", "exact"),
                population = named("bounds", "declared-rows"),
            },
        },
    };
    mutation(S) {
        module = recipe_mutations,
        refusal = RecipeMutationRefusal,
        support = recipe_mutation_support,
        family = named("bounds", "mutation"),
        point = named("bounds", "state-order"),
        fact = named("bounds", "state-order"),
        map named("bounds", "state-order") = named("bounds", "order-held"),
        permit named("bounds", "order-held") = ["declared-order-permutation"],
    };
    benchmarks {
        support = recipe_bench_support,
        table_function = recipe_bench_table,
        table = named("bounds", "bench-table"),
        reporter = recipe_bench_reporter,
        dispatch_pace {
            workload = named("bounds", "dispatch"),
            preflight = named("bounds", "dispatch-correct"),
            planted_worse = named("bounds", "dispatch-worse"),
            complexity = named("bounds", "linear"),
            axis = [2, 4, 8],
            samples = 16,
            warmups = 4,
            ratio_numerator = 3,
            ratio_denominator = 1,
            observe = [named("bounds", "rows-touched")],
        },
    };
    network {
        harness = macroonz::harness,
        module = recipe_network,
        namespace = "bounds",
        nodes = [client, server],
        link forward = client to server,
        schedule quiet = [],
    };
    concurrency {
        harness = macroonz::harness,
        module = recipe_concurrency,
        namespace = "bounds",
        transitions_hold {
            population = "transition-orders",
            interleavings = 16,
            samples = 32,
            seed = 11,
        },
    };
};
"#;

fn joined(count: usize, row: impl Fn(usize) -> String) -> String {
    (0..count).map(row).collect::<Vec<_>>().join(" ")
}

fn comma_joined(count: usize, row: impl Fn(usize) -> String) -> String {
    (0..count).map(row).collect::<Vec<_>>().join(", ")
}

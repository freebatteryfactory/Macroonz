//! The complete lawful recipe load, invoked only as a named long campaign.

use super::limit_contract::joined;
use super::{DOOR, emitted_bytes};
use macroonz_compiler::recipe::{
    CODEC_LIMIT, HarnessPosture, RELATION_LIMIT, RELATION_ROW_LIMIT, RELATION_TABLE_LIMIT,
    TRANSITION_LIMIT, VOCABULARY_LIMIT,
};
use macroonz_compiler::{RENDERED_BYTE_LIMIT, TEXT_SOURCE_BYTE_LIMIT, TextCapture};

#[test]
#[ignore = "explicit long campaign; invoke this maximal recipe by exact name"]
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

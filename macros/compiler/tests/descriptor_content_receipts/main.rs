//! The six descriptor declarations observed through their public canonical-content contract.
//!
//! Each byte length and digest keeps every declared field, physical binding segment, and authored roster position externally observable without importing a private encoder.
//! The reversal changes only authored order, so an encoder that sorted a roster would fail beside the exact receipts rather than appearing equivalent.

use macroonz_compiler::descriptor::{
    Grammar, bench, concurrency, mutation, network, shadow, trial,
};
use macroonz_compiler::{CanonicalContent, CapturedInput, SpanHandle, TextCapture};

const TRIAL_BODY: &str = r#"
    support = greet_support,
    module = greet_trials,
    table = named("lane", "greet-table"),
    suite checks = named("lane", "unit") {
        greet_answers {
            claim = named("lane", "greet-answers"),
            subject = named("lane", "greet"),
            check = named("lane", "exact"),
            population = named("lane", "smalls"),
        },
    },
"#;

const MUTATION_BODY: &str = r#"
    module = pressed,
    refusal = PressRefusal,
    support = press_support,
    family = named("lane", "refusals"),
    point = named("lane", "press-point"),
    fact = named("lane", "cause-order"),
    map named("lane", "cause-order") = named("lane", "order-held"),
    permit named("lane", "order-held") = ["declared-order-permutation"],
"#;

const MUTATION_ITEM: &str = "pub enum Cause { First, Second, Third }";

const BENCH_BODY: &str = r#"
    support = pace_support,
    table_function = pace_table,
    table = named("lane", "pace-table"),
    reporter = pace_reporter,
    encode_pace {
        workload = named("lane", "encode"),
        preflight = named("lane", "encode-correct"),
        planted_worse = named("lane", "encode-worse"),
        complexity = named("lane", "linear"),
        axis = [2, 4, 8],
        samples = 16,
        warmups = 4,
        ratio_numerator = 3,
        ratio_denominator = 1,
        observe = [named("lane", "bytes-touched")],
    },
"#;

const SHADOW_BODY: &str = "loom = renamed_facade::loom, names = [Arc, Mutex]";

const NETWORK_BODY: &str = r#"
    harness = renamed_facade::harness,
    module = net,
    namespace = "lane",
    nodes = [client, server],
    link forward = client to server,
    link back = server to client,
    schedule quiet = [],
    schedule outage = [drop forward at 0, duplicate back at 1],
"#;

const REVERSED_NETWORK_BODY: &str = r#"
    harness = renamed_facade::harness,
    module = net,
    namespace = "lane",
    nodes = [server, client],
    link back = server to client,
    link forward = client to server,
    schedule quiet = [],
    schedule outage = [duplicate back at 1, drop forward at 0],
"#;

const CONCURRENCY_BODY: &str = r#"
    harness = renamed_facade::harness,
    module = explorations,
    namespace = "lane",
    transfers_hold {
        population = "transfer-orders",
        interleavings = 16,
        samples = 32,
        seed = 11,
    },
"#;

fn captured(source: &str) -> Result<CapturedInput, ()> {
    TextCapture::read(source)
        .map(|read| read.input().clone())
        .map_err(|_refusal| ())
}

fn trees(input: &CapturedInput) -> Vec<&macroonz_compiler::CapturedTokenTree> {
    input.trees().iter().collect()
}

fn canonical_content(content: &impl CanonicalContent) -> Vec<u8> {
    let mut bytes = Vec::new();
    content.encode_content_into(&mut bytes);
    bytes
}

fn receipt(bytes: &[u8]) -> (usize, String) {
    (bytes.len(), blake3::hash(bytes).to_hex().to_string())
}

fn trial_content() -> Result<Vec<u8>, ()> {
    let input = captured(TRIAL_BODY)?;
    let content = trial::captured(
        &trees(&input),
        SpanHandle::at(0),
        Grammar {
            attribute: "trials",
        },
    )
    .map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

fn mutation_content() -> Result<Vec<u8>, ()> {
    let body = captured(MUTATION_BODY)?;
    let item = captured(MUTATION_ITEM)?;
    let grammar = Grammar {
        attribute: "mutations",
    };
    let declaration =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_refusal| ())?;
    let content =
        mutation::completed(declaration, &trees(&item), grammar).map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

fn bench_content() -> Result<Vec<u8>, ()> {
    let input = captured(BENCH_BODY)?;
    let content = bench::captured(
        &trees(&input),
        SpanHandle::at(0),
        Grammar { attribute: "bench" },
    )
    .map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

fn shadow_content() -> Result<Vec<u8>, ()> {
    let input = captured(SHADOW_BODY)?;
    let content = shadow::chosen(
        &input,
        Grammar {
            attribute: "shadow",
        },
    )
    .map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

fn network_content(source: &str) -> Result<Vec<u8>, ()> {
    let input = captured(source)?;
    let content = network::declared(
        &input,
        Grammar {
            attribute: "network",
        },
    )
    .map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

fn concurrency_content() -> Result<Vec<u8>, ()> {
    let input = captured(CONCURRENCY_BODY)?;
    let content = concurrency::declared(
        &input,
        Grammar {
            attribute: "concurrency",
        },
    )
    .map_err(|_refusal| ())?;
    Ok(canonical_content(&content))
}

#[test]
/// Claim: every descriptor kind retains the exact canonical bytes its lawful declaration currently publishes.
/// Subject: the six public `CanonicalContent` implementations reached through their public capture roads.
/// Population: one lawful trial, mutation, benchmark, shadow, network, and concurrency declaration.
/// Reversal: the authored-order lane below changes only ordered members and must produce different bytes.
/// Denominator: every descriptor kind that implements `CanonicalContent` in this compiler adapter.
/// Evidence ceiling: these six declarations pin their complete bytes by length and digest, not every lawful declaration.
/// Retained-regression policy: a changed receipt requires an explicit identity and encoded-byte semantic ruling.
fn every_descriptor_kind_publishes_its_exact_canonical_content() -> Result<(), ()> {
    let actual = [
        ("trial", receipt(&trial_content()?)),
        ("mutation", receipt(&mutation_content()?)),
        ("bench", receipt(&bench_content()?)),
        ("shadow", receipt(&shadow_content()?)),
        ("network", receipt(&network_content(NETWORK_BODY)?)),
        ("concurrency", receipt(&concurrency_content()?)),
    ];
    let expected = [
        (
            "trial",
            (
                288,
                "761ba479d36027754143b93d11a47e994f2c79eae55e977cae4604ee2ac64c0a".to_owned(),
            ),
        ),
        (
            "mutation",
            (
                1_159,
                "3f903af81ac616db5f7956a832e596665cf440acf55f2ab3c83951a03dd055b8".to_owned(),
            ),
        ),
        (
            "bench",
            (
                341,
                "1c92eee431dc9d9d356691925b8858566b378c20ef29246e908f7c738ce0d9a1".to_owned(),
            ),
        ),
        (
            "shadow",
            (
                240,
                "d1bef88d44273023ebd4c3fdc8101405de5665894eb874649264b69002d3b2f1".to_owned(),
            ),
        ),
        (
            "network",
            (
                347,
                "aca8d3e60c85ad01608148fa48efb67d28126aa800ca5808cd4a19256e531de6".to_owned(),
            ),
        ),
        (
            "concurrency",
            (
                154,
                "7575852096c9c5f99fe1b7eb16e146f967d4b71839fc1651f8068252501fea32".to_owned(),
            ),
        ),
    ];
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
/// Claim: authored order is a canonical network-content member rather than presentation trivia.
/// Subject: the public network declaration capture and canonical-content roads.
/// Population: one topology with two nodes, two links, two schedules, and two faults.
/// Hostile control: the same members are reversed across node, link, and fault rosters.
/// Denominator: every authored roster in this specimen whose order can move without changing membership.
/// Evidence ceiling: this distinguishes ordering from membership for one network declaration, not arbitrary grammar equivalence.
/// Retained-regression policy: the reversed control remains unequal unless an encoded-byte semantic ruling changes the contract.
fn authored_order_is_a_canonical_content_member() -> Result<(), ()> {
    assert_ne!(
        network_content(NETWORK_BODY)?,
        network_content(REVERSED_NETWORK_BODY)?
    );
    Ok(())
}

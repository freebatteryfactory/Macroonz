//! The six descriptor declarations observed through their public canonical-content contract.
//!
//! Each byte length and digest keeps every declared field, physical binding segment, and authored roster position externally observable without importing a private encoder.
//! The reversal changes only authored order, so an encoder that sorted a roster would fail beside the exact receipts rather than appearing equivalent.
//! The provider roster's composition contract is observed in the `provider_roster` module rather than beside the receipts.

use macroonz_compiler::descriptor::{
    CaptureCause, CaptureIssue, DeclarationError, Grammar, Name, Seat, bench, concurrency,
    mutation, network, shadow, trial,
};
use macroonz_compiler::{
    BENCH_HELPER_FAMILY, CONCURRENCY_HELPER_FAMILY, CanonicalContent, CapturedInput,
    FIRST_HELPER_FAMILY, Family, GeneratedToken, LineBody, NETWORK_HELPER_FAMILY, Observed, Phase,
    RefusalClass, Refused, SECOND_HELPER_FAMILY, SHADOW_HELPER_FAMILY, SpanHandle, TextCapture,
};
#[path = "../support/attribute_specimens.rs"]
mod attribute_specimens;
mod provider_roster;

use attribute_specimens::{BENCH_BODY, MUTATION_BODY, MUTATION_ITEM, TRIAL_BODY};

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

/// One enum carrying the requested number of distinct variants.
fn enum_with_members(members: u32) -> Result<String, core::fmt::Error> {
    use core::fmt::Write as _;

    let mut item = "pub enum Cause {".to_owned();
    for member in 0..members {
        write!(&mut item, "V{member},")?;
    }
    item.push('}');
    Ok(item)
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

fn assert_helper_refusal_contract<E>(
    refusal: &E,
    family: Family,
    related_count: usize,
) -> Result<(), ()>
where
    E: Refused + core::fmt::Display,
{
    assert_eq!(<E as Refused>::PHASE, Phase::Capture);
    assert_eq!(<E as Refused>::FAMILY, family);
    assert_eq!(refusal.class(), RefusalClass::DeclarationNotRead);
    assert_eq!(refusal.first(), refusal.to_string());
    assert_eq!(refusal.observed(), Observed::SeatAbsent);
    assert_eq!(refusal.body(), LineBody::SingleCause);
    assert_eq!(refusal.related().len(), related_count);
    let [_repair] = refusal.repairs().as_slice() else {
        return Err(());
    };
    Ok(())
}

/// Claim: every descriptor helper error projects through one contract while retaining its declared family and related-material posture.
/// Subject: all six public helper capture error types through the `Refused` contract.
/// Population: one empty declaration refused by each helper grammar.
/// Hostile control: mutation carries no related material while the other five carry one canonical refusal, so one copied posture disagrees.
/// Denominator: every helper capture error type in the descriptor adapter.
/// Evidence ceiling: this establishes shared projection mechanics and each family's selected related posture, not every capture issue row.
/// Retained-regression policy: any family, summary, body, repair count, or related-material movement requires an explicit diagnostic ruling.
#[test]
fn every_helper_capture_error_retains_its_projection_contract() -> Result<(), ()> {
    let input = captured("")?;
    let trees = trees(&input);
    let at = SpanHandle::at(0);
    assert_helper_refusal_contract(
        &trial::captured(
            &trees,
            at,
            Grammar {
                attribute: "trials",
            },
        )
        .err()
        .ok_or(())?,
        FIRST_HELPER_FAMILY,
        1,
    )?;
    assert_helper_refusal_contract(
        &mutation::captured(
            &trees,
            at,
            Grammar {
                attribute: "mutations",
            },
        )
        .err()
        .ok_or(())?,
        SECOND_HELPER_FAMILY,
        0,
    )?;
    assert_helper_refusal_contract(
        &bench::captured(&trees, at, Grammar { attribute: "bench" })
            .err()
            .ok_or(())?,
        BENCH_HELPER_FAMILY,
        1,
    )?;
    assert_helper_refusal_contract(
        &shadow::chosen(
            &input,
            Grammar {
                attribute: "shadow",
            },
        )
        .err()
        .ok_or(())?,
        SHADOW_HELPER_FAMILY,
        1,
    )?;
    assert_helper_refusal_contract(
        &network::declared(
            &input,
            Grammar {
                attribute: "network",
            },
        )
        .err()
        .ok_or(())?,
        NETWORK_HELPER_FAMILY,
        1,
    )?;
    assert_helper_refusal_contract(
        &concurrency::declared(
            &input,
            Grammar {
                attribute: "concurrency",
            },
        )
        .err()
        .ok_or(())?,
        CONCURRENCY_HELPER_FAMILY,
        1,
    )?;
    Ok(())
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

#[test]
/// Claim: declared-order completion refuses an order with no adjacent pair and admits the first order that has one.
/// Subject: `mutation::completed_from_order` over an already informed caller-owned roster.
/// Population: one one-member order and one two-member order.
/// Hostile control: the two-member order must produce exactly one adjacent transposition rather than sharing the one-member refusal.
/// Denominator: both sides of the lower declared-order boundary.
/// Evidence ceiling: this proves the minimum pressable width, while the separate magnitude lane proves the upper boundary.
/// Retained-regression policy: changing the refusal or first admitted alternative count requires an explicit mutation-vocabulary ruling.
fn declared_order_completion_requires_one_adjacent_pair() -> Result<(), ()> {
    let grammar = Grammar {
        attribute: "mutations",
    };
    let body = captured(MUTATION_BODY)?;
    let unpressable =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_refusal| ())?;
    let refusal = mutation::completed_from_order(
        unpressable,
        &["Only".to_owned()],
        SpanHandle::at(0),
        grammar,
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.refusal().issue(),
        CaptureIssue::Grammar {
            cause: CaptureCause::OrderUnpressable,
        }
    );

    let first_pressable =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_refusal| ())?;
    let surface = mutation::completed_from_order(
        first_pressable,
        &["First".to_owned(), "Second".to_owned()],
        SpanHandle::at(0),
        grammar,
    )
    .map_err(|_refusal| ())?;
    assert_eq!(surface.site().alternatives().len(), 1);
    Ok(())
}

#[test]
/// Claim: the declared-order ceiling counts the alternatives the completion operation creates rather than the enum members it reads.
/// Subject: mutation completion before the independent generated-token magnitude is applied by rendering.
/// Population: sixty-five members producing exactly sixty-four adjacent alternatives, and sixty-six members producing sixty-five.
/// Hostile control: the sixty-six-member order must refuse with the truthful alternative count and seat.
/// Denominator: both sides of the public `ALTERNATIVE_LIMIT` boundary.
/// Evidence ceiling: a completed maximum-width surface may still meet an independent generated-token magnitude when rendered.
/// Retained-regression policy: changing the count or limit requires an explicit mutation-vocabulary ruling.
fn declared_order_completion_counts_emitted_alternatives_at_its_limit() -> Result<(), ()> {
    let grammar = Grammar {
        attribute: "mutations",
    };
    let body = captured(MUTATION_BODY)?;
    let lawful_source = enum_with_members(65).map_err(|_| ())?;
    let lawful_item = captured(&lawful_source)?;
    let lawful_declaration =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_| ())?;
    let lawful_surface =
        mutation::completed(lawful_declaration, &trees(&lawful_item), grammar).map_err(|_| ())?;
    assert_eq!(
        lawful_surface.site().alternatives().len(),
        mutation::ALTERNATIVE_LIMIT
    );

    let unbounded_source = enum_with_members(66).map_err(|_| ())?;
    let unbounded_item = captured(&unbounded_source)?;
    let unbounded_declaration =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_| ())?;
    let refusal = mutation::completed(unbounded_declaration, &trees(&unbounded_item), grammar)
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.refusal().issue(),
        CaptureIssue::Vocabulary {
            refusal: DeclarationError::Unbounded {
                seat: Seat::Alternative,
                bound: 64,
                observed: 65,
            },
        }
    );
    Ok(())
}

/// Claim: one mutation site refuses two alternatives carrying the same operation before admitting its bounded roster.
/// Subject: the public mutation alternative and site constructors.
/// Population: two alternatives whose families and meanings differ while their semantic operation bytes agree.
/// Hostile control: the differing family and meaning prevent whole-value equality from detecting the duplicate.
/// Denominator: the site constructor's operation-identity duplicate boundary.
/// Evidence ceiling: this establishes operation identity and refusal precedence, not completion's separate generation of adjacent transpositions.
/// Retained-regression policy: admitting both operations or reporting magnitude first requires an explicit mutation-vocabulary ruling.
#[test]
fn mutation_sites_refuse_doubled_operations_before_magnitude() -> Result<(), ()> {
    let first = mutation::Alternative::stated(
        mutation::FamilySlug::declared("first").map_err(|_| ())?,
        vec![7],
        vec![GeneratedToken::word("First")],
    )
    .map_err(|_| ())?;
    let second = mutation::Alternative::stated(
        mutation::FamilySlug::declared("second").map_err(|_| ())?,
        vec![7],
        vec![GeneratedToken::word("Second")],
    )
    .map_err(|_| ())?;
    let refusal = mutation::Site::declared(
        Name::named("lane", "point").map_err(|_| ())?,
        Name::named("lane", "fact").map_err(|_| ())?,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![first, second],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal,
        DeclarationError::Doubled {
            seat: Seat::Alternative,
        }
    );
    Ok(())
}

//! The three roads a generic attribute walks, exercised from outside: a captured body and semantic item in, one sealed carrier expansion out.
//!
//! Every claim below is asked through the road a proc host takes — `descriptor::door` — with nothing reached around it.
//! The positive lanes establish that each road's carrier really composes what its reading produced, and each refusal lane reverses one clause of that, so a road that stopped reading or stopped refusing is caught from this side of the wall.

use macroonz_compiler::descriptor::bench::BENCH_HELPER_POSITION;
use macroonz_compiler::descriptor::door;
use macroonz_compiler::descriptor::mutation::MUTATION_HELPER_POSITION;
use macroonz_compiler::descriptor::trial::TRIAL_HELPER_POSITION;
use macroonz_compiler::descriptor::{Emitter, Grammar};
use macroonz_compiler::request;
use macroonz_compiler::support::SupportCarrier;
use macroonz_compiler::{
    CrateBinding, Diagnostic, Door, Expansion, PartitionCargo, Phase, Producer, TextCapture,
};

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.attribute.grammar",
    "lane::attribute",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "attribute",
    },
);

/// The trial grammar this lane registers.
const TRIALS: Grammar = Grammar {
    attribute: "trials",
};

/// The mutation grammar this lane registers.
const MUTATIONS: Grammar = Grammar {
    attribute: "mutations",
};

/// The bench grammar this lane registers.
const BENCH: Grammar = Grammar { attribute: "bench" };

/// This lane's own act, for the trial road.
const TRIALS_EMITTER: Emitter = Emitter {
    namespace: "lane",
    producer: "attribute-lane",
    door: "trials",
};

/// This lane's own act, for the bench road.
const BENCH_EMITTER: Emitter = Emitter {
    namespace: "lane",
    producer: "attribute-lane",
    door: "bench",
};

#[path = "../support/attribute_specimens.rs"]
mod attribute_specimens;
mod support_refusals;

use attribute_specimens::{BENCH_BODY, MUTATION_BODY, MUTATION_ITEM, TRIAL_BODY};

/// The semantic item the trial and bench helpers exercise.
const DECLARATION_ITEM: &str = "pub struct Declaration;";

/// The trial road walked over one source, or nothing where the lane's own source did not capture.
fn trials(source: &str) -> Option<Result<Expansion<SupportCarrier>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    let item = TextCapture::read(DECLARATION_ITEM).ok()?;
    Some(door::trials(
        read.input(),
        item.input(),
        TRIALS,
        TRIALS_EMITTER,
        &DOOR,
    ))
}

/// The mutation road walked over one body and one item, on the same terms.
fn mutations(body: &str, item: &str) -> Option<Result<Expansion<SupportCarrier>, Diagnostic>> {
    let read = TextCapture::read(body).ok()?;
    let sat_on = TextCapture::read(item).ok()?;
    Some(door::mutations(
        read.input(),
        sat_on.input(),
        MUTATIONS,
        &DOOR,
    ))
}

/// The bench road walked over one source, on the same terms.
fn bench(source: &str) -> Option<Result<Expansion<SupportCarrier>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    let item = TextCapture::read(DECLARATION_ITEM).ok()?;
    Some(door::bench(
        read.input(),
        item.input(),
        BENCH,
        BENCH_EMITTER,
        &DOOR,
    ))
}

/// The declaration-site text one carrier expansion emits.
fn emitted(expansion: &Expansion<SupportCarrier>) -> Option<String> {
    expansion
        .emit()
        .tokens()
        .map(macroonz_compiler::GeneratedTree::inspected)
}

/// Removes the last trailing comma while leaving every declared value unchanged.
fn without_trailing_comma(source: &str) -> Option<String> {
    let at = source.rfind(',')?;
    let mut changed = source.to_owned();
    changed.remove(at);
    Some(changed)
}

/// A trial declaration becomes one carrier at the declaration site and nothing anywhere else.
///
/// The stamped table rides INSIDE the carrier — the gate's stamped seat — so the expansion's own test-carrier and bench-carrier deliveries carry nothing.
#[test]
fn a_trial_declaration_becomes_one_carrier_at_the_declaration_site() -> Result<(), ()> {
    let carrier = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("macro_rules"));
    assert!(text.contains("__macroonz_support_"));
    assert!(text.contains("greet_support"));
    assert!(text.contains("generated_support"));
    assert!(text.contains("expected"));
    assert!(matches!(
        carrier.test_carrier(),
        PartitionCargo::NothingPlanned
    ));
    assert!(matches!(
        carrier.bench_carrier(),
        PartitionCargo::NothingPlanned
    ));
    Ok(())
}

/// The carrier's matcher binds every metavariable the stamped table spells.
///
/// The three host facts, and each row's three attachment seats under the row's own lens.
#[test]
fn the_trial_matcher_binds_every_spelled_metavariable() -> Result<(), ()> {
    let carrier = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("harness : $harness : ident $( :: $harness_segment : ident ) * ,"));
    assert!(text.contains("$harness $( :: $harness_segment ) * :: generated_support"));
    for clause in [
        "invocation",
        "target",
        "clock",
        "greet_answers_subject_revision",
        "greet_answers_check_revision",
        "greet_answers_call",
    ] {
        assert!(text.contains(clause), "the matcher does not bind {clause}");
    }
    Ok(())
}

/// A local support address preserves the macro-invocation separator after Rust's hygienic crate root.
#[test]
fn a_local_support_address_keeps_its_macro_invocation_path() -> Result<(), ()> {
    let local = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    let local_text = emitted(&local).ok_or(())?;
    assert!(
        local_text.contains("$crate :: __macroonz_support_"),
        "{local_text}"
    );
    assert!(
        !local_text.contains("$crate __macroonz_support_"),
        "{local_text}"
    );
    Ok(())
}

/// A trial body missing a required clause refuses at capture, and no carrier exists.
#[test]
fn a_trial_body_missing_its_support_clause_refuses() -> Result<(), ()> {
    let refusal = trials("module = greet_trials").ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// Claim: A mutation declaration becomes one carrier whose opaque seat calls the public discovery-lowering road.
/// Subject: The rendered module carried by one complete mutation declaration.
/// Population: The carrier text, all three declared variants, and the one generated discovery call.
/// Hostile control: The assertion rejects the private semantic-home path and requires the exact public operation path.
/// Denominator: The complete emitted carrier for the declaration and item used by this fixture.
/// Evidence ceiling: This compiler-side test establishes generated tokens only, not downstream execution.
/// Retained regression: Public-path drift, private-home leakage, and lost declared variants remain permanent regressions.
#[test]
fn a_mutation_declaration_becomes_one_carrier_carrying_the_module() -> Result<(), ()> {
    let carrier = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("press_support"));
    assert!(text.contains("declared-order-permutation"));
    assert!(
        text.contains(
            "$harness $( :: $harness_segment ) * :: muterprater :: discover :: lower_discoveries"
        ),
        "{text}"
    );
    assert!(!text.contains("$harness $( :: $harness_segment ) * :: muterprater :: discovery"));
    for variant in ["First", "Second", "Third"] {
        assert!(text.contains(variant), "the order does not carry {variant}");
    }
    assert!(matches!(
        carrier.test_carrier(),
        PartitionCargo::NothingPlanned
    ));
    Ok(())
}

/// The mutation item is the declaration root rather than a dependency beside the helper body.
#[test]
fn the_mutation_item_is_the_request_root() -> Result<(), ()> {
    let carrier = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    let captured_item = TextCapture::read(MUTATION_ITEM).map_err(|_refusal| ())?;
    assert_eq!(
        carrier.plan().account().commitment(),
        request::committed(captured_item.input())
    );
    assert!(carrier.plan().account().dependencies().is_empty());
    let reordered = mutations(MUTATION_BODY, "pub enum Cause { Third, Second, First }")
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert_ne!(carrier.identity(), reordered.identity());
    Ok(())
}

/// Prove one helper road's movement and non-movement slice.
fn assert_helper_movement(
    name: &str,
    first: &Expansion<SupportCarrier>,
    changed: &Expansion<SupportCarrier>,
) -> Result<(), ()> {
    let first_account = first.plan().account();
    let changed_account = changed.plan().account();
    let first_content = first_account.content();
    let changed_content = changed_account.content();
    let first_helper = first_content.helper().ok_or(())?;
    let changed_helper = changed_content.helper().ok_or(())?;
    assert_eq!(
        first_account.commitment(),
        changed_account.commitment(),
        "{name} moved the semantic declaration"
    );
    assert_eq!(
        first_account.kind(),
        changed_account.kind(),
        "{name} moved the unrelated projection kind"
    );
    assert_eq!(first_content.root(), changed_content.root());
    assert_eq!(first_content.expectation(), changed_content.expectation());
    assert_eq!(first_content.address(), changed_content.address());
    assert_eq!(first_content.declared(), changed_content.declared());
    assert_eq!(first_content.deferred(), changed_content.deferred());
    assert_eq!(first_content.bench(), changed_content.bench());
    assert_ne!(
        first_helper, changed_helper,
        "{name} did not move the captured helper"
    );
    assert_ne!(
        first_account.content_commitment(),
        changed_account.content_commitment(),
        "{name} did not carry helper movement into the assembly content"
    );
    assert_ne!(
        first.identity(),
        changed.identity(),
        "{name} did not carry helper movement into the sealed expansion"
    );
    Ok(())
}

/// Prove that one actual door uses the position its semantic helper owner declares.
fn assert_helper_position(
    expansion: &Expansion<SupportCarrier>,
    body: &str,
    item: &str,
    position: u32,
) -> Result<(), ()> {
    let captured_body = TextCapture::read(body).map_err(|_| ())?;
    let captured_item = TextCapture::read(item).map_err(|_| ())?;
    assert_eq!(
        expansion.plan().account().content().helper(),
        Some(request::committed_helper(
            captured_item.input(),
            captured_body.input(),
            position,
        ))
    );
    Ok(())
}

/// Each attribute-helper door seats its captured body at the public position its owner declares.
#[test]
fn the_three_attribute_doors_use_their_declared_helper_positions() -> Result<(), ()> {
    assert_eq!(
        [
            TRIAL_HELPER_POSITION,
            MUTATION_HELPER_POSITION,
            BENCH_HELPER_POSITION,
        ],
        [0, 1, 2],
    );
    let trial = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    assert_helper_position(&trial, TRIAL_BODY, DECLARATION_ITEM, TRIAL_HELPER_POSITION)?;

    let mutation = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert_helper_position(
        &mutation,
        MUTATION_BODY,
        MUTATION_ITEM,
        MUTATION_HELPER_POSITION,
    )?;

    let benchmark = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    assert_helper_position(
        &benchmark,
        BENCH_BODY,
        DECLARATION_ITEM,
        BENCH_HELPER_POSITION,
    )?;
    Ok(())
}

/// Each helper identity moves independently while the semantic item and unrelated kind identity remain fixed.
///
/// Removing a trailing comma changes the helper's canonical token material without changing the declaration each grammar reads, so the final assembly commitment and carrier expansion can move only through the captured-helper seat.
#[test]
fn all_three_helper_roads_move_only_the_helper_side_of_the_join() -> Result<(), ()> {
    let trial_changed = without_trailing_comma(TRIAL_BODY).ok_or(())?;
    let first_trial = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    let changed_trial = trials(&trial_changed).ok_or(())?.ok().ok_or(())?;
    assert_helper_movement("trials", &first_trial, &changed_trial)?;

    let mutation_changed = without_trailing_comma(MUTATION_BODY).ok_or(())?;
    let first_mutation = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    let changed_mutation = mutations(&mutation_changed, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert_helper_movement("mutations", &first_mutation, &changed_mutation)?;

    let bench_changed = without_trailing_comma(BENCH_BODY).ok_or(())?;
    let first_bench = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    let changed_bench = bench(&bench_changed).ok_or(())?.ok().ok_or(())?;
    assert_helper_movement("bench", &first_bench, &changed_bench)?;
    Ok(())
}

/// An item that is not an enum states no declared order, and the road refuses it.
#[test]
fn a_mutation_item_without_an_order_refuses() -> Result<(), ()> {
    let refusal = mutations(MUTATION_BODY, "pub struct Flat;")
        .ok_or(())?
        .err()
        .ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// An order of one member has no transposition, and the road says so rather than pressing nothing.
#[test]
fn a_single_variant_order_refuses_as_unpressable() -> Result<(), ()> {
    let refusal = mutations(MUTATION_BODY, "pub enum Cause { Only }")
        .ok_or(())?
        .err()
        .ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// A standalone mutation attribute owns its carrier, so a body without a support address refuses.
#[test]
fn a_mutation_body_without_a_support_address_refuses() -> Result<(), ()> {
    let body = r#"
        module = pressed,
        refusal = PressRefusal,
        family = named("lane", "refusals"),
        point = named("lane", "press-point"),
        fact = named("lane", "cause-order"),
    "#;
    let refusal = mutations(body, MUTATION_ITEM).ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// An unknown mutation key refuses as undeclared before its following tokens can imply another clause shape.
#[test]
fn mutation_unknown_keys_keep_their_declared_precedence() -> Result<(), ()> {
    let body = MUTATION_BODY.replacen("module = pressed", "mystery pressed", 1);
    let refusal = mutations(&body, MUTATION_ITEM).ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    assert!(
        refusal
            .summary()
            .contains("a clause is not one the grammar declares"),
        "{}",
        refusal.summary()
    );
    Ok(())
}

/// A bench declaration becomes one carrier writing the bench form: stamped table, opaque reporter.
#[test]
fn a_bench_declaration_becomes_one_carrier_writing_the_bench_form() -> Result<(), ()> {
    let carrier = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("pace_support"));
    assert!(text.contains("benches"));
    assert!(text.contains("reporter"));
    assert!(text.contains("pace_reporter"));
    assert!(text.contains("DeclaredBudgets :: declared ( 16 , 4 , 3 , 1 )"));
    assert!(!text.contains("divan"));
    assert!(matches!(
        carrier.test_carrier(),
        PartitionCargo::NothingPlanned
    ));
    assert!(matches!(
        carrier.bench_carrier(),
        PartitionCargo::NothingPlanned
    ));
    Ok(())
}

/// The benchmark carrier asks the target for exactly the executable facts the generated binding consumes.
#[test]
fn the_bench_matcher_names_the_target_owned_seats() -> Result<(), ()> {
    let carrier = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    for clause in [
        "reporter",
        "encode_pace_measured",
        "encode_pace_planted_worse",
        "encode_pace_judge",
        "encode_pace_preflight",
    ] {
        assert!(text.contains(clause), "the matcher omits {clause}");
    }
    for retired in ["backend", "declaring", "run_worse", "run_preflight"] {
        assert!(
            !text.contains(retired),
            "the retired bench vocabulary still emits {retired}"
        );
    }
    Ok(())
}

/// The retired backend grammar and an omitted exact-ratio half both refuse at capture.
#[test]
fn retired_or_incomplete_benchmark_syntax_refuses() -> Result<(), ()> {
    let backend = BENCH_BODY.replacen(
        "reporter = pace_reporter,",
        "reporter = pace_reporter, backend = divan,",
        1,
    );
    let missing_denominator = BENCH_BODY.replacen("ratio_denominator = 1,", "", 1);
    for source in [&backend, &missing_denominator] {
        let refusal = bench(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture);
    }
    Ok(())
}

/// The two generated benchmark items cannot claim one target-namespace spelling.
#[test]
fn benchmark_table_and_reporter_names_cannot_collide() -> Result<(), ()> {
    let collided = BENCH_BODY.replacen("reporter = pace_reporter,", "reporter = pace_table,", 1);
    let refusal = bench(&collided).ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    assert!(refusal.summary().contains("generated-item"));
    Ok(())
}

/// Numeric width and observation-roster closure are enforced before generated code exists.
#[test]
fn benchmark_budget_width_and_observation_roster_are_closed() -> Result<(), ()> {
    let wide_samples = BENCH_BODY.replacen("samples = 16,", "samples = 4294967296,", 1);
    let typed_samples = BENCH_BODY.replacen("samples = 16,", "samples = 16u32,", 1);
    let separated_samples = BENCH_BODY.replacen("samples = 16,", "samples = 1_6,", 1);
    let no_observation = BENCH_BODY.replacen(
        "observe = [named(\"lane\", \"bytes-touched\")],",
        "observe = [],",
        1,
    );
    let duplicate_observation = BENCH_BODY.replacen(
        "observe = [named(\"lane\", \"bytes-touched\")],",
        "observe = [named(\"lane\", \"bytes-touched\"), named(\"lane\", \"bytes-touched\")],",
        1,
    );
    for (source, cause) in [
        (
            &wide_samples,
            "an authored number outruns the width of its seat",
        ),
        (&typed_samples, "a clause is not one key and one value"),
        (&separated_samples, "a clause is not one key and one value"),
        (
            &no_observation,
            "the declaration states no work-observation",
        ),
        (
            &duplicate_observation,
            "one work-observation of the declaration is stated twice",
        ),
    ] {
        let refusal = bench(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal.summary().contains(cause),
            "{source} refused under the wrong cause"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}

/// Benchmark axis and lens namespaces refuse repeated identities before rendering.
#[test]
fn benchmark_namespaces_refuse_repeated_identities() -> Result<(), ()> {
    let doubled_axis = BENCH_BODY.replacen("axis = [2, 4, 8],", "axis = [2, 4, 2],", 1);
    let doubled_lens = BENCH_BODY.replacen(
        "    encode_pace {",
        "    encode_pace {\n        workload = named(\"lane\", \"other\"),\n        preflight = named(\"lane\", \"other-correct\"),\n        planted_worse = named(\"lane\", \"other-worse\"),\n        complexity = named(\"lane\", \"linear\"),\n        axis = [2, 4],\n        samples = 8,\n        warmups = 2,\n        ratio_numerator = 2,\n        ratio_denominator = 1,\n        observe = [named(\"lane\", \"other-bytes\")],\n    },\n    encode_pace {",
        1,
    );
    for (source, cause) in [
        (
            &doubled_axis,
            "one axis-size of the declaration is stated twice",
        ),
        (&doubled_lens, "one lens of the declaration is stated twice"),
    ] {
        let refusal = bench(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture);
        assert!(refusal.summary().contains(cause), "{}", refusal.summary());
    }
    Ok(())
}

/// Mutation policy namespaces refuse repeated families, facts, and claims.
#[test]
fn mutation_policy_namespaces_refuse_repeated_identities() -> Result<(), ()> {
    let doubled_family = MUTATION_BODY.replacen(
        "[\"declared-order-permutation\"]",
        "[\"declared-order-permutation\", \"declared-order-permutation\"]",
        1,
    );
    let doubled_fact = MUTATION_BODY.replacen(
        "map named(\"lane\", \"cause-order\") = named(\"lane\", \"order-held\"),",
        "map named(\"lane\", \"cause-order\") = named(\"lane\", \"order-held\"),\n    map named(\"lane\", \"cause-order\") = named(\"lane\", \"other\"),",
        1,
    );
    let doubled_claim = MUTATION_BODY.replacen(
        "permit named(\"lane\", \"order-held\") = [\"declared-order-permutation\"],",
        "permit named(\"lane\", \"order-held\") = [\"declared-order-permutation\"],\n    permit named(\"lane\", \"order-held\") = [\"other-family\"],",
        1,
    );
    for (source, cause) in [
        (
            &doubled_family,
            "one operator-family of the declaration is stated twice",
        ),
        (
            &doubled_fact,
            "one fact-mapping of the declaration is stated twice",
        ),
        (
            &doubled_claim,
            "one permission of the declaration is stated twice",
        ),
    ] {
        let refusal = mutations(source, MUTATION_ITEM)
            .ok_or(())?
            .err()
            .ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture);
        assert!(refusal.summary().contains(cause), "{}", refusal.summary());
    }
    Ok(())
}

/// Trial label and generated namespaces refuse repeated identities and cross-seat shadows.
#[test]
fn trial_namespaces_refuse_repeated_identities() -> Result<(), ()> {
    let doubled_role = TRIAL_BODY.replacen(
        "claim = named(\"lane\", \"greet-answers\"),",
        "claim = named(\"lane\", \"greet-answers\"),\n            roles = [named(\"lane\", \"reader\"), named(\"lane\", \"reader\")],",
        1,
    );
    let doubled_tag = TRIAL_BODY.replacen(
        "claim = named(\"lane\", \"greet-answers\"),",
        "claim = named(\"lane\", \"greet-answers\"),\n            tags = [named(\"lane\", \"fast\"), named(\"lane\", \"fast\")],",
        1,
    );
    let doubled_aggregate = format!(
        "{TRIAL_BODY}\n    suite checks = named(\"lane\", \"other-suite\") {{\n        other_answers {{\n            claim = named(\"lane\", \"other-answers\"),\n            subject = named(\"lane\", \"other\"),\n            check = named(\"lane\", \"exact\"),\n            population = named(\"lane\", \"smalls\"),\n        }},\n    }},"
    );
    let doubled_lens = format!(
        "{TRIAL_BODY}\n    suite other = named(\"lane\", \"other-suite\") {{\n        greet_answers {{\n            claim = named(\"lane\", \"other-answers\"),\n            subject = named(\"lane\", \"other\"),\n            check = named(\"lane\", \"exact\"),\n            population = named(\"lane\", \"smalls\"),\n        }},\n    }},"
    );
    let shadowed_lens = TRIAL_BODY.replacen("        greet_answers {", "        checks {", 1);
    for (source, cause) in [
        (&doubled_role, "one role of the declaration is stated twice"),
        (&doubled_tag, "one tag of the declaration is stated twice"),
        (
            &doubled_aggregate,
            "one aggregate of the declaration is stated twice",
        ),
        (&doubled_lens, "one lens of the declaration is stated twice"),
        (
            &shadowed_lens,
            "one lens of the declaration is stated twice",
        ),
    ] {
        let refusal = trials(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture);
        assert!(refusal.summary().contains(cause), "{}", refusal.summary());
    }
    Ok(())
}

/// A bench axis of one point is not a curve, and the road refuses the row rather than reading a growth class off a point.
#[test]
fn a_bench_axis_of_one_point_refuses() -> Result<(), ()> {
    let body = r#"
        support = pace_support,
        table_function = pace_table,
        table = named("lane", "pace-table"),
        reporter = pace_reporter,
        encode_pace {
            workload = named("lane", "encode"),
            preflight = named("lane", "encode-correct"),
            planted_worse = named("lane", "encode-worse"),
            complexity = named("lane", "linear"),
            axis = [2],
            samples = 16,
            warmups = 4,
            ratio_numerator = 3,
            ratio_denominator = 1,
            observe = [named("lane", "bytes-touched")],
        },
    "#;
    let refusal = bench(body).ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// Attribute assignments require one equals sign before a non-empty value in every descriptor grammar.
#[test]
fn attribute_assignments_require_the_complete_shared_shape() -> Result<(), ()> {
    let trial = TRIAL_BODY.replacen("support = greet_support", "support : greet_support", 1);
    let mutation = MUTATION_BODY.replacen("module = pressed", "module : pressed", 1);
    let benchmark = BENCH_BODY.replacen("reporter = pace_reporter", "reporter : pace_reporter", 1);
    for refusal in [
        trials(&trial).ok_or(())?.err().ok_or(())?,
        mutations(&mutation, MUTATION_ITEM)
            .ok_or(())?
            .err()
            .ok_or(())?,
        bench(&benchmark).ok_or(())?.err().ok_or(())?,
    ] {
        assert_eq!(refusal.phase(), Phase::Capture);
        assert!(
            refusal
                .summary()
                .contains("a clause is not one key and one value"),
            "{}",
            refusal.summary()
        );
    }
    Ok(())
}

/// A trial separator separating nothing refuses at capture — leading, doubled, or inside a row body — while trailing commas stay ordinary Rust.
#[test]
fn a_trial_separator_separating_nothing_refuses() -> Result<(), ()> {
    let leading = r#"
        , support = greet_support,
        module = greet_trials,
        table = named("lane", "greet-table"),
    "#;
    let doubled = r#"
        support = greet_support,,
        module = greet_trials,
        table = named("lane", "greet-table"),
    "#;
    let dangling_in_row = r#"
        support = greet_support,
        module = greet_trials,
        table = named("lane", "greet-table"),
        suite checks = named("lane", "unit") {
            greet_answers {
                claim = named("lane", "greet-answers"),,
                subject = named("lane", "greet"),
                check = named("lane", "exact"),
                population = named("lane", "smalls"),
            },
        },
    "#;
    for source in [leading, doubled, dangling_in_row] {
        let refusal = trials(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal
                .summary()
                .contains("a separator stands where no clause does"),
            "{source} does not name the dangling separator"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}

/// A bench separator separating nothing refuses at capture, and an axis whose numbers stand unseparated refuses rather than being read as two lawful sizes.
#[test]
fn a_bench_separator_separating_nothing_refuses() -> Result<(), ()> {
    let doubled = BENCH_BODY.replacen("support = pace_support,", "support = pace_support,,", 1);
    let dangling_axis = BENCH_BODY.replacen("axis = [2, 4, 8],", "axis = [2,, 4],", 1);
    let unseparated_axis = BENCH_BODY.replacen("axis = [2, 4, 8],", "axis = [2 4],", 1);
    for source in [&doubled, &dangling_axis] {
        let refusal = bench(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal
                .summary()
                .contains("a separator stands where no clause does"),
            "{source} does not name the dangling separator"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    let unseparated = bench(&unseparated_axis).ok_or(())?.err().ok_or(())?;
    assert_eq!(unseparated.phase(), Phase::Capture);
    Ok(())
}

/// A mutation separator separating nothing refuses at capture, and a permission roster whose slugs stand unseparated refuses rather than being read as two lawful families.
#[test]
fn a_mutation_separator_separating_nothing_refuses() -> Result<(), ()> {
    let doubled = MUTATION_BODY.replacen("module = pressed,", "module = pressed,,", 1);
    let dangling_permit = MUTATION_BODY.replacen(
        "= [\"declared-order-permutation\"],",
        "= [, \"declared-order-permutation\"],",
        1,
    );
    let unseparated_permit = MUTATION_BODY.replacen(
        "= [\"declared-order-permutation\"],",
        "= [\"declared-order-permutation\" \"declared-order-permutation\"],",
        1,
    );
    for source in [&doubled, &dangling_permit] {
        let refusal = mutations(source, MUTATION_ITEM)
            .ok_or(())?
            .err()
            .ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal
                .summary()
                .contains("a separator stands where no clause does"),
            "{source} does not name the dangling separator"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    let unseparated = mutations(&unseparated_permit, MUTATION_ITEM)
        .ok_or(())?
        .err()
        .ok_or(())?;
    assert_eq!(unseparated.phase(), Phase::Capture);
    Ok(())
}

/// Two declarations mint two carriers under two exported names, keyed by each plan's own identity.
#[test]
fn two_declarations_mint_two_exported_names() -> Result<(), ()> {
    let first = trials(TRIAL_BODY).ok_or(())?.ok().ok_or(())?;
    let second = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    let first_text = emitted(&first).ok_or(())?;
    let second_text = emitted(&second).ok_or(())?;
    let name = |text: &str| -> Option<String> {
        let opened = text.find("__macroonz_support_")?;
        Some(text.get(opened..opened.checked_add(83)?)?.to_owned())
    };
    let first_name = name(&first_text).ok_or(())?;
    let second_name = name(&second_text).ok_or(())?;
    assert_ne!(first_name, second_name);
    Ok(())
}

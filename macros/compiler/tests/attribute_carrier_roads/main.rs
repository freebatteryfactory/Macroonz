//! The three roads a generic attribute walks, exercised from outside: a captured body in, one sealed carrier expansion out.
//!
//! Every claim below is asked through the road a proc host takes — `descriptor::door` — with nothing reached around it.
//! The positive lanes establish that each road's carrier really composes what its reading produced, and each refusal lane reverses one clause of that, so a road that stopped reading or stopped refusing is caught from this side of the wall.

use macroonz::descriptor::door;
use macroonz::descriptor::{Emitter, Grammar};
use macroonz::support::SupportCarrier;
use macroonz::{
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

/// One lawful trial declaration body.
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

/// One lawful mutation declaration body.
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

/// The item a mutation declaration sits on: three variants, so two adjacent transpositions exist.
const MUTATION_ITEM: &str = "pub enum Cause { First, Second, Third }";

/// One lawful bench declaration body.
const BENCH_BODY: &str = r#"
    support = pace_support,
    module = pace_benches,
    table = named("lane", "pace-table"),
    adapter = pace_adapter,
    backend = divan,
    encode_pace {
        workload = named("lane", "encode"),
        preflight = named("lane", "encode-correct"),
        planted_worse = named("lane", "encode-worse"),
        complexity = named("lane", "linear"),
        axis = [2, 4, 8],
        samples = 16,
        warmup = 4,
        ratio = 3,
        run = declaring::ops::encode,
        run_worse = declaring::ops::encode_slow,
        run_preflight = declaring::ops::encode_check,
        observe = [declaring::ops::bytes_touched],
    },
"#;

/// The trial road walked over one source, or nothing where the lane's own source did not capture.
fn trials(source: &str) -> Option<Result<Expansion<SupportCarrier>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::trials(
        read.input().clone(),
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
        read.input().clone(),
        sat_on.input(),
        MUTATIONS,
        &DOOR,
    ))
}

/// The bench road walked over one source, on the same terms.
fn bench(source: &str) -> Option<Result<Expansion<SupportCarrier>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::bench(
        read.input().clone(),
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
        .map(macroonz::GeneratedTree::inspected)
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

/// A trial body missing a required clause refuses at capture, and no carrier exists.
#[test]
fn a_trial_body_missing_its_support_clause_refuses() -> Result<(), ()> {
    let refusal = trials("module = greet_trials").ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
    Ok(())
}

/// A mutation declaration becomes one carrier whose opaque seat carries the rendered module.
///
/// The declared order is the item's own variant list: every variant spelling appears in the emitted carrier, and so does the one operator family the door produces alternatives under.
#[test]
fn a_mutation_declaration_becomes_one_carrier_carrying_the_module() -> Result<(), ()> {
    let carrier = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("press_support"));
    assert!(text.contains("declared-order-permutation"));
    for variant in ["First", "Second", "Third"] {
        assert!(text.contains(variant), "the order does not carry {variant}");
    }
    assert!(matches!(
        carrier.test_carrier(),
        PartitionCargo::NothingPlanned
    ));
    Ok(())
}

/// The item's capture rides the mutation request as a dependency, so editing the item moves the plan.
#[test]
fn the_mutation_item_rides_as_a_dependency() -> Result<(), ()> {
    let carrier = mutations(MUTATION_BODY, MUTATION_ITEM)
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert_eq!(carrier.plan().account().dependencies().len(), 1);
    let reordered = mutations(MUTATION_BODY, "pub enum Cause { Third, Second, First }")
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert_ne!(carrier.identity(), reordered.identity());
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

/// A bench declaration becomes one carrier writing the bench form: stamped table, opaque reporter.
#[test]
fn a_bench_declaration_becomes_one_carrier_writing_the_bench_form() -> Result<(), ()> {
    let carrier = bench(BENCH_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&carrier).ok_or(())?;
    assert!(text.contains("pace_support"));
    assert!(text.contains("benches"));
    assert!(text.contains("reporter"));
    assert!(text.contains("pace_adapter"));
    assert!(text.contains("divan"));
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

/// A bench axis of one point is not a curve, and the road refuses the row rather than reading a growth class off a point.
#[test]
fn a_bench_axis_of_one_point_refuses() -> Result<(), ()> {
    let body = r#"
        support = pace_support,
        module = pace_benches,
        table = named("lane", "pace-table"),
        adapter = pace_adapter,
        backend = divan,
        encode_pace {
            workload = named("lane", "encode"),
            preflight = named("lane", "encode-correct"),
            planted_worse = named("lane", "encode-worse"),
            complexity = named("lane", "linear"),
            axis = [2],
            samples = 16,
            warmup = 4,
            ratio = 3,
            run = declaring::ops::encode,
            run_worse = declaring::ops::encode_slow,
            run_preflight = declaring::ops::encode_check,
        },
    "#;
    let refusal = bench(body).ok_or(())?.err().ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Capture);
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

//! The lawful attribute declaration bodies shared by every compiler lane that drives the trial, mutation, and benchmark roads.

/// One lawful trial declaration body.
pub(crate) const TRIAL_BODY: &str = r#"
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
pub(crate) const MUTATION_BODY: &str = r#"
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
pub(crate) const MUTATION_ITEM: &str = "pub enum Cause { First, Second, Third }";

/// One lawful bench declaration body.
pub(crate) const BENCH_BODY: &str = r#"
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

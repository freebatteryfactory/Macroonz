//! A clause outside the benchmark grammar refuses through the actual `bench` proc entry at that clause.

#[macroonz_macros::bench(
    support = pace_support,
    table_function = pace_table,
    table = named("proc", "pace-table"),
    reporter = pace_reporter,
    backend = retired,
    encode_pace {
        workload = named("proc", "encode"),
        preflight = named("proc", "encode-correct"),
        planted_worse = named("proc", "encode-worse"),
        complexity = named("proc", "linear"),
        axis = [2, 4],
        samples = 16,
        warmups = 4,
        ratio_numerator = 3,
        ratio_denominator = 1,
        observe = [named("proc", "bytes-touched")],
    },
)]
mod held {}

fn main() {}

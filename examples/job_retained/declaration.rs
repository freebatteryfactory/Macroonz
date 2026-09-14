//! Work accumulation over the shared Job record and its generated input-bearing trial.

macroonz::recipe! {
    /// Caller-owned accumulation of ordered work increments.
    pub(crate) mod work {
        /// Adds each increment to the record and advances its attempt count.
        #[must_use]
        pub(crate) fn accumulate(input: &[u8]) -> crate::job::Record {
            let mut record = crate::job::Record::assembled(0, 0);
            for amount in input {
                record.count = record.count.saturating_add(u16::from(*amount));
                record.attempts = record.attempts.saturating_add(1);
            }
            record
        }

        bake! {
            evidence {
                trials {
                    support = job_replay_support,
                    module = trials,
                    table = named("neutral-job", "work-counts"),
                    input = { ::std::vec::Vec<u8> },
                    suite saved_input = named("neutral-job", "saved-input") {
                        bounded_work {
                            claim = named("neutral-job", "work-count-never-exceeds-three"),
                            roles = [named("neutral-job", "regression")],
                            tags = [named("neutral-job", "declared-input")],
                            subject = named("neutral-job", "work-counter"),
                            check = named("neutral-job", "bounded-work"),
                            population = named("neutral-job", "work-count-histories"),
                            binding = {
                                subject_revision = { $consumer::checks::subject_revision() },
                                check_revision = { $consumer::checks::revision(include_bytes!("checks.rs")) },
                                call = { $consumer::checks::within_bound },
                            },
                        },
                    },
                };
            };
        }
    }
}

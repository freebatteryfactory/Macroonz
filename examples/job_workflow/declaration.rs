//! Caller-owned job structure and the declarations that attach independent checks.

macroonz::recipe! {
    /// Work stages, requested events and a persisted count owned by this caller.
    pub mod job {
        /// One job's current stage.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Work that has not been queued.
            Draft,
            /// Work awaiting completion.
            Queued,
            /// Completed work.
            Done,
        }

        /// One requested change to a job.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Requests that draft work be queued.
            Queue,
            /// Requests completion of queued work.
            Complete,
            /// Returns queued work to its draft stage.
            Cancel,
        }

        /// Caller-owned work and attempt counts.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Record {
            /// The persisted work count.
            pub count: u16,
            /// The number of attempts associated with the work.
            pub attempts: u8,
        }

        impl Record {
            /// Assembles the caller's record from its decoded counts.
            #[must_use]
            pub const fn assembled(count: u16, attempts: u8) -> Self { Self { count, attempts } }
        }

        bake! {
            vocabularies { Stage; Event; };
            transitions(Stage, Event) {
                (Draft, Queue) => Queued with(target) { Ok(target) };
                (Queued, Complete) => Done with(target) { Ok(target) };
                (Queued, Cancel) => Draft with(target) { Ok(target) };
                (Done, Queue) => Queued with(target) { Ok(target) };
            };
            absence(refused);
            codecs {
                record(Record) {
                    direction(round_trip);
                    refusal(RecordDecodeError);
                    assembly(assembled, total);
                    members {
                        count: u16 => count(required);
                        attempts: u8 => count(required);
                    };
                };
            };
            projections { companions; dispatch(apply); codec; };
            evidence {
                trials {
                    support = job_support,
                    module = trials,
                    table = named("neutral-job", "checks"),
                    suite workflow = named("neutral-job", "workflow") {
                        completion {
                            claim = named("neutral-job", "a-queued-job-completes"),
                            roles = [named("neutral-job", "regression")],
                            tags = [named("neutral-job", "declared-input")],
                            subject = named("neutral-job", "declared-job"),
                            check = named("neutral-job", "completion"),
                            population = named("neutral-job", "authored-examples"),
                            binding = {
                                subject_revision = { $consumer::checks::subject_revision() },
                                check_revision = { $consumer::checks::check_revision() },
                                call = { $consumer::checks::job_completes },
                            },
                        },
                    },
                    suite codec = named("neutral-job", "codec") {
                        round_trip {
                            claim = named("neutral-job", "a-record-preserves-count"),
                            roles = [named("neutral-job", "regression")],
                            tags = [named("neutral-job", "declared-input")],
                            subject = named("neutral-job", "declared-job"),
                            check = named("neutral-job", "round-trip"),
                            population = named("neutral-job", "authored-examples"),
                            binding = {
                                subject_revision = { $consumer::checks::subject_revision() },
                                check_revision = { $consumer::checks::check_revision() },
                                call = { $consumer::checks::record_preserves_count },
                            },
                        },
                    },
                };
                benchmarks {
                    support = job_bench_support,
                    table_function = table,
                    table = named("neutral-job", "dispatch-cost"),
                    reporter = reporter,
                    dispatch_cost {
                        workload = named("neutral-job", "dispatch-batch"),
                        preflight = named("neutral-job", "completes"),
                        planted_worse = named("neutral-job", "quadratic-dispatches"),
                        complexity = named("neutral-job", "linear-dispatches"),
                        axis = [2, 4, 8],
                        samples = 2,
                        warmups = 1,
                        ratio_numerator = 2,
                        ratio_denominator = 1,
                        observe = [named("neutral-job", "dispatch-invocations")],
                    },
                };
            };
        }
    }
}

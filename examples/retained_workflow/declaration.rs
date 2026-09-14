//! Caller-owned byte counting and its one generated independent trial binding.

macroonz::recipe! {
    /// Two caller implementations of the same byte-counting task.
    pub(crate) mod counted {
        /// Counts bytes while deliberately overlooking the value one.
        #[must_use]
        pub(crate) fn lossy(bytes: &[u8]) -> usize {
            bytes.iter().filter(|byte| **byte != 1u8).count()
        }

        /// Counts every byte by advancing once per element.
        #[must_use]
        pub(crate) fn corrected(bytes: &[u8]) -> usize {
            bytes.iter().fold(0usize, |count, _byte| count.saturating_add(1))
        }

        bake! {
            evidence {
                trials {
                    support = count_support,
                    module = trials,
                    table = named("counting", "bytes"),
                    input = { ::std::vec::Vec<u8> },
                    suite independent = named("counting", "independent") {
                        count {
                            claim = named("counting", "every-byte"),
                            subject = named("counting", "count"),
                            check = named("counting", "slice-length"),
                            population = named("counting", "byte-vectors"),
                            binding = {
                                subject_revision = { $consumer::checks::revision(b"omit-one-v1") },
                                check_revision = { $consumer::checks::revision(b"slice-length-v1") },
                                call = { $consumer::checks::lossy },
                            },
                        },
                    },
                };
            };
        }
    }
}

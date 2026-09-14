//! The caller's commands, state and declared network and exploration choices.

macroonz::recipe! {
    /// The declared network and command-order explorations.
    pub mod world {
        bake! {
            evidence {
                network {
                    harness = macroonz::harness,
                    module = network,
                    namespace = "schedule-example",
                    nodes = [depositor, withdrawer, ledger],
                    link deposits = depositor to ledger,
                    link withdrawals = withdrawer to ledger,
                    schedule quiet = [],
                    schedule pressure = [
                        duplicate deposits at 0,
                        delay deposits at 0 by 2,
                        drop deposits at 1,
                        partition withdrawals from 0 until 2,
                    ],
                };
                concurrency {
                    harness = macroonz::harness,
                    module = exploration,
                    namespace = "schedule-example",
                    exhaustive {
                        population = "delivery-orders",
                        interleavings = 2,
                        samples = 3,
                        seed = 19,
                    },
                    sampled {
                        population = "delivery-orders",
                        interleavings = 1,
                        samples = 3,
                        seed = 19,
                    },
                    no_work {
                        population = "delivery-orders",
                        interleavings = 0,
                        samples = 3,
                        seed = 19,
                    },
                };
            };
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Move {
    Deposit(u64),
    Withdraw(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Account {
    pub(super) balance: i128,
}

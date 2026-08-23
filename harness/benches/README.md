# benches — ordinary benchmark targets

Each target in this directory is an ordinary consumer of the public [`bench`](../src/bench/README.md) receiver. A target builds a table, supplies explicit host input, obtains one immutable report, and may hand that report to a renderer; the receiver owns work judgment, while a renderer owns no verdict.

`neutral_receiver.rs` is the backend-free handwritten specimen. It shares the same neutral declaration as the focused behavior lane, exercises the exact generated-support pin partition, and adds no benchmark backend as a second execution authority.

//! A clause outside the network grammar refuses through the actual `network` proc entry at that clause.

macroonz_macros::network! {
    harness = mh,
    module = net,
    namespace = "proc",
    nodes = [client, server],
    link forward = client to server,
    latency = 3,
}

fn main() {}

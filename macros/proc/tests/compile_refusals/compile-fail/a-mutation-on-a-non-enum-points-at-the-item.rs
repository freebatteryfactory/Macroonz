//! A mutation attribute on the wrong item kind refuses through the actual proc entry at the item that states no variant order.

#[macroonz_macros::mutations(
    module = pressed,
    refusal = PressRefusal,
    support = press_support,
    family = named("proc", "refusals"),
    point = named("proc", "press-point"),
    fact = named("proc", "cause-order"),
    map named("proc", "cause-order") = named("proc", "order-held"),
    permit named("proc", "order-held") = ["declared-order-permutation"],
)]
struct Flat;

fn main() {}

//! The proc-macro seat of `macroonz`: three generic attributes, each a carrier and nothing else.
//!
//! Every grammar an attribute here reads is the compiler's `descriptor` home's, every road from a reading to the tokens a declaration site receives is the compiler's `support` home's, and the walk between them is the descriptor home's own `door` road.
//! What this crate adds is exactly what a proc host owns: token conversion, span custody, one compiler call, diagnostic placement, and emission — plus the five facts of its own act, declared once beside each attribute.
//!
//! Each attribute expands to one exported carrier and then the item it decorates, byte for byte as the author wrote it.
//! The carrier is inert until a consumption target invokes it, so an ordinary build compiles the item and one macro definition and nothing more.

use macroonz::descriptor::door;
use macroonz::descriptor::{Emitter, Grammar};
use macroonz::{CrateBinding, Door, Producer, host};
use proc_macro::TokenStream;

/// The grammar spelling the `trials` attribute registers.
const TRIALS_GRAMMAR: Grammar = Grammar {
    attribute: "trials",
};

/// The grammar spelling the `mutations` attribute registers.
const MUTATIONS_GRAMMAR: Grammar = Grammar {
    attribute: "mutations",
};

/// The grammar spelling the `bench` attribute registers.
const BENCH_GRAMMAR: Grammar = Grammar { attribute: "bench" };

/// This crate's own act, for the trial door.
const TRIALS_EMITTER: Emitter = Emitter {
    namespace: "macroonz",
    producer: "macroonz-macros",
    door: "trials",
};

/// This crate's own act, for the bench door.
const BENCH_EMITTER: Emitter = Emitter {
    namespace: "macroonz",
    producer: "macroonz-macros",
    door: "bench",
};

/// Who is asking, wherever a `trials` expansion refuses.
const TRIALS_DOOR: Door = Door::declared(
    "macroonz",
    "macroonz.trials",
    "macroonz_macros::trials",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "macroonz",
        name: "macroonz-macros",
    },
);

/// Who is asking, wherever a `mutations` expansion refuses.
const MUTATIONS_DOOR: Door = Door::declared(
    "macroonz",
    "macroonz.mutations",
    "macroonz_macros::mutations",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "macroonz",
        name: "macroonz-macros",
    },
);

/// Who is asking, wherever a `bench` expansion refuses.
const BENCH_DOOR: Door = Door::declared(
    "macroonz",
    "macroonz.bench",
    "macroonz_macros::bench",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "macroonz",
        name: "macroonz-macros",
    },
);

/// Declares a trial table beside the item this attribute sits on.
///
/// The body is the trial grammar, read whole by `macroonz::descriptor::trial`: the exported support name, the stamped module, the table's own name, and each aggregate seat with its rows.
/// The expansion is one exported carrier holding the stamped table inert, followed by the item unchanged; a consumption target invokes the carrier by the declared support name and supplies its own host facts and callables there.
///
/// A malformed declaration expands to `compile_error!` at the offending token, carrying the compiler's own rendering of the established cause.
#[proc_macro_attribute]
pub fn trials(body: TokenStream, item: TokenStream) -> TokenStream {
    let mut expanded = host::expand(body, |capture| {
        door::trials(capture, TRIALS_GRAMMAR, TRIALS_EMITTER, &TRIALS_DOOR)
    });
    expanded.extend(item);
    expanded
}

/// Declares a mutation surface over the enum this attribute sits on.
///
/// The body is the mutation grammar, read whole by `macroonz::descriptor::mutation`: the surface's address, the evaluation family, the point and owner fact, the fact-to-claim mappings, and the operator permissions.
/// The door completes the site from the item itself — the enum's variant list is the declared order, the unchanged operation is that order as authored, and each alternative is one adjacent transposition of it under the `declared-order` operator family.
/// The expansion is one exported carrier holding the rendered module as proved test-carrier cargo, followed by the item unchanged.
///
/// A malformed declaration, and an item that states no order this grammar can read, expand to `compile_error!` at the offending token.
#[proc_macro_attribute]
pub fn mutations(body: TokenStream, item: TokenStream) -> TokenStream {
    let mut expanded = host::expand_on(body, item.clone(), |captured_body, captured_item| {
        door::mutations(
            captured_body,
            &captured_item,
            MUTATIONS_GRAMMAR,
            &MUTATIONS_DOOR,
        )
    });
    expanded.extend(item);
    expanded
}

/// Declares a bench table and its reporter adapter beside the item this attribute sits on.
///
/// The body is the bench grammar, read whole by `macroonz::descriptor::bench`: the exported support name, the stamped module, the table's own name, the adapter module and its one backend value, and each row with its references, axis, budgets, and callables.
/// The expansion is one exported carrier holding the table in its stamped seat and the adapter in its opaque seat, followed by the item unchanged.
///
/// A malformed declaration expands to `compile_error!` at the offending token, carrying the compiler's own rendering of the established cause.
#[proc_macro_attribute]
pub fn bench(body: TokenStream, item: TokenStream) -> TokenStream {
    let mut expanded = host::expand(body, |capture| {
        door::bench(capture, BENCH_GRAMMAR, BENCH_EMITTER, &BENCH_DOOR)
    });
    expanded.extend(item);
    expanded
}

# host — the bridge to `proc_macro`

The one place in this crate that names a compiler's own token type.

`proc_macro` is an API only a proc-macro crate may call, and this crate is ordinary callable Rust that must stay callable.
So the bridge lives behind the `host` feature, which only a proc-macro crate turns on, and it does exactly four things — none of which is a decision.

## Capture

A `TokenStream` becomes a `CapturedInput` through the token home's checked builder, walked natively: groups stay groups, nothing is re-lexed, and no balance is re-discovered.

Which value a literal's spelling names is asked of `capture_literal`.
The forms, the value each one names, and the two ways reading one refuses are the compiler's grammar; a host deciding them here would be a second grammar nobody ever compared against the first.

The four magnitudes a capture stands under — nesting depth, tokens per level, tokens across the whole tree, and the capture-work budget — are declared in `token/` and spent by the walk here.
This host reports which one stopped it; it declares none.

## Custody

One handle is issued per token, in reading order, by the builder inside [`Spans`] — the table of compiler spans this host keeps while converting.

That table is the whole reason a refusal lands on the offending token rather than on the declaration's first one.
A `SpanTable` cannot do it: the compiler holds no compiler spans and cannot, so the seam between the two is a handle and the resolution is the producer's.

The table is the caller's value, not something the capture consumes, because a refusal names a handle in it: a table swallowed by the road that refused could not resolve the very handle the refusal carries.
The builder owns both the retained spans and the issued denominator, so custody cannot be counted on a second road.

## Call

The road runs once, over that capture, and answers with one sealed expansion or one diagnostic.

Read [`expand()`] and there is no grammar, no roster, no shape decision, no identity, no plan, and no message.
Every sentence a person reads was composed inside the compiler, where the typed value it projects lives; this host does not even build the string it emits.

## Emit and place

An expansion's proved declaration-site cargo becomes a `TokenStream`, one delivery at a time.
No third generated tree is assembled out of them here — a tree this host joined would be bytes no proof committed to — and a delivery that planned nothing emits nothing, because an absence is an answer a reader reads rather than tokens a compiler receives.

A diagnostic becomes a `compile_error!` at the token its site names.
A diagnostic that names no token, and a handle this table does not reach, are both reported at the invocation.
Neither answers with the declaration's first span, which is a real token the observation is not about and would read exactly like an answer.

## Why a refused capture is not a diagnostic

A diagnostic carries the prefix, the grammar, and the callable entry of a door, and a capture runs before any door is named — the caller states its door inside the road, over the capture this one produced.

So [`CaptureError`] is an ordinary error: it prints, it is a `core::error::Error`, and it places itself at the position it is a fact about.
A magnitude is a fact about the whole declaration and no one token overran it, so it is reported at the invocation; a literal this crate could not read is a fact about exactly one token, whose handle was issued before its payload was read, so the span is held and the report goes there.

A caller that does hold a door projects it through `Diagnostic::refused` instead, under the placement `AtToken { token, spans: &SpanTable::ProducerHeld }` — which is the posture this host stands in, and the reason that arm exists.

## The seats

`types.rs` declares the span custody, the capture refusal, and what a value answers to be emitted.

Its own child `type_guard.rs` holds every road that reaches the custody's private seat.
`type_contract.rs` states the refusal's row position and the contracts it stands under, and implements emission for an expansion.
`capture.rs`, `emit.rs`, and `place.rs` are the three conversions; `expand.rs` is the whole road one entry point is; `encode.rs` writes the bytes one refused capture is named by.

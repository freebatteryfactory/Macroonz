# support — the exported carrier a consumption target invokes

A declaration expands where it stands.
What a test target or a bench target needs cannot expand there: an ordinary build of the crate holding the declaration would compile every byte of it.

This home is the one vehicle that carries it across.

## One carrier

The carrier is an exported `macro_rules!` definition, hidden, named by the plan's own identity at full width, whose body is one invocation of the harness's gate.

A macro definition nobody invokes is a definition nobody compiles, so the cargo rides inert until a consumption target names it.
The name is derived rather than chosen, because the definition lands at the root of whatever crate the declaration sits in and shares one namespace with every other exported macro there.
Nobody can know that spelling before the expansion runs, so a delivery a person is meant to invoke renders the address its author chose as well: an ordinary exported definition whose one rule forwards every token to the hidden one.

## Two seats behind one pin

The gate's grammar writes a coupled pair, and one pin governs both.

One seat carries material in the harness's own declaration grammar, which the gate forwards to its stamp.
The other carries token trees the gate never parses and emits verbatim.
On a matched pin the gate releases both; on a mismatch it emits its own refusal and neither, which is what makes the pin a door rather than a comment.

The pin crosses as a roster of decimal byte values because the gate matches TOKENS.
A byte string has many spellings of one value and an unsuffixed integer has exactly one, so the two sides are one token by construction rather than by an escaping convention nobody controls.

## Nothing unproved crosses

Every token in an opaque seat was rendered and PROVED somewhere else.

There is no road to a carrier that takes a token tree on its own: cargo is read off a terminal's own proved delivery and compared against what that delivery carries, and the reading records which terminal and which delivery it came from.
The stamped seat's material is the other half, refused seat by seat at this home's own door before a token of it exists.

## What an assembly establishes

Holding one means five things were settled while it was built.

- **One declaration.** Every carried axis's terminal stands over the declaration the assembly does. A carrier composing two declarations' cargo is one exported name delivering material from two places, whichever one the caller meant.
- **One published pin.** The expectation the gate is pinned against is the one these services publish, at full width. An expectation minted beside it would put a pin in the carrier that no publication act wrote.
- **Each terminal's delivery consumed once.** Two axes reading one terminal's one delivery would deliver those tokens twice into one target.
- **No cargo reaching a second destination.** An axis reads the delivery its own row names and no other. Cargo read from the declaration-site delivery into a carrier seat is material the ordinary build already compiles, carried again into a target that compiles it too.
- **One delivery form.** One carrier is one gate invocation, and one gate invocation is one coupled pair of seats.

The sixth is settled later, at the one road to a rendered carrier: the carrier's own plan and the assembly must stand over one declaration.
Nothing before that road holds both values, and a plan for another declaration agrees with every reading downstream — the carrier would be born wearing that plan's key and origin over cargo that is not its.

## Three axes, three materials

They are composed rather than collapsed into one payload with a discriminant, because the materials are genuinely different and a seat that could hold either is a seat nobody answers for.

The DECLARED axis carries stamp-grammar tokens and the matcher clauses those tokens consume, rendered by whoever owns that grammar.
The DEFERRED axis carries one terminal's proved test-carrier cargo.
The BENCH axis carries one terminal's proved bench-carrier cargo.

Which form the gate is invoked under follows from the axes rather than being stated beside them: a carried bench axis is the bench form, anything else is the trial form.
The bench form's stamped seat is required, because the gate's own transcription of that seat has no empty row.

An axis nothing filled carries the DISPOSITION of what would have filled it.
"This seat is empty" is a shape a reader cannot act on, and "nobody asked for it" and "it does not apply here" are answers to different questions the deciding road already gave.

## How it says no

Three refusals, each an ordinary error that prints, is a `core::error::Error`, and projects into a diagnostic through the one contract every refusing step implements.

The DECLARATION refusal is about this home's own vocabulary — a name that names nothing, a spelling that is not an identifier, a path that names no item.
The ASSEMBLY refusal carries every way a set of closed outputs did not compose, together, because an assembly failing on two declarations and a doubled consumption at once is repaired in one attempt rather than two.
The SHELL refusal is the one road's own answer: the plan and the assembly are not one declaration's, or the composed carrier outgrew the token magnitude.

## The seats

`types.rs` declares, and its own child `type_guard.rs` holds every road that reaches a private field — the name parsers, the promotion of proved cargo, the assembly, and the one road to a rendered carrier.

`establish.rs` is the verification pass, pure, reading each axis through the same answers any caller gets.
`render.rs` is the token half: the pin roster, the gate invocation, the exported definition, and the forwarding address.
`encode.rs` writes the bytes one refusal is named by.
`type_contract.rs` states the rosters' constant tables and the contracts the three refusals stand under.

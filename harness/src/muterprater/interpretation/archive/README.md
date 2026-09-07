# archive

Owned historical no-mutation comparisons through explicit caller encodings.

`retain_parity` borrows a complete reading and records its raw disposition.
`retain_parity_standing` borrows a qualified or rejected standing and retains its actual disposition and entire reading.
`read_parity` takes declared bytes and limits only; it invokes no encoder, decoder, witness, equivalence, evaluation or host operation.
The result owns its data independently of the source reading and byte buffer.

## Caller encoding

`ValueEncoder<Value>` binds a capture-free function pointer to an admitted convention name, version, schema commitment and executable revision binding.
Input uses one binding; production and evaluation meanings share another binding whose metadata is encoded once.
This convention is separate from the [input](../../../input/README.md) owner's admitted case format and earns no input identity.
An encoder returns exact owned bytes or `ValueEncodingRefusal`, retaining its caller-owned cause and optional report-owned `ForeignText`.
The archive adds the input, production or evaluation role to a refusal without turning it into a trial finding.

Known metadata, nested-record and roster bounds are checked before any encoder call.
Reached encoders run once each, in input, production and evaluation order.
Each returned buffer is bounded before copying, and the complete envelope size is checked before its allocation.
A later refusal returns no partial archive and does not consume the original reading.
Empty and repeated encodings are lawful; the caller's convention owns what those bytes mean.

Callbacks keep ordinary Rust effects, unwinding, termination and allocation limits.
Byte bounds do not constrain allocations inside caller code.
Encoding an interior-mutable value at retention time does not prove its state during earlier execution or equivalence calls.
The framework does not establish that the encoding faithfully represents the generic Rust value.

## Historical consistency

The [descriptor archive](../../../descriptor/archive/README.md) retains the witness's complete row, attachment names, separate revision claims and producer standing.
Its admitted historical coordinates feed the descriptor's existing trial-key writer and the report's existing unprofiled trial writer.
This derives an integrity address without constructing a live row, key, trial identity or callable attachment.

Both [trial archives](../../../report/archive/README.md) must claim that trial, the witness's exact subject/check revision addresses and the attachment revision-posture meet.
Their complete execution standing and sites must agree.
Their input metadata must be absent, reflecting the no-mutation observer's unit-invocation road; the separately encoded generic input is not an input-aware execution-key coordinate.
An equivalence refusal retains the complete finding and must claim the same trial.
Pair production/evaluation revisions remain independent of attachment revisions.
The surface is an address claim, without an invented pair digest, discovery denominator or current producer-schema assertion.

The [properties owner](../../../properties/README.md) owns substrate semantics.
A historical standing roster is nonempty and strictly ordered by namespace and stem; duplicates and out-of-order names refuse.
Its independence arm is an explicit declaration and cannot be reached by loading an empty roster.

The shared interpretation decision checks production report qualification, evaluation report qualification, zero firings and equivalence agreement in that order.
Only an executed passing attempt qualifies a report; measurement is independent.
A raw record makes no claim that qualification ran.
A qualified or rejected historical disposition must agree with that decision, including the first refusal and exact nonzero firing count.
No historical disposition constructs current parity qualification, selection, compiled pressure, interpreted trust or human admission.
Envelope integrity establishes neither authenticity nor execution.

## Format

All integers are big-endian.
`bytes(x)` is an eight-byte length followed by `x`; names are framed namespace then framed stem.
The envelope begins with the thirty-two-byte address derived over the following body under `PARITY_ARCHIVE_TAG`.
Its body, in order, is:

1. Format `u32(1)`, kind `u32(1)` and historical custody `u32(0)`.
2. Pair family name, production and evaluation revision records, then framed thirty-two-byte surface claim.
3. Framed complete descriptor binding encoding.
4. Input convention, then framed input bytes.
5. Meaning convention, framed production bytes, then framed evaluation bytes.
6. Evaluation firings as `u32`.
7. Substrate slot: `0` independent, or `1` followed by `u64(count)` and strictly ordered names.
8. Equivalence slot: `0` passed, or `1` followed by the report archive's complete finding encoding.
9. Framed complete production trial archive, then framed complete evaluation trial archive.
10. Disposition: `0` raw, `1` qualified, or `2` followed by a refusal slot.

A convention is its name, `u32` version, framed thirty-two-byte schema claim and descriptor-owned revision record.
Each revision record is framed according to the descriptor archive's format.
Rejection slots are `0` production, `1` evaluation, `2` activation followed by `u32(firings)`, and `3` meanings disagreed.
No function addresses or reconstruction handles are serialized.

## Bounds

`ParityArchiveLimits` independently bounds complete envelope bytes, outer framed fields, nested descriptor bindings, nested trial archives, each value's bytes and substrate count.
The value ceiling and outer field ceiling both apply to each value.
The reader bounds the complete envelope before hashing, names and values before copying, and substrate counts before walking them.
It reserves no storage from untrusted counts, consumes all members and refuses unknown formats, kinds, custody claims, slots and trailing material.

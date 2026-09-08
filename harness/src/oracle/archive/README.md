# archive

This home retains the complete data each oracle verdict actually carries for historical inspection.

It composes the existing [report archive](../../report/archive/) byte limits and envelope admission with the oracle owners' method-specific vocabulary and pure guards.
A loaded record is historical data, never a current verdict, conclusion, compiler observation, callable binding, qualification or human admission.
Its address establishes the integrity of its supplied body, not the identity of its writer or the execution of a producer, parser or compiler.

Vector disagreements retain both complete buffers and derive their first difference through the [vector](../vector/) owner's comparison operation.
Transcript disagreements retain both unequal identity claims without their original preimages.
Successful vector and transcript verdicts carry no successful-case payload to reconstruct.
Structural and compiled verdicts are caller-constructible, so retention preserves their stated fields without asserting that a comparator produced them.
Equal purported disagreement coordinates, zero structural positions or cardinalities and empty free member text remain lawful where the existing construction permits them.
Compiled error codes, relative paths, source positions and primary spans retain their existing smart-constructor boundaries.

Structural positions and cardinalities are historical u64 values, not indexes on the reader's platform.
Vector positions are derived from admitted buffers and therefore fit the reader's platform.
The structural source, expected declaration, compiled read-back values, complete compiler output, transcript context and absent successful-case material are not invented from a verdict.

## Envelope

The leading thirty-two bytes are the address derived under historical-oracle-verdict/v1.
Its body contains, in order:

- Format u32be one.
- The oracle method as u32be: vector one, transcript two, structural three, compiled read-back four or exact compilation five.
- Historical custody u32be zero.
- One method-specific disposition byte and exactly its members.

The reader receives its expected method independently and refuses a different kind.
A frame is the identity owner's u64be byte length followed by those exact bytes.
Every text frame is UTF-8 without rewriting.
The writer checks total and per-frame sizes before allocating retained material; the reader checks the envelope before hashing and every frame before copying.
Unknown versions, custody, methods, dispositions, disagreement slots and trailing material refuse.
No population count authorizes allocation, no recursive value is read and no function pointer is serialized.

## Vector and transcript

Disposition zero is agreement with no members.
Disposition one is disagreement with two framed members.

For vector disagreement, the members are the complete expected and produced buffers.
The buffers must differ; their first unequal byte, or their lengths when one is a prefix, is derived by the existing vector operation rather than stored as a second independently changeable claim.

For transcript disagreement, the members are the rederived and published identity claims, in that order.
Each has exactly thirty-two bytes, and they must differ.
Neither becomes a `ContentAddress` without its absent preimage.

## Structural

Disposition zero is conformity and two is unparsable, both without members.
Disposition one is a disagreement byte followed by the members below.

| Slot | Disagreement | Members |
| --- | --- | --- |
| 0 | unexpected item | none |
| 1 | output cardinality | declared u64be, read u64be |
| 2 | duplicate implementation | position u64be |
| 3 | implementation target | position u64be |
| 4 | trait path | position u64be |
| 5 | implementation posture | position u64be |
| 6 | meaning-bearing attribute | position u64be, framed attribute text |
| 7 | unexpected member | position u64be, framed member text |
| 8 | duplicate member | position u64be, framed member text |
| 9 | missing member | position u64be, framed member text |
| 10 | unread member value | position u64be, framed member text |
| 11 | differing member value | position u64be, framed member text |

These are the retained verdict's fields, not a reconstructed structural reading or parse result.
Free attribute/member strings are not descriptor names.

## Compiled read-back

Disposition zero is conformity without members.
Disposition one is a disagreement byte: accepted where refusal was declared zero, refused where acceptance was declared one, unexpected member two, duplicate member three, missing member four or differing member value five.
Slots zero and one have no members; the other slots have one framed exact member string.
The existing `CompiledDisagreement` is pure caller-stated data and carries no conclusion operation by itself.

## Exact compilation

Disposition zero is conformity without members.
Disposition one is a disagreement byte with these members:

| Slot | Disagreement | Members |
| --- | --- | --- |
| 0 | accepted where refusal was declared | none |
| 1 | refused where acceptance was declared | observed framed code, observed span |
| 2 | error code | expected framed code, observed framed code |
| 3 | primary span | expected span, observed span |

One span contains its framed canonical relative source path, then start line, start column, end line and end column as u64be.
Codes retain the existing E-plus-four-digits grammar.
Paths retain the existing normalized relative-path grammar, and coordinates remain one-based.
Zero-width spans remain lawful; reversed spans refuse through their existing owner.
The existing `CompilationDisagreement` contains these informed values but establishes no compiler execution.

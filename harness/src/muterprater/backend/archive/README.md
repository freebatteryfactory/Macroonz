# Historical backend manifests

This home retains an imported backend manifest as bounded owned historical data through `muterprater::backend_archive`.
It preserves the invocation's separate command tokens and target labels, the adapter profile, output and source revision claims, the entire ordered [mutation run](../../verdict/archive/README.md), the independent announced count, and every unread line with its ordinal and loss markers.
Source files are unique and in strict spelling order; unread ordinals are strictly increasing.
The source roster names exactly the distinct reported files.
Repeated mutation targets and disagreement between announced and parsed populations remain observable.

## Authority and original material

The invocation and profile must name the same backend and nonempty version under the [wrap grammar](../wrap/README.md).
Every retained mutation respects that grammar's target and outcome road.
Owner and family mappings remain historical claims; loading never invokes current callbacks to reclassify them.
The historical types provide no conversion to current artifact custody, adapter qualification, mutation execution, trust, pressure or admission.

`retain_backend` retains the identities and readings already held by a manifest.
Its console and original source bytes are absent.
`retain_backend_with_material` additionally accepts the complete console and exact source bytes from its caller.
Those bytes must join the saved output and source identities using the backend owner's existing content domains.
The supplied source roster may arrive in any order but must match the manifest exactly, without duplicates.

Original material is retained all together or is absent.
When present, the existing line parser checks the console's first baseline, last announcement, ordered coordinates, normalized-damage identities, outcome axes, rejection text and unread lines against the saved reading.
The full console retains spacing and bytes that parsed reports alone cannot reconstruct.
This consistency check establishes neither producer authenticity, an actual backend execution, current-source currency nor completeness beyond the supplied material.
No process, filesystem, current ownership lookup or storage mechanism is invoked here.

## Bounds

`BackendArchiveLimits` declares envelope bytes, framed-field bytes, mutation reports, argument tokens, source files and unread lines independently.
The envelope ceiling precedes reading; each length or population is admitted before the corresponding copy or walk.
The writer checks sizes and populations before encoding preimages.
The nested mutation run has the same byte bounds and its own report ceiling.
Source material is arbitrary bytes; the complete console and command/path labels are UTF-8.
Argument tokens and target/toolchain labels may be empty, matching their live owners; versions, executables and source file spellings may not.

## Envelope grammar

Integers are big endian.
A frame is a `u64` byte length followed by that many bytes.
The leading raw thirty-two-byte address is derived over the remaining body under `historical-backend-manifest/v1`.
The body begins with `u32` format one, kind one and historical custody zero.

The remaining members occur in this order:

- Invocation: backend byte zero, framed version, framed executable, `u64` argument population and that many framed arguments, then framed target and toolchain.
- Profile: backend byte zero, stated-version byte one, framed version, console-source byte zero and `u32` grammar version.
- Output identity as a thirty-two-byte frame.
- `u64` source population, then each framed file spelling and thirty-two-byte revision frame in strict file order.
- A frame containing the complete mutation-run envelope, including its own integrity address.
- Announcement byte zero for unstated, or one followed by a `u32` count.
- `u64` unread population, then each `u64` ordinal and present [historical foreign text](../../../report/archive/README.md).
- Original-material byte zero for absent, or one followed by the framed console and one framed source buffer for each file in the retained source roster's order.

Unknown slots, unsupported versions or custody, noncanonical rosters, invalid text, malformed nested records, contradictory joins, truncated members and trailing bytes refuse.
The nested verdict owner derives the mutation census; no separate summary is encoded.

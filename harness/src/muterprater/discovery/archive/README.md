# archive

Owned historical evaluation surfaces without executable selection or policy authority.

`retain_surface` wraps the discovery owner's complete canonical surface preimage.
`read_surface` reads that same preimage and owns every retained field independently of its source buffer.
It invokes no callback, current operator-bank lookup, policy admission or execution.
The surface's family, policy address, point names, owner claims, original operations, activation sites and complete alternative rosters survive retention.
The absent full policy, producer discovery order and rejected or unmapped discoveries are not reconstructed.

## Identities and consistency

Surface and alternative addresses are rederived from complete supplied preimages under the discovery owner's existing domains and framing.
The policy address remains a claim because its permission roster is absent.
Historical family slugs remain exact nonempty UTF-8 data, including slugs outside the current operator catalogue.
The reader creates no live surface, point, membership, alternative, selection or directive.
Envelope integrity establishes neither authenticity nor current permission.

`ArchivedSelection` retains a surface address, point name and alternative address for composed historical records.
Without the surface preimage these coordinates remain claims; they create no current selection.
Its shared member grammar is a framed surface address, the existing namespaced-name encoding, then a framed alternative address.

Points must be strictly ordered by namespace and stem.
Each point has a nonempty alternative roster in strictly increasing derived-identity order.
Original and alternative operations are nonempty, and an alternative cannot equal its original.
Duplicate family/operation pairs derive the same alternative identity and therefore refuse through the strict ordering check.
A point-free surface remains lawful and supplies no discovery-denominator claim.

## Format and bounds

All integers are big-endian; framed bytes use an eight-byte length followed by their material.
The envelope begins with the thirty-two-byte address derived over its body under `SURFACE_ARCHIVE_TAG`.
The body is format `u32(1)`, kind `u32(1)`, historical custody `u32(0)`, framed thirty-two-byte surface identity, then the framed canonical surface preimage.
The [discovery encoder](../encode.rs) owns the canonical preimage grammar and its alternative identities.
No new canonical surface format is introduced.

`SurfaceArchiveLimits` independently bounds complete envelope bytes, every framed field, total points and alternatives per point.
The canonical preimage is itself one bounded field.
Retention checks all bounds before allocating that preimage.
Loading bounds the envelope before hashing, fields before copying and each count before walking its roster; it never reserves from untrusted counts.
Unknown envelope formats, kinds or custody slots, malformed names, addresses, noncanonical order and trailing material refuse.

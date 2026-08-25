# identity — one derivation mechanism

An address is `blake3::derive_key` over a preimage, under a context assembled from one stem, the minting home's `DomainTag`, and the position that tag carries.
Separation is by derivation context rather than by a prefix inside the message, so two addresses over one preimage under different tags are unrelated values.
Nothing here knows what is being named: a tag arrives as an argument from the home that owns the kind, so the substrate carries no semantic noun.

An address commits to the preimage its minting home wrote and to nothing else.

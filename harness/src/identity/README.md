# identity — one derivation mechanism

An address is `blake3::derive_key` over a preimage, under a context assembled from one stem, the minting home's `DomainTag`, and the position that tag carries.
Separation is by derivation context rather than by a prefix inside the message, so two addresses over one preimage under different tags are unrelated values.
Nothing here knows what is being named: a tag arrives as an argument from the home that owns the kind, so the substrate carries no semantic noun.
The same substrate splits addressed envelopes and reads framed bodies without taking ownership of a caller's schema or refusal vocabulary.
One crate-private stamp declares typed content-address wrappers and their value or borrowed reader while each semantic home retains the operation that earns its address.

An address commits to the preimage its minting home wrote and to nothing else.

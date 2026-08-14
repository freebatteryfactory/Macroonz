# refusal — the planning refusal family

How the services say no while planning.

Planning issues are independent and co-establishable — a plan may name an unknown
kind *and* exceed a declared bound in one pass — so the family takes the machine's
issue-collection shape: a bounded, non-empty collection over a closed issue set,
carrying its enumeration posture as an instance value. No primary issue is ever
elected, and a zero-issue refusal is unrepresentable.

Every seam in the plane that can refuse returns this family body. The universal
refusal envelope is the publication form and is minted where reasons are
registered, which is the machine's business, not the plane's.

## The seats

`types.rs` declares: the two closed rosters this home states, the pair a
contradiction stands between, the closed issue set, and the family body. The
body's one seat is private, and its own child `type_guard.rs` is the invariant
nucleus that holds every road reaching it: the one-issue seam, the
co-establishing pass, the bounded seam's own spelling, and the borrow the seat is
read back on. Readable and writable are different permissions and this home grants
one of them — a refusal a caller cannot read is a refusal nobody can act on, and a
refusal a caller can WRITE is a seam minting the plane's answer without running
the pass that establishes it. `type_contract.rs` states the family's declared
shape and the issue roster's own slot table.

Both halves of "writable" are closed, because closing one of them is closing
neither. The private seat closes the literal; the three mints are crate-internal
so that no caller outside the services can hand an issue to a road and receive a
refusal no pass raised, and no holder of the borrow can clone the issues out and
seat them under a fresh body. `pub(crate)` is this family's strongest reachable
scope: it is the plane's shared planning family, so the passes that establish its
issues live in four homes rather than in one `type_guard.rs`, and the crate is
the narrowest boundary that contains them all. What stays open is stated at the
declaration — inside the services, any module can still mint.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.

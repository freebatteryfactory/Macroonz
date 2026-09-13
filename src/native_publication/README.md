# Native publication

This home binds the compiler's complete proved publication output to explicitly declared filesystem destinations.
It is available through the root package's opt-in `native-tooling` feature.
The [command owner](command/README.md) composes the supported prepare, inspect, check, generate and recovery actions around an explicitly supplied generation operation.
The [publication workflow example](../../examples/publication_workflow/README.md) supplies an executable stdin adapter and a caller-owned generation operation through that same library surface.

`Publication::declared` consumes a sealed compiler expansion and a complete logical-address-to-path binding.
`with_stamp` adds every landing of a stamp only after its record and definition agree with the expansion's actual published unit.
The caller retains its kind, renderer, addresses, authored Rust and destination choices.
The compiler retains canonical generation and identity authority.

The [inventory owner](inventory/README.md) defines path admission, complete-set binding, resource bounds and the distinction between canonical token and published-byte digests.
An inventory is prepared material, not an observation that files were formatted, compiled or installed.

The [formatter owner](formatter/README.md) executes an explicitly selected rustfmt through `native_process` and retains its actual physical output separately from canonical token identity.
Formatting has no installation or compiler-acceptance authority.

The [prepared-set owner](prepared/README.md) joins a complete physical output set to its original inventory and bounds the aggregate formatted bytes.
The [staging owner](staging/README.md) joins those bytes with declared authored inputs and requires actual compilation, source comparison and prepared-file dependency membership.
The [destination owner](destination/README.md) owns read-only complete-output checks and preflighted, recoverable installation of an actually compiled prepared set.

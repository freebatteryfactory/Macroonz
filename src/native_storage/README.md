# Native storage

This home owns directory-contained byte custody for immutable, named batches.
It is available through the root package's `native-tooling` feature.
Unix and Windows select the native implementation; other targets retain the API and refuse effects with `StorageError::Unavailable`.

## Declared input

Open an existing, explicitly supplied directory with `StorageRoot::open`.
Admit payloads with `StorageBatch::informed`, supplying limits on both artifact count and aggregate bytes.
`StorageName` accepts one to ninety-six lowercase ASCII letters, digits, underscores or hyphens without normalization.
These are mechanical names, not archive identities or arbitrary destination paths.
Physical names receive an `item-` prefix, so even logical names such as `con` do not select a Windows device.

The root directory capability contains subsequent accesses, including resolution through links.
Opening the explicitly supplied root is the only ambient path operation.
No parent discovery, environment lookup, clock, entropy or semantic judgment enters this home.

## Transaction

`StorageTransaction::begin` exclusively creates one batch directory.
Call `write_next` until it returns `None`, then consume the transaction with `commit`.
Each payload uses exclusive creation and is flushed before its write is counted.
Commit refuses any remaining payload, flushes the complete mechanical inventory to a prepared marker, then publishes it by a non-overwriting hard link named `.committed`.
No fallible filesystem operation follows successful link publication.
Readers require this publication marker, refuse missing or extra payloads against its exact roster, and return the whole bounded payload inventory in lexical name order.
An error never returns a partial payload collection.

An operating-system file lock excludes cooperating readers, writers and recovery operations across the entire root.
Contention returns `Busy` without waiting.
Dropping a transaction releases custody but deliberately leaves its unpublished directory for recovery.
After an error, release that transaction before attempting recovery.
`recover` refuses a published batch and checks the bounded existing inventory against the newly declared payload names before removing anything.
It removes the declared unpublished files and prepared marker, then returns a transaction that writes the supplied batch anew.
An interruption during removal or replacement leaves the same recoverable unpublished state.
Unknown entries, links and non-file payloads refuse recovery rather than authorizing recursive deletion.

## Physical representation and limits

The root contains `.macroonz-storage-lock` and one `item-NAME` directory per batch.
Each batch contains `item-NAME` payload files plus `.prepared` and `.committed` control files when those stages were reached.
The published inventory is UTF-8 `macroonz-storage/1` followed by LF, then each logical payload name in strictly increasing lexical order followed by LF.
The roster is nonempty, and repetitions, noncanonical names or a missing terminal LF refuse.
This is the physical file roster, not an archive envelope or content-identity encoding.
The lock file is retained across operations and must not be removed while the root is in use.
The markers describe publication by this protocol; they carry no content digest or independent attestation.
Locks coordinate users of this protocol and do not authenticate files or stop unrelated software from altering them.
Storage readers reject malformed published inventories and non-regular entries; semantic corruption remains the byte owner's question.

Payload reads consume at most the remaining aggregate byte allowance plus one overflow-detection byte for the current file.
Inventory collection stops at the declared artifact bound; control files do not consume that bound.
The published roster read is bounded by its header plus ninety-seven bytes per permitted artifact, with one overflow-detection byte.
Bounds limit payload resources, not operating-system scheduling or I/O latency.
File flushing and process-interruption recovery do not establish power-loss durability for directory entries or every filesystem.
Unsupported locking or hard-link operations return the operating-system error without silently weakening the protocol.

## Composition and authority

Pass canonical bytes from the existing [archive owner](../../harness/src/report/archive/README.md) into a batch and pass loaded bytes back to that owner's bounded reader.
Loading does not manufacture execution evidence, current replay authority or human proposal/depot admission.
This home owns no replacement archive format, identity scheme, replay runner, publication inventory or authored-file installation policy.
The compiler's publication owner remains responsible for which generated destinations it owns.

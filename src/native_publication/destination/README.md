# Publication destinations

This home checks and installs complete publication sets against explicitly recorded destination ownership and current physical files.
The caller selects an existing destination root and bounds for the complete previous/expected path union, physical bytes and ownership metadata.
The root contains one publication ownership roster; independent compiler operations must compose their complete output inventory or select distinct destination roots.
Ordinary authored neighbors are outside that roster and are never inferred to be generated from their location, filename or content.

`PublicationDestination::open` resolves the supplied root and opens that directory capability without creating anything.
The resolved root also lets the command owner exclude disposable workspace and declared compiler output writes from the destination tree.
`check` takes a shared lock only when the existing storage lock file is present, reads bounded ownership and destination bytes, and returns every reached discrepancy in lexical path order.
Neither call creates a lock, marker, directory or file.
An installed ownership record without its required cooperating lock refuses.
An absent ownership record remains uninitialized, even when there are no expected outputs.

The current record lives at `.macroonz-publication/current`.
It is canonical compact JSON followed by LF: the array `['macroonz-publication/1', files]`, with JSON double quotes and files in strictly increasing path order.
Each file is an array of its portable relative path, canonical digest as thirty-two byte integers, physical digest in the same form, and physical byte length.
Exact paths, count/byte bounds and alias/parent refusal reuse the inventory owner.
Invalid types, extra fields, ordering, duplicate paths, noncanonical bytes and unknown formats refuse.
The record describes prior installation claims; it is not a compiler observation or an authentication mechanism.

Checking inspects only the union of recorded and expected paths and their directory prefixes.
Missing files, existing unowned destinations, stale expected material, physical tampering and extra previously owned paths remain distinct observations.
An installation journal at `.macroonz-publication/item-pending` prevents a current result.
Links and incompatible filesystem kinds refuse instead of expanding traversal or overwrite authority.
The directory capability contains accesses; the lock coordinates participating operations and does not prevent unrelated software from changing files concurrently.
Unix and Windows select native operations; other targets retain the API and report unavailability.

## Installation and recovery

`begin` requires the staging owner's actual `CompiledPublication` and acquires exclusive destination custody.
It reads the complete old/new destination union before changing any output and refuses existing unowned destinations or modified previously owned files.
Missing owned files may be repaired, stale owned files replaced and retired owned files removed.
The file bound covers that union, and the byte bound admits the sum of the larger old/new length at each path so every intermediate installation remains inspectable within the declared bound.
Only admitted output paths are changed; authored staging inputs are never installed implicitly.

Before the first output change, an immutable native-storage batch at `.macroonz-publication/item-pending` commits an installation intent.
Its `intent` payload is canonical compact JSON followed by LF: an array containing `macroonz-publication-intent/1`, the previous ownership-record bytes or null, the next ownership-record bytes, and physical payload byte arrays in next-record order.
Byte arrays contain JSON integer bytes, including the encoded ownership records.
Admission checks exact payload commitments and the complete old/new path union.
The encoded journal is bounded by four times the physical byte limit plus eight times the metadata limit plus four times the file limit plus 256 bytes; arithmetic overflow refuses.
These retained claims enable recovery and cannot reconstruct a fresh compiler observation or authenticate the host filesystem.

`write_next` applies one lexically ordered replacement or removal and advances only after success.
Each step first checks that the committed journal still contains this installation's complete intent.
A replacement is written and synchronized in the reserved control directory, then renamed over an owned destination or linked into an absent destination without overwriting an intervening creation.
A leftover temporary link is removed before a new temporary file is exclusively created; its inode is never truncated for reuse.
Current files must still match their old or intended new bytes, and a removal applies only to the explicitly retired owned path.
There is no recursive output deletion or whole-directory ownership inference.

`commit` refuses unfinished steps, changed journal material, changed output or a conflicting ownership record.
It verifies the complete resulting set, replaces the current ownership record last, then retires the journal through the storage owner's bounded exact-payload removal.
Dropping custody leaves the committed intent available for explicit recovery.
`recover` reacquires exclusive custody, admits the retained intent and compatible current output, and restarts its idempotent file operations.
An interruption after ownership replacement but during journal retirement is completed as retirement; a retirement error can therefore follow successfully installed output and remains an error to the caller.
Uncommitted preparation can be restarted only from a fresh compiled publication; an empty preparation or retirement remainder may be removed without output changes.

Readers that ignore the cooperating lock may observe an intermediate mixture of old and new files.
The protocol handles interrupted processes and bounded recovery; it does not claim multi-file atomic visibility, power-loss durability, authentication of caller-authored compilation fixtures or exclusion of nonparticipating filesystem writers.

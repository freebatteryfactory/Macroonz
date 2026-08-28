# structural — syntax read independently

This home maps rendered Rust through a foreign parser and compares the resulting syntax with a caller-authored declaration.

It can establish what implementations, paths, postures, attributes, and members an artifact declares.
It cannot establish that any path resolves, that the artifact typechecks, or that a constant evaluates to the value its syntax suggests.

Every complete observation carries paths as a root posture plus indivisible typed segments.
The constructor refuses empty segments and embedded `::` separators so different segment rosters cannot name one path.

An observed member roster retains duplicates because repetition in the artifact is a finding.
A caller's declared member roster refuses duplicate names before it exists so one expected member has one authority.

Parsing is this home's operation and remains mounted at the established parent path.

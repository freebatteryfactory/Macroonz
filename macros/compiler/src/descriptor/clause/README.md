# `clause` — mechanical reading shared by descriptor grammars

Descriptor kinds own their words, rows, rosters, and semantic declarations.
This home owns only the punctuation mechanics they share: comma-separated groups, assignment values, one-time seat filling, and the primitive identifier, text, number, and direct-binding readings.
It admits assignment-shaped clauses against a caller-supplied key roster, refuses repeated keys, and carries grammar-owned nested clauses without interpreting them.
Namespaced references are read here because their token shape is common while their meaning remains with the receiving grammar.

A caller supplies its concrete refusal constructors, so the operation preserves the diagnostic family and exact authored site owned by that grammar.
No generic operation here decides which key is lawful or what a captured value means.

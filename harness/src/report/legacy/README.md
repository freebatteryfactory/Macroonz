# legacy — sparse historical JSON

This home admits the supplied fields of a lossy historical `witness` record.
It owns one bounded JSON shape, exact source custody and explicit missing, null and present data.
The caller independently supplies the expected `kind` and `schema` labels.
Those labels select the caller's convention; they do not authenticate the source or change this reader's field grammar.

## Fields and presence

The source is one JSON object with these optional members:

| Member | Present value |
| --- | --- |
| `kind` | Text equal to the independently expected `kind`; null refuses. |
| `schema` | A `u64` integer equal to the independently expected `schema`; null refuses. |
| `witness` | An array of integer octets. |
| `input_profile` | An object with optional name text and revision `u64` members. |
| `trial_name`, `subject_name`, `check_name` | Text retained without current name admission. |
| `subject_revision`, `check_revision` | Opaque `u64` markers, not current executable revision addresses. |
| `target`, `toolchain` | Text retained as source claims. |
| `execution_digest`, `fingerprint_digest` | Exactly sixty-four lowercase hexadecimal digits, retained through the archive owner's `AddressClaim`. |
| `reported_outcome` | Uninterpreted text, never a current conclusion. |

Except for `kind` and `schema`, every member may be null.
Missing, null and present values remain distinct, including an empty `witness`, empty strings, zero numeric markers, an empty profile object and absent or null profile leaves.
No absent header is supplied from the caller's expected labels.
Free-form names cannot be split into current namespaces, and numeric revision markers cannot become current revision addresses.
Only supplied representations that actually match a current coordinate permit a claim comparison.

Unknown members, duplicate decoded member names, wrong value types, malformed UTF-8 or escapes, nonintegral or out-of-range numbers, and trailing material refuse the record.
Escaped and unescaped spellings of the same member are duplicates.
Text is neither trimmed nor normalized.
The exact original JSON bytes remain available independently of decoded inspection.

## Bounds

`LegacyLimits` bounds source bytes before parsing.
It separately bounds decoded text bytes per value, `witness` population, total object members and container depth.
The root object has depth one; the `witness` array and input-profile object have depth two.
Object-member accounting includes the profile's members.
Only the declared shapes are consumed; there is no unrestricted generic JSON tree.

The retained-byte budget charges the exact source length, decoded text lengths, `witness` octets, eight bytes per present numeric value and thirty-two bytes per present digest.
The charge is checked before each owned copy or collection growth; source storage is copied only after complete admission.
Fixed record storage, collection capacity and allocator overhead are not an exact heap quota.
Serde owns JSON syntax and may decode escaped strings into temporary scratch before a visitor runs.
That scratch is bounded by the admitted source, rather than by the individual retained-field ceiling.
The reader claims no allocator measurement or arbitrary caller-code budget.

## Authority and composition

A loaded `LegacyRecord` supplies historical claims only.
It cannot become a current report, execution key, complete archived capsule, authenticated source or replay permission.
`AddressClaim` preserves the existing archive owner's distinction between a digest claim and a derived `ContentAddress`.
This home executes nothing and touches no filesystem or host state.
Current decoding and execution remain the input and runner owners' operations, using independently supplied current bindings.

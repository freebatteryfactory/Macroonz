//! The closed context grammar of declared names, evaluated at compile time.

/// Whether a roster of declared names really separates: every name inside the context grammar, and no name declared twice.
///
/// Written for the `const` block the `subjects!` roster stamp emits, and evaluated at compile time — a name that would collapse two key spaces is a compile error rather than a defect a reader has to notice.
///
/// The grammar is the closed one [`Subject::NAME`](crate::identity::Subject::NAME) declares: lowercase ASCII letters and digits in `-`-joined segments, with no leading, trailing, or doubled separator.
#[must_use]
pub const fn names_are_separating(names: &[&str]) -> bool {
    match names.split_first() {
        None => true,
        Some((first, rest)) => {
            name_is_grammatical(first) && !names_contain(rest, first) && names_are_separating(rest)
        }
    }
}

/// Where in a `-`-joined segment the grammar walk stands.
#[derive(Clone, Copy)]
enum Segment {
    /// No character of the current segment has been read yet.
    Opening,
    /// At least one character of the current segment has been read.
    Inside,
}

/// Whether one declared name stands inside the closed context grammar.
pub(crate) const fn name_is_grammatical(name: &str) -> bool {
    grammatical(name.as_bytes(), Segment::Opening)
}

/// The grammar walk, carrying where in a segment the reader stands.
///
/// It opens at [`Segment::Opening`], so an empty name and a leading separator both refuse; it ends well only from [`Segment::Inside`], so a trailing separator refuses too.
const fn grammatical(bytes: &[u8], at: Segment) -> bool {
    match bytes.split_first() {
        None => matches!(at, Segment::Inside),
        Some((byte, rest)) => {
            if *byte == b'-' {
                matches!(at, Segment::Inside) && grammatical(rest, Segment::Opening)
            } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
                grammatical(rest, Segment::Inside)
            } else {
                false
            }
        }
    }
}

/// Whether a roster already carries one declared name.
const fn names_contain(names: &[&str], name: &str) -> bool {
    match names.split_first() {
        None => false,
        Some((first, rest)) => {
            same_bytes(first.as_bytes(), name.as_bytes()) || names_contain(rest, name)
        }
    }
}

/// Whether two declared names are the same bytes.
const fn same_bytes(left: &[u8], right: &[u8]) -> bool {
    match (left.split_first(), right.split_first()) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some((here, left_rest)), Some((there, right_rest))) => {
            *here == *there && same_bytes(left_rest, right_rest)
        }
    }
}

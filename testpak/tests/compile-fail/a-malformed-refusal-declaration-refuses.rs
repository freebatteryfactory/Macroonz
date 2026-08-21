//! The derive's refusal reaches the compiler, at the offending token.
//!
//! A shape word outside the machine's roster is not silently ignored and does
//! not expand to nothing. The services establish the cause and the token it sits
//! at, the shell resolves that token's handle to the exact compiler span, and
//! the build stops — the difference between a refusal and a smaller success.

use threadpak_macros::RefusalFamily;

#[derive(RefusalFamily)]
#[refusal(family = "fixture.demo", shape = tri_state, order(NotCanonical = "not-canonical"))]
enum MalformedFamily {
    NotCanonical,
}

fn main() {
    let _ = MalformedFamily::NotCanonical;
}

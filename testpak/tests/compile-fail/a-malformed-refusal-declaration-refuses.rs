//! The derive's refusal reaches the compiler.
//!
//! A shape word outside the machine's roster is not silently ignored and does
//! not expand to nothing. The services establish the cause and the byte it sits
//! at, the shell projects that into `compile_error!`, and the build stops — the
//! difference between a refusal and a smaller success.

use threadpak_macros::RefusalFamilyDerive;

#[derive(RefusalFamilyDerive)]
#[refusal(shape = tri_state, order(NotCanonical = "fixture.demo.not-canonical"))]
enum MalformedFamily {
    NotCanonical,
}

fn main() {
    let _ = MalformedFamily::NotCanonical;
}

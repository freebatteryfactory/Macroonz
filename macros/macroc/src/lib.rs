//! `threadpak-macroc`: the metaprogramming services.
//!
//! The services are ordinary callable Rust — planning, rendering, inspection,
//! explanation — reached the same way by any caller. They depend inward on
//! the machine and never back outward: nothing here knows a proc-macro
//! exists. The Rust-facing expansion shell (`threadpak-macros`) is one thin
//! surface over this crate; a future language frontend would be another.
//!
//! This file is a topology skeleton. It carries exactly enough to prove the
//! inward edge is load-bearing rather than decorative: a public function
//! whose signature names a public type owned by band 13 of the machine.

/// The machine's frontend-role type, re-exported rather than restated. The
/// expansion shell reaches the machine's vocabulary through the services, so
/// the shell needs no edge of its own to the machine and no copy of the type.
pub use threadpak::declaration::FrontendRole;

/// Names the front door a role stands for.
#[must_use]
pub const fn describe_frontend_role(role: FrontendRole) -> &'static str {
    match role {
        FrontendRole::RustDeclaration => "the live Rust-declaration front door",
        FrontendRole::ApplicationLanguage => "the pluggable application-language front door",
    }
}

#[cfg(test)]
mod tests {
    use super::{FrontendRole, describe_frontend_role};
    use threadpak_macros::ThreadpakSkeleton;

    /// A consumer shaped like an application: it holds a type owned by the
    /// machine and wears a derive owned by the expansion shell.
    #[derive(ThreadpakSkeleton)]
    struct Consumer {
        role: FrontendRole,
    }

    #[test]
    fn a_consumer_composes_the_machine_and_the_shell() {
        let consumer = Consumer {
            role: FrontendRole::RustDeclaration,
        };
        assert_eq!(
            describe_frontend_role(consumer.role),
            "the live Rust-declaration front door"
        );
    }
}

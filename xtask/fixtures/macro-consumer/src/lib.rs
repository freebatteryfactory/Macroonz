//! The outside consumer fixture: the composition proof, shaped like a real
//! consumer.
//!
//! A compiler service never depends on its frontend surfaces, even for tests.
//! So the question "does a caller who holds the machine's types and wears the
//! shell's derive actually compile?" cannot be answered from inside either
//! participant — an answer from inside is the participant grading itself, and
//! it buys that answer with a dependency edge that reverses the topology.
//!
//! This crate answers it from outside. It depends on `threadpak` and on
//! `threadpak-macros`, exactly as an application would, and on neither of their
//! internals. It exports nothing: the whole crate is the fixture, and its one
//! test is the proof.
//!
//! It lives under `xtask/fixtures/` because it is tooling — first-class, never
//! on the production dependency path.

#[cfg(test)]
mod tests {
    use threadpak::declaration::FrontendRole;
    use threadpak_macros::ThreadpakSkeleton;

    /// A consumer shaped like an application: it holds a type owned by the
    /// machine and wears a derive owned by the expansion shell. Nothing here
    /// reaches the services the shell is a surface over.
    #[derive(ThreadpakSkeleton)]
    struct Consumer {
        role: FrontendRole,
    }

    /// The composition proof: the machine's public type and the shell's derive
    /// meet on one struct in a crate that owns neither.
    #[test]
    fn a_consumer_composes_the_machine_and_the_shell() {
        let consumer = Consumer {
            role: FrontendRole::RustDeclaration,
        };
        assert!(matches!(consumer.role, FrontendRole::RustDeclaration));
        assert_ne!(consumer.role, FrontendRole::ApplicationLanguage);
    }
}

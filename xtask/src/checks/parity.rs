//! The working law is one law written into two files, and it stays one law.
//!
//! `AGENTS.md` and `CLAUDE.md` are read by different tools and edited by
//! different hands. Byte-identity is the only claim that survives that: any
//! weaker comparison lets the two drift into two laws that both look official.

use std::fs;
use std::path::Path;

/// `AGENTS.md` and `CLAUDE.md` carry the same working law and must stay
/// byte-identical.
pub(crate) fn check_agents_claude_parity(root: &Path) -> Result<(), String> {
    let agents = fs::read(root.join("AGENTS.md")).map_err(|e| format!("AGENTS.md: {e}"))?;
    let claude = fs::read(root.join("CLAUDE.md")).map_err(|e| format!("CLAUDE.md: {e}"))?;
    if agents == claude {
        Ok(())
    } else {
        Err(String::from("AGENTS.md and CLAUDE.md differ"))
    }
}

#[cfg(test)]
mod tests {
    use super::check_agents_claude_parity;
    use crate::checks::scratch::Scratch;

    /// Planted reversal: the two working-law files drift apart. One of them
    /// edited alone is exactly how a working law stops being one law.
    #[test]
    fn a_drifted_working_law_pair_is_a_violation() {
        let scratch = Scratch::named("agents-parity");
        scratch.write("AGENTS.md", "the working law\n");
        scratch.write("CLAUDE.md", "the working law\n");
        assert!(check_agents_claude_parity(scratch.root()).is_ok());

        scratch.write("CLAUDE.md", "the working law, edited on one side only\n");
        let found = check_agents_claude_parity(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("differ")));
    }
}

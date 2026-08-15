//! The working law is one law written into two files, and it stays one law.
//!
//! `AGENTS.md` and `CLAUDE.md` are read by different tools and edited by
//! different hands. Byte-identity is the only claim that survives that: any
//! weaker comparison lets the two drift into two laws that both look official.

use crate::repository::snapshot::RepositorySnapshot;

/// The working law, in the two files that carry it.
const WORKING_LAW: [&str; 2] = ["AGENTS.md", "CLAUDE.md"];

/// `AGENTS.md` and `CLAUDE.md` carry the same working law and must stay
/// byte-identical.
///
/// Both are read out of the one snapshot, so the comparison is between the
/// bytes one reading took — not between two reads that could have happened at
/// two moments.
pub(crate) fn check_agents_claude_parity(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let mut carried = Vec::new();
    for named in WORKING_LAW {
        carried.push(snapshot.files().bytes(named).taken(named)?);
    }
    if carried.windows(2).all(|pair| pair.first() == pair.last()) {
        Ok(())
    } else {
        Err(format!("{} and {} differ", WORKING_LAW[0], WORKING_LAW[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::check_agents_claude_parity;
    use crate::checks::scratch::Scratch;

    /// Planted reversal: the two working-law files drift apart. One of them
    /// edited alone is exactly how a working law stops being one law.
    #[test]
    fn a_drifted_working_law_pair_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("agents-parity")?;
        scratch.write("AGENTS.md", "the working law\n")?;
        scratch.write("CLAUDE.md", "the working law\n")?;
        assert!(check_agents_claude_parity(&scratch.read()?).is_ok());

        scratch.write("CLAUDE.md", "the working law, edited on one side only\n")?;
        let found = check_agents_claude_parity(&scratch.read()?);
        assert!(found.is_err_and(|reason| reason.contains("differ")));
        Ok(())
    }

    /// Planted reversal: one of the two files is not there at all. Absence is
    /// UNKNOWN rather than agreement — a reading that answered a missing file
    /// with empty bytes would have found two empty files identical.
    #[test]
    fn a_missing_half_of_the_working_law_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("agents-parity-missing")?;
        scratch.write("AGENTS.md", "the working law\n")?;
        let found = check_agents_claude_parity(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("CLAUDE.md") && reason.contains("not there")),
            "a missing working-law file read as agreement"
        );
        Ok(())
    }
}

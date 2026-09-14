//! Admission of the canonical strict policy and its exact Instant-only derivative.

use super::PolicyProfiles;

impl PolicyProfiles {
    pub(crate) fn derive(strict: String) -> Result<Self, String> {
        let marker = r#"path = "std::time::Instant""#;
        if strict.matches(marker).count() != 1 {
            return Err("the strict policy must contain exactly one Instant entry".to_owned());
        }
        let section = "disallowed-types = [\n";
        if strict.matches(section).count() != 1 {
            return Err("the disallowed-types section has an unexpected shape".to_owned());
        }
        let (before, remaining) = strict
            .split_once(section)
            .ok_or("the disallowed-types section is absent")?;
        if !before.is_empty() && !before.ends_with('\n') {
            return Err("the disallowed-types section is not a standalone entry".to_owned());
        }
        let (entries, _) = remaining
            .split_once("]\n")
            .ok_or("the disallowed-types section is not closed")?;
        let entry = entries
            .lines()
            .find(|line| line.contains(marker))
            .ok_or("the Instant restriction is outside disallowed-types")?;
        let reason = entry
            .trim()
            .strip_prefix(r#"{ path = "std::time::Instant", reason = ""#)
            .and_then(|value| value.strip_suffix(r#"" },"#))
            .ok_or("the Instant restriction has an unexpected shape")?;
        if reason.contains(['"', '\\']) {
            return Err("the Instant reason requires an unescaped plain string".to_owned());
        }
        let mut native = String::new();
        for line in strict.split_inclusive('\n') {
            if !line.contains(marker) {
                native.push_str(line);
            }
        }
        Ok(Self { strict, native })
    }

    pub(crate) fn strict(&self) -> &str {
        &self.strict
    }

    pub(crate) fn native(&self) -> &str {
        &self.native
    }
}

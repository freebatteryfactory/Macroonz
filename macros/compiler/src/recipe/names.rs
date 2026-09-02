//! Mechanical generated-name derivation shared by preflight and projection.

/// Derive one stable upper-snake companion constant from a caller-owned identifier.
pub(super) fn companion_constant(name: &str, suffix: &str) -> String {
    let name = name.strip_prefix("r#").unwrap_or(name);
    let mut generated = String::new();
    let mut previous_lowercase = false;
    for character in name.chars() {
        if character.is_uppercase() && previous_lowercase {
            generated.push('_');
        }
        for uppercase in character.to_uppercase() {
            generated.push(uppercase);
        }
        previous_lowercase = character.is_lowercase() || character.is_numeric();
    }
    generated.push('_');
    generated.push_str(suffix);
    generated
}

/// Compare Rust identifier spellings after removing raw-identifier syntax.
pub(super) fn identifier_key(spelling: &str) -> &str {
    spelling.strip_prefix("r#").unwrap_or(spelling)
}

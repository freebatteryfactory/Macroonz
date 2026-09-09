//! Exact policy derivation and the explicitly invoked scoped Clippy wall.

mod check;
mod consumer;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod inventory;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod ownership;
mod probes;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod storage;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod transaction;
mod types;
#[cfg(feature = "harness")]
mod workflow;

use types::PolicyProfiles;

#[test]
fn deriving_native_permission_preserves_every_other_policy_byte() -> Result<(), String> {
    let before = "msrv = \"1.98.1\"\ndisallowed-types = [\n";
    let entry = "    { path = \"std::time::Instant\", reason = \"declared clock\" },\n";
    let after = "    { path = \"std::time::SystemTime\", reason = \"still forbidden\" },\n]\n";
    let strict = format!("{before}{entry}{after}");
    let profiles = PolicyProfiles::derive(strict.clone())?;
    assert_eq!(profiles.strict(), strict);
    assert_eq!(profiles.native(), format!("{before}{after}"));
    Ok(())
}

#[test]
fn missing_duplicated_or_reframed_permission_refuses_derivation() {
    let entry = "    { path = \"std::time::Instant\", reason = \"declared clock\" },\n";
    for raw in [
        String::new(),
        format!("{entry}{entry}"),
        "disallowed-types = [\"std::time::Instant\"]\n".to_owned(),
        "# path = \"std::time::Instant\"\n".to_owned(),
        "other = { path = \"std::time::Instant\", reason = \"different seat\" },\n".to_owned(),
        format!("disallowed-types = [\n]\ndisallowed-methods = [\n{entry}]\n"),
        format!("# disallowed-types = [\n{entry}]\n"),
        "disallowed-types = [\n{ path = \"std::time::Instant\", reason = \"one\" }, { path = \"std::time::SystemTime\", reason = \"two\" },\n]\n".to_owned(),
    ] {
        assert!(PolicyProfiles::derive(raw).is_err());
    }
}

#[test]
#[ignore = "Runs the required scoped Clippy wall and independent refusal subjects."]
fn scoped_clippy_wall() -> Result<(), String> {
    check::qualify()
}

#[test]
fn native_storage_dependencies_require_the_native_feature_and_target() -> Result<(), String> {
    consumer::qualify_graphs()
}

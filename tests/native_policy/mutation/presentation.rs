//! Read-only presentation assertions at existing native mutation crossings.

use crate::presentation_formats::{field, parsed};
use macroonz::harness::muterprater::backend_archive::ArchivedBackendManifest;
use macroonz::native_mutation::{MutationError, MutationOutput, PendingMutation};
use macroonz::presentation;
use serde_json::json;
use std::fmt::Write as _;

pub(super) fn accepted(output: &MutationOutput) -> Result<(), String> {
    let value = parsed(&presentation::native_mutation(output))?;
    let manifest = output.manifest().map_err(ToString::to_string)?;
    let imported = parsed(&presentation::backend_manifest(manifest))?;
    assert_eq!(field(&value, "/record/manifest/kind")?, "established");
    assert_eq!(
        field(&value, "/record/manifest/value")?,
        field(&imported, "/record")?
    );
    assert_eq!(field(&value, "/record/process/state")?, "finished");
    assert_eq!(field(&value, "/record/qualification/kind")?, "established");
    assert_eq!(
        field(&value, "/record/qualification/value/standing")?,
        &json!({"kind":"checked", "value":"27.0.0"})
    );
    assert_eq!(
        field(&value, "/record/qualification/value/profile/ceiling")?,
        "witness-rejection"
    );
    assert_eq!(
        field(&value, "/record/request/process/arguments")?,
        &json!(output.request().process().arguments())
    );
    assert_eq!(
        field(&value, "/record/backend_version/status/code")?,
        &json!(0i32)
    );
    assert_eq!(
        field(&value, "/record/compiler_version/status/code")?,
        &json!(0i32)
    );
    originals(&value, output)
}

fn originals(value: &serde_json::Value, output: &MutationOutput) -> Result<(), String> {
    assert_eq!(
        field(value, "/record/original_sources")?
            .as_array()
            .ok_or("missing original sources")?
            .len(),
        output.original_sources().count()
    );
    for (at, (file, bytes)) in output.original_sources().enumerate() {
        let path = format!("/record/original_sources/{at}");
        assert_eq!(field(value, &format!("{path}/file"))?, file);
        let mut expected = String::new();
        for byte in bytes {
            write!(&mut expected, "{byte:02x}").map_err(|error| error.to_string())?;
        }
        assert_eq!(
            field(value, &format!("{path}/bytes"))?.as_str(),
            Some(expected.as_str())
        );
    }
    Ok(())
}

pub(super) fn archived(
    output: &MutationOutput,
    historical: &ArchivedBackendManifest,
) -> Result<(), String> {
    let current = parsed(&presentation::backend_manifest(
        output.manifest().map_err(ToString::to_string)?,
    ))?;
    let value = parsed(&presentation::archived_backend(historical))?;
    assert_eq!(field(&value, "/standing")?, "historical-unauthenticated");
    for path in [
        "/record/invocation",
        "/record/output",
        "/record/reading/profile",
        "/record/reading/unparsed",
        "/record/reading/announced",
    ] {
        assert_eq!(field(&value, path)?, field(&current, path)?);
    }
    assert!(field(&value, "/record/original_console")?.is_string());
    assert!(field(&value, "/record/sources/0/original")?.is_string());
    Ok(())
}

pub(super) fn observation_failure(output: &MutationOutput, cause: &str) -> Result<(), String> {
    let value = parsed(&presentation::native_mutation(output))?;
    assert_eq!(field(&value, "/record/manifest/kind")?, "refused");
    assert_eq!(
        field(&value, "/record/manifest/value/phase")?,
        "mutation-observation"
    );
    assert_eq!(field(&value, "/record/manifest/value/cause/kind")?, cause);
    assert_eq!(
        field(&value, "/record/qualification")?,
        field(&value, "/record/manifest")?
    );
    assert!(value.pointer("/record/manifest/value/reading").is_none());
    originals(&value, output)
}

pub(super) fn query(error: &MutationError, phase: &str, state: &str) -> Result<(), String> {
    let value = parsed(&presentation::mutation_error(error))?;
    assert_eq!(field(&value, "/record/phase")?, "preparation");
    assert_eq!(field(&value, "/record/cause/kind")?, "query");
    assert_eq!(field(&value, "/record/cause/value/phase")?, phase);
    assert_eq!(field(&value, "/record/cause/value/run/state")?, state);
    let MutationError::Query { request, cause, .. } = error else {
        return Err("expected query refusal".to_owned());
    };
    assert_eq!(
        field(&value, "/record/cause/value/request/arguments")?,
        &json!(request.arguments())
    );
    assert_eq!(
        field(&value, "/record/cause/value/cause")?.as_str(),
        Some(cause.as_str())
    );
    assert!(value.pointer("/record/manifest").is_none());
    Ok(())
}

pub(super) fn pending(record: &PendingMutation) -> Result<(), String> {
    let value = parsed(&presentation::pending_mutation(record))?;
    assert_eq!(field(&value, "/record/state")?, "pending-cleanup");
    assert!(value.pointer("/record/manifest").is_none());
    assert!(value.pointer("/record/process/status").is_none());
    Ok(())
}

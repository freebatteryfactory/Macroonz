use super::command_fixture::{generate, preparation};
use super::configure::{publication, standin as formatter_body};
use super::destination_fixture::snapshot;
use super::fixture;
use crate::compiler::configure::{bounds, host, root, standin, tool};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::native_publication::{BakeCommand, BakeOutput, Publication, bake};
use macroonz::presentation::{bake_error, bake_output};

#[test]
fn presentation_preserves_declared_order_landings_empty_inventory_and_caller_refusal()
-> Result<(), String> {
    let calls = std::cell::Cell::new(0usize);
    let output = bake(BakeCommand::Prepare, || {
        calls.set(calls.get().saturating_add(1));
        publication()
    })
    .map_err(|error| error.to_string())?;
    let shown = parsed(&bake_output(&output))?;
    assert_eq!(calls.get(), 1);
    assert_eq!(field(&shown, "/record/kind")?, "declared");
    let files = field(&shown, "/record/value/files")?
        .as_array()
        .ok_or("no files")?;
    assert_eq!(files.len(), 4);
    for (index, (path, site, address)) in [
        ("generated/definition.rs", None, fixture::FIRST),
        ("generated/alpha.rs", Some("alpha"), fixture::FIRST),
        ("generated/beta.rs", Some("beta"), fixture::FIRST),
        ("generated/other.rs", None, fixture::SECOND),
    ]
    .into_iter()
    .enumerate()
    {
        let file = files.get(index).ok_or("missing file")?;
        assert_eq!(field(file, "/path")?, path);
        assert_eq!(field(file, "/site")?.as_str(), site);
        assert_eq!(field(file, "/unit/address/subject")?, address.subject);
        assert_eq!(
            decoded_hex(field(file, "/unit/address/bytes")?)?,
            address.bytes
        );
    }
    let empty = Publication::declared(fixture::inline_only()?, &[], fixture::LIMITS)
        .map_err(|error| error.to_string())?;
    let empty = parsed(&bake_output(&BakeOutput::Declared(empty)))?;
    assert_eq!(
        field(&empty, "/record/value/files")?,
        &serde_json::json!([])
    );
    let caller = ("Passed|<script>caller</script>\n", u64::MAX);
    let refused = bake::<super::types::Example, _>(BakeCommand::Prepare, || Err(caller))
        .err()
        .ok_or("caller refusal lost")?;
    let failure = parsed(&bake_error(&refused))?;
    assert_eq!(field(&failure, "/record/cause/kind")?, "declaration");
    assert_eq!(field(&failure, "/record/cause/value/owner")?, "caller");
    assert_eq!(
        field(&failure, "/record/cause/value/representation")?,
        "debug-text"
    );
    assert_eq!(
        field(&failure, "/record/cause/value/detail")?,
        &format!("{caller:?}")
    );
    assert_eq!(field(&failure, "/record/pending_cleanup")?, false);
    assert!(failure.pointer("/record/verdict").is_none());
    Ok(())
}

#[test]
fn presentation_keeps_completed_formatting_before_the_failed_file() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let mut preparation = preparation(&source, &work, &host)?;
    let body = formatter_body(
        "use std::io::Read; let mut input=String::new(); std::io::stdin().read_to_string(&mut input)?; if input.contains(\"OTHER\") { std::io::stderr().write_all(b\"Passed|<script>failure</script>\\n\")?; return Err(std::io::Error::other(\"formatter refused\")); } std::io::stdout().write_all(input.as_bytes())?;",
    );
    let executable = standin(&source, &host, "partial-format", &body)?;
    preparation.formatter.as_mut().ok_or("no formatter")?.tool =
        tool(&host, &executable, &source, bounds()?)?;
    let refusal = bake(BakeCommand::Inspect(Box::new(preparation)), publication)
        .err()
        .ok_or("failed formatting returned output")?;
    let before = snapshot(&work)?;
    let shown = parsed(&bake_error(&refusal))?;
    assert_eq!(snapshot(&work)?, before);
    assert_eq!(field(&shown, "/record/cause/kind")?, "formatting");
    let completed = field(&shown, "/record/cause/value/completed")?
        .as_array()
        .ok_or("missing completed prefix")?;
    assert_eq!(completed.len(), 3);
    for (value, path) in completed.iter().zip([
        "generated/definition.rs",
        "generated/alpha.rs",
        "generated/beta.rs",
    ]) {
        assert_eq!(field(value, "/path")?, path);
        assert_eq!(field(value, "/source/kind")?, "admitted");
    }
    let run = field(&shown, "/record/cause/value/run/value")?;
    assert_eq!(field(run, "/path")?, "generated/other.rs");
    assert_eq!(field(run, "/source/kind")?, "refused");
    assert_eq!(field(run, "/source/value/kind")?, "process");
    assert_eq!(field(run, "/process/status/success")?, false);
    assert!(
        field(run, "/process/stderr/shown")?
            .as_str()
            .ok_or("no stderr")?
            .contains("Passed|<script>failure</script>")
    );
    assert_eq!(field(&shown, "/record/pending_cleanup")?, false);
    Ok(())
}

#[test]
fn presentation_retains_successful_compilation_when_destination_installation_refuses()
-> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    std::fs::create_dir(output.join("generated")).map_err(|error| error.to_string())?;
    std::fs::write(output.join("generated/other.rs"), b"authored neighbor")
        .map_err(|error| error.to_string())?;
    let mut preparation = preparation(&source, &work, &host)?;
    preparation.formatter = None;
    let refusal = bake(
        generate(preparation, &source, &output, &host, bounds()?)?,
        publication,
    )
    .err()
    .ok_or("unowned output was overwritten")?;
    assert_eq!(
        std::fs::read(output.join("generated/other.rs")).map_err(|error| error.to_string())?,
        b"authored neighbor"
    );
    assert!(!output.join(".macroonz-publication").exists());
    let before = snapshot(&output)?;
    let shown = parsed(&bake_error(&refusal))?;
    assert_eq!(snapshot(&output)?, before);
    assert_eq!(field(&shown, "/record/cause/kind")?, "destination");
    let cause = field(&shown, "/record/cause/value")?;
    assert_eq!(field(cause, "/error/kind")?, "conflict");
    assert_eq!(field(cause, "/error/value")?, "generated/other.rs");
    assert_eq!(
        field(cause, "/compiled/compiler/process/status/success")?,
        true
    );
    assert_eq!(
        field(cause, "/compiled/prepared/files")?
            .as_array()
            .ok_or("no inventory")?
            .len(),
        4
    );
    assert!(shown.pointer("/record/verdict").is_none());
    Ok(())
}

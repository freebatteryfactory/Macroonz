use super::fixture;
use super::types::Example;
use macroonz::native_publication::{FormatOutput, FormatRun, Publication};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub(super) fn publication() -> Result<Publication<Example>, String> {
    let (expansion, stamp) = fixture::expanded([7, 9, 42])?;
    let landings = fixture::landings()?;
    Publication::declared(
        expansion,
        &fixture::bindings("generated/definition.rs", "generated/other.rs")?,
        fixture::LIMITS,
    )
    .and_then(|publication| publication.with_stamp(stamp, &landings))
    .map_err(|error| error.to_string())
}

pub(super) fn configuration(root: &Path) -> Result<PathBuf, String> {
    let path = root.join("declared.toml");
    std::fs::write(&path, b"max_width = 100\nnewline_style = \"Windows\"\n")
        .map_err(|error| error.to_string())?;
    Ok(path)
}

pub(super) fn scratch(root: &Path) -> Result<File, String> {
    std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(root.join("formatter-input"))
        .map_err(|error| error.to_string())
}

pub(super) fn finish(run: FormatRun) -> Result<FormatOutput, String> {
    match run {
        FormatRun::Finished(output) => Ok(*output),
        FormatRun::Pending(pending) => {
            let detail = format!("unexpected formatter cleanup: {:?}", pending.process());
            drop(pending.finish(Duration::from_secs(5)));
            Err(detail)
        }
    }
}

pub(super) fn standin(body: &str) -> String {
    format!(
        "use std::io::Write; fn main() -> std::io::Result<()> {{ if std::env::args().any(|value| value == \"--version\") {{ return std::io::stdout().write_all(b\"rustfmt 1.9.0-stable (independent control)\\n\"); }} {body} Ok(()) }}"
    )
}

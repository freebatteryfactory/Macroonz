//! Prepare, inspect, check, generate or recover a caller-owned publication through the root library.

mod configuration;
mod generate;
mod type_contract;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::native_publication::{BakeOutput, PreparedPublication, bake};
use serde_json::{Value, json};
use std::io::Write;

fn main() -> Result<(), String> {
    let (command, value) = configuration::read()?;
    let output = bake(command, || generate::publication(value)).map_err(|error| {
        error
            .finish_cleanup(std::time::Duration::from_secs(5))
            .to_string()
    })?;
    let (report, current) = match output {
        BakeOutput::Declared(publication) => (
            json!({ "action": "prepare", "files": publication.files().map(|file| {
            json!({ "path": file.path().spelling(), "canonical_digest": file.canonical_digest().as_bytes(), "source": file.source() })
        }).collect::<Vec<_>>() }),
            true,
        ),
        BakeOutput::Prepared(prepared) => (
            json!({ "action": "inspect", "files": files(&prepared) }),
            true,
        ),
        BakeOutput::Checked {
            prepared,
            comparison,
        } => (
            json!({
                "action": "check", "current": comparison.is_current(), "files": files(&prepared),
                "issues": comparison.issues().iter().map(|issue| json!({
                    "path": issue.path.spelling(), "problem": format!("{:?}", issue.problem),
                })).collect::<Vec<_>>(),
            }),
            comparison.is_current(),
        ),
        BakeOutput::Generated(compiled) => (
            json!({
                "action": "generate", "files": files(compiled.prepared()),
                "compiler_directory": compiled.compiler().request().process().directory().to_str().ok_or("compiler directory is not Unicode")?,
                "executable": compiled.compiler().executable().and_then(std::path::Path::to_str).ok_or("compiled executable is absent or not Unicode")?,
            }),
            true,
        ),
        BakeOutput::Recovered => (
            json!({ "action": "recover", "standing": "historical installation intent" }),
            true,
        ),
    };
    writeln!(std::io::stdout(), "{report}").map_err(|error| error.to_string())?;
    if current {
        Ok(())
    } else {
        Err("publication check found discrepancies".to_owned())
    }
}

fn files(prepared: &PreparedPublication<types::PublishedValue>) -> Vec<Value> {
    prepared
        .files()
        .map(|file| {
            json!({
                "path": file.path().spelling(),
                "canonical_digest": file.canonical_digest().as_bytes(),
                "published_digest": file.published_digest().as_bytes(),
                "bytes": file.bytes(),
            })
        })
        .collect()
}

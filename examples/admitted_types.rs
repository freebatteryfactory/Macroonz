//! Renders reusable caller-validated nominal types as ordinary source through the root facade.

use macroonz::compiler::stamp::{
    Pattern, Site, SiteRoot, Stamp, StampName, Visibility, definition, invocation,
};
use macroonz::compiler::{GeneratedTree, TextCapture};
use std::io::Write;

fn tree(source: &str) -> Result<GeneratedTree, String> {
    TextCapture::read(source)
        .map_err(|error| error.to_string())?
        .input()
        .fragment()
        .generated()
        .map_err(|error| error.to_string())
}

fn site(name: &str, home: &str, inner: GeneratedTree, validator: &str) -> Result<Site, String> {
    Site::declared(
        name,
        SiteRoot::spelled(vec!["crate".to_owned()]).map_err(|error| error.to_string())?,
        Visibility::Public,
        vec![
            tree(name)?,
            tree(home)?,
            tree("transparent")?,
            inner,
            tree("crate::Admission")?,
            tree(&format!("crate::{validator}"))?,
        ],
    )
    .map_err(|error| error.to_string())
}

fn source(name: &str, pattern: Pattern, sites: Vec<Site>) -> Result<String, String> {
    let stamp = Stamp::declared(
        StampName::declared(name).map_err(|error| error.to_string())?,
        pattern,
        sites,
    )
    .map_err(|error| error.to_string())?;
    let mut tokens = definition(&stamp).map_err(|error| error.to_string())?;
    for site in stamp.sites() {
        tokens.extend(invocation(&stamp, site).map_err(|error| error.to_string())?);
    }
    Ok(GeneratedTree::assembled(tokens)
        .map_err(|error| error.to_string())?
        .inspected())
}

fn borrowed() -> Result<String, String> {
    let parameters = [tree("'data")?, tree("T")?, tree("const N: usize")?];
    let arguments = [tree("'data")?, tree("T")?, tree("N")?];
    let pattern = macroonz::pattern::admitted_newtype(
        &parameters,
        &arguments,
        &[tree("T: core::fmt::Display")?],
    )
    .map_err(|error| error.to_string())?;
    source(
        "borrowed_type",
        pattern,
        vec![site(
            "BorrowedBatch",
            "batch_home",
            tree("&'data [T; N]")?,
            "admit_batch",
        )?],
    )
}

/// Renders definitions and their named adoption sites without installing or compiling them.
fn generated_source() -> Result<String, String> {
    let pattern =
        macroonz::pattern::admitted_newtype(&[], &[], &[]).map_err(|error| error.to_string())?;
    let mut generated = source(
        "admitted_type",
        pattern,
        vec![
            site(
                "Identifier",
                "identifier_home",
                tree("u64")?,
                "admit_identifier",
            )?,
            site("Label", "label_home", tree("String")?, "admit_label")?,
        ],
    )?;
    generated.push('\n');
    generated.push_str(&borrowed()?);
    Ok(generated)
}

fn main() -> Result<(), String> {
    std::io::stdout()
        .write_all(generated_source()?.as_bytes())
        .map_err(|error| error.to_string())
}

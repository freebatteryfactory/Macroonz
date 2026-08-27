//! Parses one explicit, versioned declaration read from standard input.

use std::path::{Component, PathBuf};

use crate::manifest::{BYTE_LIMIT_MAXIMUM, ENTRY_LIMIT_MAXIMUM};

const REQUEST_HEADER: &str = "durafx-sealer-request-v1";

#[derive(Debug)]
pub(crate) enum Command {
    Seal(SealDeclaration),
    Verify { repository: PathBuf, run: PathBuf },
}

#[derive(Debug)]
pub(crate) struct SealDeclaration {
    pub(crate) repository: PathBuf,
    pub(crate) staging: PathBuf,
    pub(crate) plane: String,
    pub(crate) source_revision: String,
    pub(crate) host_target: String,
    pub(crate) entry_limit: u64,
    pub(crate) byte_limit: u64,
    pub(crate) label: String,
}

pub(crate) fn parse(bytes: &[u8]) -> Result<Command, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|error| format!("request is not UTF-8: {error}"))?;
    let record = text
        .strip_suffix("\r\n")
        .or_else(|| text.strip_suffix('\n'))
        .ok_or_else(|| "request must have one LF or CRLF record terminator".to_owned())?;
    let mut fields = record.split('\t');
    if fields.next() != Some(REQUEST_HEADER) {
        return Err(format!("request header must be `{REQUEST_HEADER}`"));
    }
    let command = declared_text(&mut fields, "command")?;
    match command.as_str() {
        "seal" => parse_seal(&mut fields),
        "verify" => parse_verify(&mut fields),
        _ => Err("request command must be `seal` or `verify`".to_owned()),
    }
}

fn parse_seal<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Command, String> {
    let declaration = SealDeclaration {
        repository: declared_path(lines, "repository")?,
        staging: declared_path(lines, "staging")?,
        plane: declared_text(lines, "plane")?,
        source_revision: declared_text(lines, "source-revision")?,
        host_target: declared_text(lines, "host-target")?,
        entry_limit: declared_limit(lines, "entry-limit", ENTRY_LIMIT_MAXIMUM)?,
        byte_limit: declared_limit(lines, "byte-limit", BYTE_LIMIT_MAXIMUM)?,
        label: declared_text(lines, "label")?,
    };
    require_end(lines)?;
    Ok(Command::Seal(declaration))
}

fn parse_verify<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Command, String> {
    let repository = declared_path(lines, "repository")?;
    let run = declared_path(lines, "run")?;
    require_end(lines)?;
    Ok(Command::Verify { repository, run })
}

fn declared_path<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<PathBuf, String> {
    let value = declared_value(lines, name)?;
    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err(format!("request field `{name}` must be an absolute path"));
    }
    if value
        .split(['/', '\\'])
        .any(|component| component == "." || component == "..")
    {
        return Err(format!(
            "request field `{name}` cannot contain `.` or `..` components"
        ));
    }
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {}
            Component::CurDir | Component::ParentDir => {
                return Err(format!(
                    "request field `{name}` is not lexically normalized"
                ));
            }
        }
    }
    Ok(path)
}

fn declared_text<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<String, String> {
    declared_value(lines, name).map(str::to_owned)
}

fn declared_limit<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    name: &str,
    maximum: u64,
) -> Result<u64, String> {
    let value = declared_value(lines, name)?;
    let parsed = value
        .parse::<u64>()
        .map_err(|error| format!("request field `{name}` is not an unsigned integer: {error}"))?;
    if parsed == 0 || parsed > maximum {
        return Err(format!(
            "request field `{name}` must be between 1 and {maximum}"
        ));
    }
    if parsed.to_string() != value {
        return Err(format!(
            "request field `{name}` is not canonical unsigned decimal"
        ));
    }
    Ok(parsed)
}

fn declared_value<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<&'a str, String> {
    let line = lines
        .next()
        .ok_or_else(|| format!("request lacks required field `{name}`"))?;
    let prefix = format!("{name}=");
    let value = line
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("request expected field `{name}`"))?;
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(format!(
            "request field `{name}` must be nonempty text without control characters"
        ));
    }
    Ok(value)
}

fn require_end<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<(), String> {
    if lines.next().is_some() {
        return Err("request has additional fields".to_owned());
    }
    Ok(())
}

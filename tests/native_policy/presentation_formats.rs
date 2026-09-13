//! Independent reading of emitted tables against every node of the JSON document.

use macroonz::presentation::Presentation;
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) fn parsed(shown: &Presentation) -> Result<Value, String> {
    agree(shown)?;
    serde_json::from_str(&shown.json()).map_err(|error| error.to_string())
}

pub(super) fn field<'value>(value: &'value Value, path: &str) -> Result<&'value Value, String> {
    value
        .pointer(path)
        .ok_or_else(|| format!("missing presentation field {path}"))
}

pub(super) fn decoded_hex(value: &Value) -> Result<Vec<u8>, String> {
    let text = value.as_str().ok_or("hexadecimal text missing")?;
    let (pairs, remainder) = text.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|pair| {
            let pair = std::str::from_utf8(pair).map_err(|error| error.to_string())?;
            u8::from_str_radix(pair, 16).map_err(|error| error.to_string())
        })
        .collect()
}

fn entities(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

fn markdown_cell(text: &str) -> String {
    let mut decoded = String::new();
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character == '\\' {
            if let Some(literal) = characters.next() {
                decoded.push(literal);
            }
        } else {
            decoded.push(character);
        }
    }
    entities(&decoded)
}

pub(super) fn agree(shown: &Presentation) -> Result<(), String> {
    read_formats(&shown.json(), &shown.markdown(), &shown.html()).map(|_source| ())
}

pub(super) fn read_formats(json: &str, markdown: &str, html: &str) -> Result<Value, String> {
    let source: Value = serde_json::from_str(json).map_err(|error| error.to_string())?;
    let markdown_rows = markdown
        .lines()
        .skip(2)
        .map(|line| {
            let cells = line
                .strip_prefix("| ")
                .and_then(|line| line.strip_suffix(" |"))
                .ok_or("malformed Markdown row")?;
            let (path, value) = cells.split_once(" | ").ok_or("missing Markdown column")?;
            Ok((markdown_cell(path), markdown_cell(value)))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let html_rows = html
        .split("<tr><td>")
        .skip(1)
        .map(|row| {
            let (path, rest) = row.split_once("</td><td>").ok_or("missing HTML column")?;
            let (value, _after) = rest.split_once("</td></tr>").ok_or("unclosed HTML row")?;
            Ok((entities(path), entities(value)))
        })
        .collect::<Result<Vec<_>, String>>()?;
    assert_eq!(markdown_rows, html_rows);
    let fields: BTreeMap<_, _> = html_rows.iter().cloned().collect();
    assert_eq!(fields.len(), html_rows.len(), "duplicate displayed path");
    let mut pending = vec![(String::new(), &source)];
    let mut visited = 0usize;
    while let Some((path, value)) = pending.pop() {
        visited = visited.saturating_add(1);
        let rendered = fields
            .get(&path)
            .ok_or_else(|| format!("omitted presentation node {path}"))?;
        match value {
            Value::Object(members) => {
                assert_eq!(rendered, &format!("object({})", members.len()));
                pending.extend(
                    members
                        .iter()
                        .map(|(key, child)| (format!("{path}/{key}"), child)),
                );
            }
            Value::Array(members) => {
                assert_eq!(rendered, &format!("array({})", members.len()));
                pending.extend(
                    members
                        .iter()
                        .enumerate()
                        .map(|(index, child)| (format!("{path}/{index}"), child)),
                );
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                let decoded: Value =
                    serde_json::from_str(rendered).map_err(|error| error.to_string())?;
                assert_eq!(&decoded, value, "changed presentation field {path}");
            }
        }
    }
    assert_eq!(fields.len(), visited, "invented presentation node");
    Ok(source)
}

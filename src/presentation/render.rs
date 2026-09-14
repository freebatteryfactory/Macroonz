//! Complete traversal and escaping at the two markup boundaries.

use super::types::Markup;
use serde_json::Value;
use std::fmt::Write;

fn rows(path: &str, value: &Value, into: &mut Vec<(String, String)>) {
    match value {
        Value::Array(values) => {
            into.push((path.to_owned(), format!("array({})", values.len())));
            for (index, member) in values.iter().enumerate() {
                rows(&format!("{path}/{index}"), member, into);
            }
        }
        Value::Object(values) => {
            into.push((path.to_owned(), format!("object({})", values.len())));
            for (key, member) in values {
                rows(&format!("{path}/{key}"), member, into);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            into.push((path.to_owned(), value.to_string()));
        }
    }
}

fn escaped(source: &str, markup: Markup) -> String {
    let mut result = String::new();
    for character in source.chars() {
        match character {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            value if value.is_control() => {
                let _written = write!(result, "&#{};", u32::from(value));
            }
            value if matches!(markup, Markup::Markdown) && value.is_ascii_punctuation() => {
                result.push('\\');
                result.push(value);
            }
            value => result.push(value),
        }
    }
    result
}

pub(super) fn markdown(value: &Value) -> String {
    let mut fields = Vec::new();
    rows("", value, &mut fields);
    let mut result = String::from("| Path | Value |\n| --- | --- |\n");
    for (path, shown) in fields {
        let _written = writeln!(
            result,
            "| {} | {} |",
            escaped(&path, Markup::Markdown),
            escaped(&shown, Markup::Markdown)
        );
    }
    result
}

pub(super) fn html(value: &Value) -> String {
    let mut fields = Vec::new();
    rows("", value, &mut fields);
    let mut result =
        String::from("<table><thead><tr><th>Path</th><th>Value</th></tr></thead><tbody>");
    for (path, shown) in fields {
        let _written = write!(
            result,
            "<tr><td>{}</td><td>{}</td></tr>",
            escaped(&path, Markup::Html),
            escaped(&shown, Markup::Html)
        );
    }
    result.push_str("</tbody></table>");
    result
}

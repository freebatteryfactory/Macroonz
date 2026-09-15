//! Schema-local projection of Cargo diagnostics through the declared JSON reader.

use super::{Diagnostic, Span};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

pub(super) fn cargo_diagnostic(line: &str) -> Result<Option<Diagnostic>, String> {
    // Validate the complete envelope before selecting its payload schema, independent of order.
    let envelope: Envelope<Discard> =
        serde_json::from_str(line).map_err(|error| error.to_string())?;
    if envelope.reason.as_deref() != Some("compiler-message") {
        return Ok(None);
    }
    let diagnostic: Envelope<Diagnostic> =
        serde_json::from_str(line).map_err(|error| error.to_string())?;
    diagnostic
        .message
        .map(Some)
        .ok_or_else(|| "a compiler-message row carried no diagnostic".to_owned())
}

struct Envelope<T> {
    reason: Option<String>,
    message: Option<T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Envelope<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Self {
            reason: None,
            message: None,
        })
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for Envelope<T> {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Cargo message object")
    }

    fn visit_map<M: MapAccess<'de>>(mut self, mut map: M) -> Result<Self, M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "reason" => field_once(&mut map, &mut self.reason, "reason")?,
                "message" => field_once(&mut map, &mut self.message, "message")?,
                _ => {
                    map.next_value::<Discard>()?;
                }
            }
        }
        Ok(self)
    }
}

fn field_once<'de, M: MapAccess<'de>, T: Deserialize<'de>>(
    map: &mut M,
    slot: &mut Option<T>,
    name: &'static str,
) -> Result<(), M::Error> {
    if slot.is_some() {
        return Err(M::Error::duplicate_field(name));
    }
    *slot = Some(map.next_value()?);
    Ok(())
}

impl<'de> Deserialize<'de> for Diagnostic {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Self::default())
    }
}

impl<'de> Visitor<'de> for Diagnostic {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a rustc diagnostic object")
    }

    fn visit_map<M: MapAccess<'de>>(mut self, mut map: M) -> Result<Self, M::Error> {
        // The outer slot records occurrence even when the code's value is null.
        let mut code: Option<Code> = None;
        let mut spans = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "code" => field_once(&mut map, &mut code, "code")?,
                "level" => field_once(&mut map, &mut self.level, "level")?,
                "spans" => field_once(&mut map, &mut spans, "spans")?,
                _ => {
                    map.next_value::<Discard>()?;
                }
            }
        }
        self.code = code.and_then(|code| code.0);
        self.spans = spans.unwrap_or_default();
        Ok(self)
    }
}

struct Code(Option<String>);

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(Self(None))
    }
}

impl<'de> Visitor<'de> for Code {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a diagnostic code object or null")
    }

    fn visit_none<E: Error>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(self)
    }

    fn visit_map<M: MapAccess<'de>>(mut self, mut map: M) -> Result<Self, M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            if key == "code" {
                field_once(&mut map, &mut self.0, "code")?;
            } else {
                map.next_value::<Discard>()?;
            }
        }
        Ok(self)
    }
}

impl<'de> Deserialize<'de> for Span {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Self::default())
    }
}

impl<'de> Visitor<'de> for Span {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a rustc source span object")
    }

    fn visit_map<M: MapAccess<'de>>(mut self, mut map: M) -> Result<Self, M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "file_name" => field_once(&mut map, &mut self.file_name, "file_name")?,
                "line_start" => field_once(&mut map, &mut self.line_start, "line_start")?,
                "line_end" => field_once(&mut map, &mut self.line_end, "line_end")?,
                "column_start" => field_once(&mut map, &mut self.column_start, "column_start")?,
                "column_end" => field_once(&mut map, &mut self.column_end, "column_end")?,
                "is_primary" => field_once(&mut map, &mut self.is_primary, "is_primary")?,
                _ => {
                    map.next_value::<Discard>()?;
                }
            }
        }
        Ok(self)
    }
}

// IgnoredAny bypasses the pinned reader's scalar-string and ordinary depth checks.
// Consume unknown values through normal decoding without retaining their meaning or a tree.
struct Discard;

impl<'de> Deserialize<'de> for Discard {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Self)
    }
}

impl<'de> Visitor<'de> for Discard {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value within the reader's numeric and nesting limits")
    }

    fn visit_unit<E: Error>(self) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_bool<E: Error>(self, _: bool) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_i64<E: Error>(self, _: i64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_u64<E: Error>(self, _: u64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_f64<E: Error>(self, _: f64) -> Result<Self, E> {
        Ok(self)
    }
    fn visit_str<E: Error>(self, _: &str) -> Result<Self, E> {
        Ok(self)
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut sequence: S) -> Result<Self, S::Error> {
        while sequence.next_element::<Self>()?.is_some() {}
        Ok(self)
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self, M::Error> {
        while map.next_key::<String>()?.is_some() {
            map.next_value::<Self>()?;
        }
        Ok(self)
    }
}

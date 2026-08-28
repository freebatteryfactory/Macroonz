//! Structural reading of the rustc JSON facts emitted for one hostile source.

use super::render::PrimarySpan;

#[derive(Default)]
struct Diagnostic {
    code: Option<String>,
    level: Option<String>,
    spans: Vec<Span>,
}

#[derive(Default)]
struct Span {
    file_name: Option<String>,
    line_start: Option<u64>,
    line_end: Option<u64>,
    column_start: Option<u64>,
    column_end: Option<u64>,
    is_primary: Option<bool>,
}

pub(super) fn require_one_mismatch(output: &[u8], expected: &PrimarySpan) -> Result<(), String> {
    let text = std::str::from_utf8(output)
        .map_err(|error| format!("Cargo JSON was not UTF-8: {error}"))?;
    let mut relevant = Vec::new();

    for line in text.lines() {
        if let Some(diagnostic) = cargo_diagnostic(line)?
            && diagnostic.level.as_deref() == Some("error")
            && diagnostic.code.as_deref() == Some("E0308")
        {
            relevant.push(diagnostic);
        }
    }

    if relevant.len() != 1 {
        return Err(format!(
            "expected exactly one relevant E0308 diagnostic, observed {}",
            relevant.len()
        ));
    }

    let diagnostic = relevant
        .first()
        .ok_or_else(|| "the counted E0308 diagnostic disappeared".to_owned())?;
    let primary: Vec<_> = diagnostic
        .spans
        .iter()
        .filter(|span| span.is_primary == Some(true))
        .collect();
    if primary.len() != 1 {
        return Err(format!(
            "expected exactly one primary E0308 span, observed {}",
            primary.len()
        ));
    }
    let observed = primary
        .first()
        .ok_or_else(|| "the counted primary span disappeared".to_owned())?;
    if !span_matches(observed, expected) {
        return Err(format!(
            "E0308 primary span did not resolve to {}:{}:{}-{}",
            expected.file_name, expected.line, expected.column_start, expected.column_end
        ));
    }
    Ok(())
}

fn span_matches(observed: &Span, expected: &PrimarySpan) -> bool {
    let file_matches = observed.file_name.as_deref().is_some_and(|file| {
        file.replace('\\', "/")
            .ends_with(&expected.file_name.replace('\\', "/"))
    });
    file_matches
        && observed.line_start == Some(expected.line)
        && observed.line_end == Some(expected.line)
        && observed.column_start == Some(expected.column_start)
        && observed.column_end == Some(expected.column_end)
}

fn cargo_diagnostic(line: &str) -> Result<Option<Diagnostic>, String> {
    let mut cursor = Cursor::new(line);
    cursor.expect_byte(b'{')?;
    let mut reason = None;
    let mut diagnostic = None;

    while cursor.object_has_field() {
        let key = cursor.string()?;
        cursor.expect_byte(b':')?;
        match key.as_str() {
            "reason" => reason = Some(cursor.string()?),
            "message" => diagnostic = Some(parse_diagnostic(&mut cursor)?),
            _ => cursor.skip_value()?,
        }
        cursor.finish_field()?;
    }

    if reason.as_deref() == Some("compiler-message") {
        diagnostic
            .map(Some)
            .ok_or_else(|| "a compiler-message row carried no diagnostic".to_owned())
    } else {
        Ok(None)
    }
}

fn parse_diagnostic(cursor: &mut Cursor<'_>) -> Result<Diagnostic, String> {
    cursor.expect_byte(b'{')?;
    let mut diagnostic = Diagnostic::default();
    while cursor.object_has_field() {
        let key = cursor.string()?;
        cursor.expect_byte(b':')?;
        match key.as_str() {
            "code" => diagnostic.code = parse_code(cursor)?,
            "level" => diagnostic.level = Some(cursor.string()?),
            "spans" => diagnostic.spans = parse_spans(cursor)?,
            _ => cursor.skip_value()?,
        }
        cursor.finish_field()?;
    }
    Ok(diagnostic)
}

fn parse_code(cursor: &mut Cursor<'_>) -> Result<Option<String>, String> {
    if cursor.consume_literal("null") {
        return Ok(None);
    }
    cursor.expect_byte(b'{')?;
    let mut code = None;
    while cursor.object_has_field() {
        let key = cursor.string()?;
        cursor.expect_byte(b':')?;
        if key == "code" {
            code = Some(cursor.string()?);
        } else {
            cursor.skip_value()?;
        }
        cursor.finish_field()?;
    }
    Ok(code)
}

fn parse_spans(cursor: &mut Cursor<'_>) -> Result<Vec<Span>, String> {
    cursor.expect_byte(b'[')?;
    let mut spans = Vec::new();
    while cursor.array_has_value() {
        spans.push(parse_span(cursor)?);
        cursor.finish_value()?;
    }
    Ok(spans)
}

fn parse_span(cursor: &mut Cursor<'_>) -> Result<Span, String> {
    cursor.expect_byte(b'{')?;
    let mut span = Span::default();
    while cursor.object_has_field() {
        let key = cursor.string()?;
        cursor.expect_byte(b':')?;
        match key.as_str() {
            "file_name" => span.file_name = Some(cursor.string()?),
            "line_start" => span.line_start = Some(cursor.unsigned()?),
            "line_end" => span.line_end = Some(cursor.unsigned()?),
            "column_start" => span.column_start = Some(cursor.unsigned()?),
            "column_end" => span.column_end = Some(cursor.unsigned()?),
            "is_primary" => span.is_primary = Some(cursor.boolean()?),
            _ => cursor.skip_value()?,
        }
        cursor.finish_field()?;
    }
    Ok(span)
}

struct Cursor<'source> {
    source: &'source [u8],
    offset: usize,
}

impl<'source> Cursor<'source> {
    fn new(source: &'source str) -> Self {
        Self {
            source: source.as_bytes(),
            offset: 0,
        }
    }

    fn object_has_field(&mut self) -> bool {
        self.skip_whitespace();
        !self.consume_byte(b'}')
    }

    fn array_has_value(&mut self) -> bool {
        self.skip_whitespace();
        !self.consume_byte(b']')
    }

    fn finish_field(&mut self) -> Result<(), String> {
        self.finish_delimited(b'}')
    }

    fn finish_value(&mut self) -> Result<(), String> {
        self.finish_delimited(b']')
    }

    fn finish_delimited(&mut self, closing: u8) -> Result<(), String> {
        self.skip_whitespace();
        if self.peek() == Some(closing) {
            Ok(())
        } else {
            self.expect_byte(b',')
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        self.expect_byte(b'"')?;
        let mut value = String::new();
        loop {
            let byte = self
                .next()
                .ok_or_else(|| "unterminated JSON string".to_owned())?;
            match byte {
                b'"' => return Ok(value),
                b'\\' => self.push_escape(&mut value)?,
                ascii if ascii.is_ascii() => value.push(char::from(ascii)),
                _ => self.push_utf8(byte, &mut value)?,
            }
        }
    }

    fn push_escape(&mut self, value: &mut String) -> Result<(), String> {
        let escaped = self
            .next()
            .ok_or_else(|| "unterminated JSON escape".to_owned())?;
        match escaped {
            b'"' => value.push('"'),
            b'\\' => value.push('\\'),
            b'/' => value.push('/'),
            b'b' => value.push('\u{0008}'),
            b'f' => value.push('\u{000c}'),
            b'n' => value.push('\n'),
            b'r' => value.push('\r'),
            b't' => value.push('\t'),
            b'u' => {
                let scalar = self.hex_scalar()?;
                let character = char::from_u32(scalar)
                    .ok_or_else(|| format!("invalid JSON Unicode scalar {scalar:#x}"))?;
                value.push(character);
            }
            other => return Err(format!("unsupported JSON escape {other:#x}")),
        }
        Ok(())
    }

    fn hex_scalar(&mut self) -> Result<u32, String> {
        let mut scalar = 0_u32;
        for _ in 0_u8..4_u8 {
            let digit = self
                .next()
                .and_then(|byte| char::from(byte).to_digit(16))
                .ok_or_else(|| "invalid JSON Unicode escape".to_owned())?;
            scalar = scalar
                .checked_mul(16)
                .and_then(|value| value.checked_add(digit))
                .ok_or_else(|| "JSON Unicode escape overflowed".to_owned())?;
        }
        Ok(scalar)
    }

    fn push_utf8(&mut self, first: u8, value: &mut String) -> Result<(), String> {
        let width =
            utf8_width(first).ok_or_else(|| format!("invalid UTF-8 lead byte {first:#x}"))?;
        let start = self.offset.saturating_sub(1);
        let end = start
            .checked_add(width)
            .ok_or_else(|| "UTF-8 span overflowed".to_owned())?;
        let bytes = self
            .source
            .get(start..end)
            .ok_or_else(|| "truncated UTF-8 in JSON string".to_owned())?;
        let text = std::str::from_utf8(bytes)
            .map_err(|error| format!("invalid UTF-8 in JSON string: {error}"))?;
        value.push_str(text);
        self.offset = end;
        Ok(())
    }

    fn unsigned(&mut self) -> Result<u64, String> {
        self.skip_whitespace();
        let start = self.offset;
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.offset = self.offset.saturating_add(1);
        }
        let bytes = self
            .source
            .get(start..self.offset)
            .ok_or_else(|| "JSON number span was invalid".to_owned())?;
        let text = std::str::from_utf8(bytes)
            .map_err(|error| format!("JSON number was not UTF-8: {error}"))?;
        text.parse::<u64>()
            .map_err(|error| format!("JSON number was not unsigned: {error}"))
    }

    fn boolean(&mut self) -> Result<bool, String> {
        self.skip_whitespace();
        if self.consume_literal("true") {
            Ok(true)
        } else if self.consume_literal("false") {
            Ok(false)
        } else {
            Err("expected JSON boolean".to_owned())
        }
    }

    fn skip_value(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'"') => self.string().map(|_| ()),
            Some(b'{') => self.skip_object(),
            Some(b'[') => self.skip_array(),
            Some(b't') if self.consume_literal("true") => Ok(()),
            Some(b'f') if self.consume_literal("false") => Ok(()),
            Some(b'n') if self.consume_literal("null") => Ok(()),
            Some(b'-' | b'0'..=b'9') => self.skip_number(),
            other => Err(format!("unsupported JSON value at {other:?}")),
        }
    }

    fn skip_object(&mut self) -> Result<(), String> {
        self.expect_byte(b'{')?;
        while self.object_has_field() {
            self.string()?;
            self.expect_byte(b':')?;
            self.skip_value()?;
            self.finish_field()?;
        }
        Ok(())
    }

    fn skip_array(&mut self) -> Result<(), String> {
        self.expect_byte(b'[')?;
        while self.array_has_value() {
            self.skip_value()?;
            self.finish_value()?;
        }
        Ok(())
    }

    fn skip_number(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        let start = self.offset;
        while self.peek().is_some_and(|byte| {
            byte.is_ascii_digit() || matches!(byte, b'-' | b'+' | b'.' | b'e' | b'E')
        }) {
            self.offset = self.offset.saturating_add(1);
        }
        if self.offset == start {
            Err("expected JSON number".to_owned())
        } else {
            Ok(())
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), String> {
        self.skip_whitespace();
        match self.next() {
            Some(observed) if observed == expected => Ok(()),
            observed => Err(format!(
                "expected JSON byte {expected:#x}, observed {observed:?} at {}",
                self.offset
            )),
        }
    }

    fn consume_literal(&mut self, literal: &str) -> bool {
        self.skip_whitespace();
        let end = self.offset.saturating_add(literal.len());
        if self.source.get(self.offset..end) == Some(literal.as_bytes()) {
            self.offset = end;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.offset = self.offset.saturating_add(1);
        }
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.offset = self.offset.saturating_add(1);
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.offset).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.offset = self.offset.saturating_add(1);
        Some(byte)
    }
}

fn utf8_width(first: u8) -> Option<usize> {
    match first {
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

//! Canonical reading of stable rustc coverage exports.

use super::{CoverageObservation, CoveragePoint, CoverageReadRefusal};
use std::collections::BTreeSet;

/// Read one `llvm-cov export -format=lcov` document into canonical covered points.
///
/// Zero-count lines and untaken branches do not enter the observation.
///
/// # Errors
///
/// Refuses non-UTF-8 material, malformed recognized rows, empty source records, and points without a preceding source record.
pub fn read_lcov(bytes: &[u8]) -> Result<CoverageObservation, CoverageReadRefusal> {
    let text = std::str::from_utf8(bytes).map_err(|_error| CoverageReadRefusal::NonUtf8)?;
    let mut source: Option<String> = None;
    let mut points = BTreeSet::new();
    for (index, row) in text.lines().enumerate() {
        let record = index.saturating_add(1);
        if let Some(path) = row.strip_prefix("SF:") {
            if path.is_empty() {
                return Err(CoverageReadRefusal::EmptySource { record });
            }
            source = Some(path.to_owned());
            continue;
        }
        if row == "end_of_record" {
            source = None;
            continue;
        }
        if let Some(fields) = row.strip_prefix("DA:") {
            read_line(fields, source.as_deref(), record, &mut points)?;
            continue;
        }
        if let Some(fields) = row.strip_prefix("BRDA:") {
            read_branch(fields, source.as_deref(), record, &mut points)?;
        }
    }
    Ok(CoverageObservation::established(points))
}

fn read_line(
    fields: &str,
    source: Option<&str>,
    record: usize,
    points: &mut BTreeSet<CoveragePoint>,
) -> Result<(), CoverageReadRefusal> {
    let mut columns = fields.split(',');
    let line = parse(columns.next()).ok_or(CoverageReadRefusal::MalformedLine { record })?;
    let count = parse(columns.next()).ok_or(CoverageReadRefusal::MalformedLine { record })?;
    if count == 0 {
        return Ok(());
    }
    let path = source.ok_or(CoverageReadRefusal::MissingSource { record })?;
    points.insert(CoveragePoint::Line {
        source: path.to_owned(),
        line,
    });
    Ok(())
}

fn read_branch(
    fields: &str,
    source: Option<&str>,
    record: usize,
    points: &mut BTreeSet<CoveragePoint>,
) -> Result<(), CoverageReadRefusal> {
    let mut columns = fields.split(',');
    let line = parse(columns.next()).ok_or(CoverageReadRefusal::MalformedBranch { record })?;
    let block = parse(columns.next()).ok_or(CoverageReadRefusal::MalformedBranch { record })?;
    let branch = parse(columns.next()).ok_or(CoverageReadRefusal::MalformedBranch { record })?;
    let taken = columns
        .next()
        .ok_or(CoverageReadRefusal::MalformedBranch { record })?;
    if taken == "-" {
        return Ok(());
    }
    let count = taken
        .parse::<u64>()
        .map_err(|_error| CoverageReadRefusal::MalformedBranch { record })?;
    if count == 0 {
        return Ok(());
    }
    let path = source.ok_or(CoverageReadRefusal::MissingSource { record })?;
    points.insert(CoveragePoint::Branch {
        source: path.to_owned(),
        line,
        block,
        branch,
    });
    Ok(())
}

fn parse(value: Option<&str>) -> Option<u64> {
    value?.parse::<u64>().ok()
}

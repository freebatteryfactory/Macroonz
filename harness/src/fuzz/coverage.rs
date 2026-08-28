//! Canonical reading of stable rustc coverage exports.

use super::{
    CoverageObservation, CoveragePoint, CoverageReadRefusal, CoverageSource, CoverageSourceRoot,
};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::{Component, Path};

/// Read one `llvm-cov export -format=lcov` document into canonical covered points.
///
/// Zero-count lines and untaken branches do not enter the observation.
///
/// # Errors
///
/// Refuses non-UTF-8 material, malformed recognized rows, empty source records, and points without a preceding source record.
/// A source path is also refused when it is relative, traverses a parent, falls outside the declared checkout root, or names the root without a relative source member.
pub fn read_lcov(
    root: &CoverageSourceRoot,
    bytes: &[u8],
) -> Result<CoverageObservation, CoverageReadRefusal> {
    let text = std::str::from_utf8(bytes).map_err(|_error| CoverageReadRefusal::NonUtf8)?;
    let mut source: Option<CoverageSource> = None;
    let mut points = BTreeSet::new();
    for (index, row) in text.lines().enumerate() {
        let record = index.saturating_add(1);
        if let Some(path) = row.strip_prefix("SF:") {
            if path.is_empty() {
                return Err(CoverageReadRefusal::EmptySource { record });
            }
            source = Some(read_source(root, path, record)?);
            continue;
        }
        if row == "end_of_record" {
            source = None;
            continue;
        }
        if let Some(fields) = row.strip_prefix("DA:") {
            read_line(fields, source.as_ref(), record, &mut points)?;
            continue;
        }
        if let Some(fields) = row.strip_prefix("BRDA:") {
            read_branch(fields, source.as_ref(), record, &mut points)?;
        }
    }
    Ok(CoverageObservation::established(points))
}

fn read_line(
    fields: &str,
    source: Option<&CoverageSource>,
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
        source: path.clone(),
        line,
    });
    Ok(())
}

fn read_branch(
    fields: &str,
    source: Option<&CoverageSource>,
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
        source: path.clone(),
        line,
        block,
        branch,
    });
    Ok(())
}

fn read_source(
    root: &CoverageSourceRoot,
    spelling: &str,
    record: usize,
) -> Result<CoverageSource, CoverageReadRefusal> {
    let spelling = comparable_path(spelling);
    let Some(checkout) = root.checkout().to_str() else {
        return Err(CoverageReadRefusal::NonUtf8);
    };
    let checkout = comparable_path(checkout);
    let path = Path::new(spelling.as_ref());
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(CoverageReadRefusal::SourceTraversal { record });
    }
    if !path.is_absolute() {
        return Err(CoverageReadRefusal::RelativeSource { record });
    }
    let relative = path
        .strip_prefix(Path::new(checkout.as_ref()))
        .map_err(|_error| CoverageReadRefusal::SourceOutsideRoot { record })?;
    let mut canonical = String::new();
    for component in relative.components() {
        let Component::Normal(segment) = component else {
            return Err(CoverageReadRefusal::SourceTraversal { record });
        };
        let Some(segment) = segment.to_str() else {
            return Err(CoverageReadRefusal::NonUtf8);
        };
        if !canonical.is_empty() {
            canonical.push('/');
        }
        canonical.push_str(segment);
    }
    if canonical.is_empty() {
        return Err(CoverageReadRefusal::EmptyRelativeSource { record });
    }
    Ok(CoverageSource::established(root.logical(), canonical))
}

fn comparable_path(spelling: &str) -> Cow<'_, str> {
    #[cfg(windows)]
    {
        if let Some(unc) = spelling.strip_prefix(r"\\?\UNC\") {
            return Cow::Owned(format!(r"\\{unc}"));
        }
        if let Some(ordinary) = spelling.strip_prefix(r"\\?\") {
            return Cow::Borrowed(ordinary);
        }
    }
    Cow::Borrowed(spelling)
}

fn parse(value: Option<&str>) -> Option<u64> {
    value?.parse::<u64>().ok()
}

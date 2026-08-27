//! Owns canonical placement receipts and exhaustive payload manifests.

use std::ffi::OsStr;

pub(crate) const MANIFEST_NAME: &str = "DURAFX-MANIFEST.blake3";
pub(crate) const PLACEMENT_RECEIPT_NAME: &str = "DURAFX-RECEIPT.txt";
pub(crate) const SEMANTIC_RECEIPT_NAME: &str = "receipt.md";
pub(crate) const COMPLETION_NAME: &str = "DURAFX-COMPLETE";
pub(crate) const MANIFEST_BYTE_LIMIT: u64 = 67_108_864;
pub(crate) const PLACEMENT_RECEIPT_BYTE_LIMIT: u64 = 4_096;
pub(crate) const ENTRY_LIMIT_MAXIMUM: u64 = 100_000;
pub(crate) const BYTE_LIMIT_MAXIMUM: u64 = 1_099_511_627_776;
const MANIFEST_HEADER: &str = "durafx-manifest-v1";
const PLACEMENT_RECEIPT_HEADER: &str = "durafx-placement-receipt-v1";
const RUN_IDENTITY_HEADER: &str = "durafx-run-identity-v1";
const NORMALIZED_PATH_BYTE_LIMIT: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PayloadRecord {
    pub(crate) path: String,
    pub(crate) bytes: u64,
    pub(crate) hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlacementReceipt {
    pub(crate) plane: String,
    pub(crate) source_revision: String,
    pub(crate) host_target: String,
    pub(crate) entry_limit: u64,
    pub(crate) byte_limit: u64,
    pub(crate) label: String,
}

pub(crate) fn complete_records(
    staged: &[PayloadRecord],
    placement_receipt: &[u8],
) -> Result<Vec<PayloadRecord>, String> {
    let mut records = staged.to_vec();
    records.push(PayloadRecord {
        path: PLACEMENT_RECEIPT_NAME.to_owned(),
        bytes: u64::try_from(placement_receipt.len())
            .map_err(|error| format!("placement receipt length cannot be represented: {error}"))?,
        hash: blake3::hash(placement_receipt).to_hex().to_string(),
    });
    records.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(records)
}

pub(crate) fn render_placement_receipt(receipt: &PlacementReceipt) -> Vec<u8> {
    format!(
        "{PLACEMENT_RECEIPT_HEADER}\nplane={}\nsource-revision={}\nhost-target={}\nentry-limit={}\nbyte-limit={}\nlabel={}\nsemantic-receipt={}\n",
        receipt.plane,
        receipt.source_revision,
        receipt.host_target,
        receipt.entry_limit,
        receipt.byte_limit,
        receipt.label,
        SEMANTIC_RECEIPT_NAME
    )
    .into_bytes()
}

pub(crate) fn render_manifest(records: &[PayloadRecord]) -> Result<Vec<u8>, String> {
    let mut rendered = format!("{MANIFEST_HEADER}\n");
    for record in records {
        let row = format!("file\t{}\t{}\t{}\n", record.bytes, record.hash, record.path);
        let next_length = rendered
            .len()
            .checked_add(row.len())
            .ok_or_else(|| "manifest length overflowed".to_owned())?;
        let next_length = u64::try_from(next_length)
            .map_err(|error| format!("manifest length cannot be represented: {error}"))?;
        if next_length > MANIFEST_BYTE_LIMIT {
            return Err(format!(
                "manifest exceeds its {MANIFEST_BYTE_LIMIT}-byte limit"
            ));
        }
        rendered.push_str(&row);
    }
    Ok(rendered.into_bytes())
}

pub(crate) fn run_digest(
    records: &[PayloadRecord],
    placement: &PlacementReceipt,
) -> Result<String, String> {
    let identity = format!(
        "{RUN_IDENTITY_HEADER}\nplane={}\nsource-revision={}\nhost-target={}\nentry-limit={}\nbyte-limit={}\nsemantic-receipt={}\n",
        placement.plane,
        placement.source_revision,
        placement.host_target,
        placement.entry_limit,
        placement.byte_limit,
        SEMANTIC_RECEIPT_NAME
    );
    let evidence = records
        .iter()
        .filter(|record| record.path != PLACEMENT_RECEIPT_NAME)
        .cloned()
        .collect::<Vec<_>>();
    let evidence_manifest = render_manifest(&evidence)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(identity.as_bytes());
    hasher.update(&evidence_manifest);
    Ok(hasher.finalize().to_hex().to_string())
}

pub(crate) fn parse_manifest(
    bytes: &[u8],
    record_limit: u64,
) -> Result<Vec<PayloadRecord>, String> {
    if !bytes.ends_with(b"\n") {
        return Err("manifest is not newline-terminated".to_owned());
    }
    let text =
        std::str::from_utf8(bytes).map_err(|error| format!("manifest is not UTF-8: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(MANIFEST_HEADER) {
        return Err("manifest header is not canonical".to_owned());
    }
    let mut records = Vec::new();
    let mut previous_path = None::<String>;
    for line in lines {
        let next_count = u64::try_from(records.len())
            .map_err(|error| format!("manifest record count cannot be represented: {error}"))?
            .checked_add(1)
            .ok_or_else(|| "manifest record count overflowed".to_owned())?;
        if next_count > record_limit {
            return Err(format!(
                "manifest exceeds its declared {record_limit}-record limit"
            ));
        }
        let record = parse_record(line)?;
        if previous_path
            .as_deref()
            .is_some_and(|previous| previous >= record.path.as_str())
        {
            return Err("manifest paths are not in strict canonical order".to_owned());
        }
        previous_path = Some(record.path.clone());
        records.push(record);
    }
    require_receipt_records(&records)?;
    if render_manifest(&records)? != bytes {
        return Err("manifest encoding is not canonical".to_owned());
    }
    Ok(records)
}

pub(crate) fn validate_manifest_budget(
    records: &[PayloadRecord],
    placement: &PlacementReceipt,
) -> Result<(), String> {
    let evidence = records
        .iter()
        .filter(|record| record.path != PLACEMENT_RECEIPT_NAME)
        .collect::<Vec<_>>();
    let evidence_count = u64::try_from(evidence.len())
        .map_err(|error| format!("manifest evidence count cannot be represented: {error}"))?;
    if evidence_count > placement.entry_limit {
        return Err(format!(
            "manifest declares {evidence_count} evidence files beyond its {}-entry limit",
            placement.entry_limit
        ));
    }
    let mut evidence_bytes = 0_u64;
    for record in evidence {
        evidence_bytes = evidence_bytes
            .checked_add(record.bytes)
            .ok_or_else(|| "manifest evidence byte total overflowed".to_owned())?;
        if evidence_bytes > placement.byte_limit {
            return Err(format!(
                "manifest declares evidence bytes beyond its {}-byte limit",
                placement.byte_limit
            ));
        }
    }
    Ok(())
}

fn parse_record(line: &str) -> Result<PayloadRecord, String> {
    let mut fields = line.splitn(4, '\t');
    if fields.next() != Some("file") {
        return Err(format!("manifest row is not a file record: {line}"));
    }
    let byte_count = fields
        .next()
        .ok_or_else(|| format!("manifest row lacks byte length: {line}"))?
        .parse::<u64>()
        .map_err(|error| format!("manifest byte length is invalid: {error}"))?;
    let hash = fields
        .next()
        .ok_or_else(|| format!("manifest row lacks hash: {line}"))?;
    validate_hash(hash)?;
    let path = fields
        .next()
        .ok_or_else(|| format!("manifest row lacks path: {line}"))?;
    validate_normalized_path(path)?;
    Ok(PayloadRecord {
        path: path.to_owned(),
        bytes: byte_count,
        hash: hash.to_owned(),
    })
}

fn require_receipt_records(records: &[PayloadRecord]) -> Result<(), String> {
    let has_placement = records
        .iter()
        .any(|record| record.path == PLACEMENT_RECEIPT_NAME);
    if !has_placement {
        return Err("manifest does not include the placement receipt".to_owned());
    }
    let semantic_receipt = records
        .iter()
        .find(|record| record.path == SEMANTIC_RECEIPT_NAME)
        .ok_or_else(|| {
            "manifest does not include the caller-authored semantic receipt".to_owned()
        })?;
    if semantic_receipt.bytes == 0 {
        return Err("manifest declares an empty caller-authored semantic receipt".to_owned());
    }
    Ok(())
}

fn validate_hash(hash: &str) -> Result<(), String> {
    let lowercase_hex = hash
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if hash.len() != 64 || !lowercase_hex {
        return Err(format!("manifest hash is not lowercase BLAKE3 hex: {hash}"));
    }
    Ok(())
}

pub(crate) fn validate_normalized_path(path: &str) -> Result<(), String> {
    if path.len() > NORMALIZED_PATH_BYTE_LIMIT {
        return Err(format!(
            "manifest path exceeds its {NORMALIZED_PATH_BYTE_LIMIT}-byte limit"
        ));
    }
    if path == MANIFEST_NAME
        || path == COMPLETION_NAME
        || path.starts_with('/')
        || path.ends_with('/')
    {
        return Err(format!("manifest path is reserved or not relative: {path}"));
    }
    for component in path.split('/') {
        normalize_component(OsStr::new(component))?;
    }
    Ok(())
}

pub(crate) fn normalize_component(component: &OsStr) -> Result<String, String> {
    let text = component
        .to_str()
        .ok_or_else(|| "payload paths must be UTF-8".to_owned())?;
    if text.is_empty()
        || text == "."
        || text == ".."
        || text.contains('/')
        || text.contains('\\')
        || text.chars().any(char::is_control)
    {
        return Err(format!("payload component cannot be normalized: {text:?}"));
    }
    Ok(text.to_owned())
}

pub(crate) fn parse_placement_receipt(bytes: &[u8]) -> Result<PlacementReceipt, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("placement receipt is not UTF-8: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(PLACEMENT_RECEIPT_HEADER) {
        return Err("placement receipt header is not canonical".to_owned());
    }
    let receipt = PlacementReceipt {
        plane: placement_value(lines.next(), "plane")?,
        source_revision: placement_value(lines.next(), "source-revision")?,
        host_target: placement_value(lines.next(), "host-target")?,
        entry_limit: placement_limit(lines.next(), "entry-limit", ENTRY_LIMIT_MAXIMUM)?,
        byte_limit: placement_limit(lines.next(), "byte-limit", BYTE_LIMIT_MAXIMUM)?,
        label: placement_value(lines.next(), "label")?,
    };
    validate_seat("plane", &receipt.plane, 64)?;
    validate_seat("source revision", &receipt.source_revision, 80)?;
    validate_seat("host-target", &receipt.host_target, 80)?;
    validate_seat("label", &receipt.label, 40)?;
    let semantic_receipt = placement_value(lines.next(), "semantic-receipt")?;
    if semantic_receipt != SEMANTIC_RECEIPT_NAME {
        return Err("placement receipt names the wrong semantic receipt".to_owned());
    }
    if lines.next().is_some() || render_placement_receipt(&receipt) != bytes {
        return Err("placement receipt encoding is not canonical".to_owned());
    }
    Ok(receipt)
}

pub(crate) fn validate_seat(name: &str, value: &str, maximum: usize) -> Result<(), String> {
    if value.is_empty() || value.len() > maximum {
        return Err(format!(
            "{name} must contain between 1 and {maximum} ASCII characters"
        ));
    }
    if value == "." || value == ".." {
        return Err(format!("{name} cannot be a relative path component"));
    }
    let lawful = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'));
    if !lawful {
        return Err(format!(
            "{name} may contain only ASCII letters, digits, `.`, `_`, and `-`"
        ));
    }
    Ok(())
}

fn placement_value(line: Option<&str>, key: &str) -> Result<String, String> {
    let field = line.ok_or_else(|| format!("placement receipt lacks `{key}`"))?;
    field
        .strip_prefix(&format!("{key}="))
        .map(str::to_owned)
        .ok_or_else(|| format!("placement receipt field `{key}` is not canonical"))
}

fn placement_limit(line: Option<&str>, key: &str, maximum: u64) -> Result<u64, String> {
    let value = placement_value(line, key)?;
    let parsed = value
        .parse::<u64>()
        .map_err(|error| format!("placement receipt field `{key}` is invalid: {error}"))?;
    if parsed == 0 || parsed > maximum || parsed.to_string() != value {
        return Err(format!(
            "placement receipt field `{key}` is not canonical decimal between 1 and {maximum}"
        ));
    }
    Ok(parsed)
}

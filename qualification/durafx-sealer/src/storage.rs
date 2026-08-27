//! Owns filesystem traversal, payload transfer, and the advisory storage guard.

use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use crate::manifest::{
    self, COMPLETION_NAME, MANIFEST_NAME, PLACEMENT_RECEIPT_NAME, PayloadRecord,
    SEMANTIC_RECEIPT_NAME,
};

const COPY_BUFFER_BYTES: usize = 65_536;

#[derive(Clone, Copy, Debug)]
pub(crate) struct CensusBudget {
    entry_limit: u64,
    byte_limit: u64,
}

impl CensusBudget {
    pub(crate) const fn declared(entry_limit: u64, byte_limit: u64) -> Self {
        Self {
            entry_limit,
            byte_limit,
        }
    }

    pub(crate) fn extended(
        self,
        additional_entries: u64,
        additional_bytes: u64,
    ) -> Result<Self, String> {
        let entry_limit = self
            .entry_limit
            .checked_add(additional_entries)
            .ok_or_else(|| "bundle entry limit overflowed".to_owned())?;
        let byte_limit = self
            .byte_limit
            .checked_add(additional_bytes)
            .ok_or_else(|| "bundle byte limit overflowed".to_owned())?;
        Ok(Self {
            entry_limit,
            byte_limit,
        })
    }

    pub(crate) const fn entry_limit(self) -> u64 {
        self.entry_limit
    }
}

struct CensusProgress {
    budget: CensusBudget,
    entries: u64,
    bytes: u64,
}

impl CensusProgress {
    const fn new(budget: CensusBudget) -> Self {
        Self {
            budget,
            entries: 0,
            bytes: 0,
        }
    }

    fn admit_entry(&mut self) -> Result<(), String> {
        self.entries = self
            .entries
            .checked_add(1)
            .ok_or_else(|| "staging entry count overflowed".to_owned())?;
        if self.entries > self.budget.entry_limit {
            return Err(format!(
                "staging exceeds its declared {}-entry limit",
                self.budget.entry_limit
            ));
        }
        Ok(())
    }

    fn admit_bytes(&mut self, bytes: u64) -> Result<(), String> {
        self.bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or_else(|| "staging byte count overflowed".to_owned())?;
        if self.bytes > self.budget.byte_limit {
            return Err(format!(
                "staging exceeds its declared {}-byte limit",
                self.budget.byte_limit
            ));
        }
        Ok(())
    }

    const fn remaining_entries(&self) -> u64 {
        self.budget.entry_limit.saturating_sub(self.entries)
    }

    const fn remaining_bytes(&self) -> u64 {
        self.budget.byte_limit.saturating_sub(self.bytes)
    }
}

#[derive(Debug)]
pub(crate) enum StagingRefusal {
    MissingSemanticReceipt,
    SemanticReceiptNotRegularFile,
    EmptySemanticReceipt,
    Detail(String),
}

impl fmt::Display for StagingRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSemanticReceipt => write!(
                formatter,
                "missing-semantic-receipt: staging root must contain caller-authored `{SEMANTIC_RECEIPT_NAME}`"
            ),
            Self::SemanticReceiptNotRegularFile => write!(
                formatter,
                "invalid-semantic-receipt: root `{SEMANTIC_RECEIPT_NAME}` must be a regular file"
            ),
            Self::EmptySemanticReceipt => write!(
                formatter,
                "empty-semantic-receipt: root `{SEMANTIC_RECEIPT_NAME}` must contain caller-authored evidence material"
            ),
            Self::Detail(message) => formatter.write_str(message),
        }
    }
}

impl From<String> for StagingRefusal {
    fn from(message: String) -> Self {
        Self::Detail(message)
    }
}

#[derive(Debug)]
pub(crate) struct SourceRecord {
    pub(crate) source: PathBuf,
    pub(crate) manifest: PayloadRecord,
}

pub(crate) fn census_staging(
    staging: &Path,
    budget: CensusBudget,
) -> Result<Vec<SourceRecord>, StagingRefusal> {
    validate_staging_location(staging)?;
    let metadata = fs::symlink_metadata(staging).map_err(|error| {
        detail(format!(
            "cannot inspect staging {}: {error}",
            staging.display()
        ))
    })?;
    if metadata_has_indirection(&metadata) {
        return Err(detail(format!(
            "staging has filesystem indirection: {}",
            staging.display()
        )));
    }
    if !metadata.is_dir() {
        return Err(detail(format!(
            "staging is not a directory: {}",
            staging.display()
        )));
    }
    let mut pending = vec![(staging.to_path_buf(), String::new())];
    let mut records = Vec::new();
    let mut semantic_receipt_bytes = None::<u64>;
    let mut census = CensusProgress::new(budget);
    while let Some((directory, prefix)) = pending.pop() {
        let entries = read_entries(&directory, census.remaining_entries()).map_err(detail)?;
        if entries.is_empty() && !prefix.is_empty() {
            return Err(detail(format!(
                "staging contains unrepresented empty directory `{prefix}`"
            )));
        }
        for entry in entries {
            census.admit_entry().map_err(detail)?;
            inspect_staged_entry(
                &entry,
                &prefix,
                &mut pending,
                &mut records,
                &mut semantic_receipt_bytes,
                &mut census,
            )?;
        }
    }
    let Some(receipt_bytes) = semantic_receipt_bytes else {
        return Err(StagingRefusal::MissingSemanticReceipt);
    };
    if receipt_bytes == 0 {
        return Err(StagingRefusal::EmptySemanticReceipt);
    }
    records.sort_by(|left, right| left.manifest.path.cmp(&right.manifest.path));
    Ok(records)
}

fn inspect_staged_entry(
    entry: &fs::DirEntry,
    prefix: &str,
    pending: &mut Vec<(PathBuf, String)>,
    records: &mut Vec<SourceRecord>,
    semantic_receipt_bytes: &mut Option<u64>,
    census: &mut CensusProgress,
) -> Result<(), StagingRefusal> {
    let name = manifest::normalize_component(&entry.file_name())?;
    let relative = join_relative(prefix, &name);
    refuse_reserved_staging_path(&relative)?;
    let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
        detail(format!(
            "cannot inspect {}: {error}",
            entry.path().display()
        ))
    })?;
    let file_type = metadata.file_type();
    let is_semantic_receipt = prefix.is_empty() && name == SEMANTIC_RECEIPT_NAME;
    if is_semantic_receipt && !file_type.is_file() {
        return Err(StagingRefusal::SemanticReceiptNotRegularFile);
    }
    if metadata_has_indirection(&metadata) {
        return Err(detail(format!(
            "staging contains filesystem indirection `{relative}`"
        )));
    }
    if file_type.is_dir() {
        reject_payload_directory(&name)?;
        pending.push((entry.path(), relative));
        return Ok(());
    }
    if !file_type.is_file() {
        return Err(detail(format!(
            "staging contains non-file entry `{relative}`"
        )));
    }
    reject_payload_file(&name)?;
    let (byte_count, hash) = hash_file(&entry.path(), census.remaining_bytes()).map_err(detail)?;
    census.admit_bytes(byte_count).map_err(detail)?;
    if is_semantic_receipt {
        *semantic_receipt_bytes = Some(byte_count);
    }
    records.push(SourceRecord {
        source: entry.path(),
        manifest: PayloadRecord {
            path: relative,
            bytes: byte_count,
            hash,
        },
    });
    Ok(())
}

fn detail(message: String) -> StagingRefusal {
    StagingRefusal::Detail(message)
}

fn refuse_reserved_staging_path(relative: &str) -> Result<(), StagingRefusal> {
    if relative.eq_ignore_ascii_case(MANIFEST_NAME)
        || relative.eq_ignore_ascii_case(PLACEMENT_RECEIPT_NAME)
        || relative.eq_ignore_ascii_case(COMPLETION_NAME)
    {
        return Err(detail(format!("staging uses reserved path `{relative}`")));
    }
    Ok(())
}

pub(crate) fn census_bundle(
    run: &Path,
    budget: CensusBudget,
) -> Result<Vec<PayloadRecord>, String> {
    let mut pending = vec![(run.to_path_buf(), String::new())];
    let mut records = Vec::new();
    let mut census = CensusProgress::new(budget);
    while let Some((directory, prefix)) = pending.pop() {
        let entries = read_entries(&directory, census.remaining_entries())?;
        if entries.is_empty() && !prefix.is_empty() {
            return Err(format!(
                "bundle contains unrepresented empty directory `{prefix}`"
            ));
        }
        for entry in entries {
            census.admit_entry()?;
            inspect_bundle_entry(&entry, &prefix, &mut pending, &mut records, &mut census)?;
        }
    }
    records.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(records)
}

fn inspect_bundle_entry(
    entry: &fs::DirEntry,
    prefix: &str,
    pending: &mut Vec<(PathBuf, String)>,
    records: &mut Vec<PayloadRecord>,
    census: &mut CensusProgress,
) -> Result<(), String> {
    let name = manifest::normalize_component(&entry.file_name())?;
    let relative = join_relative(prefix, &name);
    let metadata = fs::symlink_metadata(entry.path())
        .map_err(|error| format!("cannot inspect {}: {error}", entry.path().display()))?;
    let file_type = metadata.file_type();
    if metadata_has_indirection(&metadata) {
        return Err(format!(
            "bundle contains filesystem indirection `{relative}`"
        ));
    }
    if file_type.is_dir() {
        if prefix.is_empty() && name == COMPLETION_NAME {
            let mut entries = fs::read_dir(entry.path())
                .map_err(|error| format!("cannot read completion marker directory: {error}"))?;
            match entries.next() {
                Some(Ok(_)) => {
                    return Err("completion marker directory is not empty".to_owned());
                }
                Some(Err(error)) => {
                    return Err(format!("cannot enumerate completion marker: {error}"));
                }
                None => {}
            }
            return Ok(());
        }
        reject_payload_directory(&name)?;
        pending.push((entry.path(), relative));
        return Ok(());
    }
    if !file_type.is_file() {
        return Err(format!("bundle contains non-file entry `{relative}`"));
    }
    if relative == MANIFEST_NAME {
        let (byte_count, _hash) = hash_file(&entry.path(), census.remaining_bytes())?;
        census.admit_bytes(byte_count)?;
        return Ok(());
    }
    reject_payload_file(&name)?;
    manifest::validate_normalized_path(&relative)?;
    let (byte_count, hash) = hash_file(&entry.path(), census.remaining_bytes())?;
    census.admit_bytes(byte_count)?;
    records.push(PayloadRecord {
        path: relative,
        bytes: byte_count,
        hash,
    });
    Ok(())
}

fn read_entries(directory: &Path, entry_limit: u64) -> Result<Vec<fs::DirEntry>, String> {
    let reader = fs::read_dir(directory)
        .map_err(|error| format!("cannot read directory {}: {error}", directory.display()))?;
    let mut entries = Vec::new();
    for entry in reader {
        entries.push(entry.map_err(|error| {
            format!(
                "cannot enumerate directory {}: {error}",
                directory.display()
            )
        })?);
        let entry_count = u64::try_from(entries.len())
            .map_err(|error| format!("directory entry count cannot be represented: {error}"))?;
        if entry_count > entry_limit {
            return Err(format!(
                "staging exceeds its declared entry limit while reading {}",
                directory.display()
            ));
        }
    }
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn join_relative(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}/{name}")
    }
}

fn reject_payload_directory(name: &str) -> Result<(), String> {
    if name.eq_ignore_ascii_case("target")
        || name.eq_ignore_ascii_case(".durafx")
        || name.eq_ignore_ascii_case(COMPLETION_NAME)
    {
        return Err(format!(
            "staging contains forbidden storage component `{name}`"
        ));
    }
    let cargo_directories = [".fingerprint", "build", "deps", "incremental"];
    if cargo_directories
        .iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
    {
        return Err(format!("staging contains Cargo build directory `{name}`"));
    }
    Ok(())
}

fn reject_payload_file(name: &str) -> Result<(), String> {
    if name.eq_ignore_ascii_case("target")
        || name.eq_ignore_ascii_case(".durafx")
        || name.eq_ignore_ascii_case(COMPLETION_NAME)
    {
        return Err(format!(
            "staging contains forbidden storage component `{name}`"
        ));
    }
    let lowercase = name.to_ascii_lowercase();
    let compiled_suffixes = [
        ".a", ".d", ".dll", ".dylib", ".exe", ".lib", ".o", ".obj", ".pdb", ".rlib", ".rmeta",
        ".so", ".wasm",
    ];
    let compiled = lowercase.starts_with("build-script-")
        || compiled_suffixes
            .iter()
            .any(|suffix| lowercase.ends_with(suffix));
    if compiled {
        return Err(format!(
            "staging contains compiled or Cargo build artifact `{name}`"
        ));
    }
    Ok(())
}

fn validate_staging_location(path: &Path) -> Result<(), String> {
    let components = normal_components(path)?;
    if components
        .iter()
        .any(|component| component.eq_ignore_ascii_case(".durafx"))
    {
        return Err(format!(
            "staging cannot be inside `.durafx`: {}",
            path.display()
        ));
    }
    let target_count = components
        .iter()
        .filter(|component| component.eq_ignore_ascii_case("target"))
        .count();
    if target_count == 0 {
        return Ok(());
    }
    let lawful_handoff = components
        .windows(3)
        .last()
        .is_some_and(|window| match window {
            [target, qualification, _run] => {
                target.eq_ignore_ascii_case("target")
                    && qualification.eq_ignore_ascii_case("qualification")
            }
            _ => false,
        });
    if target_count == 1 && lawful_handoff {
        return Ok(());
    }
    Err(format!(
        "staging under `target` must name one explicit `target/qualification/<run>` tree: {}",
        path.display()
    ))
}

fn normal_components(path: &Path) -> Result<Vec<String>, String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(name) => Some(
                name.to_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("staging path is not UTF-8: {}", path.display())),
            ),
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => None,
        })
        .collect()
}

fn hash_file(path: &Path, byte_limit: u64) -> Result<(u64, String), String> {
    let mut file = File::open(path)
        .map_err(|error| format!("cannot open payload {}: {error}", path.display()))?;
    let mut hasher = blake3::Hasher::new();
    let mut byte_count = 0_u64;
    let mut buffer = vec![0_u8; COPY_BUFFER_BYTES];
    loop {
        let read_count = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot read payload {}: {error}", path.display()))?;
        if read_count == 0 {
            break;
        }
        let chunk = buffer
            .get(..read_count)
            .ok_or_else(|| format!("read exceeded buffer for {}", path.display()))?;
        let read_bytes = u64::try_from(read_count)
            .map_err(|error| format!("payload size cannot be represented: {error}"))?;
        let next_count = byte_count
            .checked_add(read_bytes)
            .ok_or_else(|| format!("payload is too large: {}", path.display()))?;
        if next_count > byte_limit {
            return Err(format!(
                "payload bytes exceed the declared limit while reading {}",
                path.display()
            ));
        }
        hasher.update(chunk);
        byte_count = next_count;
    }
    Ok((byte_count, hasher.finalize().to_hex().to_string()))
}

pub(crate) fn payload_path(root: &Path, normalized: &str) -> PathBuf {
    normalized
        .split('/')
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

pub(crate) fn copy_checked(source: &SourceRecord, destination: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(&source.source)
        .map_err(|error| format!("cannot recheck {}: {error}", source.source.display()))?;
    if metadata_has_indirection(&metadata) || !metadata.is_file() {
        return Err(format!(
            "payload changed type while sealing: {}",
            source.source.display()
        ));
    }
    let mut input = File::open(&source.source)
        .map_err(|error| format!("cannot reopen {}: {error}", source.source.display()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    let (byte_count, hash) = transfer(&mut input, &mut output, destination, source.manifest.bytes)?;
    if byte_count != source.manifest.bytes || hash != source.manifest.hash {
        return Err(format!(
            "payload changed while sealing: `{}`",
            source.manifest.path
        ));
    }
    Ok(())
}

fn transfer(
    input: &mut File,
    output: &mut File,
    destination: &Path,
    byte_limit: u64,
) -> Result<(u64, String), String> {
    let mut hasher = blake3::Hasher::new();
    let mut byte_count = 0_u64;
    let mut buffer = vec![0_u8; COPY_BUFFER_BYTES];
    loop {
        let read_count = input
            .read(&mut buffer)
            .map_err(|error| format!("cannot copy to {}: {error}", destination.display()))?;
        if read_count == 0 {
            break;
        }
        let chunk = buffer
            .get(..read_count)
            .ok_or_else(|| format!("copy exceeded buffer for {}", destination.display()))?;
        let read_bytes = u64::try_from(read_count)
            .map_err(|error| format!("payload size cannot be represented: {error}"))?;
        let next_count = byte_count
            .checked_add(read_bytes)
            .ok_or_else(|| format!("payload is too large: {}", destination.display()))?;
        if next_count > byte_limit {
            return Err(format!(
                "payload changed beyond its declared length while copying to {}",
                destination.display()
            ));
        }
        output
            .write_all(chunk)
            .map_err(|error| format!("cannot write {}: {error}", destination.display()))?;
        hasher.update(chunk);
        byte_count = next_count;
    }
    output
        .sync_all()
        .map_err(|error| format!("cannot flush {}: {error}", destination.display()))?;
    Ok((byte_count, hasher.finalize().to_hex().to_string()))
}

pub(crate) fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot create {}: {error}", path.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    file.sync_all()
        .map_err(|error| format!("cannot flush {}: {error}", path.display()))
}

pub(crate) fn apply_readonly_guard(root: &Path, entry_limit: u64) -> Result<(), String> {
    let mut paths = collect_tree_paths(root, entry_limit)?;
    paths.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for path in paths {
        guard_path(&path)?;
    }
    Ok(())
}

fn collect_tree_paths(root: &Path, entry_limit: u64) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        let path_count = u64::try_from(paths.len())
            .map_err(|error| format!("guard path count cannot be represented: {error}"))?;
        let remaining = entry_limit.saturating_sub(path_count);
        for entry in read_entries(&directory, remaining)? {
            let metadata = fs::symlink_metadata(entry.path())
                .map_err(|error| format!("cannot inspect {}: {error}", entry.path().display()))?;
            let file_type = metadata.file_type();
            if metadata_has_indirection(&metadata) {
                return Err(format!(
                    "cannot guard filesystem indirection {}",
                    entry.path().display()
                ));
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            }
            paths.push(entry.path());
            let next_path_count = u64::try_from(paths.len())
                .map_err(|error| format!("guard path count cannot be represented: {error}"))?;
            if next_path_count > entry_limit {
                return Err(format!(
                    "bundle exceeds its {entry_limit}-entry guard limit"
                ));
            }
        }
    }
    Ok(paths)
}

#[cfg(unix)]
fn guard_path(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect permissions for {}: {error}", path.display()))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() & !0o222);
    fs::set_permissions(path, permissions)
        .map_err(|error| format!("cannot set permissions for {}: {error}", path.display()))
}

#[cfg(not(unix))]
fn guard_path(path: &Path) -> Result<(), String> {
    let _metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect guard target {}: {error}", path.display()))?;
    Ok(())
}

#[cfg(unix)]
fn release_path(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect permissions for {}: {error}", path.display()))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() | 0o200);
    fs::set_permissions(path, permissions)
        .map_err(|error| format!("cannot set permissions for {}: {error}", path.display()))
}

#[cfg(not(unix))]
fn release_path(path: &Path) -> Result<(), String> {
    let _metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect release target {}: {error}", path.display()))?;
    Ok(())
}

pub(crate) fn make_tree_writable(root: &Path, entry_limit: u64) -> Result<(), String> {
    let mut paths = collect_tree_paths(root, entry_limit)?;
    paths.sort_by_key(|path| path.components().count());
    release_path(root)?;
    for path in paths {
        release_path(&path)?;
    }
    Ok(())
}

pub(crate) fn remove_temporary(path: &Path, entry_limit: u64) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata_has_indirection(&metadata) || !metadata.is_dir() {
                return Err(format!(
                    "temporary seat is not a real directory: {}",
                    path.display()
                ));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(format!(
                "cannot inspect temporary seat {}: {error}",
                path.display()
            ));
        }
    }
    make_tree_writable(path, entry_limit)?;
    fs::remove_dir_all(path)
        .map_err(|error| format!("cannot remove temporary seat {}: {error}", path.display()))
}

pub(crate) fn canonical_real_directory(path: &Path, role: &str) -> Result<PathBuf, String> {
    let mut ancestors = path.ancestors().collect::<Vec<_>>();
    ancestors.reverse();
    for ancestor in ancestors {
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        let metadata = fs::symlink_metadata(ancestor).map_err(|error| {
            format!(
                "cannot inspect {role} component {}: {error}",
                ancestor.display()
            )
        })?;
        if metadata_has_indirection(&metadata) {
            return Err(format!(
                "{role} path contains filesystem indirection at {}",
                ancestor.display()
            ));
        }
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect {role} {}: {error}", path.display()))?;
    if !metadata.is_dir() {
        return Err(format!("{role} is not a directory: {}", path.display()));
    }
    fs::canonicalize(path)
        .map_err(|error| format!("cannot resolve {role} {}: {error}", path.display()))
}

pub(crate) fn metadata_has_indirection(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink() || platform_reparse_point(metadata)
}

#[cfg(windows)]
fn platform_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
const fn platform_reparse_point(_metadata: &fs::Metadata) -> bool {
    false
}

pub(crate) fn read_bounded(path: &Path, byte_limit: u64, role: &str) -> Result<Vec<u8>, String> {
    let file = File::open(path)
        .map_err(|error| format!("cannot open {role} {}: {error}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(byte_limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read {role} {}: {error}", path.display()))?;
    let observed = u64::try_from(bytes.len())
        .map_err(|error| format!("{role} length cannot be represented: {error}"))?;
    if observed > byte_limit {
        return Err(format!("{role} exceeds its {byte_limit}-byte limit"));
    }
    Ok(bytes)
}

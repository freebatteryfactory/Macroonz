//! External observations of the declared evidence-sealing protocol.

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MANIFEST_NAME: &str = "DURAFX-MANIFEST.blake3";
const COMPLETION_NAME: &str = "DURAFX-COMPLETE";
const TEST_RECEIPT: &[u8] = b"# Test receipt\n\nThe fixture declares its proposition, command, tool posture, observed result, and evidence ceiling.\n";

struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let root =
            std::env::temp_dir().join(format!("durafx-sealer-test-{}-{name}", std::process::id()));
        if root.exists() {
            make_writable(&root)?;
            fs::remove_dir_all(&root)?;
        }
        fs::create_dir(&root)?;
        Ok(Self { root })
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.root.exists() {
            let _writable_result = make_writable(&self.root);
            let _remove_result = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn identical_declared_inputs_are_deterministic() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("determinism")?;
    let first_staging = stage_with_receipt(
        &scratch,
        "one/stage",
        &[("nested/a.txt", b"alpha"), ("b.bin", b"beta")],
    )?;
    let second_staging = stage_with_receipt(
        &scratch,
        "two/stage",
        &[("nested/a.txt", b"alpha"), ("b.bin", b"beta")],
    )?;
    let first_repository = scratch.path("one");
    let second_repository = scratch.path("two");
    let first = seal(&first_staging, &first_repository, "pilot")?;
    let second = seal(&second_staging, &second_repository, "pilot")?;
    assert_eq!(first.file_name(), second.file_name());
    assert_eq!(
        fs::read(first.join(MANIFEST_NAME))?,
        fs::read(second.join(MANIFEST_NAME))?
    );
    assert!(first.join(COMPLETION_NAME).is_dir());
    assert_advisory_guard(&first.join("nested/a.txt"))?;
    assert!(verify(&first_repository, &first)?.status.success());
    assert!(verify(&second_repository, &second)?.status.success());
    Ok(())
}

#[test]
fn distinct_payloads_do_not_collide() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("collision")?;
    let first_stage = stage_with_receipt(&scratch, "first", &[("payload", b"first")])?;
    let second_stage = stage_with_receipt(&scratch, "second", &[("payload", b"second")])?;
    let first = seal(&first_stage, &scratch.root, "same-label")?;
    let second = seal(&second_stage, &scratch.root, "same-label")?;
    assert_ne!(first, second);
    assert!(first.is_dir());
    assert!(second.is_dir());
    Ok(())
}

#[test]
fn an_existing_destination_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("existing")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"same")])?;
    let first = seal(&staging, &scratch.root, "repeat")?;
    let before = fs::read(first.join(MANIFEST_NAME))?;
    let second = seal_output(&staging, &scratch.root, "repeat")?;
    assert!(!second.status.success());
    assert!(failure(&second).contains("destination already exists"));
    assert_eq!(fs::read(first.join(MANIFEST_NAME))?, before);
    assert!(verify(&scratch.root, &first)?.status.success());
    Ok(())
}

#[test]
fn verification_reports_byte_drift() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("byte-drift")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"before")])?;
    let run = seal(&staging, &scratch.root, "drift")?;
    make_writable(&run)?;
    fs::write(run.join("payload"), b"after")?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("changed file `payload`"));
    Ok(())
}

#[test]
fn verification_reports_an_added_file() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("added")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "added")?;
    make_writable(&run)?;
    fs::write(run.join("unexpected"), b"extra")?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("additional file `unexpected`"));
    Ok(())
}

#[test]
fn verification_reports_a_removed_file() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("removed")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "removed")?;
    make_writable(&run)?;
    fs::remove_file(run.join("payload"))?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("missing file `payload`"));
    Ok(())
}

#[test]
fn manifest_paths_are_slash_normalized() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("normalized")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("nested/leaf.bin", b"bytes")])?;
    let run = seal(&staging, &scratch.root, "normalized")?;
    let manifest = fs::read_to_string(run.join(MANIFEST_NAME))?;
    assert!(manifest.contains("\tnested/leaf.bin\n"));
    assert!(!manifest.contains("nested\\leaf.bin"));
    Ok(())
}

#[test]
fn verification_reports_directory_key_disagreement() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("directory-key")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "key")?;
    let moved = run.with_file_name("wrong-key");
    fs::rename(&run, &moved)?;
    let output = verify(&scratch.root, &moved)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("directory key mismatch for run key"));
    Ok(())
}

#[test]
fn staged_symlinks_refuse() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("symlink")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let link = staging.join("linked");
    create_file_symlink(&staging.join("payload"), &link)?;
    let output = seal_output(&staging, &scratch.root, "symlink")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("staging contains filesystem indirection"));
    Ok(())
}

#[cfg(windows)]
#[test]
fn staged_windows_junctions_refuse() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("junction")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let target = scratch.path("junction-target");
    fs::create_dir(&target)?;
    let junction = staging.join("linked-directory");
    let creation = Command::new("cmd")
        .arg("/D")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(&junction)
        .arg(&target)
        .output()?;
    if !creation.status.success() {
        return Err(format!(
            "cannot create Windows junction fixture: {}",
            String::from_utf8_lossy(&creation.stderr)
        )
        .into());
    }
    let output = seal_output(&staging, &scratch.root, "junction")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("staging contains filesystem indirection"));
    Ok(())
}

#[test]
fn missing_semantic_receipt_has_a_typed_refusal() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("missing-semantic-receipt")?;
    let staging = stage_without_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let output = seal_output(&staging, &scratch.root, "missing-receipt")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("missing-semantic-receipt"));
    assert!(failure(&output).contains("caller-authored `receipt.md`"));
    Ok(())
}

#[test]
fn an_explicit_target_qualification_run_is_lawful_staging() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("qualification-staging")?;
    let staging = stage_with_receipt(
        &scratch,
        "target/qualification/replay-posture-mutants",
        &[("mutants.out/outcomes.json", b"{}")],
    )?;
    let run = seal(&staging, &scratch.root, "qualification")?;
    assert!(run.join("receipt.md").is_file());
    assert!(run.join("mutants.out/outcomes.json").is_file());
    let manifest = fs::read_to_string(run.join(MANIFEST_NAME))?;
    assert!(manifest.contains("\tDURAFX-RECEIPT.txt\n"));
    assert!(manifest.contains("\treceipt.md\n"));
    assert!(verify(&scratch.root, &run)?.status.success());
    Ok(())
}

#[test]
fn a_missing_label_field_refuses_without_a_default() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("missing-label")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let request = seal_request(&staging, &scratch.root, "missing")?;
    let mut request_without_label = request
        .strip_suffix("\tlabel=missing\n")
        .ok_or("test request lacks its label field")?
        .to_owned();
    request_without_label.push('\n');
    let output = invoke(request_without_label.as_bytes())?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("request lacks required field `label`"));
    Ok(())
}

#[test]
fn non_utf8_protocol_bytes_refuse() -> Result<(), Box<dyn Error>> {
    let output = invoke(&[0xff_u8, b'\n'])?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("request is not UTF-8"));
    Ok(())
}

#[test]
fn an_embedded_protocol_newline_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("protocol-embedded-newline")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let request = seal_request(&staging, &scratch.root, "crlf")?;
    let hostile = request.replace("\trepository=", "\nrepository=");
    let output = invoke(hostile.as_bytes())?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("control characters"));
    Ok(())
}

#[test]
fn a_crlf_record_terminator_is_unambiguous() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("protocol-crlf")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let request = seal_request(&staging, &scratch.root, "crlf")?;
    let mut crlf = request
        .strip_suffix('\n')
        .ok_or("test request lacks its record terminator")?
        .to_owned();
    crlf.push_str("\r\n");
    let output = invoke(crlf.as_bytes())?;
    assert!(output.status.success(), "{}", failure(&output));
    Ok(())
}

#[test]
fn an_additional_protocol_field_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("protocol-additional")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let mut request = seal_request(&staging, &scratch.root, "additional")?;
    let terminator = request.pop();
    assert_eq!(terminator, Some('\n'));
    request.push_str("\tunexpected=value\n");
    let output = invoke(request.as_bytes())?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("request has additional field"));
    Ok(())
}

#[test]
fn a_relative_protocol_path_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("protocol-relative")?;
    let request = format!(
        "durafx-sealer-request-v1\tcommand=seal\trepository={}\tstaging=relative/stage\tplane=fuzz\tsource-revision=3d0a66ac7ce10f9347b33dc8afd1dd7cc118b6e2\thost-target=windows-x86_64-pc-windows-msvc\tentry-limit=1000\tbyte-limit=1048576\tlabel=relative\n",
        path_text(&scratch.root)?
    );
    let output = invoke(request.as_bytes())?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("must be an absolute path"));
    Ok(())
}

#[test]
fn a_protocol_without_a_record_terminator_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("protocol-terminator")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let request = seal_request(&staging, &scratch.root, "terminator")?;
    let unterminated = request
        .strip_suffix('\n')
        .ok_or("test request lacks its record terminator")?;
    let output = invoke(unterminated.as_bytes())?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("record terminator"));
    Ok(())
}

#[test]
fn staging_outside_the_declared_repository_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("repository-boundary")?;
    let staging = stage_with_receipt(&scratch, "source-repository/stage", &[("payload", b"kept")])?;
    let repository = scratch.path("warehouse-repository");
    fs::create_dir(&repository)?;
    let output = seal_output(&staging, &repository, "repository")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("strict descendant"));
    assert!(!repository.join(".durafx").exists());
    Ok(())
}

#[test]
fn target_and_a_cargo_profile_tree_refuse_as_staging() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("profile-staging")?;
    let target = stage_with_receipt(&scratch, "target", &[("root-output", b"compiled")])?;
    let profile = stage_with_receipt(&scratch, "target/debug", &[("output", b"compiled")])?;
    for staging in [target, profile] {
        let output = seal_output(&staging, &scratch.root, "profile")?;
        assert!(!output.status.success());
        assert!(failure(&output).contains("target/qualification/<run>"));
    }
    Ok(())
}

#[test]
fn a_descendant_of_a_qualification_run_refuses_as_staging() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("qualification-descendant")?;
    let staging = stage_with_receipt(
        &scratch,
        "target/qualification/run/descendant",
        &[("payload", b"evidence")],
    )?;
    let output = seal_output(&staging, &scratch.root, "descendant")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("target/qualification/<run>"));
    Ok(())
}

#[test]
fn the_repository_root_cannot_be_staging() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("repository-overlap")?;
    let _receipt = stage_with_receipt(&scratch, "stage", &[("payload", b"evidence")])?;
    let output = seal_output(&scratch.root, &scratch.root, "overlap")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("strict descendant"));
    assert!(!scratch.root.join(".durafx").exists());
    Ok(())
}

#[test]
fn a_nested_compiled_artifact_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("compiled-artifact")?;
    let staging = stage_with_receipt(
        &scratch,
        "target/qualification/run",
        &[("driver.exe", b"compiled")],
    )?;
    let output = seal_output(&staging, &scratch.root, "compiled")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("compiled or Cargo build artifact"));
    Ok(())
}

#[test]
fn nested_cargo_build_debris_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("build-debris")?;
    let staging = stage_with_receipt(
        &scratch,
        "target/qualification/run",
        &[(".fingerprint/state", b"compiled")],
    )?;
    let output = seal_output(&staging, &scratch.root, "debris")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("Cargo build directory"));
    Ok(())
}

#[test]
fn a_missing_completion_marker_refuses_verification() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("missing-completion")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "missing-completion")?;
    make_writable(&run)?;
    fs::remove_dir(run.join(COMPLETION_NAME))?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("run is incomplete"));
    Ok(())
}

#[test]
fn a_nonempty_completion_marker_refuses_verification() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("nonempty-completion")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "nonempty-completion")?;
    fs::write(run.join(COMPLETION_NAME).join("unexpected"), b"material")?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("completion marker is not empty"));
    Ok(())
}

#[test]
fn a_file_cannot_impersonate_the_completion_marker() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("file-completion")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "file-completion")?;
    fs::remove_dir(run.join(COMPLETION_NAME))?;
    fs::write(run.join(COMPLETION_NAME), b"complete")?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("completion marker is not a real directory"));
    Ok(())
}

#[test]
fn staged_empty_directories_refuse_instead_of_disappearing() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("staged-empty-directory")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    fs::create_dir(staging.join("empty"))?;
    let output = seal_output(&staging, &scratch.root, "empty")?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("unrepresented empty directory `empty`"));
    Ok(())
}

#[test]
fn added_empty_directories_refuse_verification() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("added-empty-directory")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "added-empty")?;
    fs::create_dir(run.join("empty"))?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("unrepresented empty directory `empty`"));
    Ok(())
}

#[test]
fn exact_declared_limits_admit_and_one_less_refuses() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("declared-limits")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"x")])?;
    let byte_limit = u64::try_from(TEST_RECEIPT.len())?
        .checked_add(1)
        .ok_or("test byte limit overflowed")?;
    let exact = seal_with_limits(&staging, &scratch.root, "exact", 2, byte_limit)?;
    assert!(verify(&scratch.root, &exact)?.status.success());

    let entry_refusal =
        seal_output_with_limits(&staging, &scratch.root, "entry-low", 1, byte_limit)?;
    assert!(!entry_refusal.status.success());
    assert!(failure(&entry_refusal).contains("declared entry limit"));

    let byte_refusal = seal_output_with_limits(
        &staging,
        &scratch.root,
        "byte-low",
        2,
        byte_limit.saturating_sub(1),
    )?;
    assert!(!byte_refusal.status.success());
    assert!(failure(&byte_refusal).contains("declared limit"));
    Ok(())
}

#[test]
fn labels_are_navigation_while_limits_are_identity() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("identity-inputs")?;
    let first_staging = stage_with_receipt(&scratch, "first", &[("payload", b"same")])?;
    let second_staging = stage_with_receipt(&scratch, "second", &[("payload", b"same")])?;
    let first = seal_with_limits(&first_staging, &scratch.root, "first-label", 10, 4096)?;
    let second = seal_with_limits(&second_staging, &scratch.root, "second-label", 10, 4096)?;
    let first_digest = run_digest(&first, "first-label")?;
    let second_digest = run_digest(&second, "second-label")?;
    assert_eq!(first_digest, second_digest);
    assert_ne!(
        fs::read(first.join(MANIFEST_NAME))?,
        fs::read(second.join(MANIFEST_NAME))?
    );

    let third_staging = stage_with_receipt(&scratch, "third", &[("payload", b"same")])?;
    let third = seal_with_limits(&third_staging, &scratch.root, "first-label", 11, 4096)?;
    assert_ne!(first_digest, run_digest(&third, "first-label")?);
    Ok(())
}

#[test]
fn a_manifest_beyond_its_declared_byte_budget_refuses_before_comparison()
-> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("manifest-budget")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"kept")])?;
    let run = seal(&staging, &scratch.root, "manifest-budget")?;
    make_writable(&run)?;
    let manifest_path = run.join(MANIFEST_NAME);
    let manifest = fs::read_to_string(&manifest_path)?;
    let payload_row = manifest
        .lines()
        .find(|line| line.ends_with("\tpayload"))
        .ok_or("manifest lacks payload row")?;
    let mut fields = payload_row.splitn(4, '\t');
    let kind = fields.next().ok_or("payload row lacks kind")?;
    let _bytes = fields.next().ok_or("payload row lacks bytes")?;
    let hash = fields.next().ok_or("payload row lacks hash")?;
    let path = fields.next().ok_or("payload row lacks path")?;
    let hostile_row = format!("{kind}\t1048577\t{hash}\t{path}");
    fs::write(&manifest_path, manifest.replace(payload_row, &hostile_row))?;
    let output = verify(&scratch.root, &run)?;
    assert!(!output.status.success());
    assert!(failure(&output).contains("manifest declares evidence bytes beyond"));
    Ok(())
}

#[test]
fn concurrent_same_key_sealers_have_one_winner() -> Result<(), Box<dyn Error>> {
    let scratch = Scratch::new("concurrent-reservation")?;
    let staging = stage_with_receipt(&scratch, "stage", &[("payload", b"same")])?;
    let request = seal_request(&staging, &scratch.root, "concurrent")?.into_bytes();
    let other_request = request.clone();
    let first_worker =
        std::thread::spawn(move || invoke(&request).map_err(|error| error.to_string()));
    let second_worker =
        std::thread::spawn(move || invoke(&other_request).map_err(|error| error.to_string()));
    let first = first_worker
        .join()
        .map_err(|_| "first sealer worker panicked")??;
    let second = second_worker
        .join()
        .map_err(|_| "second sealer worker panicked")??;
    let successes = [first.status.success(), second.status.success()]
        .into_iter()
        .filter(|success| *success)
        .count();
    assert_eq!(successes, 1);
    let successful = if first.status.success() {
        &first
    } else {
        &second
    };
    let run = PathBuf::from(String::from_utf8(successful.stdout.clone())?.trim());
    assert!(verify(&scratch.root, &run)?.status.success());
    Ok(())
}

fn stage_with_receipt(
    scratch: &Scratch,
    name: &str,
    files: &[(&str, &[u8])],
) -> Result<PathBuf, Box<dyn Error>> {
    let root = stage_without_receipt(scratch, name, files)?;
    let receipt = root.join("receipt.md");
    fs::write(receipt, TEST_RECEIPT)?;
    Ok(root)
}

fn stage_without_receipt(
    scratch: &Scratch,
    name: &str,
    files: &[(&str, &[u8])],
) -> Result<PathBuf, Box<dyn Error>> {
    let root = scratch.path(name);
    fs::create_dir_all(&root)?;
    for (relative, bytes) in files {
        let path = relative
            .split('/')
            .fold(root.clone(), |path, component| path.join(component));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }
    Ok(root)
}

fn seal(staging: &Path, repository: &Path, label: &str) -> Result<PathBuf, Box<dyn Error>> {
    seal_with_limits(staging, repository, label, 1000, 1_048_576)
}

fn seal_with_limits(
    staging: &Path,
    repository: &Path,
    label: &str,
    entry_limit: u64,
    byte_limit: u64,
) -> Result<PathBuf, Box<dyn Error>> {
    let output = seal_output_with_limits(staging, repository, label, entry_limit, byte_limit)?;
    if !output.status.success() {
        return Err(failure(&output).into());
    }
    Ok(PathBuf::from(String::from_utf8(output.stdout)?.trim()))
}

fn seal_output(staging: &Path, repository: &Path, label: &str) -> Result<Output, Box<dyn Error>> {
    seal_output_with_limits(staging, repository, label, 1000, 1_048_576)
}

fn seal_output_with_limits(
    staging: &Path,
    repository: &Path,
    label: &str,
    entry_limit: u64,
    byte_limit: u64,
) -> Result<Output, Box<dyn Error>> {
    let request = seal_request_with_limits(staging, repository, label, entry_limit, byte_limit)?;
    invoke(request.as_bytes())
}

fn verify(repository: &Path, run: &Path) -> Result<Output, Box<dyn Error>> {
    let request = format!(
        "durafx-sealer-request-v1\tcommand=verify\trepository={}\trun={}\n",
        path_text(repository)?,
        path_text(run)?
    );
    invoke(request.as_bytes())
}

fn seal_request(staging: &Path, repository: &Path, label: &str) -> Result<String, Box<dyn Error>> {
    seal_request_with_limits(staging, repository, label, 1000, 1_048_576)
}

fn seal_request_with_limits(
    staging: &Path,
    repository: &Path,
    label: &str,
    entry_limit: u64,
    byte_limit: u64,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "durafx-sealer-request-v1\tcommand=seal\trepository={}\tstaging={}\tplane=fuzz\tsource-revision=3d0a66ac7ce10f9347b33dc8afd1dd7cc118b6e2\thost-target=windows-x86_64-pc-windows-msvc\tentry-limit={entry_limit}\tbyte-limit={byte_limit}\tlabel={label}\n",
        path_text(repository)?,
        path_text(staging)?,
    ))
}

fn run_digest(run: &Path, label: &str) -> Result<String, Box<dyn Error>> {
    let key = run
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or("run key is not UTF-8")?;
    key.strip_prefix(&format!("{label}-"))
        .map(str::to_owned)
        .ok_or_else(|| Box::<dyn Error>::from("run key does not carry its label"))
}

fn path_text(path: &Path) -> Result<&str, Box<dyn Error>> {
    path.to_str()
        .ok_or_else(|| Box::<dyn Error>::from("protocol path is not UTF-8"))
}

fn invoke(request: &[u8]) -> Result<Output, Box<dyn Error>> {
    let mut child = Command::new(binary())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or("child standard input was not piped")?;
    input.write_all(request)?;
    drop(input);
    let output = child.wait_with_output()?;
    Ok(output)
}

fn binary() -> &'static OsStr {
    OsStr::new(env!("CARGO_BIN_EXE_durafx-sealer"))
}

fn failure(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn make_writable(root: &Path) -> Result<(), Box<dyn Error>> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let metadata = fs::symlink_metadata(&path)?;
        set_writable(&path, metadata.permissions())?;
        if metadata.is_dir() {
            for entry in fs::read_dir(&path)? {
                pending.push(entry?.path());
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn assert_advisory_guard(path: &Path) -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::PermissionsExt;

    let mode = fs::metadata(path)?.permissions().mode();
    assert_eq!(mode & 0o222, 0);
    Ok(())
}

#[cfg(not(unix))]
fn assert_advisory_guard(path: &Path) -> Result<(), Box<dyn Error>> {
    let _metadata = fs::metadata(path)?;
    Ok(())
}

#[cfg(unix)]
fn set_writable(path: &Path, mut permissions: fs::Permissions) -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::PermissionsExt;

    permissions.set_mode(permissions.mode() | 0o200);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_writable(path: &Path, _permissions: fs::Permissions) -> Result<(), Box<dyn Error>> {
    let _metadata = fs::symlink_metadata(path)?;
    Ok(())
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}

#[cfg(not(any(unix, windows)))]
fn create_file_symlink(_target: &Path, _link: &Path) -> std::io::Result<()> {
    Err(std::io::ErrorKind::Unsupported.into())
}

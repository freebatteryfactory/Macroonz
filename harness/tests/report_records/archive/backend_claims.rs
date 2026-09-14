//! Original-console loss markers and historical callback claims remain inspectable.

use super::backend_vector::{self as vector, BackendVector, FILE, Material, SOURCE};
use super::backends::LIMITS;
use super::proposal_vector::{TargetVector, name};
use super::trial_vector::foreign;
use super::vector::{frame, hash};
use macroonz_harness::muterprater::backend_archive::{BackendArchiveRefusal, read_backend};
use macroonz_harness::report::TextFidelity;
use macroonz_harness::report::archive::ArchivedTruncation;

#[test]
fn full_console_checks_foreign_loss_markers_without_losing_the_original_suffix() -> Result<(), ()> {
    let offered = format!("{}é", "a".repeat(4095));
    let console = vector::CONSOLE.replace("summary", &offered);
    let mut vector = BackendVector::declared(Material::Complete);
    vector.output.clear();
    frame(
        &hash("mutation-backend-output/v1", console.as_bytes()),
        &mut vector.output,
    );
    vector.material = vector::material(console.as_bytes(), &[SOURCE]);
    let mut loss = vec![1];
    loss.extend_from_slice(&4096u64.to_be_bytes());
    loss.extend_from_slice(&4097u64.to_be_bytes());
    loss.push(1);
    let prefix = offered.as_bytes().get(..4096).ok_or(())?;
    vector.unparsed = vector::unread(1, &[(3, foreign(prefix, &loss))]);
    let record = read_backend(&vector.encoded(), LIMITS).map_err(|_| ())?;
    let [line] = record.unparsed() else {
        return Err(());
    };
    assert_eq!(line.text().bytes(), prefix);
    assert_eq!(line.text().fidelity(), TextFidelity::LossyReplacement);
    assert_eq!(
        line.text().truncation(),
        ArchivedTruncation::TruncatedAt {
            admitted: 4096,
            offered: 4097
        }
    );
    assert_eq!(record.original_console(), Some(console.as_bytes()));
    loss.get_mut(9..17)
        .ok_or(())?
        .copy_from_slice(&4098u64.to_be_bytes());
    vector.unparsed = vector::unread(1, &[(3, foreign(prefix, &loss))]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::ConsoleReadingMismatch)
    );
    Ok(())
}

#[test]
fn current_bank_membership_and_owner_callbacks_are_not_required_to_inspect_historical_claims()
-> Result<(), ()> {
    let mut original = vector::record();
    let mut target = TargetVector::external();
    let mut preimage = 1u32.to_be_bytes().to_vec();
    frame(FILE.as_bytes(), &mut preimage);
    preimage.extend_from_slice(&43u32.to_be_bytes());
    preimage.extend_from_slice(&7u32.to_be_bytes());
    frame(b"replace true with false", &mut preimage);
    target.identity = vec![0];
    frame(&hash("mutation-target/v1", &preimage), &mut target.identity);
    target.family = vec![1];
    frame(
        b"historical-family-outside-current-bank",
        &mut target.family,
    );
    target.owner = vec![1];
    name((b"historical-owner", b"claim"), &mut target.owner);
    original.target = target.body();
    let mut vector = BackendVector::declared(Material::Complete);
    vector.run = vector::run(&[original.encoded()]);
    let record = read_backend(&vector.encoded(), LIMITS).map_err(|_| ())?;
    let [mutation] = record.run().reports() else {
        return Err(());
    };
    assert_eq!(
        mutation.target().family(),
        Some("historical-family-outside-current-bank")
    );
    let owner = mutation.target().owner().ok_or(())?;
    assert_eq!(owner.namespace(), "historical-owner");
    assert_eq!(owner.stem(), "claim");
    Ok(())
}

#[test]
fn empty_target_labels_and_unstated_announcements_remain_lawful_historical_metadata()
-> Result<(), ()> {
    let mut vector = BackendVector::declared(Material::Absent);
    vector.invocation = vec![0];
    frame(b"27-test", &mut vector.invocation);
    frame(b"cargo", &mut vector.invocation);
    vector.invocation.extend_from_slice(&0u64.to_be_bytes());
    frame(b"", &mut vector.invocation);
    frame(b"", &mut vector.invocation);
    vector.announced = vec![0];
    let record = read_backend(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert!(record.invocation().arguments().is_empty());
    assert_eq!(record.invocation().target().target().spelling(), "");
    assert_eq!(record.invocation().target().toolchain().spelling(), "");
    assert_eq!(
        record.announced(),
        macroonz_harness::muterprater::AnnouncedRoster::Unstated
    );
    Ok(())
}

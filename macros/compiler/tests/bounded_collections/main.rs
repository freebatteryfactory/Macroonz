//! Cardinality-bearing compiler collections observed through their public roads.
//!
//! The lane asks each constructor at, below, and beyond its ceiling, then pairs every refusal with a lawful control.
//! It also proves that retained order, capping posture, and error chaining remain readable without reaching any private field.

mod assignment;

use core::error::Error;
use macroonz_compiler::bounded::{
    Bounded as HomeBounded, Capped as HomeCapped, Capping as HomeCapping,
    DuplicateKey as HomeDuplicateKey, Empty as HomeEmpty, KeyedRoster as HomeKeyedRoster,
    KeyedRosterError as HomeKeyedRosterError, NonEmpty as HomeNonEmpty,
    NonEmptyError as HomeNonEmptyError, Overflow as HomeOverflow,
};
use macroonz_compiler::{
    Bounded, Capped, Capping, DuplicateKey, Empty, KeyedRoster, KeyedRosterError, NonEmpty,
    NonEmptyError, Overflow,
};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

static SCRATCH_ORDINAL: AtomicU32 = AtomicU32::new(0);

fn scratch_path() -> PathBuf {
    let ordinal = SCRATCH_ORDINAL.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "macroonz_bounded_refusal_{}_{ordinal}",
        std::process::id()
    ))
}

fn compiled(source: &str) -> Result<Output, String> {
    let scratch = scratch_path();
    let source_dir = scratch.join("src");
    std::fs::create_dir_all(&source_dir).map_err(|error| error.to_string())?;
    let dependency = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    let manifest = format!(
        "[package]\nname = \"bounded-refusal\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\nmacroonz-compiler = {{ path = \"{dependency}\" }}\n\n[workspace]\n"
    );
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    std::fs::write(source_dir.join("main.rs"), source).map_err(|error| error.to_string())?;
    let output = Command::new("rustup")
        .arg("run")
        .arg("1.98.0")
        .arg("cargo")
        .arg("build")
        .arg("--offline")
        .arg("--manifest-path")
        .arg(scratch.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .output()
        .map_err(|error| error.to_string());
    drop(std::fs::remove_dir_all(&scratch));
    output
}

fn build_refuses(source: &str, sentence: &str) -> Result<(), String> {
    let output = compiled(source)?;
    if output.status.success() {
        return Err("the structurally invalid collection compiled".to_owned());
    }
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    if !diagnostic.contains(sentence) {
        return Err(diagnostic.into_owned());
    }
    Ok(())
}

/// An empty bounded collection is lawful even when its ceiling admits nothing.
#[test]
fn bounded_empty_is_lawful_at_a_zero_ceiling() {
    let held = Bounded::<u8, 0>::empty();
    assert!(held.is_empty());
    assert_eq!(held.len(), 0);
    assert_eq!(held.as_slice(), &[]);
    assert_eq!(held.iter().copied().collect::<Vec<_>>(), Vec::<u8>::new());
}

/// Complete offerings at and below the ceiling retain their exact order, while one beyond it refuses with both magnitudes.
#[test]
fn bounded_construction_retains_order_and_names_overflow() -> Result<(), Overflow> {
    let below = Bounded::<u8, 3>::new(vec![3, 1])?;
    let exact = Bounded::<u8, 3>::new(vec![3, 1, 2])?;
    let refused = Bounded::<u8, 3>::new(vec![3, 1, 2, 4]).err();

    assert_eq!(below.as_slice(), &[3, 1]);
    assert_eq!(exact.iter().copied().collect::<Vec<_>>(), vec![3, 1, 2]);
    assert_eq!(
        refused,
        Some(Overflow {
            capacity: 3,
            offered: 4,
        })
    );
    Ok(())
}

/// Fixed offerings keep their order, and checked growth refuses before changing a full collection.
#[test]
fn fixed_and_incremental_bounded_roads_preserve_the_held_sequence() -> Result<(), Overflow> {
    let fixed = Bounded::<u8, 3>::from_array([7, 8, 9]);
    assert_eq!(fixed.as_slice(), &[7, 8, 9]);

    let mut growing = Bounded::<u8, 2>::empty();
    growing.try_push(4)?;
    growing.try_push(5)?;
    let before = growing.as_slice().to_vec();
    let refused = growing.try_push(6);

    assert_eq!(
        refused,
        Err(Overflow {
            capacity: 2,
            offered: 3,
        })
    );
    assert_eq!(growing.as_slice(), before.as_slice());
    Ok(())
}

/// Fixed and capped constructors refuse structurally impossible const-generic combinations during a real build.
///
/// Trybuild exercises `cargo check`, which does not monomorphize these non-const collection constructors and therefore cannot observe their const-block assertions.
#[test]
fn impossible_const_generic_collections_refuse_during_codegen() -> Result<(), String> {
    build_refuses(
        include_str!("build-fail/a-fixed-bounded-offering-cannot-exceed-its-ceiling.rs"),
        "a fixed list longer than the ceiling it is declared under",
    )?;
    build_refuses(
        include_str!("build-fail/a-capped-collection-cannot-have-a-zero-ceiling.rs"),
        "a capped list under a ceiling that admits no item",
    )
}

/// Non-empty construction gives a total first item and separates absence from overflow.
#[test]
fn nonempty_construction_separates_absence_from_overflow() -> Result<(), NonEmptyError> {
    let one = NonEmpty::<u8, 3>::one(5);
    let many = NonEmpty::<u8, 3>::new(vec![5, 6, 7])?;
    let absent = NonEmpty::<u8, 3>::new(Vec::new()).err();
    let overflowing = NonEmpty::<u8, 3>::new(vec![5, 6, 7, 8]).err();

    assert_eq!(one.first(), &5);
    assert_eq!(one.count(), 1);
    assert_eq!(many.first(), &5);
    assert_eq!(many.split(), (&5, [6, 7].as_slice()));
    assert_eq!(many.iter().copied().collect::<Vec<_>>(), vec![5, 6, 7]);
    assert_eq!(
        (&many).into_iter().copied().collect::<Vec<_>>(),
        vec![5, 6, 7]
    );
    assert_eq!(absent, Some(NonEmptyError::Empty(Empty)));
    assert_eq!(
        overflowing,
        Some(NonEmptyError::Overflow(Overflow {
            capacity: 3,
            offered: 4,
        }))
    );
    Ok(())
}

/// A zero ceiling still distinguishes an absent offering from a present offering that cannot fit.
#[test]
fn zero_ceiling_nonempty_refusals_keep_their_distinct_causes() {
    assert_eq!(
        NonEmpty::<u8, 0>::new(Vec::new()),
        Err(NonEmptyError::Empty(Empty))
    );
    assert_eq!(
        NonEmpty::<u8, 0>::new(vec![1]),
        Err(NonEmptyError::Overflow(Overflow {
            capacity: 0,
            offered: 1,
        }))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Member {
    name: String,
    value: u8,
}

fn member(name: &str, value: u8) -> Member {
    Member {
        name: name.to_owned(),
        value,
    }
}

/// A lawful caller-keyed roster retains declaration order and one key per member for every read road.
#[test]
fn keyed_roster_retains_order_and_supports_checked_reads() -> Result<(), KeyedRosterError<String, 3>>
{
    let roster = KeyedRoster::<Member, String, 3>::new(
        vec![member("beta", 2), member("alpha", 1), member("gamma", 3)],
        |held| held.name.clone(),
    )?;

    assert_eq!(roster.first(), &member("beta", 2));
    assert_eq!(roster.first_key(), "beta");
    assert_eq!(roster.count(), 3);
    assert_eq!(
        roster.members().map(|held| held.value).collect::<Vec<_>>(),
        vec![2, 1, 3]
    );
    assert_eq!(
        roster.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["beta", "alpha", "gamma"]
    );
    assert_eq!(
        roster
            .indexed()
            .map(|(index, key, held)| (index, key.as_str(), held.value))
            .collect::<Vec<_>>(),
        vec![(0, "beta", 2), (1, "alpha", 1), (2, "gamma", 3)]
    );
    assert_eq!(
        roster.at(1).map(|(key, held)| (key.as_str(), held.value)),
        Some(("alpha", 1))
    );
    assert_eq!(roster.at(3), None);
    assert_eq!(roster.index_of("gamma"), Some(2));
    assert_eq!(roster.index_of("foreign"), None);
    assert_eq!(roster.get("alpha").map(|held| held.value), Some(1));
    assert_eq!(roster.get("foreign"), None);
    Ok(())
}

/// One-member sugar establishes the same total first-member and borrowed-key roads without an allocation-shaped declaration.
#[test]
fn keyed_roster_one_is_zero_ceremony() {
    let roster = KeyedRoster::<Member, String, 1>::one(member("sole", 7), "sole".to_owned());

    assert_eq!(roster.first().value, 7);
    assert_eq!(roster.first_key(), "sole");
    assert_eq!(roster.get("sole").map(|held| held.value), Some(7));
    assert_eq!(roster.indexed().count(), 1);
}

/// Empty and overflowing offerings refuse before the caller's key projection can run, while an exact-bound offering projects each key once.
#[test]
fn keyed_roster_settles_magnitude_before_key_work() -> Result<(), KeyedRosterError<String, 2>> {
    let projected = AtomicUsize::new(0);
    let empty = KeyedRoster::<Member, String, 2>::new(Vec::new(), |held| {
        projected.fetch_add(1, Ordering::SeqCst);
        held.name.clone()
    });
    let overflow = KeyedRoster::<Member, String, 2>::new(
        vec![member("a", 1), member("b", 2), member("c", 3)],
        |held| {
            projected.fetch_add(1, Ordering::SeqCst);
            held.name.clone()
        },
    );

    assert_eq!(empty, Err(KeyedRosterError::Empty(Empty)));
    assert_eq!(
        overflow,
        Err(KeyedRosterError::Overflow(Overflow {
            capacity: 2,
            offered: 3,
        }))
    );
    assert_eq!(projected.load(Ordering::SeqCst), 0);

    let exact =
        KeyedRoster::<Member, String, 2>::new(vec![member("a", 1), member("b", 2)], |held| {
            projected.fetch_add(1, Ordering::SeqCst);
            held.name.clone()
        })?;
    assert_eq!(exact.count(), 2);
    assert_eq!(projected.load(Ordering::SeqCst), 2);
    Ok(())
}

/// Duplicate admission refuses rather than silently deduplicating and reports every distinct key once with all declaration positions.
#[test]
fn keyed_roster_reports_every_duplicate_key_once() -> Result<(), String> {
    let projected = AtomicUsize::new(0);
    let result = KeyedRoster::<Member, String, 8>::new(
        vec![
            member("unique", 0),
            member("a", 1),
            member("b", 2),
            member("a", 3),
            member("b", 4),
            member("b", 5),
            member("c", 6),
            member("c", 7),
        ],
        |held| {
            projected.fetch_add(1, Ordering::SeqCst);
            held.name.clone()
        },
    );
    let duplicates = match result {
        Err(KeyedRosterError::DuplicateKeys(duplicates)) => duplicates,
        Err(refusal) => return Err(refusal.to_string()),
        Ok(_) => return Err("the duplicate offering was admitted".to_owned()),
    };
    let observed = duplicates
        .iter()
        .map(|duplicate| {
            (
                duplicate.key().as_str(),
                duplicate.first_position(),
                duplicate
                    .repeated_positions()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        observed,
        vec![("a", 1, vec![3]), ("b", 2, vec![4, 5]), ("c", 6, vec![7]),]
    );
    assert_eq!(projected.load(Ordering::SeqCst), 8);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FiniteMember {
    ordinal: usize,
    key: u8,
}

fn finite_keys(mut encoded: usize, length: usize) -> Vec<u8> {
    let mut keys = Vec::with_capacity(length);
    for _ in 0..length {
        keys.push(match encoded % 3 {
            0 => 0,
            1 => 1,
            _ => 2,
        });
        encoded /= 3;
    }
    keys
}

fn expected_finite_duplicates(keys: &[u8]) -> Vec<(u8, usize, Vec<usize>)> {
    let mut expected = (0_u8..3)
        .filter_map(|key| {
            let mut positions = keys
                .iter()
                .enumerate()
                .filter_map(|(index, observed)| (*observed == key).then_some(index));
            let first = positions.next()?;
            let repeated = positions.collect::<Vec<_>>();
            (!repeated.is_empty()).then_some((key, first, repeated))
        })
        .collect::<Vec<_>>();
    expected.sort_by_key(|(_, first, _)| *first);
    expected
}

fn finite_roster(keys: &[u8]) -> Result<KeyedRoster<FiniteMember, u8, 4>, KeyedRosterError<u8, 4>> {
    let members = keys
        .iter()
        .copied()
        .enumerate()
        .map(|(ordinal, key)| FiniteMember { ordinal, key })
        .collect::<Vec<_>>();
    KeyedRoster::new(members, |held| held.key)
}

fn verify_lawful_finite_sequence(keys: &[u8]) -> Result<(), String> {
    let roster = finite_roster(keys)
        .map_err(|refusal| format!("lawful finite sequence {keys:?} refused: {refusal}"))?;
    let expected_members = keys
        .iter()
        .copied()
        .enumerate()
        .map(|(ordinal, key)| FiniteMember { ordinal, key })
        .collect::<Vec<_>>();

    assert_eq!(
        roster.members().copied().collect::<Vec<_>>(),
        expected_members
    );
    assert_eq!(roster.keys().copied().collect::<Vec<_>>(), keys);
    assert_eq!(
        roster
            .indexed()
            .map(|(index, key, held)| (index, *key, held.ordinal))
            .collect::<Vec<_>>(),
        expected_members
            .iter()
            .map(|held| (held.ordinal, held.key, held.ordinal))
            .collect::<Vec<_>>()
    );
    Ok(())
}

fn verify_duplicate_finite_sequence(
    keys: &[u8],
    expected: &[(u8, usize, Vec<usize>)],
) -> Result<(), String> {
    let duplicates = match finite_roster(keys) {
        Err(KeyedRosterError::DuplicateKeys(duplicates)) => duplicates,
        Err(refusal) => {
            return Err(format!(
                "duplicate finite sequence {keys:?} reached the wrong refusal: {refusal}"
            ));
        }
        Ok(_) => {
            return Err(format!(
                "duplicate finite sequence {keys:?} was silently admitted"
            ));
        }
    };
    let observed = duplicates
        .iter()
        .map(|duplicate| {
            (
                *duplicate.key(),
                duplicate.first_position(),
                duplicate
                    .repeated_positions()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(observed, expected, "finite sequence {keys:?}");
    Ok(())
}

/// Every nonempty key sequence through the small finite ceiling either remains ordered and admitted or reports the complete ordered duplicate partition.
#[test]
fn keyed_roster_matches_an_exhaustive_finite_duplicate_oracle() -> Result<(), String> {
    for length in 1_usize..=4 {
        let sequence_count = (0..length).fold(1_usize, |count, _| count.saturating_mul(3));
        for encoded in 0..sequence_count {
            let keys = finite_keys(encoded, length);
            let expected_duplicates = expected_finite_duplicates(&keys);
            if expected_duplicates.is_empty() {
                verify_lawful_finite_sequence(&keys)?;
            } else {
                verify_duplicate_finite_sequence(&keys, &expected_duplicates)?;
            }
        }
    }
    Ok(())
}

/// Complete and truncated cappings retain the same ordered-prefix grammar and differ only in their truthful posture.
#[test]
fn capped_collections_record_the_exact_omitted_magnitude() {
    let complete = Capped::<u8, 3>::all(NonEmpty::one(9));
    let exact = Capped::<u8, 3>::first_n(9, [8, 7].into_iter());
    let truncated = Capped::<u8, 3>::first_n(9, [8, 7, 6, 5].into_iter());

    assert_eq!(
        complete.items().iter().copied().collect::<Vec<_>>(),
        vec![9]
    );
    assert_eq!(complete.capping(), Capping::Complete);
    assert_eq!(
        exact.items().iter().copied().collect::<Vec<_>>(),
        vec![9, 8, 7]
    );
    assert_eq!(exact.capping(), Capping::Complete);
    assert_eq!(
        truncated.items().iter().copied().collect::<Vec<_>>(),
        vec![9, 8, 7]
    );
    assert_eq!(truncated.capping(), Capping::Truncated { omitted: 2 });
}

/// Refusal conversions preserve the concrete cause, exact sentence, and standard error source.
#[test]
fn refusal_trait_contracts_preserve_the_concrete_cause() -> Result<(), String> {
    let empty = NonEmptyError::from(Empty);
    let overflow = NonEmptyError::from(Overflow {
        capacity: 2,
        offered: 3,
    });

    assert_eq!(
        empty.to_string(),
        "no item offered where at least one is required"
    );
    assert_eq!(overflow.to_string(), "3 items offered where at most 2 fit");
    assert!(empty.source().is_some_and(<dyn Error>::is::<Empty>));
    assert!(overflow.source().is_some_and(<dyn Error>::is::<Overflow>));

    let roster_empty = KeyedRosterError::<String, 2>::Empty(Empty);
    let roster_overflow = KeyedRosterError::<String, 2>::Overflow(Overflow {
        capacity: 2,
        offered: 3,
    });
    let roster_duplicate = KeyedRoster::<String, String, 2>::new(
        vec!["same".to_owned(), "same".to_owned()],
        Clone::clone,
    )
    .err()
    .ok_or_else(|| "the duplicate roster was admitted".to_owned())?;
    let roster_duplicates = KeyedRoster::<String, String, 4>::new(
        vec![
            "left".to_owned(),
            "left".to_owned(),
            "right".to_owned(),
            "right".to_owned(),
        ],
        Clone::clone,
    )
    .err()
    .ok_or_else(|| "the multiply duplicated roster was admitted".to_owned())?;

    assert_eq!(roster_empty.to_string(), empty.to_string());
    assert_eq!(roster_overflow.to_string(), overflow.to_string());
    assert_eq!(
        roster_duplicate.to_string(),
        "one caller-declared key occurred more than once"
    );
    assert_eq!(
        roster_duplicates.to_string(),
        "2 caller-declared keys occurred more than once"
    );
    assert!(roster_empty.source().is_some_and(<dyn Error>::is::<Empty>));
    assert!(
        roster_overflow
            .source()
            .is_some_and(<dyn Error>::is::<Overflow>)
    );
    assert!(roster_duplicate.source().is_none());
    Ok(())
}

/// The module path and crate-root compatibility path name the same complete public vocabulary.
#[test]
fn both_public_navigation_roads_name_one_vocabulary() {
    let through_root: Bounded<u8, 1> = Bounded::from_array([1]);
    let through_home: HomeBounded<u8, 1> = through_root;
    let through_root_again: Bounded<u8, 1> = through_home;
    let nonempty: HomeNonEmpty<u8, 1> = NonEmpty::one(1);
    let capped: HomeCapped<u8, 1> = Capped::all(nonempty);
    let capping: HomeCapping = Capping::Complete;
    let empty: HomeEmpty = Empty;
    let overflow: HomeOverflow = Overflow {
        capacity: 0,
        offered: 1,
    };
    let refusal: HomeNonEmptyError = NonEmptyError::from(overflow);
    let roster: HomeKeyedRoster<u8, &'static str, 1> = KeyedRoster::one(4, "four");
    let root_roster: KeyedRoster<u8, &'static str, 1> = roster;
    let duplicate: Option<HomeDuplicateKey<&'static str, 1>> = None;
    let roster_refusal: Option<HomeKeyedRosterError<&'static str, 1>> = None;

    assert_eq!(through_root_again.as_slice(), &[1]);
    assert_eq!(capped.capping(), capping);
    assert_eq!(empty, Empty);
    assert_eq!(refusal, NonEmptyError::Overflow(overflow));
    assert_eq!(root_roster.get("four"), Some(&4));
    assert_eq!(duplicate, Option::<DuplicateKey<&'static str, 1>>::None);
    assert_eq!(
        roster_refusal,
        Option::<KeyedRosterError<&'static str, 1>>::None
    );
}

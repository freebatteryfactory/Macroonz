//! Cardinality-bearing compiler collections observed through their public roads.
//!
//! The lane asks each constructor at, below, and beyond its ceiling, then pairs every refusal with a lawful control.
//! It also proves that retained order, capping posture, and error chaining remain readable without reaching any private field.

use core::error::Error;
use macroonz_compiler::bounded::{
    Bounded as HomeBounded, Capped as HomeCapped, Capping as HomeCapping, Empty as HomeEmpty,
    NonEmpty as HomeNonEmpty, NonEmptyError as HomeNonEmptyError, Overflow as HomeOverflow,
};
use macroonz_compiler::{Bounded, Capped, Capping, Empty, NonEmpty, NonEmptyError, Overflow};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

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
fn refusal_trait_contracts_preserve_the_concrete_cause() {
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

    assert_eq!(through_root_again.as_slice(), &[1]);
    assert_eq!(capped.capping(), capping);
    assert_eq!(empty, Empty);
    assert_eq!(refusal, NonEmptyError::Overflow(overflow));
}

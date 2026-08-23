//! The trial-declaration lane: what a declaration's trial rows are named under,
//! and what the grammar that reads them refuses.
//!
//! One declaration is read THREE ways — what it IS, what it SAYS, and what it
//! states about a consumer's test target — and the whole point of the third
//! reading is that it moves independently of the first. That is a fact about
//! identities no type can carry, so it is observed here, over the callable
//! services road, with no proc-macro anywhere in the path.
//!
//! # Reversals
//!
//! A lane that only asked "does a declaration with trials capture?" would pass
//! against a reader that folded the trial attribute into the semantic commitment
//! and against one that dropped it entirely. So the separations are REQUIRED: two
//! declarations differing only in their trial rows must agree on the name their
//! implementation projection is about and disagree on the name their carrier is
//! about, and a declaration that states no trials must reach neither.
//!
//! The refusals are required to be PRECISE for the same reason. A grammar that
//! answered every malformed clause with one cause would pass a lane that only
//! checked that it refused; each seat below names the cause it establishes and
//! the home that established it.

use threadpak_macroc::derive_refusal::{SurfaceCaptureRefusal, TrialDeclarationPosture};
use threadpak_macroc::plane::{CapturedDeclarationSubject, ProjectionIdentity};
use threadpak_macroc::test_descriptor::{TrialDeclarationCause, TrialDeclarationRefusal};
use threadpak_macroc::{captured_text, compile_refusal_text};

/// The independent semantic, documentation, and trial names read from one capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Readings {
    /// What the declaration IS.
    semantic: ProjectionIdentity<CapturedDeclarationSubject>,
    /// What it SAYS.
    documentation: ProjectionIdentity<CapturedDeclarationSubject>,
    /// The trial declaration's independent commitment, where one was declared.
    trials: Option<ProjectionIdentity<CapturedDeclarationSubject>>,
}

/// The three names one source captures to, or nothing where it refused.
fn readings(source: &str) -> Option<Readings> {
    let (_, surface) = captured_text(source).ok()?;
    let trials = match surface.trials() {
        TrialDeclarationPosture::NotDeclared => None,
        TrialDeclarationPosture::Declared(declared) => Some(declared.commitment()),
    };
    Some(Readings {
        semantic: surface.identity(),
        documentation: surface.documentation_identity(),
        trials,
    })
}

/// The declaration every reading below stands over, with one clause of its trial
/// attribute left open for a seat to fill.
///
/// Written as one shape so the differences between the readings are exactly the
/// differences a seat states, rather than differences between two texts that also
/// happen to disagree about something else.
fn declaration(trials: &str) -> String {
    format!(
        "#[refusal(family = \"fixture.demo\", shape = single_cause, \
         order(NotCanonical = \"not-canonical\"))]\n{trials}\nenum DemoFamily {{ NotCanonical }}"
    )
}

/// The trial attribute one lawful declaration carries.
fn lawful_trials(claim: &str) -> String {
    format!(
        "#[threadpak_trials(support = demo_trials, module = generated_demo_trials, \
         table = named(\"fixture\", \"demo-trials\"), \
         suite construction = named(\"fixture\", \"construction\") {{ \
         a_demo_row {{ claim = named(\"fixture\", \"{claim}\"), \
         subject = named(\"fixture\", \"demo-subject\"), \
         check = named(\"fixture\", \"demo-check\"), \
         population = named(\"fixture\", \"demo-population\"), }}, }},)]"
    )
}

/// The refusal one source establishes, or nothing where it captured.
fn refused(source: &str) -> Option<SurfaceCaptureRefusal> {
    captured_text(source).err()
}

/// The grammar cause one source establishes, or nothing where it captured or
/// refused in another home.
fn grammar_cause(source: &str) -> Option<TrialDeclarationCause> {
    match refused(source)? {
        SurfaceCaptureRefusal::Trials(TrialDeclarationRefusal::Grammar { cause, .. }) => {
            Some(cause)
        }
        SurfaceCaptureRefusal::Trials(TrialDeclarationRefusal::Carrier { .. })
        | SurfaceCaptureRefusal::Declaration(_)
        | SurfaceCaptureRefusal::Mutations(_) => None,
    }
}

/// A declaration's trial rows are named under their own commitment, and the
/// implementation projection's name does not move when they do.
///
/// This is the whole reason the third reading exists. Two declarations that
/// differ only in a trial row are the SAME contract exercised differently: the
/// name an implementation projection stands on is the same, and the name the
/// CARRIER stands on is not.
#[test]
fn a_trial_edit_moves_the_carrier_and_leaves_the_implementation() {
    let one = readings(&declaration(&lawful_trials("a-demo-claim")));
    let other = readings(&declaration(&lawful_trials("another-demo-claim")));
    assert!(one.is_some() && other.is_some());
    assert_eq!(
        one.map(|read| read.semantic),
        other.map(|read| read.semantic)
    );
    // Two absent readings would compare EQUAL here, so this seat is what keeps
    // the pair above from passing vacuously as well as what states the move.
    assert_ne!(
        one.and_then(|read| read.trials),
        other.and_then(|read| read.trials)
    );
}

/// A declaration that states no trials carries no trial commitment, while one that states trials carries the commitment read from those rows.
#[test]
fn a_declaration_with_no_trials_has_no_trial_commitment() {
    let bare = captured_text(&declaration("")).ok();
    assert!(bare.as_ref().is_some_and(|(_, surface)| matches!(
        surface.trials(),
        TrialDeclarationPosture::NotDeclared
    )));
    assert!(readings(&declaration("")).is_some_and(|read| read.trials.is_none()));

    let stated = declaration(&lawful_trials("a-demo-claim"));
    let carried = captured_text(&stated).ok();
    assert!(carried.as_ref().is_some_and(|(_, surface)| matches!(
        surface.trials(),
        TrialDeclarationPosture::Declared(_)
    )));
    assert!(readings(&stated).is_some_and(|read| read.trials.is_some()));
}

/// A trial edit leaves the implementation projection's rendered OUTPUT
/// byte-for-byte where it was.
///
/// The identity seat above says the two declarations plan the same
/// implementation. This one says the two declarations EMIT the same
/// implementation, which is the fact a consumer's normal build actually
/// receives — and the two are different claims: a name that agreed while the
/// bytes moved would be a name that had stopped tracking what it named.
///
/// The comparison is over the declaration-site partition, which is exactly the
/// cargo an ordinary build compiles. What the two declarations DO differ in is
/// the carrier, and the seat above states that.
#[test]
fn a_trial_edit_leaves_the_declaration_sites_own_bytes() {
    let one = compile_refusal_text(&declaration(&lawful_trials("a-demo-claim"))).ok();
    let other = compile_refusal_text(&declaration(&lawful_trials("another-demo-claim"))).ok();
    let emitted = |closed: &Option<(_, threadpak_macroc::RefusalFamilyExpansion)>| {
        closed
            .as_ref()
            .and_then(|(_, expansion)| expansion.inspected())
    };
    let first = emitted(&one);
    let second = emitted(&other);
    assert!(first.is_some());
    assert_eq!(first, second);
}

/// The prose reading and the trial reading are three separate names over one
/// surface, and none of them is either of the others.
///
/// The semantic commitment sets both aside, so a reworded sentence and an edited
/// trial row each move exactly one name — and the two moved names are not the
/// same name either.
#[test]
fn three_readings_of_one_declaration_reach_three_names() {
    let read = readings(&declaration(&lawful_trials("a-demo-claim")));
    assert!(read.is_some_and(|read| {
        read.trials.is_some_and(|trials| {
            read.semantic != read.documentation
                && read.semantic != trials
                && read.documentation != trials
        })
    }));
}

/// The seats a producer performs and the seats a consumption target supplies
/// have no clause in this grammar.
///
/// Six keys, one cause. Every one of them is a real seat of the road a generated
/// row travels — an origin the producer mints, a schema it pins against, a
/// revision and a callable the test target holds, a budget and a clock that
/// target declares — and each of them reaching the SAME cause is the statement:
/// a trial declaration states descriptor meaning, and the wall is where the rest
/// of it lives.
#[test]
fn no_producer_fact_and_no_host_fact_has_a_clause() {
    for reached_for in [
        "origin = named(\"fixture\", \"generated\")",
        "schema = named(\"fixture\", \"demo\")",
        "subject_revision = named(\"fixture\", \"r1\")",
        "call = named(\"fixture\", \"demo-check\")",
        "invocation = named(\"fixture\", \"budget\")",
        "clock = named(\"fixture\", \"unmeasured\")",
    ] {
        let trials = format!(
            "#[threadpak_trials(support = demo_trials, module = generated_demo_trials, \
             table = named(\"fixture\", \"demo-trials\"), \
             suite construction = named(\"fixture\", \"construction\") {{ \
             a_demo_row {{ claim = named(\"fixture\", \"a-demo-claim\"), \
             subject = named(\"fixture\", \"demo-subject\"), \
             check = named(\"fixture\", \"demo-check\"), \
             population = named(\"fixture\", \"demo-population\"), {reached_for}, }}, }},)]"
        );
        assert_eq!(
            grammar_cause(&declaration(&trials)),
            Some(TrialDeclarationCause::NotADeclarableClause),
            "reaching for `{reached_for}` did not refuse as an undeclarable clause"
        );
    }
}

/// One declaration carries the trial attribute once.
///
/// Two of them are two declarations of one carrier's rows standing beside each
/// other, and neither is the one — so the reading refuses rather than electing
/// whichever it read last.
#[test]
fn a_second_trial_attribute_refuses() {
    let doubled = format!(
        "{}\n{}",
        lawful_trials("a-demo-claim"),
        lawful_trials("another-demo-claim")
    );
    assert_eq!(
        grammar_cause(&declaration(&doubled)),
        Some(TrialDeclarationCause::NotDeclaredOnce)
    );
}

/// A required clause the declaration does not state refuses under its own cause.
#[test]
fn a_required_clause_that_is_absent_refuses() {
    let short = "#[threadpak_trials(module = generated_demo_trials, \
                 table = named(\"fixture\", \"demo-trials\"), \
                 suite construction = named(\"fixture\", \"construction\") { \
                 a_demo_row { claim = named(\"fixture\", \"a-demo-claim\"), \
                 subject = named(\"fixture\", \"demo-subject\"), \
                 check = named(\"fixture\", \"demo-check\"), \
                 population = named(\"fixture\", \"demo-population\"), }, },)]";
    assert_eq!(
        grammar_cause(&declaration(short)),
        Some(TrialDeclarationCause::NotCovered)
    );
}

/// A value written where a namespaced reference is required refuses under its
/// own cause rather than under the clause's.
///
/// The two are different repairs: a clause nobody declared is a key to delete,
/// and a value the reader cannot read is a value to rewrite.
#[test]
fn a_value_that_is_not_a_namespaced_reference_refuses() {
    let malformed = "#[threadpak_trials(support = demo_trials, module = generated_demo_trials, \
                     table = \"demo-trials\", \
                     suite construction = named(\"fixture\", \"construction\") { \
                     a_demo_row { claim = named(\"fixture\", \"a-demo-claim\"), \
                     subject = named(\"fixture\", \"demo-subject\"), \
                     check = named(\"fixture\", \"demo-check\"), \
                     population = named(\"fixture\", \"demo-population\"), }, },)]";
    assert_eq!(
        grammar_cause(&declaration(malformed)),
        Some(TrialDeclarationCause::NotANamedReference)
    );
}

/// A declaration whose values the CARRIER's own vocabulary refuses reaches that
/// home's body, whole, rather than this grammar's.
///
/// Two homes answer at one seam and each answer is carried unchanged: the
/// stamped module puts every seat and every lens in one namespace, and a
/// declaration that would declare one function twice is the carrier's fact rather
/// than the grammar's. A single roster covering both would give a malformed
/// clause and a doubled lens one shape.
#[test]
fn a_doubled_lens_refuses_in_the_carriers_own_family() {
    let doubled = "#[threadpak_trials(support = demo_trials, module = generated_demo_trials, \
                   table = named(\"fixture\", \"demo-trials\"), \
                   suite construction = named(\"fixture\", \"construction\") { \
                   a_demo_row { claim = named(\"fixture\", \"one\"), \
                   subject = named(\"fixture\", \"demo-subject\"), \
                   check = named(\"fixture\", \"demo-check\"), \
                   population = named(\"fixture\", \"demo-population\"), }, \
                   a_demo_row { claim = named(\"fixture\", \"two\"), \
                   subject = named(\"fixture\", \"demo-subject\"), \
                   check = named(\"fixture\", \"demo-check\"), \
                   population = named(\"fixture\", \"demo-population\"), }, },)]";
    let refusal = refused(&declaration(doubled));
    assert!(matches!(
        refusal,
        Some(SurfaceCaptureRefusal::Trials(
            TrialDeclarationRefusal::Carrier {
                refusal:
                    threadpak_macroc::test_descriptor::ShellDeclarationRefusal::LensSpellingDoubled,
                ..
            }
        ))
    ));
}

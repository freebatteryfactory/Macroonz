//! Every question a kind owes, answered exactly once, over the plan and the proof the answers are ABOUT.
//!
//! A complete view is the coverage proof itself, so there is no partial one to mistake for a whole one: a set of answers that could not be completed is a refusal instead.
//! This lane declares a kind with a question of its own, so both rosters — the compiler's nine and the kind's one — are under judgement rather than only the half every kind shares.
//!
//! # Reversals
//!
//! A protocol that accepted any answer sheet would satisfy every positive assertion here.
//! So each is paired with the sheet that must refuse: a universal question unanswered, one answered twice, a declared question unanswered, one answered twice, and an answer naming a question its own roster does not carry.

use macroonz::{
    Answer, Bounded, CrateBinding, Diagnostic, Door, Expansion, ExplanationIssue, GeneratedToken,
    GeneratedTree, Kind, Phase, Producer, Question, Request, SoleRole, TextCapture,
    UNIVERSAL_QUESTION_COUNT, UniversalAnswer, UniversalQuestion, View, encode_bytes,
};

/// The kind this lane explains: one rendered unit, and one question of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Greeting;

impl Kind for Greeting {
    const NAME: &'static str = "lane.greeting";
    type Content = &'static str;
    type Role = SoleRole;
    type Question = Asked;
}

/// The questions this kind's roster carries, and one row it does not.
///
/// The second row is how an answer standing outside its own roster is written down at all: a roster of one with a second spelling nobody declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Asked {
    /// Whom does this greeting greet?
    Whom,
    /// A question this kind's roster does not carry.
    Unlisted,
}

impl Question for Asked {
    const ALL: &'static [Self] = &[Self::Whom];

    type Answer = Answered;

    fn name(self) -> &'static str {
        match self {
            Self::Whom => "whom",
            Self::Unlisted => "unlisted",
        }
    }
}

/// The typed answers this kind's questions take.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Answered {
    /// Whom the greeting greets.
    Whom(&'static str),
    /// An answer to the question the roster does not carry.
    Unlisted,
}

impl Answer for Answered {
    type Question = Asked;

    fn question(&self) -> Asked {
        match self {
            Self::Whom(_) => Asked::Whom,
            Self::Unlisted => Asked::Unlisted,
        }
    }

    fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::Whom(whom) => {
                into.push(0);
                encode_bytes(whom.as_bytes(), into);
            }
            Self::Unlisted => {
                into.push(1);
                encode_bytes(&[], into);
            }
        }
    }

    fn human(&self) -> String {
        match self {
            Self::Whom(whom) => format!("it greets {whom}"),
            Self::Unlisted => String::from("it answers a question nobody asked"),
        }
    }
}

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.greeting.grammar",
    "lane::greeting",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "greeting",
    },
);

/// One declared input this lane hands the compiler.
const DECLARATION: &str = "struct Greeting { line: Line }";

/// What one request answering these questions produces, or nothing where the declaration itself could not be read.
fn expansion(answers: Vec<Answered>) -> Option<Result<Expansion<Greeting>, Diagnostic>> {
    let read = TextCapture::read(DECLARATION).ok()?;
    Some(
        Request::<Greeting>::over(read.input().clone(), "greeting", &DOOR)
            .answering(answers)
            .render(|_plan, out| {
                out.unit(
                    SoleRole::Sole,
                    GeneratedTree::assembled(vec![GeneratedToken::word("greeting")])?,
                )
            }),
    )
}

/// The lawful expansion every hostile answer sheet below is compared against.
fn lawful() -> Option<Expansion<Greeting>> {
    expansion(vec![Answered::Whom("world")])?.ok()
}

/// Every question the kind owes is answered exactly once, across both rosters.
///
/// Load-bearing in its own right: a protocol that refused everything would satisfy every reversal below and be worthless.
#[test]
fn every_question_a_kind_owes_is_answered_exactly_once() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let view = bound.explain();
    assert_eq!(view.universal().len(), UNIVERSAL_QUESTION_COUNT);
    assert_eq!(view.declared().len(), Asked::ALL.len());
    assert_eq!(
        view.seats(),
        UNIVERSAL_QUESTION_COUNT.saturating_add(Asked::ALL.len())
    );
    for question in <UniversalQuestion as Question>::ALL {
        let answered = view
            .universal()
            .iter()
            .filter(|answer| answer.question() == *question)
            .count();
        assert_eq!(answered, 1);
    }
    assert_eq!(view.plan(), bound.plan().identity());
    assert_eq!(view.closure(), bound.closure().identity());
    Ok(())
}

/// One set of answers is one explanation whichever order it was supplied in.
///
/// The seats stand in their rosters' declared order and never in the caller's, and that order is what the identity is derived over — so a caller cannot rename an explanation by shuffling its answer sheet.
#[test]
fn one_set_of_answers_is_one_explanation_whichever_order_it_arrived_in() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut shuffled = bound.explain().universal().to_vec();
    shuffled.reverse();
    let rewritten = View::complete(
        bound.plan(),
        bound.closure(),
        shuffled,
        vec![Answered::Whom("world")],
    )
    .map_err(|_| ())?;
    assert_eq!(rewritten.identity(), bound.explain().identity());
    Ok(())
}

/// A universal question left unanswered refuses, naming the question.
#[test]
fn a_universal_question_left_unanswered_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut short: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    let dropped = short.remove(0);
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        short,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::UniversalUnanswered {
            question: dropped.question(),
        }
    );
    Ok(())
}

/// An output answer that does not restate the proof's own rendered roster refuses, carrying both counts.
///
/// The lawful rows are derivable from the closure the view is completed over, so a shortened or emptied set cannot ride a coverage-complete view.
#[test]
fn an_output_answer_beside_the_proof_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut shortened: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    for answer in &mut shortened {
        if let UniversalAnswer::OutputAndDigest { outputs } = answer {
            *outputs = Bounded::empty();
        }
    }
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        shortened,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::OutputsBesideTheProof {
            expected: 1u16,
            observed: 0u16,
            diverges: 0u16,
        }
    );
    Ok(())
}

/// An output answer that doubles the proof's one row refuses, and the issue points at the position past the roster's own end.
#[test]
fn a_doubled_output_row_refuses_at_its_own_position() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut doubled: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    for answer in &mut doubled {
        if let UniversalAnswer::OutputAndDigest { outputs } = answer {
            let mut rows = outputs.as_slice().to_vec();
            let Some(first) = rows.first().cloned() else {
                return Err(());
            };
            rows.push(first);
            *outputs = Bounded::new(rows).map_err(|_overflow| ())?;
        }
    }
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        doubled,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::OutputsBesideTheProof {
            expected: 1u16,
            observed: 2u16,
            diverges: 1u16,
        }
    );
    Ok(())
}

/// An output answer restating the right COUNT but another proof's row refuses, and the divergence coordinate names the first foreign row.
///
/// This is the substitution a bare count comparison would wave through: one row offered beside one row proved, and they are not the same row.
#[test]
fn a_foreign_output_row_refuses_at_position_zero() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let read = TextCapture::read(DECLARATION).map_err(|_| ())?;
    let foreign = Request::<Greeting>::over(read.input().clone(), "farewell", &DOOR)
        .answering(vec![Answered::Whom("mars")])
        .render(|_plan, out| {
            out.unit(
                SoleRole::Sole,
                GeneratedTree::assembled(vec![GeneratedToken::word("farewell")])?,
            )
        })
        .map_err(|_| ())?;
    let mut found = None;
    for other in foreign.explain().universal() {
        if let UniversalAnswer::OutputAndDigest { outputs } = other {
            found = Some(outputs.clone());
        }
    }
    let theirs = found.ok_or(())?;
    let mut swapped: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    for answer in &mut swapped {
        if let UniversalAnswer::OutputAndDigest { outputs } = answer {
            *outputs = theirs.clone();
        }
    }
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        swapped,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::OutputsBesideTheProof {
            expected: 1u16,
            observed: 1u16,
            diverges: 0u16,
        }
    );
    Ok(())
}

/// The coverage pass and the output pass co-establish: a sheet failing both is refused once, carrying both findings.
#[test]
fn coverage_and_output_findings_arrive_together() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut hostile: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    hostile.retain(|answer| !matches!(answer, UniversalAnswer::Profile { .. }));
    for answer in &mut hostile {
        if let UniversalAnswer::OutputAndDigest { outputs } = answer {
            *outputs = Bounded::empty();
        }
    }
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        hostile,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(refusal.issues().count(), 2usize);
    assert!(
        refusal
            .issues()
            .iter()
            .any(|issue| matches!(issue, ExplanationIssue::OutputsBesideTheProof { .. }))
    );
    Ok(())
}

/// A universal question answered twice refuses, naming the question.
#[test]
fn a_universal_question_answered_twice_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let mut doubled: Vec<UniversalAnswer> = bound.explain().universal().to_vec();
    let repeated = doubled.first().ok_or(())?.clone();
    let question = repeated.question();
    doubled.push(repeated);
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        doubled,
        vec![Answered::Whom("world")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::UniversalAnsweredTwice { question }
    );
    Ok(())
}

/// The kind's own question left unanswered refuses, naming the question and its position.
///
/// A kind narrows nothing on the universal roster and adds its own beside it, so an unanswered declared seat is reported in the kind's own vocabulary rather than as a shortfall in the compiler's.
#[test]
fn the_kinds_own_question_left_unanswered_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let universal = bound.explain().universal().to_vec();
    let refusal = View::complete(bound.plan(), bound.closure(), universal, Vec::new())
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::DeclaredUnanswered {
            question: "whom",
            slot: Asked::Whom.slot(),
        }
    );
    Ok(())
}

/// The kind's own question answered twice refuses, naming the question and its position.
#[test]
fn the_kinds_own_question_answered_twice_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let universal = bound.explain().universal().to_vec();
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        universal,
        vec![Answered::Whom("world"), Answered::Whom("everyone")],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::DeclaredAnsweredTwice {
            question: "whom",
            slot: Asked::Whom.slot(),
        }
    );
    Ok(())
}

/// An answer naming a question its own roster does not carry refuses.
///
/// The roster is the quantifier both ways: every row of it must be answered, and every answer must name a row of it.
#[test]
fn an_answer_outside_its_own_roster_refuses() -> Result<(), ()> {
    let bound = lawful().ok_or(())?;
    let universal = bound.explain().universal().to_vec();
    let refusal = View::complete(
        bound.plan(),
        bound.closure(),
        universal,
        vec![Answered::Whom("world"), Answered::Unlisted],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ExplanationIssue::QuestionOutsideRoster {
            question: "unlisted",
        }
    );
    Ok(())
}

/// A coverage refusal reaches a caller of the whole road as this door's diagnostic, at the explanation step.
#[test]
fn a_coverage_refusal_reaches_the_caller_as_a_diagnostic() -> Result<(), ()> {
    let refused = expansion(Vec::new()).ok_or(())?.err().ok_or(())?;
    assert_eq!(refused.phase(), Phase::Explanation);
    assert!(refused.summary().starts_with("lane: "));
    assert!(refused.summary().contains("whom"));
    Ok(())
}

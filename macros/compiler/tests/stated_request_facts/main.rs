//! Repeated statements on one request, observed through the plan, trace, and explanation the complete public road returns.
//!
//! Each optional statement is made twice with distinguishable values.
//! The resulting expansion must carry only the second value, proving that replacement is the builder contract rather than an incidental field assignment.

use macroonz_compiler::request::committed;
use macroonz_compiler::{
    Answer, CrateBinding, Destination, Door, GeneratedToken, GeneratedTree, Kind, OwnerFact,
    OwnerIdentity, Producer, Profile, Question, Request, Role, SELECTION_FACT, TextCapture,
    TraceDecision, Version,
};

/// The one publishing seat this lane plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The generated publication.
    Publication,
    /// A second independently selectable publication seat.
    Companion,
    /// A role value outside the declared roster.
    Foreign,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Publication, Self::Companion];

    fn name(self) -> &'static str {
        match self {
            Self::Publication => "publication",
            Self::Companion => "companion",
            Self::Foreign => "foreign",
        }
    }

    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

/// The one question this lane requires the caller to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Asked {
    /// Which statement survived replacement?
    Selected,
}

impl Question for Asked {
    const ALL: &'static [Self] = &[Self::Selected];

    type Answer = Answered;

    fn name(self) -> &'static str {
        "selected"
    }
}

/// The two distinguishable answers offered to the same question.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answered {
    /// The first statement.
    First,
    /// The replacement statement.
    Second,
}

impl Answer for Answered {
    type Question = Asked;

    fn question(&self) -> Asked {
        Asked::Selected
    }

    fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(match self {
            Self::First => 0,
            Self::Second => 1,
        });
    }

    fn human(&self) -> String {
        match self {
            Self::First => "the first statement".to_owned(),
            Self::Second => "the replacement statement".to_owned(),
        }
    }
}

/// The projection kind whose request statements this lane observes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Publication;

impl Kind for Publication {
    const NAME: &'static str = "lane.publication";
    type Content = &'static str;
    type Role = Seat;
    type Question = Asked;
}

/// Who is asking on this external road.
const DOOR: Door = Door::declared(
    "lane",
    "lane.publication.grammar",
    "lane::publication",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "publication",
    },
);

/// The first rendering profile statement.
const FIRST_PROFILE: Profile = Profile::declared("lane", "first", Version::declared(1));

/// The rendering profile that replaces it.
const SECOND_PROFILE: Profile = Profile::declared("lane", "second", Version::declared(1));

/// The first assumption statement.
const FIRST_FACT: OwnerFact = OwnerFact {
    home: "lane",
    name: "first",
};

/// The assumption that replaces it.
const SECOND_FACT: OwnerFact = OwnerFact {
    home: "lane",
    name: "second",
};

/// The first publication-address statement.
const FIRST_ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "lane.address",
    bytes: [1; 32],
};

/// The publication address that replaces it.
const SECOND_ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "lane.address",
    bytes: [2; 32],
};

/// Every repeatable request statement keeps its replacement.
#[test]
fn every_repeated_request_statement_keeps_the_last_value() -> Result<(), ()> {
    let source = TextCapture::read("struct Publication;").map_err(|_| ())?;
    let first_dependency = TextCapture::read("struct First;").map_err(|_| ())?;
    let second_dependency = TextCapture::read("struct Second;").map_err(|_| ())?;
    let first_commitment = committed(first_dependency.input());
    let second_commitment = committed(second_dependency.input());

    let expansion = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .depending_on(vec![first_commitment])
        .depending_on(vec![second_commitment])
        .profile(FIRST_PROFILE)
        .profile(SECOND_PROFILE)
        .assuming(vec![FIRST_FACT])
        .assuming(vec![SECOND_FACT])
        .publishing_at(Seat::Publication, FIRST_ADDRESS)
        .publishing_at(Seat::Publication, SECOND_ADDRESS)
        .answering(vec![Answered::First])
        .answering(vec![Answered::Second])
        .selecting(Seat::Companion, Vec::new())
        .selecting(Seat::Publication, Vec::new())
        .render(|_plan, out| {
            out.unit(
                Seat::Publication,
                GeneratedTree::assembled(vec![GeneratedToken::word("publication")])?,
            )
        })
        .map_err(|_| ())?;

    assert_eq!(
        expansion.plan().account().dependencies(),
        &[second_commitment]
    );
    assert_eq!(expansion.plan().context().profile(), SECOND_PROFILE);
    assert_eq!(
        expansion
            .plan()
            .membership()
            .under(Seat::Publication)
            .ok_or(())?
            .output
            .address,
        Some(SECOND_ADDRESS)
    );
    let selected: Vec<OwnerFact> = expansion
        .plan()
        .trace()
        .entries()
        .iter()
        .filter_map(|entry| match entry.decision {
            TraceDecision::SelectedBecause(fact) => Some(fact),
            TraceDecision::OmittedBecause(_) | TraceDecision::NotRun => None,
        })
        .collect();
    assert!(selected.contains(&SECOND_FACT));
    assert!(!selected.contains(&FIRST_FACT));
    assert_eq!(expansion.explain().declared(), &[Answered::Second]);
    assert_ne!(first_commitment, second_commitment);
    Ok(())
}

fn rendered_word(word: &'static str) -> Result<GeneratedTree, macroonz_compiler::Overflow> {
    GeneratedTree::assembled(vec![GeneratedToken::word(word)])
}

/// The default and explicit complete selection agree, while one selected role becomes the exact planned set before rendering.
#[test]
fn request_selection_is_nonempty_checked_and_identity_bearing() -> Result<(), String> {
    assert_eq!(
        SELECTION_FACT,
        OwnerFact {
            home: "request",
            name: "a-requests-selected-seats-are-its-complete-output-set",
        }
    );
    let source = TextCapture::read("struct Publication;").map_err(|refusal| refusal.to_string())?;
    let default = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .publishing_at(Seat::Publication, FIRST_ADDRESS)
        .publishing_at(Seat::Companion, SECOND_ADDRESS)
        .render(|_plan, out| {
            out.unit(Seat::Publication, rendered_word("publication")?)?;
            out.unit(Seat::Companion, rendered_word("companion")?)
        })
        .map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let explicit = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Publication, vec![Seat::Companion])
        .publishing_at(Seat::Publication, FIRST_ADDRESS)
        .publishing_at(Seat::Companion, SECOND_ADDRESS)
        .render(|_plan, out| {
            out.unit(Seat::Publication, rendered_word("publication")?)?;
            out.unit(Seat::Companion, rendered_word("companion")?)
        })
        .map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let reversed = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Companion, vec![Seat::Publication])
        .publishing_at(Seat::Publication, FIRST_ADDRESS)
        .publishing_at(Seat::Companion, SECOND_ADDRESS)
        .render(|_plan, out| {
            out.unit(Seat::Publication, rendered_word("publication")?)?;
            out.unit(Seat::Companion, rendered_word("companion")?)
        })
        .map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let selected = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Companion, Vec::new())
        .publishing_at(Seat::Companion, SECOND_ADDRESS)
        .render(|plan, out| {
            assert_eq!(plan.membership().count(), 1);
            assert!(plan.membership().under(Seat::Publication).is_none());
            out.unit(Seat::Companion, rendered_word("companion")?)
        })
        .map_err(|diagnostic| diagnostic.summary().to_owned())?;

    assert_eq!(default.plan().identity(), explicit.plan().identity());
    assert_eq!(default.plan().identity(), reversed.plan().identity());
    assert_ne!(default.plan().identity(), selected.plan().identity());
    Ok(())
}

/// Selection admission and address consumption keep their existing typed planning authority.
#[test]
fn request_selection_refuses_doubled_foreign_and_inert_seats() -> Result<(), String> {
    let source = TextCapture::read("struct Publication;").map_err(|refusal| refusal.to_string())?;
    let doubled = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Publication, vec![Seat::Publication])
        .render(|_plan, _out| Ok(()))
        .err()
        .ok_or_else(|| "the doubled selection was admitted".to_owned())?;
    let foreign = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Foreign, Vec::new())
        .render(|_plan, _out| Ok(()))
        .err()
        .ok_or_else(|| "the foreign selection was admitted".to_owned())?;
    let inert = Request::<Publication>::over(source.input().clone(), "publication", &DOOR)
        .answering(vec![Answered::Second])
        .selecting(Seat::Publication, Vec::new())
        .publishing_at(Seat::Companion, SECOND_ADDRESS)
        .render(|_plan, _out| Ok(()))
        .err()
        .ok_or_else(|| "the inert address was admitted".to_owned())?;

    assert!(doubled.summary().contains("2 members stand under the seat"));
    assert!(
        foreign
            .summary()
            .contains("stands outside the kind's declared roster")
    );
    assert!(
        inert
            .summary()
            .contains("which no publication act consumes")
    );
    Ok(())
}

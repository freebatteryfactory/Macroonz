//! Repeated statements on one request, observed through the plan, trace, and explanation the complete public road returns.
//!
//! Each optional statement is made twice with distinguishable values.
//! The resulting expansion must carry only the second value, proving that replacement is the builder contract rather than an incidental field assignment.

use macroonz_compiler::request::committed;
use macroonz_compiler::{
    Answer, CrateBinding, Destination, Door, GeneratedToken, GeneratedTree, Kind, OwnerFact,
    OwnerIdentity, Producer, Profile, Question, Request, Role, TextCapture, TraceDecision, Version,
};

/// The one publishing seat this lane plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The generated publication.
    Publication,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Publication];

    fn name(self) -> &'static str {
        "publication"
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

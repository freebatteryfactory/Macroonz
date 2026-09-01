//! Historical-subject producer and consumer specimen.

pub(super) const SUBJECT_JOURNEYS_PRODUCER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

pub mod codec_subject {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ledger {
        pub count: u16,
    }

    impl Ledger {
        pub const fn assembled(count: u16) -> Self {
            Self { count }
        }
    }
}

bakery::recipe! {
    pub mod surface {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            Ready,
            Observed,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            Observe,
        }

        fn observe() {}

        bake! {
            vocabularies(State, Event);
            transitions {
                (Ready, Observe) => Observed with(super::observe);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                typestate;
                compile_contract;
                property;
            };
            support(surface_recipe_support);
        }
    }
}

bakery::recipe! {
    pub mod evolution {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Version {
            V1,
            V2,
            V3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Upgrade {
            V1ToV2,
            V2ToV3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Versioned {
            version: Version,
            value: u64,
        }

        impl Versioned {
            pub const fn new(version: Version, value: u64) -> Self {
                Self { version, value }
            }

            pub const fn version(self) -> Version {
                self.version
            }

            pub const fn value(self) -> u64 {
                self.value
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum EvolutionRefusal {
            Transition(baked::TransitionRefusal),
            Overflow,
        }

        impl From<baked::TransitionRefusal> for EvolutionRefusal {
            fn from(refusal: baked::TransitionRefusal) -> Self {
                Self::Transition(refusal)
            }
        }

        fn v1_to_v2() {}

        fn v2_to_v3() {}

        pub fn evolve(
            versioned: Versioned,
            upgrade: Upgrade,
        ) -> Result<Versioned, EvolutionRefusal> {
            let next = baked::advance(versioned.version, upgrade)?;
            let increment = match upgrade {
                Upgrade::V1ToV2 => 5,
                Upgrade::V2ToV3 => 7,
            };
            let value = versioned
                .value
                .checked_add(increment)
                .ok_or(EvolutionRefusal::Overflow)?;
            Ok(Versioned::new(next, value))
        }

        bake! {
            vocabularies(Version, Upgrade);
            transitions {
                (V1, V1ToV2) => V2 with(super::v1_to_v2);
                (V2, V2ToV3) => V3 with(super::v2_to_v3);
            };
            absence(refused);
            projections {
                companions;
                dispatch(advance);
                typestate;
                compile_contract;
                property;
            };
            support(evolution_recipe_support);
        }
    }
}

bakery::recipe! {
    pub mod guarded {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Standing {
            Idle,
            Armed,
            Done,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            Arm,
            Finish,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Capability {
            Basic,
            Elevated,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Operation {
            Arm,
            Finish,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum GuardRefusal {
            Unauthorized,
            Transition(baked::TransitionRefusal),
        }

        impl From<baked::TransitionRefusal> for GuardRefusal {
            fn from(refusal: baked::TransitionRefusal) -> Self {
                Self::Transition(refusal)
            }
        }

        fn arm() {}

        fn finish() {}

        const fn operation(event: Event) -> Operation {
            match event {
                Event::Arm => Operation::Arm,
                Event::Finish => Operation::Finish,
            }
        }

        const fn admitted(capability: Capability, operation: Operation) -> bool {
            matches!(
                (capability, operation),
                (Capability::Basic, Operation::Arm)
                    | (Capability::Elevated, Operation::Arm | Operation::Finish)
            )
        }

        pub fn execute(
            capability: Capability,
            standing: Standing,
            event: Event,
        ) -> Result<Standing, GuardRefusal> {
            if !admitted(capability, operation(event)) {
                return Err(GuardRefusal::Unauthorized);
            }
            baked::apply(standing, event).map_err(GuardRefusal::from)
        }

        bake! {
            vocabularies(Standing, Event);
            transitions {
                (Idle, Arm) => Armed with(super::arm);
                (Armed, Finish) => Done with(super::finish);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                compile_contract;
                property;
            };
            support(guarded_recipe_support);
        }
    }
}
";

pub(super) const SUBJECT_JOURNEYS_CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use bakery::compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
    CodecMemberShape, CodecPlacement, CodecProjection, CodecShape, CodecTypePath, PathRooting,
    render_codec,
};
use bakery::compiler::{
    Bounded, CanonicalContent, CrateBinding, Door, Producer, Request, TextCapture,
};

const CODEC_DOOR: Door = Door::declared(
    "subject-journeys",
    "subject-journeys.codec",
    "subject-journeys::codec",
    CrateBinding::declared("bakery"),
    Producer {
        namespace: "subject-journeys",
        name: "compiler",
    },
);

fn in_scope(spelling: &str) -> Result<CodecTypePath, String> {
    CodecTypePath::spelled(PathRooting::InScope, vec![spelling.to_owned()])
        .map_err(|refusal| refusal.to_string())
}

fn codec_content() -> Result<CodecContent, String> {
    let member = CodecMember::declared(
        "count",
        in_scope("u16")?,
        CodecMemberShape::Count,
        Cardinality::Required,
    )
    .map_err(|refusal| refusal.to_string())?;
    let assembly = CodecAssembly::stated("assembled", AssemblyPosture::Total)
        .map_err(|refusal| refusal.to_string())?;
    let shape = CodecShape::declared(
        in_scope("Ledger")?,
        "LedgerDecodeError",
        assembly,
        vec![member],
    )
    .map_err(|refusal| refusal.to_string())?;
    Ok(CodecContent {
        shape,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    })
}

#[test]
fn codec_uses_the_callable_compiler_without_a_subject_engine() -> Result<(), String> {
    let content = codec_content()?;
    assert_eq!(content.canonical_content_bytes(), CODEC_CONTENT_BYTES);
    let capture = TextCapture::read("struct Ledger;").map_err(|refusal| refusal.to_string())?;
    let expansion = Request::<CodecProjection>::over(capture.input().clone(), content, &CODEC_DOOR)
        .render(render_codec)
        .map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let emitted = expansion
        .emit()
        .tokens()
        .ok_or_else(|| "the codec expansion delivered no declaration-site tokens".to_owned())?
        .inspected();
    assert!(emitted.contains("impl Ledger"));
    assert!(emitted.contains("encode_canonical"));
    assert!(emitted.contains("decode_canonical"));
    assert_eq!(expansion.closure().plan(), expansion.plan().identity());
    assert_eq!(expansion.closure().rendered().count(), 1);
    Ok(())
}

#[test]
fn projection_roster_is_complete_without_subject_role_plumbing() {
    use renamed_recipe_adopter::surface::{Event, State, baked};

    assert_eq!(baked::STATE_VARIANTS, [State::Ready, State::Observed]);
    assert_eq!(baked::EVENT_VARIANTS, [Event::Observe]);
    assert_eq!(baked::TRANSITIONS, [(State::Ready, Event::Observe, State::Observed)]);
    assert_eq!(baked::apply(State::Ready, Event::Observe), Ok(State::Observed));
    assert!(matches!(
        baked::apply(State::Observed, Event::Observe),
        Err(baked::TransitionRefusal::Absent)
    ));
    let _stage = baked::typestate::Stage::<baked::typestate::Ready>::new();
}

#[test]
fn version_evolution_matches_the_handwritten_subject_behavior() {
    use renamed_recipe_adopter::evolution::{Upgrade, Version};

    for (version, value, upgrade) in [
        (Version::V1, 10, Upgrade::V1ToV2),
        (Version::V2, 15, Upgrade::V2ToV3),
        (Version::V1, 10, Upgrade::V2ToV3),
        (Version::V1, u64::MAX, Upgrade::V1ToV2),
    ] {
        assert_eq!(
            generated_evolution(version, value, upgrade),
            handwritten_evolution(version, value, upgrade)
        );
    }
}

#[test]
fn guarded_transition_and_policy_join_only_in_subject_behavior() {
    use renamed_recipe_adopter::guarded::{Capability, Event, Standing, execute};

    for (capability, standing, event) in [
        (Capability::Elevated, Standing::Idle, Event::Arm),
        (Capability::Elevated, Standing::Armed, Event::Finish),
        (Capability::Basic, Standing::Armed, Event::Finish),
        (Capability::Elevated, Standing::Idle, Event::Finish),
    ] {
        assert_eq!(
            execute(capability, standing, event),
            handwritten_guarded(capability, standing, event)
        );
    }
}

fn generated_evolution(
    version: renamed_recipe_adopter::evolution::Version,
    value: u64,
    upgrade: renamed_recipe_adopter::evolution::Upgrade,
) -> Result<(renamed_recipe_adopter::evolution::Version, u64), &'static str> {
    use renamed_recipe_adopter::evolution::{EvolutionRefusal, Versioned, evolve};

    evolve(Versioned::new(version, value), upgrade)
        .map(|versioned| (versioned.version(), versioned.value()))
        .map_err(|refusal| match refusal {
            EvolutionRefusal::Transition(_) => "transition",
            EvolutionRefusal::Overflow => "overflow",
        })
}

fn handwritten_evolution(
    version: renamed_recipe_adopter::evolution::Version,
    value: u64,
    upgrade: renamed_recipe_adopter::evolution::Upgrade,
) -> Result<(renamed_recipe_adopter::evolution::Version, u64), &'static str> {
    use renamed_recipe_adopter::evolution::{Upgrade, Version};

    let (next, increment) = match (version, upgrade) {
        (Version::V1, Upgrade::V1ToV2) => (Version::V2, 5),
        (Version::V2, Upgrade::V2ToV3) => (Version::V3, 7),
        _ => return Err("transition"),
    };
    value
        .checked_add(increment)
        .map(|evolved| (next, evolved))
        .ok_or("overflow")
}

fn handwritten_guarded(
    capability: renamed_recipe_adopter::guarded::Capability,
    standing: renamed_recipe_adopter::guarded::Standing,
    event: renamed_recipe_adopter::guarded::Event,
) -> Result<
    renamed_recipe_adopter::guarded::Standing,
    renamed_recipe_adopter::guarded::GuardRefusal,
> {
    use renamed_recipe_adopter::guarded::{
        Capability, Event, GuardRefusal, Standing, baked,
    };

    if matches!((capability, event), (Capability::Basic, Event::Finish)) {
        return Err(GuardRefusal::Unauthorized);
    }
    match (standing, event) {
        (Standing::Idle, Event::Arm) => Ok(Standing::Armed),
        (Standing::Armed, Event::Finish) => Ok(Standing::Done),
        _ => Err(GuardRefusal::Transition(
            baked::TransitionRefusal::Absent,
        )),
    }
}

const CODEC_CONTENT_BYTES: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 8, 105, 110, 45, 115, 99, 111, 112, 101, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 6, 76, 101, 100, 103, 101, 114, 0, 0, 0, 0, 0, 0, 0, 17, 76,
    101, 100, 103, 101, 114, 68, 101, 99, 111, 100, 101, 69, 114, 114, 111, 114, 0, 0, 0, 0,
    0, 0, 0, 9, 97, 115, 115, 101, 109, 98, 108, 101, 100, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 5, 99, 111, 117, 110, 116, 0, 0, 0, 0, 0,
    0, 0, 8, 105, 110, 45, 115, 99, 111, 112, 101, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
    0, 0, 3, 117, 49, 54, 0, 0, 0, 0, 0, 0, 0, 5, 99, 111, 117, 110, 116, 0, 0, 0, 0, 0,
    0, 0, 8, 114, 101, 113, 117, 105, 114, 101, 100, 0, 0, 0, 0, 0, 0, 0, 10, 114, 111,
    117, 110, 100, 45, 116, 114, 105, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
"#;

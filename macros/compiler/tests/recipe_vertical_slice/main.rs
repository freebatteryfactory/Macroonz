//! The callable recipe host and the paved wrapper envelope over one informed structural slice.

use macroonz_compiler::recipe::{
    HarnessPosture, ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink,
    RecipeBake, RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{
    CanonicalContent, CrateBinding, Destination, Door, GeneratedDelimiter, GeneratedToken,
    GeneratedTree, Producer, TextCapture, documentation, group,
};

const DOOR: Door = Door::declared(
    "recipe-crossing",
    "recipe-crossing.grammar",
    "recipe-crossing::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-crossing",
        name: "recipe",
    },
);

const COMPLETE_RECIPE: &str = r"
pub mod door {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        Closed,
        Open,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
            dispatch(apply);
            compile_contract;
            property;
        };
        support(door_recipe_support);
    }
}
";

const COMPANION_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
    }
}
";

struct MirroredCompanions;

impl RecipeProjector for MirroredCompanions {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Companions);
        assert_eq!(request.effective().role(), RecipeRole::Companions);
        let recipe = view.recipe();
        let mut tokens = roster_constant(
            "STATE_VARIANTS",
            "The state variants in caller-authored order.",
            recipe.states_name(),
            recipe
                .states()
                .members()
                .map(macroonz_compiler::recipe::RecipeMember::spelling),
        )?;
        tokens.extend(roster_constant(
            "EVENT_VARIANTS",
            "The event variants in caller-authored order.",
            recipe.events_name(),
            recipe
                .events()
                .members()
                .map(macroonz_compiler::recipe::RecipeMember::spelling),
        )?);
        tokens.extend(transition_constant(recipe)?);
        sink.offer(GeneratedTree::assembled(tokens)?)
    }
}

#[test]
fn the_callable_and_wrapper_hosts_emit_one_canonical_projection() -> Result<(), ()> {
    let callable = bake(COMPLETE_RECIPE)?;
    let wrapped_source =
        format!("{{ macroonz }} __macroonz_test_carrier_available {{ {COMPLETE_RECIPE} }}");
    let wrapped_capture = TextCapture::read(&wrapped_source).map_err(|_| ())?;
    let wrapped =
        macroonz_compiler::recipe::bake_wrapped(wrapped_capture.input(), &DOOR).map_err(|_| ())?;

    for destination in [
        Destination::DeclarationSite,
        Destination::TestCarrier,
        Destination::BenchCarrier,
    ] {
        assert_eq!(
            cargo_bytes(callable.projection(), destination),
            cargo_bytes(wrapped.projection(), destination),
            "the two hosts disagreed under {}",
            destination.name()
        );
    }
    assert_eq!(
        callable
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        wrapped
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    Ok(())
}

#[test]
fn a_caller_owned_projector_has_the_standard_clients_exact_authority() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Companions,
        &MirroredCompanions,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_projector_cannot_replace_an_unselected_role() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Dispatch,
        &MirroredCompanions,
    )
    .err()
    .ok_or(())?;
    assert!(refusal.summary().contains("unselected role `dispatch`"));
    Ok(())
}

#[test]
fn semantic_effect_movement_moves_identity_even_when_rendered_companions_do_not() -> Result<(), ()>
{
    let first = bake(COMPANION_RECIPE)?;
    let changed = COMPANION_RECIPE.replace("crate::effects::open", "crate::effects::unlock");
    let second = bake(&changed)?;

    assert_eq!(emitted_bytes(&first), emitted_bytes(&second));
    assert_ne!(
        first
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        second
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    Ok(())
}

#[test]
fn every_direct_type_namespace_collision_refuses_before_projection() -> Result<(), ()> {
    for declaration in [
        "mod baked {}",
        "struct baked;",
        "enum baked {}",
        "union baked { value: u8 }",
        "trait baked {}",
        "type baked = ();",
        "extern crate baked;",
        "struct r#baked;",
    ] {
        let source = format!(
            r"
pub mod door {{
    {declaration}

    pub enum State {{
        Closed,
        Open,
    }}

    pub enum Event {{
        OpenDoor,
    }}

    bake! {{
        vocabularies(State, Event);
        transitions {{
            (Closed, OpenDoor) => Open with(crate::effects::open);
        }};
        absence(refused);
        projections {{
            companions;
        }};
    }}
}}
"
        );
        let read = TextCapture::read(&source).map_err(|_| ())?;
        let refusal =
            macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
                .err()
                .ok_or(())?;
        assert!(refusal.summary().contains("generated name `baked`"));
    }
    Ok(())
}

fn bake(source: &str) -> Result<RecipeBake, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR).map_err(|_| ())
}

fn cargo_bytes(
    expansion: &macroonz_compiler::Expansion<macroonz_compiler::recipe::RecipeProjection>,
    destination: Destination,
) -> Option<Vec<u8>> {
    expansion
        .emission()
        .joined(destination)
        .and_then(macroonz_compiler::PartitionCargo::tokens)
        .map(GeneratedTree::canonical_bytes)
}

fn emitted_bytes(bake: &RecipeBake) -> Option<Vec<u8>> {
    bake.emit().tokens().map(GeneratedTree::canonical_bytes)
}

fn roster_constant<'name>(
    constant: &str,
    sentence: &str,
    vocabulary: &str,
    members: impl Iterator<Item = &'name str>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut tokens = documentation(sentence)?;
    tokens.extend([GeneratedToken::word("pub"), GeneratedToken::word("const")]);
    tokens.push(GeneratedToken::word(constant));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::alone('&'));
    tokens.push(group(GeneratedDelimiter::Bracket, super_path(vocabulary))?);
    tokens.push(GeneratedToken::alone('='));
    tokens.push(GeneratedToken::alone('&'));
    tokens.push(group(
        GeneratedDelimiter::Bracket,
        comma_separated(members.map(|member| variant(vocabulary, member)).collect()),
    )?);
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

fn transition_constant(
    recipe: &macroonz_compiler::recipe::Recipe,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut tokens = documentation("The informed transition rows in caller-authored order.")?;
    tokens.extend([GeneratedToken::word("pub"), GeneratedToken::word("const")]);
    tokens.push(GeneratedToken::word("TRANSITIONS"));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::alone('&'));
    tokens.push(group(
        GeneratedDelimiter::Bracket,
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                super_path(recipe.states_name()),
                super_path(recipe.events_name()),
                super_path(recipe.states_name()),
            ]),
        )?],
    )?);
    tokens.push(GeneratedToken::alone('='));
    tokens.push(GeneratedToken::alone('&'));
    let rows = recipe
        .transitions()
        .members()
        .map(|transition| {
            group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    variant(recipe.states_name(), transition.from()),
                    variant(recipe.events_name(), transition.event()),
                    variant(recipe.states_name(), transition.to()),
                ]),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut separated = Vec::new();
    for row in rows {
        separated.extend([row, GeneratedToken::alone(',')]);
    }
    tokens.push(group(GeneratedDelimiter::Bracket, separated)?);
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

fn super_path(name: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(name),
    ]
}

fn variant(vocabulary: &str, member: &str) -> Vec<GeneratedToken> {
    let mut tokens = super_path(vocabulary);
    tokens.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(member),
    ]);
    tokens
}

fn comma_separated(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone(','));
        }
        tokens.extend(part);
    }
    tokens
}

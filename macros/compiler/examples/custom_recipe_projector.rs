//! A caller-owned recipe projector using only the callable compiler package.

use macroonz_compiler::recipe::{
    HarnessPosture, LoweringSource, ProjectionError, ProjectionOffered, ProjectionRequest,
    ProjectionSink, RecipeProjector, RecipeRole, RecipeView, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};
use macroonz_compiler::{
    CrateBinding, Destination, Door, GeneratedDelimiter, GeneratedToken, GeneratedTree, Overflow,
    Producer, TextCapture, constant, group,
};

const RECIPE_DOOR: Door = Door::declared(
    "custom-recipe-projector",
    "custom-recipe-projector.recipe",
    "custom-recipe-projector::bake",
    CrateBinding::declared("macroonz_compiler"),
    Producer {
        namespace: "custom-recipe-projector",
        name: "compiler",
    },
);

const SOURCE: &str = r"
pub mod inventory {
    pub enum Left {
        First,
        Second,
    }

    pub enum Right {
        Alpha,
        Beta,
        Gamma,
    }

    bake! {
        vocabularies(Left, Right);
        transitions {
            (First, Alpha) => Second with(crate::observe);
            (Second, Beta) => First with(crate::observe);
        };
        absence(refused);
        projections {
            companions;
        };
    }
}
";

struct StructuralDimensions;

impl RecipeProjector for StructuralDimensions {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Companions);
        assert_eq!(request.destination(), Destination::DeclarationSite);
        assert_eq!(request.effective().source(), LoweringSource::Preset);

        let recipe = view.recipe();
        let state_count = recipe.states().count();
        let event_count = recipe.events().count();
        let transition_count = recipe.transitions().count();
        let values = [
            generated_count(state_count, VOCABULARY_LIMIT)?,
            generated_count(event_count, VOCABULARY_LIMIT)?,
            generated_count(transition_count, TRANSITION_LIMIT)?,
        ];
        let mut tokens = vec![GeneratedToken::word("pub")];
        tokens.extend(constant(
            "STRUCTURAL_DIMENSIONS",
            tuple([
                GeneratedToken::word("usize"),
                GeneratedToken::word("usize"),
                GeneratedToken::word("usize"),
            ])?,
            tuple(values)?,
        ));
        tokens.extend(effect_paths(recipe)?);
        sink.offer(GeneratedTree::assembled(tokens)?)
    }
}

fn effect_paths(
    recipe: &macroonz_compiler::recipe::Recipe,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut values = Vec::new();
    for (position, transition) in recipe.transitions().members().enumerate() {
        if position > 0 {
            values.push(GeneratedToken::alone(','));
        }
        values.extend([
            GeneratedToken::word("stringify"),
            GeneratedToken::alone('!'),
            group(
                GeneratedDelimiter::Parenthesis,
                transition.effect().tokens().to_vec(),
            )?,
        ]);
    }
    Ok(constant(
        "STRUCTURAL_EFFECT_PATHS",
        vec![
            GeneratedToken::alone('&'),
            group(
                GeneratedDelimiter::Bracket,
                vec![GeneratedToken::alone('&'), GeneratedToken::word("str")],
            )?,
        ],
        vec![
            GeneratedToken::alone('&'),
            group(GeneratedDelimiter::Bracket, values)?,
        ],
    ))
}

fn generated_count(value: usize, limit: usize) -> Result<GeneratedToken, ProjectionError> {
    u64::try_from(value)
        .map(GeneratedToken::number)
        .map_err(|_| {
            ProjectionError::Tokens(Overflow {
                capacity: limit,
                offered: value,
            })
        })
}

fn tuple<const N: usize>(
    members: [GeneratedToken; N],
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut separated = Vec::new();
    for (position, member) in members.into_iter().enumerate() {
        if position > 0 {
            separated.push(GeneratedToken::alone(','));
        }
        separated.push(member);
    }
    Ok(vec![group(GeneratedDelimiter::Parenthesis, separated)?])
}

fn main() -> Result<(), String> {
    let captured = TextCapture::read(SOURCE).map_err(|error| error.to_string())?;
    let baked = macroonz_compiler::recipe::bake_with(
        captured.input(),
        HarnessPosture::Available,
        &RECIPE_DOOR,
        RecipeRole::Companions,
        &StructuralDimensions,
    )
    .map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let emitted = baked
        .emit()
        .tokens()
        .ok_or_else(|| "the custom declaration-site projection was not delivered".to_owned())?
        .inspected();
    assert!(emitted.contains("STRUCTURAL_DIMENSIONS"));
    assert!(emitted.contains("( 2 , 3 , 2 )"));
    assert!(emitted.contains("STRUCTURAL_EFFECT_PATHS"));
    assert!(emitted.contains("crate :: observe"));
    Ok(())
}

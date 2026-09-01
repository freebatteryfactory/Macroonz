use macroonz_compiler::Output;
use macroonz_compiler::recipe::{ProjectionSink, RecipeProjection, RecipeRole};

fn forged<'output, 'plan>(
    output: &'output mut Output<'plan, RecipeProjection>,
) -> ProjectionSink<'output, 'plan> {
    ProjectionSink {
        output,
        role: RecipeRole::Companions,
    }
}

fn main() {}

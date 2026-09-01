use macroonz_compiler::recipe::{Recipe, RecipeView};

fn forged(recipe: &Recipe) -> RecipeView<'_> {
    RecipeView { recipe }
}

fn main() {}

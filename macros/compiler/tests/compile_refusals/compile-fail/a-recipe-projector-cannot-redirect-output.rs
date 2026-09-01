use macroonz_compiler::GeneratedTree;
use macroonz_compiler::recipe::{ProjectionSink, RecipeRole};

fn redirect(sink: ProjectionSink<'_, '_>, tree: GeneratedTree) {
    let _ = sink.offer_as(RecipeRole::Dispatch, tree);
}

fn main() {}

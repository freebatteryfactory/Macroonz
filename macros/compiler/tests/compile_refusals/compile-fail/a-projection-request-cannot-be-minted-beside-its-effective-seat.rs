use macroonz_compiler::recipe::{EffectiveProjection, ProjectionRequest};

fn forged(effective: &EffectiveProjection) -> ProjectionRequest<'_> {
    ProjectionRequest { effective }
}

fn main() {}

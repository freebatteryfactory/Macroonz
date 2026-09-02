//! The standard typestate projection over one informed state vocabulary.

use super::render_tokens::{derive, public, row_projection_error, static_str};
use super::{ProjectionError, Recipe, RecipeMember};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, absolute_path, associated_constant,
    associated_function, decorated, documentation, function_signature, group, implementation,
    inline_module, keyed_roster_items, trait_declaration, tuple_struct, unit_struct, use_item,
};

pub(super) fn typestate(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let Some(subject) = recipe
        .effective(super::RecipeRole::Typestate)
        .and_then(super::EffectiveProjection::subject)
        .and_then(|name| recipe.vocabulary(name))
    else {
        return Err(ProjectionError::Render(
            crate::render::RenderError::NothingRendered,
        ));
    };
    let mut items = use_item(absolute_path(&["core", "marker", "PhantomData"]), None);
    items.extend(stage_trait()?);
    items.extend(
        keyed_roster_items(subject.members(), |_position, spelling, member| {
            stage_member(spelling, member)
        })
        .map_err(row_projection_error)?,
    );
    let marker = vec![GeneratedToken::word("Marker")];
    let phantom = vec![
        GeneratedToken::word("PhantomData"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone('>'),
    ];
    items.extend(decorated(
        vec![
            documentation("A type-level carrier over one caller-declared stage.")?,
            derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
        ],
        public(),
        tuple_struct(
            GeneratedToken::word("Stage"),
            vec![marker],
            vec![decorated(Vec::new(), public(), phantom)],
            Vec::new(),
        )?,
    ));
    items.extend(stage_inherent()?);
    items.extend(stage_default()?);
    let projected = decorated(
        vec![documentation(
            "Type-level stages derived from the caller-authored state vocabulary.",
        )?],
        public(),
        inline_module(GeneratedToken::word("typestate"), items)?,
    );
    GeneratedTree::assembled(projected).map_err(ProjectionError::Tokens)
}

fn stage_trait() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let name = associated_constant(GeneratedToken::word("NAME"), static_str(), None);
    Ok(decorated(
        vec![documentation(
            "One caller-declared member admitted as a typestate stage.",
        )?],
        public(),
        trait_declaration(
            Vec::new(),
            GeneratedToken::word("RecipeStage"),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            decorated(
                vec![documentation("The caller-authored stage spelling.")?],
                Vec::new(),
                name,
            ),
        )?,
    ))
}

fn stage_member(
    spelling: &str,
    member: &RecipeMember,
) -> Result<Vec<GeneratedToken>, crate::bounded::Overflow> {
    let mut tokens = decorated(
        vec![
            documentation("One caller-declared typestate stage.")?,
            derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
        ],
        public(),
        unit_struct(member.name_token().clone(), Vec::new(), Vec::new()),
    );
    tokens.extend(implementation(
        Vec::new(),
        Vec::new(),
        Some(vec![GeneratedToken::word("RecipeStage")]),
        vec![member.name_token().clone()],
        Vec::new(),
        associated_constant(
            GeneratedToken::word("NAME"),
            static_str(),
            Some(vec![GeneratedToken::text(spelling)]),
        ),
    )?);
    Ok(tokens)
}

fn stage_inherent() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let constructor = decorated(
        vec![documentation("Constructs the zero-sized stage carrier.")?],
        public(),
        associated_function(
            function_signature(
                vec![GeneratedToken::word("const")],
                GeneratedToken::word("new"),
                Vec::new(),
                Vec::new(),
                Some(vec![GeneratedToken::word("Self")]),
                Vec::new(),
            )?,
            Some(vec![
                GeneratedToken::word("Self"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("PhantomData")],
                )?,
            ]),
        )?,
    );
    implementation(
        Vec::new(),
        vec![vec![GeneratedToken::word("Marker")]],
        None,
        generic_stage(),
        Vec::new(),
        constructor,
    )
    .map_err(ProjectionError::Tokens)
}

fn stage_default() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let body = vec![
        GeneratedToken::word("Self"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("PhantomData")],
        )?,
    ];
    let function = associated_function(
        function_signature(
            Vec::new(),
            GeneratedToken::word("default"),
            Vec::new(),
            Vec::new(),
            Some(vec![GeneratedToken::word("Self")]),
            Vec::new(),
        )?,
        Some(body),
    )?;
    implementation(
        Vec::new(),
        vec![vec![GeneratedToken::word("Marker")]],
        Some(absolute_path(&["core", "default", "Default"])),
        generic_stage(),
        Vec::new(),
        function,
    )
    .map_err(ProjectionError::Tokens)
}

fn generic_stage() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("Stage"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone('>'),
    ]
}

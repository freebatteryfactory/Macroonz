//! Reading the one inline-module recipe grammar into informed structural values.

use super::types::{ProjectionStanding, RecipeError, RecipeIssue, RecipeParts};
use super::{
    EffectiveProjection, HarnessPosture, LoweringSource, Recipe, RecipeMember, RecipeRole,
    RecipeTransition,
};
use crate::bounded::AbsencePosture;
use crate::support::SupportName;
use crate::token::{
    AuthoredItemKind, CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedFragment,
    CapturedInput, CapturedSpacing, CapturedTokenTree, GeneratedTree, preserved_tokens,
};

/// The private suffix that declares one recipe inside its authored module.
const BAKE: &str = "bake";

impl Recipe {
    /// Read one inline authored module and its final `bake!` declaration into an informed recipe.
    ///
    /// # Errors
    ///
    /// Returns the exact structural, grammar, membership, collision, or feature-posture refusal established before planning.
    pub(in crate::recipe) fn read(
        input: &CapturedInput,
        harness: HarnessPosture,
    ) -> Result<Self, RecipeError> {
        let item = input.authored_item().map_err(|refusal| {
            RecipeError::at(RecipeIssue::InlineModuleRequired, refusal.token())
        })?;
        if item.kind() != AuthoredItemKind::Module {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(item.kind_token().span()),
            ));
        }
        let Some((name_token, module_name)) = item.name() else {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(item.kind_token().span()),
            ));
        };
        let Some((CapturedDelimiter::Brace, body)) = item.body() else {
            return Err(RecipeError::at(
                RecipeIssue::InlineModuleRequired,
                Some(name_token.span()),
            ));
        };
        let (authored, declaration) = bake_suffix(body)?;
        collision_free(authored)?;
        let read = read_bake(declaration, harness)?;
        let states = enum_members(authored, read.states_name.as_str())?;
        let events = enum_members(authored, read.events_name.as_str())?;

        let attributes = item
            .attributes()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let signature = item
            .signature()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let mut head = attributes.tokens().to_vec();
        head.extend(signature.tokens().iter().cloned());
        let module_head = GeneratedTree::assembled(head)
            .map_err(|_| fragment_refusal(Some(name_token.span())))?;
        let authored_body = GeneratedTree::assembled(
            preserved_tokens(authored).map_err(|refusal| fragment_refusal(refusal.token()))?,
        )
        .map_err(|_| fragment_refusal(body.first_span()))?;

        Recipe::informed(RecipeParts {
            module_name: module_name.to_owned(),
            module_head,
            authored_body,
            states_name: read.states_name,
            state_members: states,
            events_name: read.events_name,
            event_members: events,
            transitions: read.transitions,
            absence: read.absence,
            projections: read.projections,
            support: read.support,
        })
    }
}

/// The mechanically read bake declaration before structural informing.
struct BakeRead {
    states_name: String,
    events_name: String,
    transitions: Vec<RecipeTransition>,
    absence: AbsencePosture,
    projections: [ProjectionStanding; 4],
    support: Option<SupportName>,
}

/// Split the required final `bake! { ... }` suffix from the authored module body.
fn bake_suffix(
    body: CapturedFragment<'_>,
) -> Result<(&[CapturedTokenTree], CapturedFragment<'_>), RecipeError> {
    let tokens = body.tokens();
    let suffix = match tokens {
        [authored @ .., name, bang, group]
            if name.word() == Some(BAKE)
                && bang.punct() == Some('!')
                && group.group_fragment(CapturedDelimiter::Brace).is_some() =>
        {
            (authored, group)
        }
        [authored @ .., name, bang, group, end]
            if name.word() == Some(BAKE)
                && bang.punct() == Some('!')
                && group.group_fragment(CapturedDelimiter::Brace).is_some()
                && end.punct() == Some(';') =>
        {
            (authored, group)
        }
        _ => {
            return Err(RecipeError::at(
                RecipeIssue::BakeRequiredLast,
                body.last_span().or(body.enclosing_span()),
            ));
        }
    };
    let declaration = suffix
        .1
        .group_fragment(CapturedDelimiter::Brace)
        .ok_or_else(|| RecipeError::at(RecipeIssue::BakeRequiredLast, Some(suffix.1.span())))?;
    Ok((suffix.0, declaration))
}

/// Refuse a direct authored type-namespace occupant of the generated child name before any projector runs.
fn collision_free(authored: &[CapturedTokenTree]) -> Result<(), RecipeError> {
    for pair in authored.windows(2) {
        let [kind, name] = pair else {
            continue;
        };
        if matches!(
            kind.word(),
            Some("mod" | "struct" | "enum" | "union" | "trait" | "type")
        ) && name
            .word()
            .or_else(|| name.raw_identifier())
            .is_some_and(|spelling| spelling == "baked")
        {
            return Err(generated_name_collision(name));
        }
    }
    for triple in authored.windows(3) {
        let [external, crate_word, name] = triple else {
            continue;
        };
        if external.word() == Some("extern")
            && crate_word.word() == Some("crate")
            && name
                .word()
                .or_else(|| name.raw_identifier())
                .is_some_and(|spelling| spelling == "baked")
        {
            return Err(generated_name_collision(name));
        }
    }
    Ok(())
}

fn generated_name_collision(name: &CapturedTokenTree) -> RecipeError {
    RecipeError::at(
        RecipeIssue::GeneratedNameCollision {
            name: "baked".to_owned(),
        },
        Some(name.span()),
    )
}

/// Read the fixed first vertical-slice grammar from the bake group.
fn read_bake(
    declaration: CapturedFragment<'_>,
    harness: HarnessPosture,
) -> Result<BakeRead, RecipeError> {
    let mut cursor = declaration.cursor();
    let (states_name, events_name) = read_vocabularies(&mut cursor)?;

    cursor.word("transitions").map_err(grammar)?;
    let transitions = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::TRANSITION_LIMIT }>(';', read_transition)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;

    let absence = read_absence(&mut cursor)?;

    cursor.word("projections").map_err(grammar)?;
    let requested = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, 4>(';', read_projection)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;

    let support = read_support(&mut cursor)?;
    cursor.finish().map_err(grammar)?;
    let projections = projections(&requested, harness)?;
    support_matches_projections(&projections, support.as_ref(), declaration.last_span())?;
    Ok(BakeRead {
        states_name: states_name.clone(),
        events_name: events_name.clone(),
        transitions,
        absence,
        projections,
        support,
    })
}

fn read_vocabularies(cursor: &mut CaptureCursor<'_>) -> Result<(String, String), RecipeError> {
    cursor.word("vocabularies").map_err(grammar)?;
    let mut vocabularies = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (_, states_name) = vocabularies.identifier().map_err(grammar)?;
    vocabularies
        .punctuation(',', CapturedSpacing::Alone)
        .map_err(grammar)?;
    let (_, events_name) = vocabularies.identifier().map_err(grammar)?;
    let names = (states_name.to_owned(), events_name.to_owned());
    vocabularies.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(names)
}

fn read_absence(cursor: &mut CaptureCursor<'_>) -> Result<AbsencePosture, RecipeError> {
    cursor.word("absence").map_err(grammar)?;
    let mut clause = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, name) = clause.identifier().map_err(grammar)?;
    let absence = match name {
        "allowed" => AbsencePosture::Allowed,
        "refused" => AbsencePosture::Refusal,
        _ => {
            return Err(RecipeError::at(
                RecipeIssue::Grammar(crate::token::CaptureReadIssue::Unexpected(
                    crate::token::CaptureExpectation::Word("allowed or refused".to_owned()),
                )),
                Some(token.span()),
            ));
        }
    };
    clause.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(absence)
}

fn read_support(cursor: &mut CaptureCursor<'_>) -> Result<Option<SupportName>, RecipeError> {
    if cursor.is_finished() {
        return Ok(None);
    }
    cursor.word("support").map_err(grammar)?;
    let mut address = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, spelling) = address.identifier().map_err(grammar)?;
    let declared = SupportName::declared(spelling).map_err(|_| {
        RecipeError::at(
            RecipeIssue::GeneratedNameNotIdentifier {
                name: spelling.to_owned(),
            },
            Some(token.span()),
        )
    })?;
    address.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(Some(declared))
}

fn support_matches_projections(
    projections: &[ProjectionStanding; 4],
    support: Option<&SupportName>,
    at: Option<crate::token::SpanHandle>,
) -> Result<(), RecipeError> {
    let evidence = generated(projections, RecipeRole::CompileContract)
        || generated(projections, RecipeRole::Property);
    match (evidence, support.is_some()) {
        (true, false) => Err(RecipeError::at(RecipeIssue::SupportAddressRequired, at)),
        (false, true) => Err(RecipeError::at(RecipeIssue::SupportAddressUnneeded, at)),
        (true, true) | (false, false) => Ok(()),
    }
}

/// Read one relation row and preserve its effect path structurally.
fn read_transition(cursor: &mut CaptureCursor<'_>) -> Result<RecipeTransition, CaptureReadRefusal> {
    let mut endpoints = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (from_token, from) = endpoints.identifier()?;
    endpoints.punctuation(',', CapturedSpacing::Alone)?;
    let (_, event) = endpoints.identifier()?;
    endpoints.finish()?;
    cursor.fat_arrow()?;
    let (_, to) = cursor.identifier()?;
    cursor.word("with")?;
    let mut effect = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (effect, ()) = effect.fragment(|path| {
        path.identifier()?;
        while !path.is_finished() {
            path.punctuation(':', CapturedSpacing::Joint)?;
            path.punctuation(':', CapturedSpacing::Alone)?;
            path.identifier()?;
        }
        Ok(())
    })?;
    let effect = effect.generated().map_err(|refusal| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::CursorRangeContradiction,
            refusal.token(),
        )
    })?;
    Ok(RecipeTransition::authored(
        from.to_owned(),
        event.to_owned(),
        to.to_owned(),
        effect,
        from_token.span(),
    ))
}

/// One requested role with its mechanical configuration.
#[derive(Clone)]
struct RequestedProjection {
    role: RecipeRole,
    name: Option<String>,
    source: LoweringSource,
    at: crate::token::SpanHandle,
}

/// Read one projection request.
fn read_projection(
    cursor: &mut CaptureCursor<'_>,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    let (role, name, source) = match spelling {
        "companions" => (RecipeRole::Companions, None, LoweringSource::Preset),
        "dispatch" => {
            let mut configured = cursor.group(CapturedDelimiter::Parenthesis)?;
            let (configured_name, configured_spelling) = configured.identifier()?;
            if configured_name.raw_identifier().is_some() {
                return Err(CaptureReadRefusal::projected(
                    crate::token::CaptureReadIssue::Unexpected(
                        crate::token::CaptureExpectation::Identifier,
                    ),
                    Some(configured_name.span()),
                ));
            }
            configured.finish()?;
            (
                RecipeRole::Dispatch,
                Some(configured_spelling.to_owned()),
                LoweringSource::Configuration,
            )
        }
        "compile_contract" => (RecipeRole::CompileContract, None, LoweringSource::Preset),
        "property" => (RecipeRole::Property, None, LoweringSource::Preset),
        _ => {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                    "a recipe projection".to_owned(),
                )),
                Some(token.span()),
            ));
        }
    };
    Ok(RequestedProjection {
        role,
        name,
        source,
        at: token.span(),
    })
}

/// Build the complete projection account and enforce harness posture before planning.
fn projections(
    requested: &[RequestedProjection],
    harness: HarnessPosture,
) -> Result<[ProjectionStanding; 4], RecipeError> {
    for (position, row) in requested.iter().enumerate() {
        if requested
            .iter()
            .take(position)
            .any(|earlier| earlier.role == row.role)
        {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateProjection { role: row.role },
                Some(row.at),
            ));
        }
        if harness == HarnessPosture::Unavailable
            && matches!(row.role, RecipeRole::CompileContract | RecipeRole::Property)
        {
            return Err(RecipeError::at(
                RecipeIssue::HarnessUnavailable { role: row.role },
                Some(row.at),
            ));
        }
    }
    Ok([
        standing(requested, RecipeRole::Companions),
        standing(requested, RecipeRole::Dispatch),
        standing(requested, RecipeRole::CompileContract),
        standing(requested, RecipeRole::Property),
    ])
}

fn standing(requested: &[RequestedProjection], role: RecipeRole) -> ProjectionStanding {
    requested
        .iter()
        .find(|row| row.role == role)
        .map_or(ProjectionStanding::NotRequested, |row| {
            ProjectionStanding::Generated(EffectiveProjection::effective(
                role,
                row.name.clone(),
                row.source,
            ))
        })
}

fn generated(projections: &[ProjectionStanding; 4], role: RecipeRole) -> bool {
    let [companions, dispatch, compile_contract, property] = projections;
    matches!(
        match role {
            RecipeRole::Companions => companions,
            RecipeRole::Dispatch => dispatch,
            RecipeRole::CompileContract => compile_contract,
            RecipeRole::Property => property,
        },
        ProjectionStanding::Generated(_)
    )
}

/// Read one named authored enum and its unit-variant roster.
fn enum_members(
    authored: &[CapturedTokenTree],
    sought: &str,
) -> Result<Vec<RecipeMember>, RecipeError> {
    let found = authored.windows(2).position(|pair| {
        matches!(pair, [kind, name]
            if kind.word() == Some("enum")
                && name
                    .word()
                    .or_else(|| name.raw_identifier())
                    .is_some_and(|spelling| spelling == sought))
    });
    let Some(position) = found else {
        return Err(RecipeError::at(
            RecipeIssue::VocabularyNotFound {
                name: sought.to_owned(),
            },
            authored.first().map(CapturedTokenTree::span),
        ));
    };
    let name_position = position.saturating_add(1);
    let Some(name) = authored.get(name_position) else {
        return Err(RecipeError::at(
            RecipeIssue::VocabularyNotFound {
                name: sought.to_owned(),
            },
            authored.first().map(CapturedTokenTree::span),
        ));
    };
    let body_position = name_position.saturating_add(1);
    let body = authored
        .get(body_position..)
        .and_then(|after_name| {
            after_name
                .iter()
                .find_map(|candidate| candidate.group_fragment(CapturedDelimiter::Brace))
        })
        .ok_or_else(|| {
            RecipeError::at(
                RecipeIssue::VocabularyNotFound {
                    name: sought.to_owned(),
                },
                Some(name.span()),
            )
        })?;
    unit_variants(body, sought)
}

fn unit_variants(
    body: CapturedFragment<'_>,
    vocabulary: &str,
) -> Result<Vec<RecipeMember>, RecipeError> {
    let mut members = Vec::new();
    for row in variant_rows(body.tokens()) {
        if row.is_empty() {
            continue;
        }
        members.push(member(row, vocabulary)?);
    }
    Ok(members)
}

fn member(row: &[CapturedTokenTree], vocabulary: &str) -> Result<RecipeMember, RecipeError> {
    let stripped = without_outer_attributes(row);
    let (token, spelling) = match stripped {
        [token] => identifier(token),
        [token, equals, discriminant @ ..]
            if equals.punct() == Some('=') && !discriminant.is_empty() =>
        {
            identifier(token)
        }
        [token, ..] => (token, None),
        [] => {
            return Err(RecipeError::at(
                RecipeIssue::VariantNotUnit {
                    vocabulary: vocabulary.to_owned(),
                    variant: "<unnamed>".to_owned(),
                },
                row.first().map(CapturedTokenTree::span),
            ));
        }
    };
    spelling
        .map(|name| RecipeMember::authored(name.to_owned(), token.span()))
        .ok_or_else(|| variant_refusal(token, vocabulary))
}

fn variant_rows(tokens: &[CapturedTokenTree]) -> Vec<&[CapturedTokenTree]> {
    let mut rows = Vec::new();
    let mut opening = 0usize;
    for (position, token) in tokens.iter().enumerate() {
        if token.punct() == Some(',') {
            if let Some(row) = tokens.get(opening..position) {
                rows.push(row);
            }
            opening = position.saturating_add(1);
        }
    }
    if let Some(row) = tokens.get(opening..tokens.len()) {
        rows.push(row);
    }
    rows
}

fn without_outer_attributes(mut row: &[CapturedTokenTree]) -> &[CapturedTokenTree] {
    loop {
        match row {
            [hash, attribute, rest @ ..]
                if hash.punct() == Some('#')
                    && attribute
                        .group_fragment(CapturedDelimiter::Bracket)
                        .is_some() =>
            {
                row = rest;
            }
            _ => return row,
        }
    }
}

fn identifier(token: &CapturedTokenTree) -> (&CapturedTokenTree, Option<&str>) {
    (token, token.word().or_else(|| token.raw_identifier()))
}

fn variant_refusal(token: &CapturedTokenTree, vocabulary: &str) -> RecipeError {
    let spelling = token
        .word()
        .or_else(|| token.raw_identifier())
        .unwrap_or("<unnamed>");
    RecipeError::at(
        RecipeIssue::VariantNotUnit {
            vocabulary: vocabulary.to_owned(),
            variant: spelling.to_owned(),
        },
        Some(token.span()),
    )
}

fn grammar(refusal: CaptureReadRefusal) -> RecipeError {
    let (issue, at) = refusal.into_parts();
    RecipeError::at(RecipeIssue::Grammar(issue), at)
}

fn fragment_refusal(at: Option<crate::token::SpanHandle>) -> RecipeError {
    RecipeError::at(RecipeIssue::FragmentNotGenerated, at)
}

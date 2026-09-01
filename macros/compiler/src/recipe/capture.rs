//! Reading the one inline-module recipe grammar into informed structural values.

use super::types::{ProjectionStanding, RecipeError, RecipeIssue, RecipeParts};
use super::{
    EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture, LoweringSource, Recipe,
    RecipeEvidence, RecipeMember, RecipeRole, RecipeTransition,
};
use crate::relation::AbsencePosture;
use crate::support::SupportName;
use crate::token::{
    AuthoredItemKind, CaptureReadRefusal, CapturedDelimiter, CapturedInput, CapturedTokenTree,
    preserved_tree,
};

#[path = "capture_bake.rs"]
mod bake;
#[path = "capture_dispatch.rs"]
mod dispatch;
#[path = "capture_evidence.rs"]
mod evidence;
#[path = "capture_module.rs"]
mod module;
#[path = "capture_projection.rs"]
mod projection;

use bake::read_bake;
use module::{bake_suffix, collision_free, enum_members};

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
        let read = read_bake(declaration, harness, input.issued())?;
        let states = enum_members(authored, read.states.spelling.as_str())?;
        let events = enum_members(authored, read.events.spelling.as_str())?;

        let attributes = item
            .attributes()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let signature = item
            .signature()
            .generated()
            .map_err(|refusal| fragment_refusal(refusal.token()))?;
        let module_head = attributes
            .joined(&signature)
            .map_err(|_| fragment_refusal(Some(name_token.span())))?;
        let authored_body =
            preserved_tree(authored).map_err(|refusal| fragment_refusal(refusal.token()))?;

        Recipe::informed(RecipeParts {
            module_name: module_name.to_owned(),
            module_name_token: identifier_token(name_token, module_name),
            module_head,
            authored_body,
            module_body_at: body.enclosing_span(),
            states_name: read.states.spelling,
            states_name_token: read.states.token,
            state_members: states,
            events_name: read.events.spelling,
            events_name_token: read.events.token,
            event_members: events,
            transitions: read.transitions,
            absence: read.absence,
            projections: read.projections,
            evidence: read.evidence,
            support: read.support,
        })
    }
}

/// The mechanically read bake declaration before structural informing.
struct BakeRead {
    states: CapturedName,
    events: CapturedName,
    transitions: Vec<RecipeTransition>,
    absence: AbsencePosture,
    projections: [ProjectionStanding; 10],
    evidence: [Option<RecipeEvidence>; EVIDENCE_LIMIT],
    support: Option<SupportName>,
}

/// One exact identifier read from recipe syntax before its structural role is informed.
struct CapturedName {
    spelling: String,
    token: crate::token::GeneratedToken,
}

/// The two vocabularies named by one recipe declaration.
struct VocabularyNames {
    states: CapturedName,
    events: CapturedName,
}

/// One requested role with its mechanical configuration.
#[derive(Clone)]
struct RequestedProjection {
    role: RecipeRole,
    name: Option<String>,
    source: LoweringSource,
    exact: Option<CapturedInput>,
    at: crate::token::SpanHandle,
}

/// One descriptor-native evidence role and its generated or target-unavailable standing.
#[derive(Clone)]
struct RequestedEvidence {
    role: RecipeRole,
    target: Option<EvidenceTarget>,
    body: Option<CapturedInput>,
    at: crate::token::SpanHandle,
}

fn identifier_token(token: &CapturedTokenTree, spelling: &str) -> crate::token::GeneratedToken {
    if token.raw_identifier().is_some() {
        crate::token::GeneratedToken::raw_identifier(spelling)
    } else {
        crate::token::GeneratedToken::word(spelling)
    }
}

fn grammar(refusal: CaptureReadRefusal) -> RecipeError {
    let (issue, at) = refusal.into_parts();
    RecipeError::at(RecipeIssue::Grammar(issue), at)
}

fn fragment_refusal(at: Option<crate::token::SpanHandle>) -> RecipeError {
    RecipeError::at(RecipeIssue::FragmentNotGenerated, at)
}

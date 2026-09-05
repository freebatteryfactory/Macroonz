//! How every recipe refusal reads, what it observed, and the one repair it names.

use super::{ExactFunctionIssue, ExactProjectionSeat, RecipeError, RecipeIssue};
use crate::bounded::Bounded;
use crate::diagnostic::{LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair};
use crate::identity::human_projection;
use crate::recipe::types::{RECIPE_FACT, RECIPE_FAMILY};
use crate::recipe::{PROJECTION_LIMIT, RecipeRole};
use core::fmt;

impl fmt::Display for RecipeIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(into)
    }
}

impl RecipeIssue {
    fn write(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InlineModuleRequired => write_inline_module_required(into),
            Self::BakeRequiredLast => write_bake_required_last(into),
            Self::GeneratedNameCollision { name } => write_generated_name_collision(into, name),
            Self::GeneratedNameNotIdentifier { name } => write_generated_identifier(into, name),
            Self::Grammar(issue) => write!(into, "the recipe grammar was not read: {issue}"),
            Self::VocabularyNotFound { name } => write_vocabulary_not_found(into, name),
            Self::VocabularyEmpty { name } => write_vocabulary_empty(into, name),
            Self::VariantNotUnit {
                vocabulary,
                variant,
            } => write_variant_shape(into, vocabulary, variant),
            Self::DuplicateMember { vocabulary, member } => {
                write_duplicate_member(into, vocabulary, member)
            }
            Self::DuplicateVocabulary { name } => write_duplicate_vocabulary(into, name),
            Self::ForeignMember { vocabulary, member } => {
                write_foreign_member(into, vocabulary, member)
            }
            Self::DuplicateTransition { state, event } => {
                write_duplicate_transition(into, state, event)
            }
            Self::DuplicateRelationRow {
                relation,
                left,
                right,
            } => write_duplicate_relation_row(into, relation, left, right),
            Self::DuplicateRelation { name } => write_duplicate_relation(into, name),
            Self::DuplicateCodec { name } => write_duplicate_codec(into, name),
            Self::CodecDeclaration { name, reason } => {
                write!(into, "recipe codec `{name}` was refused: {reason}")
            }
            Self::CodecOwnerNotRecord { codec, owner } => write_codec_owner(into, codec, owner),
            Self::RelationNotFound { name } => {
                write!(into, "the recipe names no relation `{name}`")
            }
            Self::DuplicateRelationPosture { relation } => {
                write_duplicate_relation_posture(into, relation)
            }
            Self::DuplicateRelationQuestion { relation, question } => {
                write_duplicate_relation_question(into, relation, question)
            }
            Self::RelationPostureMismatch {
                relation,
                question,
                required,
                observed,
            } => write_relation_posture_mismatch(into, relation, question, required, observed),
            Self::RelationPostureInapplicable { relation, question } => {
                write_relation_posture_inapplicable(into, relation, question)
            }
            Self::RelationPayloadShapeMismatch {
                relation,
                expected,
                observed,
            } => write_relation_payload_shape(into, relation, expected.name(), observed.name()),
            Self::DuplicateProjection { role } => write_duplicate_projection(into, *role),
            Self::DuplicateRelationTable { relation } => {
                write_duplicate_relation_table(into, relation)
            }
            Self::RelationTableExactRequired { relation } => {
                write_relation_table_exact_required(into, relation)
            }
            Self::RelationTableTransitionUnsupported { relation } => {
                write_relation_table_transition(into, relation)
            }
            Self::ProjectionRequired => {
                into.write_str("a recipe must request at least one projection")
            }
            Self::ProjectionDependencyAbsent { role, required } => {
                write_projection_dependency(into, *role, *required)
            }
            Self::ProjectionSubjectRequired { role, expected } => {
                write!(into, "projection `{}` requires {expected}", role.name())
            }
            Self::AllowedAbsenceNeedsFallback => into.write_str(
                "dispatch with allowed absence requires an explicit caller-owned fallback",
            ),
            Self::HarnessUnavailable { role } => write_harness_unavailable(into, *role),
            Self::SupportAddressRequired => into
                .write_str("an evidence projection requires one explicit exported support address"),
            Self::SupportAddressUnneeded => into.write_str(
                "a support address was declared although no evidence projection was requested",
            ),
            Self::ReplacementUnplanned { role } => write_replacement_unplanned(into, *role),
            Self::DuplicateReplacement { role } => write_duplicate_replacement(into, *role),
            Self::ReplacementRosterUnbounded { observed } => {
                write_replacement_overflow(into, *observed)
            }
            Self::FragmentNotGenerated => into.write_str(
                "captured caller-authored Rust could not be preserved as generated tokens",
            ),
            Self::ExactFunction { seat, issue } => write_exact_function(*seat, *issue, into),
            Self::ExactDispatchBindingAbsent { binding } => {
                write_dispatch_binding_absent(into, binding)
            }
        }
    }

    fn classification(&self) -> RecipeIssueClassification {
        match self {
            Self::InlineModuleRequired
            | Self::BakeRequiredLast
            | Self::VocabularyNotFound { .. }
            | Self::RelationNotFound { .. } => RecipeRepair::RecipeShape.absent(),
            Self::VocabularyEmpty { .. } => RecipeRepair::VocabularyEmpty.absent(),
            Self::SupportAddressRequired => RecipeRepair::Support.absent(),
            Self::ProjectionRequired | Self::ProjectionDependencyAbsent { .. } => {
                RecipeRepair::ProjectionSelection.absent()
            }
            Self::ProjectionSubjectRequired { .. } => RecipeRepair::ProjectionSubject.absent(),
            Self::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded { .. }) => {
                RecipeRepair::SequenceLimit.bound_exceeded()
            }
            Self::HarnessUnavailable { .. } => RecipeRepair::Harness.profile_disagreement(),
            Self::DuplicateMember { .. }
            | Self::DuplicateVocabulary { .. }
            | Self::DuplicateTransition { .. }
            | Self::DuplicateRelationRow { .. }
            | Self::DuplicateRelation { .. }
            | Self::DuplicateCodec { .. }
            | Self::DuplicateRelationPosture { .. }
            | Self::DuplicateRelationQuestion { .. }
            | Self::DuplicateProjection { .. }
            | Self::DuplicateRelationTable { .. }
            | Self::GeneratedNameCollision { .. } => {
                RecipeRepair::Duplicate.identity_disagreement()
            }
            Self::DuplicateReplacement { .. } => RecipeRepair::Replacement.identity_disagreement(),
            Self::GeneratedNameNotIdentifier { .. }
            | Self::Grammar(_)
            | Self::VariantNotUnit { .. }
            | Self::ReplacementUnplanned { .. }
            | Self::FragmentNotGenerated => RecipeRepair::RecipeShape.contract_disagreement(),
            Self::ForeignMember { .. } => RecipeRepair::ForeignMember.contract_disagreement(),
            Self::AllowedAbsenceNeedsFallback => {
                RecipeRepair::AllowedAbsence.contract_disagreement()
            }
            Self::SupportAddressUnneeded => RecipeRepair::Support.contract_disagreement(),
            Self::ReplacementRosterUnbounded { .. } => {
                RecipeRepair::Replacement.contract_disagreement()
            }
            Self::RelationTableExactRequired { .. }
            | Self::ExactFunction {
                seat: ExactProjectionSeat::RelationTable,
                issue:
                    ExactFunctionIssue::FunctionRequired
                    | ExactFunctionIssue::ParameterCount { .. }
                    | ExactFunctionIssue::ParameterBinding { .. },
            } => RecipeRepair::RelationTableSignature.contract_disagreement(),
            Self::RelationTableTransitionUnsupported { .. } => {
                RecipeRepair::RelationTableTransition.contract_disagreement()
            }
            Self::RelationPostureMismatch { .. } => {
                RecipeRepair::RelationPostureMismatch.contract_disagreement()
            }
            Self::RelationPostureInapplicable { .. } => {
                RecipeRepair::RelationPostureInapplicable.contract_disagreement()
            }
            Self::RelationPayloadShapeMismatch { .. } => {
                RecipeRepair::RelationPayloadShape.contract_disagreement()
            }
            Self::CodecDeclaration { .. } => RecipeRepair::CodecDeclaration.contract_disagreement(),
            Self::CodecOwnerNotRecord { .. } => RecipeRepair::CodecOwner.contract_disagreement(),
            Self::ExactFunction {
                seat: ExactProjectionSeat::Dispatch,
                issue: ExactFunctionIssue::BodyRefused,
            } => RecipeRepair::DispatchBody.contract_disagreement(),
            Self::ExactFunction {
                seat: ExactProjectionSeat::Dispatch,
                issue:
                    ExactFunctionIssue::FunctionRequired
                    | ExactFunctionIssue::ParameterCount { .. }
                    | ExactFunctionIssue::ParameterBinding { .. },
            }
            | Self::ExactDispatchBindingAbsent { .. } => {
                RecipeRepair::DispatchSignature.contract_disagreement()
            }
            Self::ExactFunction {
                seat: ExactProjectionSeat::RelationTable,
                issue: ExactFunctionIssue::BodyRefused,
            } => RecipeRepair::RelationTableBody.contract_disagreement(),
        }
    }
}

#[derive(Clone, Copy)]
struct RecipeIssueClassification {
    observed: Observed,
    repair: RecipeRepair,
}

impl RecipeIssueClassification {
    const fn new(observed: Observed, repair: RecipeRepair) -> Self {
        Self { observed, repair }
    }
}

#[derive(Clone, Copy)]
enum RecipeRepair {
    Harness,
    Support,
    ProjectionSelection,
    ProjectionSubject,
    AllowedAbsence,
    DispatchBody,
    DispatchSignature,
    RelationTableSignature,
    RelationTableBody,
    RelationTableTransition,
    Replacement,
    Duplicate,
    RelationPostureMismatch,
    RelationPostureInapplicable,
    RelationPayloadShape,
    CodecDeclaration,
    CodecOwner,
    VocabularyEmpty,
    SequenceLimit,
    RecipeShape,
    ForeignMember,
}

impl RecipeRepair {
    const fn absent(self) -> RecipeIssueClassification {
        RecipeIssueClassification::new(Observed::SeatAbsent, self)
    }

    const fn bound_exceeded(self) -> RecipeIssueClassification {
        RecipeIssueClassification::new(Observed::BoundExceeded, self)
    }

    const fn profile_disagreement(self) -> RecipeIssueClassification {
        RecipeIssueClassification::new(Observed::ProfileDisagreement, self)
    }

    const fn identity_disagreement(self) -> RecipeIssueClassification {
        RecipeIssueClassification::new(Observed::IdentityDisagreement, self)
    }

    const fn contract_disagreement(self) -> RecipeIssueClassification {
        RecipeIssueClassification::new(Observed::ContractDisagreement, self)
    }
}

fn write_inline_module_required(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str("a recipe must contain exactly one inline Rust module")
}

fn write_bake_required_last(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str("the recipe module must end with exactly one `bake!` declaration")
}

fn write_generated_name_collision(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(into, "generated recipe name `{name}` is already occupied")
}

fn write_generated_identifier(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(
        into,
        "generated spelling `{name}` is not one Rust identifier"
    )
}

fn write_vocabulary_not_found(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(into, "the recipe names no authored enum `{name}`")
}

fn write_vocabulary_empty(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(into, "authored enum `{name}` states no variants")
}

fn write_variant_shape(
    into: &mut fmt::Formatter<'_>,
    vocabulary: &str,
    variant: &str,
) -> fmt::Result {
    write!(
        into,
        "authored enum `{vocabulary}` variant `{variant}` is not a unit variant"
    )
}

fn write_duplicate_member(
    into: &mut fmt::Formatter<'_>,
    vocabulary: &str,
    member: &str,
) -> fmt::Result {
    write!(
        into,
        "authored enum `{vocabulary}` states member `{member}` more than once"
    )
}

fn write_duplicate_vocabulary(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(
        into,
        "recipe vocabulary `{name}` is declared more than once"
    )
}

fn write_foreign_member(
    into: &mut fmt::Formatter<'_>,
    vocabulary: &str,
    member: &str,
) -> fmt::Result {
    write!(
        into,
        "a relation row names undeclared `{vocabulary}` member `{member}`"
    )
}

fn write_duplicate_transition(
    into: &mut fmt::Formatter<'_>,
    state: &str,
    event: &str,
) -> fmt::Result {
    write!(
        into,
        "more than one transition occupies state `{state}` and event `{event}`"
    )
}

fn write_duplicate_relation_row(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
    left: &str,
    right: &str,
) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` states endpoint pair `{left}` and `{right}` more than once"
    )
}

fn write_duplicate_relation(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(into, "recipe relation `{name}` is declared more than once")
}

fn write_duplicate_codec(into: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    write!(into, "recipe codec `{name}` is declared more than once")
}

fn write_codec_owner(into: &mut fmt::Formatter<'_>, codec: &str, owner: &str) -> fmt::Result {
    write!(
        into,
        "recipe codec `{codec}` owner `{owner}` is not an authored record struct"
    )
}

fn write_duplicate_relation_posture(into: &mut fmt::Formatter<'_>, relation: &str) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` carries more than one posture block"
    )
}

fn write_duplicate_relation_question(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
    question: &str,
) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` answers structural question `{question}` more than once"
    )
}

fn write_relation_posture_mismatch(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
    question: &str,
    required: &str,
    observed: &str,
) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` requires {question} `{required}` but its rows compute `{observed}`"
    )
}

fn write_relation_posture_inapplicable(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
    question: &str,
) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` asks same-roster question `{question}` across two vocabularies"
    )
}

fn write_relation_payload_shape(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
    expected: &str,
    observed: &str,
) -> fmt::Result {
    write!(
        into,
        "relation `{relation}` mixes `{expected}` and `{observed}` row payload contracts"
    )
}

fn write_duplicate_projection(into: &mut fmt::Formatter<'_>, role: RecipeRole) -> fmt::Result {
    write!(
        into,
        "projection `{}` is requested more than once",
        role.name()
    )
}

fn write_duplicate_relation_table(into: &mut fmt::Formatter<'_>, relation: &str) -> fmt::Result {
    write!(
        into,
        "relation table `{relation}` is requested more than once"
    )
}

fn write_relation_table_exact_required(
    into: &mut fmt::Formatter<'_>,
    relation: &str,
) -> fmt::Result {
    write!(
        into,
        "relation table `{relation}` carries payloads and requires one exact Rust function signature"
    )
}

fn write_relation_table_transition(into: &mut fmt::Formatter<'_>, relation: &str) -> fmt::Result {
    write!(
        into,
        "relation table `{relation}` carries transition payloads owned by the dispatch projection"
    )
}

fn write_projection_dependency(
    into: &mut fmt::Formatter<'_>,
    role: RecipeRole,
    required: RecipeRole,
) -> fmt::Result {
    write!(
        into,
        "projection `{}` requires selected projection `{}`",
        role.name(),
        required.name()
    )
}

fn write_harness_unavailable(into: &mut fmt::Formatter<'_>, role: RecipeRole) -> fmt::Result {
    write!(
        into,
        "projection `{}` requires the facade harness feature, which is unavailable",
        role.name()
    )
}

fn write_replacement_unplanned(into: &mut fmt::Formatter<'_>, role: RecipeRole) -> fmt::Result {
    write!(
        into,
        "a caller-owned projector was supplied for unselected role `{}`",
        role.name()
    )
}

fn write_duplicate_replacement(into: &mut fmt::Formatter<'_>, role: RecipeRole) -> fmt::Result {
    write!(
        into,
        "caller-owned projector role `{}` is replaced more than once",
        role.name()
    )
}

fn write_replacement_overflow(into: &mut fmt::Formatter<'_>, observed: usize) -> fmt::Result {
    write!(
        into,
        "{observed} caller-owned projectors were supplied where at most {PROJECTION_LIMIT} fit"
    )
}

fn write_dispatch_binding_absent(into: &mut fmt::Formatter<'_>, binding: &str) -> fmt::Result {
    write!(
        into,
        "exact dispatch selector `{binding}` does not name one simple parameter binding"
    )
}

fn write_exact_function(
    seat: ExactProjectionSeat,
    issue: ExactFunctionIssue,
    into: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match (seat, issue) {
        (ExactProjectionSeat::Dispatch, ExactFunctionIssue::FunctionRequired) => into.write_str(
            "exact dispatch braces must contain one semicolon-terminated Rust function signature",
        ),
        (ExactProjectionSeat::Dispatch, ExactFunctionIssue::BodyRefused) => into.write_str(
            "exact dispatch cannot carry a caller-authored body because the standard projector owns the relation-accounted body",
        ),
        (ExactProjectionSeat::Dispatch, ExactFunctionIssue::ParameterCount { observed }) => write!(
            into,
            "exact dispatch requires two parameters but the signature states {observed}"
        ),
        (ExactProjectionSeat::Dispatch, ExactFunctionIssue::ParameterBinding { position }) => {
            write!(
                into,
                "exact dispatch parameter {position} must use one simple identifier binding"
            )
        }
        (ExactProjectionSeat::RelationTable, ExactFunctionIssue::FunctionRequired) => {
            exact_relation_table_function(into)
        }
        (ExactProjectionSeat::RelationTable, ExactFunctionIssue::BodyRefused) => {
            exact_relation_table_body(into)
        }
        (ExactProjectionSeat::RelationTable, ExactFunctionIssue::ParameterCount { observed }) => {
            exact_relation_table_count(observed, into)
        }
        (ExactProjectionSeat::RelationTable, ExactFunctionIssue::ParameterBinding { position }) => {
            exact_relation_table_binding(position, into)
        }
    }
}

fn exact_relation_table_function(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str(
        "exact relation-table braces must contain one semicolon-terminated Rust function signature",
    )
}

fn exact_relation_table_body(into: &mut fmt::Formatter<'_>) -> fmt::Result {
    into.write_str(
        "an exact relation table cannot carry a caller-authored body because the standard projector owns the row-accounted body",
    )
}

fn exact_relation_table_count(observed: usize, into: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        into,
        "an exact relation table requires two parameters but the signature states {observed}"
    )
}

fn exact_relation_table_binding(position: usize, into: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        into,
        "exact relation-table parameter {position} must use one simple identifier binding"
    )
}

impl fmt::Display for RecipeError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.issue())
    }
}

impl core::error::Error for RecipeError {}

impl Refused for RecipeError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: crate::diagnostic::Family = RECIPE_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::DeclarationNotRead
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        self.issue().classification().observed
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        let description = match self.issue().classification().repair {
            RecipeRepair::Harness => human_projection!(
                "enable the facade harness feature or remove the harness-owned projection from this recipe"
            ),
            RecipeRepair::Support => human_projection!(
                "state one support address exactly when the recipe selects test-carrier projections"
            ),
            RecipeRepair::ProjectionSelection => human_projection!(
                "select at least one projection and include every projection dependency it names"
            ),
            RecipeRepair::ProjectionSubject => {
                human_projection!("name the vocabulary or typed lowering this projection consumes")
            }
            RecipeRepair::AllowedAbsence => human_projection!(
                "declare typed refusal for absent rows or state an explicit caller-owned fallback before requesting dispatch"
            ),
            RecipeRepair::DispatchBody => human_projection!(
                "remove the exact function body and leave the semicolon-terminated signature for the standard dispatch projector to fill"
            ),
            RecipeRepair::DispatchSignature => human_projection!(
                "write `dispatch { fn apply(state: State, event: Event) -> Result<State, TransitionRefusal>; };` with exactly two simple bindings, or select the state and event bindings before an exact signature that carries additional parameters"
            ),
            RecipeRepair::RelationTableSignature => human_projection!(
                "write `relation_tables { policy { fn lookup(left: Left, right: Right) -> Option<Payload>; }; };` with exactly two simple identifier bindings"
            ),
            RecipeRepair::RelationTableBody => human_projection!(
                "remove the exact function body and leave the semicolon-terminated signature for the standard relation-table projector to fill"
            ),
            RecipeRepair::RelationTableTransition => human_projection!(
                "request dispatch for the transition lowering or select a non-transition relation for a typed relation table"
            ),
            RecipeRepair::Replacement => human_projection!(
                "supply at most one caller-owned projector for each selected recipe role"
            ),
            RecipeRepair::Duplicate => human_projection!(
                "state each authored member, relation endpoint pair, transition seat, projection role, and generated name once"
            ),
            RecipeRepair::RelationPostureMismatch => human_projection!(
                "change the declared relation rows or state the structural posture those rows actually satisfy"
            ),
            RecipeRepair::RelationPostureInapplicable => human_projection!(
                "use one vocabulary on both relation sides before requiring a self-relation or cycle answer"
            ),
            RecipeRepair::RelationPayloadShape => human_projection!(
                "use one unlabeled, path, exact-Rust, or transition payload contract for every row in one relation"
            ),
            RecipeRepair::CodecDeclaration => human_projection!(
                "repair the codec declaration under the existing codec owner's typed contract"
            ),
            RecipeRepair::CodecOwner => human_projection!(
                "name one record-shaped struct authored in the recipe module as this codec's owner"
            ),
            RecipeRepair::VocabularyEmpty => {
                human_projection!("state at least one unit variant in every selected vocabulary")
            }
            RecipeRepair::SequenceLimit => {
                human_projection!("keep each captured sequence at or below its declared magnitude")
            }
            RecipeRepair::RecipeShape => human_projection!(
                "write one inline module whose final bake declaration names only the vocabularies, relations, codecs, postures, evidence, support, and projections the recipe actually uses"
            ),
            RecipeRepair::ForeignMember => human_projection!(
                "name a member declared by the relation endpoint vocabulary or repair that endpoint vocabulary"
            ),
        };
        Bounded::from_array([Repair {
            declared_by: RECIPE_FACT,
            description,
        }])
    }
}

//! Reading one refusal-family declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! /// <documentation>
//! #[refusal(
//!     crate = <binding>,                     // optional; defaults to `threadpak`
//!     family = "<domain>.<family>",
//!     shape = <shape-word>,
//!     order(<Variant> = "<local-key>", ...),
//! )]
//! enum <FamilyName> {
//!     /// <documentation>
//!     <Variant>,
//!     ...
//! }
//! ```
//!
//! - `<binding>` is how the consumer names the machine on its own dependency
//!   list. It is optional because most consumers do not rename it, and it is
//!   captured because some do.
//! - `family` states the family's stable identity. The causes' identities are
//!   DERIVED from it and their local keys, under band 00's canonical key
//!   grammar, so no author writes a shared prefix out by hand.
//! - `<shape-word>` is one of `single_cause`, `issue_collection`,
//!   `inseparable_pair`. The words map onto the machine's own [`FamilyShape`]
//!   roster; this module carries the spelling of the words and not a second
//!   roster of shapes.
//! - `order(...)` states the canonical selection order, required exactly when
//!   the shape is `single_cause` and admitted only then. Its order is the
//!   *selector's* order and need not match the order the variants are written in.
//! - Variants carry nothing but their own names and the documentation written
//!   on them, and a local key is a quoted text with no escape sequence in it.
//!
//! # Documentation is read, not skipped
//!
//! A documentation comment is an ATTRIBUTE by the time a declaration reaches
//! this grammar — `#[doc = "…"]`, one attribute per written line — so what is
//! read here is the form the language already produces, on the family and on
//! every variant, into typed rows the surface carries as declared data.
//! Nothing about it is a special case in the walk: a row is the `#`, the
//! bracket, the `doc` word, the `=`, and the text, read exactly.
//!
//! **Every other attribute is exactly as unread as it was.** The
//! refusal-attribute search still passes over what it does not name at the
//! declaration's own level, and an unrecognized attribute on a VARIANT still
//! refuses on the `#` that opens it, under the cause and at the site it always
//! did. There is no bucket here for attributes nobody claimed.
//!
//! # One captured surface, two authored facts
//!
//! The declaration is read ONCE and named TWICE, and the two names are two
//! readings rather than two accounts.
//!
//! - The **semantic commitment** stands over the declaration's tokens with every
//!   documentation attribute dropped from the walk. It is what an implementation,
//!   a test, or a codec projection is about: the family's shape, its identity,
//!   its binding, and its causes, unmoved by a reworded sentence.
//! - The **documentation commitment** stands over the semantic commitment and
//!   the ordered documentation rows. It is what a documentation projection is
//!   about, and it MOVES when the prose moves — which is the whole reason it is
//!   its own fact.
//!
//! Neither is a fold of the other and neither is a second account of what the
//! content stands on: the rows are cut from the same material the semantic
//! commitment stands over. The normalization below reaches the token home's own
//! canonical encoding through the token home's own roads, so nothing here is a
//! second spelling of what a captured tree's bytes are.
//!
//! Spans enter neither. A handle is the producer's own table index, two
//! producers reading one declaration issue different ones, and the diagnostic
//! rail is where a handle belongs.
//!
//! # Tokens, not text
//!
//! Everything below walks [`CapturedTokenTree`] values.
//! Groups are already groups, so nothing here re-discovers balance, and every
//! refusal names the exact token it was established at rather than a byte
//! somewhere near it.
//!
//! # Refusal precision
//!
//! A real enum whose variant carries a payload is not "not an enum".
//! A struct is not "not an enum". A generic enum is not "not an enum".
//! Each of those is a real declaration meeting a real limit of this grammar, and
//! each gets a cause that says which limit — because a caller told `NotAnEnum`
//! about a perfectly good enum goes looking for the wrong problem.

use super::types::{
    CapturedCause, CapturedCommitments, CapturedDocumentation, CapturedFamilyFacts, CrateBinding,
    DeclaredTrials, DeriveCauseLimit, DocumentedDeclaration, RefusalDeriveCapture,
    RefusalDeriveRefusal, RefusalDeriveSurface, RefusalSite, SHAPE_WORD_INSEPARABLE_PAIR,
    SHAPE_WORD_ISSUE_COLLECTION, SHAPE_WORD_SINGLE_CAUSE, SurfaceCaptureRefusal,
    TrialDeclarationPosture,
};
use crate::plane::{
    AuthoringLimitProfile, CapturedDeclarationSubject, CapturedTokenLimit, ProjectionIdentity,
    ProjectionRole, ProjectionTranscript, encode_length,
};
use crate::test_descriptor::{
    TRIAL_ATTRIBUTE, TrialDeclarationCause, TrialDeclarationRefusal, captured_trials,
};
use crate::token::{
    CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle, TextCapture, TextReadCause,
    TextReadRefusal,
};
use threadpak::refusal::FamilyShape;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

/// Capture one refusal-family declaration from a typed token tree.
///
/// # Errors
///
/// Returns [`RefusalDeriveRefusal`] carrying the established
/// [`RefusalDeriveCapture`] cause and the token it was established at.
pub fn captured(input: &CapturedInput) -> Result<RefusalDeriveSurface, SurfaceCaptureRefusal> {
    let trees: Vec<&CapturedTokenTree> = input.trees().collect();
    if trees.len() > CapturedTokenLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, first_span(&trees)).into());
    }
    let declared = read_enum(&trees)?;
    let attribute = read_attribute(&trees)?;
    let causes = read_causes(&attribute, &declared)?;
    let causes = Bounded::admitted_const(
        causes,
        &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
    )
    .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, declared.body_span))?;
    let DeclaredEnum {
        family_name,
        variants,
        body_span: _,
    } = declared;
    // The family's own rows first, then each variant's, in body order — the
    // order the rows were written, which is the order a reader reads.
    let mut written = read_family_documentation(&trees);
    for variant in variants {
        written.extend(variant.documentation);
    }
    let documentation = Bounded::admitted_const(
        written,
        &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
    )
    // The roster spans the declaration's own level and the body's, so the site
    // is the declaration's opening rather than either level's.
    .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, first_span(&trees)))?;

    // The three commitments, in the order they depend on each other: the
    // semantic one stands over the declaration alone, and the documentation and
    // trial ones each stand over that name and their own material.
    let semantic = semantic_commitment(input, &trees)?;
    let documented = documentation_commitment(semantic, &documentation);
    let trials = read_trials(&trees, semantic, input.issued())?;

    Ok(RefusalDeriveSurface::assembled(
        CapturedFamilyFacts {
            family_name,
            family_id: attribute.family_id,
            binding: attribute.binding,
            shape: attribute.shape,
        },
        causes,
        documentation,
        trials,
        CapturedCommitments::derived(semantic, documented),
    ))
}

/// Read the `#[threadpak_trials(...)]` attribute, where one is declared, into the
/// posture the surface carries.
///
/// # Where the reading happens
///
/// The ATTRIBUTE is found here, because a helper attribute is a fact about the
/// derive's own grammar and this is the road that walks a declaration's
/// attributes. What is INSIDE it is read by the home that owns the vocabulary it
/// states, through [`captured_trials`] — so the door owns the door and the
/// carrier owns the carrier, and neither home carries a copy of the other's law.
///
/// # Errors
///
/// Returns [`TrialDeclarationCause::NotDeclaredOnce`] where the declaration
/// carries the attribute twice — two declarations of one carrier's rows stand
/// beside each other and neither is the one — and
/// [`TrialDeclarationCause::NotBodied`] where the attribute states no
/// parenthesized body at all. Everything past that is the trial grammar's own
/// refusal, carried whole.
fn read_trials(
    trees: &[&CapturedTokenTree],
    semantic: ProjectionIdentity<CapturedDeclarationSubject>,
    issued: u32,
) -> Result<TrialDeclarationPosture, SurfaceCaptureRefusal> {
    let mut found: Option<(&CapturedTokenTree, SpanHandle)> = None;
    for index in 0..trees.len() {
        let Some((bracketed, token)) = attribute_at(trees, index) else {
            continue;
        };
        if bracketed.first().and_then(|head| head.word()) != Some(TRIAL_ATTRIBUTE) {
            continue;
        }
        if found.is_some() {
            return Err(TrialDeclarationRefusal::Grammar {
                cause: TrialDeclarationCause::NotDeclaredOnce,
                at: token,
            }
            .into());
        }
        let Some(body) = bracketed.get(1).copied() else {
            return Err(TrialDeclarationRefusal::Grammar {
                cause: TrialDeclarationCause::NotBodied,
                at: token,
            }
            .into());
        };
        found = Some((body, token));
    }
    let Some((body, token)) = found else {
        return Ok(TrialDeclarationPosture::NotDeclared);
    };
    let Some((CapturedDelimiter::Parenthesis, inner)) = body.group() else {
        return Err(TrialDeclarationRefusal::Grammar {
            cause: TrialDeclarationCause::NotBodied,
            at: token,
        }
        .into());
    };
    let declared: Vec<&CapturedTokenTree> = inner.iter().collect();
    let payload = captured_trials(&declared, token)?;
    let commitment = trial_commitment(semantic, &declared, issued, token)?;
    Ok(TrialDeclarationPosture::Declared(Box::new(
        DeclaredTrials::read(commitment, payload),
    )))
}

/// The TRIAL commitment: what this declaration states about a consumer's test
/// target, over the name of what the declaration IS.
///
/// # Construction
///
/// The identity is derived under [`ProjectionRole::TrialDeclaration`], anchored
/// on the SEMANTIC commitment at its full thirty-two bytes, at position zero,
/// over the trial attribute's own body as one captured tree's canonical bytes.
///
/// The material is the TOKEN home's, through the token home's own encoding, for
/// the reason the semantic commitment's material is: a byte spelling written here
/// would be a second answer to what a captured tree encodes as, and the two would
/// agree until either was edited. Nothing here re-encodes the typed payload the
/// grammar read, because the payload is a READING of exactly these bytes and a
/// second encoding of it would be a second thing to keep true.
///
/// # Errors
///
/// Returns [`RefusalDeriveCapture::Unbounded`] where the attribute's own level
/// would not fit the declared magnitude. The trees came out of a capture that
/// already fit it, so the arm is the checked constructor's, carried honestly
/// rather than assumed away.
fn trial_commitment(
    semantic: ProjectionIdentity<CapturedDeclarationSubject>,
    declared: &[&CapturedTokenTree],
    issued: u32,
    at: SpanHandle,
) -> Result<ProjectionIdentity<CapturedDeclarationSubject>, RefusalDeriveRefusal> {
    let material = CapturedInput::taken(
        declared.iter().map(|tree| (*tree).clone()).collect(),
        issued,
    )
    .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, at))?;
    Ok(ProjectionIdentity::derived(
        ProjectionTranscript::under_projection(
            ProjectionRole::TrialDeclaration,
            &semantic,
            &material.canonical_bytes(),
            0,
        ),
    ))
}

/// The SEMANTIC commitment: what this declaration is, with its prose set aside.
///
/// # Construction
///
/// The declaration's trees are walked once with every documentation attribute
/// dropped, the retained trees are handed back to the token home as a captured
/// input, and the identity is derived under
/// [`ProjectionRole::CapturedDeclaration`], rooted, at position zero, over that
/// input's own canonical bytes.
///
/// The bytes are the TOKEN home's, through the token home's own roads. A byte
/// spelling written here would be a second answer to what a captured tree
/// encodes as, and the two would agree until either was edited.
///
/// # Errors
///
/// Returns [`RefusalDeriveCapture::Unbounded`] where a rebuilt level would not
/// fit the declared magnitude. The normalization only ever REMOVES trees, so no
/// level it rebuilds is wider than the level it read — the arm is the checked
/// constructor's, carried honestly rather than assumed away.
fn semantic_commitment(
    input: &CapturedInput,
    trees: &[&CapturedTokenTree],
) -> Result<ProjectionIdentity<CapturedDeclarationSubject>, RefusalDeriveRefusal> {
    let normalized = undocumented(trees)?;
    // The producer's own handle count travels with the normalized view, so the
    // value states what the producer issued rather than a number this road
    // invented. Nothing reads it: the view is built here, read once for its
    // canonical bytes, and dropped.
    let material = CapturedInput::taken(normalized, input.issued())
        .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, first_span(trees)))?;
    Ok(ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::CapturedDeclaration,
        &material.canonical_bytes(),
        0,
    )))
}

/// One level of the declaration with its documentation attributes dropped, and
/// every group inside it normalized the same way.
///
/// The walk descends into groups because a variant's rows are written inside the
/// enum body, which is ONE tree at the declaration's own level. A normalization
/// that stopped at the top level would leave every variant's prose inside the
/// semantic commitment, and the split would be true of the family's sentences
/// alone.
///
/// # Bounds
///
/// It drops exactly the two attributes this grammar reads as a SECOND fact about
/// the declaration: a `#` followed by a bracket whose body is the `doc = "…"`
/// form, and the trial attribute. Both are declaration material whose meaning is
/// its own reading — what the declaration SAYS, and what it states about a
/// consumer's test target — and each is named under its own commitment instead,
/// so a reworded sentence and an edited trial row both leave the implementation
/// projection's name where it was.
///
/// The `#[refusal(...)]` attribute is RETAINED, because it is what the
/// declaration IS: the family identity, the binding, the shape, and the canonical
/// cause order are the semantic commitment's whole subject.
///
/// An attribute this grammar does not read is retained too, because it is
/// declaration material this home makes no claim about — and a normalization that
/// swallowed it would be deciding that somebody else's attribute carries no
/// meaning.
///
/// # Errors
///
/// Returns [`RefusalDeriveCapture::Unbounded`] at the group a rebuilt level
/// would not fit.
fn undocumented(
    trees: &[&CapturedTokenTree],
) -> Result<Vec<CapturedTokenTree>, RefusalDeriveRefusal> {
    let mut kept: Vec<CapturedTokenTree> = Vec::new();
    let mut at = 0usize;
    while at < trees.len() {
        if attribute_at(trees, at).is_some_and(|(bracketed, _)| read_as_a_second_fact(&bracketed)) {
            at = at.saturating_add(2);
            continue;
        }
        let Some(tree) = trees.get(at) else {
            break;
        };
        kept.push(match tree.group() {
            Some((delimiter, inner)) => {
                let inside: Vec<&CapturedTokenTree> = inner.iter().collect();
                CapturedTokenTree::group_of(
                    delimiter,
                    undocumented(&inside)?,
                    tree.path().clone(),
                    tree.span(),
                )
                .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, tree.span()))?
            }
            None => (*tree).clone(),
        });
        at = at.saturating_add(1);
    }
    Ok(kept)
}

/// The DOCUMENTATION commitment: what this declaration SAYS, over the name of
/// what it is.
///
/// # Construction
///
/// The identity is derived under [`ProjectionRole::DeclarationDocumentation`],
/// anchored on the SEMANTIC commitment at its full thirty-two bytes, at position
/// zero, over the roster's own length followed by every row in the order the
/// walk read them — the family's own rows ahead of the variants', and each
/// variant's in the order its lines were written.
///
/// Total: a declaration that wrote no prose carries an empty roster, which is a
/// stated fact and derives a name of its own. It is not the semantic
/// commitment's name and it is not an absence — two declarations that documented
/// nothing agree here, which is exactly what a documentation projection over
/// either of them should see.
fn documentation_commitment(
    semantic: ProjectionIdentity<CapturedDeclarationSubject>,
    rows: &Bounded<CapturedDocumentation, CapturedTokenLimit>,
) -> ProjectionIdentity<CapturedDeclarationSubject> {
    let mut material = Vec::new();
    encode_length(rows.len(), &mut material);
    for row in rows.iter() {
        row.encode_into(&mut material);
    }
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::DeclarationDocumentation,
        &semantic,
        &material,
        0,
    ))
}

/// Read one declared input from TEXT and capture it — the callable route, which
/// is what makes a diagnostic's reproduction route a real road.
///
/// # Errors
///
/// Returns [`RefusalDeriveRefusal`] for both the read and the capture: a text
/// that cannot be cut into tokens establishes a grammar cause exactly as a
/// declaration that cuts fine and says the wrong thing does.
///
/// The two are the same CAUSE family and different SITES, and the refusal says
/// which: a capture refusal names the token it was established at, and a read
/// refusal names the byte it was born at, because no table exists for it to have
/// a handle into. See [`RefusalSite`].
pub fn captured_text(
    source: &str,
) -> Result<(TextCapture, RefusalDeriveSurface), SurfaceCaptureRefusal> {
    let read = TextCapture::read(source).map_err(text_refusal)?;
    let surface = captured(read.input())?;
    Ok((read, surface))
}

/// The capture cause one text-read refusal establishes, at the byte the read was
/// born carrying.
///
/// # The site
///
/// A read that refused produced no capture, and therefore no span table and no
/// handle: there is nothing for a handle to index, and handle zero in particular
/// would read exactly like an honest handle naming the declaration's first
/// token.
/// So the refusal travels under [`RefusalSite::BeforeCapture`] carrying
/// [`TextReadRefusal::coordinate`] — the byte the cause was established at,
/// which the refusal was born holding and which this road only puts into the
/// coordinate shape the rest of the seam wears.
const fn text_refusal(refusal: TextReadRefusal) -> RefusalDeriveRefusal {
    let cause = match refusal.cause {
        TextReadCause::NotTerminated | TextReadCause::NotEscapeFree => {
            RefusalDeriveCapture::NotKeyed
        }
        TextReadCause::NotBalanced | TextReadCause::NotOpened => RefusalDeriveCapture::NotAnEnum,
        TextReadCause::Unbounded(_) => RefusalDeriveCapture::Unbounded,
    };
    RefusalDeriveRefusal::established(cause, RefusalSite::BeforeCapture(refusal.coordinate()))
}

/// One established capture refusal at one token of the captured declaration.
///
/// Every refusal on this road is post-capture, so every one of them has a real
/// handle the producer issued while capturing, and this is the only road that
/// writes one.
const fn refuse(cause: RefusalDeriveCapture, token: SpanHandle) -> RefusalDeriveRefusal {
    RefusalDeriveRefusal::established(cause, RefusalSite::AtToken(token))
}

/// The handle of the first token, for a refusal about the declaration's opening.
///
/// # Nonclaims
///
/// A declared input carrying no token at all has no first token, and the handle
/// answered for it names nothing.
/// That is not a substitution: the producer's table for an empty capture reaches
/// no handle, so the coordinate seat states
/// [`SiteCoordinate::NotReached`](crate::diagnostics::SiteCoordinate::NotReached)
/// and the reader is told the locating half is missing rather than being sent to
/// a token that is not there.
fn first_span(trees: &[&CapturedTokenTree]) -> SpanHandle {
    trees.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// The item words this grammar recognizes as real Rust declarations that are
/// nevertheless not enums.
///
/// A declaration spelling one of these gets
/// [`RefusalDeriveCapture::UnsupportedDeclarationForm`], never `NotAnEnum`.
const OTHER_ITEM_FORMS: [&str; 8] = [
    "struct", "union", "trait", "fn", "impl", "type", "const", "static",
];

/// The enum declaration as it was read.
struct DeclaredEnum {
    /// The declared family's Rust name.
    family_name: String,
    /// The variants, in the order the body writes them.
    variants: Vec<DeclaredVariant>,
    /// The token the body opens at.
    body_span: SpanHandle,
}

/// One variant as the body declares it: the name it spells, and the
/// documentation written on it.
///
/// The two travel together because they were read together, and because a row
/// names the variant it was written on: a roster of rows assembled beside a
/// roster of names would be two lists joined by position, and position is
/// exactly what an author's edit moves.
struct DeclaredVariant {
    /// The variant's own bare name.
    spelling: String,
    /// The documentation rows written on it, in the order they were written.
    documentation: Vec<CapturedDocumentation>,
}

/// The attribute as it was read.
struct DeclaredAttribute {
    /// How the consumer names the machine.
    binding: CrateBinding,
    /// The declared family identity.
    family_id: String,
    /// The machine's body shape the declared word names.
    shape: FamilyShape,
    /// The token the shape word sits at.
    shape_span: SpanHandle,
    /// The declared order rows, where an order clause was declared, and the
    /// token that clause opens at.
    order: Option<(Vec<OrderedCause>, SpanHandle)>,
}

/// One completely consumed clause inside the closed `refusal` helper grammar.
enum RefusalClause<'trees> {
    /// The consumer crate binding.
    Crate {
        /// The one value token.
        value: &'trees CapturedTokenTree,
        /// The token the key sits at.
        at: SpanHandle,
    },
    /// The refusal family identity.
    Family {
        /// The one value token.
        value: &'trees CapturedTokenTree,
        /// The token the key sits at.
        at: SpanHandle,
    },
    /// The refusal family shape.
    Shape {
        /// The one value token.
        value: &'trees CapturedTokenTree,
        /// The token the key sits at.
        at: SpanHandle,
    },
    /// The `order(...)` clause.
    Order {
        /// The trees inside the order body.
        body: Vec<&'trees CapturedTokenTree>,
        /// The token the clause name sits at.
        at: SpanHandle,
        /// The token the parenthesized body sits at.
        body_at: SpanHandle,
    },
}

/// The closed clause vocabulary used for duplicate and value lookup.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RefusalClauseKey {
    Crate,
    Family,
    Shape,
    Order,
}

impl RefusalClause<'_> {
    /// The closed-roster key this clause declares.
    const fn key(&self) -> RefusalClauseKey {
        match self {
            Self::Crate { .. } => RefusalClauseKey::Crate,
            Self::Family { .. } => RefusalClauseKey::Family,
            Self::Shape { .. } => RefusalClauseKey::Shape,
            Self::Order { .. } => RefusalClauseKey::Order,
        }
    }

    /// The token the clause begins at.
    const fn at(&self) -> SpanHandle {
        match self {
            Self::Crate { at, .. }
            | Self::Family { at, .. }
            | Self::Shape { at, .. }
            | Self::Order { at, .. } => *at,
        }
    }
}

/// One row of a declared `order(...)` clause: the cause it states, and the token
/// its local key sits at.
///
/// The token travels with the row because the refusals about a row are about
/// THAT row — a repeated local key, a spelling the body does not declare — and a
/// refusal that pointed at the enum body instead would name the one place in the
/// declaration that is not the problem.
struct OrderedCause {
    /// The cause this row declares.
    cause: CapturedCause,
    /// The token the row's local key sits at.
    key_span: SpanHandle,
}

/// Read the enum declaration: the item form, the keyword, the name, the profile
/// limits, and the body's variants.
fn read_enum(trees: &[&CapturedTokenTree]) -> Result<DeclaredEnum, RefusalDeriveRefusal> {
    let words: Vec<(usize, &str)> = trees
        .iter()
        .enumerate()
        .filter_map(|(index, tree)| tree.word().map(|word| (index, word)))
        .collect();
    let Some((enum_index, _)) = words.iter().copied().find(|(_, word)| *word == "enum") else {
        if let Some((index, _)) = words
            .iter()
            .copied()
            .find(|(_, word)| OTHER_ITEM_FORMS.contains(word))
        {
            return Err(refuse(
                RefusalDeriveCapture::UnsupportedDeclarationForm,
                span_at(trees, index),
            ));
        }
        return Err(refuse(RefusalDeriveCapture::NotAnEnum, first_span(trees)));
    };
    let name_index = enum_index.saturating_add(1);
    let family_name = trees
        .get(name_index)
        .and_then(|tree| tree.word())
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotNamed, span_at(trees, enum_index)))?
        .to_owned();

    // Everything between the name and the body is a form this profile does not
    // read: generic parameters, and a `where` clause.
    let body_index = (name_index.saturating_add(1)..trees.len())
        .find(|index| {
            trees
                .get(*index)
                .and_then(|tree| tree.group())
                .is_some_and(|(delimiter, _)| delimiter == CapturedDelimiter::Brace)
        })
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotBodied, span_at(trees, name_index)))?;
    if body_index > name_index.saturating_add(1) {
        return Err(refuse(
            RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            span_at(trees, name_index.saturating_add(1)),
        ));
    }

    let body_span = span_at(trees, body_index);
    let body = trees
        .get(body_index)
        .and_then(|tree| tree.group())
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotBodied, body_span))?;
    let body_trees: Vec<&CapturedTokenTree> = body.1.iter().collect();
    let variants = read_variants(&body_trees)?;
    if variants.is_empty() {
        return Err(refuse(RefusalDeriveCapture::NotInhabited, body_span));
    }
    if variants.len() > DeriveCauseLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, body_span));
    }
    Ok(DeclaredEnum {
        family_name,
        variants,
        body_span,
    })
}

/// Read the variants out of one enum body: each one's documentation, and each
/// one's bare name.
fn read_variants(
    body: &[&CapturedTokenTree],
) -> Result<Vec<DeclaredVariant>, RefusalDeriveRefusal> {
    let mut variants: Vec<DeclaredVariant> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            close_variant(&group, &mut variants)?;
            group.clear();
        } else {
            group.push(tree);
        }
    }
    close_variant(&group, &mut variants)?;
    Ok(variants)
}

/// Close one comma-separated group.
///
/// A group opens with the documentation attributes written on the variant and
/// closes on the variant's own bare name.
/// Empty groups are trailing commas; a name followed by anything is a variant
/// carrying a payload — a real variant this grammar does not admit, not a
/// non-enum.
///
/// # What is unchanged here
///
/// An attribute this grammar does not read stops the documentation walk at
/// once, and the group is then closed exactly as it always was: the first token
/// the walk stopped at is that attribute's `#`, it spells no word, and the
/// refusal is [`RefusalDeriveCapture::NotAnEnum`] there.
/// The same answer stands for a group that carries documentation and no name —
/// a row was written on a variant that is not there, and the group still spells
/// no variant.
fn close_variant(
    group: &[&CapturedTokenTree],
    variants: &mut Vec<DeclaredVariant>,
) -> Result<(), RefusalDeriveRefusal> {
    let (written, at) = read_documentation(group);
    let remainder = group.get(at..).unwrap_or_default();
    let Some((first, rest)) = remainder.split_first() else {
        // A group of nothing at all is a trailing comma. A group whose whole
        // content was documentation spells no variant — the rows were written
        // on a variant that is not there — and it is closed on its own first
        // token exactly as an unread attribute is.
        return match group.first() {
            Some(opening) => Err(refuse(RefusalDeriveCapture::NotAnEnum, opening.span())),
            None => Ok(()),
        };
    };
    let Some(word) = first.word() else {
        return Err(refuse(RefusalDeriveCapture::NotAnEnum, first.span()));
    };
    if let Some(extra) = rest.first() {
        return Err(refuse(
            RefusalDeriveCapture::UnsupportedVariantPayload,
            extra.span(),
        ));
    }
    let declared_on = DocumentedDeclaration::Variant(word.to_owned());
    let documentation = written
        .into_iter()
        .map(|(text, token)| CapturedDocumentation::read(declared_on.clone(), text, token))
        .collect();
    variants.push(DeclaredVariant {
        spelling: word.to_owned(),
        documentation,
    });
    Ok(())
}

/// The attribute standing at one position of a walk: the trees inside its
/// bracket, and the token the bracket itself sits at.
///
/// One shape, read once: a `#` and the bracket that follows it. Every other
/// arrangement answers with nothing and leaves the position to whoever was
/// already reading it.
fn attribute_at<'trees>(
    trees: &[&'trees CapturedTokenTree],
    index: usize,
) -> Option<(Vec<&'trees CapturedTokenTree>, SpanHandle)> {
    if trees.get(index).and_then(|hash| hash.punct()) != Some('#') {
        return None;
    }
    let bracket = trees.get(index.saturating_add(1))?;
    match bracket.group() {
        Some((CapturedDelimiter::Bracket, inner)) => Some((inner.iter().collect(), bracket.span())),
        Some(_) | None => None,
    }
}

/// Whether one attribute body is material this grammar names under a commitment
/// of its own rather than inside the semantic one.
///
/// Exactly two: a documentation row, and the trial declaration. Both are
/// declaration material the semantic commitment sets aside, and both are read
/// back through the seat that carries them.
fn read_as_a_second_fact(bracketed: &[&CapturedTokenTree]) -> bool {
    documented_text(bracketed).is_some()
        || bracketed.first().and_then(|head| head.word()) == Some(TRIAL_ATTRIBUTE)
}

/// The text one attribute body states as documentation, where the body is the
/// `doc = "…"` form an ordinary documentation comment produces.
///
/// Exactly that form and no other: three trees, the word `doc`, the assignment,
/// and a text literal. A body that is anything else — a longer one, a `doc`
/// spelled with a list, an attribute somebody else owns — answers with nothing,
/// which is what keeps this road from becoming a bucket for attributes nobody
/// claimed.
fn documented_text<'trees>(bracketed: &[&'trees CapturedTokenTree]) -> Option<&'trees str> {
    if bracketed.len() != 3 {
        return None;
    }
    if bracketed.first().and_then(|named| named.word()) != Some("doc") {
        return None;
    }
    if bracketed.get(1).and_then(|assigned| assigned.punct()) != Some('=') {
        return None;
    }
    bracketed.get(2).and_then(|value| value.text())
}

/// The documentation attributes one walk opens with, and the position the walk
/// stands at once they are read.
///
/// The rows come back as the text and the token they were read at rather than
/// as finished rows, because a row names the declaration it was written on and
/// that name is not known until the declaration itself has been read.
fn read_documentation<'trees>(
    trees: &[&'trees CapturedTokenTree],
) -> (Vec<(&'trees str, SpanHandle)>, usize) {
    let mut written: Vec<(&str, SpanHandle)> = Vec::new();
    let mut at = 0usize;
    while let Some((bracketed, token)) = attribute_at(trees, at) {
        let Some(text) = documented_text(&bracketed) else {
            break;
        };
        written.push((text, token));
        at = at.saturating_add(2);
    }
    (written, at)
}

/// The documentation rows the FAMILY declaration itself carries.
///
/// Every `#[doc = "…"]` at the declaration's own level, in the order it was
/// written. The enum body is ONE tree at this level and is not descended into,
/// so a variant's rows are read where the variant is read and never twice.
fn read_family_documentation(trees: &[&CapturedTokenTree]) -> Vec<CapturedDocumentation> {
    let mut rows: Vec<CapturedDocumentation> = Vec::new();
    for index in 0..trees.len() {
        let Some((bracketed, token)) = attribute_at(trees, index) else {
            continue;
        };
        if let Some(text) = documented_text(&bracketed) {
            rows.push(CapturedDocumentation::read(
                DocumentedDeclaration::Family,
                text,
                token,
            ));
        }
    }
    rows
}

/// The handle of the token at one position of the walk.
///
/// # Nonclaims
///
/// Every caller reaches this with a position the walk already read a token at,
/// so the fallback below is unreachable rather than a default; where a table has
/// no such handle the producer's resolution says so, and this home invents no
/// position for it.
fn span_at(trees: &[&CapturedTokenTree], index: usize) -> SpanHandle {
    trees
        .get(index)
        .map_or(SpanHandle::at(0), |tree| tree.span())
}

/// Read the `#[refusal(...)]` attribute: the crate binding, the family
/// identity, the shape word, and the order clause.
///
/// # Where each refusal sits
///
/// The token the attribute itself sits at stays inside this function: a seat the
/// attribute fails to declare is refused ON the attribute, and every refusal
/// that needs it is raised here.
/// The first token of the whole declared input is a different place, and
/// pointing there for a missing `shape = ...` sends a reader to the item rather
/// than to the attribute that is short a seat.
fn read_attribute(trees: &[&CapturedTokenTree]) -> Result<DeclaredAttribute, RefusalDeriveRefusal> {
    // No attribute at all is a fact about the declaration's opening, and the
    // first token is where a reader starts. Every refusal PAST this line is a
    // fact about the attribute, and names the attribute's own token.
    let bodies = refusal_attribute_bodies(trees)?;
    let Some((body, attribute_span)) = bodies.first() else {
        return Err(refuse(
            RefusalDeriveCapture::NotFamilyDeclared,
            first_span(trees),
        ));
    };
    if let Some((_, duplicate)) = bodies.get(1) {
        return Err(refuse(RefusalDeriveCapture::NotDeclaredOnce, *duplicate));
    }
    let clauses = refusal_clauses(body)?;

    let binding = match assigned_clause(&clauses, RefusalClauseKey::Crate) {
        Some(value) => CrateBinding::declared(
            value
                .word()
                .ok_or_else(|| refuse(RefusalDeriveCapture::NotAClause, value.span()))?,
        ),
        None => CrateBinding::default_binding(),
    };

    let family_value = assigned_clause(&clauses, RefusalClauseKey::Family)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotFamilyDeclared, *attribute_span))?;
    let family_id = family_value
        .text()
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAClause, family_value.span()))?;
    let family_span = family_value.span();
    if !is_family_grammatical(family_id) {
        return Err(refuse(
            RefusalDeriveCapture::NotFamilyGrammatical,
            family_span,
        ));
    }

    let shape_value = assigned_clause(&clauses, RefusalClauseKey::Shape)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotShapeDeclared, *attribute_span))?;
    let word = shape_value
        .word()
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAClause, shape_value.span()))?;
    let shape_span = shape_value.span();
    let shape = admitted_shape(word)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAnAdmittedShape, shape_span))?;

    let order = match order_clause(&clauses) {
        Some((clause, span)) => Some((read_order_rows(clause, span)?, span)),
        None => None,
    };

    Ok(DeclaredAttribute {
        binding,
        family_id: family_id.to_owned(),
        shape,
        shape_span,
        order,
    })
}

/// Every `#[refusal(...)]` body, with the token its attribute sits at.
fn refusal_attribute_bodies<'trees>(
    trees: &[&'trees CapturedTokenTree],
) -> Result<Vec<(Vec<&'trees CapturedTokenTree>, SpanHandle)>, RefusalDeriveRefusal> {
    let mut bodies = Vec::new();
    for (index, tree) in trees.iter().enumerate() {
        let Some((delimiter, inner)) = tree.group() else {
            continue;
        };
        if delimiter != CapturedDelimiter::Bracket
            || index
                .checked_sub(1)
                .and_then(|hash| trees.get(hash))
                .and_then(|hash| hash.punct())
                != Some('#')
        {
            continue;
        }
        let bracketed: Vec<&CapturedTokenTree> = inner.iter().collect();
        let named = bracketed
            .first()
            .and_then(|head| head.word())
            .is_some_and(|word| word == "refusal");
        if !named {
            continue;
        }
        let Some(after_name) = bracketed.get(1) else {
            return Err(refuse(RefusalDeriveCapture::NotAClause, tree.span()));
        };
        let Some((CapturedDelimiter::Parenthesis, body)) = after_name.group() else {
            return Err(refuse(RefusalDeriveCapture::NotAClause, after_name.span()));
        };
        if bracketed.len() != 2 {
            return Err(refuse(
                RefusalDeriveCapture::NotAClause,
                bracketed.get(2).map_or(tree.span(), |extra| extra.span()),
            ));
        }
        bodies.push((body.iter().collect(), tree.span()));
    }
    Ok(bodies)
}

/// Cut and completely account the comma-delimited clauses in one helper body.
fn refusal_clauses<'trees>(
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<RefusalClause<'trees>>, RefusalDeriveRefusal> {
    let mut clauses = Vec::new();
    let mut group = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refuse(RefusalDeriveCapture::NotAClause, tree.span()));
            }
            close_refusal_clause(&group, &mut clauses)?;
            group.clear();
        } else {
            group.push(*tree);
        }
    }
    if !group.is_empty() {
        close_refusal_clause(&group, &mut clauses)?;
    }
    for (index, clause) in clauses.iter().enumerate() {
        if clauses
            .iter()
            .take(index)
            .any(|earlier| earlier.key() == clause.key())
        {
            return Err(refuse(RefusalDeriveCapture::NotDeclaredOnce, clause.at()));
        }
    }
    Ok(clauses)
}

/// Close one complete helper clause.
fn close_refusal_clause<'trees>(
    group: &[&'trees CapturedTokenTree],
    clauses: &mut Vec<RefusalClause<'trees>>,
) -> Result<(), RefusalDeriveRefusal> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(());
    };
    let Some(key) = head.word() else {
        return Err(refuse(RefusalDeriveCapture::NotAClause, head.span()));
    };
    if key == "order" {
        let [body] = rest else {
            return Err(refuse(
                RefusalDeriveCapture::NotAClause,
                rest.first().map_or(head.span(), |tree| tree.span()),
            ));
        };
        let Some((CapturedDelimiter::Parenthesis, inside)) = body.group() else {
            return Err(refuse(RefusalDeriveCapture::NotAClause, body.span()));
        };
        clauses.push(RefusalClause::Order {
            body: inside.iter().collect(),
            at: head.span(),
            body_at: body.span(),
        });
        return Ok(());
    }
    let [assigned_by, value] = rest else {
        return Err(refuse(
            RefusalDeriveCapture::NotAClause,
            rest.first().map_or(head.span(), |tree| tree.span()),
        ));
    };
    if assigned_by.punct() != Some('=') {
        return Err(refuse(RefusalDeriveCapture::NotAClause, assigned_by.span()));
    }
    let clause = match key {
        "crate" => RefusalClause::Crate {
            value,
            at: head.span(),
        },
        "family" => RefusalClause::Family {
            value,
            at: head.span(),
        },
        "shape" => RefusalClause::Shape {
            value,
            at: head.span(),
        },
        _ => {
            return Err(refuse(
                RefusalDeriveCapture::NotADeclarableClause,
                head.span(),
            ));
        }
    };
    clauses.push(clause);
    Ok(())
}

/// The one assigned value for a declared key.
fn assigned_clause<'trees>(
    clauses: &[RefusalClause<'trees>],
    key: RefusalClauseKey,
) -> Option<&'trees CapturedTokenTree> {
    clauses.iter().find_map(|clause| match clause {
        RefusalClause::Crate { value, .. } if key == RefusalClauseKey::Crate => Some(*value),
        RefusalClause::Family { value, .. } if key == RefusalClauseKey::Family => Some(*value),
        RefusalClause::Shape { value, .. } if key == RefusalClauseKey::Shape => Some(*value),
        RefusalClause::Crate { .. }
        | RefusalClause::Family { .. }
        | RefusalClause::Shape { .. }
        | RefusalClause::Order { .. } => None,
    })
}

/// The order body and its own group token, where declared.
fn order_clause<'clauses, 'trees>(
    clauses: &'clauses [RefusalClause<'trees>],
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| match clause {
        RefusalClause::Order { body, body_at, .. } => Some((body.as_slice(), *body_at)),
        RefusalClause::Crate { .. }
        | RefusalClause::Family { .. }
        | RefusalClause::Shape { .. } => None,
    })
}

/// The `Variant = "local-key"` rows inside an order clause.
///
/// Every refusal here names the token the walk was standing on rather than the
/// clause it is inside: a clause of nine rows whose fourth row is missing its
/// `=` is repaired at that `=`, and a refusal at the clause's opening
/// parenthesis would make the reader find it.
fn read_order_rows(
    clause: &[&CapturedTokenTree],
    span: SpanHandle,
) -> Result<Vec<OrderedCause>, RefusalDeriveRefusal> {
    let mut rows: Vec<OrderedCause> = Vec::new();
    let mut index = 0usize;
    while index < clause.len() {
        let named = clause
            .get(index)
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotCovered, span))?;
        let spelling = named
            .word()
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotCovered, named.span()))?;
        let assigned = clause.get(index.saturating_add(1));
        if assigned.and_then(|tree| tree.punct()) != Some('=') {
            return Err(refuse(
                RefusalDeriveCapture::NotCovered,
                assigned.map_or(named.span(), |tree| tree.span()),
            ));
        }
        let value = clause.get(index.saturating_add(2)).ok_or_else(|| {
            refuse(
                RefusalDeriveCapture::NotCovered,
                assigned.map_or(named.span(), |tree| tree.span()),
            )
        })?;
        let local_key = value
            .text()
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotKeyed, value.span()))?;
        if !is_local_key_grammatical(local_key) {
            return Err(refuse(RefusalDeriveCapture::NotKeyed, value.span()));
        }
        rows.push(OrderedCause {
            cause: CapturedCause::read(spelling, local_key),
            key_span: value.span(),
        });
        index = index.saturating_add(3);
        if index < clause.len() {
            let separator = clause.get(index);
            if separator.and_then(|tree| tree.punct()) != Some(',') {
                return Err(refuse(
                    RefusalDeriveCapture::NotCovered,
                    separator.map_or(span, |tree| tree.span()),
                ));
            }
            index = index.saturating_add(1);
        }
    }
    Ok(rows)
}

/// Read the declared causes: the order clause where the shape carries one, and
/// then coverage and distinctness against the body.
///
/// # Where each refusal sits
///
/// A coverage disagreement has two directions and they sit in two places.
/// An ordered ROW naming a variant the body does not declare is a fact about
/// that row, and the refusal names the row's own token; a body VARIANT the
/// clause does not name is a fact about the body, and the refusal names the
/// body. A repeated local key is a fact about the row that repeats it, so the
/// refusal names the SECOND occurrence — the first one is not the problem.
fn read_causes(
    attribute: &DeclaredAttribute,
    declared: &DeclaredEnum,
) -> Result<Vec<CapturedCause>, RefusalDeriveRefusal> {
    let rows: &[OrderedCause] = match (attribute.shape, attribute.order.as_ref()) {
        (FamilyShape::SingleCause, None) => {
            return Err(refuse(
                RefusalDeriveCapture::NotOrderDeclared,
                attribute.shape_span,
            ));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, Some((_, span))) => {
            return Err(refuse(RefusalDeriveCapture::NotOrderAdmitted, *span));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, None) => {
            return Ok(Vec::new());
        }
        (FamilyShape::SingleCause, Some((declared_rows, _))) => declared_rows.as_slice(),
    };
    if let Some(row) = rows
        .iter()
        .find(|row| !body_names_once(&declared.variants, row.cause.spelling()))
    {
        return Err(refuse(RefusalDeriveCapture::NotCovered, row.key_span));
    }
    if declared
        .variants
        .iter()
        .any(|variant| !rows_name_once(rows, variant.spelling.as_str()))
    {
        return Err(refuse(RefusalDeriveCapture::NotCovered, declared.body_span));
    }
    if let Some(row) = first_repeated_key(rows) {
        return Err(refuse(RefusalDeriveCapture::NotDistinct, row.key_span));
    }
    if u16::try_from(rows.len()).is_err() {
        return Err(refuse(RefusalDeriveCapture::Unbounded, declared.body_span));
    }
    Ok(rows.iter().map(|row| row.cause.clone()).collect())
}

/// The machine's body shape one authored word names.
fn admitted_shape(word: &str) -> Option<FamilyShape> {
    match word {
        SHAPE_WORD_SINGLE_CAUSE => Some(FamilyShape::SingleCause),
        SHAPE_WORD_ISSUE_COLLECTION => Some(FamilyShape::IssueCollection),
        SHAPE_WORD_INSEPARABLE_PAIR => Some(FamilyShape::InseparablePair),
        _ => None,
    }
}

/// Whether one text is a single lowercase kebab-case segment.
fn is_local_key_grammatical(key: &str) -> bool {
    !key.is_empty()
        && !key.starts_with('-')
        && !key.ends_with('-')
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Whether one text is two lowercase kebab-case segments joined by a dot.
fn is_family_grammatical(identity: &str) -> bool {
    let mut segments = identity.split('.');
    let domain = segments.next();
    let family = segments.next();
    let extra = segments.next();
    match (domain, family, extra) {
        (Some(domain), Some(family), None) => {
            is_local_key_grammatical(domain) && is_local_key_grammatical(family)
        }
        _ => false,
    }
}

/// Whether the enum body declares one variant of this spelling, exactly once.
///
/// One half of coverage, asked from the ORDER clause's side, so the caller holds
/// the row that asked and can name its token.
fn body_names_once(variants: &[DeclaredVariant], spelling: &str) -> bool {
    variants
        .iter()
        .filter(|variant| variant.spelling == spelling)
        .count()
        == 1
}

/// Whether the order clause names this body variant, exactly once.
///
/// The other half, asked from the BODY's side — a variant the clause never
/// mentions has no row to point at, so the refusal names the body.
fn rows_name_once(rows: &[OrderedCause], variant: &str) -> bool {
    rows.iter()
        .filter(|row| row.cause.spelling() == variant)
        .count()
        == 1
}

/// The first row whose local key an earlier row already declared — the
/// local-uniqueness proof the derive owes, answered with the offending row
/// rather than with a yes or no.
///
/// The SECOND occurrence is what is returned, because that is the row that
/// repeats: the first declaration of a key is a perfectly good row, and sending
/// a reader there would name the one place that is correct.
///
/// Family uniqueness is a different question and is owed to the composition
/// root, which band 00's key grammar says out loud.
fn first_repeated_key(rows: &[OrderedCause]) -> Option<&OrderedCause> {
    rows.iter().enumerate().find_map(|(index, row)| {
        rows.iter()
            .take(index)
            .any(|earlier| earlier.cause.local_key() == row.cause.local_key())
            .then_some(row)
    })
}

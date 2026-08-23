//! The token half of the road: the published stamp's front grammar, the seat
//! module its internal arm transcribes, and the one invocation each covered seat
//! is migrated to.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, every literal is a
//! typed literal whose quoting the tree owns, and no function here composes Rust
//! source. The Rust a person reads is [`crate::token::GeneratedTree`]'s own
//! projection, which is a projection of what is emitted rather than the thing
//! itself.
//!
//! # What is emitted calls nothing
//!
//! The artifact is a `macro_rules!` definition and its body is self-contained
//! tokens. Every path it writes into the machine's own vocabulary begins with
//! `$crate`, so the definition resolves through the crate it is landed in and
//! names these services nowhere — which is the whole reason a stamp inside the
//! machine can exist at all, since the machine carries no dependency edge to
//! this side.
//!
//! # The reach is transported, never copied
//!
//! The stamped item sits one module deeper than the coordinate the caller wrote
//! its reach at. A reach copied straight through would publish the seat to the
//! caller's own parent, so the front arm renders the caller's literal tokens at
//! the caller's coordinate and the TRANSPORTED tokens one level in. Which reach
//! becomes which is [`SeatVisibility::transported`]'s answer and is not decided
//! here.
//!
//! # The one thing the front grammar refuses
//!
//! `macro_rules!` cannot transport an opaque `vis` fragment one module deeper, so
//! a wrapper that captured a whole visibility and forwarded it has handed over
//! something the stamp cannot place. The last arm of every rendered stamp says
//! so with the machine's own compile-time refusal rather than guessing a reach:
//! a guessed reach publishes somebody's private seat, and nothing downstream
//! would report it.

use super::{
    CoupledSeatDeclaration, SeatMint, SeatMintForm, SeatPath, SeatVisibility, StampName,
    StampRenderIssue, TransportedReach,
};
use crate::plane::GeneratedTokenLimit;
use crate::token::{GeneratedDelimiter, GeneratedToken};
use macroonz::ConstLimit;

// ---------------------------------------------------------------------------
// The grammar's own spellings.
// ---------------------------------------------------------------------------

/// The internal arm a readers-only declaration is transcribed through.
pub const TRANSCRIBE_ARM: &str = "transcribe";

/// The internal arm a minting declaration is transcribed through.
pub const TRANSCRIBE_MINTING_ARM: &str = "transcribe_minting";

/// The clause naming the issue roster the seat carries.
pub const OVER_CLAUSE: &str = "over";

/// The first word of the clause naming the magnitude the roster is bounded by.
pub const BOUNDED_CLAUSE: &str = "bounded";

/// The second word of that clause.
pub const BY_CLAUSE: &str = "by";

/// The first word of the clause naming a minting seat's admission profile.
pub const ESTABLISHED_CLAUSE: &str = "established";

/// The second word of that clause, and the whole of the internal arm's own.
pub const UNDER_CLAUSE: &str = "under";

/// The first word of the clause naming the module the seat is seated in.
pub const SEATED_CLAUSE: &str = "seated";

/// The second word of that clause.
pub const IN_CLAUSE: &str = "in";

/// The metavariable the caller's attributes travel in.
pub const NOTE_PARAMETER: &str = "note";

/// The metavariable the refusal family's spelling travels in.
pub const FAMILY_PARAMETER: &str = "family";

/// The metavariable the issue roster travels in.
pub const ISSUE_PARAMETER: &str = "issue";

/// The metavariable the bounding magnitude travels in.
pub const BOUND_PARAMETER: &str = "bound";

/// The metavariable a minting seat's admission profile travels in.
pub const PROFILE_PARAMETER: &str = "profile";

/// The metavariable the seat module's spelling travels in.
pub const HOME_PARAMETER: &str = "home";

/// The metavariable the transported reach travels in.
pub const INTERNAL_REACH_PARAMETER: &str = "internal_reach";

/// The metavariable the caller's own reach travels in.
pub const CALLER_REACH_PARAMETER: &str = "caller_reach";

/// The metavariable the refusing arm catches an opaque reach in.
pub const OPAQUE_REACH_PARAMETER: &str = "opaque_reach";

/// The fragment kind an attribute is matched as.
pub const META_FRAGMENT: &str = "meta";

/// The fragment kind a spelling is matched as.
pub const IDENT_FRAGMENT: &str = "ident";

/// The fragment kind a type is matched as.
pub const TYPE_FRAGMENT: &str = "ty";

/// The fragment kind a visibility is matched as.
pub const VIS_FRAGMENT: &str = "vis";

/// The coupled seat itself — the one value a body carries.
pub const ADMITTED_PREFIX: &str = "AdmittedPrefix";

/// The road a complete examination's report is built by.
pub const EXAMINED_ROAD: &str = "examined_completely";

/// What a body says about its own coverage.
pub const COMPLETION_POSTURE: &str = "CompletionPosture";

/// The non-empty bounded collection a body's issues are read back as.
pub const NON_EMPTY_BOUNDED: &str = "NonEmptyBounded";

/// The compile-time magnitude witness a mint stands under.
pub const POSITIVE_LIMIT: &str = "PositiveLimit";

/// The road that proves a declared magnitude admits an item.
pub const INHABITED_ROAD: &str = "inhabited_under_profile";

/// Which declared bound an enumeration would stop at.
pub const STOP_BOUND: &str = "StopBound";

/// The declared issue bound, which is the bound a seat's own magnitude is.
pub const DECLARED_ISSUE_BOUND: &str = "DeclaredIssueBound";

/// The contract every refusal family realizes.
pub const REFUSAL_FAMILY: &str = "RefusalFamily";

/// The roster of lawful body shapes.
pub const FAMILY_SHAPE: &str = "FamilyShape";

/// The shape a coupled seat always takes.
pub const ISSUE_COLLECTION: &str = "IssueCollection";

/// The family contract's shape seat.
pub const SHAPE_SEAT: &str = "SHAPE";

/// The road a coupled seat's carried material is read back through.
pub const CARRIED_ROAD: &str = "carried";

/// The road a coupled seat's coverage claim is read back through.
pub const COMPLETION_ROAD: &str = "completion";

/// The private seat's own field spelling.
pub const BODY_SEAT: &str = "body";

/// The reader that hands back the established issues.
pub const ISSUES_ROAD: &str = "issues";

/// The reader that hands back the coverage claim.
pub const POSTURE_ROAD: &str = "posture";

/// The mint road a minting seat carries.
pub const ESTABLISHED_ROAD: &str = "established";

// ---------------------------------------------------------------------------
// The sentences the emission documents itself with.
// ---------------------------------------------------------------------------

/// The sentence the published stamp documents itself with.
///
/// Fixed text rather than a composed one: the stamp is written once and covers
/// every seat, so a sentence carrying one family's spelling would put one seat's
/// material into an item every other seat reads.
pub const STAMP_SENTENCE: &str = "Stamps one collection-shaped refusal family as a coupled seat: \
     the private body, its two readers, its mint road where one is declared, and its family \
     declaration, all inside a module of their own. The module's entire content is this stamp's \
     output, so the complete set of roads to the body is the set written here and the compiler is \
     what establishes it.";

/// The sentence the mint road is documented with.
pub const ESTABLISHED_SENTENCE: &str = "The report one complete examination amounts to: the \
     issues it established, and the coverage claim the same construction performs. The posture is \
     the act's rather than the caller's, so a body carrying every issue cannot claim it truncated \
     and a body that dropped issues cannot claim completeness.";

/// The sentence the issue reader is documented with.
pub const ISSUES_SENTENCE: &str = "The established issues — at least one, at most the declared \
     bound.";

/// The sentence the coverage reader is documented with.
pub const POSTURE_SENTENCE: &str = "What this body says about its own coverage.";

/// The sentence the refusing arm answers an opaque reach with.
///
/// It names the front door and no crate path. A consumer may reach this stamp
/// under a name this side never learns, and a sentence spelling one would send a
/// reader to a path their own crate does not have.
pub const OPAQUE_REACH_REFUSAL: &str = "this coupled-seat stamp requires visibility tokens at its \
     public front door; an opaque forwarded `vis` fragment cannot be transported one module \
     deeper, and a reach guessed in its place would publish a private seat nobody declared \
     visible.";

// ---------------------------------------------------------------------------
// The token primitives.
// ---------------------------------------------------------------------------

/// The issue a tree that outgrew the declared token magnitude amounts to.
///
/// One bound, read from one place, by every road in this home.
pub fn unbounded() -> StampRenderIssue {
    StampRenderIssue::StampTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the group carries more
/// tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, StampRenderIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One stamp metavariable, as the two tokens that spell it.
///
/// The `$` is written JOINT so the projection a person reads is `$family` rather
/// than `$ family`; the token pair is the same either way, and nothing parses the
/// projection.
#[must_use]
pub fn metavariable(spelling: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('$'), GeneratedToken::word(spelling)]
}

/// One matcher fragment: the metavariable and the kind it is matched as.
#[must_use]
pub fn fragment(spelling: &str, kind: &str) -> Vec<GeneratedToken> {
    let mut tokens = metavariable(spelling);
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(kind));
    tokens
}

/// One path into the machine's own vocabulary, rooted at the crate the stamp is
/// landed in.
///
/// `$crate` and never a spelled crate name: the stamp is source inside the
/// machine and a consumer may reach that crate under any name, so the one root
/// that always resolves is the expansion's own.
#[must_use]
pub fn machine_path(segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::joint('$'), GeneratedToken::word("crate")];
    for segment in segments {
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment));
    }
    tokens
}

/// One type path a seat declared, spelled as the segments the caller stated.
///
/// No crate root is written in front of it. A seat is invoked from inside the
/// home that declares its issue roster and its magnitude, so what a segment names
/// resolves at the invocation the way the home's own source resolves it.
#[must_use]
pub fn seat_path(path: &SeatPath) -> Vec<GeneratedToken> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for (position, segment) in path.segments().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::joint(':'));
            tokens.push(GeneratedToken::alone(':'));
        }
        tokens.push(GeneratedToken::word(segment.as_str()));
    }
    tokens
}

/// One attribute over the body a caller spelled.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One `#[doc = "…"]` attribute, as the tokens that spell it.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn documentation(sentence: &str) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    attribute(vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// One `#[must_use = "…"]` attribute, as the tokens that spell it.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn obligation(sentence: &str) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    attribute(vec![
        GeneratedToken::word("must_use"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// `#[derive(Debug, Clone, PartialEq, Eq, Hash)]`, as the tokens that spell it.
///
/// The derive set is the STAMP's and not the caller's, exactly as it is for every
/// stamp the machine already carries: a coupled seat is compared, cloned into a
/// diagnostic, shown in a report, and keyed — and nothing about a refusal body
/// needs ordering.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn derive_attribute() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let named = group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word("Debug"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Clone"),
            GeneratedToken::alone(','),
            GeneratedToken::word("PartialEq"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Eq"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Hash"),
        ],
    )?;
    attribute(vec![GeneratedToken::word("derive"), named])
}

/// The literal tokens one declared reach is spelled with, at the coordinate the
/// caller wrote it.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the tokens outgrow the
/// declared token magnitude.
pub fn declared_reach(reach: SeatVisibility) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let scoped = |inside: &str| -> Result<Vec<GeneratedToken>, StampRenderIssue> {
        Ok(vec![
            GeneratedToken::word("pub"),
            group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word(inside)],
            )?,
        ])
    };
    match reach {
        SeatVisibility::Private => Ok(Vec::new()),
        SeatVisibility::SelfReach => scoped("self"),
        SeatVisibility::SuperReach => scoped("super"),
        SeatVisibility::CrateReach => scoped("crate"),
        SeatVisibility::PublicReach => Ok(vec![GeneratedToken::word("pub")]),
    }
}

/// The literal tokens one transported reach is spelled with, at the coordinate
/// one module deeper.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the tokens outgrow the
/// declared token magnitude.
pub fn transported_reach(reach: TransportedReach) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    match reach {
        TransportedReach::SuperReach => Ok(vec![
            GeneratedToken::word("pub"),
            group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("super")],
            )?,
        ]),
        TransportedReach::AncestorReach => Ok(vec![
            GeneratedToken::word("pub"),
            group(
                GeneratedDelimiter::Parenthesis,
                vec![
                    GeneratedToken::word("in"),
                    GeneratedToken::word("super"),
                    GeneratedToken::joint(':'),
                    GeneratedToken::alone(':'),
                    GeneratedToken::word("super"),
                ],
            )?,
        ]),
        TransportedReach::CrateReach => Ok(vec![
            GeneratedToken::word("pub"),
            group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("crate")],
            )?,
        ]),
        TransportedReach::PublicReach => Ok(vec![GeneratedToken::word("pub")]),
    }
}

// ---------------------------------------------------------------------------
// The grammar's shared halves.
// ---------------------------------------------------------------------------

/// `$(#[$note:meta])*` — the caller's attributes, as the matcher reads them.
///
/// The caller's whole attribute list travels rather than a doc sentence and an
/// obligation sentence separately, so an attribute the stamp has no seat for is
/// transported instead of dropped.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the matcher outgrows
/// the declared token magnitude.
pub fn note_matcher() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let inner = group(
        GeneratedDelimiter::Bracket,
        fragment(NOTE_PARAMETER, META_FRAGMENT),
    )?;
    Ok(vec![
        GeneratedToken::joint('$'),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::alone('#'), inner],
        )?,
        GeneratedToken::alone('*'),
    ])
}

/// `$(#[$note])*` — the same attributes, as an expansion writes them back.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the expansion outgrows
/// the declared token magnitude.
pub fn note_forward() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let inner = group(GeneratedDelimiter::Bracket, metavariable(NOTE_PARAMETER))?;
    Ok(vec![
        GeneratedToken::joint('$'),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::alone('#'), inner],
        )?,
        GeneratedToken::alone('*'),
    ])
}

/// The declaration itself, as a matcher: the family, the roster it is over, the
/// magnitude it is bounded by, the admission profile a minting form names, and
/// the module it is seated in.
#[must_use]
pub fn declaration_matcher(form: SeatMintForm) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("struct")];
    tokens.extend(fragment(FAMILY_PARAMETER, IDENT_FRAGMENT));
    tokens.push(GeneratedToken::word(OVER_CLAUSE));
    tokens.extend(fragment(ISSUE_PARAMETER, TYPE_FRAGMENT));
    tokens.push(GeneratedToken::alone(','));
    tokens.push(GeneratedToken::word(BOUNDED_CLAUSE));
    tokens.push(GeneratedToken::word(BY_CLAUSE));
    tokens.extend(fragment(BOUND_PARAMETER, TYPE_FRAGMENT));
    tokens.push(GeneratedToken::alone(','));
    tokens.extend(match form {
        SeatMintForm::ReadersOnly => Vec::new(),
        SeatMintForm::Minting => {
            let mut clause = vec![
                GeneratedToken::word(ESTABLISHED_CLAUSE),
                GeneratedToken::word(UNDER_CLAUSE),
            ];
            clause.extend(fragment(PROFILE_PARAMETER, TYPE_FRAGMENT));
            clause.push(GeneratedToken::alone(','));
            clause
        }
    });
    tokens.push(GeneratedToken::word(SEATED_CLAUSE));
    tokens.push(GeneratedToken::word(IN_CLAUSE));
    tokens.push(GeneratedToken::word("mod"));
    tokens.extend(fragment(HOME_PARAMETER, IDENT_FRAGMENT));
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// The declaration as an expansion writes it back to the internal arm.
///
/// The admission profile is not part of it: a minting form carries the profile in
/// the internal arm's own header, so the declaration that reaches the transcriber
/// has one shape whichever front arm forwarded it.
#[must_use]
pub fn declaration_forward() -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("struct")];
    tokens.extend(metavariable(FAMILY_PARAMETER));
    tokens.push(GeneratedToken::word(OVER_CLAUSE));
    tokens.extend(metavariable(ISSUE_PARAMETER));
    tokens.push(GeneratedToken::alone(','));
    tokens.push(GeneratedToken::word(BOUNDED_CLAUSE));
    tokens.push(GeneratedToken::word(BY_CLAUSE));
    tokens.extend(metavariable(BOUND_PARAMETER));
    tokens.push(GeneratedToken::alone(','));
    tokens.push(GeneratedToken::word(SEATED_CLAUSE));
    tokens.push(GeneratedToken::word(IN_CLAUSE));
    tokens.push(GeneratedToken::word("mod"));
    tokens.extend(metavariable(HOME_PARAMETER));
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

// ---------------------------------------------------------------------------
// The rules.
// ---------------------------------------------------------------------------

/// One rule: a matcher, the arrow, the expansion, and the separator.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the rule outgrows the
/// declared token magnitude.
pub fn rule(
    matcher: Vec<GeneratedToken>,
    expansion: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    Ok(vec![
        group(GeneratedDelimiter::Parenthesis, matcher)?,
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
        group(GeneratedDelimiter::Brace, expansion)?,
        GeneratedToken::alone(';'),
    ])
}

/// One front arm: the reach the caller writes literally, and the forward into the
/// internal arm carrying both reaches as literal tokens.
///
/// The two reaches are rendered here rather than captured, which is what makes
/// the transport exact: the caller's own tokens land at the caller's coordinate,
/// and [`SeatVisibility::transported`]'s answer lands one module in.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the arm outgrows the
/// declared token magnitude.
pub fn front_arm(
    stamp: &StampName,
    reach: SeatVisibility,
    form: SeatMintForm,
) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut matcher = note_matcher()?;
    matcher.extend(declared_reach(reach)?);
    matcher.extend(declaration_matcher(form));

    let arm = match form {
        SeatMintForm::ReadersOnly => TRANSCRIBE_ARM,
        SeatMintForm::Minting => TRANSCRIBE_MINTING_ARM,
    };
    let mut forwarded = vec![
        GeneratedToken::joint('@'),
        GeneratedToken::word(arm),
        group(
            GeneratedDelimiter::Bracket,
            transported_reach(reach.transported())?,
        )?,
        group(GeneratedDelimiter::Bracket, declared_reach(reach)?)?,
    ];
    forwarded.extend(match form {
        SeatMintForm::ReadersOnly => Vec::new(),
        SeatMintForm::Minting => {
            let mut clause = vec![GeneratedToken::word(UNDER_CLAUSE)];
            clause.extend(metavariable(PROFILE_PARAMETER));
            clause.push(GeneratedToken::alone(','));
            clause
        }
    });
    forwarded.extend(note_forward()?);
    forwarded.extend(declaration_forward());

    let mut expansion = machine_path(&[stamp.spelling()]);
    expansion.push(GeneratedToken::alone('!'));
    expansion.push(group(GeneratedDelimiter::Brace, forwarded)?);

    rule(matcher, expansion)
}

/// The seat module one internal arm transcribes, and the re-export that publishes
/// it at the caller's own coordinate.
///
/// The module's ENTIRE content is written here. A `macro_rules!` expansion is
/// closed — no second `mod` block and no hand-written item can be added to a
/// module that exists only inside one — so the complete set of roads to the
/// private body is the set below, and it is the machine's own compiler that
/// establishes it rather than a reader auditing the file the seat landed in.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the module outgrows the
/// declared token magnitude.
pub fn seat_module(form: SeatMintForm) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut body = vec![
        GeneratedToken::word("use"),
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('*'),
        GeneratedToken::alone(';'),
    ];
    body.extend(note_forward()?);
    body.extend(derive_attribute()?);
    body.extend(metavariable(INTERNAL_REACH_PARAMETER));
    body.push(GeneratedToken::word("struct"));
    body.extend(metavariable(FAMILY_PARAMETER));

    let mut seat = vec![GeneratedToken::word(BODY_SEAT), GeneratedToken::alone(':')];
    seat.extend(machine_path(&[ADMITTED_PREFIX]));
    seat.extend(roster_arguments());
    seat.push(GeneratedToken::alone(','));
    body.push(group(GeneratedDelimiter::Brace, seat)?);

    body.push(GeneratedToken::word("impl"));
    body.extend(metavariable(FAMILY_PARAMETER));
    body.push(group(GeneratedDelimiter::Brace, inherent_roads(form)?)?);

    body.push(GeneratedToken::word("impl"));
    body.extend(machine_path(&[REFUSAL_FAMILY]));
    body.push(GeneratedToken::word("for"));
    body.extend(metavariable(FAMILY_PARAMETER));
    body.push(group(GeneratedDelimiter::Brace, family_contract())?);

    let mut tokens = vec![GeneratedToken::word("mod")];
    tokens.extend(metavariable(HOME_PARAMETER));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);

    tokens.extend(metavariable(CALLER_REACH_PARAMETER));
    tokens.push(GeneratedToken::word("use"));
    tokens.extend(metavariable(HOME_PARAMETER));
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(metavariable(FAMILY_PARAMETER));
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

/// One internal arm: the two reaches as bracketed literals, a minting form's
/// admission profile, and the declaration in its one forwarded shape.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the arm outgrows the
/// declared token magnitude.
pub fn transcribe_arm(form: SeatMintForm) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let arm = match form {
        SeatMintForm::ReadersOnly => TRANSCRIBE_ARM,
        SeatMintForm::Minting => TRANSCRIBE_MINTING_ARM,
    };
    let mut matcher = vec![
        GeneratedToken::joint('@'),
        GeneratedToken::word(arm),
        group(
            GeneratedDelimiter::Bracket,
            fragment(INTERNAL_REACH_PARAMETER, VIS_FRAGMENT),
        )?,
        group(
            GeneratedDelimiter::Bracket,
            fragment(CALLER_REACH_PARAMETER, VIS_FRAGMENT),
        )?,
    ];
    matcher.extend(match form {
        SeatMintForm::ReadersOnly => Vec::new(),
        SeatMintForm::Minting => {
            let mut clause = vec![GeneratedToken::word(UNDER_CLAUSE)];
            clause.extend(fragment(PROFILE_PARAMETER, TYPE_FRAGMENT));
            clause.push(GeneratedToken::alone(','));
            clause
        }
    });
    matcher.extend(note_matcher()?);
    matcher.extend(declaration_matcher(SeatMintForm::ReadersOnly));
    rule(matcher, seat_module(form)?)
}

/// The arm that refuses an opaque forwarded reach, in the machine's own
/// compile-time vocabulary.
///
/// One arm per declared form, because the two forms are two grammars: an arm that
/// caught both by swallowing the tail would report a visibility problem for a
/// declaration whose real defect is somewhere else entirely.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the arm outgrows the
/// declared token magnitude.
pub fn refusing_arm(form: SeatMintForm) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut matcher = note_matcher()?;
    matcher.extend(fragment(OPAQUE_REACH_PARAMETER, VIS_FRAGMENT));
    matcher.extend(declaration_matcher(form));

    let mut expansion = GeneratedToken::absolute_path(&["core", "compile_error"]);
    expansion.push(GeneratedToken::alone('!'));
    expansion.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::text(OPAQUE_REACH_REFUSAL)],
    )?);
    rule(matcher, expansion)
}

// ---------------------------------------------------------------------------
// The two artifacts.
// ---------------------------------------------------------------------------

/// The published stamp definition: the exported `macro_rules!` the publication
/// road lands in the machine as visible source.
///
/// The rules are written in one order and it is the order that makes the grammar
/// unambiguous: every minting front arm, then every readers-only front arm, then
/// the two internal arms, then the two refusing arms. A front arm naming no
/// visibility token has to precede the refusing arm whose `vis` fragment also
/// matches nothing, or a private declaration would reach the refusal instead of
/// the transcriber.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the definition outgrows
/// the declared token magnitude.
pub fn stamp_definition(stamp: &StampName) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut rules: Vec<GeneratedToken> = Vec::new();
    for form in [SeatMintForm::Minting, SeatMintForm::ReadersOnly] {
        for reach in SeatVisibility::ALL {
            rules.extend(front_arm(stamp, reach, form)?);
        }
    }
    for form in [SeatMintForm::Minting, SeatMintForm::ReadersOnly] {
        rules.extend(transcribe_arm(form)?);
    }
    for form in [SeatMintForm::Minting, SeatMintForm::ReadersOnly] {
        rules.extend(refusing_arm(form)?);
    }

    let mut tokens = documentation(STAMP_SENTENCE)?;
    tokens.extend(attribute(vec![GeneratedToken::word("macro_export")])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(stamp.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, rules)?);
    Ok(tokens)
}

/// The one invocation one covered seat is migrated to: the caller's prose, the
/// reach it declared, and the four facts the stamp is parameterized by.
///
/// The stamp is reached as `crate::<name>!`, which is how the machine's own homes
/// reach the stamps it already carries: an exported macro lands at the crate root,
/// and a home invoking one from inside that crate names the root it is already in.
///
/// # Errors
///
/// Returns [`StampRenderIssue::StampTreeUnbounded`] where the invocation outgrows
/// the declared token magnitude.
pub fn seat_invocation(
    stamp: &StampName,
    declared: &CoupledSeatDeclaration,
) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut body = documentation(declared.prose().note.as_str())?;
    body.extend(obligation(declared.prose().obligation.as_str())?);
    body.extend(declared_reach(declared.reach())?);
    body.push(GeneratedToken::word("struct"));
    body.push(GeneratedToken::word(declared.names().family()));
    body.push(GeneratedToken::word(OVER_CLAUSE));
    body.extend(seat_path(declared.issue()));
    body.push(GeneratedToken::alone(','));
    body.push(GeneratedToken::word(BOUNDED_CLAUSE));
    body.push(GeneratedToken::word(BY_CLAUSE));
    body.extend(seat_path(declared.bound()));
    body.push(GeneratedToken::alone(','));
    body.extend(match declared.mint() {
        SeatMint::ReadersOnly => Vec::new(),
        SeatMint::EstablishedUnder(profile) => {
            let mut clause = vec![
                GeneratedToken::word(ESTABLISHED_CLAUSE),
                GeneratedToken::word(UNDER_CLAUSE),
            ];
            clause.extend(seat_path(profile));
            clause.push(GeneratedToken::alone(','));
            clause
        }
    });
    body.push(GeneratedToken::word(SEATED_CLAUSE));
    body.push(GeneratedToken::word(IN_CLAUSE));
    body.push(GeneratedToken::word("mod"));
    body.push(GeneratedToken::word(declared.names().home()));
    body.push(GeneratedToken::alone(';'));

    Ok(vec![
        GeneratedToken::word("crate"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(stamp.spelling()),
        GeneratedToken::alone('!'),
        group(GeneratedDelimiter::Brace, body)?,
    ])
}

// ---------------------------------------------------------------------------
// The seat module's own halves.
// ---------------------------------------------------------------------------

/// `<$issue, $bound>` — the two arguments every seat-shaped type takes.
fn roster_arguments() -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('<')];
    tokens.extend(metavariable(ISSUE_PARAMETER));
    tokens.push(GeneratedToken::alone(','));
    tokens.extend(metavariable(BOUND_PARAMETER));
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// The inherent roads a stamped seat carries: the mint where one was declared,
/// then the two readers.
///
/// The readers travel with the mint because they are the same private seat read
/// back, and a form that carries no mint still carries both: a body nobody in the
/// machine can yet assemble is still a body a reader holding one reads.
fn inherent_roads(form: SeatMintForm) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut roads: Vec<GeneratedToken> = match form {
        SeatMintForm::ReadersOnly => Vec::new(),
        SeatMintForm::Minting => mint_road()?,
    };
    roads.extend(issues_road()?);
    roads.extend(posture_road()?);
    Ok(roads)
}

/// The mint road: one complete examination's issues in, the coupled seat out.
fn mint_road() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut arguments = vec![GeneratedToken::word("first"), GeneratedToken::alone(':')];
    arguments.extend(metavariable(ISSUE_PARAMETER));
    arguments.push(GeneratedToken::alone(','));
    arguments.push(GeneratedToken::word("rest"));
    arguments.push(GeneratedToken::alone(':'));
    arguments.extend(GeneratedToken::absolute_path(&["std", "vec", "Vec"]));
    arguments.push(GeneratedToken::alone('<'));
    arguments.extend(metavariable(ISSUE_PARAMETER));
    arguments.push(GeneratedToken::alone('>'));
    arguments.push(GeneratedToken::alone(','));

    // The magnitude is spelled out rather than inferred. The token roster has
    // no arm that writes `_` — it is the wildcard token and not an identifier —
    // and naming `$bound` is the stronger spelling anyway: the witness the mint
    // stands under is the seat's own declared magnitude, said once at the seat
    // and once here, rather than a hole a reader has to resolve.
    let mut admitted = vec![GeneratedToken::alone('&')];
    admitted.extend(machine_path(&[POSITIVE_LIMIT]));
    admitted.push(GeneratedToken::joint(':'));
    admitted.push(GeneratedToken::alone(':'));
    admitted.push(GeneratedToken::alone('<'));
    admitted.extend(metavariable(BOUND_PARAMETER));
    admitted.push(GeneratedToken::alone(','));
    admitted.extend(metavariable(PROFILE_PARAMETER));
    admitted.push(GeneratedToken::alone('>'));
    admitted.push(GeneratedToken::joint(':'));
    admitted.push(GeneratedToken::alone(':'));
    admitted.push(GeneratedToken::word(INHABITED_ROAD));
    admitted.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);

    let mut examined = vec![
        GeneratedToken::word("first"),
        GeneratedToken::alone(','),
        GeneratedToken::word("rest"),
        GeneratedToken::alone(','),
    ];
    examined.extend(admitted);
    examined.push(GeneratedToken::alone(','));
    examined.extend(machine_path(&[STOP_BOUND]));
    examined.push(GeneratedToken::joint(':'));
    examined.push(GeneratedToken::alone(':'));
    examined.push(GeneratedToken::word(DECLARED_ISSUE_BOUND));
    examined.push(GeneratedToken::alone(','));

    let mut built = vec![GeneratedToken::word(BODY_SEAT), GeneratedToken::alone(':')];
    built.extend(machine_path(&[ADMITTED_PREFIX]));
    built.push(GeneratedToken::joint(':'));
    built.push(GeneratedToken::alone(':'));
    built.push(GeneratedToken::word(EXAMINED_ROAD));
    built.push(group(GeneratedDelimiter::Parenthesis, examined)?);
    built.push(GeneratedToken::alone(','));

    let inner = vec![
        GeneratedToken::word("Self"),
        group(GeneratedDelimiter::Brace, built)?,
    ];

    let mut tokens = documentation(ESTABLISHED_SENTENCE)?;
    tokens.extend(attribute(vec![GeneratedToken::word("must_use")])?);
    tokens.extend(metavariable(INTERNAL_REACH_PARAMETER));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(ESTABLISHED_ROAD));
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.push(GeneratedToken::word("Self"));
    tokens.push(group(GeneratedDelimiter::Brace, inner)?);
    Ok(tokens)
}

/// The reader that hands back the established issues, borrowed.
fn issues_road() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let mut returned = vec![GeneratedToken::alone('&')];
    returned.extend(machine_path(&[NON_EMPTY_BOUNDED]));
    returned.extend(roster_arguments());
    reader_road(ISSUES_SENTENCE, ISSUES_ROAD, returned, CARRIED_ROAD)
}

/// The reader that hands back what the body says about its own coverage.
fn posture_road() -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    reader_road(
        POSTURE_SENTENCE,
        POSTURE_ROAD,
        machine_path(&[COMPLETION_POSTURE]),
        COMPLETION_ROAD,
    )
}

/// One reader: the documented, transported-reach `const fn` that forwards to the
/// coupled seat's own road of the same question.
///
/// Both readers are one shape because they are one act: the private seat read
/// back. A seat that kept a loose carry beside a loose posture could not hand
/// these two over, because each borrows out of the same value the other is read
/// off and no third seat exists to read either from.
fn reader_road(
    sentence: &str,
    spelling: &str,
    returned: Vec<GeneratedToken>,
    forwarded: &str,
) -> Result<Vec<GeneratedToken>, StampRenderIssue> {
    let body = vec![
        GeneratedToken::word("self"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(BODY_SEAT),
        GeneratedToken::alone('.'),
        GeneratedToken::word(forwarded),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
    ];
    let mut tokens = documentation(sentence)?;
    tokens.extend(attribute(vec![GeneratedToken::word("must_use")])?);
    tokens.extend(metavariable(INTERNAL_REACH_PARAMETER));
    tokens.push(GeneratedToken::word("const"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::alone('&'), GeneratedToken::word("self")],
    )?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(returned);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// Returns the issue-collection shape implementation every coupled seat realizes.
fn family_contract() -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("const"),
        GeneratedToken::word(SHAPE_SEAT),
        GeneratedToken::alone(':'),
    ];
    tokens.extend(machine_path(&[FAMILY_SHAPE]));
    tokens.push(GeneratedToken::alone('='));
    tokens.extend(machine_path(&[FAMILY_SHAPE]));
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(ISSUE_COLLECTION));
    tokens.push(GeneratedToken::alone(';'));

    tokens
}

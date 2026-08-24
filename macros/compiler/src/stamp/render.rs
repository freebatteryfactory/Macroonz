//! The token half: the definition a pattern is written into, and the one invocation each site is migrated to.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, every sentence is a typed literal whose quoting the tree owns, and nothing here composes Rust source.
//! The Rust a person reads is the tree's own projection, which is a projection of what is emitted rather than the thing itself.
//!
//! # One declaration, both halves
//!
//! A matcher and an invocation are two walks over one declared shape.
//! A literal part is the same tokens on both walks, a seat is a metavariable on one and the site's own material on the other, and the reach coordinate is the only place where the two walks write different things — which is the one thing about a reach that cannot be got right by copying.
//!
//! # What is emitted calls nothing
//!
//! The definition's body is the caller's token material, and this compiler is named nowhere in it.
//! The only tokens written from here are the grammar around that body: the arms, the two bracketed reaches, and the refusal.

use super::{
    Fragment, Part, Pattern, Seat, Seating, Site, Stamp, StampError, TransportedReach, Visibility,
};
use crate::token::{self, GeneratedDelimiter, GeneratedToken};

/// The internal arm a front arm forwards a declaration through.
///
/// Reserved: a shape whose first part is this word would be read as a forwarded declaration rather than as a site's own.
pub const TRANSCRIBE_ARM: &str = "transcribe";

/// The metavariable the site's own reach travels in, for a body to write at the site's coordinate.
pub const DECLARED_REACH: &str = "declared_reach";

/// The metavariable the transported reach travels in, for a body to write one module in.
pub const TRANSPORTED_REACH: &str = "transported_reach";

/// The sentence the refusing arm answers an opaque reach with.
///
/// It names the front door and no crate path: a consumer may reach a stamp under a name this side never learns, and a sentence spelling one would send a reader to a path their own crate does not have.
pub const OPAQUE_REACH_REFUSAL: &str = "this stamp requires visibility tokens at its front door; \
     an opaque forwarded `vis` fragment cannot be transported one module deeper, and a reach \
     guessed in its place would publish an item nobody declared visible.";

/// The metavariable the refusing arm catches an opaque reach in.
const OPAQUE_REACH: &str = "opaque_reach";

/// The fragment a visibility is matched as.
///
/// The one kind [`Fragment`] does not carry: only the two internal seats match one, and each of them is handed literal tokens by the arm that forwarded it.
const VIS_FRAGMENT: &str = "vis";

/// The published definition: the exported `macro_rules!` a publication road lands as visible source.
///
/// # Ordering
///
/// The rules are written in one order and it is the order that makes the grammar unambiguous: every front arm, then the internal arm, then the arm that refuses an opaque reach.
/// A front arm naming no visibility token has to precede that last one, whose `vis` fragment also matches nothing, or a site writing no reach would reach the refusal instead of the body.
///
/// A pattern that gives a reach no coordinate is one rule and nothing else: there is no reach to transport, so there is nothing to forward through and nothing to refuse.
///
/// # Errors
///
/// Returns [`StampError::TokensUnbounded`] where the definition outgrows the declared token magnitude.
pub fn definition(stamp: &Stamp) -> Result<Vec<GeneratedToken>, StampError> {
    let pattern = stamp.pattern();
    let mut rules: Vec<GeneratedToken> = Vec::new();
    if pattern.reaches() {
        for reach in Visibility::ALL {
            rules.extend(front_arm(stamp, *reach)?);
        }
        rules.extend(transcribe_arm(pattern)?);
        rules.extend(refusing_arm(pattern)?);
    } else {
        let expansion = pattern.body().tokens().to_vec();
        rules.extend(rule(matched(pattern, &[])?, expansion)?);
    }

    let mut tokens = token::documentation(pattern.note())?;
    tokens.extend(token::attribute(vec![GeneratedToken::word(
        "macro_export",
    )])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(stamp.name().spelling()));
    tokens.push(token::group(GeneratedDelimiter::Brace, rules)?);
    Ok(tokens)
}

/// The one invocation a covered site is migrated to: the stamp, reached the way that site reaches it, over the shape it was declared in.
///
/// The site's arguments are as many as the shape has seats, settled where the site met its pattern, so the walk fills every seat and runs out at neither end.
///
/// # Errors
///
/// Returns [`StampError::TokensUnbounded`] where the invocation outgrows the declared token magnitude.
pub fn invocation(stamp: &Stamp, site: &Site) -> Result<Vec<GeneratedToken>, StampError> {
    let mut body: Vec<GeneratedToken> = Vec::new();
    let mut supplied = site.arguments().iter();
    for part in stamp.pattern().parts() {
        let written = match part {
            Part::Literal(material) => material.tokens().to_vec(),
            Part::Seat(_) => supplied
                .next()
                .map_or_else(Vec::new, |argument| argument.tokens().to_vec()),
            Part::Reach => declared_reach(site.reach())?,
        };
        body.extend(written);
    }

    let mut tokens = stamp_path(site, stamp.name().spelling());
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(token::group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// How one seat is written back: the tokens a body writes where that seat's material belongs.
///
/// # Errors
///
/// Returns [`StampError::TokensUnbounded`] where the tokens outgrow the declared token magnitude.
pub fn forwarded(seat: &Seat) -> Result<Vec<GeneratedToken>, StampError> {
    match seat.seating() {
        Seating::One(_) => Ok(token::metavariable(seat.name())),
        Seating::Many(_) => repeated(token::metavariable(seat.name()), Some(',')),
        Seating::Attributes => {
            let inner = token::group(
                GeneratedDelimiter::Bracket,
                token::metavariable(seat.name()),
            )?;
            repeated(vec![GeneratedToken::alone('#'), inner], None)
        }
    }
}

/// The literal tokens one declared reach is spelled with, at the coordinate the site wrote it.
///
/// # Errors
///
/// Returns [`StampError::TokensUnbounded`] where the tokens outgrow the declared token magnitude.
pub fn declared_reach(reach: Visibility) -> Result<Vec<GeneratedToken>, StampError> {
    match reach {
        Visibility::Private => Ok(Vec::new()),
        Visibility::Module => scoped(vec![GeneratedToken::word("self")]),
        Visibility::Parent => scoped(vec![GeneratedToken::word("super")]),
        Visibility::Crate => scoped(vec![GeneratedToken::word("crate")]),
        Visibility::Public => Ok(vec![GeneratedToken::word("pub")]),
    }
}

/// The literal tokens one transported reach is spelled with, at the coordinate one module deeper.
///
/// # Errors
///
/// Returns [`StampError::TokensUnbounded`] where the tokens outgrow the declared token magnitude.
pub fn transported_reach(reach: TransportedReach) -> Result<Vec<GeneratedToken>, StampError> {
    match reach {
        TransportedReach::Enclosing => scoped(vec![GeneratedToken::word("super")]),
        TransportedReach::Ancestor => scoped(vec![
            GeneratedToken::word("in"),
            GeneratedToken::word("super"),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word("super"),
        ]),
        TransportedReach::Crate => scoped(vec![GeneratedToken::word("crate")]),
        TransportedReach::Public => Ok(vec![GeneratedToken::word("pub")]),
    }
}

/// One front arm: the reach the site writes literally, and the forward into the internal arm carrying both reaches as literal tokens.
///
/// The two reaches are rendered here rather than captured, which is what makes the transport exact.
fn front_arm(stamp: &Stamp, reach: Visibility) -> Result<Vec<GeneratedToken>, StampError> {
    let pattern = stamp.pattern();
    let declared = declared_reach(reach)?;
    let matcher = matched(pattern, &declared)?;

    let mut carried = vec![
        GeneratedToken::joint('@'),
        GeneratedToken::word(TRANSCRIBE_ARM),
        token::group(
            GeneratedDelimiter::Bracket,
            transported_reach(reach.transported())?,
        )?,
        token::group(GeneratedDelimiter::Bracket, declared)?,
    ];
    carried.extend(restated(pattern)?);

    let mut expansion = token::twin_path("crate", &[stamp.name().spelling()]);
    expansion.push(GeneratedToken::alone('!'));
    expansion.push(token::group(GeneratedDelimiter::Brace, carried)?);
    rule(matcher, expansion)
}

/// The internal arm: the two reaches as bracketed visibilities, the declared shape behind them, and the caller's body as the whole expansion.
fn transcribe_arm(pattern: &Pattern) -> Result<Vec<GeneratedToken>, StampError> {
    let mut matcher = vec![
        GeneratedToken::joint('@'),
        GeneratedToken::word(TRANSCRIBE_ARM),
        token::group(
            GeneratedDelimiter::Bracket,
            fragment_of(TRANSPORTED_REACH, VIS_FRAGMENT),
        )?,
        token::group(
            GeneratedDelimiter::Bracket,
            fragment_of(DECLARED_REACH, VIS_FRAGMENT),
        )?,
    ];
    matcher.extend(matched(pattern, &[])?);
    rule(matcher, pattern.body().tokens().to_vec())
}

/// The arm that refuses an opaque forwarded reach, in the consumer's own compile-time vocabulary.
fn refusing_arm(pattern: &Pattern) -> Result<Vec<GeneratedToken>, StampError> {
    let opaque = fragment_of(OPAQUE_REACH, VIS_FRAGMENT);
    let matcher = matched(pattern, &opaque)?;
    let mut expansion = token::absolute_path(&["core", "compile_error"]);
    expansion.push(GeneratedToken::alone('!'));
    expansion.push(token::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::text(OPAQUE_REACH_REFUSAL)],
    )?);
    rule(matcher, expansion)
}

/// The declared shape as a matcher reads it, with the reach coordinate spelled by whatever arm is asking.
fn matched(pattern: &Pattern, reach: &[GeneratedToken]) -> Result<Vec<GeneratedToken>, StampError> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for part in pattern.parts() {
        let written = match part {
            Part::Literal(material) => material.tokens().to_vec(),
            Part::Seat(seat) => seat_matched(seat)?,
            Part::Reach => reach.to_vec(),
        };
        tokens.extend(written);
    }
    Ok(tokens)
}

/// The declared shape as an expansion writes it back to the internal arm.
///
/// The reach is not part of it: it rides ahead in the two brackets, so the shape reaching the internal arm is one shape whichever front arm forwarded it.
fn restated(pattern: &Pattern) -> Result<Vec<GeneratedToken>, StampError> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for part in pattern.parts() {
        let written = match part {
            Part::Literal(material) => material.tokens().to_vec(),
            Part::Seat(seat) => forwarded(seat)?,
            Part::Reach => Vec::new(),
        };
        tokens.extend(written);
    }
    Ok(tokens)
}

/// How one seat is matched.
fn seat_matched(seat: &Seat) -> Result<Vec<GeneratedToken>, StampError> {
    match seat.seating() {
        Seating::One(fragment) => Ok(fragment_of(seat.name(), fragment.name())),
        Seating::Many(fragment) => repeated(fragment_of(seat.name(), fragment.name()), Some(',')),
        Seating::Attributes => {
            let inner = token::group(
                GeneratedDelimiter::Bracket,
                fragment_of(seat.name(), Fragment::Attribute.name()),
            )?;
            repeated(vec![GeneratedToken::alone('#'), inner], None)
        }
    }
}

/// One matcher fragment: the metavariable and the kind it is matched as.
fn fragment_of(name: &str, kind: &str) -> Vec<GeneratedToken> {
    let mut tokens = token::metavariable(name);
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(kind));
    tokens
}

/// One repetition over the tokens inside it, separated where a separator was stated.
fn repeated(
    inner: Vec<GeneratedToken>,
    separator: Option<char>,
) -> Result<Vec<GeneratedToken>, StampError> {
    let mut tokens = vec![
        GeneratedToken::joint('$'),
        token::group(GeneratedDelimiter::Parenthesis, inner)?,
    ];
    if let Some(mark) = separator {
        tokens.push(GeneratedToken::alone(mark));
    }
    tokens.push(GeneratedToken::alone('*'));
    Ok(tokens)
}

/// One scoped visibility `pub(inside)`.
fn scoped(inside: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, StampError> {
    Ok(vec![
        GeneratedToken::word("pub"),
        token::group(GeneratedDelimiter::Parenthesis, inside)?,
    ])
}

/// One rule: a matcher, the arrow, the expansion, and the separator.
fn rule(
    matcher: Vec<GeneratedToken>,
    expansion: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, StampError> {
    Ok(vec![
        token::group(GeneratedDelimiter::Parenthesis, matcher)?,
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
        token::group(GeneratedDelimiter::Brace, expansion)?,
        GeneratedToken::alone(';'),
    ])
}

/// The path one site invokes its stamp by: the root it named, then the stamp's own spelling.
fn stamp_path(site: &Site, name: &str) -> Vec<GeneratedToken> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for segment in site.root().segments() {
        if !tokens.is_empty() {
            tokens.push(GeneratedToken::joint(':'));
            tokens.push(GeneratedToken::alone(':'));
        }
        tokens.push(GeneratedToken::word(segment.as_str()));
    }
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(name));
    tokens
}

//! The token half: the pin roster, the one gate invocation a carrier's body is, the exported definition around it, and the forwarding address beside it.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose quoting the tree owns, and every brace is a group.
//! No function here composes Rust source; the Rust a person reads is the generated tree's own projection.
//!
//! # The crate a path is rooted at is never spelled
//!
//! Every path a carrier writes begins with the carrier's own root-and-segments METAVARIABLES for the crate it is rooted at, and the consumption target supplies the path once, at the invocation.
//! A consumer that reaches the harness through a facade or renamed dependency gets its own path back, and this home never learns what that path is.
//! The gate's own binding clause receives the very same metavariables, which is what makes that binding load-bearing rather than decorative: the gate proves the path the consumer passed reaches the same declaration the gate's own crate reaches, so a wrong path refuses at the door instead of as an unresolved path somewhere inside a seat.
//!
//! # The pin is a roster of canonical tokens
//!
//! The gate matches TOKENS.
//! A byte string has many spellings of one value and the spelling on this side is a literal writer's choice rather than a declaration anybody made, so a byte string here would be a producer hostage to an escaping convention nobody controls.
//! An unsuffixed integer has exactly one rendering, so the two sides are one token by construction.

use super::super::cargo::{AxisCargo, DeclaredCargo};
use super::super::types::{
    BoundPath, CrateFacing, DeclaringBinding, DeliveryForm, SchemaId, SupportName,
};
use super::ShellName;
use crate::bounded::Overflow;
use crate::request::Door;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, attribute, documentation, group, metavariable,
    segmented_twin_path,
};

/// The gate a carrier's body invokes.
pub const GATE_MACRO: &str = "generated_support";

/// The gate's clause carrying the producer's own expectation.
pub const EXPECTED_CLAUSE: &str = "expected";

/// The suffix naming the repeated path-segment binding beside a crate-facing root binding.
const PATH_SEGMENT_SUFFIX: &str = "_segment";

/// The repeated path-segment binding belonging to one crate-facing root binding.
fn segment_binding(facing: CrateFacing) -> String {
    format!("{}{PATH_SEGMENT_SUFFIX}", facing.name())
}

/// One path rooted at the complete segmented path a carrier binds for this facing.
#[must_use]
pub(crate) fn rooted_path(facing: CrateFacing, segments: &[&str]) -> Vec<GeneratedToken> {
    segmented_twin_path(facing.name(), &segment_binding(facing), segments)
}

/// One path a caller declared, spelled from the crate it was rooted at.
#[must_use]
pub fn rendered_path(path: &BoundPath) -> Vec<GeneratedToken> {
    let segments: Vec<&str> = path.segments().iter().map(String::as_str).collect();
    rooted_path(path.facing(), &segments)
}

/// The producer's expectation, as the bracketed roster of decimal byte values the gate's opening arm matches.
///
/// # Errors
///
/// Returns [`Overflow`] where the roster outgrows the declared token magnitude.
pub fn expectation_roster(expectation: SchemaId) -> Result<GeneratedToken, Overflow> {
    let mut bytes: Vec<GeneratedToken> = Vec::new();
    for byte in expectation.as_bytes() {
        bytes.push(GeneratedToken::number(u64::from(*byte)));
        bytes.push(GeneratedToken::alone(','));
    }
    group(GeneratedDelimiter::Bracket, bytes)
}

/// One `<name>: $<name>:<fragment>,` clause of a carrier's matcher.
#[must_use]
pub fn matched_clause(name: &str, fragment: &str) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone(':')];
    tokens.extend(metavariable(name));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(fragment));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

/// A carrier's matcher: the binding every rendered path is rooted at, and exactly the clauses the declared cargo consumes.
///
/// The binding is asked for always, because every expression a carrier renders is rooted at it and the gate's own clause is what proves the path the consumer passed reaches the right crate.
/// The rest is the declared cargo's own, carried beside the body that spells it — an argument a consumer supplies that nothing spells is a value the plan decided and nothing read.
#[must_use]
pub fn matcher(declared: &AxisCargo<DeclaredCargo>) -> Vec<GeneratedToken> {
    matcher_for(DeclaringBinding::Absent, declared)
}

pub(super) fn matcher_requiring_declaring(
    declared: &AxisCargo<DeclaredCargo>,
) -> Vec<GeneratedToken> {
    matcher_for(DeclaringBinding::Required, declared)
}

fn matcher_for(
    declaring: DeclaringBinding,
    declared: &AxisCargo<DeclaredCargo>,
) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    if declaring == DeclaringBinding::Required {
        tokens.extend(path_matcher(CrateFacing::Declaring));
    }
    tokens.extend(path_matcher(CrateFacing::Harness));
    if let AxisCargo::Carried(cargo) = declared {
        tokens.extend(cargo.matched().tokens().iter().cloned());
    }
    tokens
}

fn path_matcher(facing: CrateFacing) -> Vec<GeneratedToken> {
    let binding = facing.name();
    let segment = segment_binding(facing);
    let mut tokens = vec![GeneratedToken::word(binding), GeneratedToken::alone(':')];
    tokens.extend(metavariable(binding));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word("ident"));
    tokens.push(GeneratedToken::joint('$'));
    tokens.push(GeneratedToken::fixed_group(
        GeneratedDelimiter::Parenthesis,
        [
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::joint('$'),
            GeneratedToken::word(&segment),
            GeneratedToken::alone(':'),
            GeneratedToken::word("ident"),
        ],
    ));
    tokens.push(GeneratedToken::alone('*'));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

/// The gate invocation a carrier's body IS: the producer's expectation, the binding, and the form's coupled pair of seats.
///
/// # Both seats, always
///
/// A seat may be EMPTY and it is still written, because a gate arm that had to match two clause shapes would be two arms and one pin would open two doors.
/// The stamped seat carries material under the address's own grammar, which the gate forwards to its stamp; the opaque seat carries token trees the gate never parses and emits verbatim.
/// They are separate seats because they are two vocabularies: folding the opaque trees in beside the stamped body would hand the stamp items it has no clause for, and standing them outside the invocation would release them on a pin MISMATCH.
///
/// # Errors
///
/// Returns [`Overflow`] where either seat, or the invocation around them, outgrows the declared token magnitude.
pub fn gate_invocation(
    form: DeliveryForm,
    expectation: GeneratedToken,
    stamped: Vec<GeneratedToken>,
    opaque: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let facing = CrateFacing::Harness;
    let binding = facing.name();
    let mut clauses = vec![
        GeneratedToken::word(EXPECTED_CLAUSE),
        GeneratedToken::alone(':'),
        expectation,
        GeneratedToken::alone(','),
        GeneratedToken::word(binding),
        GeneratedToken::alone(':'),
    ];
    clauses.extend(rooted_path(facing, &[]));
    clauses.push(GeneratedToken::alone(','));
    clauses.push(GeneratedToken::word(form.name()));
    clauses.push(GeneratedToken::alone(':'));
    clauses.push(group(GeneratedDelimiter::Brace, stamped)?);
    clauses.push(GeneratedToken::alone(','));
    clauses.push(GeneratedToken::word(form.opaque()));
    clauses.push(GeneratedToken::alone(':'));
    clauses.push(group(GeneratedDelimiter::Brace, opaque)?);
    clauses.push(GeneratedToken::alone(','));
    let mut tokens = rooted_path(facing, &[GATE_MACRO]);
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Brace, clauses)?);
    Ok(tokens)
}

/// The exported carrier: a hidden definition under the mangled name, with one rule matching what the delivery consumes and expanding to the body it guards.
///
/// # Errors
///
/// Returns [`Overflow`] where the definition outgrows the declared token magnitude.
pub fn exported_shell(
    name: &ShellName,
    sentence: &str,
    matched: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = documentation(sentence)?;
    tokens.extend(attribute(vec![
        GeneratedToken::word("doc"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("hidden")],
        )?,
    ])?);
    tokens.extend(attribute(vec![GeneratedToken::word("macro_export")])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(name.spelling()));
    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, matched)?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, body)?);
    rule.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, rule)?);
    Ok(tokens)
}

/// The author-chosen address: an exported definition under the spelling a declaration chose, whose one rule forwards its whole input to the hidden carrier.
///
/// # Why an address exists at all
///
/// The carrier is exported under the plan's identity at full width, so two declarations in one crate can never mint one exported name — and no person can know that spelling before the expansion runs.
/// There is no road from a runtime string to a macro invocation, so a carrier with no second name is a carrier nobody can invoke.
///
/// # What it forwards
///
/// Every semantic token, unread.
/// An ordinary address forwards through the defining crate's own root.
/// Where generated cargo needs declaration-owned items across a target boundary, the address reads the explicitly supplied declaring path only to reach the hidden carrier and forwards that same binding beside every other token.
/// The hidden carrier remains the sole owner of the complete matcher grammar.
///
/// # Errors
///
/// Returns [`Overflow`] where the definition outgrows the declared token magnitude.
pub fn public_alias(
    name: &ShellName,
    address: &SupportName,
    sentence: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    public_alias_for(name, address, sentence, DeclaringBinding::Absent)
}

pub(super) fn public_alias_requiring_declaring(
    name: &ShellName,
    address: &SupportName,
    sentence: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    public_alias_for(name, address, sentence, DeclaringBinding::Required)
}

fn public_alias_for(
    name: &ShellName,
    address: &SupportName,
    sentence: &str,
    declaring: DeclaringBinding,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = documentation(sentence)?;
    tokens.extend(attribute(vec![GeneratedToken::word("macro_export")])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(address.spelling()));

    let (matched, mut forwarded) = match declaring {
        DeclaringBinding::Absent => (repeated_input()?, metavariable("crate")),
        DeclaringBinding::Required => {
            let facing = CrateFacing::Declaring;
            (declaring_input()?, rooted_path(facing, &[]))
        }
    };
    forwarded.push(GeneratedToken::joint(':'));
    forwarded.push(GeneratedToken::alone(':'));
    forwarded.push(GeneratedToken::word(name.spelling()));
    forwarded.push(GeneratedToken::alone('!'));
    let passed = forwarded_input(declaring)?;
    forwarded.push(group(GeneratedDelimiter::Brace, passed)?);

    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, matched)?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, forwarded)?);
    rule.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, rule)?);
    Ok(tokens)
}

fn repeated_input() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut taken = metavariable("input");
    taken.push(GeneratedToken::alone(':'));
    taken.push(GeneratedToken::word("tt"));
    let mut repeated = vec![GeneratedToken::joint('$')];
    repeated.push(group(GeneratedDelimiter::Parenthesis, taken)?);
    repeated.push(GeneratedToken::alone('*'));
    Ok(repeated)
}

fn declaring_input() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut matched = path_matcher(CrateFacing::Declaring);
    matched.extend(repeated_input()?);
    Ok(matched)
}

fn forwarded_input(declaring: DeclaringBinding) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut passed = Vec::new();
    if declaring == DeclaringBinding::Required {
        passed.extend([
            GeneratedToken::word(CrateFacing::Declaring.name()),
            GeneratedToken::alone(':'),
        ]);
        passed.extend(rooted_path(CrateFacing::Declaring, &[]));
        passed.push(GeneratedToken::alone(','));
    }
    passed.push(GeneratedToken::joint('$'));
    passed.push(group(
        GeneratedDelimiter::Parenthesis,
        metavariable("input"),
    )?);
    passed.push(GeneratedToken::alone('*'));
    Ok(passed)
}

/// The sentence the exported carrier documents itself with.
///
/// Composed from the DOOR's producer and from nothing a declaration wrote: the item lands at the root of a consumer's crate, so a sentence carrying owner material would put a declaration's own words somewhere nobody asked for them, and a reader who trips over a mangled name there is owed the name of whoever put it there.
pub(super) fn shell_sentence(door: &Door) -> String {
    let producer = door.producer();
    let namespace = producer.namespace;
    let name = producer.name;
    format!(
        "Generated support carrier from {namespace}/{name}: deferred tokens a consumption \
         target invokes. Hidden and mangled because it is machinery. Its body is one gate \
         invocation, and the gate compares this producer's expected schema identity against \
         the published one before any constructor reaches type checking."
    )
}

/// The sentence the author-chosen address documents itself with.
///
/// On [`shell_sentence`]'s terms.
/// The address's own NAME is owner material — the author chose it — and the sentence beside it says what the item is rather than restating what the author called it.
pub(super) fn alias_sentence(door: &Door) -> String {
    let producer = door.producer();
    let namespace = producer.namespace;
    let name = producer.name;
    format!(
        "Generated support from {namespace}/{name}: invoke this from a consumption target to \
         receive what this declaration states. It forwards every token to the hidden plan-keyed \
         carrier, whose matcher is the grammar and whose gate compares the producer's expected \
         schema identity against the published one."
    )
}

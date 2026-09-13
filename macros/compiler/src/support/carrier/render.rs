//! The token half: a carrier's staged gate invocation, internal transcription and forwarding address.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose quoting the tree owns, and every brace is a group.
//! No function here composes Rust source; the Rust a person reads is the generated tree's own projection.
//!
//! # The crate a path is rooted at is never spelled
//!
//! Every path a carrier writes begins with the carrier's own root-and-segments metavariables for the crate it is rooted at.
//! The consuming target supplies its declaring and consumer paths and selects a harness gate, including through a facade or renamed dependency.
//! The consumer path selects the gate; the gate supplies its own hygienic root and table stamp to the carrier's internal transcription after schema admission.
//! Framework references then use that supplied root, while caller-owned fragments retain their own meaning and resolution.
//!
//! # The pin is a roster of canonical tokens
//!
//! The gate matches TOKENS.
//! A byte string has many spellings of one value and the spelling on this side is a literal writer's choice rather than a declaration anybody made, so a byte string here would be a producer hostage to an escaping convention nobody controls.
//! An unsuffixed integer has exactly one rendering, so the two sides are one token by construction.

use super::super::assembly::SupportAssembly;
use super::super::cargo::{AxisCargo, DeclaredCargo};
use super::super::types::{
    BoundPath, CrateFacing, DeclaringBinding, DeliveryForm, SchemaId, SupportName,
};
use super::ShellName;
use super::types::CarrierRoute;
use crate::bounded::Overflow;
use crate::request::Door;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, attribute, documentation, group, metavariable,
    segmented_twin_path,
};

/// The gate a carrier's public entrance invokes.
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
    tokens.extend(fragment_of(name, fragment));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

/// A complete explicit invocation matcher for a carrier without declaring-crate references.
///
/// The harness path selects a gate, followed by exactly the clauses the declared cargo consumes.
/// The staged shell splits this grammar across its public entrance and admitted transcription.
#[must_use]
pub fn matcher(declared: &AxisCargo<DeclaredCargo>) -> Vec<GeneratedToken> {
    matcher_for(DeclaringBinding::Absent, declared)
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
    let mut tokens = vec![
        GeneratedToken::word(facing.name()),
        GeneratedToken::alone(':'),
    ];
    tokens.extend(path_capture(facing));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

fn path_capture(facing: CrateFacing) -> Vec<GeneratedToken> {
    let binding = facing.name();
    let segment = segment_binding(facing);
    let mut tokens = fragment_of(binding, "ident");
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
    tokens
}

/// Invokes the selected gate with the producer's schema, the delivery continuation and opaque input.
///
/// # Errors
///
/// Returns [`Overflow`] when the complete invocation exceeds the token magnitude.
pub fn gate_invocation(
    form: DeliveryForm,
    expectation: GeneratedToken,
    continuation: Vec<GeneratedToken>,
    input: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut clauses = vec![
        GeneratedToken::word(EXPECTED_CLAUSE),
        GeneratedToken::alone(':'),
        expectation,
        GeneratedToken::alone(','),
        GeneratedToken::word(form.name()),
        GeneratedToken::alone(':'),
    ];
    clauses.extend(continuation);
    clauses.extend([
        GeneratedToken::alone(','),
        GeneratedToken::word("with"),
        GeneratedToken::alone(':'),
        group(GeneratedDelimiter::Brace, input)?,
        GeneratedToken::alone(','),
    ]);
    let mut tokens = rooted_path(CrateFacing::Harness, &[GATE_MACRO]);
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
    shell_definition(name, sentence, alias_rule(matched, body)?)
}

fn shell_definition(
    name: &ShellName,
    sentence: &str,
    rules: Vec<GeneratedToken>,
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
    tokens.extend([
        GeneratedToken::word("macro_rules"),
        GeneratedToken::alone('!'),
        GeneratedToken::word(name.spelling()),
        group(GeneratedDelimiter::Brace, rules)?,
    ]);
    Ok(tokens)
}

pub(super) fn staged_shell(
    name: &ShellName,
    assembly: &SupportAssembly,
    sentence: &str,
    stamped: Vec<GeneratedToken>,
    opaque: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let declaring = assembly.declaring_binding();
    let mut rules = Vec::new();
    let routes: &[CarrierRoute] = match declaring {
        DeclaringBinding::Absent => &[CarrierRoute::DefiningCrate],
        DeclaringBinding::Required => &[
            CarrierRoute::LocalDeclaration,
            CarrierRoute::NamedDeclaration,
        ],
    };
    for route in routes {
        rules.extend(staged_front(name, assembly, *route)?);
    }
    let mut matched = vec![
        GeneratedToken::alone('@'),
        GeneratedToken::word(assembly.form().name()),
    ];
    let harness = path_capture(CrateFacing::Harness);
    matched.push(group(GeneratedDelimiter::Brace, harness)?);
    matched.push(group(
        GeneratedDelimiter::Brace,
        fragment_of("stamp", "path"),
    )?);
    let mut context = Vec::new();
    if declaring == DeclaringBinding::Required {
        context.extend(path_matcher(CrateFacing::Declaring));
    }
    if let AxisCargo::Carried(cargo) = assembly.declared() {
        context.extend(cargo.matched().tokens().iter().cloned());
    }
    matched.push(group(GeneratedDelimiter::Brace, context)?);
    let mut body = Vec::new();
    if !stamped.is_empty() {
        body.extend(metavariable("stamp"));
        body.push(GeneratedToken::alone('!'));
        body.push(group(GeneratedDelimiter::Brace, stamped)?);
    }
    body.extend(opaque);
    rules.extend(alias_rule(matched, body)?);
    shell_definition(name, sentence, rules)
}

fn fragment_of(name: &str, kind: &str) -> Vec<GeneratedToken> {
    let mut tokens = metavariable(name);
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(kind));
    tokens
}

fn staged_front(
    name: &ShellName,
    assembly: &SupportAssembly,
    route: CarrierRoute,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut matched = Vec::new();
    match route {
        CarrierRoute::DefiningCrate => {}
        CarrierRoute::LocalDeclaration => matched.extend([
            GeneratedToken::word("declaring"),
            GeneratedToken::alone(':'),
            GeneratedToken::word("crate"),
            GeneratedToken::alone(','),
        ]),
        CarrierRoute::NamedDeclaration => {
            matched.extend(path_matcher(CrateFacing::Declaring));
        }
    }
    matched.extend(path_matcher(CrateFacing::Harness));
    matched.extend(repeated_input()?);
    let mut callback = match route {
        CarrierRoute::DefiningCrate => metavariable("crate"),
        CarrierRoute::LocalDeclaration => Vec::new(),
        CarrierRoute::NamedDeclaration => rooted_path(CrateFacing::Declaring, &[]),
    };
    if !callback.is_empty() {
        callback.extend([GeneratedToken::joint(':'), GeneratedToken::alone(':')]);
    }
    callback.push(GeneratedToken::word(name.spelling()));
    let context = match route {
        CarrierRoute::DefiningCrate => forwarded_input(DeclaringBinding::Absent)?,
        CarrierRoute::LocalDeclaration => local_forwarded_input()?,
        CarrierRoute::NamedDeclaration => forwarded_input(DeclaringBinding::Required)?,
    };
    let body = gate_invocation(
        assembly.form(),
        expectation_roster(assembly.expectation())?,
        callback,
        context,
    )?;
    alias_rule(matched, body)
}
/// The author-chosen address: an exported definition under the spelling a declaration chose, forwarding its input to the hidden carrier.
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

    let mut rules = Vec::new();
    if declaring == DeclaringBinding::Required {
        let matched = local_declaring_input()?;
        let passed = local_forwarded_input()?;
        let forwarded = alias_call(Vec::new(), name, passed)?;
        rules.extend(alias_rule(matched, forwarded)?);
    }

    let (matched, forwarded) = match declaring {
        DeclaringBinding::Absent => (repeated_input()?, metavariable("crate")),
        DeclaringBinding::Required => {
            let facing = CrateFacing::Declaring;
            (declaring_input()?, rooted_path(facing, &[]))
        }
    };
    let passed = forwarded_input(declaring)?;
    let forwarded = alias_call(forwarded, name, passed)?;
    rules.extend(alias_rule(matched, forwarded)?);
    tokens.push(group(GeneratedDelimiter::Brace, rules)?);
    Ok(tokens)
}

fn alias_call(
    mut root: Vec<GeneratedToken>,
    name: &ShellName,
    passed: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    if !root.is_empty() {
        root.push(GeneratedToken::joint(':'));
        root.push(GeneratedToken::alone(':'));
    }
    root.push(GeneratedToken::word(name.spelling()));
    root.push(GeneratedToken::alone('!'));
    root.push(group(GeneratedDelimiter::Brace, passed)?);
    Ok(root)
}

fn alias_rule(
    matched: Vec<GeneratedToken>,
    forwarded: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, matched)?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, forwarded)?);
    rule.push(GeneratedToken::alone(';'));
    Ok(rule)
}

fn repeated_input() -> Result<Vec<GeneratedToken>, Overflow> {
    let taken = fragment_of("input", "tt");
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

fn local_declaring_input() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut matched = vec![
        GeneratedToken::word(CrateFacing::Declaring.name()),
        GeneratedToken::alone(':'),
        GeneratedToken::word("crate"),
        GeneratedToken::alone(','),
    ];
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

fn local_forwarded_input() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut passed = vec![
        GeneratedToken::word(CrateFacing::Declaring.name()),
        GeneratedToken::alone(':'),
        GeneratedToken::word("crate"),
        GeneratedToken::alone(','),
    ];
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
        "Generated support carrier from {namespace}/{name}: a staged delivery invoked by a \
         consumption target. Its public entrance selects the harness gate, which admits the \
         producer's schema before supplying its own binding to the internal transcription."
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

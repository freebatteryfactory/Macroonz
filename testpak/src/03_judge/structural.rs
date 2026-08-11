//! Lane B — the structural read: what the artifact DECLARES, recovered from an
//! independent parse of the rendered text.
//!
//! # The question lane A cannot be asked
//!
//! Lane A's claim is exactly *the rendered text contains this exact declared
//! textual form*. Every structural question is outside it, and no number of
//! anchors moves it inside: whether the artifact declares an implementation at
//! all, what that implementation targets, which contract it realizes, whether
//! the anchored constant is a MEMBER of it or merely bytes sitting nearby,
//! whether an item nobody planned came along, and whether the same item was
//! emitted twice. A byte scan that answered any of those would have to decide
//! what the text MEANS, and deciding that means implementing the reader's own
//! understanding of Rust — at which point the scan stops being dumb, which was
//! the property that made it worth trusting.
//!
//! # So the parse is somebody else's
//!
//! This lane answers those questions by handing the text to `syn` and reading
//! the tree it hands back. `syn` shares nothing with the producer: not the
//! capture, not the plan, not the renderer, not the token type the renderer
//! writes, not the projection that turns that token tree into text. It is a
//! parser written by people who never saw this machine, and it decides where an
//! item begins, what a path is, and which constants are members of which
//! implementation, by Rust's rules rather than by ours.
//!
//! That is what makes this lane's agreement worth something. Two readers written
//! by the same hands against the same document agree because they share the
//! challenged understanding; agreement with a parser nobody here wrote is not
//! correlated with the renderer at all.
//!
//! # What this lane does NOT claim
//!
//! It reads syntax, and syntax is not meaning. This lane never claims that the
//! artifact TYPECHECKS, that the paths it spells resolve to anything, that the
//! trait it names exists, that the target type exists, that the implementation
//! is coherent, or that any constant evaluates to the value its spelling
//! suggests. `::threadpak::refusal::SomethingElse` is read here as *a different
//! path than the one declared* and never as *no such contract*. Every one of
//! those is lane C's, where `rustc` compiles the artifact and the test reads the
//! constants back AS VALUES.
//!
//! [`StructuralVerdict::Unparsable`] carries the same discipline as lane A's
//! `Unreadable`: it is a failure class, never a skip, and never a softer
//! [`StructuralVerdict::Deviates`]. A caller that folded it into `Conforms`
//! would be asserting over a reading that never happened.

/// One cause row, as the artifact declares it: the stable identity minted for
/// the cause, and the spelling that cause is projected under.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CauseRow {
    /// The stable identity the row states.
    pub identity: String,
    /// The spelling the row states.
    pub spelling: String,
}

/// One trait implementation the artifact declares, as this lane read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationStructure {
    /// The type the implementation targets, spelled as its path.
    pub target: String,
    /// The trait path the implementation realizes, spelled with its leading
    /// `::` when it carries one.
    pub trait_path: String,
    /// The body-shape word the member constant `SHAPE` states, where this
    /// implementation states one.
    pub shape: Option<String>,
    /// The spellings the member constant `SELECTION_ORDER` states, in order,
    /// where this implementation states it.
    pub selection_order: Option<Vec<String>>,
    /// The cause rows the member constant `DECLARED_ORDER` states, in order,
    /// where this implementation states it.
    pub cause_rows: Option<Vec<CauseRow>>,
}

/// Everything this lane recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The trait implementations the artifact declares, in the order it
    /// declares them.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not trait implementations
    /// of a named type. Nothing lawful renders one, so any count above zero is
    /// a finding rather than a detail.
    pub other_items: usize,
}

/// What the caller states the artifact should declare, written independently of
/// the thing under judgement.
///
/// Every roster here is authored by the caller beside the declaration it handed
/// to the producer. Nothing in this structure is obtained by asking the producer
/// what it did.
#[derive(Debug, Clone, Copy)]
pub struct DeclaredStructure<'a> {
    /// The one type every declared implementation targets.
    pub target: &'a str,
    /// The trait paths the artifact declares, in the order it declares them.
    pub traits: &'a [&'a str],
    /// The body-shape word exactly one implementation states.
    pub shape: &'a str,
    /// The cause spellings, in declared order. Both the selection order and the
    /// cause rows are held to this one roster, because both project the same
    /// declared causes.
    pub spellings: &'a [&'a str],
    /// The stable cause identities, in declared order.
    pub identities: &'a [&'a str],
}

/// Which structural fact the artifact and the declaration disagree about.
///
/// One finding, named. A verdict that only said "no" would leave every caller
/// guessing which of seven questions came back wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralDisagreement {
    /// The artifact declares an item that is not a trait implementation of a
    /// named type.
    UnexpectedItem,
    /// One trait-and-target pair is implemented more than once.
    DuplicateImplementation,
    /// The artifact declares a different number of implementations than the
    /// declaration names.
    OutputCardinality,
    /// An implementation targets a type the declaration did not name.
    ImplementationTarget,
    /// An implementation realizes a trait path the declaration did not name, or
    /// names them in another order.
    TraitPath,
    /// The stated body-shape word is not the declared one, or is stated by no
    /// implementation or by more than one.
    FamilyShape,
    /// The stated selection order is not the declared roster, in order.
    SelectionOrder,
    /// The stated cause rows are not the declared identities and spellings, in
    /// order.
    CauseRows,
}

/// What one structural reading concluded.
///
/// Three answers, and none of them is silence — see the module documentation for
/// why `Unparsable` is its own failure class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralVerdict {
    /// The artifact declares exactly what the caller declared it would.
    Conforms,
    /// The artifact and the declaration disagree, about this.
    Deviates(StructuralDisagreement),
    /// The text is not parseable Rust, so nothing structural was read at all.
    Unparsable,
}

/// Read one rendered artifact's structure, or `None` where the text is not
/// parseable Rust.
///
/// `None` is a real answer and never "nothing was wrong": a caller that treated
/// it as conformance would be reporting a verdict about a tree that was never
/// built.
#[must_use]
pub fn structure_of(rendered: &str) -> Option<ArtifactStructure> {
    let file = syn::parse_file(rendered).ok()?;
    let mut implementations = Vec::new();
    let mut other_items = 0usize;
    for item in &file.items {
        match item {
            syn::Item::Impl(declared) => match implementation_structure(declared) {
                Some(structure) => implementations.push(structure),
                None => other_items = other_items.saturating_add(1),
            },
            _ => other_items = other_items.saturating_add(1),
        }
    }
    Some(ArtifactStructure {
        implementations,
        other_items,
    })
}

/// Judge one rendered artifact's structure against an independently declared
/// one.
///
/// **The claim this function supports** is lane B's and only lane B's: the
/// artifact DECLARES these implementations, of these traits, for this target,
/// carrying these members. It says nothing about whether any of it compiles.
#[must_use]
pub fn judge_structure(rendered: &str, declared: &DeclaredStructure<'_>) -> StructuralVerdict {
    let Some(structure) = structure_of(rendered) else {
        return StructuralVerdict::Unparsable;
    };
    match disagreement(&structure, declared) {
        Some(found) => StructuralVerdict::Deviates(found),
        None => StructuralVerdict::Conforms,
    }
}

/// The first structural disagreement between one reading and one declaration.
///
/// The order is deliberate and coarse-to-fine: an artifact carrying an item
/// nobody planned is reported as that, not as whichever member constant the
/// extra item happened to disturb.
fn disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    if structure.other_items > 0 {
        return Some(StructuralDisagreement::UnexpectedItem);
    }
    if duplicated(structure) {
        return Some(StructuralDisagreement::DuplicateImplementation);
    }
    if structure.implementations.len() != declared.traits.len() {
        return Some(StructuralDisagreement::OutputCardinality);
    }
    if structure
        .implementations
        .iter()
        .any(|found| found.target != declared.target)
    {
        return Some(StructuralDisagreement::ImplementationTarget);
    }
    if structure
        .implementations
        .iter()
        .zip(declared.traits.iter())
        .any(|(found, expected)| found.trait_path != *expected)
    {
        return Some(StructuralDisagreement::TraitPath);
    }
    member_disagreement(structure, declared)
}

/// The first disagreement among the members the implementations carry.
fn member_disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    let shapes: Vec<&String> = structure
        .implementations
        .iter()
        .filter_map(|found| found.shape.as_ref())
        .collect();
    if sole(&shapes).is_none_or(|shape| shape.as_str() != declared.shape) {
        return Some(StructuralDisagreement::FamilyShape);
    }

    let orders: Vec<&Vec<String>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.selection_order.as_ref())
        .collect();
    if sole(&orders).is_none_or(|order| !same_spellings(order, declared.spellings)) {
        return Some(StructuralDisagreement::SelectionOrder);
    }

    let rosters: Vec<&Vec<CauseRow>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.cause_rows.as_ref())
        .collect();
    if sole(&rosters).is_none_or(|rows| !same_rows(rows, declared)) {
        return Some(StructuralDisagreement::CauseRows);
    }
    None
}

/// The one member of a one-member roster, or `None` for any other count. Two
/// implementations both stating `SHAPE` is as much a finding as neither doing
/// so, and this is where both become one.
fn sole<'a, T>(found: &'a [&'a T]) -> Option<&'a T> {
    match found.split_first() {
        Some((first, [])) => Some(first),
        _ => None,
    }
}

/// Whether one trait-and-target pair is implemented more than once.
fn duplicated(structure: &ArtifactStructure) -> bool {
    structure
        .implementations
        .iter()
        .enumerate()
        .any(|(position, found)| {
            structure
                .implementations
                .iter()
                .skip(position.saturating_add(1))
                .any(|other| other.target == found.target && other.trait_path == found.trait_path)
        })
}

/// Whether one read roster is the declared roster, spelling for spelling and
/// position for position.
fn same_spellings(read: &[String], declared: &[&str]) -> bool {
    read.len() == declared.len()
        && read
            .iter()
            .zip(declared.iter())
            .all(|(found, expected)| found == expected)
}

/// Whether the read cause rows are the declared identities and spellings, in
/// order. Both columns are held: a roster keeping every spelling and recycling
/// one identity is as wrong as a permuted one.
fn same_rows(read: &[CauseRow], declared: &DeclaredStructure<'_>) -> bool {
    read.len() == declared.identities.len()
        && read.len() == declared.spellings.len()
        && read
            .iter()
            .zip(declared.identities.iter().zip(declared.spellings.iter()))
            .all(|(row, (identity, spelling))| {
                row.identity == *identity && row.spelling == *spelling
            })
}

/// One trait implementation, read out of the tree, or `None` where the item is
/// an implementation of something this lane cannot name — an inherent impl, or
/// one whose target is not a plain path.
fn implementation_structure(declared: &syn::ItemImpl) -> Option<ImplementationStructure> {
    let (path, _) = declared.trait_.as_ref()?;
    let target = type_path(&declared.self_ty)?;
    let mut shape = None;
    let mut selection_order = None;
    let mut cause_rows = None;
    for member in &declared.items {
        let syn::ImplItem::Const(constant) = member else {
            continue;
        };
        match constant.ident.to_string().as_str() {
            "SHAPE" => shape = last_segment(&constant.expr),
            "SELECTION_ORDER" => selection_order = string_list(&constant.expr),
            "DECLARED_ORDER" => cause_rows = cause_row_list(&constant.expr),
            _ => {}
        }
    }
    Some(ImplementationStructure {
        target,
        trait_path: path_spelling(path),
        shape,
        selection_order,
        cause_rows,
    })
}

/// One path, spelled back with its segments and its leading `::`.
fn path_spelling(path: &syn::Path) -> String {
    let mut spelled = String::new();
    if path.leading_colon.is_some() {
        spelled.push_str("::");
    }
    for (position, segment) in path.segments.iter().enumerate() {
        if position > 0 {
            spelled.push_str("::");
        }
        spelled.push_str(&segment.ident.to_string());
    }
    spelled
}

/// The path one type spells, or `None` where the type is not a plain path.
fn type_path(declared: &syn::Type) -> Option<String> {
    match declared {
        syn::Type::Path(typed) => Some(path_spelling(&typed.path)),
        _ => None,
    }
}

/// The last segment of a path expression — the variant word of a path-spelled
/// constant.
fn last_segment(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Path(spelled) = expression else {
        return None;
    };
    spelled
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

/// The string literals of a `&[…]` expression, in order.
fn string_list(expression: &syn::Expr) -> Option<Vec<String>> {
    array_elements(expression)?.map(string_literal).collect()
}

/// The cause rows of a `Type::declared(&[…])` expression, in order.
fn cause_row_list(expression: &syn::Expr) -> Option<Vec<CauseRow>> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    array_elements(call.args.first()?)?.map(cause_row).collect()
}

/// The elements of a `&[…]` expression.
fn array_elements(expression: &syn::Expr) -> Option<syn::punctuated::Iter<'_, syn::Expr>> {
    let syn::Expr::Reference(reference) = expression else {
        return None;
    };
    let syn::Expr::Array(array) = reference.expr.as_ref() else {
        return None;
    };
    Some(array.elems.iter())
}

/// One `Row::declared(Identity::declared("…"), "…")` element, as its two
/// columns.
fn cause_row(expression: &syn::Expr) -> Option<CauseRow> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    let mut arguments = call.args.iter();
    let minted = arguments.next()?;
    let spelling = string_literal(arguments.next()?)?;
    let syn::Expr::Call(identity) = minted else {
        return None;
    };
    Some(CauseRow {
        identity: string_literal(identity.args.first()?)?,
        spelling,
    })
}

/// The value of a string-literal expression.
fn string_literal(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Lit(literal) = expression else {
        return None;
    };
    let syn::Lit::Str(text) = &literal.lit else {
        return None;
    };
    Some(text.value())
}

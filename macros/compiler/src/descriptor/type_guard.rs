//! The descriptor home's invariant nucleus: every road that reaches a private field of the shared vocabulary.
//!
//! A name is parsed here, so a reference that names nothing is not a value anybody can hold.
//! A spelling is admitted here, so a rendered identifier that a consumer's compiler would read as something else is unwritable.
//! A path is rooted here, so an expression naming a crate and nothing in it does not exist.
//! A composition is built here, after the duplicate scan agreed.

use super::super::composition::doubled_providers;
use super::{
    Binding, BoundPath, COMPOSITION_ISSUE_LIMIT, CaptureCause, CaptureIssue, Composition,
    CompositionError, CompositionIssue, DeclarationError, DirectBinding, FunctionName, Grammar,
    HelperRefusal, ModuleName, Name, PATH_SEGMENT_LIMIT, PROVIDER_LIMIT, Provider, Seat,
    SupportName, TypeName,
};
use crate::bounded::{Capped, Capping, NonEmpty};
use crate::token::SpanHandle;

impl Grammar {
    /// The grammar written in one named helper attribute.
    #[must_use]
    pub const fn named(attribute: &'static str) -> Self {
        Self { attribute }
    }
}

impl DeclarationError {
    /// The refusal one overrun roster amounts to, with both counts widened once.
    pub fn unbounded(seat: Seat, bound: usize, observed: usize) -> Self {
        Self::Unbounded {
            seat,
            bound: u64::try_from(bound).unwrap_or(u64::MAX),
            observed: u64::try_from(observed).unwrap_or(u64::MAX),
        }
    }
}

impl Name {
    /// This name, from the owner that declares a spelling and the spelling.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem, so exactly one cause is true of any refused name.
    pub fn named(namespace: &str, stem: &str) -> Result<Self, DeclarationError> {
        if namespace.is_empty() {
            return Err(DeclarationError::NamespaceEmpty);
        }
        if stem.is_empty() {
            return Err(DeclarationError::StemEmpty);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            stem: stem.to_owned(),
        })
    }

    /// The owner that declares the spelling.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    /// The spelling itself.
    #[must_use]
    pub fn stem(&self) -> &str {
        self.stem.as_str()
    }
}

/// Declares one rendered-identifier newtype's checked constructor and its reading.
///
/// The check and the refusal are one fact for every spelling written in identifier position, and four copies of it would be four things to keep true.
/// What differs between the rows is the seat, which is the type the row declares.
macro_rules! rendered_identifiers {
    ($( $name:ident, $reading:literal );+ $(;)?) => {
        $(
            impl $name {
                #[doc = concat!("This ", $reading, ", from the spelling an author wrote.")]
                ///
                /// # Errors
                ///
                /// Returns [`DeclarationError::NotAnIdentifier`] where the spelling cannot name a rendered item — not one Rust identifier, or a keyword the language already took: it is written into a consumer's target in identifier position, and either disagreement renders tokens that compiler reads as something else.
                pub fn declared(spelling: &str) -> Result<Self, DeclarationError> {
                    if rendered_name(spelling) {
                        Ok(Self(spelling.to_owned()))
                    } else {
                        Err(DeclarationError::NotAnIdentifier)
                    }
                }

                #[doc = concat!("The spelling this ", $reading, " carries.")]
                #[must_use]
                pub fn spelling(&self) -> &str {
                    self.0.as_str()
                }
            }
        )+
    };
}

rendered_identifiers! {
    SupportName, "exported support name";
    ModuleName, "stamped module name";
    TypeName, "declared type name";
    FunctionName, "declared function name";
}

impl BoundPath {
    /// The path rooted at one crate binding, over the segments that follow it.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no segment was supplied, [`DeclarationError::NotAnIdentifier`] where a segment cannot name a rendered item — a path rooted at a binding traverses items, so a segment outside the alphabet or on the keyword roster renders a path no consumer's compiler reads — and [`DeclarationError::Unbounded`] where the segments outgrow [`PATH_SEGMENT_LIMIT`].
    pub fn rooted(binding: Binding, segments: Vec<String>) -> Result<Self, DeclarationError> {
        Ok(Self {
            binding,
            segments: path_segments(segments)?,
        })
    }

    /// Which crate binding this path is rooted at.
    #[must_use]
    pub const fn binding(&self) -> Binding {
        self.binding
    }

    /// The segments after the binding, in the order they were declared; structurally at least one.
    #[must_use]
    pub fn segments(&self) -> &NonEmpty<String, PATH_SEGMENT_LIMIT> {
        &self.segments
    }
}

impl DirectBinding {
    /// One direct projection's physical dependency path, from its ordered item segments.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no segment was supplied, [`DeclarationError::NotAnIdentifier`] where a segment cannot name a rendered item, and [`DeclarationError::Unbounded`] where the path outgrows [`PATH_SEGMENT_LIMIT`].
    pub fn declared(segments: Vec<String>) -> Result<Self, DeclarationError> {
        path_segments(segments).map(|segments| Self { segments })
    }

    /// The dependency path's segments, in resolution order; structurally at least one.
    #[must_use]
    pub fn segments(&self) -> &NonEmpty<String, PATH_SEGMENT_LIMIT> {
        &self.segments
    }
}

/// One informed item-path segment roster, shared by logical carrier paths and physical direct bindings.
fn path_segments(
    segments: Vec<String>,
) -> Result<NonEmpty<String, PATH_SEGMENT_LIMIT>, DeclarationError> {
    if segments.is_empty() {
        return Err(DeclarationError::Absent {
            seat: Seat::PathSegment,
        });
    }
    for segment in &segments {
        if !rendered_name(segment) {
            return Err(DeclarationError::NotAnIdentifier);
        }
    }
    let offered = segments.len();
    NonEmpty::new(segments)
        .map_err(|_| DeclarationError::unbounded(Seat::PathSegment, PATH_SEGMENT_LIMIT, offered))
}

impl HelperRefusal {
    /// One refusal established by the grammar's own reading, at the token the clause sits at.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self {
            grammar,
            issue: CaptureIssue::Grammar { cause },
            at,
        }
    }

    /// One refusal the vocabulary established over a value the grammar read, at that value's own token.
    pub const fn vocabulary_refused(
        grammar: Grammar,
        refusal: DeclarationError,
        at: SpanHandle,
    ) -> Self {
        Self {
            grammar,
            issue: CaptureIssue::Vocabulary { refusal },
            at,
        }
    }

    /// One refusal from a shared descriptor reading, retained under the issue's own authority.
    pub const fn capture_refused(grammar: Grammar, issue: CaptureIssue, at: SpanHandle) -> Self {
        match issue {
            CaptureIssue::Grammar { cause } => Self::grammar_refused(grammar, cause, at),
            CaptureIssue::Vocabulary { refusal } => Self::vocabulary_refused(grammar, refusal, at),
        }
    }

    /// The grammar that was reading.
    #[must_use]
    pub const fn grammar(&self) -> Grammar {
        self.grammar
    }

    /// Which of the two readings refused, and its own answer.
    #[must_use]
    pub const fn issue(&self) -> CaptureIssue {
        self.issue
    }

    /// The token the refusal was established at.
    #[must_use]
    pub const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl CompositionError {
    /// The refusal one established issue list amounts to, or nothing where the list is empty.
    #[must_use]
    pub fn established(issues: Vec<CompositionIssue>) -> Option<Self> {
        let mut walk = issues.into_iter();
        let first = walk.next()?;
        Some(Self {
            body: Capped::<CompositionIssue, COMPOSITION_ISSUE_LIMIT>::first_n(first, walk),
        })
    }

    /// The refusal one overrun provider set amounts to.
    pub fn bounded(bound: usize, observed: usize) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(CompositionIssue::ProvidersUnbounded {
                bound: u64::try_from(bound).unwrap_or(u64::MAX),
                observed: u64::try_from(observed).unwrap_or(u64::MAX),
            })),
        }
    }

    /// The first established issue.
    #[must_use]
    pub fn first_issue(&self) -> &CompositionIssue {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the scan established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<CompositionIssue, COMPOSITION_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether the body kept every issue the scan established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

impl Composition {
    /// Declare the complete provider set.
    ///
    /// Duplicates are refused rather than deduplicated: silently keeping one of two entries is how a composition stops matching the providers that exist.
    ///
    /// # Errors
    ///
    /// Returns [`CompositionError`] naming every provider identity declared more than once, and the provider seat where the set outgrows [`PROVIDER_LIMIT`].
    pub fn declared(providers: Vec<Provider>) -> Result<Self, CompositionError> {
        if let Some(refusal) = CompositionError::established(doubled_providers(&providers)) {
            return Err(refusal);
        }
        let offered = providers.len();
        NonEmpty::new(providers)
            .map(|admitted| Self {
                providers: admitted,
            })
            .map_err(|_| CompositionError::bounded(PROVIDER_LIMIT, offered))
    }

    /// The first declared provider; structurally there is one.
    #[must_use]
    pub fn first(&self) -> &Provider {
        self.providers.first()
    }

    /// Every declared provider; structurally at least one.
    ///
    /// The set is keyed by provider identity, so nothing identity-bearing is derived from this order.
    #[must_use]
    pub fn providers(&self) -> &NonEmpty<Provider, PROVIDER_LIMIT> {
        &self.providers
    }
}

pub use crate::token::{rendered_identifier, rendered_name};

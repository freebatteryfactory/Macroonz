//! The descriptor home's invariant nucleus: every road that reaches a private field of the shared vocabulary.
//!
//! A name is parsed here, so a reference that names nothing is not a value anybody can hold.
//! A spelling is admitted here, so a rendered identifier that a consumer's compiler would read as something else is unwritable.
//! A path is rooted here, so an expression naming a crate and nothing in it does not exist.
//! A composition is built here, after the duplicate scan agreed.

use super::super::composition::doubled_providers;
use super::{
    CaptureCause, CaptureIssue, Composition, CompositionError, CompositionIssue, DeclarationError,
    DirectBinding, FunctionName, Grammar, HelperRefusal, ModuleName, Name, PATH_SEGMENT_LIMIT,
    PROVIDER_LIMIT, Provider, Seat, SupportName, TypeName,
};
use crate::bounded::{NonEmpty, NonEmptyError};
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

/// One informed item-path segment roster for physical direct bindings.
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
    fn established(issues: Vec<CompositionIssue>) -> Option<Self> {
        let mut walk = issues.into_iter();
        let first = walk.next()?;
        Some(Self {
            first,
            further: walk.collect(),
        })
    }

    /// The refusal one declaration issue amounts to.
    const fn one(issue: CompositionIssue) -> Self {
        Self {
            first: issue,
            further: Vec::new(),
        }
    }

    /// The first established issue.
    #[must_use]
    pub const fn first_issue(&self) -> &CompositionIssue {
        &self.first
    }

    /// Every issue this refusal carries, in the order the declaration pass established them; structurally at least one and never truncated.
    pub fn issues(&self) -> impl Iterator<Item = &CompositionIssue> {
        core::iter::once(&self.first).chain(self.further.iter())
    }

    /// How many complete issues the declaration pass established.
    pub(in crate::descriptor) const fn issue_count(&self) -> usize {
        self.further.len().saturating_add(1)
    }
}

impl Composition {
    /// One complete composition from its sole provider.
    #[must_use]
    pub const fn of_one(provider: Provider) -> Self {
        Self {
            providers: NonEmpty::one(provider),
        }
    }

    /// Declare the complete provider set.
    ///
    /// The ordinary caller supplies its natural `Vec`; this guard turns it into the private non-empty bounded representation before any duplicate work begins.
    /// Duplicates are refused rather than deduplicated: silently keeping one of two entries is how a composition stops matching the providers that exist.
    ///
    /// # Errors
    ///
    /// Returns [`CompositionError`] carrying [`DeclarationError::Absent`] where no provider was supplied, [`DeclarationError::Unbounded`] where the provider set outgrows [`PROVIDER_LIMIT`], then every provider identity declared more than once after the roster is admitted.
    pub fn declared(providers: Vec<Provider>) -> Result<Self, CompositionError> {
        let admitted = NonEmpty::new(providers).map_err(|refusal| {
            let refusal = match refusal {
                NonEmptyError::Empty(_) => DeclarationError::Absent {
                    seat: Seat::Provider,
                },
                NonEmptyError::Overflow(overflow) => {
                    DeclarationError::unbounded(Seat::Provider, overflow.capacity, overflow.offered)
                }
            };
            CompositionError::one(CompositionIssue::Declaration { refusal })
        })?;
        if let Some(refusal) = CompositionError::established(doubled_providers(&admitted)) {
            return Err(refusal);
        }
        Ok(Self {
            providers: admitted,
        })
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

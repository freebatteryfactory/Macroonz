#![doc = include_str!("README.md")]

mod compare;
mod conclude;
mod types;

pub use compare::{compared, compared_compilation};
pub use types::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledObservation,
    CompiledVerdict, DeclaredBehavior, DeclaredCompilation, DeclaredReadBack,
    DeclaredReadBackRoster, DeclaredReadBackRosterRefusal, DiagnosticAnchor, ObservedCompilation,
    ObservedMember, ObservedValue, PrimarySourceSpan, PrimarySourceSpanRefusal, RelativeSourcePath,
    RelativeSourcePathRefusal, RustcErrorCode, RustcErrorCodeRefusal, SourcePosition,
    SourcePositionRefusal,
};

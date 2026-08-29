//! A compiler-diagnostic anchor and its coordinates can be minted only through their informing constructors.

use macroonz_harness::oracle::{
    DeclaredCompilation, DiagnosticAnchor, ObservedCompilation, PrimarySourceSpan,
    RelativeSourcePath, RustcErrorCode, SourcePosition,
};

fn bypass(
    code: RustcErrorCode,
    source: RelativeSourcePath,
    start: SourcePosition,
    end: SourcePosition,
    anchor: DiagnosticAnchor,
) {
    let primary = PrimarySourceSpan { source, start, end };
    let _ = DiagnosticAnchor { code, primary };
    let _ = DeclaredCompilation {
        refusal: Some(anchor.clone()),
    };
    let _ = ObservedCompilation {
        refusal: Some(anchor),
    };
}

fn main() {}

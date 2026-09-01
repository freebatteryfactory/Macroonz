//! The pure comparison half of an exact compilation contract through the Macroonz facade.
//!
//! A subject-owned host still runs rustc or Cargo and supplies the observation.
//! This example compares declared and observed compiler outcomes without pretending it invoked a compiler.

use macroonz::harness::oracle::{
    CompilationVerdict, DeclaredCompilation, DiagnosticAnchor, ObservedCompilation,
    PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};

fn main() -> Result<(), String> {
    let declared = DeclaredCompilation::refuses(anchor("E0308", 7u64, 5u64, 12u64)?);
    let observed = ObservedCompilation::refused(anchor("E0308", 7u64, 5u64, 12u64)?);
    let verdict = macroonz::harness::oracle::compiled::compared_compilation(&observed, &declared);
    if verdict == CompilationVerdict::Conforms {
        Ok(())
    } else {
        Err(format!("the compilation contract disagreed: {verdict:?}"))
    }
}

fn anchor(code: &str, line: u64, start: u64, end: u64) -> Result<DiagnosticAnchor, String> {
    let source = RelativeSourcePath::informed("src/main.rs")
        .map_err(|refusal| format!("the source path was refused: {refusal:?}"))?;
    let start = SourcePosition::informed(line, start)
        .map_err(|refusal| format!("the start position was refused: {refusal:?}"))?;
    let end = SourcePosition::informed(line, end)
        .map_err(|refusal| format!("the end position was refused: {refusal:?}"))?;
    let primary = PrimarySourceSpan::informed(source, start, end)
        .map_err(|refusal| format!("the primary span was refused: {refusal:?}"))?;
    let code = RustcErrorCode::informed(code)
        .map_err(|refusal| format!("the error code was refused: {refusal:?}"))?;
    Ok(DiagnosticAnchor::at(code, primary))
}

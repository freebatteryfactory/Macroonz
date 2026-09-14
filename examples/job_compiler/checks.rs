use macroonz::harness::oracle::{
    CompilationDisagreement, CompilationVerdict, DeclaredCompilation, DiagnosticAnchor,
    PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use macroonz::native_compiler::CompilerOutput;

pub(super) fn anchor(code: &str, start: u64, end: u64) -> Result<DiagnosticAnchor, String> {
    let code = RustcErrorCode::informed(code).map_err(debug)?;
    let source = RelativeSourcePath::informed("fixtures/wrong-state.rs").map_err(debug)?;
    let start = SourcePosition::informed(3, start).map_err(debug)?;
    let end = SourcePosition::informed(3, end).map_err(debug)?;
    let primary = PrimarySourceSpan::informed(source, start, end).map_err(debug)?;
    Ok(DiagnosticAnchor::at(code, primary))
}

pub(super) fn compared(
    compiled: &CompilerOutput,
    declared: &DeclaredCompilation,
) -> Result<CompilationVerdict, String> {
    compiled.compared(declared).map_err(|error| {
        format!(
            "compiler observation was not established: {error:?}; {}",
            String::from_utf8_lossy(compiled.process().stderr().bytes())
        )
    })
}

pub(super) fn conforms(
    compiled: &CompilerOutput,
    declared: &DeclaredCompilation,
) -> Result<(), String> {
    let verdict = compared(compiled, declared)?;
    if verdict != CompilationVerdict::Conforms {
        return Err(format!("compilation disagreed: {verdict:?}"));
    }
    Ok(())
}

pub(super) fn disagreements(
    lawful: &CompilerOutput,
    hostile: &CompilerOutput,
) -> Result<(), String> {
    let wrong_code = compared(
        hostile,
        &DeclaredCompilation::refuses(anchor("E0277", 26, 29)?),
    )?;
    if !matches!(
        wrong_code,
        CompilationVerdict::Deviates(CompilationDisagreement::ErrorCode { .. })
    ) {
        return Err(format!("wrong code did not disagree: {wrong_code:?}"));
    }
    let shifted = compared(
        hostile,
        &DeclaredCompilation::refuses(anchor("E0308", 27, 30)?),
    )?;
    if !matches!(
        shifted,
        CompilationVerdict::Deviates(CompilationDisagreement::PrimarySpan { .. })
    ) {
        return Err(format!("shifted span did not disagree: {shifted:?}"));
    }
    let twin = compared(
        lawful,
        &DeclaredCompilation::refuses(anchor("E0308", 26, 29)?),
    )?;
    if twin != CompilationVerdict::Deviates(CompilationDisagreement::AcceptedWhereRefusalDeclared) {
        return Err(format!("lawful twin did not disagree: {twin:?}"));
    }
    Ok(())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

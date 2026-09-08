//! Fresh processes inspect complete retained fields after all source values and input buffers are gone.

use super::{compiled, vector};
use crate::archive_process;
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedOracle, ArchivedStructuralDisagreement, ArchivedVerdict, read_verdict,
    retain_compilation, retain_compiled, retain_structural, retain_transcript, retain_vector,
};
use macroonz_harness::oracle::{
    ByteDifference, CompilationDisagreement, CompilationVerdict, CompiledDisagreement,
    CompiledVerdict, DiagnosticAnchor, RustcErrorCode, SpecifiedContext, StructuralDisagreement,
    StructuralVerdict, TranscriptDerivation,
};
use macroonz_harness::report::archive::ArchiveLimits;
use std::error::Error;
use std::io::Read as _;

const LIMITS: ArchiveLimits = ArchiveLimits::declared(2048, 256);
type Record = (ArchivedOracle, ArchivedMethod, &'static str);

fn retained_records() -> Result<Vec<Record>, Box<dyn Error>> {
    let vector = vector::verdict(&[0, 255, 7], &[0, 254, 7, 8]).map_err(|()| "vector")?;
    let context = SpecifiedContext::spelled(&["outside", "process"]).map_err(|_| "context")?;
    let transcript = TranscriptDerivation::opened()
        .discriminant(91)
        .derived(&context)
        .compared(&[17; 32]);
    let structural = StructuralVerdict::Deviates(StructuralDisagreement::MeaningBearingAttribute {
        at: 23,
        attribute: "exact\0é".to_owned(),
    });
    let compiled = CompiledVerdict::Deviates(CompiledDisagreement::DuplicateMember {
        member: "member\0é".to_owned(),
    });
    let compilation =
        CompilationVerdict::Deviates(CompilationDisagreement::RefusedWhereAcceptanceDeclared {
            observed: DiagnosticAnchor::at(
                RustcErrorCode::informed("E0308").map_err(|_| "code")?,
                compiled::span("src/case.rs", [11, 7, 13, 17]).map_err(|()| "span")?,
            ),
        });
    Ok(vec![
        (
            retain_vector(&vector, LIMITS).map_err(|_| "vector retention")?,
            ArchivedMethod::Vector,
            "archive::process::vector_child",
        ),
        (
            retain_transcript(transcript, LIMITS).map_err(|_| "transcript retention")?,
            ArchivedMethod::Transcript,
            "archive::process::transcript_child",
        ),
        (
            retain_structural(&structural, LIMITS).map_err(|_| "structural retention")?,
            ArchivedMethod::Structural,
            "archive::process::structural_child",
        ),
        (
            retain_compiled(&compiled, LIMITS).map_err(|_| "compiled retention")?,
            ArchivedMethod::Compiled,
            "archive::process::compiled_child",
        ),
        (
            retain_compilation(&compilation, LIMITS).map_err(|_| "compilation retention")?,
            ArchivedMethod::Compilation,
            "archive::process::compilation_child",
        ),
    ])
}

#[test]
fn every_method_keeps_complete_owned_data_through_a_fresh_process() -> Result<(), Box<dyn Error>> {
    for (record, method, child) in retained_records()? {
        let mut original = record.encoded().to_vec();
        drop(record);
        let mut returned = archive_process::round_trip(&original, child)?;
        assert_eq!(returned, original);
        original.fill(0);
        let loaded = read_verdict(&returned, method, LIMITS).map_err(|_| "returned record")?;
        returned.fill(0);
        assert!(loaded.encoded().iter().any(|byte| *byte != 0));
    }
    Ok(())
}

fn from_stdin(method: ArchivedMethod) -> Result<ArchivedOracle, Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(4096)
        .read_to_end(&mut encoded)?;
    let loaded = read_verdict(&encoded, method, LIMITS).map_err(|_| "child record")?;
    encoded.fill(0);
    Ok(loaded)
}

#[test]
#[ignore = "launched by the parent with the retained vector bytes on stdin"]
fn vector_child() -> Result<(), Box<dyn Error>> {
    let loaded = from_stdin(ArchivedMethod::Vector)?;
    let ArchivedVerdict::VectorDisagrees(found) = loaded.verdict() else {
        return Err("vector disposition".into());
    };
    assert_eq!(found.expected(), &[0, 255, 7]);
    assert_eq!(found.produced(), &[0, 254, 7, 8]);
    assert_eq!(found.difference(), ByteDifference::AtByte { at: 1 });
    archive_process::publish(loaded.encoded())
}

#[test]
#[ignore = "launched by the parent with the retained transcript bytes on stdin"]
fn transcript_child() -> Result<(), Box<dyn Error>> {
    let loaded = from_stdin(ArchivedMethod::Transcript)?;
    let ArchivedVerdict::TranscriptDisagrees(found) = loaded.verdict() else {
        return Err("transcript disposition".into());
    };
    assert_eq!(
        found.rederived().as_bytes(),
        &blake3::derive_key("outside/process", &[91])
    );
    assert_eq!(found.published().as_bytes(), &[17; 32]);
    archive_process::publish(loaded.encoded())
}

#[test]
#[ignore = "launched by the parent with the retained structural bytes on stdin"]
fn structural_child() -> Result<(), Box<dyn Error>> {
    let loaded = from_stdin(ArchivedMethod::Structural)?;
    assert_eq!(
        loaded.verdict(),
        &ArchivedVerdict::StructuralDeviates(
            ArchivedStructuralDisagreement::MeaningBearingAttribute {
                at: 23,
                attribute: "exact\0é".to_owned(),
            }
        ),
    );
    archive_process::publish(loaded.encoded())
}

#[test]
#[ignore = "launched by the parent with the retained compiler read-back bytes on stdin"]
fn compiled_child() -> Result<(), Box<dyn Error>> {
    let loaded = from_stdin(ArchivedMethod::Compiled)?;
    assert_eq!(
        loaded.verdict(),
        &ArchivedVerdict::CompiledDeviates(CompiledDisagreement::DuplicateMember {
            member: "member\0é".to_owned(),
        }),
    );
    archive_process::publish(loaded.encoded())
}

#[test]
#[ignore = "launched by the parent with the retained exact compilation bytes on stdin"]
fn compilation_child() -> Result<(), Box<dyn Error>> {
    let loaded = from_stdin(ArchivedMethod::Compilation)?;
    let ArchivedVerdict::CompilationDeviates(
        CompilationDisagreement::RefusedWhereAcceptanceDeclared { observed },
    ) = loaded.verdict()
    else {
        return Err("compilation disposition".into());
    };
    assert_eq!(observed.code().spelling(), "E0308");
    let span = observed.primary();
    assert_eq!(span.source().spelling(), "src/case.rs");
    assert_eq!(span.start().line(), 11);
    assert_eq!(span.start().column(), 7);
    assert_eq!(span.end().line(), 13);
    assert_eq!(span.end().column(), 17);
    archive_process::publish(loaded.encoded())
}

use macroonz::harness::descriptor::NamespacedName;
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageCorpus, CoverageHostFailure, CoveragePoint, FuzzExecution,
    RustcProfileRefusal,
};
use macroonz::native_coverage::{self, NativeCoverage};

pub(super) fn execute(coverage: &NativeCoverage) -> Result<CoverageCorpus, String> {
    let mut corpus = coverage.corpus();
    let inputs: [&[u8]; 3] = [
        &[0],
        &[0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 7],
        &[0],
    ];
    let root = NamespacedName::named("neutral-job", "consumer").map_err(debug)?;
    for (at, input) in inputs.into_iter().enumerate() {
        let result = native_coverage::observe(coverage, &mut corpus, input)
            .map_err(|error| error.to_string())?;
        if result.execution() != FuzzExecution::Success || result.candidate() != input {
            return Err(format!(
                "coverage execution or input disagreed at {at}: {result:?}"
            ));
        }
        for point in result.observation().points() {
            let source = match point {
                CoveragePoint::Line { source, .. } | CoveragePoint::Branch { source, .. } => source,
            };
            if source.root() != root || source.relative() != "fixtures/coverage-record.rs" {
                return Err(format!("coverage escaped the declared fixture: {source:?}"));
            }
        }
        let admission = corpus.admit(result).map_err(debug)?;
        match (at, &admission) {
            (0 | 1, CoverageAdmission::Interesting(bytes)) if bytes.as_bytes() == input => {}
            (2, CoverageAdmission::Known) => {}
            _ => return Err(format!("unexpected novelty at {at}: {admission:?}")),
        }
    }
    let Err(exhausted) = native_coverage::observe(coverage, &mut corpus, &[0]) else {
        return Err("an extra coverage attempt was accepted".to_owned());
    };
    if !matches!(
        exhausted.cause(),
        CoverageHostFailure::Refused(RustcProfileRefusal::CaseBudgetExhausted { bound: 3 })
    ) {
        return Err(format!("unexpected attempt refusal: {exhausted}"));
    }
    if corpus.attempted_cases() != 3
        || corpus.attempted_input_bytes() != 18
        || corpus.interesting().len() != 2
        || corpus.retained_bytes() != 17
    {
        return Err(format!("coverage accounting disagreed: {corpus:?}"));
    }
    Ok(corpus)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

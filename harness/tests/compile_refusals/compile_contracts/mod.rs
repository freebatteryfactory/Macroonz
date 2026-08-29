//! External handwritten and facade-generated compilation contracts admitted through the runner.
//!
//! The disposable package deliberately resolves the current checkout through relative path dependencies.
//! This retained claim covers external package shape and public-road composition, not packaged path independence.

use super::swap_pairs::{diagnostic, scratch};
use macroonz_harness::clock::{HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, DerivedRevision,
    ExecutableAttachment, ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role,
    Row, SubjectRoute, Tag,
};
use macroonz_harness::oracle::{
    CompilationDisagreement, CompilationVerdict, DeclaredCompilation, DiagnosticAnchor,
    ObservedCompilation, PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, CheckRevisionId, FailureClass, FindingLocation, HostTrialRecord,
    InvocationProfile, ReplayPosture, RunAttempt, SubjectRevisionId, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialReport, TrialSite,
};
use macroonz_harness::runner::{
    Invocation, Selection, SelectionPlan, TrialBinding, TrialTable, record_all, record_one,
    trial_identity,
};

const OWNER: &str = "compile-contract-holder";
const HANDWRITTEN_LAWFUL: &str = concat!(
    "fn require(_: u8) {}\n",
    "\n",
    "fn main() {\n",
    "    require(7u8);\n",
    "}\n",
);
const HANDWRITTEN_HOSTILE: &str = concat!(
    "fn require(_: u8) {}\n",
    "\n",
    "fn main() {\n",
    "    let offered = \"hostile\";\n",
    "    require(offered);\n",
    "}\n",
);
const GENERATED_LAWFUL: &str = concat!(
    "bakery::macros::network! {\n",
    "    harness = bakery::harness,\n",
    "    module = generated_net,\n",
    "    namespace = \"compile-contract\",\n",
    "    nodes = [client, server],\n",
    "    link request = client to server,\n",
    "    schedule quiet = [],\n",
    "}\n",
    "\n",
    "fn main() -> Result<(), generated_net::Fault> {\n",
    "    let topology = generated_net::topology()?;\n",
    "    let schedule = generated_net::quiet()?;\n",
    "    let _node_count = topology.nodes().len();\n",
    "    let _discipline_count = schedule.disciplines().len();\n",
    "    Ok(())\n",
    "}\n",
);
const GENERATED_HOSTILE: &str = concat!(
    "bakery::macros::network! {\n",
    "    harness = bakery::harness,\n",
    "    module = generated_net,\n",
    "    namespace = \"compile-contract\",\n",
    "    nodes = [client, server],\n",
    "    link request = client to server,\n",
    "    schedule quiet = [],\n",
    "}\n",
    "\n",
    "fn require(_: bakery::harness::network::Topology) {}\n",
    "\n",
    "fn main() -> Result<(), generated_net::Fault> {\n",
    "    let schedule = generated_net::quiet()?;\n",
    "    require(schedule);\n",
    "    Ok(())\n",
    "}\n",
);

struct Subject {
    stem: &'static str,
    file_name: &'static str,
    source: &'static str,
}

struct AdmissionCase {
    stem: &'static str,
    source: &'static str,
    conclusion: TrialConclusion,
    expected_cause: Option<&'static str>,
}

const SUBJECTS: [Subject; 4] = [
    Subject {
        stem: "handwritten-lawful",
        file_name: "handwritten-lawful.rs",
        source: HANDWRITTEN_LAWFUL,
    },
    Subject {
        stem: "handwritten-hostile",
        file_name: "handwritten-hostile.rs",
        source: HANDWRITTEN_HOSTILE,
    },
    Subject {
        stem: "generated-lawful",
        file_name: "generated-lawful.rs",
        source: GENERATED_LAWFUL,
    },
    Subject {
        stem: "generated-hostile",
        file_name: "generated-hostile.rs",
        source: GENERATED_HOSTILE,
    },
];

/// Handwritten and generated subjects share one exact compiler contract and one runner admission road.
#[test]
fn external_compilation_contracts_enter_derived_runner_standing() -> Result<(), String> {
    let scratch = scratch::Scratch::claimed()?;
    let outcome = (|| -> Result<(), String> {
        for subject in &SUBJECTS {
            scratch.write_source(subject.file_name, subject.source)?;
        }
        let host = scratch.generate_lockfile().map_err(|failure| {
            format!("compile-contract lock generation was not runnable: {failure:?}")
        })?;

        let observations = SUBJECTS
            .iter()
            .map(|subject| observe(&scratch, subject))
            .collect::<Result<Vec<_>, _>>()?;
        let cases = admission_cases(&observations)?;
        admit(&cases, &host)
    })();
    scratch.finish(outcome)
}

fn admission_cases(observations: &[ObservedCompilation]) -> Result<Vec<AdmissionCase>, String> {
    let [
        handwritten_lawful,
        handwritten_hostile,
        generated_lawful,
        generated_hostile,
    ] = observations
    else {
        return Err("the fixed compilation subject roster changed shape".to_owned());
    };
    let handwritten_anchor = anchor("handwritten-hostile.rs", "E0308", 5u64, 13u64, 20u64)?;
    assert_eq!(handwritten_hostile.refusal(), Some(&handwritten_anchor));
    let generated_anchor = anchor("generated-hostile.rs", "E0308", 14u64, 13u64, 21u64)?;
    assert_eq!(generated_hostile.refusal(), Some(&generated_anchor));
    let handwritten_lawful_verdict = compare(
        handwritten_lawful,
        &DeclaredCompilation::compiles(),
        "handwritten lawful",
    )?;
    let handwritten_hostile_verdict = compare(
        handwritten_hostile,
        &DeclaredCompilation::refuses(handwritten_anchor),
        "handwritten hostile",
    )?;
    let generated_lawful_verdict = compare(
        generated_lawful,
        &DeclaredCompilation::compiles(),
        "generated lawful",
    )?;
    let generated_hostile_verdict = compare(
        generated_hostile,
        &DeclaredCompilation::refuses(generated_anchor.clone()),
        "generated hostile",
    )?;
    let (wrong_code_verdict, wrong_span_verdict) =
        mismatch_verdicts(generated_hostile, &generated_anchor)?;
    Ok(vec![
        AdmissionCase {
            stem: "handwritten-lawful",
            source: HANDWRITTEN_LAWFUL,
            conclusion: concluded(&handwritten_lawful_verdict),
            expected_cause: None,
        },
        AdmissionCase {
            stem: "handwritten-hostile",
            source: HANDWRITTEN_HOSTILE,
            conclusion: concluded(&handwritten_hostile_verdict),
            expected_cause: None,
        },
        AdmissionCase {
            stem: "generated-lawful",
            source: GENERATED_LAWFUL,
            conclusion: concluded(&generated_lawful_verdict),
            expected_cause: None,
        },
        AdmissionCase {
            stem: "generated-hostile",
            source: GENERATED_HOSTILE,
            conclusion: concluded(&generated_hostile_verdict),
            expected_cause: None,
        },
        AdmissionCase {
            stem: "generated-wrong-code-control",
            source: GENERATED_HOSTILE,
            conclusion: concluded(&wrong_code_verdict),
            expected_cause: Some("compiled-diagnostic-error-code"),
        },
        AdmissionCase {
            stem: "generated-wrong-span-control",
            source: GENERATED_HOSTILE,
            conclusion: concluded(&wrong_span_verdict),
            expected_cause: Some("compiled-diagnostic-primary-span"),
        },
    ])
}

fn mismatch_verdicts(
    generated_hostile: &ObservedCompilation,
    generated_anchor: &DiagnosticAnchor,
) -> Result<(CompilationVerdict, CompilationVerdict), String> {
    let wrong_code = anchor("generated-hostile.rs", "E0277", 14u64, 13u64, 21u64)?;
    let wrong_code_verdict = macroonz_harness::oracle::compiled::compared_compilation(
        generated_hostile,
        &DeclaredCompilation::refuses(wrong_code.clone()),
    );
    assert_eq!(
        wrong_code_verdict,
        CompilationVerdict::Deviates(CompilationDisagreement::ErrorCode {
            expected: wrong_code.code().clone(),
            observed: generated_anchor.code().clone(),
        })
    );
    let wrong_span = anchor("generated-hostile.rs", "E0308", 14u64, 14u64, 22u64)?;
    let wrong_span_verdict = macroonz_harness::oracle::compiled::compared_compilation(
        generated_hostile,
        &DeclaredCompilation::refuses(wrong_span.clone()),
    );
    assert_eq!(
        wrong_span_verdict,
        CompilationVerdict::Deviates(CompilationDisagreement::PrimarySpan {
            expected: wrong_span.primary().clone(),
            observed: generated_anchor.primary().clone(),
        })
    );
    Ok((wrong_code_verdict, wrong_span_verdict))
}

fn observe(scratch: &scratch::Scratch, subject: &Subject) -> Result<ObservedCompilation, String> {
    let output = scratch
        .check(subject.stem)
        .map_err(|failure| format!("{} was not runnable: {failure:?}", subject.stem))?;
    let locus = RelativeSourcePath::informed(&format!("src/bin/{}", subject.file_name))
        .map_err(|refusal| format!("{} locus was refused: {refusal:?}", subject.stem))?;
    diagnostic::observed_compilation(&output, scratch.root(), &locus)
}

fn anchor(
    file_name: &str,
    code: &str,
    line: u64,
    column_start: u64,
    column_end: u64,
) -> Result<DiagnosticAnchor, String> {
    let source = RelativeSourcePath::informed(&format!("src/bin/{file_name}"))
        .map_err(|refusal| format!("diagnostic source was refused: {refusal:?}"))?;
    let start = SourcePosition::informed(line, column_start)
        .map_err(|refusal| format!("diagnostic start was refused: {refusal:?}"))?;
    let end = SourcePosition::informed(line, column_end)
        .map_err(|refusal| format!("diagnostic end was refused: {refusal:?}"))?;
    let primary = PrimarySourceSpan::informed(source, start, end)
        .map_err(|refusal| format!("diagnostic span was refused: {refusal:?}"))?;
    let code = RustcErrorCode::informed(code)
        .map_err(|refusal| format!("diagnostic code was refused: {refusal:?}"))?;
    Ok(DiagnosticAnchor::at(code, primary))
}

fn compare(
    observed: &ObservedCompilation,
    declared: &DeclaredCompilation,
    context: &str,
) -> Result<CompilationVerdict, String> {
    let verdict = macroonz_harness::oracle::compiled::compared_compilation(observed, declared);
    if verdict == CompilationVerdict::Conforms {
        Ok(verdict)
    } else {
        Err(format!(
            "{context} compilation contract disagreed: {verdict:?}"
        ))
    }
}

fn concluded(verdict: &CompilationVerdict) -> TrialConclusion {
    verdict.concluded(FindingLocation::at(file!(), line!()))
}

fn retained_check(_: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn admit(cases: &[AdmissionCase], host: &scratch::HostFacts) -> Result<(), String> {
    let check_material = [
        include_bytes!("mod.rs").as_slice(),
        include_bytes!("../swap_pairs/diagnostic.rs").as_slice(),
        include_bytes!("../swap_pairs/scratch.rs").as_slice(),
    ]
    .concat();
    let check_revision = RevisionBinding::derived(DerivedRevision::from_material(&check_material));
    let invocation = invocation(host);
    let mut bindings = Vec::with_capacity(cases.len());
    let mut records = Vec::with_capacity(cases.len());

    for case in cases {
        let subject_revision =
            RevisionBinding::derived(DerivedRevision::from_material(case.source.as_bytes()));
        let binding = binding(case.stem, subject_revision, check_revision)?;
        let record = HostTrialRecord::recorded(
            trial_identity(binding.row()),
            RunAttempt::Executed(case.conclusion.clone()),
            MeasurementReading::Unavailable,
        );
        let admitted = record_one(&binding, &invocation, record.clone())
            .map_err(|refusal| format!("record_one refused {}: {refusal:?}", case.stem))?;
        assert_report(&admitted, case, subject_revision, check_revision, host)?;
        bindings.push(binding);
        records.push(record);
    }

    let table = TrialTable::authored(
        AuthoredTableName::named(OWNER, "external-compilation-contracts")
            .map_err(|refusal| format!("table name was refused: {refusal:?}"))?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(|refusal| format!("compile-contract table was refused: {refusal:?}"))?;
    let report = record_all(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &invocation,
        records,
    )
    .map_err(|refusal| format!("record_all refused the compile-contract roster: {refusal:?}"))?;
    assert_eq!(report.denominator(), cases.len());

    for (case, accounting) in cases.iter().zip(report.census()) {
        let subject_revision =
            RevisionBinding::derived(DerivedRevision::from_material(case.source.as_bytes()));
        let revisions = accounting.revisions();
        assert_eq!(
            revisions.subject(),
            SubjectRevisionId::of_binding(subject_revision)
        );
        assert_eq!(
            revisions.check(),
            CheckRevisionId::of_binding(check_revision)
        );
        let admitted = accounting
            .disposition()
            .report()
            .ok_or_else(|| format!("{} was not selected", case.stem))?;
        assert_report(admitted, case, subject_revision, check_revision, host)?;
    }
    Ok(())
}

fn binding(
    stem: &'static str,
    subject_revision: RevisionBinding,
    check_revision: RevisionBinding,
) -> Result<TrialBinding, String> {
    let subject = SubjectRoute::named(OWNER, stem)
        .map_err(|refusal| format!("subject route was refused: {refusal:?}"))?;
    let check = CheckRef::named(OWNER, "exact-compilation")
        .map_err(|refusal| format!("check reference was refused: {refusal:?}"))?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "compiler-to-runner-admission")
            .map_err(|refusal| format!("claim reference was refused: {refusal:?}"))?,
        ExecutionSuite::named(OWNER, "external-compiler")
            .map_err(|refusal| format!("execution suite was refused: {refusal:?}"))?,
        Classification::authored(
            vec![
                Role::named(OWNER, "holder")
                    .map_err(|refusal| format!("role was refused: {refusal:?}"))?,
            ],
            vec![
                Tag::named(OWNER, stem)
                    .map_err(|refusal| format!("tag was refused: {refusal:?}"))?,
            ],
        )
        .map_err(|refusal| format!("classification was refused: {refusal:?}"))?,
        subject,
        check,
        PopulationRef::named(OWNER, "fixed-external-sources")
            .map_err(|refusal| format!("population was refused: {refusal:?}"))?,
        Origin::HandWritten,
    )
    .map_err(|refusal| format!("row was refused: {refusal:?}"))?;
    Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check,
            subject_revision,
            check_revision,
            retained_check,
        ),
        Provenance::Unproduced,
    )
    .map_err(|refusal| format!("binding was refused: {refusal:?}"))
}

fn invocation(host: &scratch::HostFacts) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(4_096u64),
            TimeBudget::declared(60_000_000_000u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared(host.target()),
            ToolchainIdentity::declared(host.toolchain()),
        ),
        TrialSite::located(
            module_path!(),
            file!(),
            line!(),
            "external-compilation-contracts",
        ),
        HarnessClock::unavailable(),
    )
}

fn assert_report(
    report: &TrialReport,
    case: &AdmissionCase,
    subject_revision: RevisionBinding,
    check_revision: RevisionBinding,
    host: &scratch::HostFacts,
) -> Result<(), String> {
    let standing = report.standing();
    let key = standing.key();
    assert_eq!(key.target().target().spelling(), host.target());
    assert_eq!(key.target().toolchain().spelling(), host.toolchain());
    assert_eq!(
        key.subject(),
        SubjectRevisionId::of_binding(subject_revision)
    );
    assert_eq!(key.check(), CheckRevisionId::of_binding(check_revision));
    assert_eq!(standing.replay(), ReplayPosture::ExactDerived);
    match (case.expected_cause, report.attempt()) {
        (None, RunAttempt::Executed(TrialConclusion::Passed)) => Ok(()),
        (Some(expected), RunAttempt::Executed(TrialConclusion::Refused(finding))) => {
            assert_eq!(finding.class(), FailureClass::OracleDisagreement);
            assert_eq!(
                finding.cause().family(),
                macroonz_harness::oracle::ORACLE_CAUSE_FAMILY
            );
            assert_eq!(finding.cause().local(), expected);
            Ok(())
        }
        _ => Err(format!(
            "{} carried an unexpected admitted attempt: {:?}",
            case.stem,
            report.attempt()
        )),
    }
}

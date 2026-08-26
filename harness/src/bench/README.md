# bench — a number that means the same thing tomorrow

A wall-clock number on its own is a rumor.

Run the same code twice and the timings disagree.
Move it to another host or place it beside noisy work and they disagree more.
This home therefore qualifies a benchmark from declared work before it observes time.

## The evidence road

A benchmark row declares the workload, input axis, correctness preflight, planted-worse control, budgets, contention posture, optional formula, and complexity claim that give one comparison meaning.
Its `BenchRowKey` changes whenever one of those facts changes, so readings from different declarations cannot quietly share a row identity.

The executable attachment supplies the measured callable, the planted-worse callable, the owner-written judge, and the work observations those callables may record.
`BenchBinding` joins the declaration to that target-owned execution material only when their semantic names agree.

```mermaid
flowchart LR
    declared["Declared row<br/>identity + budgets"] --> admitted{"Target and toolchain<br/>agree?"}
    admitted -- no --> no_run["No caller code runs"]
    admitted -- yes --> preflight{"Correctness<br/>preflight passes?"}
    preflight -- no --> refused_preflight["Preflight-refused reading"]
    preflight -- yes --> primary["Count measured and<br/>planted-worse work"]
    primary --> control{"Control refused and<br/>gap distinguished?"}
    control -- no --> refused_control["Undistinguished-control reading"]
    control -- yes --> measured{"Measured curve satisfies<br/>the owner's claim?"}
    measured -- no --> refused_primary["Primary-work-refused reading"]
    measured -- yes --> timed["Warmups, then timed samples"]
    timed --> stable{"Timed work still<br/>qualifies?"}
    stable -- no --> no_report["No partial report"]
    stable -- yes --> qualified["Qualified reading<br/>with secondary time"]

    classDef declaration fill:#e8f1ff,stroke:#315a8a,color:#17324d,stroke-width:2px
    classDef gate fill:#fff4cc,stroke:#9a6b00,color:#513700,stroke-width:2px
    classDef refusal fill:#ffe8e8,stroke:#a43d3d,color:#5a1f1f,stroke-width:2px
    classDef evidence fill:#e8f8ed,stroke:#2f7d4a,color:#174329,stroke-width:2px
    class declared,primary,timed declaration
    class admitted,preflight,control,measured,stable gate
    class no_run,refused_preflight,refused_control,refused_primary,no_report refusal
    class qualified evidence
```

The complete execution order and the exact row-identity preimage are caller contracts on the public operations that establish them.
Independent external observations rederive the identity and reverse every qualifying gate.

## The judge and recorder

`WorkJudgmentInput` carries the formula, complexity claim, budgets, measured curve, and planted-worse curve.
It carries no duration, measurement reading, or clock, so qualification cannot depend on wall time.

A `WorkRecorder` is scoped to the observation roster in its attachment.
It refuses an observation outside that roster and refuses arithmetic overflow rather than inventing or wrapping work.

The planted-worse control makes the instrument falsifiable.
If the owner judge cannot reject the control and establish the declared exact gap, the measured curve does not qualify for timing.

## The report boundary

`BenchReport` retains one reading per authored binding, in authored order, and only this home can mint it.
Each reading keeps the complete row, the target it stood on, the preflight report, and the evidence for the stage it reached.
`bench_verdict` names the first row that did not qualify.

A renderer may read a completed report.
It cannot mint a report, reach a callable or judge, change a stage, or change the denominator.

## Ownership

The public owner remains `macroonz_harness::bench`.
The private declaration owner holds row meaning and row identity, while the private work owner holds scoped recording, work curves, owner judgment, and executable attachments.
Host execution joins those values to caller-declared target and clock facts without creating a new public child path.

## Nonclaims

This home is not a benchmark backend and does not define what fast means.
It fits no curve, applies no universal threshold, and does not interpret an owner's formula or complexity claim.
It reads no ambient host fact: target, toolchain, clock, and contention posture arrive as declared inputs.
Wall time remains a secondary observation of work that already qualified.

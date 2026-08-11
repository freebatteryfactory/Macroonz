# 17_pakvm — the executor's value machine

Band 17. Imports time (ConsumedBudgetEvidence), semantic (BoundDimensionRow),
identity, and the root calculus. The executor: the closed value algebra's
nine categories and five prohibited inhabitants, the two-tier memory model
with dumb generational arena indices, the three executor-side live handles
(structurally !Send/!Sync via the raw-pointer phantom), the capture record
and its seven invalid captures, the six step productions and five VM
terminals, the one-shot continuation record with the deadline-carriage rule,
and the six Transition-System Closure obligations.

## One twin killed

The resume-refusal union (nine conditions collected across three chapters) is
NOT a second family here — it is enforced through the port home's
`ResponseBinding` (12 causes, authored order): duplicate response →
`Duplicate`, second resume → `SecondResume`, wrong request/response type →
`WrongRequest`/`WrongType`, wrong Attempt → `DeadAttempt`, wrong generation →
`WrongGeneration`, expired deadline → `Expired`, late after terminal →
`Late`, over-bound → `OverBound`. One law, one owner, the membrane enforces.

## The ownership walls (facts the executor cannot construct)

Physical Attempt facts (completed/failed/refused/resource-exhausted/
outcome-unknown) are the membrane's observations (18); cancellation before or
after durable admission and reconciliation are the runtime's interpretations
(19). Each owner's outcomes compose by typed reference — named axes on the
outcome that has them, never one optional-field envelope. An execution result
never manufactures durability. The Attempt handle is 18's; the secret-use
handle is 22's; `Cursor` is navigation's immutable continuation EVIDENCE,
not a live handle.

## Value-algebra carrier (named owner)

The concrete exhaustive value enum lands with the executor machinery in this
home's own machinery phase — the category roster, prohibited inhabitants,
and residences are law now; the payloads need the arena types.

## Obligations

```yaml
home: 17_pakvm
obligations:
  - id: pakvm.value-algebra-is-closed
    challenge_kind: compile-law
    green: laws.rs pakvm::value_algebra_is_closed
    red: owed-to-testpak — an Any/host-object/function-pointer inhabitant
      must not compile
  - id: pakvm.live-handles-do-not-cross-threads
    challenge_kind: compile-refusal
    green: laws.rs pakvm::live_handles_do_not_cross_threads
    red: owed-to-testpak — sending a handle across threads must not compile
  - id: pakvm.continuation-record-binds-twelve
    challenge_kind: compile-law
    green: laws.rs pakvm::continuation_record_binds_twelve
    red: owed-to-testpak — a live monotonic member must not compile
  - id: pakvm.terminals-are-five-and-owned
    challenge_kind: compile-law
    green: laws.rs pakvm::terminals_are_five_and_owned
    red: owed-to-testpak — a physical fact constructed by the executor must
      not compile
  - id: pakvm.captures-and-closure-obligations
    challenge_kind: compile-law
    green: laws.rs pakvm::captures_and_closure_obligations
    red: owed-to-testpak
```

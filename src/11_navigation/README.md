# 11_navigation — the semantic address space and navigation

Band 11. Imports history (cuts, `SourceClosure`, federation vector), identity,
refusal, bounds, value, and the root calculus. The machine's sense of WHERE:
frames, axes, addresses, journal views, frame transformations, the navigation
ladder, `Fix<T>`, the positioning refusal, bounded traversal, typed paths,
paging, cursors, and logical time-travel inspection. Navigation consumes exact
cuts; it never mints order.

## The admission-vs-state law

A fact's admission `Address` answers where that fact entered the address
space; a state coordinate or `Fix<T>` answers where accepted evidence places
the application now. One never substitutes for the other.

## The nine postures, resolved by shape

One admitted route IS the success channel; several lawful alternatives are the
fix's bounded member; incomplete search is the closure axis; ambiguous
destination and approximation are fix shapes; stale inputs are the freshness
axis; no-route-under-closure, unauthorized source region, unsupported
operation, and exceeded bounds are `PositioningRefusal`'s four causes. Every
posture lands on exactly one owner; no posture enum exists.

## Authored fresh here (flagged in the code)

Type names for the four ladder roles (`NavigationRequest`,
`SemanticPathProgram`, `ResolvedRoute`, `AdmittedRoute`),
`FrameTransformation`, the `MultiAuthorityRelationship` carrier, the `Region`/
`State` payloads of `FixShape`, `HistoricalReconstruction`, and the cursor
transplantation SELECTION ORDER (WrongFamily → CrossSource → CrossGeneration →
CrossQuery → CrossFilter → CrossOrder → CrossDirection → CrossCut — family
gates decode; generations are scoped to sources; a query precedes its own
refinements; cuts compare last).

## Prohibited collapses (binding)

one universal identity/version/digest/status/position/cursor/receipt · package
path as semantic identity · route/host/placement/connection as write authority
· admission address as derived fix or physical placement · semantic route as
physical route/cursor/checkpoint/cut/capability/completeness proof ·
overlapping query regions as overlapping write authority · HLC as exact
durable order or federation progress · one fabricated cross-authority sequence
· cursor or push notice as durable progress · session identity as checkpoint
authority · derived applied cut as event truth. A universal `ThreadId` rides
this list; "logical thread" stays live prose — no product type bears that
name.

## The final law (the heartbeat's acceptance criterion)

The machine preserves a logical thread only by keeping its semantic location
and navigation honest: which frame gives an address meaning, where a fact was
admitted, which evidence produced a derived fix, which path and destination
were evaluated, which authority regions and exact cuts participated, what
chronology says, what durable order proves, how an operation continues, and
which admitted checkpoint permits progress. No shared representation,
convenient API, physical route, or generated implementation may collapse those
questions.

## Obligations

```yaml
home: 11_navigation
obligations:
  - id: navigation.positioning-order-is-declared
    challenge_kind: compile-law
    green: laws.rs navigation::positioning_order_is_declared
    red: owed-to-testpak
  - id: navigation.fix-binds-orthogonal-axes
    challenge_kind: compile-law
    green: laws.rs navigation::fix_binds_orthogonal_axes
    red: owed-to-testpak — an enum-flattened fix must not be constructible
  - id: navigation.frame-version-rides-authority-position
    challenge_kind: compile-refusal
    green: laws.rs navigation::frame_version_rides_authority_position
    red: testpak/tests/compile-fail/cross-frame-comparison-on-a-production-guard.rs
  - id: navigation.a-production-guard-cannot-be-laundered
    challenge_kind: compile-refusal
    green: structural (the position is a private seat of a stamped guard, and the
      absence of any road out is derived rather than attempted — see band 02, and
      cargo xtask check's stamped-guards-seal-their-position)
    red: testpak/tests/compile-fail/a-production-scope-guard-cannot-be-laundered.rs
  - id: navigation.axis-capabilities-are-declared
    challenge_kind: compile-refusal
    green: laws.rs navigation::axis_capabilities_are_nine
    red: owed-to-testpak — an undeclared capability operation must not compile
  - id: navigation.cursor-transplantation-owes-its-order
    challenge_kind: compile-law
    green: laws.rs navigation::cursor_transplantation_owes_its_order
    red: owed-to-testpak
  - id: navigation.continuation-roles-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs navigation::continuation_roles_do_not_unify
    red: owed-to-testpak — a From/Into among cursor/checkpoint/cut must not compile
  - id: navigation.rosters-hold
    challenge_kind: compile-law
    green: laws.rs navigation::traversal_path_and_checkpoint_rosters_hold
    red: owed-to-testpak
```

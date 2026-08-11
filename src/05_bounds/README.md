# 05_bounds — bound classes and the affine budget

Band 05. Imports refusal and the root calculus. Seated after numeric by the
topology probe: budgets carry typed magnitudes; numeric never needed bounds back.

## The seven classes

Work · Memory · Result · Effect · Suspension · Output · Time — seven, closed.
Time is the durable deadline-policy budget, enforced at the time home riding
this home's affine shape. The first five are the cross-domain minimum: no
computation is admitted without enforceable finite bounds in all five. The
register is two-level; the dimension level under each class is owed to the
execution home — dimensions derive from what operators charge, and the
operator register is authored there. Carried here only as the registered
`DimensionId` shape.

## Budgets are affine — the monotone-shrink law at the type level

`Budget<D>` is neither `Copy` nor `Clone`. `charge` consumes the budget and
yields the smaller successor or a typed refusal. No widening operation exists in
this home — value can be lost at a boundary, never manufactured; the only
reverse is a named, authority-bearing, receipt-leaving morphism owned where
grants live.

## The metering law

Bounds are semantic; the meter is a mechanism. Backend instruction count never
becomes part of semantic meaning — determinism rides on this.

## Deliberately not here (owner named, seat queued)

Deadlines — their own axis by law, never a bound class: the time home owns the
policy and rides this budget shape with its own dimension marker. The dimension
roster — execution home, after the operator register. The widening morphism —
authority and evidence homes. The 16 decode maxima — the bytes home instantiates
them from these shapes.

## Obligations

```yaml
home: 05_bounds
obligations:
  - id: bounds.classes-are-closed-and-seven
    challenge_kind: compile-law
    green: laws.rs bounds::classes_are_closed_and_seven
    red: owed-to-testpak
  - id: bounds.cross-domain-minimum-is-five
    challenge_kind: compile-law
    green: laws.rs bounds::cross_domain_minimum_is_five
    red: owed-to-testpak
  - id: bounds.budget-is-affine
    challenge_kind: compile-refusal
    green: laws.rs bounds::charge_shrinks_or_refuses
    red: owed-to-testpak — cloning or copying a Budget must not compile
  - id: bounds.charge-shrinks-or-refuses
    challenge_kind: compile-law
    green: laws.rs bounds::charge_shrinks_or_refuses
    red: owed-to-testpak
  - id: bounds.dimensions-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs bounds::dimensions_do_not_unify
    red: owed-to-testpak — passing Budget<Work> where Budget<Effect> is required must not compile
```

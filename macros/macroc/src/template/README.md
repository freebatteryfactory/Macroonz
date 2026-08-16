# template — the authoring front door

A typed declaration template: the holes it declares, what a binding may fill them
with, and what one invocation of it is keyed by.

## Category-typed holes

A template's holes are typed by category, and the category travels on both ends
of every binding: a string can never become an identifier, and a type cannot
enter an expression hole. A disagreement is a typed refusal at the binding seam
([`TemplateBinding::bound`]), never a substitution nobody notices.

## Where the meaning is owned

Templating is the machine's own authoring role, declared as
[`threadpak::declaration::AuthoringRole::Quotation`], and that declaration owns
every semantic fact this home speaks of. The laws governing meta evaluation are
the owner's closed roster, [`threadpak::declaration::MetaStageLaw`]; this home
cites that surface and answers none of it a second time. Every declaration below
is a typed member summarizing an owner fact.

Any front door may offer a template surface, or none. Nothing here knows which
one is calling.

## The three locks

Band 13's [`threadpak::declaration::META_EVALUATION_LOCKS`] names the locks every
meta evaluation declares BEFORE evaluation. This home carries one typed member
per lock — [`SymbolicBoundFormula`], [`ProfileCeiling`], and
[`CheckedMeterPosture`] — so a template that declared none of them is
unrepresentable rather than refused. The lock roster's wording stays band 13's;
these members cite it.

## The stage, and the meter

The stage a judgment stands at is band 13's
[`Stage`](threadpak::declaration::Stage). A template records the stage its owner
declared it is evaluated at and nothing more: the plane never decides a stage,
never promotes material across one, and never mints Semantic Form.

[`CheckedMeterPosture`] is an obligation carrier and a stated nonclaim, not a
meter. The meter is a gated mechanism, so this home names the owner who declared
the obligation rather than pretending to run it.

## The seats

`types.rs` declares: the category roster, the holes and the commitments that fill
them, the three locks, the template, the application, and the invocation key. Its
own child `type_guard.rs` holds every road that reaches a private field — the
binding's two ends, the ceiling's axes, the template's holes, the application's
bindings, and the refusal body's one seat — which is what makes the category
proof, the complete ceiling, and the complete application structural: a
cross-category binding, a ceiling missing an axis, and an application with an
unbound hole are values nobody can build.
`type_contract.rs` states the two refusal families' declared shapes.
`establish.rs` is the pure passes those roads consume — what a hole set, a
ceiling, and a binding set each establish — reaching no private seat.

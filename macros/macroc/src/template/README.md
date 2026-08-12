# template — the declaration-template contract

What a typed template declares, what a binding may fill it with, and what one
invocation of it is keyed by.

## Category-typed holes

A template's holes are typed by category, and the category travels on both ends
of every binding: a string can never become an identifier; a type cannot enter an
expression hole. The disagreement is a typed refusal at the binding seam
([`TemplateBinding::bound`]), never a substitution nobody notices.

## Frontend-neutral, and owned elsewhere

Templating is the machine's own authoring role — band 13 declares it as
[`threadpak::declaration::AuthoringRole::Quotation`], and that declaration owns
every semantic fact this home speaks of: that a quoted fragment is typed data
rather than text, that splicing substitutes typed values only, that instantiation
mints no authority, and that produced material re-enters the ordinary validation
and linking path with no shortcut. The six stage laws governing meta evaluation
are the owner's closed roster, [`threadpak::declaration::MetaStageLaw`] — this
home cites that surface and answers none of it a second time; every fact here is
a typed member summarizing an owner declaration.

Any front door may offer a template surface, or none. Nothing here knows which
one is calling.

## The three locks are members, not prose

Band 13's [`threadpak::declaration::META_EVALUATION_LOCKS`] names the three locks
every meta evaluation declares BEFORE evaluation. This home carries one typed
member per lock — [`SymbolicBoundFormula`], [`ProfileCeiling`], and
[`CheckedMeterPosture`] — so a template that declared none of them is
unrepresentable rather than refused. The lock roster's wording stays band 13's;
these members cite it.

## The staged-meta laws, and where they live

The stage a judgment stands at is band 13's
[`Stage`](threadpak::declaration::Stage), and the staged-meta laws are band 13's
too. A template records the stage its owner declared it is evaluated at and
nothing more: the plane never decides a stage, never promotes material across
one, and never mints Semantic Form — what a template produces is declaration
material that re-enters the machine's own path untrusted, is judged at the
instantiating site, and carries no live authority of its own.

## The meter is a mechanism, and mechanisms are gated

[`CheckedMeterPosture`] is an obligation carrier and a stated nonclaim, not a
meter. The actual meter must refuse before over-limit allocation and must never
return a partial fragment set; that obligation is the owner's, the mechanism is
gated, and this home says which owner declared it rather than pretending to run
it.

## The seats

`types.rs` declares: the category roster, the holes and the commitments that fill
them, the three locks, the template, the application, and the invocation key. Its
own child `type_guard.rs` holds every road that reaches a private field — the
binding's two ends, the ceiling's axes, the template's holes, the application's
bindings — which is what makes the category proof, the complete ceiling, and the
complete application structural: a cross-category binding, a ceiling missing an
axis, and an application with an unbound hole are values nobody can build.
`type_contract.rs` states the two refusal families' declared shapes.
`establish.rs` is the pure passes those roads consume — what a hole set, a
ceiling, and a binding set each establish — and the body the established issues
amount to.

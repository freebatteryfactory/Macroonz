//! The stamp: the road from one declaration of rows to the seats a test host runs.
//!
//! It lives here, with the vocabulary it reads, because everything it names on the descriptor side is this home's — the table constructor, the binding constructor, the name parsers, and the one refusal family the whole road answers with.
//!
//! Nothing in this file is compiled where it is written.
//! A `macro_rules!` body is a token sequence expanded at the site that invokes it, so this home gains no dependency edge from anything the expansion names — including [`crate::runner`], which sits above it.
//! `$crate` names this crate wherever the expansion lands, whatever a consumer renamed the dependency to.
//!
//! # What the expansion demands of the engine
//!
//! The stamp is the road that instantiates the descriptor vocabulary's two type parameters.
//! A [`Binding`](crate::descriptor::Binding) is generic in its invocation facts and its conclusion because this home may not import a record type; the stamp expands at the engine's instantiation of them, because that is the pair the engine the seats call runs at.
//! A table whose bindings carry another pair is a lawful table and does not go through this stamp.
//!
//! Past `core` and `std` the expansion names the record vocabulary, the clock, and the engine, and no other home — each by name here, with its own home owning what it takes and returns:
//!
//! - [`runner::TrialBinding`](crate::runner::TrialBinding) and [`runner::TrialTable`](crate::runner::TrialTable), the two aliases spelling that instantiation, so no expansion writes the parameters by hand.
//! - [`clock::HarnessClock`](crate::clock::HarnessClock), whose declared road to a reading is a `const`, so a table's clock is stamped as a constant beside its budgets.
//! - [`runner::Invocation`](crate::runner::Invocation), declared once per seat and once per lens, so a report carries the site of the seat that ran it.
//! - [`runner::Selection`](crate::runner::Selection)'s by-suite arm — one aggregate seat names exactly one suite and hands it in as a one-element set.
//! - [`runner::SelectionPlan::of`](crate::runner::SelectionPlan::of), the road that states a run expects its selection to match at least one row.
//!   A stamped seat takes it and never the empty-tolerant one: a declared suite that pairs with no row is the vacuity these seats exist to catch.
//! - [`runner::run_all`](crate::runner::run_all) and [`runner::run_one`](crate::runner::run_one), whose accounting has no refusal path after caller-supplied functions return.
//! - [`runner::SeatRefusal`](crate::runner::SeatRefusal), the seats' one refusal type, reached unchanged by every construction refusal on this road.
//! - [`runner::seat_verdict`](crate::runner::seat_verdict) and [`runner::lens_verdict`](crate::runner::lens_verdict) — the engine's own verdict readings, because a fold over a report written into every expansion would be a calculator standing in as many places as there are invocations.
//!
//! # Where a host fact enters
//!
//! A run stands on facts no library here can honestly read: which target it was compiled for, which toolchain built it, and what a nanosecond reading is worth.
//! A triple assembled out of `cfg!` predicates would be a plausible spelling of a fact rather than the fact, and it would enter a cache key, so a wrong one buys a hit nothing verified.
//! So `target:` and `clock:` are declared at the invocation, in the caller's own test target, and the expansion carries them through untouched.
//! Neither clause is optional, and that is the point: the honest answer to "nothing was measured" has a name a person types.
//!
//! The third host-shaped fact, the site, is neither declared nor read: it is where the seat is written, which the expansion already knows.
//!
//! # The refusal channel
//!
//! Every seat the stamp writes is a test function returning a `Result`, and the expansion routes its own fallible constructions and verdict reading through that typed channel.
//! No expansion contains an unwrap, an expectation, an assertion, an explicit panic, or an index.
//! Row expressions and caller-supplied functions keep their own effect and unwind ceilings; the stamp does not turn arbitrary caller code into a panic-free operation.

/// Stamps one complete authored world, one aggregate seat per declared execution suite, and one named lens per row, from a single declaration.
///
/// # The grammar
///
/// Two invocation forms, differing only in the provenance the table states.
/// The unproduced form:
///
/// ```text
/// trial_table! {
///     /// Optional notes, carried onto the stamped module.
///     pub mod <module> named(<namespace literal>, <stem literal>) {
///         provenance: unproduced,
///         invocation: <expression>,
///         target: <expression>,
///         clock: <expression>,
///
///         suite <seat> named(<namespace literal>, <stem literal>) {
///             <row>: <expression>,
///             <row>: <expression>,
///         }
///     }
/// }
/// ```
///
/// The produced form replaces one clause, and nothing else:
///
/// ```text
///         provenance: produced(<namespace literal>, <stem literal>)
///             against <expression>,
/// ```
///
/// - `<module>` is the stamped module's name, and the visibility in front of `mod` is carried onto the module and onto every declaration it holds, so no public road ever ends at a private one.
/// - `named(<namespace>, <stem>)` after `mod` is the authored table's own namespaced name, parsed at run time because a name that states no owner is refused rather than stamped.
/// - `provenance:` is one of the two forms above; the produced form's expression evaluates to `Result<GeneratedSupportSchemaId, TrialTableRefusal>`, which is the shape a producer's own identity road already has.
/// - `invocation:` evaluates to an [`InvocationProfile`](crate::report::InvocationProfile), stamped as a `const` item deliberately: an ambient fact cannot appear in a `const`, so a reading or an environment value in this position is refused by the compiler rather than by a rule somebody follows.
/// - `target:` evaluates to a [`TargetBinding`](crate::report::TargetBinding), and has no default because nothing in this crate derives a triple or a toolchain identity.
///   A consumer that wants them read rather than typed writes a build script in its own crate that emits them.
/// - `clock:` evaluates to a [`HarnessClock`](crate::clock::HarnessClock), stamped as a `const` for the reason the budgets are one: what a clock declares is the road to a reading, never a reading.
///   A table that measures nothing writes [`HarnessClock::unavailable()`](crate::clock::HarnessClock::unavailable), whose reading stays distinct from an observed zero.
/// - `suite <seat> named(<namespace>, <stem>) { … }` declares one aggregate seat: `<seat>` is the test function's name, and the two literals are the [`ExecutionSuite`](crate::descriptor::ExecutionSuite) it selects on.
///   At least one suite group is required, and each group requires at least one row.
/// - `<row>: <expression>` declares one row: `<row>` names its lens, and the expression answers with one [`TrialBinding`](crate::runner::TrialBinding) or refuses in any family that discharges into [`TrialTableRefusal`](crate::descriptor::TrialTableRefusal).
///   The stamp never reads inside the expression: a row's internals are the producer's statement, and a macro that parsed them would be a second authority over this vocabulary.
///
/// Both grammars end their clauses and their rows with a comma.
///
/// # What is stamped
///
/// One module, containing a private `row` module with one function per declared row; a `table` function building the authored world through the public constructors; an `INVOCATION` constant and a `CLOCK` constant; a `target` function; one ordinary `#[test]` per suite group; and one `#[test] #[ignore = "lens"]` per row.
/// Each seat and each lens builds its own invocation, so a report carries the site of the seat that produced it rather than one site the whole table shared.
///
/// # Authority
///
/// Every declared row lands in the one table, whichever group it was declared under.
/// The grouping decides which aggregate seat exists; it never decides which rows the world holds, because a selection narrows a run and never the denominator.
///
/// A group is a seat declaration and not a claim about the rows in it: the selection reads each row's own execution suite, so a row grouped under a seat whose suite is not the row's own is simply not selected by that seat.
/// The stamp cannot check that pairing without reading inside a row expression, and does not pretend to; the engine answers it at run time, because a seat whose selection named no row refuses.
///
/// # Bounds
///
/// A seat's name and a row's name share one namespace, because both are functions in the stamped module, and the stamp takes five spellings in it — `row`, `table`, `target`, `INVOCATION`, and `CLOCK`.
/// A seat or row claiming one is an ordinary duplicate definition with an ordinary diagnostic.
///
/// The seats are `#[test]` functions, so the stamp is invoked where a test harness collects them.
///
/// The `@`-prefixed rule below is the stamp's internal transcription, not an invocation form.
/// The forms above are text rather than compiled examples, because a compiled one needs a subject, a check, and a population, and those live on the challenge side.
#[macro_export]
macro_rules! trial_table {
    (
        $(#[$note:meta])*
        $vis:vis mod $module:ident named($table_namespace:literal, $table_stem:literal) {
            provenance: unproduced,
            invocation: $invocation:expr,
            target: $target:expr,
            clock: $clock:expr,
            $(
                suite $seat:ident named($suite_namespace:literal, $suite_stem:literal) {
                    $(
                        $row:ident: $binding:expr,
                    )+
                }
            )+
        }
    ) => {
        $crate::trial_table! {
            @stamp
            [$(#[$note])*]
            [$vis]
            [$module]
            [$table_namespace]
            [$table_stem]
            [::core::result::Result::Ok($crate::descriptor::Provenance::Unproduced)]
            [$invocation]
            [$target]
            [$clock]
            $(
                [$seat]
                [$suite_namespace]
                [$suite_stem]
                {
                    $(
                        [$row]
                        [$binding]
                    )+
                }
            )+
        }
    };

    (
        $(#[$note:meta])*
        $vis:vis mod $module:ident named($table_namespace:literal, $table_stem:literal) {
            provenance: produced($producer_namespace:literal, $producer_stem:literal)
                against $schema:expr,
            invocation: $invocation:expr,
            target: $target:expr,
            clock: $clock:expr,
            $(
                suite $seat:ident named($suite_namespace:literal, $suite_stem:literal) {
                    $(
                        $row:ident: $binding:expr,
                    )+
                }
            )+
        }
    ) => {
        $crate::trial_table! {
            @stamp
            [$(#[$note])*]
            [$vis]
            [$module]
            [$table_namespace]
            [$table_stem]
            [
                $schema.and_then(|schema| {
                    $crate::descriptor::ProducerName::named(
                        $producer_namespace,
                        $producer_stem,
                    )
                    .map_err($crate::descriptor::TrialTableRefusal::NameNotParsed)
                    .map(|producer| $crate::descriptor::Provenance::Produced {
                        producer,
                        schema,
                    })
                })
            ]
            [$invocation]
            [$target]
            [$clock]
            $(
                [$seat]
                [$suite_namespace]
                [$suite_stem]
                {
                    $(
                        [$row]
                        [$binding]
                    )+
                }
            )+
        }
    };

    // THE ONE TRANSCRIPTION. Both invocation forms arrive here with their provenance already assembled
    // into one expression of one type, so the module below is written once and neither form can drift
    // from the other.
    (
        @stamp
        [$(#[$note:meta])*]
        [$vis:vis]
        [$module:ident]
        [$table_namespace:literal]
        [$table_stem:literal]
        [$provenance:expr]
        [$invocation:expr]
        [$target:expr]
        [$clock:expr]
        $(
            [$seat:ident]
            [$suite_namespace:literal]
            [$suite_stem:literal]
            {
                $(
                    [$row:ident]
                    [$binding:expr]
                )+
            }
        )+
    ) => {
        $(#[$note])*
        /// One stamped trial table: the authored world its rows declare, one aggregate seat per declared execution suite, and one ignored lens per row.
        $vis mod $module {
            /// One function per declared row, so each declared expression is written exactly once
            /// and both spellings read that one: the table collects these functions, and each named
            /// lens calls its own.
            mod row {
                $(
                    $(
                        /// One declared row's binding.
                        ///
                        /// # Errors
                        ///
                        /// Refuses whatever the declaration's own constructions refuse, each carried into the stamp's one family by the discharge that family declares for it.
                        pub(super) fn $row() -> ::core::result::Result<
                            $crate::runner::TrialBinding,
                            $crate::descriptor::TrialTableRefusal,
                        > {
                            ::core::result::Result::Ok($binding?)
                        }
                    )+
                )+
            }

            /// The budgets every seat in this table runs under.
            ///
            /// A `const` item, deliberately: an ambient fact cannot appear in one.
            $vis const INVOCATION: $crate::report::InvocationProfile = $invocation;

            /// The clock every seat in this table measures with.
            ///
            /// A `const` item for the same reason the budgets are one: what a clock declares is the road to a reading, never a reading.
            $vis const CLOCK: $crate::clock::HarnessClock = $clock;

            /// The target and toolchain this table's runs stand on.
            ///
            /// Declared rather than read: nothing here derives a target triple or a toolchain identity, and these facts enter an execution key.
            /// A function rather than a constant only because the binding owns its spellings and an owned string is not a `const`.
            $vis fn target() -> $crate::report::TargetBinding {
                $target
            }

            /// The complete authored world these rows declare.
            ///
            /// Every declared row is here, whichever group it was declared under.
            ///
            /// # Errors
            ///
            /// Refuses the first construction that refused, in the order the stamp builds them: the table's name, the stated provenance, each row's binding in declared order, then the authored world itself.
            $vis fn table() -> ::core::result::Result<
                $crate::runner::TrialTable,
                $crate::descriptor::TrialTableRefusal,
            > {
                let name = $crate::descriptor::AuthoredTableName::named(
                    $table_namespace,
                    $table_stem,
                )
                .map_err($crate::descriptor::TrialTableRefusal::NameNotParsed)?;
                let stated: ::core::result::Result<
                    $crate::descriptor::Provenance,
                    $crate::descriptor::TrialTableRefusal,
                > = $provenance;
                let provenance = stated?;
                let bindings = ::std::vec![
                    $(
                        $(
                            row::$row()?,
                        )+
                    )+
                ];
                $crate::descriptor::AuthoredTable::authored(name, provenance, bindings)
                    .map_err($crate::descriptor::TrialTableRefusal::TableNotAuthored)
            }

            $(
                /// One aggregate seat: the complete world, narrowed to the rows whose own execution suite is this seat's, through the report engine.
                ///
                /// An ordinary test function, so it runs by default.
                /// The verdict's own outcome is bound and goes no further: a test function's answer has exactly two channels, and what a run did beyond refusing is the report's to carry.
                ///
                /// # Errors
                ///
                /// Refuses when the world could not be built, when this seat's suite name could not be parsed, or when the run's own verdict refuses — including the run that selected nothing.
                #[test]
                fn $seat() -> ::core::result::Result<(), $crate::runner::SeatRefusal> {
                    let world = table()?;
                    let suite = $crate::descriptor::ExecutionSuite::named(
                        $suite_namespace,
                        $suite_stem,
                    )
                    .map_err($crate::descriptor::TrialTableRefusal::NameNotParsed)?;
                    let view = world.view();
                    let selection = $crate::runner::SelectionPlan::of(
                        $crate::runner::Selection::ByExecutionSuite(
                            ::std::collections::BTreeSet::from([suite]),
                        ),
                    );
                    let invocation = $crate::runner::Invocation::declared(
                        INVOCATION,
                        target(),
                        $crate::report::TrialSite::located(
                            ::core::module_path!(),
                            ::core::file!(),
                            ::core::line!(),
                            ::core::stringify!($seat),
                        ),
                        CLOCK,
                    );
                    let report = $crate::runner::run_all(&view, &selection, &invocation);
                    let _outcome = $crate::runner::seat_verdict(&report)?;
                    ::core::result::Result::Ok(())
                }
            )+

            $(
                $(
                    /// One named lens: this row alone, through the same engine the aggregate seat calls.
                    ///
                    /// Ignored by default — clickable in an editor and runnable by name, and never paid for twice in an ordinary run.
                    /// A lens and a seat differ in the site they state and in nothing else.
                    ///
                    /// # Errors
                    ///
                    /// Refuses when this row's binding could not be built, or when the trial's own verdict refuses.
                    #[test]
                    #[ignore = "lens"]
                    fn $row() -> ::core::result::Result<(), $crate::runner::SeatRefusal> {
                        let binding = row::$row()?;
                        let invocation = $crate::runner::Invocation::declared(
                            INVOCATION,
                            target(),
                            $crate::report::TrialSite::located(
                                ::core::module_path!(),
                                ::core::file!(),
                                ::core::line!(),
                                ::core::stringify!($row),
                            ),
                            CLOCK,
                        );
                        let report = $crate::runner::run_one(&binding, &invocation);
                        $crate::runner::lens_verdict(&report)
                    }
                )+
            )+
        }
    };
}

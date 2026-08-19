//! The stamp battery: the harness's own `macro_rules!` road from a declaration
//! of rows to the seats a test host runs.
//!
//! It lives here, with the vocabulary it reads, because everything it names on
//! the descriptor side is this home's: the table constructor, the binding
//! constructor, the name parsers, and the one refusal family the whole road
//! answers with.
//!
//! # A macro body is tokens
//!
//! Nothing in this file is compiled where it is written. A `macro_rules!` body
//! is a token sequence, expanded at the site that invokes it, so this home gains
//! no dependency edge from anything the expansion names — including
//! [`crate::runner`], which sits above it. `$crate` names THIS crate wherever
//! the expansion lands, whatever a consumer renamed the dependency to, so the
//! rename twins hold without the stamp knowing either name.
//!
//! # What the expansion demands
//!
//! The stamp is the road that INSTANTIATES the descriptor vocabulary's two type
//! parameters. A [`Binding`](crate::descriptor::Binding) is generic in its
//! invocation facts and its conclusion because this home may not import a record
//! type; the stamp expands at the ENGINE's instantiation of them —
//! [`Invocation`](crate::runner::Invocation) in,
//! [`TrialConclusion`](crate::report::TrialConclusion) out — because that is the
//! pair the engine the seats call runs at. The invocation facts are the whole
//! invocation and not its budgets alone: the target the run stands on, the site
//! its reports are written at, and the caller's clock ride there beside the
//! budgets, and a callable is handed all of it. A table whose bindings carry
//! another pair is a lawful table and does not go through this stamp.
//!
//! The expansion names the record vocabulary — the budgets it stamps as a
//! constant, the target and the site it hands the engine, the conclusion the
//! seam is over — and it names the engine. Past `core` and `std` it names no
//! other home. The complete list of what it demands OF THE ENGINE, so that the
//! engine's own home has the contract in writing rather than in a reader's
//! head:
//!
//! - `runner::TrialBinding` and `runner::TrialTable`, the two aliases spelling
//!   the instantiation above, so no expansion writes the parameters by hand.
//! - `runner::HostClock`, whose one declared road to a reading is a `const`, so
//!   a table's clock is stamped as a constant beside its budgets.
//! - `runner::Invocation::declared(InvocationProfile, TargetBinding, TrialSite,
//!   HostClock) -> Invocation`, called once per seat and once per lens, so a
//!   report carries the site of the seat that ran it.
//! - `runner::Selection`, with the arm
//!   `ByExecutionSuite(BTreeSet<ExecutionSuite>)` — one aggregate seat names
//!   exactly one suite and hands it in as the one-element set the arm is over.
//! - `runner::run_all(&TrialTableView<'_>, &Selection, &Invocation) ->
//!   RunReport`.
//! - `runner::run_one(&TrialBinding, &Invocation) -> TrialReport`.
//! - `runner::SeatRefusal`, the seats' one refusal type, which is `Debug` and
//!   carries `From<TrialTableRefusal>` so that every construction refusal on
//!   this road reaches a seat unchanged.
//! - `runner::seat_verdict(&RunReport) -> Result<(), SeatRefusal>`.
//! - `runner::lens_verdict(&TrialReport) -> Result<(), SeatRefusal>`.
//!
//! The two verdict readings are the engine's rather than the stamp's on
//! purpose: a fold over a report written into every expansion would be a
//! calculator standing in as many places as there are invocations.
//!
//! # Where a host fact enters
//!
//! A run stands on facts no library here can honestly read: which target it was
//! compiled for, which toolchain built it, and what a nanosecond reading is
//! worth. This crate reads none of them. The environment is not a declared port
//! here, no compile-time variable states a triple in an ordinary build, and a
//! triple assembled out of `cfg!` predicates would be a PLAUSIBLE SPELLING of a
//! fact rather than the fact — entering an execution key, which is a cache key,
//! so a wrong one buys a hit nothing verified. A toolchain identity has no
//! `cfg!` road at all.
//!
//! The stamp expands at a TEST TARGET, which is the caller's own hosting world,
//! and a test harness may read the host facts it needs to run. So the two are
//! DECLARED at the invocation, each in its own clause, and the expansion carries
//! them through untouched:
//!
//! - `target:` states the [`TargetBinding`](crate::report::TargetBinding) the
//!   run stands on. It has no default and cannot have one: nothing in this crate
//!   derives a target triple or a toolchain identity, so a default would be this
//!   home guessing at a coordinate of a cache key. A consumer that wants them
//!   read rather than typed writes a build script in ITS OWN crate that emits
//!   them and spells `env!` over its own variables — a documented recipe in the
//!   caller's world, never machinery here.
//! - `clock:` states the [`HostClock`](crate::runner::HostClock) a duration is
//!   the difference of two readings from. A caller with no measurement to offer
//!   writes [`HostClock::unmeasured()`](crate::runner::HostClock::unmeasured),
//!   which is a named, documented non-measurement rather than a silent zero:
//!   every duration then reads zero, and zero states that nothing was measured.
//!
//! Neither clause is optional, and that is the point. A run's host facts are
//! written where a reader can see them, and the honest answer to "nothing was
//! measured" has a name a person types.
//!
//! The third host-shaped fact, the SITE, is neither declared nor read: it is
//! where the seat is WRITTEN, which the expansion already knows and an author
//! would only be copying. `module_path!`, `file!`, `line!`, and the seat's own
//! identifier are compile-time facts of the invocation site rather than ambient
//! readings, so the stamp fills the site itself.
//!
//! # Seats refuse; they do not panic
//!
//! Every seat the stamp writes is a test function returning a `Result`, so a
//! failure is a returned typed value carrying its evidence — the way the rest of
//! this harness fails. No expansion contains an unwrap, an expectation, an
//! assertion, a panic, or an index.

/// Stamps one complete authored world, one aggregate seat per declared
/// execution suite, and one named lens per row, from a single declaration.
///
/// # The grammar
///
/// Two invocation forms, differing only in the provenance the TABLE states. The
/// unproduced form:
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
///
///         suite <seat> named(<namespace literal>, <stem literal>) {
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
/// Every part, exhaustively:
///
/// - `<module>` is the stamped module's name, and the visibility in front of
///   `mod` is carried onto the module and onto every declaration it holds — the
///   table and target functions, the invocation constant, the clock constant —
///   together, so no public road ever ends at a private one.
/// - `named(<namespace>, <stem>)` after `mod` is the AUTHORED TABLE's own
///   namespaced name — two literals, parsed through the public name
///   constructor at run time, because a name that states no owner is refused
///   rather than stamped.
/// - `provenance:` is one of the two forms above. `unproduced` states that no
///   producer stands behind this table. `produced(<namespace>, <stem>) against
///   <expression>` states which producer emitted it and which schema identity
///   it emitted against; the expression evaluates to
///   `Result<GeneratedSupportSchemaId, TrialTableRefusal>`, which is the shape a
///   producer's own identity road already has:
///
///   ```text
///   GeneratedSupportSchema::published()
///       .map_err(TrialTableRefusal::SchemaNotDeclared)
///       .and_then(|schema| {
///           schema.identity().map_err(TrialTableRefusal::SchemaNotEncoded)
///       })
///   ```
///
/// - `invocation:` is an expression evaluating to an
///   [`InvocationProfile`](crate::report::InvocationProfile). It is stamped as a
///   `const` item, deliberately: an ambient fact cannot appear in a `const`, so
///   a reading, an environment value, or an argument in this position is refused
///   by the compiler rather than by a rule somebody follows.
/// - `target:` is an expression evaluating to a
///   [`TargetBinding`](crate::report::TargetBinding) — the triple and toolchain
///   this table's runs stand on, DECLARED because nothing in this crate can
///   derive them and a guess would enter a cache key. It is stamped as a
///   function rather than a constant only because the binding owns its
///   spellings and an owned string is not a `const`; it is written once and both
///   spellings read that one.
/// - `clock:` is an expression evaluating to a
///   [`HostClock`](crate::runner::HostClock), stamped as a `const` item for the
///   same reason the budgets are one — what a clock declares is the ROAD to a
///   reading, a function pointer, and never a reading, so no ambient value fits
///   this seat either. A table that measures nothing writes
///   [`HostClock::unmeasured()`](crate::runner::HostClock::unmeasured) and every
///   duration it records reads zero, stating that nothing was measured.
/// - `suite <seat> named(<namespace>, <stem>) { … }` declares ONE aggregate seat.
///   `<seat>` is the test function's name; the two literals are the
///   [`ExecutionSuite`](crate::descriptor::ExecutionSuite) the seat selects on.
///   At least one suite group is required, and each group requires at least one
///   row.
/// - `<row>: <expression>` declares one row. `<row>` names its lens; the
///   expression answers with one [`runner::TrialBinding`](crate::runner::TrialBinding)
///   — the engine's own instantiation of the seam — or refuses in any family
///   that discharges into
///   [`TrialTableRefusal`](crate::descriptor::TrialTableRefusal), which is the
///   family the stamp writes the `?` for. A bare call to the public binding
///   constructor refuses in
///   [`BindingRefusal`](crate::descriptor::BindingRefusal) and travels that
///   road; an expression that builds its own parts writes `?` on each
///   construction, and every one of those is governed by this same family,
///   which is why the family carries a discharge for each construction on the
///   road to a binding. The stamp never reads inside the expression: a row's
///   internals are the producer's statement, and a macro that parsed them would
///   be a second authority over this vocabulary.
///
/// Both grammars end their clauses and their rows with a comma. The `@`-prefixed
/// rule below is the stamp's internal transcription, not an invocation form.
///
/// # What is stamped
///
/// One module, containing: a private `row` module with one function per declared
/// row, so each declared expression is written exactly once and both spellings
/// read the same one; a `table` function building the complete authored world
/// through the public constructors; an `INVOCATION` constant and a `CLOCK`
/// constant; a `target` function answering the declared host binding; one
/// ordinary `#[test]` function per suite group, which runs by default; and one
/// `#[test] #[ignore = "lens"]` function per row, which is clickable in an
/// editor and runnable by name, and never paid for twice in an ordinary run.
///
/// Each seat and each lens builds its OWN invocation from those three
/// declarations plus the site it is written at, so a report carries the site of
/// the seat that produced it rather than one site the whole table shared.
///
/// # Authority
///
/// EVERY declared row lands in the ONE table, whichever group it was declared
/// under. The grouping decides which aggregate seat EXISTS; it never decides
/// which rows the world holds, because a selection narrows a run and never the
/// denominator.
///
/// A group is a seat declaration and not a claim about the rows in it: the
/// selection reads each ROW's own execution suite, so a row grouped under a seat
/// whose suite is not the row's own is simply not selected by that seat, and the
/// run's census says so in the open. The stamp cannot check the pairing without
/// reading inside a row expression, and it does not pretend to. What it cannot
/// check at expansion the engine answers at run time: a seat whose selection
/// named no row of the denominator refuses, so a group that pairs with nothing
/// is a red seat rather than a green one that ran nothing.
///
/// # Construction
///
/// Every value the stamp builds is built through this home's public
/// constructors, and every refusal they answer with is carried unchanged into
/// [`TrialTableRefusal`](crate::descriptor::TrialTableRefusal). A declared row
/// expression's own constructions travel the same road: the stamp puts each row
/// in a function refusing in that one family, so the `?` a producer's expression
/// writes on a name, a roster, a row, or a schema identity is discharged by the
/// conversion the family declares rather than by a variant the producer
/// invented. Nothing is unwrapped, asserted, or indexed anywhere in the
/// expansion.
///
/// # Bounds
///
/// A seat's name and a row's name share one namespace, because both are
/// functions in the stamped module. Two seats, two rows, or a seat and a row
/// declared under one name is an ordinary duplicate definition, and the compiler
/// says which. The stamp writes items into that same namespace — the `row`
/// module and the `table` and `target` functions, and `INVOCATION` and `CLOCK`
/// beside them — so those five spellings are taken, and a seat or row claiming
/// one is the same ordinary duplicate with the same ordinary diagnostic.
///
/// The seats are `#[test]` functions, so the stamp is invoked where a test
/// harness collects them: a test target, or a test-configured module. In an
/// ordinary build a test function is a function nobody calls, which is what the
/// host will say about it.
///
/// The forms above are shown as text rather than as compiled examples: a
/// compiled one needs a subject, a check, and a population, and those live on
/// the challenge side rather than on this page.
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

    // THE ONE TRANSCRIPTION. Both invocation forms arrive here with their
    // provenance already assembled into one expression of one type, so the
    // module below is written once and neither form can drift from the other.
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
        /// One stamped trial table: the complete authored world its rows
        /// declare, one aggregate seat per declared execution suite, and one
        /// ignored lens per row.
        $vis mod $module {
            /// One function per declared row, each answering exactly the
            /// binding its declaration states.
            ///
            /// A row's expression is written ONCE, here, and both spellings
            /// read that one: the table collects these functions, and each
            /// named lens calls its own. A second copy of the expression would
            /// be a second row that agreed by accident.
            mod row {
                $(
                    $(
                        /// One declared row's binding.
                        ///
                        /// # Errors
                        ///
                        /// Refuses whatever the declaration's own constructions
                        /// refuse, each carried into the stamp's one family by
                        /// the discharge that family declares for it — the
                        /// binding constructor's refusal included, which is what
                        /// the `?` on the whole expression is.
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
            /// A `const` item, deliberately: an ambient fact cannot appear in
            /// one, so these budgets are DECLARED rather than read off the
            /// host.
            $vis const INVOCATION: $crate::report::InvocationProfile = $invocation;

            /// The clock every seat in this table measures with.
            ///
            /// A `const` item for the same reason the budgets are one: what a
            /// clock declares is the ROAD to a reading — a function pointer —
            /// and never a reading, so no ambient value fits this seat either.
            /// A table that measures nothing declares the unmeasured clock, and
            /// every duration it records then reads zero, which states that
            /// nothing was measured.
            $vis const CLOCK: $crate::runner::HostClock = $clock;

            /// The target and toolchain this table's runs stand on.
            ///
            /// Declared rather than read: nothing in the harness derives a
            /// target triple or a toolchain identity, and these facts enter an
            /// execution key, so a guess would buy a cache hit nothing
            /// verified. A test target is the caller's own hosting world, which
            /// is where a host fact is allowed to be stated.
            ///
            /// A function rather than a constant only because the binding owns
            /// its spellings and an owned string is not a `const`. The declared
            /// expression is written ONCE, here, and every seat and every lens
            /// reads that one.
            $vis fn target() -> $crate::report::TargetBinding {
                $target
            }

            /// The complete authored world these rows declare.
            ///
            /// # Authority
            ///
            /// Every declared row is here, whichever group it was declared
            /// under: the grouping chooses which aggregate seat exists, never
            /// which rows the world holds.
            ///
            /// # Errors
            ///
            /// Refuses the first construction that refused, in the order the
            /// stamp builds them: the table's name, the stated provenance, each
            /// row's binding in declared order, then the authored world itself.
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
                /// One aggregate seat: the complete world, narrowed to the rows
                /// whose own execution suite is this seat's, through the pure
                /// engine.
                ///
                /// An ordinary test function, so it runs by default.
                ///
                /// The invocation is built here rather than shared, so the site
                /// every report of this run carries is THIS seat's: a
                /// descriptor row carries no site of its own for the engine to
                /// read.
                ///
                /// # Errors
                ///
                /// Refuses when the world could not be built, when this seat's
                /// suite name could not be parsed, or when the run's own
                /// verdict refuses — including the run that selected nothing,
                /// which is the suite pairing the stamp cannot check.
                #[test]
                fn $seat() -> ::core::result::Result<(), $crate::runner::SeatRefusal> {
                    let world = table()?;
                    let suite = $crate::descriptor::ExecutionSuite::named(
                        $suite_namespace,
                        $suite_stem,
                    )
                    .map_err($crate::descriptor::TrialTableRefusal::NameNotParsed)?;
                    let view = world.view();
                    let selection = $crate::runner::Selection::ByExecutionSuite(
                        ::std::collections::BTreeSet::from([suite]),
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
                    $crate::runner::seat_verdict(&report)
                }
            )+

            $(
                $(
                    /// One named lens: this row alone, through the same engine
                    /// the aggregate seat calls.
                    ///
                    /// Ignored by default — clickable in an editor and runnable
                    /// by name or filter, and never paid for twice in an
                    /// ordinary run.
                    ///
                    /// The invocation is built here too, from the same three
                    /// declarations the aggregate seat reads, so a lens and a
                    /// seat differ in the site they state and in nothing else.
                    ///
                    /// # Errors
                    ///
                    /// Refuses when this row's binding could not be built, or
                    /// when the trial's own verdict refuses.
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

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
//! type; the stamp expands at the report instrument's instantiation of them —
//! [`InvocationProfile`](crate::report::InvocationProfile) in,
//! [`TrialConclusion`](crate::report::TrialConclusion) out — because that is the
//! pair the engine the seats call runs at. A table whose bindings carry another
//! pair is a lawful table and does not go through this stamp.
//!
//! The expansion also names the engine, and names nothing else outside this
//! home. The complete list, so that the engine's own home has the contract in
//! writing rather than in a reader's head:
//!
//! - `runner::Selection`, with the arm `ByExecutionSuite(ExecutionSuite)` — what
//!   one aggregate seat chooses from the complete world.
//! - `runner::run_all(&TableView<'_, I, C>, &Selection, &I) -> RunReport`.
//! - `runner::run_one(&Binding<I, C>, &I) -> TrialReport`.
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
///   `mod` is carried onto the module, its table function, and its invocation
///   constant together, so no public road ever ends at a private one.
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
///   a clock, an environment value, or an argument in this position is refused
///   by the compiler rather than by a rule somebody follows.
/// - `suite <seat> named(<namespace>, <stem>) { … }` declares ONE aggregate seat.
///   `<seat>` is the test function's name; the two literals are the
///   [`ExecutionSuite`](crate::descriptor::ExecutionSuite) the seat selects on.
///   At least one suite group is required, and each group requires at least one
///   row.
/// - `<row>: <expression>` declares one row. `<row>` names its lens; the
///   expression evaluates to
///   `Result<Binding<InvocationProfile, TrialConclusion>, BindingRefusal>` — a
///   call to the public binding constructor, whatever built its parts. The
///   stamp never reads inside it: a row's internals are the producer's
///   statement, and a macro that parsed them would be a second authority over
///   this vocabulary.
///
/// Both grammars end their clauses and their rows with a comma. The `@`-prefixed
/// rule below is the stamp's internal transcription, not an invocation form.
///
/// # What is stamped
///
/// One module, containing: a private `row` module with one function per declared
/// row, so each declared expression is written exactly once and both spellings
/// read the same one; a `table` function building the complete authored world
/// through the public constructors; an `INVOCATION` constant; one ordinary
/// `#[test]` function per suite group, which runs by default; and one
/// `#[test] #[ignore = "lens"]` function per row, which is clickable in an
/// editor and runnable by name, and never paid for twice in an ordinary run.
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
/// reading inside a row expression, and it does not pretend to.
///
/// # Construction
///
/// Every value the stamp builds is built through this home's public
/// constructors, and every refusal they answer with is carried unchanged into
/// [`TrialTableRefusal`](crate::descriptor::TrialTableRefusal). Nothing is
/// unwrapped, asserted, or indexed anywhere in the expansion.
///
/// # Bounds
///
/// A seat's name and a row's name share one namespace, because both are
/// functions in the stamped module. Two seats, two rows, or a seat and a row
/// declared under one name is an ordinary duplicate definition, and the compiler
/// says which.
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
                        /// Refuses whatever the binding constructor refuses.
                        pub(super) fn $row() -> ::core::result::Result<
                            $crate::descriptor::Binding<
                                $crate::report::InvocationProfile,
                                $crate::report::TrialConclusion,
                            >,
                            $crate::descriptor::BindingRefusal,
                        > {
                            $binding
                        }
                    )+
                )+
            }

            /// The invocation every seat in this table runs under.
            ///
            /// A `const` item, deliberately: an ambient fact cannot appear in
            /// one, so these budgets are DECLARED rather than read off the
            /// host.
            $vis const INVOCATION: $crate::report::InvocationProfile = $invocation;

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
                $crate::descriptor::AuthoredTable<
                    $crate::report::InvocationProfile,
                    $crate::report::TrialConclusion,
                >,
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
                            row::$row().map_err(
                                $crate::descriptor::TrialTableRefusal::BindingNotBound,
                            )?,
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
                /// # Errors
                ///
                /// Refuses when the world could not be built, when this seat's
                /// suite name could not be parsed, or when the run's own
                /// verdict refuses.
                #[test]
                fn $seat() -> ::core::result::Result<(), $crate::runner::SeatRefusal> {
                    let world = table()?;
                    let suite = $crate::descriptor::ExecutionSuite::named(
                        $suite_namespace,
                        $suite_stem,
                    )
                    .map_err($crate::descriptor::TrialTableRefusal::NameNotParsed)?;
                    let view = world.view();
                    let selection = $crate::runner::Selection::ByExecutionSuite(suite);
                    let report = $crate::runner::run_all(&view, &selection, &INVOCATION);
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
                    /// # Errors
                    ///
                    /// Refuses when this row's binding could not be built, or
                    /// when the trial's own verdict refuses.
                    #[test]
                    #[ignore = "lens"]
                    fn $row() -> ::core::result::Result<(), $crate::runner::SeatRefusal> {
                        let binding = row::$row().map_err(
                            $crate::descriptor::TrialTableRefusal::BindingNotBound,
                        )?;
                        let report = $crate::runner::run_one(&binding, &INVOCATION);
                        $crate::runner::lens_verdict(&report)
                    }
                )+
            )+
        }
    };
}

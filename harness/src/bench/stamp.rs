//! One authored table spelling, transcribed into the same constructors a hand-written caller uses.

/// Declare a benchmark-table function from a table written as a block.
///
/// The function returns [`BenchStampRefusal`](crate::bench::BenchStampRefusal), keeping whichever constructor refused.
/// The stamp itself owns no row grammar, no judgment, no host fact, no identity, and no reporter.
#[macro_export]
macro_rules! bench_table {
    (
        $(#[$note:meta])*
        $vis:vis fn $table:ident named($namespace:literal, $stem:literal) {
            provenance: $provenance:expr,
            bindings: [
                $($binding:expr),+ $(,)?
            ],
        }
    ) => {
        $(#[$note])*
        $vis fn $table() -> ::core::result::Result<
            $crate::bench::BenchTable,
            $crate::bench::BenchStampRefusal,
        > {
            let bindings = ::std::vec![
                $(($binding)?),+
            ];
            let table = $crate::bench::BenchTable::authored(
                $crate::bench::BenchTableName::named($namespace, $stem)?,
                $provenance,
                bindings,
            )?;
            ::core::result::Result::Ok(table)
        }
    };
}

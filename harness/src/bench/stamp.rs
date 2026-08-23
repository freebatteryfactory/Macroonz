//! Thin transcription from one authored table spelling into the public benchmark constructors.

/// Declare one benchmark-table function through the same constructors a handwritten caller uses.
///
/// The function derives its denominator from the bindings supplied here and returns [`BenchStampRefusal`](crate::bench::BenchStampRefusal) without flattening any constructor cause. The stamp owns no row grammar, work judgment, host fact, identity algorithm, or reporter.
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

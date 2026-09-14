use std::io::Write;

fn main() -> std::io::Result<()> {
    let values = [2u64, 3, 7];
    let count = values.into_iter().product::<u64>();
    writeln!(std::io::stdout(), "{count}")
}

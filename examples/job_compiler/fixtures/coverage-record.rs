#![forbid(unsafe_code)]

use neutral_job_adopter::job::Record;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes)?;
    match Record::decode_canonical(&bytes) {
        Ok(record) => accepted(record),
        Err(_) => refused(),
    }
    Ok(())
}

fn accepted(record: Record) {
    std::hint::black_box(record.count);
    std::hint::black_box(record.attempts);
}

fn refused() {
    std::hint::black_box("record-refused");
}

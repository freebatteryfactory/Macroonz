#![forbid(unsafe_code)]

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    std::hint::black_box(classify(&input));
    Ok(())
}

fn classify(input: &[u8]) -> u64 {
    match input {
        [0, tail @ ..] => zero(tail),
        [1, tail @ ..] => one(tail),
        [] => 0,
        _ => 1,
    }
}

fn zero(tail: &[u8]) -> u64 {
    if tail.contains(&0x7f) { 11 } else { 7 }
}

fn one(tail: &[u8]) -> u64 {
    if tail.len() > 1 { 23 } else { 19 }
}

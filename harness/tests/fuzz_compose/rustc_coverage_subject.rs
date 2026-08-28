#![forbid(unsafe_code)]

use std::io::{self, Read};

fn main() -> io::Result<()> {
    if std::env::args().any(|argument| argument == "--park-before-read") {
        loop {
            std::thread::park();
        }
    }
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    match input.as_slice() {
        [0xfe] => loop {
            std::thread::park();
        },
        [0xff] => std::process::abort(),
        _ => {}
    }
    std::hint::black_box(classify(&input));
    Ok(())
}

fn classify(input: &[u8]) -> u64 {
    match input {
        [0, tail @ ..] => zero(tail),
        [1, tail @ ..] => one(tail),
        [0x80, tail @ ..] => high(tail),
        [] => 0,
        _ => 1,
    }
}

fn zero(tail: &[u8]) -> u64 {
    if tail.contains(&0x7f) {
        11
    } else {
        7
    }
}

fn one(tail: &[u8]) -> u64 {
    if tail.len() > 1 {
        23
    } else {
        19
    }
}

fn high(tail: &[u8]) -> u64 {
    u64::try_from(tail.len()).unwrap_or(u64::MAX)
}

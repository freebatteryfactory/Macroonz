use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut input = Vec::new();
    std::io::stdin().take(1024).read_to_end(&mut input)?;
    let value = match input.as_slice() {
        [0, ..] => zero(),
        [1, tail @ ..] if tail.len() > 1 => many(),
        _ => 3_u64,
    };
    std::hint::black_box(value);
    Ok(())
}

fn zero() -> u64 { 11 }
fn many() -> u64 { 23 }

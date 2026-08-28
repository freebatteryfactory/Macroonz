#![doc = include_str!("../README.md")]

use macroonz_compiler::host::{Spans, capture};
use proc_macro::{Literal, TokenStream, TokenTree};

/// Capture the compiler's real input and expand to one literal containing its canonical bytes as lowercase hexadecimal.
#[proc_macro]
pub fn canonical_capture(input: TokenStream) -> TokenStream {
    let mut spans = Spans::empty();
    let observed = match capture(input, &mut spans) {
        Ok(captured) => hexadecimal(&captured.canonical_bytes()),
        Err(refusal) => format!("capture-refused:{refusal}"),
    };
    TokenStream::from(TokenTree::Literal(Literal::string(&observed)))
}

/// Lowercase hexadecimal for one canonical byte string.
fn hexadecimal(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        text.push(hex_digit(byte >> 4));
        text.push(hex_digit(byte & 0x0f));
    }
    text
}

/// One lowercase hexadecimal digit for a masked nibble.
const fn hex_digit(nibble: u8) -> char {
    match nibble {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}

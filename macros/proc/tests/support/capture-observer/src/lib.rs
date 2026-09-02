#![doc = include_str!("../README.md")]

use macroonz_compiler::host::{Spans, capture, emit, emit_tree};
use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{CrateBinding, Door, Producer};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// One declared door for the recipe emission observer.
const RECIPE_DOOR: Door = Door::declared(
    "macroonz",
    "macroonz.recipe-observer",
    "macroonz_capture_observer::recipe_module_span",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "macroonz",
        name: "macroonz-capture-observer",
    },
);

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

/// Capture the compiler's real input and return the structurally preserved generated tokens through the real host emitter.
#[proc_macro]
pub fn round_trip(input: TokenStream) -> TokenStream {
    let mut spans = Spans::empty();
    let captured = match capture(input, &mut spans) {
        Ok(captured) => captured,
        Err(refusal) => return refused(&refusal.to_string()),
    };
    let generated = match captured.fragment().generated() {
        Ok(generated) => generated,
        Err(refusal) => return refused(&refusal.to_string()),
    };
    match emit_tree(&generated, &spans) {
        Ok(emitted) => emitted,
        Err(refusal) => refused(&refusal.to_string()),
    }
}

/// Report whether a baked recipe module restores the exact authored body-group span.
#[proc_macro]
pub fn recipe_module_span(input: TokenStream) -> TokenStream {
    let authored = last_group_source(input.clone());
    let mut spans = Spans::empty();
    let captured = match capture(input, &mut spans) {
        Ok(captured) => captured,
        Err(refusal) => return refused(&refusal.to_string()),
    };
    let baked =
        match macroonz_compiler::recipe::bake(&captured, HarnessPosture::Available, &RECIPE_DOOR) {
            Ok(baked) => baked,
            Err(refusal) => return refused(refusal.summary()),
        };
    let emitted = match emit(&baked, &spans) {
        Ok(emitted) => emitted,
        Err(refusal) => return refused(&refusal.to_string()),
    };
    let answer = if authored.is_some() && authored == last_group_source(emitted) {
        "true"
    } else {
        "false"
    };
    TokenStream::from(TokenTree::Ident(Ident::new(answer, Span::call_site())))
}

/// Read the source text attached to the final top-level group.
fn last_group_source(stream: TokenStream) -> Option<String> {
    let mut source = None;
    for token in stream {
        if let TokenTree::Group(group) = token {
            source = group.span().source_text();
        }
    }
    source
}

/// One invocation-sited `compile_error!` carrying a typed refusal rendered by its owner.
fn refused(message: &str) -> TokenStream {
    let span = Span::call_site();
    let mut bang = Punct::new('!', Spacing::Alone);
    bang.set_span(span);
    let mut terminator = Punct::new(';', Spacing::Alone);
    terminator.set_span(span);
    let mut line = Literal::string(message);
    line.set_span(span);
    let mut argument = Group::new(
        Delimiter::Parenthesis,
        TokenStream::from(TokenTree::Literal(line)),
    );
    argument.set_span(span);
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(bang),
        TokenTree::Group(argument),
        TokenTree::Punct(terminator),
    ]
    .into_iter()
    .collect()
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

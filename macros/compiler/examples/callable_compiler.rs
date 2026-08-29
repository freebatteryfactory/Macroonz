//! A complete one-unit callable compiler built from the crate's public batteries.

use macroonz_compiler::{
    CapturedInput, CrateBinding, Diagnostic, Door, Expansion, GeneratedToken, GeneratedTree, Kind,
    NoQuestions, Producer, Request, SoleRole, TextCapture, constant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GreetImpl;

impl Kind for GreetImpl {
    const NAME: &'static str = "greet.impl";
    type Content = &'static str;
    type Role = SoleRole;
    type Question = NoQuestions;
}

const GREET_DOOR: Door = Door::declared(
    "greet",
    "greet.declaration",
    "greet::compile",
    CrateBinding::declared("macroonz_compiler"),
    Producer {
        namespace: "greet",
        name: "compiler",
    },
);

fn compile(capture: CapturedInput) -> Result<Expansion<GreetImpl>, Diagnostic> {
    let greeting = match capture.trees() {
        [tree] if tree.word() == Some("world") => "world",
        _ => "stranger",
    };
    Request::<GreetImpl>::over(capture, greeting, &GREET_DOOR).render(|plan, out| {
        let answer = match *plan.content() {
            "world" => 42,
            _ => 0,
        };
        let mut tokens = vec![GeneratedToken::word("pub")];
        tokens.extend(constant(
            "GREETING",
            vec![GeneratedToken::word("u64")],
            vec![GeneratedToken::number(answer)],
        ));
        out.unit(SoleRole::Sole, GeneratedTree::assembled(tokens)?)
    })
}

fn main() -> Result<(), String> {
    let captured = TextCapture::read("world").map_err(|error| error.to_string())?;
    let expansion =
        compile(captured.input().clone()).map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let emitted = expansion
        .emit()
        .tokens()
        .ok_or_else(|| "the declaration-site delivery was not planned".to_owned())?;
    assert_eq!(emitted.inspected(), "pub const GREETING : u64 = 42 ; ");
    Ok(())
}

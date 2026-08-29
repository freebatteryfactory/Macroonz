# Docs-blind custom-macro specimen

This readable custody packet retains the exact authored inputs used by the Macroonz 0.2 Q1 custom-macro census without retaining another Cargo workspace.
Every file below ends with one LF byte.

## `Cargo.toml`

```toml
[workspace]
members = ["subject", "subject-macros"]
resolver = "3"

[workspace.package]
edition = "2024"
publish = false
rust-version = "1.98.0"

[workspace.dependencies]
macroonz-compiler = { version = "=0.1.0", default-features = false }

[workspace.lints.rust]
unsafe_code = "forbid"
warnings = "deny"
```

## `rust-toolchain.toml`

```toml
[toolchain]
channel = "1.98.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

## `subject-macros/Cargo.toml`

```toml
[package]
name = "subject-macros"
version = "0.0.0"
edition.workspace = true
publish.workspace = true
rust-version.workspace = true

[lib]
proc-macro = true

[dependencies]
macroonz-compiler = { workspace = true, features = ["host"] }

[lints]
workspace = true
```

## `subject-macros/src/lib.rs`

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

extern crate proc_macro;

use macroonz_compiler::{
    CanonicalContent, CapturedInput, CrateBinding, Diagnostic, Door, Expansion, GeneratedToken,
    GeneratedTree, Kind, NoQuestions, Output, Plan, Producer, RenderError, Request, SoleRole,
    constant,
};
use proc_macro::TokenStream;

const DOOR: Door = Door::declared(
    "subject",
    "answer-declaration",
    "declare-answer",
    CrateBinding::declared("macroonz_compiler"),
    Producer {
        namespace: "subject",
        name: "answer-declaration",
    },
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AnswerItem;

impl Kind for AnswerItem {
    const NAME: &'static str = "answer-item";
    type Content = Mode;
    type Question = NoQuestions;
    type Role = SoleRole;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Emit,
    Omit,
}

impl CanonicalContent for Mode {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.push(match self {
            Self::Emit => 0,
            Self::Omit => 1,
        });
    }
}

fn read_mode(input: &CapturedInput) -> Mode {
    match input.trees() {
        [tree] if tree.word() == Some("lawful") => Mode::Emit,
        _ => Mode::Omit,
    }
}

fn answer_tree() -> Result<GeneratedTree, RenderError> {
    let mut tokens = vec![GeneratedToken::word("pub")];
    tokens.extend(constant(
        "GENERATED_ANSWER",
        vec![GeneratedToken::word("u64")],
        vec![GeneratedToken::number(42)],
    ));
    Ok(GeneratedTree::assembled(tokens)?)
}

fn render_answer(
    plan: &Plan<AnswerItem>,
    output: &mut Output<'_, AnswerItem>,
) -> Result<(), RenderError> {
    if plan.content() == &Mode::Omit {
        return Ok(());
    }

    output.unit(SoleRole::Sole, answer_tree()?)
}

fn compile(input: CapturedInput) -> Result<Expansion<AnswerItem>, Diagnostic> {
    let mode = read_mode(&input);
    Request::<AnswerItem>::over(input, mode, &DOOR).render(render_answer)
}

#[proc_macro]
pub fn declare_answer(input: TokenStream) -> TokenStream {
    macroonz_compiler::host::expand(input, compile)
}

#[cfg(test)]
mod tests {
    use super::{answer_tree, compile};
    use macroonz_compiler::{Phase, TextCapture};

    #[test]
    fn direct_compiler_matches_the_proc_road_boundary() {
        let capture = TextCapture::read("lawful").unwrap_or_else(|error| panic!("{error}"));
        let expansion =
            compile(capture.input().clone()).unwrap_or_else(|error| panic!("{error:?}"));
        let emitted = expansion
            .emit()
            .tokens()
            .unwrap_or_else(|| panic!("declaration-site cargo was not planned"));
        let expected = answer_tree().unwrap_or_else(|error| panic!("{error}"));

        assert_eq!(expansion.plan().membership().count(), 1);
        assert_eq!(emitted.canonical_bytes(), expected.canonical_bytes());
        assert_eq!(emitted.inspected(), expected.inspected());
    }

    #[test]
    fn omitted_output_is_refused() {
        let capture = TextCapture::read("omit").unwrap_or_else(|error| panic!("{error}"));
        let diagnostic = compile(capture.input().clone()).expect_err("omitted output must refuse");

        assert_eq!(diagnostic.phase(), Phase::Rendering);
    }
}
```

## `subject/Cargo.toml`

```toml
[package]
name = "subject"
version = "0.0.0"
edition.workspace = true
publish.workspace = true
rust-version.workspace = true

[features]
hostile = []
non-vacuity = []

[dependencies]
subject-macros = { path = "../subject-macros" }

[[bin]]
name = "hostile"
path = "src/bin/hostile.rs"
required-features = ["hostile"]

[[bin]]
name = "non-vacuity"
path = "src/bin/non_vacuity.rs"
required-features = ["non-vacuity"]

[lints]
workspace = true
```

## `subject/src/bin/hostile.rs`

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

use subject_macros::declare_answer;

declare_answer!(omit);

fn main() {}
```

## `subject/src/bin/non_vacuity.rs`

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

fn main() {
    let _ = GENERATED_ANSWER;
}
```

## `subject/src/lib.rs`

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

use subject_macros::declare_answer;

declare_answer!(lawful);

/// Returns the value carried by the generated ordinary Rust item.
pub const fn observed_answer() -> u64 {
    GENERATED_ANSWER
}

#[cfg(test)]
mod tests {
    use super::observed_answer;

    #[test]
    fn generated_item_is_invoked_downstream() {
        assert_eq!(observed_answer(), 42);
    }
}
```

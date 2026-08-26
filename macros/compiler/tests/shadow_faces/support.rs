use macroonz_compiler::descriptor::Grammar;
use macroonz_compiler::descriptor::door;
use macroonz_compiler::descriptor::shadow::{self, ShadowFace};
use macroonz_compiler::{
    CanonicalContent, CrateBinding, Diagnostic, Door, Expansion, Producer, TextCapture,
};

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.shadow.grammar",
    "lane::shadow",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "shadow",
    },
);

/// The shadow grammar this lane registers.
const SHADOW: Grammar = Grammar {
    attribute: "shadow",
};

/// The shadow road walked over one source, or nothing where the lane's own source did not capture.
pub(super) fn shadowed(source: &str) -> Option<Result<Expansion<ShadowFace>, Diagnostic>> {
    let bound = format!("loom = renamed_facade::loom, names = [{source}]");
    shadowed_raw(&bound)
}

/// The shadow road walked over an already-bound source.
pub(super) fn shadowed_raw(source: &str) -> Option<Result<Expansion<ShadowFace>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::shadow(read.input().clone(), SHADOW, &DOOR))
}

/// The declaration-site text one shadow expansion emits.
pub(super) fn emitted(expansion: &Expansion<ShadowFace>) -> Option<String> {
    expansion
        .emit()
        .tokens()
        .map(macroonz_compiler::GeneratedTree::inspected)
}

/// The canonical content one authored source declares, before a request binds it to a capture.
pub(super) fn canonical_content(source: &str) -> Option<Vec<u8>> {
    let read = TextCapture::read(source).ok()?;
    let content = shadow::chosen(read.input(), SHADOW).ok()?;
    let mut bytes = Vec::new();
    content.encode_content_into(&mut bytes);
    Some(bytes)
}

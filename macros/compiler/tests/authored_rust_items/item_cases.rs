//! The declared authored-item cases that exercise the structural lens.

use macroonz_compiler::AuthoredItemKind;

/// One caller-authored item and the structural facts its lens must expose.
pub(super) struct ItemCase {
    /// The complete caller-authored Rust item.
    pub(super) source: &'static str,
    /// The structural family the lens must recognize.
    pub(super) kind: AuthoredItemKind,
    /// The item name when that family owns one at its envelope.
    pub(super) name: Option<&'static str>,
    /// Whether the item itself carries an explicit unsafe qualifier.
    pub(super) safety: ItemSafety,
}

/// The explicit unsafe posture of one caller-authored item.
#[derive(Clone, Copy)]
pub(super) enum ItemSafety {
    /// The item carries no item-level unsafe qualifier.
    Safe,
    /// The item carries an explicit item-level unsafe qualifier.
    Unsafe,
}

impl ItemSafety {
    /// Whether this posture expects an explicit item-level unsafe qualifier.
    pub(super) const fn expects_unsafe(self) -> bool {
        matches!(self, Self::Unsafe)
    }
}

/// Every authored-item family and qualifier posture required by this claim.
pub(super) const REQUIRED_ITEM_CASES: &[ItemCase] = &[
    ItemCase {
        source: "pub mod room { pub const WIDTH: usize = 4; }",
        kind: AuthoredItemKind::Module,
        name: Some("room"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "#[doc = \"state\"] pub enum State<'a, T, const N: usize> where T: 'a { Ready(&'a T), Bytes([u8; N]) }",
        kind: AuthoredItemKind::Enumeration,
        name: Some("State"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub union Storage { word: u64, bytes: [u8; 8] }",
        kind: AuthoredItemKind::Union,
        name: Some("Storage"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub(crate) struct Packet<T, const N: usize>(T, [u8; N]) where T: Copy;",
        kind: AuthoredItemKind::Structure,
        name: Some("Packet"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub unsafe trait Contract<'a, T> where T: 'a { type Item<'b> where Self: 'b; unsafe fn read(&'a self) -> impl Copy + use<'a, T>; }",
        kind: AuthoredItemKind::Trait,
        name: Some("Contract"),
        safety: ItemSafety::Unsafe,
    },
    ItemCase {
        source: "pub const unsafe extern \"C\" fn apply<'a, T: Copy>(value: &'a mut T) -> impl Copy + use<'a, T> where T: 'a { *value }",
        kind: AuthoredItemKind::Function,
        name: Some("apply"),
        safety: ItemSafety::Unsafe,
    },
    ItemCase {
        source: "pub extern fn default_abi() {}",
        kind: AuthoredItemKind::Function,
        name: Some("default_abi"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub const extern fn constant_default_abi() {}",
        kind: AuthoredItemKind::Function,
        name: Some("constant_default_abi"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "unsafe impl<'a, T> Contract<'a, T> for Packet<T, 4> where T: Copy + 'a { type Item<'b> = &'b T where Self: 'b; unsafe fn read(&'a self) -> impl Copy + use<'a, T> { &self.0 } }",
        kind: AuthoredItemKind::Implementation,
        name: None,
        safety: ItemSafety::Unsafe,
    },
    ItemCase {
        source: "pub type Alias<T> where T: Copy = T;",
        kind: AuthoredItemKind::TypeAlias,
        name: Some("Alias"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub const WIDTH: usize = 4;",
        kind: AuthoredItemKind::Constant,
        name: Some("WIDTH"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub static mut COUNT: usize = 0;",
        kind: AuthoredItemKind::Static,
        name: Some("COUNT"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub static LIMIT: usize = 4;",
        kind: AuthoredItemKind::Static,
        name: Some("LIMIT"),
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "pub use crate::room::WIDTH;",
        kind: AuthoredItemKind::Use,
        name: None,
        safety: ItemSafety::Safe,
    },
    ItemCase {
        source: "extern crate core;",
        kind: AuthoredItemKind::ExternalCrate,
        name: Some("core"),
        safety: ItemSafety::Safe,
    },
];

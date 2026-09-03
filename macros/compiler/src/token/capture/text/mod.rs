#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    TEXT_SOURCE_BYTE_LIMIT, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal,
};

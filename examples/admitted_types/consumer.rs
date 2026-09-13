//! A standalone caller of the source rendered by the admitted-types example.

mod definitions;
use definitions::{BorrowedBatch, Identifier, Label};

#[derive(Debug, PartialEq, Eq)]
pub enum Admission {
    Empty,
}

pub fn admit_identifier(value: u64) -> Result<u64, Admission> {
    if value == 0 { Err(Admission::Empty) } else { Ok(value) }
}

pub fn admit_label(value: String) -> Result<String, Admission> {
    let text = value.trim().to_owned();
    if text.is_empty() { Err(Admission::Empty) } else { Ok(text) }
}

pub fn admit_batch<T: core::fmt::Display, const N: usize>(value: &[T; N]) -> Result<&[T; N], Admission> {
    if N == 0 { Err(Admission::Empty) } else { Ok(value) }
}

impl Identifier {
    pub fn encode(&self) -> [u8; 8] {
        self.as_inner().to_le_bytes()
    }
}

fn main() {
    assert_eq!(Identifier::try_new(0).err(), Some(Admission::Empty));
    let identifier = Identifier::try_new(513).expect("nonzero identifier");
    assert_eq!(identifier.encode(), [1, 2, 0, 0, 0, 0, 0, 0]);
    assert_eq!(identifier.into_inner(), 513);
    let label = Label::try_new("  tea  ".to_owned()).expect("nonempty label");
    assert_eq!(label.as_inner(), "tea");
    assert_eq!(label.into_inner(), "tea");
    let entries = [String::from("mint"), String::from("cocoa")];
    let batch = BorrowedBatch::<String, 2>::try_new(&entries).expect("nonempty batch");
    assert_eq!(batch.as_inner()[1], "cocoa");
    assert!(core::ptr::eq(batch.into_inner(), &entries));
}

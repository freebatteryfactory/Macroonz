//! A complete disposition witness has private fields, so an external kind-set implementation cannot bypass the constructor that checks every seat.

use core::marker::PhantomData;
use macroonz_compiler::{Disposition, DispositionRecord, DispositionSet, KindSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EmptyRecord;

impl DispositionRecord for EmptyRecord {
    fn into_dispositions(self) -> impl Iterator<Item = (&'static str, Disposition)> {
        core::iter::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ForgedKinds;

impl KindSet for ForgedKinds {
    type Dispositions = EmptyRecord;

    const NAMES: &'static [&'static str] = &["forged.first", "forged.second"];
}

fn forge() -> DispositionSet<ForgedKinds> {
    DispositionSet {
        dispositions: Vec::new(),
        kind_set: PhantomData,
    }
}

fn main() {
    let _road = forge;
}

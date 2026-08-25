//! An account accepts one content binding and no naked captured-declaration identity.
//!
//! Restoring the old raw-identity constructor makes this fixture compile and reopens the road where content could ride beside an unrelated account.

use macroonz_compiler::{Account, Kind, NoQuestions, SoleRole};
use macroonz_compiler::identity::{self, Identity};

struct Demo;

impl Kind for Demo {
    const NAME: &'static str = "fixture.demo";
    type Content = ();
    type Role = SoleRole;
    type Question = NoQuestions;
}

fn remint(capture: Identity<identity::CapturedDeclaration>) -> Account<Demo> {
    Account::over(capture)
}

fn main() {
    let _road = remint;
}

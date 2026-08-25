//! A plan reads kind-specific content from its account and accepts no sibling content argument.
//!
//! Restoring the old four-argument constructor makes this fixture compile and permits a plan identity derived from one account to carry another content value.

use macroonz_compiler::{Account, Context, Kind, NoQuestions, Plan, PlanDecisions, SoleRole};

struct Demo;

impl Kind for Demo {
    const NAME: &'static str = "fixture.demo";
    type Content = ();
    type Role = SoleRole;
    type Question = NoQuestions;
}

fn reattach(
    account: Account<Demo>,
    context: Context,
    decisions: PlanDecisions<SoleRole>,
) -> Plan<Demo> {
    Plan::planned(account, context, (), decisions)
}

fn main() {
    let _road = reattach;
}

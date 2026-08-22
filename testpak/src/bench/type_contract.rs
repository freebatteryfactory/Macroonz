//! Declarative conversions into the stamped benchmark-table refusal family.

use super::types::{
    BenchAttachmentRefusal, BenchBindingRefusal, BenchRowRefusal, BenchStampRefusal,
    BenchTableRefusal, DeclaredBudgetsRefusal, InputSizeAxisRefusal, WorkFormulaRefusal,
};
use crate::descriptor::{NameRefusal, TrialTableRefusal};

impl From<NameRefusal> for BenchStampRefusal {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<InputSizeAxisRefusal> for BenchStampRefusal {
    fn from(refusal: InputSizeAxisRefusal) -> Self {
        Self::InputSizeAxis(refusal)
    }
}

impl From<DeclaredBudgetsRefusal> for BenchStampRefusal {
    fn from(refusal: DeclaredBudgetsRefusal) -> Self {
        Self::Budgets(refusal)
    }
}

impl From<WorkFormulaRefusal> for BenchStampRefusal {
    fn from(refusal: WorkFormulaRefusal) -> Self {
        Self::WorkFormula(refusal)
    }
}

impl From<BenchRowRefusal> for BenchStampRefusal {
    fn from(refusal: BenchRowRefusal) -> Self {
        Self::Row(refusal)
    }
}

impl From<BenchAttachmentRefusal> for BenchStampRefusal {
    fn from(refusal: BenchAttachmentRefusal) -> Self {
        Self::Attachment(refusal)
    }
}

impl From<BenchBindingRefusal> for BenchStampRefusal {
    fn from(refusal: BenchBindingRefusal) -> Self {
        Self::Binding(refusal)
    }
}

impl From<TrialTableRefusal> for BenchStampRefusal {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Preflight(refusal)
    }
}

impl From<BenchTableRefusal> for BenchStampRefusal {
    fn from(refusal: BenchTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

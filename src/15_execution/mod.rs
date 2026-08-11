//! Band 15 — execution: the operator register, Execution Form, lowering and
//! agreement, the recursion witness, the effect batch, and kernel contracts.

pub mod types;

pub use types::{
    AlgebraicLaw, AlgebraicLawLimit, CommandKind, CommandOrdinal, EffectBatch,
    EffectBatchComposition, EffectBatchCompositionIssue, EffectCommand, EffectfulRecursionLane,
    ExecutionForm, ExecutionFormConstruction, ExecutionFormConstructionIssue,
    ExecutionFormFamilyId, ExecutionFormVersion, ForbiddenIdentitySource, GroupFenceDefect,
    INDEPENDENCE_MAY_NOT_SHARE, INDEPENDENCE_MAY_SHARE, INTERLEAVED_CLOSURE_TOTALS,
    KernelBindingPolicy, KernelBindingPolicyConstruction, KernelBindingPosture,
    KernelFallbackPolicy, KernelInterfaceContract, KernelInterfaceContractConstruction,
    KernelInterfaceContractConstructionIssue, KernelInterfaceContractRef,
    KernelQualificationEvidence, KernelRealizationId, KernelRequirement, KernelRequirementSet,
    KernelSemanticContract, KernelSemanticContractConstruction,
    KernelSemanticContractConstructionIssue, KernelSemanticContractRef, KernelSubstitutionScope,
    Measure, OPERATOR_REGISTER, OperatorDeclaration, RecursionWitness, RequiredContractKind,
    SemanticKernelFamilyId, SemanticKernelVersion, WORK_DIMENSIONS,
};

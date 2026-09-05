//! The mutation-discovery and benchmark fields the harness publishes to producers.

macro_rules! generated_support_field_banks {
    ($callback:ident) => {
        $callback! {
            mutation_discovery {
                "identity" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
                "owner_claim" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ZeroOrOne,
                "original_operation" => $crate::descriptor::FieldShape::Bytes => $crate::descriptor::FieldCardinality::ExactlyOne,
                "candidate_alternatives" => $crate::descriptor::FieldShape::MutationAlternative => $crate::descriptor::FieldCardinality::OneOrMore,
                "activation_site" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
            }
            bench {
                "workload_identity" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
                "input_size_axis" => $crate::descriptor::FieldShape::Count => $crate::descriptor::FieldCardinality::ZeroOrMore,
                "correctness_preflight" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
                "planted_worse_falsifier" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
                "declared_budgets" => $crate::descriptor::FieldShape::Count => $crate::descriptor::FieldCardinality::ZeroOrMore,
                "contention_posture" => $crate::descriptor::FieldShape::ClosedChoice(&["no-declared-contention"]) => $crate::descriptor::FieldCardinality::ExactlyOne,
                "work_formula" => $crate::descriptor::FieldShape::Bytes => $crate::descriptor::FieldCardinality::ZeroOrOne,
                "complexity_claim" => $crate::descriptor::FieldShape::NamespacedName => $crate::descriptor::FieldCardinality::ExactlyOne,
            }
        }
    };
}

pub(crate) use generated_support_field_banks;

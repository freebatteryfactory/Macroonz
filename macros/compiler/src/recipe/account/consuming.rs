//! Admission and readback of explicit consuming-projection bindings.

use super::{ConsumingMethod, ConsumingParameter, ConsumingParts, ConsumingProjection};
use crate::recipe::names::identifier_key;
use crate::recipe::types::{RecipeError, RecipeIssue};
use crate::recipe::{RecipeRelation, RecipeRelationRow, RecipeTransitionEffect, RecipeVocabulary};
use crate::token::{GeneratedToken, GeneratedTree, SpanHandle};

impl ConsumingParameter {
    pub(in crate::recipe) fn captured(
        name: String,
        token: GeneratedToken,
        kind: GeneratedTree,
        at: SpanHandle,
    ) -> Self {
        Self {
            name,
            token,
            kind,
            at,
        }
    }

    /// The caller's value binding.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The exact identifier naming this binding.
    #[must_use]
    pub const fn name_token(&self) -> &GeneratedToken {
        &self.token
    }

    /// The exact caller-authored Rust type.
    #[must_use]
    pub const fn kind(&self) -> &GeneratedTree {
        &self.kind
    }

    pub(in crate::recipe) const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl ConsumingMethod {
    pub(in crate::recipe) fn captured(
        event: String,
        name: String,
        token: GeneratedToken,
        payload: Option<ConsumingParameter>,
        at: SpanHandle,
    ) -> Self {
        Self {
            event,
            name,
            token,
            payload,
            at,
        }
    }

    /// The transition event this method binds.
    #[must_use]
    pub fn event(&self) -> &str {
        &self.event
    }

    /// The public method spelling selected by the caller.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The exact identifier naming the generated method.
    #[must_use]
    pub const fn name_token(&self) -> &GeneratedToken {
        &self.token
    }

    /// The optional caller-owned method payload.
    #[must_use]
    pub const fn payload(&self) -> Option<&ConsumingParameter> {
        self.payload.as_ref()
    }
}

impl ConsumingProjection {
    pub(in crate::recipe) fn against(
        &self,
        vocabulary: &RecipeVocabulary,
        relation: &RecipeRelation,
    ) -> Result<(), RecipeError> {
        if vocabulary.name() != relation.left_vocabulary() {
            return Err(binding(
                self.wrapper(),
                "the transition source vocabulary as its typestate subject",
                self.at(),
            ));
        }
        for reserved in ["Stage", "RecipeStage", "PhantomData", "__MacroonzPhase"] {
            if identifier_key(self.wrapper()) == reserved {
                return Err(binding(
                    self.wrapper(),
                    "a wrapper name distinct from structural typestate items",
                    self.at(),
                ));
            }
        }
        if vocabulary
            .members()
            .members()
            .any(|member| identifier_key(member.spelling()) == identifier_key(self.wrapper()))
        {
            return Err(binding(
                self.wrapper(),
                "a wrapper name distinct from every phase marker",
                self.at(),
            ));
        }
        for method in self.methods() {
            if !relation.rows().any(|row| row.right() == method.event()) {
                return Err(binding(
                    method.event(),
                    "an event used by a transition row",
                    method.at,
                ));
            }
            if ["restore", "resource", "into_resource"].contains(&identifier_key(method.name())) {
                return Err(binding(
                    method.name(),
                    "a method name distinct from wrapper admission and readback",
                    method.at,
                ));
            }
        }
        for row in relation.rows() {
            let Some(method) = self.methods().find(|method| method.event() == row.right()) else {
                return Err(binding(
                    row.right(),
                    "one declared consuming method",
                    self.at(),
                ));
            };
            self.effect_binding(row, method)?;
            let conflicting_events = self
                .methods()
                .filter(|other| identifier_key(other.name()) == identifier_key(method.name()))
                .map(ConsumingMethod::event)
                .collect::<Vec<_>>();
            if relation.rows().any(|other| {
                other.left() == row.left()
                    && other.right() != row.right()
                    && conflicting_events.contains(&other.right())
            }) {
                return Err(binding(
                    method.name(),
                    "distinct outgoing method names for one source phase",
                    method.at,
                ));
            }
        }
        Ok(())
    }

    fn effect_binding(
        &self,
        row: &RecipeRelationRow,
        method: &ConsumingMethod,
    ) -> Result<(), RecipeError> {
        let Some((_, _, RecipeTransitionEffect::ExactRust { target_binding, .. })) =
            row.payload().transition_parts()
        else {
            return Ok(());
        };
        let (GeneratedToken::Word(name) | GeneratedToken::RawIdentifier(name)) = target_binding
        else {
            return Ok(());
        };
        if [self.resource(), self.runtime()]
            .into_iter()
            .chain(method.payload())
            .any(|parameter| identifier_key(parameter.name()) == name)
        {
            return Err(binding(
                name,
                "a target binding distinct from runtime, resource and payload",
                row.effect_binding_at().unwrap_or(self.at()),
            ));
        }
        Ok(())
    }

    pub(in crate::recipe) fn informed(parts: ConsumingParts) -> Result<Self, RecipeError> {
        for parameter in [&parts.resource, &parts.runtime]
            .into_iter()
            .chain(parts.methods.iter().filter_map(ConsumingMethod::payload))
        {
            if identifier_key(parameter.name()).starts_with("__macroonz_") {
                return Err(binding(
                    parameter.name(),
                    "a caller binding outside the reserved __macroonz_ prefix",
                    parameter.at(),
                ));
            }
        }
        if parts.parameters.len() != parts.arguments.len() {
            return Err(binding(
                &parts.wrapper,
                "one type argument per declared generic parameter",
                parts.at,
            ));
        }
        if identifier_key(parts.resource.name()) == identifier_key(parts.runtime.name()) {
            return Err(binding(
                parts.runtime.name(),
                "distinct runtime and resource names",
                parts.runtime.at,
            ));
        }
        for (position, method) in parts.methods.iter().enumerate() {
            if parts
                .methods
                .iter()
                .take(position)
                .any(|earlier| earlier.event == method.event)
            {
                return Err(binding(
                    &method.event,
                    "one method declaration per event",
                    method.at,
                ));
            }
            if let Some(payload) = method.payload()
                && [parts.runtime.name(), parts.resource.name()]
                    .iter()
                    .any(|name| identifier_key(name) == identifier_key(payload.name()))
            {
                return Err(binding(
                    payload.name(),
                    "a payload name distinct from runtime and resource",
                    payload.at,
                ));
            }
        }
        Ok(Self {
            wrapper: parts.wrapper,
            wrapper_token: parts.wrapper_token,
            parameters: parts.parameters,
            arguments: parts.arguments,
            predicates: parts.predicates,
            resource: parts.resource,
            runtime: parts.runtime,
            refusal: parts.refusal,
            validator: parts.validator,
            methods: parts.methods,
            at: parts.at,
        })
    }

    /// The caller-selected opaque wrapper name.
    #[must_use]
    pub fn wrapper(&self) -> &str {
        &self.wrapper
    }

    /// The exact identifier naming the wrapper.
    #[must_use]
    pub const fn wrapper_token(&self) -> &GeneratedToken {
        &self.wrapper_token
    }

    /// The exact declared generic parameters in caller order.
    pub fn parameters(&self) -> impl Iterator<Item = &GeneratedTree> {
        self.parameters.iter()
    }

    /// The exact generic arguments in caller order.
    pub fn arguments(&self) -> impl Iterator<Item = &GeneratedTree> {
        self.arguments.iter()
    }

    /// The exact caller-owned where predicates.
    pub fn predicates(&self) -> impl Iterator<Item = &GeneratedTree> {
        self.predicates.iter()
    }

    /// The held resource and the binding visible to row effects.
    #[must_use]
    pub const fn resource(&self) -> &ConsumingParameter {
        &self.resource
    }

    /// The runtime parameter visible to validation and row effects.
    #[must_use]
    pub const fn runtime(&self) -> &ConsumingParameter {
        &self.runtime
    }

    /// The caller-owned refusal type.
    #[must_use]
    pub const fn refusal(&self) -> &GeneratedTree {
        &self.refusal
    }

    /// The caller-owned runtime validation expression.
    #[must_use]
    pub const fn validator(&self) -> &GeneratedTree {
        &self.validator
    }

    /// The event-to-method bindings in caller order.
    pub fn methods(&self) -> impl Iterator<Item = &ConsumingMethod> {
        self.methods.iter()
    }

    pub(in crate::recipe) const fn at(&self) -> SpanHandle {
        self.at
    }
}

pub(in crate::recipe) fn binding(
    name: &str,
    expected: &'static str,
    at: SpanHandle,
) -> RecipeError {
    RecipeError::at(
        RecipeIssue::ConsumingBinding {
            name: name.to_owned(),
            expected,
        },
        Some(at),
    )
}

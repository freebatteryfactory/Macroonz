//! Explicit consuming bindings, refusal precedence and custom projection readback.

use super::support::{bake, bake_with, refusal_summary};
use macroonz_compiler::recipe::{
    ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink, ProjectorReplacement,
    RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{CanonicalContent, GeneratedToken, GeneratedTree, unit_struct};

const SOURCE: &str = r"
pub mod parcel {
    pub enum Phase { Ready, Sent }
    pub enum Event { Send }
    bake! {
        vocabularies { Phase; Event; };
        transitions(Phase, Event) {
            (Ready, Send) => Sent with(destination) { crate::send(engine, value, destination, item) };
        };
        absence(refused);
        projections {
            typestate(Phase) {
                wrapper(Package);
                resource(value: crate::Value);
                runtime(engine: &mut crate::Engine);
                refusal(crate::Error);
                validate(crate::inspect);
                methods { Send => send(item: crate::Payload); };
            };
        };
    }
}
";

struct Readback;

#[test]
fn outgoing_method_collisions_are_scoped_to_one_source_phase() -> Result<(), String> {
    let duplicate = SOURCE
        .replace("Event { Send }", "Event { Send, Mark }")
        .replace(
            "(Ready, Send) =>",
            "(Ready, Mark) => Sent with(crate::mark); (Ready, Send) =>",
        )
        .replace(
            "methods { Send => send(item: crate::Payload); }",
            "methods { Send => send(item: crate::Payload); Mark => send(); }",
        );
    let refused = refusal_summary(&duplicate)
        .map_err(|()| "colliding outgoing methods were accepted".to_owned())?;
    assert!(
        refused.contains("distinct outgoing method names"),
        "{refused}"
    );
    let distinct = bake(&duplicate.replace("Mark => send()", "Mark => mark()"))
        .map_err(|()| "distinct outgoing method twin refused".to_owned())?;
    assert!(distinct.emit().tokens().is_some());
    let disjoint = bake(&duplicate.replace("(Ready, Mark)", "(Sent, Mark)"))
        .map_err(|()| "disjoint source method twin refused".to_owned())?;
    assert!(disjoint.emit().tokens().is_some());
    Ok(())
}

impl RecipeProjector for Readback {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(view.recipe().module_name(), "parcel");
        assert_eq!(request.role(), RecipeRole::Typestate);
        let config = request
            .effective()
            .consuming()
            .ok_or(ProjectionError::Render(
                macroonz_compiler::RenderError::NothingRendered,
            ))?;
        assert_eq!(config.wrapper(), "Package");
        assert_eq!(config.resource().name(), "value");
        assert_eq!(config.runtime().name(), "engine");
        assert_eq!(config.parameters().count(), 0);
        assert_eq!(config.arguments().count(), 0);
        assert_eq!(config.predicates().count(), 0);
        assert_eq!(config.refusal().inspected().trim(), "crate :: Error");
        assert_eq!(config.validator().inspected().trim(), "crate :: inspect");
        let method = config.methods().next().ok_or(ProjectionError::Render(
            macroonz_compiler::RenderError::NothingRendered,
        ))?;
        assert_eq!(method.event(), "Send");
        assert_eq!(method.name(), "send");
        assert_eq!(
            method
                .payload()
                .map(macroonz_compiler::recipe::ConsumingParameter::name),
            Some("item")
        );
        sink.offer(GeneratedTree::assembled(unit_struct(
            GeneratedToken::word("Readback"),
            Vec::new(),
            Vec::new(),
        ))?)
    }
}

#[test]
fn consuming_configuration_is_available_through_the_same_custom_projector_account() -> Result<(), ()>
{
    let standard = bake(SOURCE)?;
    let custom = bake_with(
        SOURCE,
        &[ProjectorReplacement::for_role(
            RecipeRole::Typestate,
            &Readback,
        )],
    )?;
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        custom
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    assert_ne!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    Ok(())
}

#[test]
fn every_consuming_binding_is_identity_material_and_spacing_is_not() -> Result<(), ()> {
    let baseline = bake(SOURCE)?;
    for (original, replacement) in [
        ("wrapper(Package)", "wrapper(Parcel)"),
        ("value: crate::Value", "resource: crate::Value"),
        ("crate::Value", "crate::OtherValue"),
        ("engine: &mut crate::Engine", "runtime: &mut crate::Engine"),
        ("&mut crate::Engine", "&crate::Engine"),
        ("refusal(crate::Error)", "refusal(crate::OtherError)"),
        ("validate(crate::inspect)", "validate(crate::check)"),
        ("Send => send", "Send => submit"),
        ("item: crate::Payload", "item: crate::OtherPayload"),
        ("item: crate::Payload", "payload: crate::Payload"),
        ("send(item: crate::Payload)", "send()"),
        (
            "wrapper(Package);",
            "wrapper(Package); generics { parameters { (T); }; arguments { (T); }; predicates { (T: Clone); }; };",
        ),
    ] {
        let moved = bake(&SOURCE.replace(original, replacement))?;
        assert_ne!(
            baseline
                .projection()
                .plan()
                .content()
                .canonical_content_bytes(),
            moved
                .projection()
                .plan()
                .content()
                .canonical_content_bytes(),
            "{replacement}"
        );
        assert_ne!(
            baseline.projection().identity(),
            moved.projection().identity(),
            "{replacement}"
        );
    }
    let respaced = bake(&SOURCE.replace(' ', "  "))?;
    assert_eq!(
        baseline.projection().identity(),
        respaced.projection().identity()
    );
    assert_eq!(
        baseline.emit().tokens().map(GeneratedTree::canonical_bytes),
        respaced.emit().tokens().map(GeneratedTree::canonical_bytes)
    );
    Ok(())
}

#[test]
fn missing_foreign_repeated_and_colliding_consuming_bindings_refuse() -> Result<(), String> {
    let lawful = bake(SOURCE).map_err(|()| "lawful consuming twin refused".to_owned())?;
    assert!(
        lawful
            .projection()
            .plan()
            .content()
            .effective(RecipeRole::Typestate)
            .is_some()
    );
    for (original, replacement, expected) in [
        (
            "Send => send(item: crate::Payload);",
            "",
            "one declared consuming method",
        ),
        (
            "Send => send",
            "Missing => send",
            "an event used by a transition row",
        ),
        (
            "Send => send(item: crate::Payload);",
            "Send => send(); Send => again();",
            "one method declaration per event",
        ),
        (
            "Send => send",
            "Send => restore",
            "distinct from wrapper admission",
        ),
        (
            "wrapper(Package)",
            "wrapper(Ready)",
            "distinct from every phase marker",
        ),
        (
            "wrapper(Package)",
            "wrapper(Stage)",
            "distinct from structural typestate",
        ),
        (
            "runtime(engine:",
            "runtime(value:",
            "distinct runtime and resource names",
        ),
        (
            "item: crate::Payload",
            "engine: crate::Payload",
            "payload name distinct",
        ),
        (
            "with(destination)",
            "with(value)",
            "target binding distinct",
        ),
        (
            "resource(value:",
            "resource(__macroonz_resource:",
            "reserved __macroonz_ prefix",
        ),
        (
            "wrapper(Package);",
            "wrapper(Package); generics { parameters { (T); }; arguments { }; predicates { }; };",
            "one type argument",
        ),
        ("refusal(crate::Error);", "", "refusal"),
        ("validate(crate::inspect);", "", "validate"),
        (
            "validate(crate::inspect)",
            "validate()",
            "nonempty caller Rust material",
        ),
    ] {
        let source = SOURCE.replace(original, replacement);
        let refusal =
            refusal_summary(&source).map_err(|()| format!("{replacement} did not refuse"))?;
        assert!(refusal.contains(expected), "{replacement}: {refusal}");
    }
    Ok(())
}

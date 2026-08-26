//! A codec shape's private seats cannot bypass the one declaration road that establishes a non-empty, collision-free member roster.

use macroonz_compiler::NonEmpty;
use macroonz_compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecMember, CodecMemberShape, CodecShape,
    CodecTypePath, PathRooting,
};

fn main() {
    let Ok(owner) = CodecTypePath::spelled(PathRooting::InScope, vec!["Demo".to_owned()]) else {
        return;
    };
    let Ok(held_as) = CodecTypePath::spelled(PathRooting::InScope, vec!["u16".to_owned()]) else {
        return;
    };
    let Ok(member) = CodecMember::declared(
        "count",
        held_as,
        CodecMemberShape::Count,
        Cardinality::Required,
    ) else {
        return;
    };
    let Ok(assembly) = CodecAssembly::stated("assembled", AssemblyPosture::Total) else {
        return;
    };
    let shape = CodecShape {
        owner,
        refusal: "DemoRefusal".to_owned(),
        assembly,
        members: NonEmpty::one(member),
    };
    drop(shape);
}

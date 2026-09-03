//! The schema roads: the roster law every member is parsed under, the root declaration, and the one identity this home derives.

use crate::descriptor::encode::encode_generated_support_schema;
use crate::descriptor::types::{
    BENCH_FIELDS, BenchSchema, DESCRIPTOR_FIELDS, DescriptorSchema, EncodeRefusal,
    FieldCardinality, FieldShape, GeneratedSupportSchema, GeneratedSupportSchemaId,
    MUTATION_DISCOVERY_FIELDS, MutationDiscoverySchema, SchemaField, SchemaRefusal,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use std::collections::BTreeSet;

/// The domain the generated-support schema identity is derived under.
///
/// Two kinds derived over identical preimages under different tags are unrelated values.
const GENERATED_SUPPORT_SCHEMA_DOMAIN: DomainTag = DomainTag::declared(
    "generated-support-schema",
    IdentityProfileVersion::declared(1),
);

impl GeneratedSupportSchemaId {
    /// Reify a content address whose generated-support-schema derivation the caller already established.
    ///
    /// It preserves an address; it does not prove the address came from the current declaration.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }
}

crate::identity::content_address_reference! {
    /// The content address this identity carries.
    value GeneratedSupportSchemaId;
}

impl SchemaField {
    /// One field of one producer-facing vocabulary, as the schema declares it.
    #[must_use]
    pub const fn declared(
        name: &'static str,
        shape: FieldShape,
        cardinality: FieldCardinality,
    ) -> Self {
        Self {
            name,
            shape,
            cardinality,
        }
    }

    /// The field's name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The shape its values take.
    #[must_use]
    pub const fn shape(self) -> FieldShape {
        self.shape
    }

    /// How many values it carries.
    #[must_use]
    pub const fn cardinality(self) -> FieldCardinality {
        self.cardinality
    }
}

/// The one roster law all three schema members are parsed under: a member declares at least one field, every field is named, and no name is stated twice.
fn parse_roster(fields: &'static [SchemaField]) -> Result<(), SchemaRefusal> {
    if fields.is_empty() {
        return Err(SchemaRefusal::EmptyRoster);
    }
    let mut names = BTreeSet::new();
    for field in fields {
        if field.name().is_empty() {
            return Err(SchemaRefusal::EmptyFieldName);
        }
        if !names.insert(field.name()) {
            return Err(SchemaRefusal::DuplicateFieldName(field.name()));
        }
    }
    Ok(())
}

/// The one constructor and the one reader every schema member carries, written once and stamped over the roster.
macro_rules! schema_member {
    ($($member:ident),+ $(,)?) => {
        $(
            impl $member {
                /// This member's roster, parsed under the roster law.
                ///
                /// # Errors
                ///
                /// Refuses an empty roster, then an unnamed field, then a repeated name.
                pub fn declared(fields: &'static [SchemaField]) -> Result<Self, SchemaRefusal> {
                    parse_roster(fields)?;
                    Ok(Self { fields })
                }

                /// The roster this member declares, in declared order.
                #[must_use]
                pub const fn fields(self) -> &'static [SchemaField] {
                    self.fields
                }
            }
        )+
    };
}

schema_member!(DescriptorSchema, MutationDiscoverySchema, BenchSchema);

macro_rules! implement_generated_support_members {
    ([]; $( $member:ident: $member_type:ty => $fields:ident => $tag:literal, )+) => {
        impl GeneratedSupportSchema {
            /// The root declaration over the members already parsed under their roster law.
            #[must_use]
            pub const fn declared($( $member: $member_type, )+) -> Self {
                Self {
                    $(
                        $member,
                    )+
                }
            }

            $(
                #[doc = concat!("The root's `", stringify!($member), "` member.")]
                #[must_use]
                pub const fn $member(self) -> $member_type {
                    self.$member
                }
            )+

            /// The root declaration this crate publishes, with every member parsed in declared order.
            ///
            /// The member roster projects this assembly and the canonical traversal, so neither can omit a member the root accepts.
            ///
            /// # Errors
            ///
            /// Refuses when any member's roster refuses an empty roster, an unnamed field, or a repeated field name, in root-member order.
            pub fn published() -> Result<Self, SchemaRefusal> {
                Ok(Self::declared(
                    $(
                        <$member_type>::declared($fields)?,
                    )+
                ))
            }
        }
    };
}

generated_support_members!(implement_generated_support_members);

impl GeneratedSupportSchema {
    /// The identity derived from this declaration's canonical bytes.
    ///
    /// The one derivation this home performs.
    /// A change to any member moves it, which is how one pin governs all three crossings.
    ///
    /// # Errors
    ///
    /// Refuses when the encoding refuses — a length past the width the encoding declares, which is unreachable on every target this crate is built for.
    pub fn identity(&self) -> Result<GeneratedSupportSchemaId, EncodeRefusal> {
        let preimage = encode_generated_support_schema(self)?;
        Ok(GeneratedSupportSchemaId(ContentAddress::derived(
            GENERATED_SUPPORT_SCHEMA_DOMAIN,
            &preimage,
        )))
    }
}

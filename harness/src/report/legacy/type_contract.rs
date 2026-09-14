//! Private deserialization contracts with no public evidence deserializer.

use super::read::read_member;
use super::{
    DigestSeed, KeySeed, LegacyField, LegacyInputProfile, LegacyPresence, LegacyRecord,
    LegacyRefusal, Nullable, NumberSeed, Object, ProfileSeed, RecordSeed, TextSeed, WitnessSeed,
};
use crate::report::archive::AddressClaim;
use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;

impl<'de> DeserializeSeed<'de> for RecordSeed<'_> {
    type Value = LegacyRecord;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> DeserializeSeed<'de> for ProfileSeed<'_> {
    type Value = LegacyInputProfile;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> DeserializeSeed<'de> for TextSeed<'_> {
    type Value = String;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_str(self)
    }
}

impl<'de> DeserializeSeed<'de> for NumberSeed<'_> {
    type Value = u64;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_u64(self)
    }
}

impl<'de> DeserializeSeed<'de> for DigestSeed<'_> {
    type Value = AddressClaim;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_str(self)
    }
}

impl<'de> DeserializeSeed<'de> for WitnessSeed<'_> {
    type Value = Vec<u8>;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_seq(self)
    }
}

impl<'de> DeserializeSeed<'de> for KeySeed<'_> {
    type Value = LegacyField;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_identifier(self)
    }
}

impl<'de, Seed: DeserializeSeed<'de>> DeserializeSeed<'de> for Nullable<Seed> {
    type Value = LegacyPresence<Seed::Value>;
    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_option(self)
    }
}

impl<'de, Seed: DeserializeSeed<'de>> Visitor<'de> for Nullable<Seed> {
    type Value = LegacyPresence<Seed::Value>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("null or a supported historical value")
    }
    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(LegacyPresence::Null)
    }
    fn visit_some<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        self.0.deserialize(decoder).map(LegacyPresence::Present)
    }
}

impl Visitor<'_> for TextSeed<'_> {
    type Value = String;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded UTF-8 text")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<String, E> {
        self.0.text(value)
    }
}

impl Visitor<'_> for NumberSeed<'_> {
    type Value = u64;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an unsigned 64-bit integer")
    }
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<u64, E> {
        self.0.retain(size_of::<u64>())?;
        Ok(value)
    }
}

impl Visitor<'_> for DigestSeed<'_> {
    type Value = AddressClaim;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sixty-four lowercase hexadecimal digits")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<AddressClaim, E> {
        if value.len() > self.0.limits.text {
            return Err(self.0.refuse(LegacyRefusal::TextTooLarge));
        }
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Err(self.0.refuse(LegacyRefusal::InvalidDigest));
        }
        let mut bytes = [0u8; 32];
        for (destination, pair) in bytes.iter_mut().zip(value.as_bytes().as_chunks::<2>().0) {
            let text = std::str::from_utf8(pair)
                .map_err(|_invalid| self.0.refuse::<E>(LegacyRefusal::InvalidDigest))?;
            *destination = u8::from_str_radix(text, 16)
                .map_err(|_invalid| self.0.refuse::<E>(LegacyRefusal::InvalidDigest))?;
        }
        self.0.retain(bytes.len())?;
        Ok(AddressClaim::stated(bytes))
    }
}

impl<'de> Visitor<'de> for WitnessSeed<'_> {
    type Value = Vec<u8>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded array of integer octets")
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Vec<u8>, A::Error> {
        self.0.depth(2)?;
        let mut bytes = Vec::new();
        while let Some(byte) = sequence.next_element::<u8>()? {
            if bytes.len() >= self.0.limits.witness {
                return Err(self.0.refuse(LegacyRefusal::WitnessTooLarge));
            }
            self.0.retain(1)?;
            bytes.push(byte);
        }
        Ok(bytes)
    }
}

impl Visitor<'_> for KeySeed<'_> {
    type Value = LegacyField;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a declared historical field")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<LegacyField, E> {
        let field = match (self.1, value) {
            (Object::Record, "kind") => LegacyField::Kind,
            (Object::Record, "schema") => LegacyField::Schema,
            (Object::Record, "witness") => LegacyField::Witness,
            (Object::Record, "input_profile") => LegacyField::InputProfile,
            (Object::Record, "trial_name") => LegacyField::TrialName,
            (Object::Record, "subject_name") => LegacyField::SubjectName,
            (Object::Record, "check_name") => LegacyField::CheckName,
            (Object::Record, "subject_revision") => LegacyField::SubjectRevision,
            (Object::Record, "check_revision") => LegacyField::CheckRevision,
            (Object::Record, "target") => LegacyField::Target,
            (Object::Record, "toolchain") => LegacyField::Toolchain,
            (Object::Record, "execution_digest") => LegacyField::ExecutionDigest,
            (Object::Record, "fingerprint_digest") => LegacyField::FingerprintDigest,
            (Object::Record, "reported_outcome") => LegacyField::ReportedOutcome,
            (Object::Profile, "name") => LegacyField::ProfileName,
            (Object::Profile, "revision") => LegacyField::ProfileRevision,
            (Object::Record | Object::Profile, _) => {
                return Err(self.0.refuse(LegacyRefusal::UnknownField));
            }
        };
        self.0.member(field)?;
        Ok(field)
    }
}

impl<'de> Visitor<'de> for RecordSeed<'_> {
    type Value = LegacyRecord;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one historical record object")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<LegacyRecord, A::Error> {
        self.0.depth(1)?;
        let mut record = LegacyRecord {
            source: Vec::new(),
            kind: LegacyPresence::Missing,
            schema: LegacyPresence::Missing,
            witness: LegacyPresence::Missing,
            input_profile: LegacyPresence::Missing,
            trial_name: LegacyPresence::Missing,
            subject_name: LegacyPresence::Missing,
            check_name: LegacyPresence::Missing,
            subject_revision: LegacyPresence::Missing,
            check_revision: LegacyPresence::Missing,
            target: LegacyPresence::Missing,
            toolchain: LegacyPresence::Missing,
            execution_digest: LegacyPresence::Missing,
            fingerprint_digest: LegacyPresence::Missing,
            reported_outcome: LegacyPresence::Missing,
        };
        while let Some(field) = object.next_key_seed(KeySeed(self.0, Object::Record))? {
            read_member(&mut record, field, &mut object, self.0)?;
        }
        Ok(record)
    }
}

impl<'de> Visitor<'de> for ProfileSeed<'_> {
    type Value = LegacyInputProfile;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a partial historical input profile")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<LegacyInputProfile, A::Error> {
        self.0.depth(2)?;
        let mut profile = LegacyInputProfile {
            name: LegacyPresence::Missing,
            revision: LegacyPresence::Missing,
        };
        while let Some(field) = object.next_key_seed(KeySeed(self.0, Object::Profile))? {
            match field {
                LegacyField::ProfileName => {
                    profile.name = object.next_value_seed(Nullable(TextSeed(self.0)))?;
                }
                LegacyField::ProfileRevision => {
                    profile.revision = object.next_value_seed(Nullable(NumberSeed(self.0)))?;
                }
                LegacyField::Kind
                | LegacyField::Schema
                | LegacyField::Witness
                | LegacyField::InputProfile
                | LegacyField::TrialName
                | LegacyField::SubjectName
                | LegacyField::CheckName
                | LegacyField::SubjectRevision
                | LegacyField::CheckRevision
                | LegacyField::Target
                | LegacyField::Toolchain
                | LegacyField::ExecutionDigest
                | LegacyField::FingerprintDigest
                | LegacyField::ReportedOutcome => {
                    return Err(self.0.refuse(LegacyRefusal::UnknownField));
                }
            }
        }
        Ok(profile)
    }
}

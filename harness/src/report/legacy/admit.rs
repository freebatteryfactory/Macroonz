//! Resource admission before retained data grows.

use super::{Admission, LegacyField, LegacyRefusal};
use serde::de;

impl Admission {
    pub(super) fn refuse<E: de::Error>(&mut self, refusal: LegacyRefusal) -> E {
        self.refusal.get_or_insert(refusal);
        E::custom("historical field admission refused")
    }

    pub(super) fn retain<E: de::Error>(&mut self, bytes: usize) -> Result<(), E> {
        let Some(total) = self.retained.checked_add(bytes) else {
            return Err(self.refuse(LegacyRefusal::SizeOutsidePlatform));
        };
        if total > self.limits.retained {
            return Err(self.refuse(LegacyRefusal::RetainedTooLarge));
        }
        self.retained = total;
        Ok(())
    }

    pub(super) fn text<E: de::Error>(&mut self, value: &str) -> Result<String, E> {
        if value.len() > self.limits.text {
            return Err(self.refuse(LegacyRefusal::TextTooLarge));
        }
        self.retain(value.len())?;
        Ok(value.to_owned())
    }

    pub(super) fn member<E: de::Error>(&mut self, field: LegacyField) -> Result<(), E> {
        if self.seen.contains(&field) {
            return Err(self.refuse(LegacyRefusal::DuplicateField(field)));
        }
        if self.members >= self.limits.members {
            return Err(self.refuse(LegacyRefusal::TooManyMembers));
        }
        self.members = self
            .members
            .checked_add(1)
            .ok_or_else(|| self.refuse::<E>(LegacyRefusal::SizeOutsidePlatform))?;
        self.seen.insert(field);
        Ok(())
    }

    pub(super) fn depth<E: de::Error>(&mut self, depth: usize) -> Result<(), E> {
        if depth > self.limits.depth {
            return Err(self.refuse(LegacyRefusal::TooDeep));
        }
        Ok(())
    }
}

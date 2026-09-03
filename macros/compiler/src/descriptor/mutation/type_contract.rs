//! The mutation home's stated tables: what the kind is, where its one unit lands, and how its grammar refuses.

use super::{Address, MutationCaptureError, MutationSurface, Policy, Site, Surface, SurfaceRole};
use crate::descriptor::Name;
use crate::diagnostic::SECOND_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Destination, Kind, NoQuestions, Role};
use crate::token::GeneratedToken;

impl CanonicalContent for Surface {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_address(self.address(), into);
        encode_policy(self.policy(), into);
        encode_site(self.site(), into);
    }
}

fn encode_address(address: &Address, into: &mut Vec<u8>) {
    encode_bytes(address.module.spelling().as_bytes(), into);
    match &address.support {
        None => into.push(0),
        Some(support) => {
            into.push(1);
            encode_bytes(support.spelling().as_bytes(), into);
        }
    }
    encode_bytes(address.refusal.spelling().as_bytes(), into);
}

fn encode_policy(policy: &Policy, into: &mut Vec<u8>) {
    encode_name(policy.family(), into);
    encode_length(policy.mappings().len(), into);
    for mapping in policy.mappings() {
        let mut encoded = Vec::new();
        encode_name(&mapping.fact, &mut encoded);
        encode_name(&mapping.claim, &mut encoded);
        encode_bytes(&encoded, into);
    }
    encode_length(policy.permissions().len(), into);
    for permission in policy.permissions() {
        let mut encoded = Vec::new();
        encode_name(permission.claim(), &mut encoded);
        encode_length(permission.families().count(), &mut encoded);
        for family in permission.families() {
            encode_bytes(family.slug().as_bytes(), &mut encoded);
        }
        encode_bytes(&encoded, into);
    }
}

fn encode_site(site: &Site, into: &mut Vec<u8>) {
    encode_name(site.point(), into);
    encode_name(site.fact(), into);
    encode_tokens(site.order(), into);
    encode_tokens(site.production(), into);
    encode_bytes(site.unchanged(), into);
    encode_length(site.alternatives().len(), into);
    for alternative in site.alternatives() {
        let mut encoded = Vec::new();
        encode_bytes(alternative.family().slug().as_bytes(), &mut encoded);
        encode_bytes(alternative.operation(), &mut encoded);
        encode_tokens(alternative.meaning(), &mut encoded);
        encode_bytes(&encoded, into);
    }
}

fn encode_tokens(tokens: &[GeneratedToken], into: &mut Vec<u8>) {
    let mut encoded = Vec::new();
    encode_length(tokens.len(), &mut encoded);
    for token in tokens {
        token.encode_into(&mut encoded);
    }
    encode_bytes(&encoded, into);
}

fn encode_name(name: &Name, into: &mut Vec<u8>) {
    encode_bytes(name.namespace().as_bytes(), into);
    encode_bytes(name.stem().as_bytes(), into);
}

impl Kind for MutationSurface {
    const NAME: &'static str = "mutation-surface";

    type Content = Surface;
    type Role = SurfaceRole;
    type Question = NoQuestions;
}

impl Role for SurfaceRole {
    const ALL: &'static [Self] = &[Self::Module];

    fn name(self) -> &'static str {
        match self {
            Self::Module => "module",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Module => Destination::TestCarrier,
        }
    }
}

crate::descriptor::impl_helper_capture_contract!(MutationCaptureError, SECOND_HELPER_FAMILY, none);

//! Carrier shell construction.
use super::super::render;
use super::{ShellError, ShellName, SupportShell};
use crate::identity::PlanId;
use crate::kind::Kind;
use crate::plan::Plan;
use crate::request::Door;
use crate::support::DeliveryForm;
use crate::support::assembly::SupportAssembly;
use crate::support::cargo::AxisCargo;
use crate::token::{GeneratedToken, GeneratedTree};
fn digit(nibble: u8) -> char {
    let normalized = nibble & 0x0f;
    let byte = if normalized < 10 {
        b'0'.wrapping_add(normalized)
    } else {
        b'a'.wrapping_add(normalized.wrapping_sub(10))
    };
    char::from(byte)
}
impl ShellName {
    /// The machinery-name prefix.
    pub const PREFIX: &'static str = "__macroonz_support_";
    /// The number of plan-identity bytes in the name.
    pub const KEY_BYTES: usize = 32;
    /// Mangles a complete plan identity.
    #[must_use]
    pub fn mangled(plan: PlanId) -> Self {
        let mut spelling = String::from(Self::PREFIX);
        for byte in plan.as_bytes().iter().take(Self::KEY_BYTES) {
            spelling.push(digit(byte.wrapping_shr(4)));
            spelling.push(digit(*byte));
        }
        Self { spelling }
    }
    /// Reads the exported spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}
impl SupportShell {
    /// Renders the inert shell from one plan and verified assembly.
    ///
    /// # Errors
    /// Returns a declaration mismatch or generated-tree overflow.
    pub fn assembled<C: Kind>(
        carrier: &Plan<C>,
        assembly: &SupportAssembly,
        door: &Door,
    ) -> Result<Self, ShellError> {
        let planned = carrier.account().commitment();
        let stated = assembly.root();
        if planned != stated {
            return Err(ShellError::NotOneDeclaration { stated, planned });
        }
        let name = ShellName::mangled(carrier.identity());
        let form = assembly.form();
        let pin = render::expectation_roster(assembly.expectation())?;
        let body = render::gate_invocation(form, pin, stamped(assembly), opaque(assembly, form))?;
        let matched = render::matcher(assembly.declared());
        let mut tokens =
            render::exported_shell(&name, &render::shell_sentence(door), matched, body)?;
        if let Some(address) = assembly.address() {
            tokens.extend(render::public_alias(
                &name,
                address,
                &render::alias_sentence(door),
            )?);
        }
        Ok(Self {
            name,
            tree: GeneratedTree::assembled(tokens)?,
        })
    }
    /// Reads the exported name.
    #[must_use]
    pub const fn name(&self) -> &ShellName {
        &self.name
    }
    /// Reads the rendered tree.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
    /// Takes the rendered tree.
    #[must_use]
    pub fn into_tree(self) -> GeneratedTree {
        self.tree
    }
}
fn stamped(assembly: &SupportAssembly) -> Vec<GeneratedToken> {
    match assembly.declared() {
        AxisCargo::Absent { .. } => Vec::new(),
        AxisCargo::Carried(cargo) => cargo.stamped().tokens().to_vec(),
    }
}
fn opaque(assembly: &SupportAssembly, form: DeliveryForm) -> Vec<GeneratedToken> {
    let axis = match form {
        DeliveryForm::Trials => assembly.deferred(),
        DeliveryForm::Benches => assembly.bench(),
    };
    match axis {
        AxisCargo::Absent { .. } => Vec::new(),
        AxisCargo::Carried(proved) => proved.cargo().tree().tokens().to_vec(),
    }
}

//! Canonical generated mutation material observed independently of token-composer wiring.

use super::{MUTATION_BODY, MUTATION_ITEM, captured, mutation, receipt, trees};
use macroonz_compiler::descriptor::Grammar;
use macroonz_compiler::{GeneratedTree, SpanHandle};

/// The complete rendered module retains its canonical token material, including refusal separators.
#[test]
fn generated_mutation_module_retains_exact_token_material() -> Result<(), ()> {
    let body = captured(MUTATION_BODY)?;
    let item = captured(MUTATION_ITEM)?;
    let grammar = Grammar {
        attribute: "mutations",
    };
    let declaration =
        mutation::captured(&trees(&body), SpanHandle::at(0), grammar).map_err(|_| ())?;
    let surface = mutation::completed(declaration, &trees(&item), grammar).map_err(|_| ())?;
    let tree = GeneratedTree::assembled(mutation::generated_module(&surface).map_err(|_| ())?)
        .map_err(|_| ())?;
    assert_eq!(
        receipt(&tree.canonical_bytes()),
        (
            14_796,
            "afe2584091d692000c4400d04e6bfca996eb121cf592d6b3fc4dd849e28c1806".to_owned()
        )
    );
    Ok(())
}

//! The frontage road, proven closed.
//!
//! `RefusalDerivationDraft` used to be the front door: it carried a `rendered()`
//! method, and a caller could take a rendering off it with no plan, no
//! identities, no origin graph, no trace, no explanation, and no closure. That
//! road was shorter than the receipt-rich one, which is another way of saying
//! every receipt on the receipt-rich road was optional.
//!
//! This is a different unwritable move than the one the closed-expansion fixture
//! proves. That road dies on a constructor a caller may not reach; this one dies
//! because the method a caller would reach for does not exist on the type at
//! all. The draft states what the shape fixed and answers nothing about bytes.
//!
//! No value is constructed below. The signature and the call alone are the
//! proof.

use threadpak_macroc::RefusalDerivationDraft;

fn main() {
    let take: fn(&RefusalDerivationDraft) = |draft| {
        let _ = draft.rendered();
    };
    let _ = take;
}

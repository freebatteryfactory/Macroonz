//! `threadpak-macros`: the Rust-facing expansion shell.
//!
//! The shell decides nothing. Every entry point here parses its declared
//! input and hands the work to `threadpak-macroc`, which owns the meaning.
//! The shell depends on the services and on nothing else — it reaches the
//! machine's vocabulary only through them.
//!
//! This file is a topology skeleton. Its one derive expands to nothing, but
//! its body calls the services, so the shell-to-services edge fails to
//! compile if it is ever cut.

use proc_macro::TokenStream;
use threadpak_macroc::{FrontendRole, describe_frontend_role};

/// A no-op derive that expands to nothing.
///
/// The expansion is empty by construction: the skeleton exists to price the
/// crate topology, not to author anything. What it does prove is that the
/// shell can call the services during expansion and can see the machine's
/// public types through them.
///
/// ```
/// use threadpak_macros::ThreadpakSkeleton;
///
/// #[derive(ThreadpakSkeleton)]
/// struct Local;
/// ```
#[proc_macro_derive(ThreadpakSkeleton)]
pub fn threadpak_skeleton(item: TokenStream) -> TokenStream {
    let described = describe_frontend_role(FrontendRole::RustDeclaration);
    if described.is_empty() {
        return item;
    }
    TokenStream::new()
}

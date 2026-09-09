//! Root execution, canonical retention and fresh witness execution through public owners.

mod fixture;
mod execute;
#[cfg(feature = "native-tooling")]
mod retention;
#[cfg(feature = "native-tooling")]
mod recovery;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod process;

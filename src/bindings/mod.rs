// The cxx bridge holds the canonical data structures, so it is always compiled
// (the `cxx` crate is pure Rust). The C++ *compilation* is still gated by `cpp`.
pub mod cpp;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "pyref")]
pub mod pyref;

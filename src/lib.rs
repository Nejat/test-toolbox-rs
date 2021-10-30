#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/test-toolbox/0.4.0")]

//! Utility library of helper macros for working with unit tests.
//!
//! ## Macros
//!
//! * `actual!` - declare actual variable with differing `debug` and `release` syntax
//! * `expect!` - declare expected variable with differing `debug` and `release` values
//! * `capture!` - captures `stdout` and `stderr` for testing output

#[cfg(feature = "actual")]
mod actual;
#[cfg(feature = "capture")]
mod capture;
#[cfg(feature = "expected")]
mod expect;

#[cfg(feature = "capture")]
#[doc(hidden)]
pub use gag;

#[cfg(test)]
mod tests;

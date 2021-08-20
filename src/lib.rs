#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/test-toolbox/0.1.0")]

//! Utility library of helper macros for working with unit tests.
//!
//! ## Macros
//!
//! * `actual!` - declare actual variable with differing `debug` and `release` syntax
//! * `expect!` - declare expected variable with differing `debug` and `release` values
//! * `capture!` - captures `stdout` and `stderr` for testing output

mod actual;
mod capture;
mod expect;

#[cfg(test)]
mod tests;

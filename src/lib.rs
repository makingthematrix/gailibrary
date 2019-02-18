#![allow(unknown_lints)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

pub mod engine;
pub mod fields;
pub mod ids;
pub mod langtonsant;
pub mod utils;

#[cfg(test)]
mod fields_tests;
#[cfg(test)]
mod ids_tests;

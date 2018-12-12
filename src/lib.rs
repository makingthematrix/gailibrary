#![allow(unknown_lints)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod engine;
pub mod fields;
pub mod ids;
pub mod utils;
pub mod langtonsant;

#[cfg(test)]
mod fields_tests;
#[cfg(test)]
mod ids_tests;

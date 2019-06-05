#![allow(unknown_lints)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

extern crate rayon;

pub mod engine;
pub mod examples;
pub mod fields;
pub mod ids;
pub mod utils;
pub mod visualisation;

#[cfg(test)]
mod fields_tests;
#[cfg(test)]
mod ids_tests;

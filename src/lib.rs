#![allow(unknown_lints)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate spectral;

extern crate flexi_logger;
extern crate log;

#[macro_use]
extern crate derive_more;

extern crate cgmath;
extern crate float_cmp;
extern crate time;

extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod cell;
pub mod engine;
pub mod fields;
pub mod ids;
pub mod utils;

pub mod langtonsant;

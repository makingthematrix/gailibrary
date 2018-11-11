#![allow(unknown_lints)]
#![feature(const_fn)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate spectral;

extern crate log;

extern crate cgmath;
extern crate float_cmp;
extern crate time;

#[macro_use]
extern crate derive_more;

extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod cell;
pub mod enums;
pub mod ids;
pub mod langtonsant;
pub mod math;
pub mod utils;

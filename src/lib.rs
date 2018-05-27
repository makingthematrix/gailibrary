#![allow(unknown_lints)]
#![feature(const_fn)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
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

pub mod cell;
pub mod ids;
pub mod math;
pub mod utils;
pub mod enums;
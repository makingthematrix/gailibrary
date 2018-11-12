#![allow(unknown_lints)]
#![feature(const_fn)]

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
extern crate gailibrary;

pub mod cell;
pub mod enums;
pub mod ids;
pub mod math;
pub mod utils;

pub mod langtonsant;

use flexi_logger::*;

fn main() {
    let mut b = LogSpecBuilder::new();
    b.default(log::LevelFilter::Info);
    let spec = b.finalize();
    Logger::with(spec)
        .log_to_file()
        .directory("log")
        .print_message()
        .duplicate_error()
        .duplicate_info()
        .format(flexi_logger::detailed_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    langtonsant::langtons_ant();
}

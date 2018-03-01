#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate spectral;

extern crate flexi_logger;
#[macro_use]
extern crate log;

pub mod ids;

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

    info!("Nothing here so far");
}

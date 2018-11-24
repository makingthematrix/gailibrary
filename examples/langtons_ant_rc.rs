extern crate flexi_logger;
extern crate gailibrary;

fn main() {
    feature::main();
}

mod feature {
    use flexi_logger::{LogSpecBuilder, Logger};
    use std::env;

    use gailibrary::langtonsant;

    pub fn main() {
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

        let args: Vec<String> = env::args().collect();
        println!("args: {:?}", args);

        let mut dim = 20;
        let mut steps = 100;
        if let Some(d_str) = args.get(1) {
            dim = d_str
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Unable to parse the argument for dim: {}", d_str));
        }
        if let Some(s_str) = args.get(2) {
            steps = s_str
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Unable to parse the argument for steps: {}", s_str));
        }

        langtonsant::langtons_ant_rc(dim, steps);
    }
}

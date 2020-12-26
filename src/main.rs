use clap::{App, Arg, SubCommand};

extern crate trfilter;
use trfilter::filter;
use trfilter::filter::defaults as def;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod subcmds {
    pub const SHOW: &str = "show";
}

mod args {
    pub const FILTER: &str = "filter";
}

fn main() {
    let matches = App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author("Balakrishnan Chandrasekaran <balakrishnan.c@gmail.com>")
        .about("Utility for editing Tresorit's roaming filter")
        .arg(
            Arg::with_name(args::FILTER)
                .short("f")
                .long("filter")
                .help("Absolute/relative path of a roaming filter")
                .default_value(def::FILTER_REL_PATH),
        )
        .subcommand(SubCommand::with_name(subcmds::SHOW).about("Show roaming filter"))
        .get_matches();

    if let Some(_cmd_input) = matches.subcommand_matches(subcmds::SHOW) {
        match filter::read_rules(matches.value_of(args::FILTER).unwrap()) {
            Ok(lines) => {
                for line in lines {
                    println!("[{:?}] {}", line.action, line.path)
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

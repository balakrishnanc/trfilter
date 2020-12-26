use clap::{App, SubCommand};

extern crate trfilter;
use trfilter::filter;
use trfilter::filter::defaults as def;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod subcommands {
    pub const SHOW: &str = "show";
}

fn main() {
    use self::subcommands::*;

    let _matches = App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author("Balakrishnan Chandrasekaran <balakrishnan.c@gmail.com>")
        .about("Utility for editing Tresorit's roaming filter")
        .subcommand(SubCommand::with_name(SHOW)
                    .about("Show roaming filter")
                    )
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches(SHOW) {
        match filter::read_rules(def::FILTER_REL_PATH.to_string()) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Error: {}", e)
        }
    }
}
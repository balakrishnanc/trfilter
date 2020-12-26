use clap::{App, SubCommand};

extern crate trfilter;
use trfilter::core;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod subcommands {
    pub const SHOW: &str = "show";
}

mod defaults {
    pub const ROAMING_FILTER_PATH: &str = ".tresorit/Filters/roaming.filter";
}


fn main() {
    use self::subcommands::*;
    use self::defaults::*;

    let _matches = App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author("Balakrishnan Chandrasekaran <balakrishnan.c@gmail.com>")
        .about("Utility for editing Tresorit's roaming filter")
        .subcommand(SubCommand::with_name(SHOW)
                    .about("Show roaming filter")
                    )
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches(SHOW) {
        core::read_rules(ROAMING_FILTER_PATH.to_string());
    }
}
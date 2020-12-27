use clap::{App, Arg, SubCommand};

use trfilter::filter::{self, defaults as def};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod subcmds {
    pub const SHOW: &str = "show";
}

mod args {
    pub const FILTER: &str = "filter";
}

// Show the rules read listed in the roaming filter file.
fn cmd_show(filter_file: &str) {
    match filter::list_rules(filter_file) {
        Ok(rules) => {
            for (pos, rule) in rules.iter().enumerate() {
                println!("» {:>3} {}", pos + 1, rule)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    let cli_opts = App::new(built_info::PKG_NAME)
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

    let filter_file: &str = cli_opts.value_of(args::FILTER).unwrap();

    if let Some(_cmd_input) = cli_opts.subcommand_matches(subcmds::SHOW) {
        cmd_show(filter_file);
    }
}

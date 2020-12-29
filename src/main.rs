#[macro_use]
extern crate lazy_static;
extern crate trfilter;

mod ext;
mod filter;

use clap::{App, Arg, SubCommand};
use std::process::exit;

use ext::cli;
use filter::defaults as def;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    let cli_opts = App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author("Balakrishnan Chandrasekaran <balakrishnan.c@gmail.com>")
        .about("Utility for editing Tresorit's roaming filter")
        .arg(
            Arg::with_name(cli::args::FILTER)
                .short("f")
                .long("filter")
                .help("Absolute/relative path of a roaming filter")
                .default_value(def::FILTER_REL_PATH),
        )
        .subcommand(
            SubCommand::with_name(cli::subcmds::SHOW)
                .about("Show rules specified in the roaming filter"),
        )
        .subcommand(
            SubCommand::with_name(cli::subcmds::CHECK)
                .about("Check rules in the roaming filter file"),
        )
        .subcommand(
            SubCommand::with_name(cli::subcmds::SUGGEST)
                .about("Suggest rules for adding to the roaming filter"),
        )
        .get_matches();

    let filter_file: &str = cli_opts.value_of(cli::args::FILTER).unwrap();

    if let Some(_cmd_input) = cli_opts.subcommand_matches(cli::subcmds::SHOW) {
        cli::cmd_show(filter_file);
    } else if let Some(_cmd_input) = cli_opts.subcommand_matches(cli::subcmds::CHECK) {
        cli::cmd_check(filter_file);
    } else if let Some(_cmd_input) = cli_opts.subcommand_matches(cli::subcmds::SUGGEST) {
        cli::cmd_suggest(filter_file);
    } else {
        eprintln!("{}", cli_opts.usage());
        exit(1);
    }
}

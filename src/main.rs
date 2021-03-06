#[macro_use]
extern crate lazy_static;
extern crate trfilter;

mod ext;
mod filter;

use clap::{App, Arg, SubCommand};
use std::io;
use std::process::exit;

use ext::cli;
use filter::defaults as def;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn handle_err(prefix: &str, res: io::Result<()>) {
    if let Err(err) = res {
        eprintln!("[Error] {}: {}", prefix, err);
        exit(-1);
    }
}

fn main() {
    let opts = App::new(built_info::PKG_NAME)
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
        .subcommand(
            SubCommand::with_name(cli::subcmds::UPGRADE)
                .about("Initialize or upgrade roaming filter with suggestions"),
        )
        .get_matches();

    let filter_file: &str = opts.value_of(cli::args::FILTER).unwrap();

    if let Some(_c) = opts.subcommand_matches(cli::subcmds::SHOW) {
        handle_err("Failed to show roaming filter", cli::cmd_show(filter_file));
    } else if let Some(_c) = opts.subcommand_matches(cli::subcmds::CHECK) {
        handle_err(
            "Failed to check roaming filter",
            cli::cmd_check(filter_file),
        );
    } else if let Some(_c) = opts.subcommand_matches(cli::subcmds::SUGGEST) {
        handle_err(
            "Failed to suggest updates to roaming filter",
            cli::cmd_suggest(filter_file),
        );
    } else if let Some(_c) = opts.subcommand_matches(cli::subcmds::UPGRADE) {
        handle_err(
            "Failed to upgrade roaming filter",
            cli::cmd_upgrade(filter_file),
        );
    } else {
        eprintln!("{}", opts.usage());
        exit(1);
    }
}

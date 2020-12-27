use clap::{App, Arg, SubCommand};

mod cli;
use trfilter::filter::defaults as def;

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
        .subcommand(SubCommand::with_name(cli::subcmds::SHOW).about("Show roaming filter"))
        .subcommand(SubCommand::with_name(cli::subcmds::CHECK).about("Check roaming filter"))
        .get_matches();

    let filter_file: &str = cli_opts.value_of(cli::args::FILTER).unwrap();

    if let Some(_cmd_input) = cli_opts.subcommand_matches(cli::subcmds::SHOW) {
        cli::cmd_show(filter_file);
    } else if let Some(_cmd_input) = cli_opts.subcommand_matches(cli::subcmds::CHECK) {
        cli::cmd_check(filter_file);
    }
}

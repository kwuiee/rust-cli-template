#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

use clap::{AppSettings, Clap};
use log::{debug, error, info, log_enabled, warn, Level};

#[derive(Clap)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp, setting = AppSettings::ArgRequiredElseHelp)]
#[clap(name = crate_name!(), version = crate_version!(), author = crate_authors!(), about = crate_description!())]
struct Opts {
    #[clap(long, number_of_values = 1, about = "Accept many.")]
    many: Vec<String>,
    #[clap(short, long, about = "Print verbose info.")]
    verbose: bool,
    #[clap(about = "Input bam file.")]
    path: String,
}

fn main() {
    let args = Opts::parse();
    env_logger::init();
    warn!("Hello, world!");
}

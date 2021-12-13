extern crate clap;

use anyhow::{anyhow, Context};
use clap::{App, Arg, ArgMatches, SubCommand};

mod common;
mod challenge01;
// mod challenge02;

use crate::common::ChallengeData;
use crate::challenge01::day_one_challenge;
// use crate::challenge02::DayTwoChallenge;

fn halt_on_err(error: anyhow::Error) {
    eprintln!("ERROR: {}", error);
    error.chain()
        .skip(1)
        .for_each(|cause| eprintln!("  because: {}", cause));

    std::process::exit(1);
}

fn load_and_run<'a>(matches: &ArgMatches<'a>, runner: fn(&ChallengeData) -> anyhow::Result<()>) -> anyhow::Result<()> {
    let data = ChallengeData::try_from_args(matches)
        .with_context(|| "Unable to load arguments.")?;
    
    runner(&data)
}

fn main() {
    let matches = App::new("aocode21")
        .version("2021.0.0")
        .subcommand(SubCommand::with_name("ch01")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("ch02")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .get_matches();

    let result = match matches.subcommand() {
        ("ch01", Some(sub_m)) => load_and_run(sub_m, day_one_challenge),
        _ => Err(anyhow!("No command specified"))
    };

    if let Err(e) = result {
        halt_on_err(e);
    }
}

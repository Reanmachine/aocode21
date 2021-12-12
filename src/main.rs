extern crate clap;

use clap::{App, Arg, SubCommand};

mod common;
mod challenge01;
mod challenge02;

use crate::common::{ArgumentError, Challenge};
use crate::challenge01::DayOneChallenge;
use crate::challenge02::DayTwoChallenge;

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

    let challenge: Result<Box<dyn Challenge>, ArgumentError> = match matches.subcommand() {
        ("ch01", Some(sub_m)) => DayOneChallenge::try_from(sub_m).map(|c| Box::new(c) as Box<dyn Challenge>),
        ("ch02", Some(sub_m)) => DayTwoChallenge::try_from(sub_m).map(|c| Box::new(c) as Box<dyn Challenge>),
        _ => Err(ArgumentError::InvalidSubcommand)
    };

    let result = match challenge {
        Ok(challenge) => challenge.run(),
        Err(ArgumentError::InvalidSubcommand) => Err(format!("Unknown subcommand: {}", matches.subcommand_name().unwrap())),
        Err(ArgumentError::MissingInput) => Err(format!("Missing input file.")),
    };

    match result {
        Ok(_) => {},
        Err(err) => eprintln!("ERR: {}", err)
    }
}

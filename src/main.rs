extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

trait Challenge {
    fn run(&self) -> Result<(), String>;
}

struct DayOneChallenge {
    input_file: String
}

impl Challenge for DayOneChallenge {
    fn run(&self) -> Result<(), String> {
        println!("Loading file: {}", self.input_file);
        Ok(())
    }
}

#[derive(Debug)]
enum ArgumentError {
    InvalidSubcommand,
    MissingInput,
}

impl<'a> TryFrom<&ArgMatches<'a>> for DayOneChallenge {
    type Error = ArgumentError;

    fn try_from(matches: &ArgMatches<'a>) -> Result<DayOneChallenge, Self::Error> {
        let mut obj = DayOneChallenge {
            input_file: "".to_string()
        };
        
        if let Some(input_file) = matches.value_of("INPUT") {
            obj.input_file = input_file.to_string();
        } else {
            return Err(ArgumentError::MissingInput);
        }

        return Ok(obj);
    }
}

fn main() {
    let matches = App::new("aocode21")
        .version("2021.0.0")
        .subcommand(SubCommand::with_name("ch01")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .get_matches();

    let challenge = match matches.subcommand() {
        ("ch01", Some(sub_m)) => DayOneChallenge::try_from(sub_m),
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

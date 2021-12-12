extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

use std::fs;

trait Challenge {
    fn run(&self) -> Result<(), String>;
}

#[derive(Debug)]
enum ArgumentError {
    InvalidSubcommand,
    MissingInput,
}

struct DayOneChallenge {
    input_file: String
}

impl Challenge for DayOneChallenge {
    fn run(&self) -> Result<(), String> {
        let data = match fs::read_to_string(&self.input_file) {
            Ok(d) => d,
            Err(err) => return Err(format!("Unable to load input file because -- {}", err))
        };

        let (lines, errors): (Vec<_>, Vec<_>) = data.split("\n")
            .filter(|v| *v != "")
            .map(|v| v.parse::<i32>())
            .partition(Result::is_ok);

        if errors.len() > 0 {
            return Err("One or more lines in the input file could not be converted to a number.".to_string());
        }

        let lines: Vec<i32> = lines.into_iter().map(Result::unwrap).collect();
        
        let mut increases = 0;

        for slice in lines.windows(2) {
            if slice[0] < slice[1] {
                increases = increases + 1;
            }
        }

        println!("Increases: {}", increases);
        Ok(())
    }
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

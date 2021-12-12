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

        let lines = data.split("\n");
        let mut last = -1;
        let mut increases = 0;

        for x in lines {
            if x == "" {
                continue;
            }
            
            if let Ok(val) = x.parse::<i32>() {
                if val > last && last != -1 {
                    increases = increases + 1;
                }
    
                last = val;
            } else {
                return Err(format!("The value {} is not a valid number.", x))
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

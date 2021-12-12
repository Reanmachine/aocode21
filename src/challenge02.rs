use clap::{ArgMatches};
use crate::common::{ArgumentError, Challenge};

pub struct DayTwoChallenge {
    input_file: String
}

impl Challenge for DayTwoChallenge {
    fn run(&self) -> Result<(), String> {
        println!("Challenge 2: {}", self.input_file);
        Ok(())
    }
}

impl<'a> TryFrom<&ArgMatches<'a>> for DayTwoChallenge {
    type Error = ArgumentError;

    fn try_from(matches: &ArgMatches<'a>) -> Result<DayTwoChallenge, Self::Error> {
        let mut obj = DayTwoChallenge {
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
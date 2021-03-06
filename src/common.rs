use std::fs;
use std::path::{Path, PathBuf};

use clap::{ArgMatches};
use thiserror::{Error};

#[derive(Error, Debug)]
pub enum ChallengeDataError {
    #[error("Unable to load challenge data")]
    LoadError,
    #[error("Invalid Arguments: {0}")]
    ArgumentError(String),
}

pub struct ChallengeData {
    pub input_file: PathBuf,
    lines: Vec<String>
}

impl ChallengeData {
    /// Try to construct the challenge data from a path
    fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, ChallengeDataError> {
        let path_buf = path.as_ref().to_path_buf();

        match fs::read_to_string(path) {
            Ok(value) => {
                let lines = value.lines()
                    .filter(|x| *x != "")
                    .map(|s| s.to_string()).collect();

                Ok(Self {
                    input_file: path_buf,
                    lines: lines
                })
            },
            Err(_) => Err(ChallengeDataError::LoadError)
        }
    }

    /// Try to construct the challenge data from an argument matches value
    pub fn try_from_args<'a>(matches: &'a ArgMatches) -> Result<Self, ChallengeDataError> {
        match matches.value_of("INPUT") {
            Some(input_file) => ChallengeData::try_from_path(input_file),
            None => Err(ChallengeDataError::ArgumentError("Input file missing".to_string()))
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    /// Process the lines into a collection of results/errors
    pub fn process<T, E>(&self, processor: fn(&String) -> Result<T, E>) -> (Vec<T>, Vec<E>) {
        let (values, errors) = self.lines.iter()
            .map(processor)
            .fold((Vec::new(), Vec::new()), |mut r, x| {
                match x {
                    Ok(v) => r.0.push(v),
                    Err(e) => r.1.push(e)
                }
                r
            });

        (values, errors)
    }
}

pub fn banner(challenge: &str, input: &PathBuf) {
    let input_str = match input.to_str() {
        Some(v) => v,
        None => "??"
    };

    let text = format!("// Advent of Code 2021 -- Challenge {} -- Input = {}", challenge, input_str);
    
    for _ in 0..text.to_string().len() {
        print!("/");
    }

    println!();
    println!("{}", text);
    println!();
}
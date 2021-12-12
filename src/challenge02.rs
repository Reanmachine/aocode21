use std::fs;

use anyhow::{anyhow, bail, Context, Result};
use clap::{ArgMatches};
use itertools::Itertools;

use crate::common::{ArgumentError, Challenge};

struct Submarine {
    position: i32,
    depth: i32
}

impl Submarine {
    /// Note the "depth" is increased by going down
    fn up(&mut self, by: i32) {
        self.depth = self.depth - by;
    }

    /// Note the "depth" is decreased by going up
    fn down(&mut self, by: i32) {
        self.depth = self.depth + by;
    }

    fn forward(&mut self, by: i32) {
        self.position = self.position + by;
    }

    fn compute(&self) -> i32 {
        self.position * self.depth
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Self { position: 0, depth: 0 }
    }
}

enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32)
}

fn get_direction(entry: String) -> Result<Direction> {
    let (action, value) = entry.split_whitespace()
        .take(2)
        .collect_tuple()
        .with_context(|| format!("The value '{}' does not have two elements", entry))?;

    let value: i32 = value.parse::<i32>()
        .with_context(|| format!("The value '{}' is not convertable to a number", value))?;

    match action {
        "forward" => Ok(Direction::Forward(value)),
        "up" => Ok(Direction::Up(value)),
        "down" => Ok(Direction::Down(value)),
        a => Err(anyhow!("Unknown action '{}'", a))
    }
}

pub struct DayTwoChallenge {
    input_file: String
}

impl Challenge for DayTwoChallenge {
    fn run(&self) -> std::result::Result<(), String> {
        
        let data = match fs::read_to_string(&self.input_file) {
            Ok(d) => d,
            Err(err) => return Err(format!("Unable to load input file because -- {}", err))
        };

        let (lines, errors): (Vec<_>, Vec<_>) = data.split("\n")
            .filter(|v| *v != "")
            .map(|v| get_direction(v.to_string()))
            .partition(Result::is_ok);

        if errors.len() > 0 {
            return Err("At least one input is not a valid direction".to_string());
        }

        let lines: Vec<Direction> = lines.into_iter().map(|v| v.unwrap()).collect();

        let mut submarine = Submarine::default();

        for line in lines {
            match line {
                Direction::Forward(x) => { submarine.forward(x) },
                Direction::Up(x) => { submarine.up(x) },
                Direction::Down(x) => { submarine.down(x) },
            }
        }

        let submarine = submarine;

        println!("Position: {}, Depth: {}, Result: {}", submarine.position, submarine.depth, submarine.compute());
        Ok(())
    }
}

impl<'a> TryFrom<&ArgMatches<'a>> for DayTwoChallenge {
    type Error = ArgumentError;

    fn try_from(matches: &ArgMatches<'a>) -> std::result::Result<DayTwoChallenge, Self::Error> {
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
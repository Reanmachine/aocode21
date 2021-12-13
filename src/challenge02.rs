use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;

use crate::common::{banner, ChallengeData};

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

pub fn day_two_challenge(data: &ChallengeData) -> Result<()> {
    let (lines, errors) = data.process(|v| get_direction(v.to_string()));
    
    if errors.len() != 0 {
        let failures = errors.iter().map(|e| format!("- {:?}", e)).join("\n");
        bail!("Some inputs are not valid directions:\n{}", failures);
    }

    let mut submarine = Submarine::default();

    for line in lines {
        match line {
            Direction::Forward(x) => { submarine.forward(x) },
            Direction::Up(x) => { submarine.up(x) },
            Direction::Down(x) => { submarine.down(x) },
        }
    }

    banner("02 - Part 1", &data.input_file);

    let submarine = submarine;
    println!("Position: {}, Depth: {}, Result: {}", submarine.position, submarine.depth, submarine.compute());

    Ok(())
}
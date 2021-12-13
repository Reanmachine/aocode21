use anyhow::{ensure, Result};

use crate::common::{banner, ChallengeData};

pub fn day_one_challenge(data: &ChallengeData) -> Result<()> {
    let (lines, errors) = data.process(|v| v.parse::<i32>());

    ensure!(errors.len() == 0, format!("{} entries could not be converted to numbers.", errors.len()));

    let increases = lines.windows(2)
        .filter(|x| x[0] < x[1])
        .count();

    banner("01", &data.input_file);
    println!("Increases: {}", increases);

    Ok(())
}
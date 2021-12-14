use anyhow::{ensure, Context, Result};

use crate::common::{banner, ChallengeData};



pub fn day_three_challenge(data: &ChallengeData) -> Result<()> {
    let (lines, errors) = data.process(|v| Ok(v.clone()) as Result<String>);
    
    ensure!(errors.len() == 0, "Some inputs had errors while processing.");
    ensure!(lines.iter().all(|v| v.len() == lines[0].len()), "Some inputs are of inconsistent length.");

    let width = lines[0].len();
    let majority = lines.len() / 2;
    let mut frequency = vec![0; width];

    for value in lines {
        let chars: Vec<char> = value.chars().collect();
        
        for index in 0..width {
            if chars[index] == '1' {
                frequency[index] = frequency[index] + 1;
            }
        }
    }

    let mut gamma = vec!['0'; width];
    let mut epsilon = vec!['0'; width];

    for index in 0..frequency.len() {
        if frequency[index] >= majority {
            gamma[index] = '1';
            epsilon[index] = '0';
        } else {
            gamma[index] = '0';
            epsilon[index] = '1';
        }
    }

    let gamma: String = gamma.iter().collect();
    let gamma_val = i64::from_str_radix(&gamma, 2)
        .with_context(|| format!("Unable to convert {} to number (Gamma)", gamma))?;
    let epsilon: String = epsilon.iter().collect();
    let epsilon_val = i64::from_str_radix(&epsilon, 2)
        .with_context(|| format!("Unable to convert {} to number (Epsilon)", epsilon))?;

    let consumption = gamma_val * epsilon_val;

    banner("03", &data.input_file);
    println!("Gamma: {}, Epsilon: {}, Consumption: {}", gamma_val, epsilon_val, consumption);

    Ok(())
}
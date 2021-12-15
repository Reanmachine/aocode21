use std::cmp::{min, Ordering};

use anyhow::{bail, ensure, Context, Result};

use crate::common::{banner, ChallengeData};

#[derive(Debug)]
struct DiagnosticEntry {
    raw: String,
    value: i32
}

impl DiagnosticEntry {
    fn len(&self) -> usize {
        self.raw.len()
    }

    fn at(&self, position: usize) -> Option<i32> {
        if position < self.len() as usize {
            let offset = self.len() - position - 1;
            Some(min(self.value & (1 << offset), 1))
        } else {
            None
        }
    }
}

impl TryFrom<&String> for DiagnosticEntry {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let cloned = value.clone();
        let value = i32::from_str_radix(&cloned, 2)
            .with_context(|| format!("The value {:?} is not a binary number.", cloned))?;

        if !cloned.chars().all(|c| c == '0' || c == '1') {
            bail!("The input {:?} is not a valid binary string.", cloned);
        }

        Ok(Self {
            raw: cloned,
            value: value
        })
    }
}

struct DiagnosticFeed<'a> {
    iter: std::slice::Iter<'a, DiagnosticEntry>,
    column: usize
}

impl<'a> DiagnosticFeed<'a> {
    fn new(entires: &'a Vec<DiagnosticEntry>, column: usize) -> Self {
        Self {
            iter: entires.iter(),
            column: column
        }
    }
}

impl Iterator for DiagnosticFeed<'_> {
    type Item = i32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(entry) => entry.at(self.column),
            None => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Debug)]
struct DiagnosticReport {
    width: usize,
    entries: Vec<DiagnosticEntry> 
}

impl DiagnosticReport {
    fn feed(&self, column: usize) -> DiagnosticFeed {
        DiagnosticFeed::new(&self.entries, column)
    }

    fn gamma(&self) -> Result<i32> {
        let threshold = (self.entries.len() / 2) as i32;
        let gamma_str: String = (0..self.width)
            .map(|col| self.feed(col).sum())
            .map(|v: i32| match v.cmp(&threshold) {
                Ordering::Less => '0',
                Ordering::Greater => '1',
                Ordering::Equal => '1'
            })
            .collect();

        Ok(i32::from_str_radix(&gamma_str, 2).unwrap())
    }

    fn epsilon(&self) -> Result<i32> {
        let threshold = (self.entries.len() / 2) as i32;
        let gamma_str: String = (0..self.width)
            .map(|col| self.feed(col).sum())
            .map(|v: i32| match v.cmp(&threshold) {
                Ordering::Less => '1',
                Ordering::Greater => '0',
                Ordering::Equal => '0'
            })
            .collect();

        Ok(i32::from_str_radix(&gamma_str, 2).unwrap())
    }
}

impl TryFrom<&Vec<String>> for DiagnosticReport {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let result: Result<Vec<DiagnosticEntry>, _> = value.iter()
            .map(|x| DiagnosticEntry::try_from(x))
            .collect();

        let entries = result?;
        let width = entries[0].len();

        ensure!(entries.iter().all(|x| x.len() == width), "Not all diagnostic entries have the same size.");

        Ok(Self {
            width: width,
            entries: entries
        })
    }
}

pub fn day_three_challenge(data: &ChallengeData) -> Result<()> {
    let report = DiagnosticReport::try_from(data.lines())?;

    banner("03 - Part 1", &data.input_file);

    let gamma = report.gamma()?;
    let epsilon = report.epsilon()?;

    let consumption = gamma * epsilon;

    println!("Gamma: {}, Epsilon: {}, Consumption: {}", gamma, epsilon, consumption);

    Ok(())
}
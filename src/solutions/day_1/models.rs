use anyhow::{anyhow, ensure, Error, Result};
use std::{collections::HashMap, str::FromStr};

const ERR_MISSING_VALUE: &str = "Missing value in column";
const ERR_PARSE_ERROR: &str = "Failed to parse value";
const ERR_UNEVEN_COLUMNS: &str = "Left and right columns are uneven";

pub struct Lists(pub Vec<i64>, pub Vec<i64>);

impl Lists {
    pub fn get_distance(&mut self) -> i64 {
        self.0.sort();
        self.1.sort();

        (0..self.0.len()).fold(0, |sum, index| {
            sum + self.0[index].abs_diff(self.1[index]) as i64
        })
    }

    pub fn get_similarity_score(&self) -> i64 {
        let mut counts = self.get_counts();
        (0..self.0.len()).fold(0, |sum, index| {
            sum + self.0[index] * *counts.entry(self.0[index]).or_insert(0)
        })
    }

    fn get_counts(&self) -> HashMap<i64, i64> {
        let mut counts = HashMap::new();
        let Lists(_, right) = self;

        for &n in right {
            *counts.entry(n).or_insert(0) += 1;
        }

        counts
    }
}

impl FromStr for Lists {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut col1: Vec<i64> = Vec::new();
        let mut col2 = Vec::new();

        for line in s.lines() {
            let mut parts = line.split_whitespace();

            let left: i64 = parts
                .next()
                .ok_or_else(|| anyhow!(ERR_MISSING_VALUE))?
                .parse()
                .map_err(|_| anyhow!(ERR_PARSE_ERROR))?;

            let right: i64 = parts
                .next()
                .ok_or_else(|| anyhow!(ERR_MISSING_VALUE))?
                .parse()
                .map_err(|_| anyhow!(ERR_PARSE_ERROR))?;

            col1.push(left);
            col2.push(right);
        }

        ensure!(col1.len() == col2.len(), ERR_UNEVEN_COLUMNS);
        Ok(Lists(col1, col2))
    }
}

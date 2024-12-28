use anyhow::{anyhow, ensure, Error, Result};
use std::str::FromStr;

const ERR_MISSING_VALUE: &str = "Missing value in column";
const ERR_PARSE_ERROR: &str = "Failed to parse value";
const ERR_UNEVEN_COLUMNS: &str = "Left and right columns are uneven";

pub struct Lists(pub Vec<u32>, pub Vec<u32>);

impl Lists {
    pub fn get_distance(&mut self) -> u32 {
        self.0.sort();
        self.1.sort();

        (0..self.0.len()).fold(0, |sum, index| sum + self.0[index].abs_diff(self.1[index]))
    }
}

impl FromStr for Lists {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut col1: Vec<u32> = Vec::new();
        let mut col2 = Vec::new();

        for line in s.lines() {
            let mut parts = line.split_whitespace();

            let left: u32 = parts
                .next()
                .ok_or_else(|| anyhow!(ERR_MISSING_VALUE))?
                .parse()
                .map_err(|_| anyhow!(ERR_PARSE_ERROR))?;

            let right: u32 = parts
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

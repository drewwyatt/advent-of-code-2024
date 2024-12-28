use crate::lib::solution::Solution;
use anyhow::Result;
use models::Lists;

mod models;

pub struct Day1Solution;

impl Solution for Day1Solution {
    fn part_1(&self, input: &str) -> Result<String> {
        let mut lists: Lists = input.parse()?;
        Ok(lists.get_distance().to_string())
    }

    fn part_2(&self, _input: &str) -> Result<String> {
        Ok(format!("TODO!"))
    }
}

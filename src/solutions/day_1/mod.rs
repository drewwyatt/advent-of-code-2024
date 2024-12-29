use crate::core::solution::Solution;
use anyhow::Result;
use models::Lists;

mod models;

pub struct Day1Solution;

impl Solution for Day1Solution {
    fn part_1(&self, input: &str) -> Result<String> {
        let mut lists: Lists = input.parse()?;
        Ok(lists.get_distance().to_string())
    }

    fn part_2(&self, input: &str) -> Result<String> {
        let lists: Lists = input.parse()?;
        Ok(lists.get_similarity_score().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        let solution = Day1Solution;
        let result = solution.part_1(INPUT);

        match result {
            Ok(output) => assert_eq!(output, "11".to_string()),
            Err(err) => panic!("Test failed with error: {}", err),
        }
    }

    #[test]
    fn part_2() {
        let solution = Day1Solution;
        let result = solution.part_2(INPUT);

        match result {
            Ok(output) => assert_eq!(output, "31".to_string()),
            Err(err) => panic!("Test failed with error: {}", err),
        }
    }
}

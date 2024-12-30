use crate::core::solution::Solution;
use anyhow::Result;
use models::Lists;

mod models;

pub struct Day1Solution;

impl Solution for Day1Solution {
    fn part_1(&self, input: &str) -> Result<i64> {
        let mut lists: Lists = input.parse()?;
        Ok(lists.get_distance())
    }

    fn part_2(&self, input: &str) -> Result<i64> {
        let lists: Lists = input.parse()?;
        Ok(lists.get_similarity_score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::util::assert_result_is;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        let solution = Day1Solution;
        assert_result_is(solution.part_1(INPUT), 11);
    }

    #[test]
    fn part_2() {
        let solution = Day1Solution;
        assert_result_is(solution.part_2(INPUT), 31);
    }
}

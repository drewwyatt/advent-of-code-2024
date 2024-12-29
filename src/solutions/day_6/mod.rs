use crate::core::solution::Solution;
use anyhow::Result;

mod models;

pub struct Day6Solution;

impl Solution for Day6Solution {
    fn part_1(&self, input: &str) -> Result<String> {
        todo!()
    }

    fn part_2(&self, _input: &str) -> Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::util::assert_result_is;

    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1() {
        let solution = Day6Solution;
        assert_result_is(solution.part_1(INPUT), "41");
    }
}

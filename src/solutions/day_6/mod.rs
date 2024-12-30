use crate::core::solution::Solution;
use anyhow::Result;
use models::Map;

mod models;

pub struct Day6Solution;

impl Solution for Day6Solution {
    fn part_1(&self, input: &str) -> Result<i64> {
        let mut map: Map = input.parse()?;
        map.run_protocol();
        Ok(map.count_visited())
    }

    fn part_2(&self, _input: &str) -> Result<i64> {
        Ok(-1) // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::util::assert_result_is;

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
        assert_result_is(solution.part_1(INPUT), 41);
    }
}

use crate::core::solution::Solution;
use models::Disk;

mod models;

pub struct Day9Solution;

impl Solution for Day9Solution {
    fn part_1(&self, input: &str) -> anyhow::Result<i64> {
        let mut disk: Disk = input.parse()?;
        disk.defrag()?;
        Ok(disk.checksum())
    }

    fn part_2(&self, input: &str) -> anyhow::Result<i64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::util::assert_result_is;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        let solution = Day9Solution;
        assert_result_is(solution.part_1(INPUT), 1928);
    }
}

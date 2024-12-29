use anyhow::Result;
use std::collections::HashMap;
use std::option::Option;
use std::sync::Arc;

pub trait Solution {
    fn part_1(&self, input: &str) -> Result<String>;
    fn part_2(&self, input: &str) -> Result<String>;
}

pub fn get_solution(day: &u8) -> Option<Arc<dyn Solution>> {
    let mut registry: HashMap<&u8, Arc<dyn Solution>> = HashMap::new();

    registry.insert(&1, Arc::new(crate::solutions::day_1::Day1Solution));
    registry.insert(&6, Arc::new(crate::solutions::day_6::Day6Solution));

    registry.get(day).cloned()
}

use anyhow::{Error, Result};
use std::fs;
use std::str::FromStr;

pub fn read_input_for_day(day: usize) -> Result<String> {
    let path = format!("./src/solutions/day_{}/INPUT.txt", day);
    fs::read_to_string(path).map_err(anyhow::Error::from)
}

pub fn read_input_as_vec<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let values = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<T>>();

    Ok(values)
}

pub fn assert_result_is(result: Result<i64, Error>, expected: i64) {
    match result {
        Ok(output) => assert_eq!(output, expected),
        Err(err) => panic!("Test failed with error: {}", err),
    }
}

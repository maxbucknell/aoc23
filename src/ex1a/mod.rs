use crate::{Input, Solution};
use itertools::Itertools;

pub struct Ex1A {
}

impl Ex1A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex1A {
    fn solve(&self) -> String {
        let result = calibrate_from_file("1.txt");

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

fn calibrate_from_file(path: &str) -> Result<u64, String> {
    let input = Input::get(path);

    if input.is_none() {
        return Err(format!("Could not find find in inputs: {}.", path));
    }

    let input = input.unwrap();
    let input = std::str::from_utf8(input.data.as_ref());

    if input.is_err() {
        return Err(format!("{:?}", input.unwrap_err()));
    }

    input.unwrap()
        .lines()
        .map(|line| calibrate_value(line))
        .fold_ok(0, |value, acc| value + acc)
}

fn calibrate_value(input: &str) -> Result<u64, String> {
    let mut first: Option<u64> = None;
    let mut last: Option<u64> = None;

    for char in input.chars() {
        if let Some(digit) = char.to_digit(10) {
            first = Some(u64::from(digit));
            break;
        }
    }

    for char in input.chars().rev() {
        if let Some(digit) = char.to_digit(10) {
            last = Some(u64::from(digit));
            break;
        }
    }

    match (first, last) {
        (Some(first), Some(last)) => Ok((first * 10) + last),
        (_,  _) => Err(format!("Input did not contain any digits: \"{}\"", input))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_value() {
        assert_eq!(calibrate_value("1abc2"), Ok(12));
        assert_eq!(calibrate_value("pqr3stu8vwx"), Ok(38));
        assert_eq!(calibrate_value("a1b2c3d4e5f"), Ok(15));
        assert_eq!(calibrate_value("treb7uchet"), Ok(77));
    }

    #[test]
    fn test_calibrate_from_file() {
        assert_eq!(calibrate_from_file("1a-example.txt"), Ok(142));
    }
}

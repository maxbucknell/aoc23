use crate::Solution;
use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::ops::Add;

pub struct Ex1A {
}

impl Ex1A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex1A {
    fn solve(&self) -> String {
        let stream = io::stdin().lock();
        let result = calibrate_from_stream(stream);

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

struct Input<T: BufRead> {
    line: String,
    stream: T
}

impl<T: BufRead> Input<T> {
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            line: String::new()
        }
    }
}

impl<T: BufRead> Iterator for Input<T> {
    type Item = Result<u64, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line.clear();

        match self.stream.read_line(&mut self.line) {
            Ok(0) => None,
            Ok(_) => Some(calibrate_value(&self.line)),
            Err(err) => Some(Err(format!("{:?}", err)))
        }
    }
}

fn calibrate_from_stream(stream: impl BufRead) -> Result<u64, String> {
    let mut input = Input::new(stream);

    input.fold_ok(0u64, Add::add)
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
    fn test_calibrate_from_stream() {
        let stream = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
".as_bytes();

        assert_eq!(calibrate_from_stream(stream), Ok(142));
    }
}

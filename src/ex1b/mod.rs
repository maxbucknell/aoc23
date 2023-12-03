use crate::Solution;
use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::ops::Add;

pub struct Ex1B {
}

impl Ex1B {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex1B {
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

    for (idx, _) in input.char_indices() {
        if let Some(digit) = extract_digit_from_start(&input[idx..]) {
            first = Some(u64::from(digit));
            break;
        }
    }

    for (idx, _) in input.char_indices().rev() {
        if let Some(digit) = extract_digit_from_end(&input[..(idx+1)]) {
            last = Some(u64::from(digit));
            break;
        }
    }

    match (first, last) {
        (Some(first), Some(last)) => Ok((first * 10) + last),
        (_,  _) => Err(format!("Input did not contain any digits: \"{}\"", input))
    }
}

fn extract_digit_from_start(input: &str) -> Option<u64> {
    if input.starts_with("0") {
        Some(0)
    } else if input.starts_with("1") {
        Some(1)
    } else if input.starts_with("2") {
        Some(2)
    } else if input.starts_with("3") {
        Some(3)
    } else if input.starts_with("4") {
        Some(4)
    } else if input.starts_with("5") {
        Some(5)
    } else if input.starts_with("6") {
        Some(6)
    } else if input.starts_with("7") {
        Some(7)
    } else if input.starts_with("8") {
        Some(8)
    } else if input.starts_with("9") {
        Some(9)
    } else if input.starts_with("one") {
        Some(1)
    } else if input.starts_with("two") {
        Some(2)
    } else if input.starts_with("three") {
        Some(3)
    } else if input.starts_with("four") {
        Some(4)
    } else if input.starts_with("five") {
        Some(5)
    } else if input.starts_with("six") {
        Some(6)
    } else if input.starts_with("seven") {
        Some(7)
    } else if input.starts_with("eight") {
        Some(8)
    } else if input.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn extract_digit_from_end(input: &str) -> Option<u64> {
    if input.ends_with("0") {
        Some(0)
    } else if input.ends_with("1") {
        Some(1)
    } else if input.ends_with("2") {
        Some(2)
    } else if input.ends_with("3") {
        Some(3)
    } else if input.ends_with("4") {
        Some(4)
    } else if input.ends_with("5") {
        Some(5)
    } else if input.ends_with("6") {
        Some(6)
    } else if input.ends_with("7") {
        Some(7)
    } else if input.ends_with("8") {
        Some(8)
    } else if input.ends_with("9") {
        Some(9)
    } else if input.ends_with("one") {
        Some(1)
    } else if input.ends_with("two") {
        Some(2)
    } else if input.ends_with("three") {
        Some(3)
    } else if input.ends_with("four") {
        Some(4)
    } else if input.ends_with("five") {
        Some(5)
    } else if input.ends_with("six") {
        Some(6)
    } else if input.ends_with("seven") {
        Some(7)
    } else if input.ends_with("eight") {
        Some(8)
    } else if input.ends_with("nine") {
        Some(9)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digit_from_start() {
        assert_eq!(extract_digit_from_start("two1nine"), Some(2));
        assert_eq!(extract_digit_from_start("eightwothree"), Some(8));
        assert_eq!(extract_digit_from_start("abcone2threexyz"), None);
        assert_eq!(extract_digit_from_start("one2threexyz"), Some(1));
        assert_eq!(extract_digit_from_start("xtwone3four"), None);
        assert_eq!(extract_digit_from_start("twone3four"), Some(2));
        assert_eq!(extract_digit_from_start("4nineeightseven2"), Some(4));
        assert_eq!(extract_digit_from_start("zoneight234"), None);
        assert_eq!(extract_digit_from_start("oneight234"), Some(1));
        assert_eq!(extract_digit_from_start("7pqrstsixteen"), Some(7));
    }

    #[test]
    fn test_extract_digit_from_end() {
        assert_eq!(extract_digit_from_end("two1nine"), Some(9));
        assert_eq!(extract_digit_from_end("eightwothree"), Some(3));
        assert_eq!(extract_digit_from_end("abcone2threexyz"), None);
        assert_eq!(extract_digit_from_end("abcone2three"), Some(3));
        assert_eq!(extract_digit_from_end("xtwone3four"), Some(4));
        assert_eq!(extract_digit_from_end("4nineeightseven2"), Some(2));
        assert_eq!(extract_digit_from_end("zoneight234"), Some(4));
        assert_eq!(extract_digit_from_end("7pqrstsixteen"), None);
        assert_eq!(extract_digit_from_end("7pqrstsix"), Some(6));
    }

    #[test]
    fn test_calibrate_value() {
        assert_eq!(calibrate_value("two1nine"), Ok(29));
        assert_eq!(calibrate_value("eightwothree"), Ok(83));
        assert_eq!(calibrate_value("abcone2threexyz"), Ok(13));
        assert_eq!(calibrate_value("xtwone3four"), Ok(24));
        assert_eq!(calibrate_value("4nineeightseven2"), Ok(42));
        assert_eq!(calibrate_value("zoneight234"), Ok(14));
        assert_eq!(calibrate_value("7pqrstsixteen"), Ok(76));
    }

    #[test]
        fn test_calibrate_from_stream() {
            let stream = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
".as_bytes();

            assert_eq!(calibrate_from_stream(stream), Ok(281));
        }
}

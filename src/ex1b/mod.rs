use crate::{Input, Solution};
use itertools::Itertools;

pub struct Ex1B {
}

impl Ex1B {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex1B {
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

    for (idx, _) in input.char_indices() {

        if let Some(digit) = extract_digit_from_start(&input[idx..]) {
            if first == None {
                first = Some(u64::from(digit));
            }

            last = Some(u64::from(digit));
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
    fn test_calibrate_from_file() {
        assert_eq!(calibrate_from_file("1b-example.txt"), Ok(281));
    }
}

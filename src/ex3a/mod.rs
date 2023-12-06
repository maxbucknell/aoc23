use crate::Solution;
use std::io;
use itertools::Itertools;
use std::io::BufRead;
use regex::Regex;
use std::cmp;
use std::ops::Add;


pub struct Ex3A {
}

impl Ex3A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex3A {
    fn solve(&self) -> String {
        let stream = io::stdin().lock();
        let result = solve_with_stream(stream);

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

fn solve_with_stream(stream: impl BufRead) -> Result<u64, String> {
    let input_result = Input::new(stream);

    match input_result {
        Ok(mut input) => input.fold_ok(0, Add::add),
        Err(err) => Err(err)
    }
}

/// We need to process the input stream line by line, but we need to look ahead
/// by one line, and behind by one line, to find adjacent symbols.
///
/// We just maintain three lines and rotate through them with a counter
struct Input<T: BufRead> {
    number_re: Regex,
    symbol_re: Regex,
    counter: usize,
    lines: [String; 3],
    stream: T
}

impl<T: BufRead> Input<T> {
    pub fn new(stream: T) -> Result<Self, String> {
        let number_re = Regex::new(r"\d+").unwrap();
        let symbol_re = Regex::new(r"[^\d\.]").unwrap();
        let mut result = Self {
            counter: 0,
            number_re,
            symbol_re,
            stream,
            lines: [String::new(), String::new(), String::new()]
        };

        match result.stream.read_line(&mut result.lines[0]) {
            Ok(_) => {},
            Err(err) => {
                return Err(format!("{:?}", err));
            }
        }

        match result.stream.read_line(&mut result.lines[1]) {
            Ok(_) => {},
            Err(err) => {
                return Err(format!("{:?}", err));
            }
        }

        Ok(result)
    }
}

impl<T: BufRead> Iterator for Input<T> {
    type Item = Result<u64, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = &self.lines[(self.counter + 2) % 3];
        let curr = &self.lines[self.counter];
        let next = &self.lines[(self.counter + 1) % 3];

        let result: Option<Self::Item>;

        if curr.len() == 0 {
            result = None
        } else {
            let sum = self.number_re
                .captures_iter(&curr)
                .map(|captures| {
                    let m = captures.get(0).unwrap();

                    Number {
                        value: m.as_str().parse().unwrap(),
                        start: m.start(),
                        end: m.end()
                    }
                })
                .filter(|num| {
                    let prev = num.get_adjacent_range_from_str(&prev);
                    let curr = num.get_adjacent_range_from_str(&curr);
                    let next = num.get_adjacent_range_from_str(&next);

                    self.symbol_re.is_match(prev) || self.symbol_re.is_match(next) || self.symbol_re.is_match(curr)
                })
                .fold(0u64, |acc, num| acc + num.value);

            result = Some(Ok(sum));
        }

        // We move down to 1, we need to read 2
        self.counter = (self.counter + 1) % 3;
        let mut next = &mut self.lines[(self.counter + 1) % 3];
        next.clear();

        match self.stream.read_line(&mut next) {
            Ok(_) => {},
            Err(err) => {
                return Some(Err(format!("{:?}", err)));
            }
        }

        result
    }
}

#[derive(Debug)]
struct Number {
    value: u64,
    start: usize,
    end: usize,
}

impl Number {
    fn get_adjacent_range_from_str<'a>(&self, s: &'a str) -> &'a str {
        if self.start >= s.len() {
            &s[0..0]
        } else {
            let start = if self.start == 0 { 0 } else { self.start - 1 };
            let end = cmp::min(s.len() - 1, self.end + 1);

            &s[start..end]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let stream = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
".as_bytes();

        assert_eq!(solve_with_stream(stream).unwrap(), 4361);
    }

    #[test]
    fn test_adjacent_range() {
        let s = "1234567890\n";

        // 111.......
        // xxxx......
        let n = Number {
            value: 111,
            start: 0,
            end: 3
        };

        assert_eq!(n.get_adjacent_range_from_str(&s), "1234");

        // ..111.....
        // .xxxxx....
        let n = Number {
            value: 111,
            start: 2,
            end: 5
        };

        assert_eq!(n.get_adjacent_range_from_str(&s), "23456");

        // ......1111
        // .....xxxxx
        let n = Number {
            value: 111,
            start: 6,
            end: 10
        };

        assert_eq!(n.get_adjacent_range_from_str(&s), "67890");

        // ......1111
        // .....xxxx
        let n = Number {
            value: 111,
            start: 6,
            end: 10
        };

        assert_eq!(n.get_adjacent_range_from_str("123456789\n"), "6789");

        // ......1111
        // -
        let n = Number {
            value: 111,
            start: 6,
            end: 10
        };

        assert_eq!(n.get_adjacent_range_from_str(""), "");
    }

    #[test]
    fn test_re() {
        let symbol_re = Regex::new(r"[^\d\.]").unwrap();

        assert!(symbol_re.is_match( "...@........"));
        assert!(!symbol_re.is_match(".1234567890."));
        assert!(!symbol_re.is_match("............"));
    }
}

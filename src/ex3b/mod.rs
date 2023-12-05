use crate::Solution;
use std::io;
use itertools::Itertools;
use std::io::BufRead;
use std::ops::Range;
use std::collections::HashSet;
use std::ops::Add;


pub struct Ex3B {}

impl Ex3B {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex3B {
    fn solve(&self) -> String {
        let stream = io::stdin().lock();
        let result: Result<u64, String> = solve_with_stream(stream);

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

fn solve_with_stream(stream: impl BufRead) -> Result<u64, String> {
    let mut input = Input::new(stream);

    input.fold_ok(0, Add::add)
}

struct Input<T: BufRead> {
    buffer: String,
    counter: usize,
    lines: [Option<Line>; 3],
    stream: T
}

impl<T: BufRead> Input<T> {
    pub fn new(stream: T) -> Self {
        let mut result = Self {
            buffer: String::new(),
            counter: 2,
            stream,
            lines: [None, None, None]
        };

        result.read_line();
        result.counter = 0;
        result.read_line();

        result
    }

    fn read_line(&mut self) {
        self.buffer.clear();
        let idx = (self.counter + 1) % 3;

        self.lines[idx] = match self.stream.read_line(&mut self.buffer) {
            Ok(0) => None,
            Ok(_) => Some(Line::new(&self.buffer)),
            Err(_) => None
        };
    }
}

impl<T: BufRead> Iterator for Input<T> {
    type Item = Result<u64, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = &self.lines[(self.counter + 2) % 3];
        let Some(curr) = &self.lines[self.counter] else { return None; };
        let next = &self.lines[(self.counter + 1) % 3];

        let mut total = 0u64;

        for gear in &curr.gears {
            let mut numbers: HashSet<&Number> = HashSet::new();

            for x in curr.get_adjacent_numbers_from_location(*gear) {
                numbers.insert(x);
            }

            if let Some(prev) = prev {
                for x in prev.get_adjacent_numbers_from_location(*gear) {
                    numbers.insert(x);
                }
            }

            if let Some(next) = next {
                for x in next.get_adjacent_numbers_from_location(*gear) {
                    numbers.insert(x);
                }
            }

            total += if numbers.len() == 2 {
                numbers.into_iter().fold(1u64, |acc, num| num.value * acc)
            } else {
                0
            };
        }

        self.counter = (self.counter + 1) % 3;
        self.read_line();

        Some(Ok(total))
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    nums: Vec<Number>,
    gears: Vec<usize>
}

impl Line {
    pub fn new(raw: &str) -> Self {
        let mut nums: Vec<Number> = vec![];
        let mut num = String::new();
        let mut start: Option<usize> = None;

        let mut gears: Vec<usize> = vec![];

        for (i, c) in raw.char_indices() {
            if c.is_digit(10) {
                if start.is_none() {
                    start = Some(i);
                }

                num.push(c);
            } else {
                if start.is_some() {
                    nums.push(Number {
                        value: num.parse().unwrap(),
                        start: start.unwrap(),
                        end: i
                    });

                    num.clear();
                    start = None;
                }

                if c == '*' {
                    gears.push(i);
                }
            }
        }

        Self { nums, gears }
    }

    pub fn get_number_at_location(&self, i: usize) -> Option<&Number> {
        for num in &self.nums {
            if num.start <= i && i < num.end {
                return Some(&num);
            }
        }

        None
    }

    pub fn get_adjacent_numbers_from_location(&self, location: usize) -> HashSet<&Number> {
        let mut result = HashSet::new();

        for i in get_adjacent_range_from_location(location) {
            match self.get_number_at_location(i) {
                Some(n) => {
                    result.insert(n);
                },
                None => {}
            }
        }

        result
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Number {
    value: u64,
    start: usize,
    end: usize,
}

fn get_adjacent_range_from_location(i: usize) -> Range<usize> {
    if i == 0 {
        0..2
    } else {
        Range { start: i - 1, end: i + 2 }
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

        let result = solve_with_stream(stream);

        assert_eq!(result, Ok(467835));
    }

    #[test]
    fn test_line() {
        let raw = "617*......";
        let line = Line::new(&raw);

        let expected = Line {
            gears: vec![3],
            nums: vec![
                Number {
                    value: 617,
                    start: 0,
                    end: 3
                }
            ]
        };

        assert_eq!(line, expected);

        let raw = "...........$326.......683.-761.......491......./.........26...........................*.....=.........56.305.262.......................150..";
        let line = Line::new(&raw);

        let expected = Line {
            gears: vec![86],
            nums: vec![
                Number {
                    value: 326,
                    start: 12,
                    end: 15
                },
                Number {
                    value: 683,
                    start: 22,
                    end: 25
                },
                Number {
                    value: 761,
                    start: 27,
                    end: 30
                },
                Number {
                    value: 491,
                    start: 37,
                    end: 40
                },
                Number {
                    value: 26,
                    start: 57,
                    end: 59
                },
                Number {
                    value: 56,
                    start: 102,
                    end: 104
                },
                Number {
                    value: 305,
                    start: 105,
                    end: 108
                },
                Number {
                    value: 262,
                    start: 109,
                    end: 112
                },
                Number {
                    value: 150,
                    start: 135,
                    end: 138
                }
            ]
        };

        assert_eq!(line, expected);
    }

    #[test]
    fn test_get_adjacent_range_from_str() {
        let raw = "..35..633.";
        let line = Line::new(raw);

        assert_eq!(line.get_number_at_location(0), None);
        assert_eq!(line.get_number_at_location(1), None);
        assert_eq!(line.get_number_at_location(2), Some(&line.nums[0]));
        assert_eq!(line.get_number_at_location(3), Some(&line.nums[0]));
        assert_eq!(line.get_number_at_location(4), None);
        assert_eq!(line.get_number_at_location(5), None);
        assert_eq!(line.get_number_at_location(6), Some(&line.nums[1]));
        assert_eq!(line.get_number_at_location(7), Some(&line.nums[1]));
        assert_eq!(line.get_number_at_location(8), Some(&line.nums[1]));
        assert_eq!(line.get_number_at_location(9), None);
    }

    #[test]
    fn test_get_adjacent_range_from_location() {
        assert_eq!(get_adjacent_range_from_location(0), 0usize..2usize);
        assert_eq!(get_adjacent_range_from_location(4), 3usize..6usize);
        assert_eq!(get_adjacent_range_from_location(100), 99usize..102usize);
    }

    #[test]
    fn test_get_adjacent_numbers_from_location() {
        let raw = "617*......";
        let line = Line::new(&raw);

        let result = line.get_adjacent_numbers_from_location(3);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&line.nums[0]));
    }
}

use crate::Solution;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::ops::Add;

pub struct Ex4A {}

impl Ex4A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex4A {
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
    let input = Input::new(stream);

    Ok(input.map(|c| c.score()).fold(0, Add::add))
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
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.line.clear();

        match self.stream.read_line(&mut self.line) {
            Ok(0) => None,
            Ok(_) => Some(Card::new(&self.line)),
            Err(_) => None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u64,
    nums: HashSet<u64>,
    wins: HashSet<u64>,
}

impl Card {
    pub fn new(line: &str) -> Self {
        let mut id = 0u64;
        let mut num = String::new();

        let mut wins: HashSet<u64> = HashSet::new();
        let mut nums: HashSet<u64> = HashSet::new();

        let mut is_past_wins = false;


        for c in line[4..].chars() {
            if c == ':' {
                id = num.parse().unwrap();
                num.clear();
            } else if c.is_whitespace() && num.len() > 0 {
                let n: u64 = num.parse().unwrap();
                match is_past_wins {
                    false => { wins.insert(n); },
                    true => { nums.insert(n); }
                }
                num.clear();
            } else if c.is_digit(10) {
                num.push(c);
            } else if c == '|' {
                is_past_wins = true;
            }
        }

        if num.len() > 0 {
            nums.insert(num.parse().unwrap());
            num.clear();
        }


        Self { id, nums, wins }
    }

    pub fn score(&self) -> u64 {
        let matches = self.wins.intersection(&self.nums).count();

        match matches {
            0 => 0,
            x => 2u64.pow((x as u32) - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let stream = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
".as_bytes();

        assert_eq!(solve_with_stream(stream).unwrap(), 13);
    }

    #[test]
    fn test_card() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::new(&line);

        let expected = Card {
            id: 1,
            wins: HashSet::from([41, 48, 83, 86, 17]),
            nums: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        assert_eq!(card, expected);

        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = Card::new(&line);

        let expected = Card {
            id: 2,
            wins: HashSet::from([13, 32, 20, 16, 61]),
            nums: HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]),
        };

        assert_eq!(card, expected);
    }

    #[test]
    fn test_score() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::new(&line);

        assert_eq!(card.score(), 8);

        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = Card::new(&line);

        assert_eq!(card.score(), 2);

        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let card = Card::new(&line);

        assert_eq!(card.score(), 2);

        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let card = Card::new(&line);

        assert_eq!(card.score(), 1);

        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let card = Card::new(&line);

        assert_eq!(card.score(), 0);

        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let card = Card::new(&line);

        assert_eq!(card.score(), 0);
    }
}

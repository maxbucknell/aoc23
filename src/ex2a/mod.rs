use crate::Solution;
use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::ops::Add;
use crate::ex2::game::Game;
use crate::ex2::cube_set::CubeSet;

pub struct Ex2A {
}

impl Ex2A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex2A {
    fn solve(&self) -> String {
        let set = CubeSet::with_rgb(12, 13, 14);
        let stream = io::stdin().lock();
        let result = solve_with_stream_and_set(stream, &set);

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
    type Item = Result<Game, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line.clear();

        match self.stream.read_line(&mut self.line) {
            Ok(0) => None,
            Ok(_) => Some(Game::new(&self.line)),
            Err(err) => Some(Err(format!("{:?}", err)))
        }
    }
}

fn solve_with_stream_and_set(stream: impl BufRead, set: &CubeSet) -> Result<u64, String> {
    let input = Input::new(stream);

    input
        .filter_ok(|game| game.is_possible_with_set(set))
        .map_ok(|game| game.id)
        .fold_ok(0, Add::add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let stream = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
".as_bytes();

        let set = CubeSet::with_rgb(12, 12, 12);

        let solution = solve_with_stream_and_set(stream, &set).unwrap();

        assert_eq!(solution, 8);
    }

}

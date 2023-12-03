use crate::{Input, Solution};
use itertools::Itertools;

use crate::ex2::game::Game;

pub struct Ex2B {
}

impl Ex2B {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex2B {
    fn solve(&self) -> String {
        let result = solve_with_file_and_set("2.txt");

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

fn solve_with_file_and_set(path: &str) -> Result<u64, String> {
    let Some(input) = Input::get(path) else {
        return Err(format!("Could not find file in inputs: {}.", path));
    };

    let input = std::str::from_utf8(input.data.as_ref());

    if input.is_err() {
        return Err(format!("{:?}", input.unwrap_err()));
    }

    input.unwrap()
        .lines()
        .map(|line| Game::new(line))
        .map_ok(|game| game.minimal_set().power())
        .fold_ok(0, |id, acc| acc + id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let solution = solve_with_file_and_set("2-example.txt").unwrap();

        assert_eq!(solution, 2286);
    }
}

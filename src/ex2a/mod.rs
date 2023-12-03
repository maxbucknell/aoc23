use crate::{Input, Solution};
use itertools::Itertools;



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
        let result = solve_with_file_and_set("2.txt", &set);

        match result {
            Ok(value) => value.to_string(),
            Err(err) => err
        }
    }
}

fn solve_with_file_and_set(path: &str, set: &CubeSet) -> Result<u64, String> {
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
        .filter_ok(|game| game.is_possible_with_set(set))
        .map_ok(|game| game.id)
        .fold_ok(0, |id, acc| acc + id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let solution = solve_with_file_and_set(
            "2-example.txt",
            &CubeSet::with_rgb(12, 12, 12)
        ).unwrap();

        assert_eq!(solution, 8);
    }
}

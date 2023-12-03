use regex::Regex;
use once_cell::sync::Lazy;

use crate::ex2::cube_set::CubeSet;

#[derive(PartialEq, Debug)]
pub struct Game {
    pub id: u64,
    pub rounds: Vec<CubeSet>
}

impl Game {
    pub fn new(input: &str) -> Result<Self, String> {
        static ID_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"Game (?<id>\d+):").unwrap()
        });

        static ROUND_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"[;:] [^;]+").unwrap()
        });

        static COUNT_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?<count>\d+) (?<color>blue|green|red)").unwrap()
        });


        let Some(id_captures) = ID_RE.captures(input) else {
            return Err(format!("Cannot find id in input: {}", input));
        };

        let id = &id_captures["id"];
        let Ok(id) = id.parse() else {
            return Err(format!("Could not parse id from input."));
        };

        let rounds: Vec<CubeSet> = ROUND_RE.captures_iter(input).map(|captures| {
            let round = &captures[0];
            let mut result = CubeSet::with_zero();

            COUNT_RE.captures_iter(round).for_each(|captures| {
                let count = &captures["count"];
                let count = count.parse().unwrap();
                let color = &captures["color"];

                match color {
                    "red" => result.red = count,
                    "blue" => result.blue = count,
                    "green" => result.green = count,
                    _ => {}
                };
            });

            result
        }).collect();

        Ok(Game { id, rounds })
    }

    pub fn minimal_set(&self) -> CubeSet {
        self.rounds.iter().fold(CubeSet::with_zero(), |round, max| {
            let r = if round.red > max.red {
                round.red
            } else {
                max.red
            };

            let g = if round.green > max.green {
                round.green
            } else {
                max.green
            };

            let b = if round.blue > max.blue {
                round.blue
            } else {
                max.blue
            };

            CubeSet::with_rgb(r, g, b)
        })
    }

    pub fn is_possible_with_set(&self, set: &CubeSet) -> bool {
        self.minimal_set().is_contained_by(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::new(input).unwrap();

        assert_eq!(game, Game {
            id: 1,
            rounds: vec![
                CubeSet::with_rgb(4, 0, 3),
                CubeSet::with_rgb(1, 2, 6),
                CubeSet::with_rgb(0, 2, 0)
            ]
        });

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = Game::new(input).unwrap();

        assert_eq!(game, Game {
            id: 2,
            rounds: vec![
                CubeSet::with_rgb(0, 2, 1),
                CubeSet::with_rgb(1, 3, 4),
                CubeSet::with_rgb(0, 1, 1)
            ]
        });

        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::new(input).unwrap();

        assert_eq!(game, Game {
            id: 3,
            rounds: vec![
                CubeSet::with_rgb(20, 8, 6),
                CubeSet::with_rgb(4, 13, 5),
                CubeSet::with_rgb(1, 5, 0)
            ]
        });

        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = Game::new(input).unwrap();

        assert_eq!(game, Game {
            id: 4,
            rounds: vec![
                CubeSet::with_rgb(3, 1, 6),
                CubeSet::with_rgb(6, 3, 0),
                CubeSet::with_rgb(14, 3, 15)
            ]
        });

        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game = Game::new(input).unwrap();

        assert_eq!(game, Game {
            id: 5,
            rounds: vec![
                CubeSet::with_rgb(6, 3, 1),
                CubeSet::with_rgb(1, 2, 2)
            ]
        });
    }

    #[test]
    fn test_minimal_set() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::new(input).unwrap();

        assert_eq!(game.minimal_set(), CubeSet::with_rgb(4, 2, 6));

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = Game::new(input).unwrap();

        assert_eq!(game.minimal_set(), CubeSet::with_rgb(1, 3, 4));

        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::new(input).unwrap();

        assert_eq!(game.minimal_set(), CubeSet::with_rgb(20, 13, 6));

        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = Game::new(input).unwrap();

        assert_eq!(game.minimal_set(), CubeSet::with_rgb(14, 3, 15));

        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game = Game::new(input).unwrap();

        assert_eq!(game.minimal_set(), CubeSet::with_rgb(6, 3, 2));
    }

    #[test]
    fn test_is_possible_with_set() {
        let inputs = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ];

        let bag = CubeSet::with_rgb(12, 12, 12);

        let matching_games: Vec<u64> = inputs.iter()
            .map(|input| Game::new(input).unwrap())
            .filter(|game| game.is_possible_with_set(&bag))
            .map(|game| game.id)
            .collect();

        assert_eq!(matching_games, vec![1, 2, 5]);
    }
}

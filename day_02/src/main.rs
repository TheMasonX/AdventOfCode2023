use regex::Regex;
use std::fs;
use Color::*;

fn main() {
    let input_file = "C:/Users/TheMasonX/Documents/Rust/AdventOfCode2023/day_02/input.txt";
    let input_text = match fs::read_to_string(input_file) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Couldn't open file {}: {:?}", input_file, e);
            return;
        }
    };

    let mut games = Vec::new();
    for line in input_text.lines() {
        games.push(Game::new(line));
    }

    let given_set = Set::new("12 red, 13 green, 14 blue");
    let mut output = 0;
    for game in games {
        if game.validate(&given_set) {
            output += game.id;
        }
    }

    println!("{}", output);
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let mut new_game = Game {
            id: -1,
            sets: Vec::new(),
        };

        let re = Regex::new(r"Game (?<id>\d+):").unwrap();
        let Some(caps) = re.captures(input) else {
            return new_game;
        };
        new_game.id = caps["id"].parse::<i32>().unwrap();

        let find = input.find(':').unwrap();
        let after_header = &input[find + 1..];
        let set_strings = after_header.split(';');
        for set_string in set_strings {
            new_game.add_set(Set::new(set_string));
        }

        new_game
    }

    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }

    pub fn validate(&self, given_set: &Set) -> bool {
        self.sets.iter().all(|set| set.validate(given_set))
    }
}

#[derive(Debug, PartialEq)]
pub struct Set {
    colors: Vec<ColorCount>,
}

impl Set {
    pub fn new(input: &str) -> Set {
        let mut new_set = Set { colors: Vec::new() };

        let color_strings: Vec<&str> = input.trim().split(',').collect();
        for color_string in color_strings {
            let kvp: Vec<&str> = color_string.trim().split(' ').collect();
            let int = match kvp.first().unwrap().trim().parse() {
                Ok(ok) => ok,
                Err(e) => {
                    println!("Error parsing {} into int: {:?}", color_string, e);
                    continue;
                }
            };
            let color_name = kvp.last().unwrap().trim();
            let color = match color_name {
                "red" => ColorCount::new(Red, int),
                "green" => ColorCount::new(Green, int),
                "blue" => ColorCount::new(Blue, int),
                _ => {
                    println!("Error parsing {} into '# Color'", color_string);
                    continue;
                }
            };
            new_set.colors.push(color);
        }
        new_set
    }

    pub fn validate(&self, given_set: &Set) -> bool {
        for color in &self.colors {
            for other_color in &given_set.colors {
                if !color.validate(other_color) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct ColorCount {
    color: Color,
    count: i32,
}

impl ColorCount {
    pub fn new(color: Color, count: i32) -> ColorCount {
        ColorCount { color, count }
    }

    pub fn validate(&self, other: &ColorCount) -> bool {
        if other.color != self.color {
            return true;
        }
        self.count <= other.count
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set() {
        let set_input = "9 green, 4 blue, 3 red";
        let set = Set::new(set_input);
        assert_eq!(set.colors.len(), 3);
        assert_eq!(
            set.colors,
            vec![
                ColorCount::new(Green, 9),
                ColorCount::new(Blue, 4),
                ColorCount::new(Red, 3)
            ]
        );
    }

    #[test]
    fn parse_game() {
        let game_input = "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red; 1 red, 1 green, 2 blue; 1 green, 1 red; 1 green; 1 green, 1 blue, 1 red";
        let game = Game::new(game_input);

        let mut expected_game = Game {
            id: 1,
            sets: Vec::new(),
        };
        expected_game.add_set(Set::new("1 green, 4 blue"));
        expected_game.add_set(Set::new("1 blue, 2 green, 1 red"));
        expected_game.add_set(Set::new("1 red, 1 green, 2 blue"));
        expected_game.add_set(Set::new("1 green, 1 red"));
        expected_game.add_set(Set::new("1 green"));
        expected_game.add_set(Set::new("1 green, 1 blue, 1 red"));

        assert_eq!(game.sets.len(), 6);
        assert_eq!(game, expected_game);
    }

    #[test]
    fn validate_game() {
        let given_set = Set::new("12 red, 13 green, and 14 blue");

        let valid_game = Game::new("Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let invalid_game =
            Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        assert!(valid_game.validate(&given_set));
        assert!(!invalid_game.validate(&given_set));
    }

    #[test]
    fn valid_games() {
        let given_set = Set::new("12 red, 13 green, and 14 blue");
        let expected_output = 8;

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut games = Vec::new();
        for line in input.lines() {
            games.push(Game::new(line));
        }

        let mut output = 0;
        for game in games {
            if game.validate(&given_set) {
                output += game.id;
            }
        }
        assert_eq!(expected_output, output);
    }
}

use derive_new::new;
use regex::Regex;
use smart_default::SmartDefault;
use std::collections::HashMap;
use tmx_utils::string_ext;
use Color::*;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let mut games = Vec::new();
    for line in input_text.lines() {
        games.push(Game::new(line));
    }

    let given_set = Set::new("12 red, 13 green, 14 blue");
    let mut output = 0;
    let mut power_output = 0;
    for game in games {
        if game.validate(&given_set) {
            output += game.id;
        }
        power_output += game.min_set.get_power();
    }

    println!("Ouput: {} | Power: {}", output, power_output);
}

#[derive(Debug, PartialEq, SmartDefault)]
pub struct Game {
    #[default(-1)]
    id: i32,
    #[default(Vec::new())]
    sets: Vec<Set>,
    min_set: Set,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let mut new_game = Game::default();
        let after_header = new_game.strip_header(input);
        new_game.parse_sets(after_header);
        new_game
    }

    fn strip_header<'a>(&mut self, input: &'a str) -> &'a str {
        let re = Regex::new(r"Game (?<id>\d+): ").unwrap();
        let captures = match re.captures(input) {
            Some(caps) => caps,
            None => return input,
        };
        self.id = captures["id"].parse::<i32>().unwrap_or(-1);
        &input[captures.get(0).unwrap().end()..] // return the rest of the text
    }

    fn parse_sets(&mut self, input: &str) {
        let mut sets: Vec<Set> = input.split(';').map(Set::new).collect();
        self.sets.append(&mut sets);
        self.min_set.create_min_set(&self.sets);
    }

    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }

    pub fn validate(&self, given_set: &Set) -> bool {
        self.sets.iter().all(|set| set.validate(given_set))
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Set {
    colors: Vec<ColorCount>,
}

impl Set {
    pub fn new(input: &str) -> Set {
        let mut new_set = Set::default();

        let color_strings: Vec<&str> = input.trim().split(',').collect();
        for color_string in color_strings {
            new_set.parse_color(color_string);
        }
        new_set.colors.sort();
        new_set
    }

    pub fn validate(&self, given_set: &Set) -> bool {
        let validate = |other| self.colors.iter().any(|c| !c.validate(other));
        !given_set.colors.iter().any(validate)
    }

    pub fn get_power(&self) -> i32 {
        let power: i32 = self.colors.iter().map(|c| c.count).product();
        power
    }

    fn create_min_set(&mut self, sets: &Vec<Set>) {
        let mut counts: HashMap<Color, i32> = HashMap::new();
        for set in sets {
            for color in &set.colors {
                let count = counts.entry(color.color).or_insert(color.count);
                if *count < color.count {
                    counts.entry(color.color).and_modify(|c| {
                        *c = color.count;
                    });
                }
            }
        }
        self.colors = counts
            .iter()
            .map(|(k, v)| ColorCount::new(*k, *v))
            .collect();
        self.colors.sort();
    }

    fn parse_color(&mut self, color_string: &str) {
        let kvp: Vec<&str> = color_string.trim().split(' ').collect();
        let int = match kvp.first().unwrap().trim().parse() {
            Ok(ok) => ok,
            Err(e) => {
                println!("Error parsing {} into int: {:?}", color_string, e);
                return;
            }
        };
        let color_name = kvp.last().unwrap().trim();
        self.colors.push(ColorCount::from_string(color_name, int));
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, new)]
pub struct ColorCount {
    color: Color,
    count: i32,
}

impl ColorCount {
    /// Create a new ColorCount from a string
    ///
    /// # Panics
    /// If the string is not a valid color ()
    pub fn from_string(color: &str, count: i32) -> ColorCount {
        ColorCount::new(
            match color {
                "red" => Red,
                "green" => Green,
                "blue" => Blue,
                _ => panic!("{} is not a valid color", color),
            },
            count,
        )
    }

    pub fn validate(&self, other: &ColorCount) -> bool {
        other.color != self.color || self.count <= other.count
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::izip;

    #[test]
    fn parse_set() {
        let set_input = "9 green, 4 blue, 3 red";
        let set = Set::new(set_input);
        assert_eq!(set.colors.len(), 3);
        assert_eq!(
            set.colors,
            vec![
                ColorCount::new(Red, 3),
                ColorCount::new(Green, 9),
                ColorCount::new(Blue, 4),
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
            min_set: Set {
                colors: vec![
                    ColorCount::new(Red, 1),
                    ColorCount::new(Green, 2),
                    ColorCount::new(Blue, 4),
                ],
            },
        };

        expected_game.add_set(Set::new("1 green, 4 blue"));
        expected_game.add_set(Set::new("1 blue, 2 green, 1 red"));
        expected_game.add_set(Set::new("1 red, 1 green, 2 blue"));
        expected_game.add_set(Set::new("1 green, 1 red"));
        expected_game.add_set(Set::new("1 green"));
        expected_game.add_set(Set::new("1 green, 1 blue, 1 red"));

        assert_eq!(game.sets.len(), 6);
        for (set, expected_set) in game.sets.iter().zip(expected_game.sets.iter()) {
            assert_eq!(set, expected_set);
        }
        assert_eq!(expected_game.min_set, game.min_set);
    }

    #[test]
    fn validate_game() {
        let given_set = Set::new("12 red, 13 green, 14 blue");

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

        let games: Vec<Game> = input.lines().map(Game::new).collect();

        let mut output = 0;
        games
            .iter()
            .filter(|game| game.validate(&given_set))
            .for_each(|game| {
                output += game.id;
            });
        assert_eq!(expected_output, output);
    }

    #[test]
    fn min_sets() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let min_sets: Vec<Set> = vec![
            "4 red, 2 green, 6 blue",
            "1 red, 3 green, 4 blue",
            "20 red, 13 green, 6 blue",
            "14 red, 3 green, 15 blue",
            "6 red, 3 green, 2 blue",
        ]
        .into_iter()
        .map(Set::new)
        .collect();

        let powers = vec![48, 12, 1560, 630, 36];
        let expected_total_power = 2286;
        let mut total_power = 0;

        let games: Vec<Game> = input.lines().map(Game::new).collect();
        izip!(games, min_sets, powers).for_each(|(game, min_set, power)| {
            // println!("{}: {:?} should be {:?}", game.id, game.min_set, min_set);
            assert_eq!(game.min_set, min_set);

            let calc_power = game.min_set.get_power();
            // println!("{}: {} should be {}", game.id, calc_power, power);
            assert_eq!(calc_power, power);

            total_power += calc_power;
        });

        assert_eq!(expected_total_power, total_power);
    }
}

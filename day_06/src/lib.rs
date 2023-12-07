use derive_new::new;
use itertools::Itertools;
use regex::RegexBuilder;

#[derive(Debug, new)]
pub struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    pub fn get_winning_range(&self) -> (u32, u32) {
        let quadratic_inner = f64::sqrt(self.time * self.time - 4.0 * self.distance);
        (
            f64::ceil((-self.time + quadratic_inner) / -2.0 + 0.00001) as u32,
            f64::floor((-self.time - quadratic_inner) / -2.0 - 0.00001) as u32,
        )
    }

    pub fn get_win_count(&self) -> u32 {
        let range = self.get_winning_range();
        println!("Winning range: {:?}", range);
        let winning_count = range.1 + 1 - range.0;
        println!("Winning count: {}", winning_count);
        winning_count
    }
}

#[derive(Debug)]
pub struct RaceSet {
    races: Vec<Race>,
}

impl RaceSet {
    pub fn new(_input_text: &str) -> Self {
        let re =
            RegexBuilder::new(r"Time:\s*(?<Time>(\d+\s*)+)[\s]*Distance:\s*(?<Distance>(\d+\s*)+)")
                .multi_line(true)
                .build()
                .unwrap();

        let captures = re.captures(_input_text).unwrap();
        let races = captures
            .name("Time")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .zip(
                captures
                    .name("Distance")
                    .unwrap()
                    .as_str()
                    .split_ascii_whitespace(),
            )
            .map(|(time, distance)| Race::new(time.parse().unwrap(), distance.parse().unwrap()))
            .collect();
        Self { races }
    }

    pub fn new_part_2(_input_text: &str) -> Self {
        let re =
            RegexBuilder::new(r"Time:\s*(?<Time>(\d+\s*)+)[\s]*Distance:\s*(?<Distance>(\d+\s*)+)")
                .multi_line(true)
                .build()
                .unwrap();

        let captures = re.captures(_input_text).unwrap();
        let time = captures
            .name("Time")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .join("")
            .parse::<f64>()
            .unwrap();
        let distance = captures
            .name("Distance")
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .join("")
            .parse::<f64>()
            .unwrap();
        Self {
            races: vec![Race::new(time, distance)],
        }
    }

    pub fn get_output(&self) -> u32 {
        self.races.iter().map(|race| race.get_win_count()).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "Time:      7  15   30
        Distance:  9  40  200";
        let expected = 288;

        let races = RaceSet::new(input_text);
        let actual = races.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let input_text = "Time:      7  15   30
        Distance:  9  40  200";
        let expected = 71503;

        let races = RaceSet::new_part_2(input_text);
        let actual = races.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

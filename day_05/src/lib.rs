use std::time::Instant;

use itertools::Itertools;
use regex::{Captures, RegexBuilder};

#[derive(Debug)]
pub struct Range {
    dest_start: u32,
    source_start: u32,
    length: u32,
}

impl Range {
    pub fn new(input: &str) -> Option<Self> {
        Range::parse(input.split_ascii_whitespace().collect_tuple())
    }

    fn parse(input: Option<(&str, &str, &str)>) -> Option<Self> {
        match input {
            Some((a, b, c)) => Some(Self {
                dest_start: a.parse().ok()?,
                source_start: b.parse().ok()?,
                length: c.parse().ok()?,
            }),
            None => None,
        }
    }

    /// Returns the destination index of the given source index if it is in the range, otherwise None
    ///
    /// #Examples
    ///
    /// ```
    /// use day_05::Range;
    /// let range = Range::new("50 98 2").unwrap();
    /// assert_eq!(range.get_dest_index(97), None);
    /// assert_eq!(range.get_dest_index(98), Some(50));
    /// assert_eq!(range.get_dest_index(99), Some(51));
    /// assert_eq!(range.get_dest_index(100), None);
    /// ```
    pub fn get_dest_index(&self, source_index: u32) -> Option<u32> {
        match self.is_in_range(source_index) {
            true => Some(source_index - self.source_start + self.dest_start),
            false => None,
        }
    }

    pub fn is_in_range(&self, source_index: u32) -> bool {
        source_index >= self.source_start && source_index < self.source_start + self.length
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    seed2soil: Vec<Range>,
    soil2fertilizer: Vec<Range>,
    fertilizer2water: Vec<Range>,
    water2light: Vec<Range>,
    light2temp: Vec<Range>,
    temp2humidity: Vec<Range>,
    hum2location: Vec<Range>,
}

#[allow(unused_variables)]
impl Almanac {
    pub fn new(input: &str) -> Self {
        let almanac_regex = RegexBuilder::new(
            r"seeds: (?<seeds>[ \t\d]+)\s*seed-to-soil map:(?<seed2soil>.*)soil-to-fertilizer map:(?<soil2fertilizer>.*)fertilizer-to-water map:(?<fertilizer2water>.*)water-to-light map:(?<water2light>.*)light-to-temperature map:(?<light2temp>.*)temperature-to-humidity map:(?<temp2humidity>.*)humidity-to-location map:(?<hum2location>.*)",
        )
        .dot_matches_new_line(true)
        .build()
        .unwrap();

        let captures = almanac_regex.captures(input);
        let captures = match captures {
            None => panic!("Failed to parse almanac"),
            Some(captures) => captures,
        };

        let seeds: Vec<u32> = Almanac::parse_seeds(&captures);
        let seed2soil: Vec<Range> = Almanac::parse(&captures, "seed2soil");
        let soil2fertilizer: Vec<Range> = Almanac::parse(&captures, "soil2fertilizer");
        let fertilizer2water: Vec<Range> = Almanac::parse(&captures, "fertilizer2water");
        let water2light: Vec<Range> = Almanac::parse(&captures, "water2light");
        let light2temp: Vec<Range> = Almanac::parse(&captures, "light2temp");
        let temp2humidity: Vec<Range> = Almanac::parse(&captures, "temp2humidity");
        let hum2location: Vec<Range> = Almanac::parse(&captures, "hum2location");

        Self {
            seeds,
            seed2soil,
            soil2fertilizer,
            fertilizer2water,
            water2light,
            light2temp,
            temp2humidity,
            hum2location,
        }
    }

    pub fn seed_ranges_to_soil(&mut self) -> u32 {
        let start_time = Instant::now();
        let mut lowest = u32::MAX;
        let max = self.seeds.len() / 2;
        for i in 0..max {
            let start = self.seeds[i * 2];
            let length = self.seeds[i * 2 + 1];
            // println!("{} -> {}", start, length);
            println!(
                "Starting {} of {} | {} -> {}",
                i,
                max,
                start,
                start + length
            );
            for j in 0..length {
                if length > 500 && (j % (length / 500) == 0) {
                    println!(
                        "Set #{}: {:.1}% ({} of {}) {:?} secs",
                        i + 1,
                        (j as f64 / length as f64 * 100.0),
                        j,
                        length,
                        Instant::now().duration_since(start_time)
                    );
                }
                let val = self.apply_maps(start + j);
                lowest = std::cmp::min(val, lowest);
            }
            println!("Completed {} of {}", i + 1, max);
        }
        lowest
    }

    pub fn seeds_to_soil(&mut self) -> Vec<u32> {
        let val = &self
            .seeds
            .clone() //Need to clone in order to mutate
            .iter()
            .map(|s| self.apply_maps(*s))
            .collect_vec();
        val.clone()
    }

    fn apply_maps(&mut self, seed: u32) -> u32 {
        let output = Almanac::apply_map(seed, &self.seed2soil);
        let output = Almanac::apply_map(output, &self.soil2fertilizer);
        let output = Almanac::apply_map(output, &self.fertilizer2water);
        let output = Almanac::apply_map(output, &self.water2light);
        let output = Almanac::apply_map(output, &self.light2temp);
        let output = Almanac::apply_map(output, &self.temp2humidity);
        Almanac::apply_map(output, &self.hum2location)
    }

    /// Applies the almanac to the given input
    ///
    /// # Examples
    ///
    /// ```
    /// # use day_05::Range;
    /// # use day_05::Almanac;
    /// use std::collections::HashMap;
    /// let ranges = vec![Range::new("50 98 2").unwrap()];
    /// assert_eq!(Almanac::apply_map(97, &ranges), 97);
    /// assert_eq!(Almanac::apply_map(98, &ranges), 50);
    /// assert_eq!(Almanac::apply_map(99, &ranges), 51);
    /// assert_eq!(Almanac::apply_map(100, &ranges), 100);
    /// ```
    pub fn apply_map(input: u32, ranges: &[Range]) -> u32 {
        ranges
            .iter()
            .find_map(|x| x.get_dest_index(input))
            .unwrap_or(input)
    }

    pub fn parse_seeds(captures: &Captures<'_>) -> Vec<u32> {
        match captures.name("seeds") {
            Some(some) => some,
            None => {
                println!("No seeds");
                return vec![];
            }
        }
        .as_str()
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>()
    }

    pub fn parse(captures: &Captures<'_>, name: &str) -> Vec<Range> {
        let lines = match captures.name(name) {
            Some(some) => some,
            None => return vec![],
        }
        .as_str();

        lines.lines().filter_map(Range::new).collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        let expected = 35;

        let mut almanac = Almanac::new(input_text);
        for (a, b) in almanac.seeds.iter().zip([79, 14, 55, 13].iter()) {
            assert_eq!(a, b);
        }

        let actual = *almanac.seeds_to_soil().iter().min().unwrap();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let input_text = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        let expected = 46;

        let mut almanac = Almanac::new(input_text);
        let actual = almanac.seed_ranges_to_soil();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

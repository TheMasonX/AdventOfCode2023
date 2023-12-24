use itertools::Itertools;
use nom::ParseTo;
// use regex::{Captures, RegexBuilder};

#[derive(Debug)]
pub struct History {
    sequences: Vec<Vec<i32>>,
}

impl History {
    pub fn new(input_text: &str) -> Self {
        let sequence = input_text
            .split_ascii_whitespace()
            .filter_map(|f| f.parse_to())
            .collect_vec();

        let mut last = sequence.clone();
        let mut sequences = vec![sequence.clone()];
        while let Some(next) = Self::get_next(&last) {
            last = next.clone();
            sequences.push(next);
        }
        Self { sequences }
    }

    pub fn get_next(last_sequence: &Vec<i32>) -> Option<Vec<i32>> {
        if last_sequence.iter().filter(|i| **i != 0).count() == 0 {
            return None;
        }
        let mut next: Vec<i32> = vec![];
        for i in 1..last_sequence.len() {
            let previous = last_sequence[i - 1];
            let current = last_sequence[i];
            next.push(current - previous);
        }
        Some(next)
    }

    pub fn get_output(&self) -> i32 {
        let size = self.sequences.len();
        let mut last_last = 0;
        for i in 0..size - 1 {
            let this_sequence = &self.sequences[size - i - 1];
            last_last += *this_sequence.last().unwrap();
            // println!("{:?}, last last: {}", this_sequence, last_last);
        }

        let result = self.sequences.first().unwrap().last().unwrap() + last_last;
        // println!("Result: {}", result);
        result
    }
}

#[derive(Debug)]
pub struct StructA {
    histories: Vec<History>,
}

impl StructA {
    pub fn new(input_text: &str) -> Self {
        let histories = input_text.lines().map(History::new).collect();
        Self { histories }
    }

    pub fn get_output(&self) -> i32 {
        self.histories.iter().map(|h| h.get_output()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let expected = [18, 28, 68];
        let expected_total = expected.into_iter().sum::<i32>();

        let struct_a = StructA::new(input_text);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        struct_a
            .histories
            .iter()
            .zip(expected)
            .for_each(|(actual, expected)| {
                let history_output = actual.get_output();
                println!("History {:?} result of {}", actual, history_output);
                assert_eq!(expected, history_output);
            });
        assert_eq!(expected_total, actual);
    }
}

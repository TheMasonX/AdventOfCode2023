use itertools::{multizip as zip, Itertools};
use regex::RegexBuilder;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Element {
    Start(String),
    Middle(String),
    End(String),
}

impl Element {
    pub fn new(input: &str) -> Option<Self> {
        match input {
            a if a.ends_with('A') => Some(Self::Start(input.to_string())),
            z if z.ends_with('Z') => Some(Self::End(input.to_string())),
            _ => Some(Self::Middle(input.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum Step {
    L,
    R,
}

#[derive(Debug)]
pub struct StructA {
    steps: Vec<Step>,
    maps: HashMap<Element, (Element, Element)>,
}

impl StructA {
    pub fn new(input_text: &str) -> Self {
        let re = RegexBuilder::new(r"(?<In>\w{3})\s= \((?<L>\w{3}), (?<R>\w{3})\)")
            .multi_line(true)
            .build()
            .unwrap();
        let steps = input_text
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Step::L),
                'R' => Some(Step::R),
                _ => None,
            })
            .collect_vec();

        let maps = re
            .captures_iter(input_text)
            .map(|captures| {
                zip((
                    captures.name("In").iter(),
                    captures.name("L").iter(),
                    captures.name("R").iter(),
                ))
                .map(|(input, left, right)| {
                    (
                        Element::new(input.as_str()).unwrap(),
                        (
                            Element::new(left.as_str()).unwrap(),
                            Element::new(right.as_str()).unwrap(),
                        ),
                    )
                })
                .exactly_one()
                .unwrap()
            })
            .collect_vec();
        let maps = HashMap::from_iter(maps);
        Self { steps, maps }
    }

    pub fn get_output(&self) -> u64 {
        let start_time = Instant::now();
        let mut last_step_time = start_time;
        let mut current = self
            .maps
            .iter()
            .filter_map(|f| match f.0 {
                Element::Start(_) => Some(f.0),
                _ => None,
            })
            .collect_vec();

        const PRINT_STEPS: u64 = 30000000;
        for step in 0..u64::MAX {
            if step % PRINT_STEPS == 0 {
                println!(
                    "{} Steps in {:?} | {:?} per step",
                    step,
                    Instant::now().duration_since(start_time),
                    Instant::now()
                        .duration_since(last_step_time)
                        .div_f64(PRINT_STEPS as f64)
                );
                last_step_time = Instant::now();
            }
            match self.take_step(&current, step as usize) {
                Some(new) => current = new,
                None => return step,
            }
        }
        u64::MAX
    }

    fn take_step(&self, input: &[&Element], step_count: usize) -> Option<Vec<&Element>> {
        input.iter().find(|x| !matches!(x, Element::End(_)))?;

        let step = &self.steps[step_count % self.steps.len()];
        Some(
            input
                .iter()
                .map(|input| match step {
                    Step::L => &self.maps[input].0,
                    Step::R => &self.maps[input].1,
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let expected = 6;

        let struct_a = StructA::new(input_text);
        println!("{:?}", struct_a);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

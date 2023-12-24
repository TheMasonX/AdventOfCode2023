use itertools::{multizip as zip, Itertools};
use num::integer;
use regex::RegexBuilder;
use std::collections::HashMap;

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

    pub fn get_steps(&self, parent: &StructA) -> u64 {
        let mut first: u64 = 0;
        let mut next = self;
        for step in 0..u64::MAX {
            next = parent.take_step(next, step as usize);
            if let Element::End(_) = next {
                match first {
                    0 => first = step,
                    _ => return step - first,
                }
            }
        }
        u64::MAX
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

    pub fn take_step(&self, input: &Element, step_count: usize) -> &Element {
        let step = &self.steps[step_count % self.steps.len()];
        match step {
            Step::L => &self.maps[input].0,
            Step::R => &self.maps[input].1,
        }
    }

    pub fn get_start(&self) -> Vec<&Element> {
        self.maps
            .iter()
            .filter_map(|f| match f.0 {
                Element::Start(_) => Some(f.0),
                _ => None,
            })
            .collect_vec()
    }

    pub fn get_output(&self) -> u64 {
        let starts = self.get_start();
        let step_counts = starts
            .into_iter()
            .map(|element| element.get_steps(self))
            .collect_vec();
        let mut lcm = 1;
        for step_count in step_counts {
            lcm = integer::lcm(lcm, step_count);
        }
        lcm
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
        // println!("{:?}", struct_a);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

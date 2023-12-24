use itertools::{multizip as zip, Itertools};
use regex::RegexBuilder;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Element {
    Start,
    Middle(String),
    End,
}

impl Element {
    pub fn new(input: &str) -> Option<Self> {
        match input {
            "AAA" => Some(Self::Start),
            "ZZZ" => Some(Self::End),
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

    fn take_step(&self, input: &Element, step_count: usize) -> Option<&Element> {
        if input == &Element::End {
            return None;
        }
        let step = &self.steps[step_count % self.steps.len()];
        match step {
            Step::L => Some(&self.maps[input].0),
            Step::R => Some(&self.maps[input].1),
        }
    }
    pub fn get_output(&self) -> i32 {
        let step_count = 100000000;
        let mut current = &Element::Start;
        for step in 0..100000000 {
            match self.take_step(current, step as usize) {
                Some(new) => current = new,
                None => return step,
            }
        }
        step_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        let expected = 2;

        let struct_a = StructA::new(input_text);
        println!("{:?}", struct_a);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let input_text = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        let expected = 6;

        let struct_a = StructA::new(input_text);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

use std::time::Instant;

use itertools::Itertools;
use regex::{Captures, RegexBuilder};

#[derive(Debug)]
pub struct StructA {}

impl StructA {
    pub fn new(_input_text: String) -> Self {
        Self {}
    }

    pub fn get_output(&self) -> i32 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "";
        let expected = 0;

        let mut struct_a = StructA::new(input_text);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

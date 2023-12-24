use std::time::Instant;

use itertools::Itertools;
use Direction::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    TBD,
}

pub struct Pipe {
    endA: Direction,
    endB: Direction,
    is_start: bool = false,
}

impl Pipe {
    pub fn new(character: char) -> Option<Self> {
        match character {
            '|' => Some(Self::vertical()),
            '-' => Some(Self::horizontal()),
            'L' => Some(Self {
                endA: Up,
                endB: Right,
            }),
            'J' => Some(Self {
                endA: Up,
                endB: Left,
            }),
            '7' => Some(Self {
                endA: South,
                endB: Left,
            }),
            'F' => Some(Self {
                endA: South,
                endB: Right,
            }),
            'S' => Some(Self {
                endA: TBD,
                endB: TBD,
                is_start: true,
            })
            _ => None,
        }
    }

    pub fn vertical() -> Self {
        Self {
            endA: Up,
            endB: Down,
        }
    }

    pub fn horizontal() -> Self {
        Self {
            endA: Left,
            endB: Right,
        }
    }
}

#[derive(Debug)]
pub struct StructA {}

impl StructA {
    pub fn new(input_text: &str) -> Self {
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
        let input_text = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        let expected = 0;

        let struct_a = StructA::new(input_text);
        let actual = struct_a.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }
}

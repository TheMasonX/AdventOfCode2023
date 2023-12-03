use array2d::Array2D;
use derive_new::new;
// use itertools::Itertools;
use std::cmp::{max, min};
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();
    let schematic = Schematic::new(&input_text);
    println!(
        "Input is {} by {}",
        schematic.grid.num_columns(),
        schematic.grid.num_rows()
    );
    let first_outcome = schematic.get_parts_total();
    println!("First Solution {}", first_outcome);
    let second_outcome = schematic.get_gears_total();
    println!("Second Solution {}", second_outcome);
}

#[derive(Debug, Clone)]
pub struct Schematic {
    grid: Array2D<char>,
    parts: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn new(input_text: &str) -> Self {
        let mut parts = Vec::new();
        let mut symbols = Vec::new();
        let mut grid_y = Vec::new();

        let mut part_num_chars = Vec::new();

        let lines = input_text.lines();
        for (y, line) in lines.enumerate() {
            // println!("Starting line {}", line);
            let chars: Vec<char> = line.trim().chars().collect();
            for (x, c) in chars.iter().enumerate() {
                let pos = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };

                match c {
                    c if c.is_numeric() => {
                        part_num_chars.push(*c);
                        if x < chars.len() - 1 {
                            continue; // Still may be more number chars
                        }
                    }
                    '.' => {}
                    _ => symbols.push(Symbol { symbol: *c, pos }),
                }

                if !part_num_chars.is_empty() {
                    let num_string = part_num_chars.iter().collect::<String>();
                    part_num_chars.clear();
                    if let Ok(num) = num_string.parse::<i32>() {
                        // println!("Parsed int {}", num);
                        let len = num_string.len() as i32;
                        parts.push(PartNumber {
                            number: num,
                            pos: Vec2 {
                                x: x as i32 - len,
                                y: y as i32,
                            },
                            length: len,
                            near_symbol: false,
                        })
                    }
                }
            }
            grid_y.push(chars);
        }

        let grid = Array2D::from_rows(&grid_y).unwrap();
        let mut schematic = Schematic {
            grid,
            parts: Vec::new(),
            symbols,
        };
        schematic.search_for_symbols(&mut parts);
        schematic
    }

    pub fn get_parts_total(&self) -> i32 {
        self.parts
            .iter()
            .filter_map(|p| match p.near_symbol {
                true => Some(p.number),
                _ => None,
            })
            .sum()
    }

    pub fn get_gears(&self) -> Vec<Gear> {
        let mut gears = Vec::new();

        let potential_gears: Vec<&Symbol> =
            self.symbols.iter().filter(|s| s.symbol == '*').collect();

        for g in potential_gears.iter() {
            let nearby: Vec<&PartNumber> = self
                .parts
                .iter()
                .filter(|p| self.is_within_bounds(g.pos, p.pos, p.length))
                .collect();

            let count = nearby.len();
            if count == 2 {
                gears.push(Gear {
                    pos: g.pos,
                    part_a: *nearby[0],
                    part_b: *nearby[1],
                })
            }
        }
        gears
    }

    pub fn get_gears_total(&self) -> i32 {
        self.get_gears()
            .iter()
            .map(|g| g.part_a.number * g.part_b.number)
            .sum()
    }

    fn is_within_bounds(&self, pos_other: Vec2, pos_self: Vec2, length: i32) -> bool {
        let left = max(pos_self.x - 1, 0);
        let right = min(pos_self.x + length, self.grid.num_columns() as i32);
        let top = max(pos_self.y - 1, 0);
        let bottom = min(pos_self.y + 1, self.grid.num_rows() as i32);

        pos_other.x >= left && pos_other.x <= right && pos_other.y >= top && pos_other.y <= bottom
    }

    pub fn search_for_symbols(&mut self, parts: &mut [PartNumber]) {
        for part in parts.iter_mut() {
            for s in self.symbols.iter() {
                // println!("Symbol {:?} Part{:?}", s.pos, part.pos);
                // println!(
                //     "left {} | right {} | top {} | bottom {}",
                //     left, right, top, bottom
                // );
                if self.is_within_bounds(s.pos, part.pos, part.length) {
                    part.near_symbol = true;
                    // println!("{} was near symbol {}", part.number, s.symbol);
                    break;
                }
            }

            let val = self
                .symbols
                .iter()
                .filter(|s| self.is_within_bounds(s.pos, part.pos, part.length))
                .map(|s| s.symbol)
                .collect::<Vec<char>>();

            part.near_symbol = !val.is_empty();
        }
        self.parts = parts.to_vec();
    }

    pub fn print(&self) {
        for y in 0..self.grid.num_rows() {
            for x in 0..self.grid.num_columns() {
                print!("{}", self.grid[(y, x)]);
            }
            println!();
        }

        for (i, part) in self.parts.iter().enumerate() {
            println!("Part Number #{i}: {:?}", part);
        }

        for (i, symbol) in self.symbols.iter().enumerate() {
            println!("Symbol #{i}: {:?}", symbol);
        }
    }
}

#[derive(Debug, new, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct PartNumber {
    number: i32,
    pos: Vec2,
    length: i32,
    near_symbol: bool,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    symbol: char,
    pos: Vec2,
}

#[derive(Debug, Clone, new)]
pub struct Gear {
    pos: Vec2,
    part_a: PartNumber,
    part_b: PartNumber,
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tmx_utils::string_ext;

    #[test]
    fn test_a() {
        let input_text = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let expected_parts = [
            (467, true),
            (114, false),
            (35, true),
            (633, true),
            (617, true),
            (58, false),
            (592, true),
            (755, true),
            (664, true),
            (598, true),
        ];

        let schematic = Schematic::new(input_text);
        schematic.print();

        for (i, part) in schematic.parts.iter().enumerate() {
            println!(
                "Part Number #{} is near {} | Expected {}",
                part.number, part.near_symbol, expected_parts[i].1
            );
            assert_eq!(expected_parts[i].0, part.number);
            assert_eq!(expected_parts[i].1, part.near_symbol);
        }

        let outcome = schematic.get_parts_total();

        let expected_outcome: i32 = expected_parts
            .iter()
            .map(|f| match f.1 {
                true => f.0,
                _ => 0,
            })
            .sum();
        assert_eq!(expected_outcome, outcome);
    }

    #[test]
    fn test_b() {
        let input_text = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let expected_gears = [(Vec2::new(3, 1), 16345), (Vec2::new(5, 8), 451490)];

        let schematic = Schematic::new(input_text);
        schematic.print();

        let gears = schematic.get_gears();
        println!("Found {} gears", gears.len());

        for (gear, expected) in gears.iter().zip(expected_gears.iter()) {
            println!(
                "Gear at {:?} has value {} | Expected {}",
                gear.pos,
                gear.part_a.number * gear.part_b.number,
                expected.1
            );
            assert_eq!(gear.pos, expected.0);
            assert_eq!(gear.part_a.number * gear.part_b.number, expected.1);
        }

        let outcome = schematic.get_gears_total();
        let expected_outcome: i32 = expected_gears.iter().map(|f| f.1).sum();
        assert_eq!(expected_outcome, outcome);
    }
}

use array2d::Array2D;
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
    let outcome = schematic.get_total();
    println!("First Solution {}", outcome);
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

    pub fn get_total(&self) -> i32 {
        self.parts
            .iter()
            .filter_map(|p| match p.near_symbol {
                true => Some(p.number),
                _ => None,
            })
            .sum()
    }

    pub fn search_for_symbols(&mut self, parts: &mut [PartNumber]) {
        for part in parts.iter_mut() {
            let pos = part.pos;
            let left = max(pos.x - 1, 0);
            let right = min(pos.x + part.length, self.grid.num_columns() as i32);
            let top = max(pos.y - 1, 0);
            let bottom = min(pos.y + 1, self.grid.num_rows() as i32);

            for s in self.symbols.iter() {
                // println!("Symbol {:?} Part{:?}", s.pos, part.pos);
                // println!(
                //     "left {} | right {} | top {} | bottom {}",
                //     left, right, top, bottom
                // );
                if s.pos.x >= left && s.pos.x <= right && s.pos.y >= top && s.pos.y <= bottom {
                    part.near_symbol = true;
                    // println!("{} was near symbol {}", part.number, s.symbol);
                    break;
                }
            }

            let val = self
                .symbols
                .iter()
                .filter(|s| {
                    s.pos.x >= left && s.pos.x <= right && s.pos.y >= top && s.pos.y <= bottom
                })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

        let outcome = schematic.get_total();

        let expected_outcome: i32 = expected_parts
            .iter()
            .map(|f| match f.1 {
                true => f.0,
                _ => 0,
            })
            .sum();
        assert_eq!(expected_outcome, outcome);
    }
}

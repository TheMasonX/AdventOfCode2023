use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, time};
use tmx_utils::string_ext;

#[allow(dead_code)]
pub fn new_main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();
    let card_manager = CardManager::new(&input_text);
    let _total = card_manager.waterfall_rounds();
    // println!("Second Total: {}", card_manager.waterfall_rounds());
}

#[allow(dead_code)]
pub fn old_main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();
    let mut card_manager = CardManager::new(&input_text);
    let _total = card_manager.play_all_rounds();
    // let total = card_manager
    //     .cards_lut
    //     .values()
    //     .map(|x| x.score)
    //     .sum::<i32>();
    // println!("First Total: {}", total);
    // println!("Second Total: {}", card_manager.play_all_rounds());
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardManager {
    cards_vec: Vec<Card>,
    cards_lut: HashMap<i32, Card>,
    remaining_cards: Vec<i32>,
}

impl CardManager {
    pub fn new(input_text: &str) -> Self {
        let cards_vec = input_text.lines().map(Card::new).collect_vec();
        let remaining_cards = cards_vec.iter().map(|x| x.id).collect_vec();
        let mut cards = HashMap::new();
        for card in cards_vec.clone() {
            cards.insert(card.id, card);
        }
        Self {
            cards_vec,
            cards_lut: cards,
            remaining_cards,
        }
    }

    pub fn waterfall_rounds(&self) -> i32 {
        let start = time::Instant::now();
        let mut copies: HashMap<i32, i32> =
            HashMap::from_iter(self.cards_vec.iter().map(|c| (c.id, 1)));
        for card in &self.cards_vec {
            if card.win_count == 0 {
                continue;
            }
            let current_card_copies = *copies.get(&card.id).unwrap();
            for other_index in card.id + 1..=card.id + card.win_count {
                // *copies.entry(other_index).or_insert(1) += current_card_copies;
                copies.insert(
                    other_index,
                    copies.get(&other_index).unwrap() + current_card_copies,
                );
            }
        }

        let sum = copies.values().sum();
        let end = time::Instant::now();
        println!("Waterfall took {}us", end.duration_since(start).as_micros());
        sum
    }

    pub fn play_round(&mut self) {
        let mut new_cards = Vec::new();
        for card_id in &self.remaining_cards {
            let card = self.cards_lut.get(card_id).unwrap();
            match card.win_count {
                0 => {
                    continue;
                }
                num => {
                    for n in card.id + 1..card.id + 1 + num {
                        if let Some(card) = self.cards_lut.get(&n) {
                            new_cards.push(card.id);
                        }
                    }
                }
            }
        }
        self.remaining_cards = new_cards;
    }

    pub fn play_all_rounds(&mut self) -> i32 {
        let start = time::Instant::now();
        let mut total_cards = vec![];
        let mut loop_count = 0;
        while loop_count < 100 && !self.remaining_cards.is_empty() {
            loop_count += 1;
            total_cards.append(&mut self.remaining_cards.clone());
            // println!(
            //     "Round {loop_count}: {} total cards, {} cards remaining",
            //     total_cards.len(),
            //     self.remaining_cards.len()
            // );
            self.play_round();
        }
        let sum = total_cards.len() as i32;
        let end = time::Instant::now();
        println!(
            "Rounds {loop_count}: took {}ms",
            end.duration_since(start).as_millis()
        );
        sum
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    id: i32,
    winners: Vec<i32>,
    numbers: Vec<i32>,
    win_count: i32,
    score: i32,
}

impl Card {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"Card *(?<CardID>\d*): *(?<Winners>.*) \| *(?<Numbers>.*)").unwrap();
        let captures = re.captures(input).unwrap();
        let id = captures
            .name("CardID")
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap();

        let winners = Card::parse_nums(&captures, "Winners");
        let numbers = Card::parse_nums(&captures, "Numbers");

        let win_count = numbers.iter().filter(|x| winners.contains(x)).count() as i32;
        let score = match win_count {
            0 => 0,
            1 => 1,
            _ => 2 << (win_count - 2),
        };

        Self {
            id,
            winners,
            numbers,
            win_count,
            score,
        }
    }

    fn parse_nums(captures: &regex::Captures<'_>, name: &str) -> Vec<i32> {
        captures
            .name(name)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input_text = "Card   1: 33 13 28 76 16 91 52 41 38 64 | 52 10  7 61 12 70 84 38 16 40  5 49 33 11 31 43 71 28 72 23 98 47 14 44 90";

        let expected = [Card {
            id: 1,
            winners: vec![33, 13, 28, 76, 16, 91, 52, 41, 38, 64],
            numbers: vec![
                52, 10, 7, 61, 12, 70, 84, 38, 16, 40, 5, 49, 33, 11, 31, 43, 71, 28, 72, 23, 98,
                47, 14, 44, 90,
            ],
            win_count: 5,
            score: 16,
        }];

        let mut expected_total = 0;
        let mut actual_total = 0;

        for (actual, expected) in input_text.lines().map(Card::new).zip(expected) {
            expected_total += expected.score;
            actual_total += actual.score;
            assert_eq!(actual, expected);
        }
        assert_eq!(expected_total, actual_total);
    }

    #[test]
    fn test_a() {
        let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected = [
            Card {
                id: 1,
                winners: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                win_count: 4,
                score: 8,
            },
            Card {
                id: 2,
                winners: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                win_count: 2,
                score: 2,
            },
            Card {
                id: 3,
                winners: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                win_count: 2,
                score: 2,
            },
            Card {
                id: 4,
                winners: vec![41, 92, 73, 84, 69],
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                win_count: 1,
                score: 1,
            },
            Card {
                id: 5,
                winners: vec![87, 83, 26, 28, 32],
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                win_count: 0,
                score: 0,
            },
            Card {
                id: 6,
                winners: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                win_count: 0,
                score: 0,
            },
        ];

        let mut expected_total = 0;
        let mut actual_total = 0;

        for (actual, expected) in input_text.lines().map(Card::new).zip(expected) {
            expected_total += expected.score;
            actual_total += actual.score;
            assert_eq!(actual, expected);
        }
        assert_eq!(expected_total, actual_total);
    }

    #[test]
    fn test_b() {
        let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut card_manager = CardManager::new(input_text);
        let expected = 30;

        let mut total_cards = vec![];
        let mut loop_count = 0;
        while loop_count < 100 && !card_manager.remaining_cards.is_empty() {
            loop_count += 1;
            total_cards.append(&mut card_manager.remaining_cards.clone());
            println!(
                "Round {loop_count}: {} total cards, {} cards remaining",
                total_cards.len(),
                card_manager.cards_lut.len()
            );
            card_manager.play_round();
        }

        let card_1_copies = total_cards.iter().filter(|x| **x == 1).count();
        let card_2_copies = total_cards.iter().filter(|x| **x == 2).count();
        let card_3_copies = total_cards.iter().filter(|x| **x == 3).count();
        let card_4_copies = total_cards.iter().filter(|x| **x == 4).count();
        let card_5_copies = total_cards.iter().filter(|x| **x == 5).count();
        let card_6_copies = total_cards.iter().filter(|x| **x == 6).count();
        let total_copies = card_1_copies
            + card_2_copies
            + card_3_copies
            + card_4_copies
            + card_5_copies
            + card_6_copies;

        println!(
            "1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {} | Total: {}",
            card_1_copies,
            card_2_copies,
            card_3_copies,
            card_4_copies,
            card_5_copies,
            card_6_copies,
            total_copies
        );
        assert_eq!(expected, total_copies);
        assert_eq!(expected, total_cards.len());
    }

    #[test]
    fn test_c() {
        let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut card_manager = CardManager::new(input_text);
        let expected = 30;

        assert_eq!(expected, card_manager.play_all_rounds());
    }

    #[test]
    fn test_d() {
        let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let card_manager = CardManager::new(input_text);
        let expected = 30;

        assert_eq!(expected, card_manager.waterfall_rounds());
    }
}

use itertools::Itertools;
use regex::RegexBuilder;
use std::{collections::HashMap, fmt::Debug, ops::Index};

use HandType::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Card {
    Joker,
    Num(i32),
    Queen,
    King,
    Ace,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Num(n) => write!(f, "{}", n),
            Card::Joker => write!(f, "J"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

impl Card {
    pub fn new(c: char) -> Card {
        match c {
            '2'..='9' => Card::Num(c.to_digit(10).unwrap() as i32),
            'T' => Card::Num(10),
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn get_rank(&self) -> i32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

#[derive(Debug, Eq)]
pub struct Hand {
    pub hand_type: HandType,
    cards: Vec<Card>,
    bid: i32,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: i32) -> Self {
        let duplicates = Hand::get_duplicates(&cards);
        let hand_type = Hand::get_type(duplicates);
        Self {
            cards,
            bid,
            hand_type,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let cards = input.chars().map(Card::new).collect_vec();
        Self::new(cards, 0)
    }

    /// Get the HandType of the hand
    ///
    /// # Examples
    ///
    /// ```
    /// use day_07::part_two::Hand;
    /// use day_07::part_two::HandType::*;
    ///
    /// let hand = Hand::from_string("JJJJT");
    /// assert_eq!(hand.hand_type, FiveOfAKind);
    /// let hand = Hand::from_string("JJJJJ");
    /// assert_eq!(hand.hand_type, FiveOfAKind);
    /// let hand = Hand::from_string("JJJA2");
    /// assert_eq!(hand.hand_type, FourOfAKind);
    /// ```
    pub fn get_type(duplicates: Vec<(&Card, i32)>) -> HandType {
        let joker_count = Hand::get_joker_count(&duplicates);
        match joker_count {
            0 => return Hand::get_type_no_jokers(&duplicates),
            4 | 5 => return FiveOfAKind,
            _ => (),
        }

        let mut duplicates = duplicates.clone();
        duplicates.retain_mut(|d| d.0 != &Card::Joker);

        // match duplicates.iter_mut().find(|d| d.0 != &Card::Joker) {
        match duplicates.first_mut() {
            Some(card) => {
                println!("FNJ {:?} {}", card.0, card.1);
                card.1 += joker_count;
                println!("FNJ After {:?} {}", card.0, card.1);
            }
            None => return FiveOfAKind,
        };
        Hand::get_type_no_jokers(&duplicates)
    }

    fn get_joker_count(duplicates: &[(&Card, i32)]) -> i32 {
        duplicates
            .iter()
            .filter_map(|d| match d.0 {
                Card::Joker => Some(d.1),
                _ => None,
            })
            .sum()
    }

    fn get_type_no_jokers(duplicates: &[(&Card, i32)]) -> HandType {
        match duplicates[..] {
            [a] if (a.1 == 5) => FiveOfAKind,
            [a, _] if (a.1 == 4) => FourOfAKind,
            [a, b] if (a.1 == 3 && b.1 == 2) => FullHouse,
            [a, ..] if (a.1 == 3) => ThreeOfAKind,
            [a, b, _] if (a.1 == 2 && b.1 == 2) => TwoPair,
            [a, b, _, _] if (a.1 == 2 && b.1 == 1) => OnePair,
            [_, _, _, _, _] => HighCard,
            _ => panic!("Invalid hand"),
        }
    }

    pub fn get_duplicates(cards: &[Card]) -> Vec<(&Card, i32)> {
        let mut map = HashMap::new();
        for card in cards.iter() {
            *map.entry(card).or_insert(0) += 1;
        }

        map.into_iter()
            .map(|(k, v)| (k, v))
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .collect_vec()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.hand_type.cmp(&other.hand_type)

        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            match (card, other_card) {
                (a, b) if (a == b) => continue,
                (a, b) => return a.cmp(b),
            }
        }

        std::cmp::Ordering::Equal
    }
}

#[derive(Debug)]
pub struct CardsManager {
    hands: Vec<Hand>,
}

impl CardsManager {
    pub fn new(input_text: &str) -> Self {
        let re = RegexBuilder::new(r"(?<Hand>[\d\w]{5}) (?<Bid>[\d]+)\s*")
            .multi_line(true)
            .build()
            .unwrap();
        let matches = re.captures_iter(input_text);
        let hands = matches
            .map(|captures| {
                let cards = captures
                    .name("Hand")
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(Card::new)
                    .collect_vec();
                let bid = captures.name("Bid").unwrap().as_str().parse().unwrap();
                Hand::new(cards, bid)
            })
            .collect_vec();

        let hands_debug = hands.iter().map(|h| format!("{:?}", h)).join("\n");
        println!(
            "Got {} Hands: ===========\n{}\n===========",
            hands.len(),
            hands_debug
        );
        Self { hands }
    }

    pub fn get_output(&self) -> i32 {
        self.hands
            .iter()
            .sorted()
            .enumerate()
            .map(|(i, h)| (i + 1) as i32 * h.bid)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let expected = 5905;

        let cards = CardsManager::new(input_text);
        let sorted_hands = cards.hands.iter().sorted().collect_vec();
        let hands_debug = sorted_hands
            .iter()
            .enumerate()
            .map(|(i, h)| format!("#{} {:?}", i + 1, h))
            .join("\n");
        println!("Sorted Hands: ===========\n{}\n===========", hands_debug);

        let actual = cards.get_output();
        println!("Got result of {}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_card_ordering() {
        assert!(Card::new('A') > Card::new('2'));
        assert!(Card::new('A') > Card::new('K'));
        assert!(Card::new('K') > Card::new('Q'));
        assert!(Card::new('Q') > Card::new('T'));
        assert!(Card::new('T') > Card::new('9'));
        assert!(Card::new('5') > Card::new('4'));
        assert!(Card::new('5') == Card::new('5'));
        assert!(Card::new('J') < Card::new('A'));
        assert!(Card::new('J') < Card::new('T'));
        assert!(Card::new('J') < Card::new('2'));
    }

    #[test]
    fn test_hand_type() {
        let input_expected_kind = vec![
            (Hand::from_string("AAAAA"), HandType::FiveOfAKind),
            (Hand::from_string("QQQQA"), HandType::FourOfAKind),
            (Hand::from_string("QQQQJ"), HandType::FiveOfAKind),
            (Hand::from_string("QQQJA"), HandType::FourOfAKind),
            (Hand::from_string("QQQKA"), HandType::ThreeOfAKind),
            (Hand::from_string("QQJAA"), HandType::FullHouse),
            (Hand::from_string("555JJ"), HandType::FiveOfAKind),
            (Hand::from_string("J5J5J"), HandType::FiveOfAKind),
            (Hand::from_string("JJJJJ"), HandType::FiveOfAKind),
            (Hand::from_string("JKKQQ"), HandType::FullHouse),
        ];

        for (input, expected) in input_expected_kind {
            assert_eq!(input.hand_type, expected);
        }
    }
}

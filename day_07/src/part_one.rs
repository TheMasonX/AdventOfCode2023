use itertools::Itertools;
use regex::RegexBuilder;
use std::{collections::HashMap, fmt::Debug};

use HandType::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Card {
    Num(i32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Num(n) => write!(f, "{}", n),
            Card::Jack => write!(f, "J"),
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
            'J' => Card::Jack,
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
    hand_type: HandType,
    cards: Vec<Card>,
    bid: i32,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: i32) -> Self {
        let hand_type = Hand::get_type(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }

    pub fn get_type(cards: &[Card]) -> HandType {
        let mut map = HashMap::new();
        for card in cards.iter() {
            *map.entry(card).or_insert(0) += 1;
        }

        let duplicates = map
            .into_iter()
            .map(|(k, v)| (k, v))
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .collect_vec();

        // println!("Duplicates {:?}", dups);
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
        let expected = 6440;

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
        assert!(Card::new('Q') > Card::new('J'));
        assert!(Card::new('J') > Card::new('T'));
        assert!(Card::new('T') > Card::new('9'));
        assert!(Card::new('5') > Card::new('4'));
        assert!(Card::new('5') == Card::new('5'));
    }

    #[test]
    fn test_hand_type() {
        let hand1 = Hand::new(
            vec![
                Card::new('A'),
                Card::new('A'),
                Card::new('A'),
                Card::new('A'),
                Card::new('A'),
            ],
            0,
        );
        assert_eq!(hand1.hand_type, HandType::FiveOfAKind);
        let hand2 = Hand::new(
            vec![
                Card::new('A'),
                Card::new('Q'),
                Card::new('A'),
                Card::new('A'),
                Card::new('A'),
            ],
            0,
        );
        assert_eq!(hand2.hand_type, HandType::FourOfAKind);
        let hand3 = Hand::new(
            vec![
                Card::new('3'),
                Card::new('Q'),
                Card::new('T'),
                Card::new('3'),
                Card::new('3'),
            ],
            0,
        );
        assert_eq!(hand3.hand_type, HandType::ThreeOfAKind);
        let hand4 = Hand::new(
            vec![
                Card::new('3'),
                Card::new('T'),
                Card::new('T'),
                Card::new('3'),
                Card::new('3'),
            ],
            0,
        );
        assert_eq!(hand4.hand_type, HandType::FullHouse);
    }
}

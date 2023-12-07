use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::solutions::common::split_into_lines;

#[derive(Eq)]
struct CardScore(Vec<u32>);

impl Debug for CardScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

// impl sort for CardScore
impl Ord for CardScore {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Less,
                Ordering::Less => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for CardScore {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for CardScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum PairKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePairKindError {
    data: String,
    when: u32,
}

// Implement str.as_pair_kind()
impl std::str::FromStr for PairKind {
    type Err = ParsePairKindError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse string and see how many duplicates there are
        let as_chars: Vec<char> = s.chars().collect();
        let undupe: HashSet<char> = HashSet::from_iter(as_chars.iter().cloned());

        match undupe.len() {
            1 => Ok(Self::FiveOfAKind),
            2 => {
                // Check for four of a kind and full house
                // Four of a kind: 4, 1
                // Full house: 3, 2
                let mut maps = HashMap::new();
                for char in as_chars {
                    let count = maps.entry(char).or_insert(0);
                    *count += 1;
                }

                let mut counts = maps.values().collect::<Vec<&usize>>();
                counts.sort();
                // get last
                let last = *counts.last().unwrap();
                match *last {
                    4 => Ok(Self::FourOfAKind),
                    3 => Ok(Self::FullHouse),
                    _ => Err(ParsePairKindError {
                        data: s.to_string(),
                        when: 2,
                    }),
                }
            }
            3 => {
                // Check for three of a kind and two pair
                // Three of a kind: 3, 1, 1
                // Two pair: 2, 2, 1
                let mut maps = HashMap::new();
                for char in as_chars {
                    let count = maps.entry(char).or_insert(0);
                    *count += 1;
                }

                let mut counts = maps.values().collect::<Vec<&usize>>();
                counts.sort();
                // get last
                let last = *counts.last().unwrap();
                match *last {
                    3 => Ok(Self::ThreeOfAKind),
                    2 => Ok(Self::TwoPair),
                    _ => Err(ParsePairKindError {
                        data: s.to_string(),
                        when: 3,
                    }),
                }
            }
            4 => Ok(Self::OnePair),
            5 => Ok(Self::HighCard),
            _ => Err(ParsePairKindError {
                data: s.to_string(),
                when: 0,
            }),
        }
    }
}

#[derive(Eq)]
pub struct Card {
    score: CardScore,
    kind: PairKind,
    bets: usize,
}

impl Card {
    fn new(score: CardScore, kind: PairKind, bets: usize) -> Self {
        Self { score, kind, bets }
    }

    fn parse(raw_input: &str) -> Self {
        // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
        let (input, bets) = raw_input.split_once(' ').unwrap();
        let mut score = Vec::new();
        for c in input.chars() {
            match c {
                'A' => score.push(14),
                'K' => score.push(13),
                'Q' => score.push(12),
                'J' => score.push(11),
                'T' => score.push(10),
                _ => score.push(c.to_digit(10).unwrap()),
            }
        }

        Self::new(
            CardScore(score),
            input.parse::<PairKind>().unwrap(),
            bets.parse::<usize>().unwrap(),
        )
    }

    fn score_as_str(&self) -> String {
        let mut output = String::new();
        for digit in self.score.0.iter() {
            match digit {
                14 => output.push('A'),
                13 => output.push('K'),
                12 => output.push('Q'),
                11 => output.push('J'),
                10 => output.push('T'),
                _ => output.push_str(&digit.to_string()),
            }
        }
        output
    }
}

// Implement comparator for Card
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        // Check by kind first
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.score.cmp(&other.score),
            other => other,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        // Check by kind first
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.score.eq(&other.score),
            _ => false,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// implement custom debug
impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("hand", &self.score_as_str())
            .field("kind", &self.kind)
            .field("bets", &self.bets)
            .field("score", &self.score)
            .finish()
    }
}

pub fn parse_input(input: &str) -> Vec<Card> {
    split_into_lines(input)
        .map(Card::parse)
        .collect::<Vec<Card>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cards = parse_input(input);
    cards.sort();
    cards.reverse();

    Some(
        cards
            .iter()
            .enumerate()
            .map(|(idx, card)| card.bets * (idx + 1))
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

use std::collections::{HashMap, HashSet};

use crate::solutions::common::split_into_lines;

fn zip_numbers(number_spaced: &str) -> Vec<u32> {
    number_spaced
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[derive(Debug)]
struct ScratchCard {
    number: u32,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

impl ScratchCard {
    fn new(number: u32, winning_numbers: Vec<u32>, our_numbers: Vec<u32>) -> Self {
        Self {
            number,
            winning_numbers,
            our_numbers,
        }
    }

    // Parse from a line:
    // Card X: 1 2 3 4 5 | 6 7 8 9 10
    fn parse(line: &str) -> Self {
        let (card, numbers) = line.split_once(": ").unwrap();
        let (winning, ours) = numbers.split_once(" | ").unwrap();

        let card_index = card.replace("Card ", "").trim().parse::<u32>().unwrap();

        let winning_numbers = zip_numbers(winning);
        let our_numbers = zip_numbers(ours);

        let mut total = 0;
        for our in our_numbers.iter() {
            if winning_numbers.contains(our) {
                match total {
                    0 => total += 1,
                    _ => total *= 2,
                }
            }
        }

        Self::new(card_index, winning_numbers, our_numbers)
    }

    fn intersect(&self) -> Vec<&u32> {
        let hash_win: HashSet<&u32> = HashSet::from_iter(self.winning_numbers.iter());
        let hash_our: HashSet<&u32> = HashSet::from_iter(self.our_numbers.iter());

        hash_our.intersection(&hash_win).cloned().collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        split_into_lines(input)
            .map(|line| {
                let scratch_card = ScratchCard::parse(line);
                let intersected = scratch_card.intersect();
                // multiply by 2 for each number in the intersection
                if intersected.is_empty() {
                    return 0;
                }
                let total = (intersected.len() - 1) as u32;

                2u32.pow(total)
            })
            .sum::<u32>(),
    )
}

fn recursively_scratch_card(
    current_card: &ScratchCard,
    hashed_cards: &HashMap<u32, &ScratchCard>,
    collected: &mut HashMap<u32, u32>,
    max_card: &u32,
    is_deep: bool,
) {
    let total_won = current_card.intersect().len() as u32;

    if !is_deep {
        let existing = collected.get(&current_card.number).unwrap_or(&0);
        collected.insert(current_card.number, existing + 1);
    }

    if total_won == 0 {
        return;
    }

    for index in 0..total_won {
        let act_index = current_card.number + index + 1;
        let next_card = hashed_cards.get(&act_index);
        match next_card {
            Some(&card) => {
                // we have a card to check
                // add the current one to the collected
                let existing = collected.get(&card.number).unwrap_or(&0);
                collected.insert(card.number, existing + 1);
                recursively_scratch_card(card, hashed_cards, collected, max_card, true);
            }
            None => {
                // add the current one to the collected
                if act_index > *max_card {
                    // we're out of cards
                    return;
                }

                let existing = collected.get(&act_index).unwrap_or(&0);
                collected.insert(act_index, existing + 1);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<ScratchCard> = split_into_lines(input).map(ScratchCard::parse).collect();

    let mut hashed_cards: HashMap<u32, &ScratchCard> = HashMap::new();
    for card in cards.iter() {
        hashed_cards.insert(card.number, card);
    }

    let max_card = hashed_cards.keys().max().unwrap();

    let mut collected: HashMap<u32, u32> = HashMap::new();
    for card in cards.iter() {
        recursively_scratch_card(card, &hashed_cards, &mut collected, max_card, false);
    }

    let values = collected.values().sum::<u32>();

    Some(values)
}

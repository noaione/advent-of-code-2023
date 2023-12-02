use std::collections::HashMap;

use crate::solutions::common::split_into_lines;

fn map_color(color: &str) -> Option<u32> {
    match color {
        "red" => Some(12),
        "green" => Some(13),
        "blue" => Some(14),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        split_into_lines(input)
            .filter_map(|line| {
                let (game_id, sequences) = line.split_once(": ").unwrap();
                let id = game_id.replace("Game ", "").parse::<usize>().unwrap();

                let seq_test = sequences.split("; ").any(|group| {
                    let tried = group
                        .split(", ")
                        .filter(|&color| {
                            let (count, color) = color.split_once(' ').unwrap();
                            let count_32 = count.parse::<u32>().unwrap();
                            let color_max = map_color(color).unwrap();

                            count_32 > color_max
                        })
                        .count();
                    tried > 0
                });

                if !seq_test {
                    Some(id)
                } else {
                    None
                }
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        split_into_lines(input)
            .map(|line| {
                let (_, sequences) = line.split_once(": ").unwrap();

                let mut max_most = HashMap::new();
                max_most.insert("red", 0);
                max_most.insert("green", 0);
                max_most.insert("blue", 0);

                sequences.split("; ").for_each(|group| {
                    group.split(", ").for_each(|color| {
                        let (count, color_act) = color.split_once(' ').unwrap();
                        let count_32 = count.parse::<u32>().unwrap();

                        // if count_32 is less than max_most[color_act]
                        if count_32 > max_most[color_act] {
                            max_most.insert(color_act, count_32);
                        }
                    })
                });

                max_most
                    .iter()
                    .filter_map(|(_, &v)| if v == 0 { None } else { Some(v as u64) })
                    .product::<u64>()
            })
            .sum::<u64>(),
    )
}

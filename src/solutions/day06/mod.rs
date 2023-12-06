use itertools::Itertools;

use crate::solutions::common::split_into_lines;

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = split_into_lines(input);
    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .replace("Time: ", "")
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .replace("Distance: ", "")
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (times, distances)
}

pub fn part_one(input: &str) -> Option<usize> {
    // speed measured in mm/ms
    let (maximum_times_allowed, record_distances) = parse_input(input);

    let number_of_ways: usize = maximum_times_allowed
        .iter()
        .zip(record_distances.iter())
        .map(|(t, d)| {
            let max_time = *t;
            let record_distance = *d;
            let mut possible_max = max_time;
            for hold in (0..max_time).rev() {
                let time_left = max_time - hold;
                let covered_distance = hold * time_left;
                if covered_distance > record_distance {
                    possible_max = hold;
                    break;
                }
            }

            let mut possible_min = *t;
            for hold in 1..possible_max {
                let time_left = max_time - hold;
                let covered_distance = hold * time_left;
                if covered_distance > record_distance {
                    possible_min = hold;
                    break;
                }
            }

            (possible_min..possible_max).len() + 1
        })
        .product();

    Some(number_of_ways)
}

pub fn part_two(input: &str) -> Option<usize> {
    // speed measured in mm/ms
    let (maximum_times_allowed, record_distances) = parse_input(input);

    let max_time = maximum_times_allowed
        .iter()
        .map(|f| f.to_string())
        .join("")
        .parse::<usize>()
        .unwrap();
    let record_distance = record_distances
        .iter()
        .map(|f| f.to_string())
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut possible_max = max_time;
    for hold in (0..max_time).rev() {
        let time_left = max_time - hold;
        let covered_distance = hold * time_left;
        if covered_distance > record_distance {
            possible_max = hold;
            break;
        }
    }

    let mut possible_min = 1;
    for hold in 1..possible_max {
        let time_left = max_time - hold;
        let covered_distance = hold * time_left;
        if covered_distance > record_distance {
            possible_min = hold;
            break;
        }
    }
    println!("possible_min: {}", possible_min);

    Some((possible_min..possible_max).len() + 1)
}

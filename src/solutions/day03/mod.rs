use std::collections::HashSet;

use crate::solutions::common::split_into_lines;

pub fn create_table_type(input: &str) -> Vec<String> {
    split_into_lines(input)
        .map(|line| {
            // make a parser, a line can be like this:
            // - 467..114..
            // - 617*......
            // - ...$.*....
            // We want to group numbers together,

            let mut numbers: Vec<char> = Vec::new();
            for char in line.chars() {
                let kind = match char {
                    '0'..='9' => '1',
                    '.' => '3',
                    '*' => '4',
                    _ => '2',
                };

                numbers.push(kind);
            }

            numbers.iter().collect::<String>()
        })
        .collect()
}

fn check_adjacent(
    idx: usize,
    row: &str,
    top_row: Option<&String>,
    bottom_row: Option<&String>,
) -> bool {
    let min = idx.checked_sub(1);
    let left = min.and_then(|x| row.get(x..idx)).unwrap_or("0");
    let right = row.get(idx + 1..idx + 2).unwrap_or("0");

    let top_current = top_row.and_then(|row| row.get(idx..idx + 1)).unwrap_or("0");
    let top_left = top_row
        .and_then(|row| min.and_then(|x| row.get(x..idx)))
        .unwrap_or("0");
    let top_right = top_row
        .and_then(|row| row.get(idx + 1..idx + 2))
        .unwrap_or("0");

    let bottom_current = bottom_row
        .and_then(|row| row.get(idx..idx + 1))
        .unwrap_or("0");
    let bottom_left = bottom_row
        .and_then(|row| min.and_then(|x| row.get(x..idx)))
        .unwrap_or("0");
    let bottom_right = bottom_row
        .and_then(|row| row.get(idx + 1..idx + 2))
        .unwrap_or("0");

    // check if current is adjacent to "2"
    left.chars()
        .chain(right.chars())
        .chain(top_left.chars())
        .chain(top_current.chars())
        .chain(top_right.chars())
        .chain(bottom_left.chars())
        .chain(bottom_current.chars())
        .chain(bottom_right.chars())
        .any(|x| x == '2' || x == '4')
}

fn get_adjacent_kind_number(kind_row: &str, current_idx: usize, backward: bool) -> Vec<usize> {
    let mut idx = current_idx;
    if backward {
        idx = idx.saturating_sub(1);
    } else {
        idx += 1;
    }

    if idx == 0 {
        return vec![];
    }

    let mut collected = vec![];
    while idx > 0 && idx < kind_row.len() {
        let kind = kind_row.get(idx..idx + 1).unwrap_or("0");
        if kind != "1" {
            break;
        }

        collected.push(idx);

        if backward {
            idx = idx.checked_sub(1).unwrap();
        } else {
            idx += 1;
        }
    }

    if backward {
        if idx == 0 {
            let kind = kind_row.get(0..1).unwrap_or("0");
            if kind == "1" {
                collected.push(0);
            }
        }
        // reverse the order
        collected.reverse();
    }

    collected
}

pub fn part_one(input: &str) -> Option<u32> {
    let kind_data = create_table_type(input);

    Some(
        split_into_lines(input)
            .enumerate()
            .map(|(top_idx, line)| {
                let kind_row = kind_data.get(top_idx).unwrap();

                let mut existing_number_index: HashSet<usize> = HashSet::new();
                let mut added_numbers = Vec::new();
                for (idx, char) in line.chars().enumerate() {
                    let kind = kind_row.get(idx..idx + 1).unwrap();
                    if kind != "1" {
                        continue;
                    }

                    if existing_number_index.contains(&idx) {
                        continue;
                    }

                    let top_row = top_idx.checked_sub(1).and_then(|x| kind_data.get(x));
                    let bottom_row = kind_data.get(top_idx + 1);
                    let any_adjancent = check_adjacent(idx, kind_row, top_row, bottom_row);

                    if !any_adjancent {
                        continue;
                    }

                    let adjacent_forward = get_adjacent_kind_number(kind_row, idx, false);
                    let adjacent_back = get_adjacent_kind_number(kind_row, idx, true);

                    let ad_for_test = adjacent_forward
                        .iter()
                        .find(|&x| existing_number_index.contains(x));
                    let ad_back_test = adjacent_back
                        .iter()
                        .find(|&x| existing_number_index.contains(x));

                    if ad_for_test.is_some() || ad_back_test.is_some() {
                        continue;
                    }

                    // add the current index
                    existing_number_index.insert(idx);

                    let ad_back = adjacent_back
                        .iter()
                        .map(|x| {
                            existing_number_index.insert(*x);
                            line.get(*x..*x + 1).unwrap()
                        })
                        .collect::<String>();
                    let ad_for = adjacent_forward
                        .iter()
                        .map(|x| {
                            existing_number_index.insert(*x);
                            line.get(*x..*x + 1).unwrap()
                        })
                        .collect::<String>();

                    let merged = format!("{}{}{}", ad_back, char, ad_for)
                        .parse::<u32>()
                        .unwrap();
                    added_numbers.push(merged);
                }

                added_numbers.iter().sum::<u32>()
            })
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

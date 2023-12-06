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

fn maybe_adjacent(row: Option<&String>, idx: usize) -> &str {
    row.and_then(|row| row.get(idx..idx + 1)).unwrap_or("0")
}

fn check_adjacent(
    idx: usize,
    row: &str,
    top_row: Option<&String>,
    bottom_row: Option<&String>,
    fn_check: impl Fn(char) -> bool,
) -> bool {
    let min = idx.checked_sub(1);
    let left = min.and_then(|x| row.get(x..idx)).unwrap_or("0");
    let right = row.get(idx + 1..idx + 2).unwrap_or("0");

    let top_current = maybe_adjacent(top_row, idx);
    let top_left = min.map(|x| maybe_adjacent(top_row, x)).unwrap_or("0");
    let top_right = maybe_adjacent(top_row, idx + 1);

    let bottom_current = maybe_adjacent(bottom_row, idx);
    let bottom_left = min.map(|x| maybe_adjacent(bottom_row, x)).unwrap_or("0");
    let bottom_right = maybe_adjacent(bottom_row, idx + 1);

    // check if current is adjacent to "2"
    left.chars()
        .chain(right.chars())
        .chain(top_left.chars())
        .chain(top_current.chars())
        .chain(top_right.chars())
        .chain(bottom_left.chars())
        .chain(bottom_current.chars())
        .chain(bottom_right.chars())
        .any(fn_check)
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
                    let any_adjancent = check_adjacent(idx, kind_row, top_row, bottom_row, |x| {
                        x == '2' || x == '4'
                    });

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

// TODO: Refactor
#[allow(clippy::too_many_arguments)]
fn get_adjacent_number_pairs(
    left_line: &str,
    right_line: &str,
    left_pairs: &str,
    right_pairs: &str,
    left_idx: usize,
    right_idx: usize,
    backward_left: bool,
    backward_right: bool,
) -> (u32, u32) {
    let left_adjacent = get_adjacent_kind_number(left_pairs, left_idx, backward_left);
    let right_adjacent = get_adjacent_kind_number(right_pairs, right_idx, backward_right);

    let left_num = left_adjacent
        .iter()
        .map(|x| left_line.get(*x..*x + 1).unwrap())
        .collect::<String>();
    let right_num = right_adjacent
        .iter()
        .map(|x| right_line.get(*x..*x + 1).unwrap())
        .collect::<String>();

    (
        left_num.parse::<u32>().unwrap(),
        right_num.parse::<u32>().unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let kind_data: Vec<String> = create_table_type(input);
    let line_splits: Vec<&str> = split_into_lines(input).collect();

    Some(
        line_splits
            .iter()
            .enumerate()
            .map(|(top_idx, line)| {
                let kind_row = kind_data.get(top_idx).unwrap();

                let mut number_vec: Vec<(u32, u32)> = Vec::new();
                for (idx, _) in line.chars().enumerate() {
                    let kind = kind_row.get(idx..idx + 1).unwrap();
                    if kind != "4" {
                        continue;
                    }

                    let top_row = top_idx.checked_sub(1).and_then(|x| kind_data.get(x));
                    let bottom_row = kind_data.get(top_idx + 1);
                    let any_adjancent =
                        check_adjacent(idx, kind_row, top_row, bottom_row, |x| x == '1');

                    if !any_adjancent {
                        continue;
                    }

                    let left_get = kind_row.get(idx - 1..idx).unwrap_or(".");
                    let right_get = kind_row.get(idx + 1..idx + 2).unwrap_or(".");

                    if left_get == "1" && right_get == "1" {
                        // left/right is number, get adjacent for each side
                        let (left_num, right_num) = get_adjacent_number_pairs(
                            line, line, kind_row, kind_row, idx, idx, true, false,
                        );

                        number_vec.push((left_num, right_num));
                    } else if left_get == "1" && right_get != "1" {
                        // available on left, check on top/bottom
                        let start_min = idx.saturating_sub(1);
                        let top_index = match top_row {
                            Some(x) => x.get(start_min..idx + 2).unwrap_or("").contains('1'),
                            None => false,
                        };
                        let bottom_index = match bottom_row {
                            Some(x) => x.get(start_min..idx + 2).unwrap_or("").contains('1'),
                            None => false,
                        };

                        let left_adjacent = get_adjacent_kind_number(kind_row, idx, true);
                        let left_num = left_adjacent
                            .iter()
                            .map(|x| line.get(*x..*x + 1).unwrap())
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        if top_index {
                            let top_line = line_splits.get(top_idx - 1).unwrap();
                            let top_left_adjacent =
                                get_adjacent_kind_number(top_row.unwrap(), idx, true);
                            let top_right_adjacent =
                                get_adjacent_kind_number(top_row.unwrap(), idx, false);

                            let top_left_num = top_left_adjacent
                                .iter()
                                .map(|x| top_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();
                            let top_right_num = top_right_adjacent
                                .iter()
                                .map(|x| top_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();

                            let top_cur_idx = top_row.unwrap().get(idx..idx + 1).unwrap_or(".");
                            let top_cur = match top_cur_idx {
                                "1" => top_line.get(idx..idx + 1).unwrap_or(""),
                                _ => "",
                            };

                            let top_fmt = format!("{}{}{}", top_left_num, top_cur, top_right_num)
                                .parse::<u32>()
                                .unwrap();

                            number_vec.push((left_num, top_fmt));
                        } else if bottom_index {
                            let bottom_line = line_splits.get(top_idx + 1).unwrap();
                            let bottom_left_adjacent =
                                get_adjacent_kind_number(bottom_row.unwrap(), idx, true);
                            let bottom_right_adjacent =
                                get_adjacent_kind_number(bottom_row.unwrap(), idx, false);

                            let bottom_left_num = bottom_left_adjacent
                                .iter()
                                .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();
                            let bottom_right_num = bottom_right_adjacent
                                .iter()
                                .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();

                            let bottom_cur_idx =
                                bottom_row.unwrap().get(idx..idx + 1).unwrap_or(".");
                            let bottom_cur = match bottom_cur_idx {
                                "1" => bottom_line.get(idx..idx + 1).unwrap_or(""),
                                _ => "",
                            };

                            let bottom_fmt =
                                format!("{}{}{}", bottom_left_num, bottom_cur, bottom_right_num)
                                    .parse::<u32>()
                                    .unwrap();

                            number_vec.push((left_num, bottom_fmt));
                        }
                    } else if right_get == "1" && left_get != "1" {
                        // available on left, check on top/bottom
                        let start_min = idx.saturating_sub(1);
                        let top_index = match top_row {
                            Some(x) => x.get(start_min..idx + 2).unwrap_or("").contains('1'),
                            None => false,
                        };
                        let bottom_index = match bottom_row {
                            Some(x) => x.get(start_min..idx + 2).unwrap_or("").contains('1'),
                            None => false,
                        };

                        let right_adjacent = get_adjacent_kind_number(kind_row, idx, false);
                        let right_num = right_adjacent
                            .iter()
                            .map(|x| line.get(*x..*x + 1).unwrap())
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        if top_index {
                            let top_line = line_splits.get(top_idx - 1).unwrap();
                            let top_left_adjacent =
                                get_adjacent_kind_number(top_row.unwrap(), idx, true);
                            let top_right_adjacent =
                                get_adjacent_kind_number(top_row.unwrap(), idx, false);

                            let top_left_num = top_left_adjacent
                                .iter()
                                .map(|x| top_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();
                            let top_right_num = top_right_adjacent
                                .iter()
                                .map(|x| top_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();

                            let top_cur_idx = top_row.unwrap().get(idx..idx + 1).unwrap_or(".");
                            let top_cur = match top_cur_idx {
                                "1" => top_line.get(idx..idx + 1).unwrap_or(""),
                                _ => "",
                            };

                            let top_fmt = format!("{}{}{}", top_left_num, top_cur, top_right_num)
                                .parse::<u32>()
                                .unwrap();

                            number_vec.push((right_num, top_fmt));
                        } else if bottom_index {
                            let bottom_line = line_splits.get(top_idx + 1).unwrap();
                            let bottom_left_adjacent =
                                get_adjacent_kind_number(bottom_row.unwrap(), idx, true);
                            let bottom_right_adjacent =
                                get_adjacent_kind_number(bottom_row.unwrap(), idx, false);

                            let bottom_left_num = bottom_left_adjacent
                                .iter()
                                .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();
                            let bottom_right_num = bottom_right_adjacent
                                .iter()
                                .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                .collect::<String>();

                            let bottom_cur_idx =
                                bottom_row.unwrap().get(idx..idx + 1).unwrap_or(".");
                            let bottom_cur = match bottom_cur_idx {
                                "1" => bottom_line.get(idx..idx + 1).unwrap_or(""),
                                _ => "",
                            };

                            let bottom_fmt =
                                format!("{}{}{}", bottom_left_num, bottom_cur, bottom_right_num)
                                    .parse::<u32>()
                                    .unwrap();

                            number_vec.push((right_num, bottom_fmt));
                        }
                    } else if top_row.is_some() && bottom_row.is_some() {
                        let top_row_real = top_row.unwrap();
                        let bottom_row_real = bottom_row.unwrap();

                        let top_line = line_splits.get(top_idx - 1).unwrap();
                        let bottom_line = line_splits.get(top_idx + 1).unwrap();

                        let top_mid = top_row_real.get(idx..idx + 1).unwrap_or(".");
                        let top_left = idx
                            .checked_sub(1)
                            .and_then(|x| top_row_real.get(x..x + 1))
                            .unwrap_or(".");
                        let top_right = top_row_real.get(idx + 1..idx + 2).unwrap_or(".");

                        let bottom_mid = bottom_row_real.get(idx..idx + 1).unwrap_or(".");
                        let bottom_left = idx
                            .checked_sub(1)
                            .and_then(|x| bottom_row_real.get(x..x + 1))
                            .unwrap_or(".");
                        let bottom_right = bottom_row_real.get(idx + 1..idx + 2).unwrap_or(".");

                        if top_mid != "1" && top_left == "1" && top_right == "1" {
                            let (left_num, right_num) = get_adjacent_number_pairs(
                                top_line,
                                top_line,
                                top_row_real,
                                top_row_real,
                                idx,
                                idx,
                                true,
                                false,
                            );

                            number_vec.push((left_num, right_num));
                        } else if bottom_mid != "1" && bottom_left == "1" && bottom_right == "1" {
                            let (left_num, right_num) = get_adjacent_number_pairs(
                                bottom_line,
                                bottom_line,
                                bottom_row_real,
                                bottom_row_real,
                                idx,
                                idx,
                                true,
                                false,
                            );

                            number_vec.push((left_num, right_num));
                        } else {
                            let top_left_adjacent =
                                get_adjacent_kind_number(top_row_real, idx, true);
                            let top_right_adjacent =
                                get_adjacent_kind_number(top_row_real, idx, false);
                            let bottom_left_adjacent =
                                get_adjacent_kind_number(bottom_row_real, idx, true);
                            let bottom_right_adjacent =
                                get_adjacent_kind_number(bottom_row_real, idx, false);

                            let top_any = (top_left_adjacent.len() + top_right_adjacent.len()) > 0;
                            let bottom_any =
                                (bottom_left_adjacent.len() + bottom_right_adjacent.len()) > 0;

                            if top_any && bottom_any {
                                let top_left_num = top_left_adjacent
                                    .iter()
                                    .map(|x| top_line.get(*x..*x + 1).unwrap())
                                    .collect::<String>();
                                let top_right_num = top_right_adjacent
                                    .iter()
                                    .map(|x| top_line.get(*x..*x + 1).unwrap())
                                    .collect::<String>();

                                let bottom_left_num = bottom_left_adjacent
                                    .iter()
                                    .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                    .collect::<String>();
                                let bottom_right_num = bottom_right_adjacent
                                    .iter()
                                    .map(|x| bottom_line.get(*x..*x + 1).unwrap())
                                    .collect::<String>();

                                let top_cur_idx = top_row_real.get(idx..idx + 1).unwrap_or(".");
                                let top_cur = match top_cur_idx {
                                    "1" => top_line.get(idx..idx + 1).unwrap_or(""),
                                    _ => "",
                                };
                                let bottom_cur_idx =
                                    bottom_row_real.get(idx..idx + 1).unwrap_or(".");
                                let bottom_cur = match bottom_cur_idx {
                                    "1" => bottom_line.get(idx..idx + 1).unwrap_or(""),
                                    _ => "",
                                };

                                let top_num_col =
                                    format!("{}{}{}", top_left_num, top_cur, top_right_num);
                                let bottom_num_col = format!(
                                    "{}{}{}",
                                    bottom_left_num, bottom_cur, bottom_right_num
                                );

                                number_vec.push((
                                    top_num_col.parse::<u32>().unwrap(),
                                    bottom_num_col.parse::<u32>().unwrap(),
                                ));
                            }
                        }
                    } else if let Some(top_row_real) = top_row {
                        let top_line = line_splits.get(top_idx - 1).unwrap();

                        let top_mid = top_row_real.get(idx..idx + 1).unwrap_or(".");
                        let top_left = idx
                            .checked_sub(1)
                            .and_then(|x| top_row_real.get(x..x + 1))
                            .unwrap_or(".");
                        let top_right = top_row_real.get(idx + 1..idx + 2).unwrap_or(".");

                        // If top_mid is "." and left right is number
                        if top_mid != "1" && top_left == "1" && top_right == "1" {
                            let (left_num, right_num) = get_adjacent_number_pairs(
                                top_line,
                                top_line,
                                top_row_real,
                                top_row_real,
                                idx,
                                idx,
                                true,
                                false,
                            );

                            number_vec.push((left_num, right_num));
                        }
                    } else if let Some(bottom_row_real) = bottom_row {
                        let bottom_line = line_splits.get(top_idx + 1).unwrap();

                        let bottom_mid = bottom_row_real.get(idx..idx + 1).unwrap_or(".");
                        let bottom_left = idx
                            .checked_sub(1)
                            .and_then(|x| bottom_row_real.get(x..x + 1))
                            .unwrap_or(".");
                        let bottom_right = bottom_row_real.get(idx + 1..idx + 2).unwrap_or(".");

                        // If bottom_mid is "." and left right is number
                        if bottom_mid != "1" && bottom_left == "1" && bottom_right == "1" {
                            let (left_num, right_num) = get_adjacent_number_pairs(
                                bottom_line,
                                bottom_line,
                                bottom_row_real,
                                bottom_row_real,
                                idx,
                                idx,
                                true,
                                false,
                            );

                            number_vec.push((left_num, right_num));
                        }
                    }
                }

                number_vec
                    .iter()
                    .map(|(x, y)| (*x as u64) * (*y as u64))
                    .sum::<u64>()
            })
            .sum::<u64>(),
    )
}

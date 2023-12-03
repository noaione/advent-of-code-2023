use crate::solutions::common::split_into_lines;

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
                            let color_max = match color {
                                "red" => 12,
                                "green" => 13,
                                "blue" => 14,
                                _ => panic!("Invalid color!"),
                            };

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

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        split_into_lines(input)
            .map(|line| {
                let (_, sequences) = line.split_once(": ").unwrap();

                let mut red_most: u32 = 0;
                let mut green_most: u32 = 0;
                let mut blue_most: u32 = 0;

                sequences.split("; ").for_each(|group| {
                    group.split(", ").for_each(|color| {
                        let (count, color_act) = color.split_once(' ').unwrap();
                        let count_32 = count.parse::<u32>().unwrap();

                        match color_act {
                            "red" => {
                                red_most = red_most.max(count_32);
                            }
                            "green" => {
                                green_most = green_most.max(count_32);
                            }
                            "blue" => {
                                blue_most = blue_most.max(count_32);
                            }
                            _ => panic!("Invalid color!"),
                        }
                    })
                });

                red_most * green_most * blue_most
            })
            .sum::<u32>(),
    )
}

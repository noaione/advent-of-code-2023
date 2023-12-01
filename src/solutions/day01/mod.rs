use crate::solutions::common::split_into_lines;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        split_into_lines(input)
            .map(|line| {
                // check if numbers
                let chars: Vec<u32> = line
                    .matches(char::is_numeric)
                    .map(|c| c.parse::<u32>().expect("Could not parse"))
                    .collect();

                // get first and last
                let first = chars[0];
                let last = chars[chars.len() - 1];

                // create pairs
                format!("{}{}", first, last)
                    .parse::<u32>()
                    .expect("Could not parse")
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    Some(
        split_into_lines(input)
            .map(|data| {
                let mut results: Vec<usize> = Vec::new();
                let chars: Vec<char> = data.chars().collect();

                for i in 0..data.len() {
                    // get current char
                    let current = chars.get(i).unwrap();

                    if current.is_ascii_digit() {
                        results.push(current.to_digit(10).unwrap() as usize)
                    }

                    let part = &data[i..];

                    match digits
                        .iter()
                        .enumerate()
                        .find(|(_, &digit)| part.starts_with(digit))
                    {
                        Some((idx, _)) => {
                            results.push(idx + 1);
                        }
                        None => {
                            continue;
                        }
                    }
                }

                // get first and last
                let first = results[0];
                let last = results[results.len() - 1];

                // create pairs
                format!("{}{}", first, last)
                    .parse::<u32>()
                    .expect("Could not parse")
            })
            .sum::<u32>(),
    )
}

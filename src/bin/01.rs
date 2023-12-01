use regex::Regex;

advent_of_code::solution!(1);

pub fn common_split(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        common_split(input)
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
    let digits_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    Some(
        common_split(input)
            .map(|data| {
                let result: Vec<u32> = digits_re
                    .captures_iter(data)
                    .map(|mat| {
                        let res = mat.get(0).unwrap().as_str();

                        res.parse::<u32>().unwrap_or(match res {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            _ => 0,
                        })
                    })
                    .collect();

                // get first and last
                let first = result[0];
                let last = result[result.len() - 1];

                format!("{}{}", first, last)
                    .parse::<u32>()
                    .expect("Could not parse")
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

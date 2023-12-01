advent_of_code::solution!(1);

pub fn common_split(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}

pub fn part_one(input: &str) -> Option<u32> {
    advent_of_code::solutions::day01::part_one(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    advent_of_code::solutions::day01::part_two(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let examples = "1abc2\n\
                              pqr3stu8vwx\n\
                              a1b2c3d4e5f\n\
                              treb7uchet";
        let result = part_one(examples);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53334));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(52834));
    }
}

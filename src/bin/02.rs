advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    advent_of_code::solutions::day02::part_one(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    advent_of_code::solutions::day02::part_two(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2006));
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(84911));
    }
}

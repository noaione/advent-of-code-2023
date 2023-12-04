advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    advent_of_code::solutions::day01::part_one(input)
}

pub fn part_two(input: &str) -> Option<usize> {
    advent_of_code::solutions::day01::part_two(input)
}

advent_of_code::aoc_test_ex!(DAY, Some(142), Some(281));
advent_of_code::aoc_test_real!(DAY, Some(53334), Some(52834));

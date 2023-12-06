use advent_of_code::aoc_init_bench;
use advent_of_code::Day;
use criterion::{criterion_group, criterion_main, Criterion};

aoc_init_bench!(
    name = bench_day_one;
    day = 1;
    part_one = advent_of_code::solutions::day01::part_one;
    part_two = advent_of_code::solutions::day01::part_two;
);
aoc_init_bench!(
    name = bench_day_two;
    day = 2;
    part_one = advent_of_code::solutions::day02::part_one;
    part_two = advent_of_code::solutions::day02::part_two;
);
aoc_init_bench!(
    name = bench_day_four;
    day = 4;
    part_one = advent_of_code::solutions::day04::part_one;
    // part_two = advent_of_code::solutions::day04::part_two; // too long
);
aoc_init_bench!(
    name = bench_day_five;
    day = 5;
    part_one = advent_of_code::solutions::day05::part_one;
    parser = advent_of_code::solutions::day05::parse_input;
);

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_day_one, bench_day_two, bench_day_four, bench_day_five
);
criterion_main!(benches);

use advent_of_code::Day;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day_one(c: &mut Criterion) {
    let inputs = advent_of_code::template::read_file("inputs", Day::new(1).unwrap());

    c.bench_function("part_a", |b| {
        b.iter(|| advent_of_code::solutions::day01::part_one(&inputs))
    });

    c.bench_function("part_b", |b| {
        b.iter(|| advent_of_code::solutions::day01::part_two(&inputs))
    });
}

fn bench_day_two(c: &mut Criterion) {
    let inputs = advent_of_code::template::read_file("inputs", Day::new(2).unwrap());

    c.bench_function("part_a", |b| {
        b.iter(|| advent_of_code::solutions::day02::part_one(&inputs))
    });

    c.bench_function("part_b", |b| {
        b.iter(|| advent_of_code::solutions::day02::part_two(&inputs))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_day_one, bench_day_two
);
criterion_main!(benches);

use advent_of_code::Day;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day_one(c: &mut Criterion) {
    let inputs = advent_of_code::template::read_file("inputs", Day::new(1).unwrap());

    c.bench_function("part_a", |b| {
        b.iter(|| advent_of_code::solutions::day01::part_one(&inputs))
    });

    c.bench_function("part_two", |b| {
        b.iter(|| advent_of_code::solutions::day01::part_two(&inputs))
    });
}

criterion_group!(benches, bench_day_one);
criterion_main!(benches);

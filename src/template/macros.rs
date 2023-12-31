/// Create a benchmark for a day.
#[macro_export]
macro_rules! aoc_init_bench {
    (
        name = $name:ident;
        day = $day:expr;
        part_one = $part_a_func:path;
        part_two = $part_b_func:path;
        parser = $parser_func:path;
    ) => {
        fn $name(c: &mut Criterion) {
            let inputs = advent_of_code::template::read_file("inputs", Day::new($day).unwrap());

            c.bench_function("parse_input", |b| b.iter(|| $parser_func(&inputs)));
            c.bench_function("part_a", |b| b.iter(|| $part_a_func(&inputs)));
            c.bench_function("part_b", |b| b.iter(|| $part_b_func(&inputs)));
        }
    };
    (
        name = $name:ident;
        day = $day:expr;
        part_one = $part_a_func:path;
        parser = $parser_func:path;
    ) => {
        fn $name(c: &mut Criterion) {
            let inputs = advent_of_code::template::read_file("inputs", Day::new($day).unwrap());

            c.bench_function("parse_input", |b| b.iter(|| $parser_func(&inputs)));
            c.bench_function("part_a", |b| b.iter(|| $part_a_func(&inputs)));
        }
    };
    (
        name = $name:ident;
        day = $day:expr;
        parser = $parser_func:path;
    ) => {
        fn $name(c: &mut Criterion) {
            let inputs = advent_of_code::template::read_file("inputs", Day::new($day).unwrap());

            c.bench_function("parse_input", |b| b.iter(|| $parser_func(&inputs)));
        }
    };
    (
        name = $name:ident;
        day = $day:expr;
        part_one = $part_a_func:path;
        part_two = $part_b_func:path;
    ) => {
        fn $name(c: &mut Criterion) {
            let inputs = advent_of_code::template::read_file("inputs", Day::new($day).unwrap());

            c.bench_function("part_a", |b| b.iter(|| $part_a_func(&inputs)));
            c.bench_function("part_b", |b| b.iter(|| $part_b_func(&inputs)));
        }
    };
    (
        name = $name:ident;
        day = $day:expr;
        part_one = $part_a_func:path;
    ) => {
        fn $name(c: &mut Criterion) {
            let inputs = advent_of_code::template::read_file("inputs", Day::new($day).unwrap());

            c.bench_function("part_a", |b| b.iter(|| $part_a_func(&inputs)));
        }
    };
}

/// Creates a testsuite for a day with same examples.
#[macro_export]
macro_rules! aoc_test {
    ($day:expr, $expect_a:expr, $expect_b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part_one() {
                let result = part_one(&advent_of_code::template::read_file("examples", $day));
                assert_eq!(result, $expect_a);
            }

            #[test]
            fn test_part_two() {
                let result = part_two(&advent_of_code::template::read_file("examples", $day));
                assert_eq!(result, $expect_b);
            }
        }
    };
    ($day:expr, $expect_a:expr) => {
        $crate::aoc_test!($day, $expect_a, None);
    };
    ($day:expr) => {
        $crate::aoc_test!($day, None, None);
    };
}

/// Creates a testsuite for a day with different examples.
#[macro_export]
macro_rules! aoc_test_ex {
    ($day:expr, $expect_a:expr, $expect_b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part_one() {
                let result = part_one(&advent_of_code::template::read_file_part(
                    "examples", $day, 1,
                ));
                assert_eq!(result, $expect_a);
            }

            #[test]
            fn test_part_two() {
                let result = part_two(&advent_of_code::template::read_file_part(
                    "examples", $day, 2,
                ));
                assert_eq!(result, $expect_b);
            }
        }
    };
    ($day:expr, $expect_a:expr) => {
        $crate::aoc_test_ex!($day, $expect_a, None);
    };
    ($day:expr) => {
        $crate::aoc_test_ex!($day, None, None);
    };
}

/// Creates a testsuite for a day with real input.
#[macro_export]
macro_rules! aoc_test_real {
    ($day:expr, $expect_a:expr, $expect_b:expr) => {
        #[test]
        #[cfg_attr(feature = "ci", ignore)]
        fn test_part_one_input() {
            // Only run the input test if $expect_a is not None.
            let result = part_one(&advent_of_code::template::read_file("inputs", $day));
            assert_eq!(result, $expect_a);
        }

        #[test]
        #[cfg_attr(feature = "ci", ignore)]
        fn test_part_two_input() {
            // Only run the input test if $expect_b is not None.
            let result = part_two(&advent_of_code::template::read_file("inputs", $day));
            assert_eq!(result, $expect_b);
        }
    };
    ($day:expr, $expect_a:expr) => {
        #[test]
        #[cfg_attr(feature = "ci", ignore)]
        fn test_part_one_input() {
            // Only run the input test if $expect_a is not None.
            let result = part_one(&advent_of_code::template::read_file("inputs", $day));
            assert_eq!(result, $expect_a);
        }
    };
}

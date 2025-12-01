use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

// The arrow in the puzzle starts at 50.
const START: i32 = 50;

fn main() {
    let rotations_that_end_on_zero = {
        let file = File::open("input").expect("input file not found");

        let reader = BufReader::new(file);

        count_rotations_that_end_on_zero(START, reader.lines())
    };

    println!("Rotations that end on zero: {rotations_that_end_on_zero}");

    let times_zero_is_visited = {
        let file = File::open("input").expect("input file not found");

        let reader = BufReader::new(file);

        count_times_zero_is_visited(START, reader.lines())
    };

    println!("Number of times zero is visited: {times_zero_is_visited}");

    let times_zero_is_visited_naive = {
        let file = File::open("input").expect("input file not found");

        let reader = BufReader::new(file);

        count_times_zero_is_visited_naive(START, reader.lines())
    };

    println!("Number of times zero is visited (naive version): {times_zero_is_visited_naive}");
}

// Part 1.
fn count_rotations_that_end_on_zero(
    start: i32,
    lines: impl Iterator<Item = Result<String>>,
) -> usize {
    let mut pos = start;

    // Number of times the dial is left pointing at 0 after any rotation in the
    // sequence.
    let mut zero_count = 0;

    for line in lines {
        let line = line.unwrap();

        let Some(delta) = parse_line(&line) else {
            continue;
        };

        pos = i32::rem_euclid(pos + delta, 100);

        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

// Part 2. Naive solution that just counts.
fn count_times_zero_is_visited_naive(
    start: i32,
    lines: impl Iterator<Item = Result<String>>,
) -> i32 {
    let mut pos = start;

    // Number of times the dial is at zero, at any point.
    let mut zero_count = 0;

    for line in lines {
        let line = line.unwrap();

        let Some(delta) = parse_line(&line) else {
            continue;
        };

        for _ in 0..i32::abs(delta) {
            if delta < 0 {
                pos -= 1;
            }
            if delta > 0 {
                pos += 1
            }

            pos = i32::rem_euclid(pos, 100);

            if pos == 0 {
                zero_count += 1
            }
        }
    }

    zero_count
}

// Part 2. Slightly smarter version that only needs one iteration per input line.
fn count_times_zero_is_visited(start: i32, lines: impl Iterator<Item = Result<String>>) -> i32 {
    // Arrow always starts at 50.
    let mut pos = start;

    // Number of times the dial is at zero, at any point.
    let mut zero_count = 0;

    for line in lines {
        let line = line.unwrap();

        let Some(delta) = parse_line(&line) else {
            continue;
        };

        if pos + delta < 0 {
            zero_count += i32::abs(pos + delta) / 100;
            if pos != 0 {
                zero_count += 1;
            }
        } else if delta > 0 {
            zero_count += (pos + delta) / 100;
        } else if delta < 0 && pos + delta == 0 {
            zero_count += 1;
        }

        pos = i32::rem_euclid(pos + delta, 100);
    }

    zero_count
}

/// Parse one line of the puzzle's input. Expected format `[R|L]\d+`, no
/// trailing new line.
///
/// Returns a signed integer representing the rotation amount, that's negative
/// for left rotations and positive for right rotations,.
fn parse_line(line: &str) -> Option<i32> {
    let bytes = line.as_bytes();

    let sign = match bytes.first() {
        Some(b'R') => 1,
        Some(b'L') => -1,
        _ => return None,
    };

    let amount_str = str::from_utf8(&bytes[1..]).unwrap();
    let absolute_amount = i32::from_str_radix(amount_str, 10).unwrap();

    Some(sign * absolute_amount)
}

#[cfg(test)]
mod test {
    use crate::{START, count_rotations_that_end_on_zero, count_times_zero_is_visited};
    use std::io::Result;

    const PROVIDED_TEST_INPUT: &'static str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ";

    fn get_test_input(input: &str) -> impl Iterator<Item = Result<String>> {
        input.split('\n').map(String::from).map(Result::Ok)
    }

    #[test]
    fn test_count_rotations_that_end_on_zero_with_sample_input() {
        let zero_count =
            count_rotations_that_end_on_zero(START, get_test_input(PROVIDED_TEST_INPUT));

        assert_eq!(zero_count, 3)
    }

    #[test]
    fn test_count_times_zero_is_visited_with_sample_input() {
        let zero_count = count_times_zero_is_visited(START, get_test_input(PROVIDED_TEST_INPUT));

        assert_eq!(zero_count, 6)
    }

    #[test]
    fn test_50_positive_but_less_than_full_rotation() {
        let input = "
R1
";
        let zero_count = count_times_zero_is_visited(START, get_test_input(input));

        assert_eq!(zero_count, 0)
    }

    #[test]
    fn test_50_positive_more_than_full_rotation() {
        let input = "
R150
";
        let zero_count = count_times_zero_is_visited(START, get_test_input(input));

        assert_eq!(zero_count, 2)
    }

    #[test]
    fn test_50_positive_full_rotation() {
        let input = "
R100
";
        let zero_count = count_times_zero_is_visited(START, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_50_positive_double_full_rotation() {
        let input = "
R100
R100
";
        let zero_count = count_times_zero_is_visited(START, get_test_input(input));

        assert_eq!(zero_count, 2)
    }

    #[test]
    fn test_0_positive_but_less_than_full_rotation() {
        let input = "
R1
";
        let zero_count = count_times_zero_is_visited(0, get_test_input(input));

        assert_eq!(zero_count, 0)
    }

    #[test]
    fn test_0_positive_more_than_full_rotation() {
        let input = "
R150
";
        let zero_count = count_times_zero_is_visited(0, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_0_positive_full_rotation() {
        let input = "
R100
";
        let zero_count = count_times_zero_is_visited(0, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_0_positive_double_full_rotation() {
        let input = "
R100
R100
";
        let zero_count = count_times_zero_is_visited(0, get_test_input(input));

        assert_eq!(zero_count, 2)
    }

    #[test]
    fn test_0_negative_1() {
        let input = "
L1
";
        let zero_count = count_times_zero_is_visited(0, get_test_input(input));

        assert_eq!(zero_count, 0)
    }

    #[test]
    fn test_99_positive_1() {
        let input = "
R1
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_99_positive_but_less_than_full_rotation() {
        let input = "
R15
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_99_positive_full_rotation() {
        let input = "
R100
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_99_negative_but_less_than_full_rotation() {
        let input = "
L1
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 0)
    }

    #[test]
    fn test_99_negative_more_than_full_rotation() {
        let input = "
L150
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_99_negative_full_rotation() {
        let input = "
L100
";
        let zero_count = count_times_zero_is_visited(99, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_1_negative_but_less_than_full_rotation() {
        let input = "
L5
";
        let zero_count = count_times_zero_is_visited(1, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_1_negative_more_than_full_rotation() {
        let input = "
L150
";
        let zero_count = count_times_zero_is_visited(1, get_test_input(input));

        assert_eq!(zero_count, 2)
    }

    #[test]
    fn test_1_negative_full_rotation() {
        let input = "
L100
";
        let zero_count = count_times_zero_is_visited(1, get_test_input(input));

        assert_eq!(zero_count, 1)
    }

    #[test]
    fn test_1_negative_double_full_rotation() {
        let input = "
L100
L100
";
        let zero_count = count_times_zero_is_visited(1, get_test_input(input));

        assert_eq!(zero_count, 2)
    }

    #[test]
    fn test_38_negative_38() {
        let input = "
L38
";
        let zero_count = count_times_zero_is_visited(38, get_test_input(input));

        assert_eq!(zero_count, 1)
    }
}

use std::{fs::File, io::Read};

fn main() {
    // part1();
    part2();
}

fn part1() {
    let mut file = File::open("test").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut total = 0;

    for range in input.split(',') {
        let (min_str, max_str) = range.split_once('-').unwrap();

        let min = min_str.trim().parse::<usize>().unwrap();
        let max = max_str.trim().parse::<usize>().unwrap();

        for num in min..=max {
            let number_of_digits = num.ilog10() + 1;

            if number_of_digits % 2 == 0 {
                // We need an even number of digits.

                let first_half = num / 10_usize.pow(number_of_digits / 2);
                let second_half = num % (first_half * 10_usize.pow(number_of_digits / 2));

                if first_half == second_half {
                    total += num;
                }
            }
        }
    }

    println!("{total}");
}

fn part2() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut total = 0;

    for range in input.split(',') {
        let (min_str, max_str) = range.split_once('-').unwrap();

        let min = min_str.trim().parse::<usize>().unwrap();
        let max = max_str.trim().parse::<usize>().unwrap();

        for num in min..=max {
            let string = num.to_string();





            for len in 1..=string.len() / 2 {
                let sequence = &string[0..len];

                if string
                    .as_bytes()
                    .chunks(len)
                    .all(|chunk| chunk == sequence.as_bytes())
                {
                    total += num;
                    break;
                }
            }
        }
    }

    println!("{total}");
}

fn digits(value: u64, radix: u64) -> impl Iterator<Item = u64> + Clone + 'static {
    debug_assert!(radix > 0);

    #[derive(Clone, Copy)]
    struct Digits {
        n: u64,
        divisor: u64,
    }

    impl Digits {
        fn new(n: u64, radix: u64) -> Self {
            let mut divisor = 1;
            while n >= divisor * radix {
                divisor *= radix;
            }

            Digits { n, divisor }
        }
    }

    impl Iterator for Digits {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if self.divisor == 0 {
                None
            } else {
                let v = Some(self.n / self.divisor);
                self.n %= self.divisor;
                self.divisor /= 10;
                v
            }
        }
    }

    Digits::new(value, radix)
}

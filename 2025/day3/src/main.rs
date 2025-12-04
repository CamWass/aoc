use std::{fs::File, io::Read};

fn main() {
    part2();
}

fn part1() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut total = 0;

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut highest_first_digit = None;
        let mut highest_second_digit = None;

        for (i, char) in line.chars().enumerate() {
            let digit = char.to_digit(10).unwrap();

            match highest_first_digit {
                None => highest_first_digit = Some(digit),
                Some(d) if d < digit && i != line.len() - 1 => {
                    highest_first_digit = Some(digit);
                    highest_second_digit = None;
                }
                Some(_) => match highest_second_digit {
                    None => highest_second_digit = Some(digit),
                    Some(d) if d < digit => highest_second_digit = Some(digit),
                    Some(_) => {}
                },
            }
        }

        let largest_num = highest_first_digit.unwrap() * 10 + highest_second_digit.unwrap();

        println!(
            "{} {} {largest_num}",
            highest_first_digit.unwrap(),
            highest_second_digit.unwrap()
        );

        total += largest_num;
    }

    println!("{total}");
}

fn part2() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut total = 0;

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut digits = Vec::with_capacity(12);

        for (i, char) in line.chars().enumerate() {
            let digit = char.to_digit(10).unwrap();

            let mut used = false;

            for (digit_index, &existing_digit) in digits.iter().enumerate() {
                if existing_digit < digit && line.len() - i >= 12 - digit_index {
                    digits[digit_index] = digit;
                    digits.truncate(digit_index + 1);
                    used = true;
                    break;
                }
            }

            if !used && digits.len() < 12 {
                digits.push(digit);
            }
        }

        let largest_num: usize = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| *d as usize * 10_usize.pow(i as u32))
            .sum();

        println!("{largest_num}");

        total += largest_num;
    }

    println!("{total}");
}

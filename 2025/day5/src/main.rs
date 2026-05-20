use std::{fs::File, io::Read, usize};

fn main() {
    part2();
}

fn part1() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut lines = input.trim().split('\n');

    let mut ranges: Vec<(usize, usize)> = Vec::new();

    // Parse ranges:
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            // End of ranges, start of IDs.
            break;
        }

        let (min, max) = line.split_once('-').unwrap();

        ranges.push((min.parse().unwrap(), max.parse().unwrap()));
    }

    // Parse IDs:
    let ids = lines
        .filter(|l| l.len() > 0)
        .map(|l| l.parse::<usize>().unwrap());

    let available_ids = ids.filter(|&id| ranges.iter().any(|(min, max)| id >= *min && id <= *max));

    let number_of_available_ids = available_ids.count();

    println!("{number_of_available_ids}");
}

fn part2() {
    let mut file = File::open("foo1").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut lines = input.trim().split('\n');

    let mut ranges: Vec<(usize, usize)> = Vec::new();

    // Parse ranges:
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            // End of ranges, start of IDs.
            break;
        }

        let (min, max) = line.split_once('-').unwrap();

        ranges.push((min.parse().unwrap(), max.parse().unwrap()));
    }

    // Sort ranges by their min-values, ascending, breaking ties with their
    // max-values, ascending, to ensure smaller ranges come first.
    ranges.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });



    let mut number_of_fresh_ids = 0;

    let mut cur = ranges[0];

    for range in ranges.iter().skip(1) {


        // println!("{}", range.1-range.0+1);
        if range.0 <= cur.1 && range.1 >= cur.1 {
            // println!("overlap");
            // dbg!(range, cur);
            // Ranges overlap.
            cur.1 = range.1;
        } else {
            // No overlap. Finish current range and start new one.
            // println!("Finish {:?}", cur);
            number_of_fresh_ids += cur.1 - cur.0 + 1;
            cur = *range;
        }
    }

    number_of_fresh_ids += cur.1 - cur.0 + 1;
    // println!("Finish {:?}", cur);

    println!("{number_of_fresh_ids}");
}

// fn part2_dumb() {
//     let mut file = File::open("input").expect("input file not found");

//     let mut input = String::new();
//     file.read_to_string(&mut input)
//         .expect("Failed to read input file");

//     let mut lines = input.trim().split('\n');

//     let mut ranges: Vec<(usize, usize)> = Vec::new();

//     // Parse ranges:
//     while let Some(line) = lines.next() {
//         if line.len() == 0 {
//             // End of ranges, start of IDs.
//             break;
//         }

//         let (min, max) = line.split_once('-').unwrap();

//         ranges.push((min.parse().unwrap(), max.parse().unwrap()));
//     }

// let min = ranges[0].0;
// let max = ranges.iter().map(|r|r.1).max();

// dbg!(min,max);

//     // let mut fresh_ids: Vec<usize> = ranges.into_iter().flat_map(|r| r.0..=r.1).collect();

//     // fresh_ids.sort();
//     // fresh_ids.dedup();

//     // println!("{}", fresh_ids.len());
// }

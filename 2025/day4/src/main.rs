use std::{fs::File, io::Read};

fn main() {
    part2();
}

fn part1() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut grid = Vec::with_capacity(input.len());

    let mut columns = 0;
    let mut rows = 0;

    for line in input.trim().split('\n') {
        grid.extend_from_slice(line.as_bytes());
        columns = line.len();
        rows += 1;
    }

    let mut grid_copy = grid.clone();

    let mut number_of_accessible_cells = 0;

    for (cell, &contents) in grid.iter().enumerate() {
        if contents != b'@' {
            continue;
        }

        let accessible = get_number_of_occupied_neighbours(cell, &grid, columns, rows) < 4;

        if accessible {
            number_of_accessible_cells += 1;
            grid_copy[cell] = b'X';
        }
    }

    let debug_grid = grid_copy
        .chunks(columns)
        .map(|c| str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", debug_grid);

    println!("{number_of_accessible_cells}");
}

fn part2() {
    let mut file = File::open("input").expect("input file not found");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");

    let mut grid = Vec::with_capacity(input.len());

    let mut columns = 0;
    let mut rows = 0;

    for line in input.trim().split('\n') {
        grid.extend_from_slice(line.as_bytes());
        columns = line.len();
        rows += 1;
    }

    let mut removed_count = 0;

    loop {
        let mut changed = false;

        for i in 0..grid.len() {
            if grid[i] != b'@' {
                continue;
            }

            let accessible = get_number_of_occupied_neighbours(i, &grid, columns, rows) < 4;

            if accessible {
                removed_count += 1;
                grid[i] = b'.';
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    println!("{removed_count}");
}

fn get_number_of_occupied_neighbours(cell: usize, grid: &[u8], columns: usize, rows: usize) -> u32 {
    let mut count = 0;

    let col = cell % columns;
    let row = cell / rows;

    if row != 0 {
        // Top-left
        if col != 0 {
            if grid[(row - 1) * rows + col - 1] == b'@' {
                count += 1;
            }
        }

        // Top
        if grid[(row - 1) * rows + col] == b'@' {
            count += 1;
        }

        // Top-right
        if col != columns - 1 {
            if grid[(row - 1) * rows + col + 1] == b'@' {
                count += 1;
            }
        }
    }

    // Left
    if col != 0 {
        if grid[row * rows + col - 1] == b'@' {
            count += 1;
        }
    }

    // Right
    if col != columns - 1 {
        if grid[row * rows + col + 1] == b'@' {
            count += 1;
        }
    }

    if row != rows - 1 {
        // Bottom-left
        if col != 0 {
            if grid[(row + 1) * rows + col - 1] == b'@' {
                count += 1;
            }
        }

        // Bottom
        if grid[(row + 1) * rows + col] == b'@' {
            count += 1;
        }

        // Bottom-right
        if col != columns - 1 {
            if grid[(row + 1) * rows + col + 1] == b'@' {
                count += 1;
            }
        }
    }

    count
}

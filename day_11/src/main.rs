use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const GRID_SIZE: usize = 10;

fn main() {
    let mut grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    let reader = BufReader::new(File::open("input.txt").expect("open file"));
    for (idx, line) in reader.lines().flatten().enumerate() {
        for (jdx, x) in line
            .split_terminator("")
            .skip(1)
            .map(|i| i.parse().unwrap())
            .enumerate()
        {
            grid[idx][jdx] = x;
        }
    }

    let time = Instant::now();
    let mut flashes: i32 = 0;
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let mut all_flash = 0;
    let mut iterations = 0;
    while all_flash < 100 {
        all_flash = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                process_fish(i, j, &mut grid, &mut stack);
            }
        }

        while !stack.is_empty() {
            let (row, col) = stack.pop().unwrap();
            process_fish(row as usize, col as usize, &mut grid, &mut stack);
        }

        for row in &mut grid {
            for j in row {
                if *j > 9 {
                    all_flash += 1;
                    flashes += 1;
                    *j = 0;
                }
            }
        }
        iterations += 1;
    }
    println!("Time: {:?}", time.elapsed());
    println!("Flashes: {}", flashes);
    println!("Iterations for all flash: {}", iterations);
}

fn process_fish(
    row: usize,
    col: usize,
    grid: &mut [[i32; GRID_SIZE]; GRID_SIZE],
    stack: &mut Vec<(i32, i32)>,
) {
    grid[row][col] += 1;
    if grid[row][col] == 10 {
        stack_neighbors(row as i32, col as i32, stack);
    }
}

fn stack_neighbors(row: i32, col: i32, stack: &mut Vec<(i32, i32)>) {
    if is_valid_coords(row - 1, col - 1) {
        stack.push((row - 1, col - 1))
    }
    if is_valid_coords(row - 1, col) {
        stack.push((row - 1, col))
    }
    if is_valid_coords(row - 1, col + 1) {
        stack.push((row - 1, col + 1))
    }
    if is_valid_coords(row, col - 1) {
        stack.push((row, col - 1))
    }
    if is_valid_coords(row, col + 1) {
        stack.push((row, col + 1))
    }
    if is_valid_coords(row + 1, col - 1) {
        stack.push((row + 1, col - 1))
    }
    if is_valid_coords(row + 1, col) {
        stack.push((row + 1, col))
    }
    if is_valid_coords(row + 1, col + 1) {
        stack.push((row + 1, col + 1))
    }
}

fn is_valid_coords(i: i32, j: i32) -> bool {
    if i < 0 || j < 0 || i >= GRID_SIZE as i32 || j >= GRID_SIZE as i32 {
        return false;
    }
    true
}

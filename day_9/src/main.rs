use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const GRID_SIZE: usize = 100;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("unable to open file"));
    let mut grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
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

    let mut local_mins_dfs: Vec<i32> = Vec::new();
    let mut visited: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];

    let mut start = Instant::now();
    let local_mins_linear: Vec<i32> = linear(&grid);
    let linear_duration = start.elapsed();

    start = Instant::now();
    dfs(0, 0, &grid, &mut local_mins_dfs, &mut visited);
    let dfs_duration = start.elapsed();

    let lin_ans = local_mins_linear.iter().fold(0, |acc, val| acc + val + 1);
    println!("linear danger: {}", lin_ans);
    println!("linear duration: {:?}", linear_duration);

    let dfs_ans = local_mins_dfs.iter().fold(0, |acc, val| acc + val + 1);
    println!("dfs danger: {}", dfs_ans);
    println!("dfs duration: {:?}", dfs_duration);
}

fn linear(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> Vec<i32> {
    let mut local_mins: Vec<i32> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_local_min(i, j, grid) {
                local_mins.push(grid[i][j]);
            }
        }
    }
    local_mins
}

fn dfs(
    i: i32,
    j: i32,
    grid: &[[i32; GRID_SIZE]; GRID_SIZE],
    local_mins: &mut Vec<i32>,
    visited: &mut [[bool; GRID_SIZE]; GRID_SIZE],
) {
    visited[i as usize][j as usize] = true;
    if is_local_min(i as usize, j as usize, grid) {
        local_mins.push(grid[i as usize][j as usize]);
    }
    if is_valid_coords(i - 1, j, visited) {
        dfs(i - 1, j, grid, local_mins, visited);
    }
    if is_valid_coords(i + 1, j, visited) {
        dfs(i + 1, j, grid, local_mins, visited);
    }
    if is_valid_coords(i, j + 1, visited) {
        dfs(i, j + 1, grid, local_mins, visited);
    }
    if is_valid_coords(i, j - 1, visited) {
        dfs(i, j - 1, grid, local_mins, visited);
    }
}

fn is_local_min(i: usize, j: usize, grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> bool {
    let mut up = i32::MAX;
    if let Some(idx) = i.checked_sub(1) {
        up = grid[idx][j];
    }

    let mut down = i32::MAX;
    if let Some(row) = grid.get(i + 1) {
        down = row[j];
    }

    let mut right = i32::MAX;
    if let Some(col) = grid.get(i).unwrap().get(j + 1) {
        right = *col;
    }

    let mut left = i32::MAX;
    if let Some(jdx) = j.checked_sub(1) {
        left = grid[i][jdx];
    }

    let pos = grid[i][j];
    pos < up && pos < down && pos < right && pos < left
}

fn is_valid_coords(i: i32, j: i32, visited: &[[bool; GRID_SIZE]; GRID_SIZE]) -> bool {
    if i < 0 || j < 0 || i >= GRID_SIZE as i32 || j >= GRID_SIZE as i32 {
        return false;
    }
    if visited[i as usize][j as usize] {
        return false;
    }
    true
}

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const GRID_SIZE: usize = 100;

fn main() {
    let mut time = Instant::now();
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
    println!("Time to create initial state: {:?}", time.elapsed());

    time = Instant::now();
    let lin_ans = linear(&grid).iter().fold(0, |acc, val| acc + val + 1);
    println!("linear danger: {}", lin_ans);
    println!("linear duration: {:?}", time.elapsed());

    let mut local_mins_dfs: Vec<i32> = Vec::new();
    let mut visited: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];
    time = Instant::now();
    dfs(0, 0, &grid, &mut local_mins_dfs, &mut visited);
    let dfs_duration = time.elapsed();
    let dfs_ans = local_mins_dfs.iter().fold(0, |acc, val| acc + val + 1);
    println!("dfs danger: {}", dfs_ans);
    println!("dfs duration: {:?}", dfs_duration);

    time = Instant::now();
    visited = [[false; GRID_SIZE]; GRID_SIZE];
    let basin_count = count_basins(&grid, &mut visited);
    let basin_duration = time.elapsed();
    println!("count basins: {}", basin_count);
    println!("basin duration: {:?}", basin_duration);
}

fn count_basins(
    grid: &[[i32; GRID_SIZE]; GRID_SIZE],
    visited: &mut [[bool; GRID_SIZE]; GRID_SIZE],
) -> i32 {
    let mut basins: Vec<i32> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited[i][j] && grid[i][j] != 9 {
                basins.push(basin_dfs(i as i32, j as i32, grid, visited));
            }
        }
    }
    basins.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let mut sum = 1;
    basins.iter().fold(0, |acc, basin| {
        if acc < 3 {
            sum *= basin
        }
        acc + 1
    });
    sum
}

fn basin_dfs(
    i: i32,
    j: i32,
    grid: &[[i32; GRID_SIZE]; GRID_SIZE],
    visited: &mut [[bool; GRID_SIZE]; GRID_SIZE],
) -> i32 {
    if !is_valid_coords(i, j, visited) {
        return 0;
    }
    visited[i as usize][j as usize] = true;
    if grid[i as usize][j as usize] == 9 {
        return 0;
    }
    1 + basin_dfs(i + 1, j, grid, visited)
        + basin_dfs(i - 1, j, grid, visited)
        + basin_dfs(i, j - 1, grid, visited)
        + basin_dfs(i, j + 1, grid, visited)
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
    if !is_valid_coords(i, j, visited) {
        return;
    }
    visited[i as usize][j as usize] = true;
    if is_local_min(i as usize, j as usize, grid) {
        local_mins.push(grid[i as usize][j as usize]);
    }

    dfs(i - 1, j, grid, local_mins, visited);
    dfs(i + 1, j, grid, local_mins, visited);
    dfs(i, j + 1, grid, local_mins, visited);
    dfs(i, j - 1, grid, local_mins, visited);
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

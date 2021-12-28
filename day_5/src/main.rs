use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

struct Coords {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Coords {
    pub fn new(v: Vec<&str>) -> Self {
        Self {
            x1: v[0].parse().unwrap(),
            y1: v[1].parse().unwrap(),
            x2: v[2].parse().unwrap(),
            y2: v[3].parse().unwrap(),
        }
    }
}

fn main() {
    part_one();
}

fn part_one() {
    let reader = open_file("input.txt");

    let mut cood_vec: Vec<Coords> = Vec::new();
    for r_line in reader.lines() {
        if let Ok(mut line) = r_line {
            line = line.split_whitespace().collect();
            let points: Vec<&str> = line.split("->").collect();
            let coord_array: Vec<&str> = points
                .iter()
                .flat_map(|p_s| p_s.split(",").collect::<Vec<&str>>())
                .collect();
            cood_vec.push(Coords::new(coord_array));
        }
    }

    let timer = Instant::now();
    let matrix = yes(&cood_vec);
    let mut count: u32 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] >= 2 {
                count += 1;
            }
        }
    }
    println!("time: {:?}", timer.elapsed());
    println!("{}", count);
}
fn yes(cord_vec: &Vec<Coords>) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    for c in cord_vec {
        if c.x1 == c.x2 {
            if c.y1 > c.y2 {
                for it in c.y2..c.y1 + 1 {
                    matrix[it as usize][c.x1 as usize] += 1;
                }
            }
            if c.y1 < c.y2 {
                for it in c.y1..c.y2 + 1 {
                    matrix[it as usize][c.x1 as usize] += 1;
                }
            }
        } else if c.y1 == c.y2 {
            if c.x1 > c.x2 {
                for it in c.x2..c.x1 + 1 {
                    matrix[c.y1 as usize][it as usize] += 1;
                }
            } else {
                for it in c.x1..c.x2 + 1 {
                    matrix[c.y1 as usize][it as usize] += 1;
                }
            }
        } else {
            let mut i = c.x1;
            let mut j = c.y1;
            if c.x2 > c.x1 && c.y2 > c.y1 {
                while i <= c.x2 && j <= c.y2 {
                    matrix[j as usize][i as usize] += 1;
                    if i <= c.x2 {
                        i += 1
                    }
                    if j <= c.y2 {
                        j += 1
                    }
                }
            } else if c.x2 < c.x1 && c.y2 > c.y1 {
                while i >= c.x2 && j <= c.y2 {
                    matrix[j as usize][i as usize] += 1;
                    if i >= c.x2 {
                        i -= 1
                    }
                    if j <= c.y2 {
                        j += 1
                    }
                }
            } else if c.x2 > c.x1 && c.y2 < c.y1 {
                while i <= c.x2 && j >= c.y2 {
                    matrix[j as usize][i as usize] += 1;
                    if i <= c.x2 {
                        i += 1
                    }
                    if j >= c.y2 {
                        j -= 1
                    }
                }
            } else {
                while i >= c.x2 && j >= c.y2 {
                    matrix[j as usize][i as usize] += 1;
                    if i >= c.x2 {
                        i -= 1
                    }
                    if j >= c.y2 {
                        j -= 1
                    }
                }
            }
        }
    }
    matrix
}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_one();
}

fn part_one() {
    let mut string_buffer = String::new();
    match open_file("test_input.txt").read_line(&mut string_buffer) {
        Ok(_) => {}
        Err(e) => panic!("Failed reading line {:?}", e),
    }

    let initial_state: Vec<u32> = string_buffer
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let initial_day = 0;

    let output_fishes = nick_cage(initial_state, initial_day);

    println!("output fishes: {}", output_fishes.len());
}

fn nick_cage(mut fishes: Vec<u32>, mut day: u32) -> Vec<u32> {
    if day == 80 {
        return fishes;
    }
    for i in 0..fishes.len() {
        if fishes[i] == 0 {
            fishes.push(8);
            fishes[i] = 6;
        } else {
            fishes[i] -= 1;
        }
    }
    day += 1;
    nick_cage(fishes, day)
}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

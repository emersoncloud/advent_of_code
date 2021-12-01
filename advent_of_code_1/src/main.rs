use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let reader = open_file("input.txt");

    let mut depths: Vec<u32> = Vec::new();
    for line in reader.lines() {
        if let Ok(ip) = line {
            depths.push(ip.parse().unwrap());
        }
    }

    let mut depth_increases: u32 = 0;
    let mut prev: u32 = std::u32::MAX;
    for depth in depths.iter() {
        if *depth > prev {
            depth_increases += 1;
        }
        prev = *depth;
    }

    println!("depths: {}", depth_increases);
}

fn part_two() {
    let reader = open_file("input.txt");

    let mut depths: Vec<u32> = Vec::new();
    for line in reader.lines() {
        if let Ok(ip) = line {
            depths.push(ip.parse().unwrap());
        }
    }

    let mut rolling_depths = 0;

    let mut rolling_one: u32 = depths[2] + depths[1] + depths[0];
    let mut rolling_two: u32;
    for i in 2..depths.len() {
        rolling_two = depths[i] + depths[i-1] + depths[i-2];
        if rolling_two > rolling_one {
            rolling_depths +=1;
        }
        rolling_one = rolling_two;
    }

    println!("rolling depths: {}", rolling_depths);

}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

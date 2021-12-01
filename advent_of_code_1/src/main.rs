use std::fs::File;
use std::io::{prelude::*, BufReader};
 
fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    // for kyle to show me how vscode works

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
    println!("{}", depth_increases);
    
}

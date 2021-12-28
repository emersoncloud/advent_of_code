use std::fs;
use std::time::{Instant};

fn main() {
    let mut positions: Vec<i64> = fs::read_to_string("input.txt").expect("bad file")
        .split(',').map(|s|{ s.parse().unwrap() }).collect();

    let start = Instant::now();
    positions.sort_unstable();

    let median = positions[positions.len()/2];
    let mut acc = 0;
    for pos in &positions {
        acc += (median - pos).abs();
    }

    let mut summy = 0;
    for pos in &positions {
        summy += pos;
    }
    let average: f64 = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let average_rounded = average.round() as i64;
    let average_rounded_down = average.floor() as i64;
    let mut average_acc = 0;
    for pos in &positions {
        let distance = (average_rounded_down - pos).abs();
        average_acc += (distance * (distance +1)) / 2;
    }
    // 96592275
    // 96592329



    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    println!("median: {}", median);
    println!("median_acc: {}", acc);
    println!("average: {}", average);
    println!("average_rounded: {}", average_rounded);
    println!("average accumulated: {}", average_acc);
}

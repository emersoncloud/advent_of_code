use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fs::File;
use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {
    // recursive_version();
    array_version();
}

fn array_version() {
    let start = Instant::now();
    let mut sam_state: Vec<u64> = vec![0,192,28,27,32, 21, 0, 0, 0];

    for idx in 0..256 {
        let mut new_state = vec![0; 9];
        let mut six: u64 = 0;
        let mut eight:u64 = 0;
        for i in 0..sam_state.len() {
            if i == 0 {
                six += sam_state[i];
                eight += sam_state[i];
                new_state[0] = sam_state[1];
            } else {
                new_state[i-1] = sam_state[i];
            }
        }
        new_state[6] += six;
        new_state[8] += eight;
        sam_state = new_state;
    }

    let sum: u64 =  sam_state.iter().sum();

    println!("sum: {}", sum);
    let duration = start.elapsed();
    println!("duration: {:?}", duration);
}

fn recursive_version() {
    let mut string_buffer = fs::read_to_string("input.txt").expect("bad file");
    let state:Vec<u64> = string_buffer.split(',').map(|s| {
        s.parse().unwrap()
    }
    ).collect();

    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let mut sum: u64 = 0;

    let start = Instant::now();
    for i in 0..state.len() {
        sum += recurse(state[i], 0, &mut cache);
    }
    println!("{}", sum);
    let duration = start.elapsed();
    println!("duration: {:?}", duration);
}

// days then fish lives
fn recurse(lives: u64, days: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if days == 256 {return 1;}

    if let Some(fish_count) = cache.get(&(days, lives)) {
            return *fish_count;
    }

    if lives == 0 {
        let temp1 = recurse(6, days+1, cache);
        let temp2 = recurse(8, days+1, cache);
        cache.insert((days, lives),temp1+temp2);
        return *cache.get(&(days, lives)).unwrap();
    }

    let temp1 = recurse(lives -1, days+1, cache);
    cache.insert((days, lives), temp1);
    return *cache.get(&(days, lives)).unwrap();
}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}
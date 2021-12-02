use std::fs::File;
use std::io::{prelude::*, BufReader};
use crate::Sss::{Down, Forward, Up};

fn main() {
    part_one();
    part_two();
}

enum Sss {
    Forward,
    Up,
    Down,
}

impl Sss {
    fn from(s: &str) -> Sss {
        match s {
            "forward" => Forward,
            "up" => Up,
            "down" => Down,
            otherwise => panic!("invalid enum variant"),
        }
    }
}

struct Direction {
    direction: Sss,
    magnitude: i32,
}


fn part_one() {
    let reader = open_file("input.txt");

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in reader.lines() {
        if let Ok(ip) = line {
            let d :Direction = create_direction(&ip);
            match (d.direction, d.magnitude) {
                    (Forward, x) => horizontal += x,
                    (Down, y) => depth += y,
                    (Up, y) => depth -= y,

            }
        }
    }
    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("combined: {}", horizontal * depth);

}

fn part_two() {
    let reader = open_file("input.txt");
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in reader.lines() {
        if let Ok(ip) = line {
            let d :Direction = create_direction(&ip);
            match (d.direction, d.magnitude) {
                (Forward, x) => {
                    horizontal += x;
                    depth += aim * x;
                },
                (Down, y) => aim += y,
                (Up, y) => aim -= y,

            }
        }
    }
    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("combined: {}", horizontal * depth);


}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

fn split_words(s: &String) -> Vec<&str> {
    let s_bytes = s.as_bytes();
    let mut words: Vec<&str> = vec![];

    let mut last_idx: usize = 0;

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            words.push(&s[last_idx..i]);
            last_idx = i+1;
        }
    }
    words.push(&s[last_idx..]);
    words
}

fn create_direction(s: &String) -> Direction {
    let words  = split_words(s);
    let direction = Sss::from(words[0]);
    let magnitude = words[1].parse().unwrap();

    Direction {
        direction,
        magnitude
    }
}
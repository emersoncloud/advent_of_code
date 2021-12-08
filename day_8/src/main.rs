use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Display {
    input: Vec<String>,
    output: Vec<String>,
}

impl Display {
    fn _new(input: Vec<String>, output: Vec<String>) -> Self {
        Display {input, output}
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.input, self.output)
    }
}

fn main() {
    let reader = BufReader::new(
        File::open("input.txt").expect("unable to open file")
    );

    let mut displays: Vec<Display> = Vec::new();
    for line in reader.lines().flatten() {
            let input_output: Vec<&str> = line.split('|').collect();
            let input: Vec<String> = input_output.get(0).unwrap().strip_suffix(' ').unwrap().split(' ').map(String::from).collect();
            let output: Vec<String> = input_output.get(1).unwrap().strip_prefix(' ').unwrap().split(' ').map(String::from).collect();

            displays.push(Display {input, output});
    }

    let mut count_unique_digis = 0;
    for display in displays {
        for s in &display.output {
            if s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7 {
                count_unique_digis += 1;
            }
        }
    }

    println!("unique digis: {}", count_unique_digis);
}

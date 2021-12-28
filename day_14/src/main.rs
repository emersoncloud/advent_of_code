use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(File::open("test_input.txt").unwrap());

    let mut input_string: String = String::new();
    let mut bogus: String = String::new();
    reader.read_line(&mut input_string).expect("work please!");
    reader.read_line(&mut bogus).expect("please work");

    input_string.pop();

    let mut map: HashMap<String, String> = HashMap::new();
    for line in reader.lines().flatten() {
        let input = line.split("->")
            .collect::<Vec<&str>>();

        map.insert(
            String::from(input.get(0).unwrap().strip_suffix(' ').unwrap()),
            String::from(input.get(1).unwrap().strip_prefix(' ').unwrap())
        );
    }

    part_one(input_string.as_str(), map.clone(), 10);
    part_two(input_string.as_str(), &map.clone(), 2);
}

fn part_two(pass_in_string: &str, map: &HashMap<String, String>, iters: i32) {
    let input_string: String = pass_in_string.to_string();
    let timer = Instant::now();
    let mut output: HashMap<String, i64> = HashMap::new();
    let mut input: HashMap<String, i64> = HashMap::new();

    for i in 0..(input_string.len() - 1) {
        let pair = &input_string[i..i+2];
        if let Some(current_val) = input.get(pair) {
            input.insert(String::from(pair), current_val + 1);
        } else {
            input.insert(String::from(pair), 1);
        }
    }

    for _i in 0..iters {
        output.clear();
        input.iter().for_each(|pair| {
            let new_strigns = get_strings(pair.0,  map);
            for new in new_strigns {
                if let Some(current_val) = output.get(new.as_str()) {
                    output.insert(new, current_val + *pair.1);
                } else {
                    output.insert(new, *pair.1);
                }
            }
        });
        input = output.clone();
    }

    let mut count_chars: HashMap<char, i64> = HashMap::new();
    output.iter().for_each(|o| {
        for c in o.0.as_bytes() {
            if let Some(count)  = count_chars.get(&(*c as char)) {
                count_chars.insert(*c as char, count + *o.1);
            } else {
                count_chars.insert(*c as char, *o.1);
            }
        }
    });

    let start_char = *input_string.as_bytes().get(0).unwrap() as char;
    let vec = input_string.as_bytes();
    let end_char = *input_string.as_bytes().get(vec.len()-1).unwrap() as char;
    let mut real_const_chars: HashMap<char, i64> = HashMap::new();
    for c in count_chars.iter() {
        if *c.0 == start_char || *c.0 == end_char {
            real_const_chars.insert(*c.0, c.1/2 + 1);
        } else {
            real_const_chars.insert(*c.0, c.1/2);
        }
    }

    print_output(real_const_chars, timer);
}

fn print_output(map: HashMap<char, i64>, timer: Instant) {
    let mut biggest: i64 = i64::MIN;
    let mut smallest: i64 = i64::MAX;
    for (_, value) in map.iter() {
        if *value > biggest {
            biggest = *value;
        }
        if *value < smallest {
            smallest = *value;
        }
    }
    println!("character counts: {:?}", map);
    println!("difference: big, small, diff {}, {}, {}", biggest, smallest, biggest - smallest);
    println!("timer: {:?}", timer.elapsed());
}

fn get_strings(pair: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut string_one = String::new();

    string_one.push_str(pair[0..1].to_string().as_str());
    string_one.push_str(map.get(pair).unwrap());

    let mut string_two = String::new();
    string_two.push_str(map.get(pair).unwrap());
    string_two.push_str(pair[1..2].to_string().as_str());
    return vec![string_one, string_two];
}

fn part_one(pass_in_string: &str, map: HashMap<String, String>, iters: i32) {
    let mut input_string = pass_in_string.to_string();
    let timer = Instant::now();
    let mut output_string: String = String::new();
    for _i in 0..iters {
        output_string.clear();
        for i in 0..(input_string.len() - 1) {
            let input = &input_string[i..i+2];
            if i == 0 {
                output_string.push_str(&input_string[i..i+1]);
            }
            if let Some(val) = map.get(&input_string[i..i+2]) {
                output_string.push_str(val);
            }

            output_string.push_str(&input_string[i+1..i+2]);
        }
        input_string = output_string.clone();
    }

    let mut count_chars: HashMap<char, i64> = HashMap::new();
    output_string.as_bytes().iter().for_each( |c| {
        if let Some(count) = count_chars.get( &(*c as char)) {
            count_chars.insert(*c as char, count + 1);
        } else {
            count_chars.insert(*c as char, 1);
        }
    });

    print_output(count_chars, timer);
}


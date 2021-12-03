use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let reader = open_file("input.txt");
    let mut registers: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in reader.lines() {
        if let Ok(bits) = line {
            let bit_vec = get_num_array(&bits);
            for (i, &item) in bit_vec.iter().enumerate() {
                if item == 1 {
                    registers[i] += 1;
                } else {
                    registers[i] -= 1;
                }
            }
        }
    }

    let delta: Vec<u32> = registers
        .iter()
        .map(|x| if *x > 0 { 1 } else { 0 })
        .collect();

    let epsilon: Vec<u32> = registers
        .iter()
        .map(|x| if *x > 0 { 0 } else { 1 })
        .collect();

    for x in &delta {
        print!("{}", x);
    }
    println!("");
    for x in &epsilon {
        print!("{}", x);
    }

    let mut d_string: String = String::from("");
    let mut e_string: String = String::from("");

    delta.iter().for_each(|d| {
        d_string += &*d.to_string();
    });
    epsilon.iter().for_each(|d| {
        e_string += &*d.to_string();
    });
    println!("");

    println!("{}", d_string);
    println!("{}", e_string);

    let d_num = isize::from_str_radix(&*d_string, 2).unwrap();
    println!("{}", d_num);
    let e_num = isize::from_str_radix(&*e_string, 2).unwrap();
    println!("{}", e_num);
    println!("{}", d_num * e_num);
}

fn part_two() {
    let reader = open_file("input.txt");
    let mut registers: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut delta_input: Vec<String> = vec![];
    let mut epsilon_input: Vec<String> = vec![];

    for line in reader.lines() {
        if let Ok(bits) = line {
            let bits2 = String::from(&bits);
            let bits3 = String::from(&bits);
            delta_input.push(bits2);
            epsilon_input.push(bits3);
            let bit_vec = get_num_array(&bits);
            for (i, &item) in bit_vec.iter().enumerate() {
                if item == 1 {
                    registers[i] += 1;
                } else {
                    registers[i] -= 1;
                }
            }
        }
    }

    let mut delta: Vec<u32> = registers
        .iter()
        .map(|x| if *x > 0 { 1 } else { 0 })
        .collect();

    let mut d_origin = String::from("");
    delta.iter().for_each(|d| {
        d_origin += &*d.to_string();
    });
    println!("Original delta: {}", d_origin);

    let mut start = 0;
    let mut i = 0;
    while i < delta.len() {
        let mut d_s: String = String::from("");

        delta.iter().for_each(|d| {
            d_s += &*d.to_string();
        });
        println!("Delta in use: {}", d_s);

        let bit = delta[i];
        let mut idx = 0;
        while idx < delta_input.len() {
            let d_in = &delta_input[idx];
            let d_num = d_in[start..i + 1].parse::<u32>().unwrap();
            if d_num != bit {
                delta_input.remove(idx);
                if (delta_input.len() == 1) {
                    break;
                }
            } else {
                idx += 1;
            }
        }
        if (delta_input.len() == 1) {
            break;
        }
        i += 1;
        start += 1;
        delta = compute_delta(&delta_input);
    }


    let mut epsilon: Vec<u32> = registers
        .iter()
        .map(|x| if *x > 0 { 0 } else { 1 })
        .collect();

    let mut e_origin = String::from("");
    epsilon.iter().for_each(|e| {
        e_origin += &*e.to_string();
    });
    println!("Original eps: {}", e_origin);

    let mut j = 0;
    start = 0;
    while j < epsilon.len() {
        let mut e_s: String = String::from("");

        epsilon.iter().for_each(|d| {
            e_s += &*d.to_string();
        });
        println!("Eps in use: {}", e_s);

        let bit = epsilon[j];
        let mut idx = 0;
        while idx < epsilon_input.len() {
            let e_in = &epsilon_input[idx];
            let e_num = e_in[start..(j+1)].parse::<u32>().unwrap();
            if e_num != bit {
                epsilon_input.remove(idx);
                if (epsilon_input.len() == 1) {
                    break;
                }
            } else {
                idx += 1;
            }
        }
        if (epsilon_input.len() == 1) {
            break;
        }
        j += 1;
        start += 1;
        epsilon = compute_eps(&epsilon_input);
    }


    let d_num = isize::from_str_radix(&*delta_input[0], 2).unwrap();
    println!("found delta number in binary\t{:b}", d_num);
    let e_num = isize::from_str_radix(&*epsilon_input[0], 2).unwrap();
    println!("found eps number in binary\t\t{:b}", e_num);
    println!("{}", d_num * e_num);
}

fn compute_delta(input: &Vec<String>) -> Vec<u32> {
    let mut registers: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for bits in input {
        let bit_vec = get_num_array(&bits);
        for (i, &item) in bit_vec.iter().enumerate() {
            if item == 1 {
                registers[i] += 1;
            } else {
                registers[i] -= 1;
            }
        }
    }

    registers
        .iter()
        .map(|x| if *x >= 0 { 1 } else { 0 })
        .collect()
}

fn compute_eps(input: &Vec<String>) -> Vec<u32> {
    let mut registers: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for bits in input {
        let bit_vec = get_num_array(&bits);
        for (i, &item) in bit_vec.iter().enumerate() {
            if item == 1 {
                registers[i] += 1;
            } else {
                registers[i] -= 1;
            }
        }
    }

    registers
        .iter()
        .map(|x| if *x >= 0 { 0 } else { 1 })
        .collect()
}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

fn get_num_array(st: &str) -> Vec<u32> {
    st.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn testing() {
    // input length is 3
    let x1 = 0b1010;
    let x2 = 0b1010;
    let x3 = 0b0100;

    let mut bit_one = 0b1;
    let bit_one = bit_one << 0b1;
    let bit_one = bit_one >> 0b1;
    let bit_one = bit_one << 0b1;

    let x_test = x1 ^ x2;
    println!("{:b}", x_test);
}

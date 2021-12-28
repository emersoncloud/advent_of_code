use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::{Instant, Duration};

struct Display {
    input: Vec<HashSet<char>>,
    output: Vec<String>,
    answer: Vec<i32>,
    ans_hash: HashMap<i32, char>,
    other_way_ans: HashMap<char, i32>,
    possibilities: HashMap<i32, HashSet<char>>,
}

fn positions_to_int(positions: &str) -> &str {
    let map: HashMap<&str, &str> = HashMap::from([
        ("0123456", "8"),
        ("013456", "6"),
        ("012456", "0"),
        ("012356", "9"),
        ("02356", "3"),
        ("01356", "5"),
        ("02346", "2"),
        ("1235", "4"),
        ("025", "7"),
        ("25", "1"),
    ]);
    *map.get(positions).unwrap()
}

impl Display {
    fn new(input: Vec<HashSet<char>>, output: Vec<String>) -> Self {
        Display {
            input,
            output,
            answer: Vec::with_capacity(7),
            ans_hash: HashMap::with_capacity(7),
            other_way_ans: HashMap::with_capacity(7),
            possibilities: HashMap::with_capacity(7),
        }
    }

    fn eclone(&self) -> Display {
        Display {
            input: self.input.clone(),
            output: self.output.clone(),
            answer: self.answer.clone(),
            ans_hash: self.ans_hash.clone(),
            other_way_ans: self.other_way_ans.clone(),
            possibilities: self.possibilities.clone(),
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.input, self.output)
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("unable to open file"));
    let mut bestouptputdisplay: Vec<Display> = Vec::new();

    let mut displays: Vec<Display> = Vec::new();
    for line in reader.lines().flatten() {
        let input_output: Vec<&str> = line.split('|').collect();
        let input: Vec<String> = input_output
            .get(0)
            .unwrap()
            .strip_suffix(' ')
            .unwrap()
            .split(' ')
            .map(String::from)
            .collect();
        let output: Vec<String> = input_output
            .get(1)
            .unwrap()
            .strip_prefix(' ')
            .unwrap()
            .split(' ')
            .map(String::from)
            .collect();

        let input_set: Vec<HashSet<char>> = input.into_iter().map(|i| {
            let mut set: HashSet<char> = HashSet::new();
            for c in i.chars() {
                set.insert(c);
            }
            set
        }).collect::<Vec<HashSet<char>>>();

        let d = Display::new(input_set, output);
        displays.push(d);
    }

    let timer = Instant::now();
    let mut count_unique_digis = 0;
    for display in &displays {
        for s in &display.output {
            if s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7 {
                count_unique_digis += 1;
            }
        }
    }

    let mut two_set: HashSet<char> = HashSet::new();
    let mut seven_set: HashSet<char> = HashSet::new();
    let mut four_set: HashSet<char> = HashSet::new();
    let mut six_set: Vec<HashSet<char>> = Vec::new();
    let mut five_set: Vec<HashSet<char>> = Vec::new();
    let mut three_set: HashSet<char> = HashSet::new();

    for mut display in displays {
        let mut done = false;
        while !done {
            for input in &display.input {
                if input.len() == 2 {
                    two_set.extend(input);
                }
                if input.len() == 7 {
                    seven_set.extend(input);
                }
                if input.len() == 4 {
                    four_set.extend(input);
                }
                if input.len() == 6 {
                    six_set.push(input.clone());
                }
                if input.len() == 5 {
                    five_set.push(input.clone());
                }
                if input.len() == 3 {
                    three_set.extend(input);
                }
            }

            display.ans_hash.insert(0, three_set.symmetric_difference(&two_set).next().unwrap().clone());
            display.other_way_ans.insert(*display.ans_hash.get(&0).unwrap(), 0);


            display.possibilities.insert(2, two_set.clone());
            display.possibilities.insert(5, two_set.clone());

            let mut three_four_diff_set:HashSet<char> = HashSet::new();
            for x in four_set.symmetric_difference(&three_set) {
                three_four_diff_set.insert(char::from(*x as u8));
            }

            let mut three_four_diff_real_set:HashSet<char> = HashSet::new();
            for x in three_four_diff_set {
                for key in display.other_way_ans.keys() {
                    if x != *key {
                        three_four_diff_real_set.insert(x);
                    }
                }
            }

            display.possibilities.insert(1, three_four_diff_real_set.clone());
            display.possibilities.insert(3, three_four_diff_real_set.clone());


            let mut four_seven_diff_set:HashSet<char> = HashSet::new();
            for x in seven_set.symmetric_difference(&four_set) {
                four_seven_diff_set.insert(char::from(*x as u8));
            }

            let mut four_seven_diff_set_real: HashSet<char> = HashSet::new();
            for x in four_seven_diff_set {
                for key in display.other_way_ans.keys() {
                    if x != *key {
                        four_seven_diff_set_real.insert(x);
                    }
                }
            }
            display.possibilities.insert(4, four_seven_diff_set_real.clone());
            display.possibilities.insert(6, four_seven_diff_set_real.clone());

            let mut six_zero_one: HashSet<char> = HashSet::new();
            let mut siz_one_two: HashSet<char> = HashSet::new();
            for x in six_set.get(0).unwrap().symmetric_difference(&six_set.get(1).unwrap()) {
                six_zero_one.insert(char::from(*x as u8));
            }

            for x in six_set.get(1).unwrap().symmetric_difference(&six_set.get(2).unwrap()) {
                siz_one_two.insert(char::from(*x as u8));
            }

            let mut six_real: HashSet<char> = siz_one_two.clone();
            for x in six_zero_one {
                if !six_real.contains(&x) {
                    six_real.insert(x);
                }
            }
            display.ans_hash.insert(2, char::from(*display.possibilities.get(&2).unwrap().intersection(&six_real).next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&2).unwrap(), 2);
            six_real.remove(display.ans_hash.get(&2).unwrap());

            display.ans_hash.insert(3, char::from(*display.possibilities.get(&3).unwrap().intersection(&six_real).next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&3).unwrap(), 3);
            six_real.remove(display.ans_hash.get(&3).unwrap());

            display.ans_hash.insert(4, char::from(*six_real.iter().next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&4).unwrap(), 4);

            display.ans_hash.insert(1, char::from(*display.possibilities.get(&1).unwrap().symmetric_difference(&display.ans_hash.values().map(|c| char::from(*c as u8)).collect()).next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&1).unwrap(), 1);

            display.ans_hash.insert(5, char::from(*display.possibilities.get(&5).unwrap().symmetric_difference(&display.ans_hash.values().map(|c| char::from(*c as u8)).collect()).next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&5).unwrap(), 5);

            display.ans_hash.insert(6, char::from(*display.possibilities.get(&6).unwrap().symmetric_difference(&display.ans_hash.values().map(|c| char::from(*c as u8)).collect()).next().unwrap() as u8));
            display.other_way_ans.insert(*display.ans_hash.get(&6).unwrap(), 6);
            done = true;

            two_set.clear();
            seven_set.clear();
            four_set.clear();
            six_set.clear();
            five_set.clear();
            three_set.clear();
            bestouptputdisplay.push(display.eclone());
        }
    }

    let mut acc: Vec<i32> = Vec::new();
    let mut scc_string: String = String::from("");
    let mut big_acc: i32 = 0;
    for display in bestouptputdisplay {
        for st in display.output {
            let mut num_string_d: Vec<i32> = Vec::new();
            let mut num_string = String::from("");
            for ch in st.chars() {
                num_string_d.push(*display.other_way_ans.get(&ch).unwrap());
                // num_string.push_str(display.other_way_ans.get(&ch).unwrap().to_string().as_str());
            }
            num_string_d.sort_unstable();
            for v in num_string_d {
                num_string.push_str(v.to_string().as_str());
            }
            let jack: &str = positions_to_int(num_string.as_str());
            scc_string.push_str(jack)
        }
        big_acc += scc_string.parse::<i32>().unwrap();
        scc_string = String::from("");
    }
    println!("Duration{:?}", timer.elapsed());
    println!("unique digis: {}", count_unique_digis);
    println!("acc: {:?}", big_acc);
}

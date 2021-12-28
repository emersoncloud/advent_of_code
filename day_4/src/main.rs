use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // test_board_solitions();
    part_one();
    // another_test();
}

struct BingoBoard {
    arrangement: Vec<Vec<u32>>,
    winning_options: Vec<HashSet<u32>>,
    has_won: bool,
}

impl BingoBoard {
    pub fn new(arrangement: Vec<Vec<u32>>) -> Self {
        let mut winning_options: Vec<HashSet<u32>> = vec![];
        for i in &arrangement {
            let mut hash: HashSet<u32> = HashSet::new();
            for j in i {
                hash.insert(*j);
            }
            winning_options.push(hash);
        }


        let mut j = 0;
        while &j < &arrangement.len() {
            let mut hash: HashSet<u32> = HashSet::new();
            for i in 0..arrangement.len() {
                let val = arrangement[i][j];
                hash.insert(arrangement[i][j]);
            }
            winning_options.push(hash);
            j += 1;
        }

        Self {
            arrangement,
            winning_options,
            has_won: false
        }
    }
}


fn part_one() {
    let mut reader = open_file("input.txt");
    let mut buffer = String::from("");
    reader.read_line(&mut buffer);

    let numbers_drawn: Vec<u32> = buffer
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|s| {
            let result = match s.parse() {
                Ok(val) => val,
                Err(e) => panic!("failed parsing: {}, {:?}", s, e),
            };
            result
        })
        .collect();
    reader.read_line(&mut buffer);


    let mut boards: Vec<BingoBoard> = Vec::new();

    let mut board_buffer: Vec<Vec<u32>> = vec![];
    for (idx, line) in reader.lines().enumerate() {
        if let Ok(ip) = line {
            if ip == "" {
                if board_buffer.len() < 5 {
                    println!("help");
                }
                boards.push(BingoBoard::new(board_buffer.clone()));
                board_buffer.clear();
                continue;
            }

            board_buffer.push(
              ip.split_whitespace().map(|n| {
                  n.parse().unwrap()
              }).collect()
            );

            if idx == 598 {
                if board_buffer.len() < 5 {
                    println!("help");
                }
                boards.push(BingoBoard::new(board_buffer.clone()));
                board_buffer.clear();
                continue;

            }
        }
    }

    let mut j = 5;
    let mut winning_board: Option<&BingoBoard> = None;
    let mut winning_draws: Option<&[u32]> = None;

    let mut stop = false;
    let mut big_winner: Option<&BingoBoard> = None;

    while j < numbers_drawn.len() && stop != true {
        let n = &numbers_drawn[0..j];
        for it in 0..boards.len() {
            if did_win(&boards[it], n.clone()) {
                boards[it].has_won = true;
                winning_board = Some(&boards[it]);
                winning_draws = Some(n);
                if check_wins(&boards) {
                    stop = true;
                    break;
                }
            }
        }
        j += 1;
    }

    if (winning_board.is_none() || winning_draws.is_none()) {
        panic!("either board or draws is none");
    }
    let sum = sum_unmarked(winning_board.unwrap(), winning_draws.unwrap());
    let last_draw = winning_draws.unwrap().last().unwrap();

    println!("{}", sum * *last_draw);
    println!("done");
}

fn check_wins_old(boards: Vec<BingoBoard>, mut boards_that_won: Vec<bool>) -> Option<BingoBoard>{
    let mut total_wins = 0;
    let mut last_winner: Option<BingoBoard> = None;
    let prev_wins = boards_that_won.clone();
    for i in 0..boards.len() {
        if boards[i].has_won {
            boards_that_won[i] = true;
            total_wins += 1;
        }
    }
    if total_wins == boards_that_won.len() {


    }
    last_winner
}

fn check_wins(boards: &Vec<BingoBoard>) -> bool {
    let mut total_wins = 0;
    for i in 0..boards.len() {
        if boards[i].has_won {
            total_wins += 1;
        }
    }
    println!("{}", total_wins);
    total_wins == boards.len()
}

// now I have to get the last one to win. let me think about how that would work.
// with all of the numbers drawn, record the last one to win

fn sum_unmarked(board: &BingoBoard, draws: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    let draws_set:HashSet<u32> = HashSet::from_iter(draws.iter().cloned());
    for (idx, i) in board.arrangement.iter().enumerate() {
        for (jdx, j) in board.arrangement[idx].iter().enumerate() {
            if !draws_set.contains(&board.arrangement[idx][jdx]) {
                sum += board.arrangement[idx][jdx];
            }
        }
    }

    return sum;
}

fn did_win(board: &BingoBoard, draws: &[u32]) -> bool {
    let draws_set:HashSet<u32> = HashSet::from_iter(draws.iter().cloned());
    for option in &board.winning_options {
        let test: HashSet<u32> = option.clone();
        if test.is_subset(&draws_set) {
            return true;
        }
    }
    return false;
}

fn another_test() {
    let mut x: Vec<Vec<u32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
    ];

    let mut y: Vec<Vec<u32>> = vec![
        vec![60,79,46,9,58],
        vec![97,81, 6,94,84],
        vec![38,40,17,61,29],
        vec![11,28, 0,91,15],
        vec![24,77,34,59,36],
    ];
    let b = BingoBoard::new(y);
    let c = &b;

    let numbers_drawn: Vec<u32> = vec![46, 9, 58, 79, 94, 1, 4, 6, 2, 9, 60];
    let n = &numbers_drawn[..];

    let won = did_win(c, n);
    println!("won: {}", won);
    // assert_eq!(won, true);


}

fn test_board_solitions() {
    let mut x: Vec<Vec<u32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
    ];

    let mut possible_wins: Vec<HashSet<u32>> = vec![];
    for i in &x {
        let mut hash: HashSet<u32> = HashSet::new();
        for j in i {
            hash.insert(*j);
        }
        possible_wins.push(hash);
    }


    let mut j = 0;
    while &j < &x.len() {
        let mut hash: HashSet<u32> = HashSet::new();
        for i in 0..x.len() {
            let val = x[i][j];
            hash.insert(x[i][j]);
        }
        possible_wins.push(hash);
        j += 1;
    }

    println!("debug");


}


fn part_two() {}

fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}

// how do I tell which board is the last to win
// I stop when all boards but one have won

// if 99 boards have won, stop.
// track a has won on my board object
// then each time check if there's one board that hasn't won

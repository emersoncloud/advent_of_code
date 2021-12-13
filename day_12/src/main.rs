use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

struct Graph {
    graph: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Graph {
        let hash = HashMap::new();
        Graph { graph: hash }
    }

    fn add_connection(&mut self, a: String, b: String) {
        if !self.graph.contains_key(&a) {
            self.graph.insert(a.clone(), Vec::new());
        }

        if !self.graph.contains_key(&b) {
            self.graph.insert(b.clone(), Vec::new());
        }

        if a == "start" {
            self.graph.get_mut(&a).unwrap().push(b.clone());
        } else if b == "start" || a == "end" {
            self.graph.get_mut(&b).unwrap().push(a.clone());
        } else if b == "end" {
            self.graph.get_mut(&a).unwrap().push(b.clone());
        } else {
            self.graph.get_mut(&a).unwrap().push(b.clone());
            self.graph.get_mut(&b).unwrap().push(a.clone());
        }
    }
}

fn main() {
    let mut the_graph = Graph::new();
    let mut time = Instant::now();

    let reader = BufReader::new(File::open("test.txt").unwrap());
    for line in reader.lines().flatten() {
        let start_and_end: Vec<&str> = line.split('-').collect();

        let a: String = String::from(start_and_end[0]);
        let b: String = String::from(start_and_end[1]);

        the_graph.add_connection(a, b);
    }
    println!("initialize time{:?}", time.elapsed());

    time = Instant::now();
    let mut sum = 0;
    let mut cache: HashMap<String, i32> = HashMap::new();
    the_graph
        .graph
        .get("start")
        .unwrap()
        .iter()
        .for_each(|node| {
            sum += dfs(node, &the_graph, &mut cache);
        });

    println!("sum {}", sum);
    println!("time{:?}", time.elapsed());
}

fn dfs(string: &str, the_graph: &Graph, cache: &mut HashMap<String, i32>) -> i32 {
    if string == "end" {
        return 1;
    }

    let mut theres_a_two: bool = false;
    let mut do_traverse: bool = false;
    for x in cache.values() {
        if *x > 1 {
            theres_a_two = true;
        }
    }

    if !cache.contains_key(string) {
        do_traverse =  true;
    } else {
        do_traverse = cache.contains_key(string) && !theres_a_two;
    }

    if !do_traverse {
        return 0;
    } else if string.chars().all(char::is_lowercase) {
        if let Some(val) = cache.get(string) {
            cache.insert(String::from(string), val + 1);
        } else {
            cache.insert(String::from(string), 1);
        }
    }

    let mut sum = 0;
    the_graph
        .graph
        .get(string)
        .unwrap()
        .iter()
        .for_each(|node| {
            sum += dfs(node, the_graph, cache);
        });

    if let Some(removed_val) = cache.remove_entry(string) {
        if removed_val.1 == 2 {
            cache.insert(String::from(string), removed_val.1 - 1);
        }
    }
    sum
}

fn check_cache(string: &str, cache: &HashMap<String, i32>) -> bool {
    let mut theres_a_two: bool = false;
    for x in cache.values() {
        if *x > 1 {
            theres_a_two = true;
        }
    }

    if !cache.contains_key(string) {
        return true;
    } else if cache.contains_key(string) && theres_a_two == false {
        return true;
    } else {
        return false
    }

    // if let Some(value) = cache.get(string) {
    //     if *value == 0 {
    //         return true;
    //     }
    // }
    //
    //
    // if !cache.contains_key(string) {
    //     return true;
    // }
    // cache.contains_key(string) && !theres_a_two
}

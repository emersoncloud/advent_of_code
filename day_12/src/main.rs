use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

struct Graph {
    graph: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Graph {
        let mut hash = HashMap::new();
        Graph { graph: hash }
    }

    fn add_connection(&mut self, a: String, b: String) {
        if !self.graph.contains_key(&a) {
            self.graph.insert(a.clone(), HashSet::new());
        }

        if !self.graph.contains_key(&b) {
            self.graph.insert(b.clone(), HashSet::new());
        }

        if a == "start" {
            self.graph.get_mut(&a).unwrap().insert(b.clone());
        } else if b == "start" {
            self.graph.get_mut(&b).unwrap().insert(a.clone());
        } else if a == "end" {
            self.graph.get_mut(&b).unwrap().insert(a.clone());
        } else if b == "end" {
            self.graph.get_mut(&a).unwrap().insert(b.clone());
        } else {
            self.graph.get_mut(&a).unwrap().insert(b.clone());
            self.graph.get_mut(&b).unwrap().insert(a.clone());
        }
    }
}

fn main() {
    let mut the_graph = Graph::new();

    let reader = BufReader::new(File::open("test_input.txt").unwrap());
    for line in reader.lines().flatten() {
        let start_and_end: Vec<&str> = line.split("-").collect();

        let a: String = String::from(start_and_end[0]);
        let b: String = String::from(start_and_end[1]);

        the_graph.add_connection(a, b);
        println!("start and end {:?}", start_and_end);
    }

    let mut sum = 0;
    let mut cache: HashMap<String, i32> = HashMap::new();
    the_graph
        .graph
        .get("start")
        .unwrap()
        .iter()
        .for_each(|node| {
            sum += dfs(node, node.clone(), &the_graph, &mut cache);
        });

    println!("sum {}", sum);
}

fn dfs(
    string: &String,
    string_clone: String,
    the_graph: &Graph,
    cache: &mut HashMap<String, i32>,
) -> i32 {
    if string.as_str() == "end" {
        return 1;
    }

    let cloned_clone = string_clone.clone();

    if !check_cache(&string_clone, cache) {
        return 0;
    } else {
        if string.chars().all(char::is_lowercase) {
            if let Some(val) =  cache.get(&string_clone) {
                cache.insert(string_clone, val +1);
            } else {
                cache.insert(string_clone, 1);
            }
        }
    }

    let mut sum = 0;
    let setset = the_graph.graph.get(string).unwrap();
    print!("\non top level node {}: {:?} ", string, setset);
    the_graph
        .graph
        .get(string)
        .unwrap()
        .iter()
        .for_each(|node| {
            // println!("operating on {}", node);
            sum += dfs(node, node.clone(), &the_graph, cache);

        });

    print!("got a sub-sum of {} while on top level {} \n", sum, string);
    if let Some(removed_val) = cache.remove(string) {
        if removed_val != 0 {
            cache.insert(cloned_clone, removed_val-1);
        }
    }
    return sum;
}

fn check_cache(string: &String, cache: &HashMap<String, i32>) -> bool {
    let mut theres_a_two: bool = false;
    for x in cache.values() {
        if *x > 1 {
            theres_a_two = true;
        }
    }
    let string_copy = string.clone();
    let hash_copy = cache.clone();
    if !cache.contains_key(string.as_str()) {
        return true;
    } else if cache.contains_key(string.as_str()) && theres_a_two == false {
        return true;
        // } else if cache.contains_key(string.as_str()) && theres_a_two == true {
    } else {
        return false;
    }
}

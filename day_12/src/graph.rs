use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<String, Vec<String>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.graph.keys().collect::<HashSet<&String>>() == other.graph.keys().collect::<HashSet<&String>>()
        && self.graph.values().flatten().into_iter().map(|s| s.as_str()).collect::<HashSet<&str>>() ==
            other.graph.values().flatten().into_iter().map(|s| s.as_str()).collect::<HashSet<&str>>()
    }
}

impl Graph {
    pub fn new() -> Graph {
        let hash = HashMap::new();
        Graph { graph: hash }
    }

    pub fn new_from_file(filename: &str) -> Graph {
        let hash = HashMap::new();
        let mut graph = Graph { graph: hash };
        let reader = BufReader::new(File::open(filename).unwrap());
        for line in reader.lines().flatten() {
            let start_and_end: Vec<&str> = line.split('-').collect();

            let a: String = String::from(start_and_end[0]);
            let b: String = String::from(start_and_end[1]);

            graph.add_connection(a, b);
        }
        graph
    }

    pub fn add_connection(&mut self, a: String, b: String) {
        if !self.graph.contains_key(&a) {
            self.graph.insert(a.clone(), Vec::new());
        }

        if !self.graph.contains_key(&b) {
            self.graph.insert(b.clone(), Vec::new());
        }

        if a == "start" || b == "end" {
            self.graph.get_mut(&a).unwrap().push(b.clone());
        } else if b == "start" || a == "end" {
            self.graph.get_mut(&b).unwrap().push(a.clone());
        } else {
            self.graph.get_mut(&a).unwrap().push(b.clone());
            self.graph.get_mut(&b).unwrap().push(a.clone());
        }
    }

    pub fn explore_caves(&self) -> i32 {
        let mut sum = 0;
        let mut cache: HashMap<String, i32> = HashMap::new();
        self.graph.get("start").unwrap().iter().for_each(|node| {
            sum += self.dfs(node, &mut cache);
        });
        sum
    }

    pub fn dfs(&self, string: &str, cache: &mut HashMap<String, i32>) -> i32 {
        if string == "end" {
            return 1;
        }

        let cached_maybe: Option<&i32> = cache.get(string);

        let cache_it: bool = if cached_maybe.is_none() {
            true
        } else {
            cached_maybe.is_some() && !cache.values().any(|x| *x == 2)
        };

        if !cache_it {
            return 0;
        } else if string.chars().all(char::is_lowercase) {
            if cached_maybe.is_some() {
                cache.insert(String::from(string), cached_maybe.unwrap().clone() + 1);
            } else {
                cache.insert(String::from(string), 1);
            }
        }

        let mut sum = 0;
        self.graph.get(string).unwrap().iter().for_each(|node| {
            sum += self.dfs(node, cache);
        });

        // if cached_maybe.is_some() {
        //     if *cached_maybe.unwrap() == 2 {
        //         cache.insert(String::from(string), cached_maybe.unwrap().clone() - 1);
        //     }
        // } else {
        //     cache.remove_entry(string);
        // }

        if let Some(removed_val) = cache.remove_entry(string) {
            if removed_val.1 == 2 {
                cache.insert(String::from(string), removed_val.1 - 1);
            }
        }
        sum
    }
}

#[cfg(test)]
mod my_graph_test {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

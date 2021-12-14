#![feature(test)]

extern crate test;
extern crate my_graph;
pub use my_graph::Graph;
use smallstring::SmallString;

pub fn add_two(a: i32) -> i32 {
    let mut sum = 0;
    for i in 0..a {
        sum += i;
    }
    a + sum
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use test::Bencher;
    use my_graph::Graph;

    fn create_graph() -> Graph {
        let hash_map: HashMap<SmallString, Vec<SmallString>> = HashMap::from([
            (SmallString::from("start"), vec![SmallString::from("A"), SmallString::from("b")]),
            (SmallString::from("A"), vec![SmallString::from("c"), SmallString::from("b"), SmallString::from("end")]),
            (SmallString::from("b"), vec![SmallString::from("d"), SmallString::from("A"), SmallString::from("end")]),
            (SmallString::from("c"), vec![SmallString::from("A")]),
            (SmallString::from("d"), vec![SmallString::from("b")]),
            (SmallString::from("end"), vec![]),

        ]);
        Graph {
            graph: hash_map,
        }
    }

    #[bench]
    fn how_long_find_paths(b: &mut Bencher) {
        let graph = create_graph();
        // let graph = Graph::new_from_file("test.txt");
        b.iter(|| {
            graph.explore_caves()
        });
    }
}


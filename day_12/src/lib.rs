#![feature(test)]

extern crate test;
pub use graph::Graph;
use smallstring::SmallString;

mod graph;

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
    use graph::Graph;

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

    #[test]
    fn it_creates_graph_from_file() {
        let graph = Graph::new_from_file("test_input.txt");
        let expected_graph = create_graph();
        assert_eq!(graph, expected_graph);
    }

    #[bench]
    fn bench_creates_graph_from_file(b: &mut Bencher) {
        b.iter( || {
           Graph::new_from_file("test_input.txt")
        });
    }



    #[test]
    fn it_finds_paths() {
        let graph = create_graph();
        let expected_path_count = 36;
        let path_count = graph.explore_caves();
        assert_eq!(expected_path_count, path_count);
    }

    #[bench]
    fn how_long_find_paths(b: &mut Bencher) {
        let graph = create_graph();
        // let graph = Graph::new_from_file("test.txt");
        b.iter(|| {
           graph.explore_caves()
        });
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);
            (0..n).fold(0, |a, b| a ^ b )
        })
    }
}


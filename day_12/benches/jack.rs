#![feature(test)]

extern crate test;
extern crate my_graph;
pub use my_graph::Graph;

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
        let hash_map: HashMap<String, Vec<String>> = HashMap::from([
            (String::from("start"), vec![String::from("A"), String::from("b")]),
            (String::from("A"), vec![String::from("c"), String::from("b"), String::from("end")]),
            (String::from("b"), vec![String::from("d"), String::from("A"), String::from("end")]),
            (String::from("c"), vec![String::from("A")]),
            (String::from("d"), vec![String::from("b")]),
            (String::from("end"), vec![]),

        ]);
        Graph {
            graph: hash_map,
        }
    }

    // #[test]
    // fn it_creates_graph_from_file() {
    //     let graph = Graph::new_from_file("test_input.txt");
    //     let expected_graph = create_graph();
    //     assert_eq!(graph, expected_graph);
    // }
    //
    // #[bench]
    // fn bench_creates_graph_from_file(b: &mut Bencher) {
    //     b.iter( || {
    //         Graph::new_from_file("test_input.txt")
    //     });
    // }
    //
    //
    //
    // #[test]
    // fn it_finds_paths() {
    //     let graph = create_graph();
    //     let expected_path_count = 36;
    //     let path_count = graph.explore_caves();
    //     assert_eq!(expected_path_count, path_count);
    // }

    #[bench]
    fn how_long_find_paths(b: &mut Bencher) {
        let graph = create_graph();
        // let graph = Graph::new_from_file("test.txt");
        b.iter(|| {
            graph.explore_caves()
        });
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     b.iter(|| {
    //         let n = test::black_box(1000);
    //         (0..n).fold(0, |a, b| a ^ b )
    //     })
    // }
}


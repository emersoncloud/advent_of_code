extern crate my_graph;

use my_graph::Graph;
use std::time::Instant;

fn main() {
    let mut time = Instant::now();
    let graph = Graph::new_from_file("test.txt");
    println!("initialize time{:?}", time.elapsed());

    time = Instant::now();
    let answer = graph.explore_caves();
    println!("sum: {}", answer);
    println!("explore time: {:?}", time.elapsed());
}

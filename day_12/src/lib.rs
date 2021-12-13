#![feature(test)]

extern crate test;
pub use graph::Graph;

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
    use super::*;
    use test::Bencher;

    #[test]
    fn yep_this_is_test() {
        assert_eq!(4, add_two(2))
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);
            (0..n).fold(0, |a, b| a ^ b )
        })
    }
}


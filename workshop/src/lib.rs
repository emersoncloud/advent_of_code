#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
pub fn open_file(filename: &str) -> BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("unable to open file {:?}", e),
    };

    BufReader::new(file)
}


#[cfg(test)]
mod tests {
    use crate::open_file;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_opens() {
        let reader = open_file("test.txt");
    }
}


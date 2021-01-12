use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let filename = "input.txt";

    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut n_trees = 0;
    let mut col = 0;
    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        col += 3;
        col = col % line.len();

        let b = line.as_bytes();
        if b[col] as char == '#' {
            n_trees += 1;
        }
    }

    println!("Number of trees: {}", n_trees);
}

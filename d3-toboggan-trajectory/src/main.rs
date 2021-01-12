use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut n_trees: [i64; 5] = [0, 0, 0, 0, 0];
    let mut col = [0, 0, 0, 0, 0];
    let mut l_idx = 0;
    for line in reader.lines().skip(1) {
        l_idx += 1;
        let line = line.unwrap();
        let b = line.as_bytes();

        for i in 0..4 {
            let step = 1 + i * 2;
            col[i] = (col[i] + step) % line.len();
            n_trees[i] += check_tree(b, col[i]);
        }

        if l_idx % 2 == 0 {
            col[4] = (col[4] + 1) % line.len();
            n_trees[4] += check_tree(b, col[4]);
        }
    }

    for i in 0..4 {
        println!("Number of trees [Configuration {}]: {}", i, n_trees[i]);
    }

    println!("Multiplied together: {}", n_trees.iter().product::<i64>());
}

fn check_tree(b: &[u8], c: usize) -> i64 {
    return if b[c] as char == '#' { 1 } else { 0 }
}
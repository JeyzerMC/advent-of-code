use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let filename = "input.txt";

    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut n_valid = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(":").collect();
        let rule: Vec<&str> = line[0].split(" ").collect();

        let expected_occ: Vec<usize> = rule[0].split("-").map(|x| x.parse::<usize>().expect("OK")).collect();
        let password = line[1];

        let c = password.matches(rule[1]).count();

        if c >= expected_occ[0] && c <= expected_occ[1] {
            n_valid += 1;
        }
    }

    println!("{}", n_valid);
}
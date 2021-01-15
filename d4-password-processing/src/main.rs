use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use substring::Substring;

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut fields = HashMap::new();

    fields.insert("byr", 0);
    fields.insert("iyr", 1);
    fields.insert("eyr", 2);
    fields.insert("hgt", 3);
    fields.insert("hcl", 4);
    fields.insert("ecl", 5);
    fields.insert("pid", 6);
    // fields.insert("cid", 7);

    let mut n_valid = 0;
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            sum = 0;
            continue;
        }

        let line = line.split(" ");
        for sec in line {
            let field = sec.substring(0, 3);
            if fields.contains_key(field) {
                sum += i32::pow(2, fields[field]);
            }
        }

        if sum == 127 {
            n_valid += 1;
            sum = 0;
        }
    }

    println!("The number of valid passports is: {}", n_valid);
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;
use regex::Regex;

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).expect("Couldn't open the file");
    let reader = BufReader::new(file);

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
            let val = sec.substring(4, sec.len());

            sum += match field {
                "byr" => validate_byr(val),
                "iyr" => validate_iyr(val),
                "eyr" => validate_eyr(val),
                "hgt" => validate_hgt(val),
                "hcl" => validate_hcl(val),
                "ecl" => validate_ecl(val),
                "pid" => validate_pid(val),
                _ => 0
            }
        }

        if sum == 127 {
            n_valid += 1;
            sum = 0;
        }
    }

    println!("The number of valid passports is: {}", n_valid);
}

fn validate_byr(val: &str) -> i8 {
    let birth_date = val.parse::<i32>();
    match birth_date {
        Ok(birth_date) if birth_date >= 1920 && birth_date <= 2002 => 1,
        _ => 0
    }
}

fn validate_iyr(val: &str) -> i8 {
    let issue_date = val.parse::<i32>();
    match issue_date {
        Ok(issue_date) if issue_date >= 2010 && issue_date <= 2020 => 2,
        _ => 0
    }
}

fn validate_eyr(val: &str) -> i8 {
    let exp_date = val.parse::<i32>();
    match exp_date {
        Ok(exp_date) if exp_date >= 2020 && exp_date <= 2030 => 4,
        _ => 0
    }
}

fn validate_hgt(val: &str) -> i8 {
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let cap = re.captures(val);

    if let Some(cap) = &cap {
        if cap.len() == 3 {
            let height = cap[1].parse::<i32>().expect("OK");
            match &cap[2] {
                "cm" if height >= 150 && height <= 193 => return 8,
                "in" if height >= 59 && height <= 76 => return 8,
                _ => return 0
            }
        }
    }
    0
}

fn validate_hcl(val: &str) -> i8 {
    let re = Regex::new(r"^#[1-9|a-f]{6}$").unwrap();
    let cap = re.captures(val);
    match cap {
        Some(cap) if cap.len() == 1 => 16,
        _ => 0
    }
}

fn validate_ecl(val: &str) -> i8 {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if valid.contains(&val) { 32 } else { 0 }
}

fn validate_pid(val: &str) -> i8 {
    let re = Regex::new(r"^\d{9}$").unwrap();
    let cap = re.captures(val);
    match cap {
        Some(cap) if cap.len() == 1 => 64,
        _ => 0
    }
}
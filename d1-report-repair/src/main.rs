use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong opening the file");

    let content: Vec<i32> = content.lines().map(|x| x.parse::<i32>().expect("OK")).collect();

    for c1 in &content {
        for c2 in &content {
            if c1 + c2 == 2020 {
                println!("{} + {} = {}", c1, c2, c1 + c2);
                println!("{} x {} = {}", c1, c2, c1 * c2);
                return;
            }
        }
    }
}
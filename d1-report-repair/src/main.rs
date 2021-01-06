use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong opening the file");

    let content: Vec<i32> = content.lines().map(|x| x.parse::<i32>().expect("OK")).collect();

    let mut duos: HashMap<i32, (&i32, &i32)> = HashMap::new();

    for c1 in &content {
        for c2 in &content {
            if c1 != c2 && c1 + c2 < 2020 {
                duos.insert(c1 + c2, (c1, c2));
            }
        }
    }

    for c in &content {
        for (k, v) in &duos {
            if c + k == 2020 {
                println!("{} + {} + {} = {}", c, v.0, v.1, c + v.0 + v.1);
                println!("{} * {} * {} = {}", c, v.0, v.1, c * v.0 * v.1);
                return;
            }
        }
    }
}
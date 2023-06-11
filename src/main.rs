use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(&file_path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}



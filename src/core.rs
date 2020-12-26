use std::fs;

pub fn read_rules(filename: String) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read filter file!");
    println!("{}", contents);
}
use std::env;
use std::fs;

fn read_file(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("Failed reading the file...");
    return contents;
}

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::thread;
use std::time;

fn read_thinking_alternatives(filename: &str, storage_vec: &mut Vec<String>) {
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        storage_vec.push(String::from(line.unwrap()));
    }
}

fn main() {
    let filename = String::from("src/main.rs");
    let mut phrase: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();

    read_thinking_alternatives("phrase", &mut phrase);
    read_thinking_alternatives("answer", &mut answer);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let ten_millis = time::Duration::from_millis(10);
    for test in contents.chars() {
        eprint!("{}", test);
        thread::sleep(ten_millis);
    }
}

use ansi_term::Colour;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::thread;
use std::time;

macro_rules! sleep {
    ($duration:expr) => {{
        let ten_millis = time::Duration::from_millis($duration);
        thread::sleep(ten_millis);
    }};
}

fn read_thinking_alternatives(filename: &str, storage_vec: &mut Vec<String>) {
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        storage_vec.push(String::from(line.unwrap()));
    }
}

fn get_random_str(string_vec: &mut Vec<String>) -> &String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..string_vec.len() - 1);

    &string_vec[random_index]
}

const LIMIT: u32 = 20;
const BACKSPACE: char = 8 as char;
const SPACE: char = 32 as char;

fn main() {
    let filename = String::from("src/main.rs");
    let mut phrase: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();

    read_thinking_alternatives("phrase", &mut phrase);
    read_thinking_alternatives("answer", &mut answer);

    let source_file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut limit = 1;
    for source_file_line in source_file_content.lines() {
        for char_in_file in source_file_line.chars() {
            eprint!("{}", char_in_file);
            sleep!(20);
        }
        eprintln!();
        if limit == LIMIT {
            let phrase_str = get_random_str(&mut phrase);
            let answer_str = get_random_str(&mut answer);
            let chars_to_delete = phrase_str.len() + answer_str.len() + 4;

            eprint!("{}", Colour::Red.bold().paint(phrase_str));
            for _i in 0..3 {
                eprint!("{}", Colour::Red.bold().paint("."));
                sleep!(1000);
            }
            eprint!("{}", Colour::Red.bold().paint(answer_str));
            sleep!(2800);

            for _i in 1..chars_to_delete {
                eprint!("{}", BACKSPACE);
                eprint!("{}", SPACE);
                eprint!("{}", BACKSPACE);
            }
            limit = 0;
        }
        limit += 1;
    }
}

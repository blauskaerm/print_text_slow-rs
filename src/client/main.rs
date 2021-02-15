use ansi_term::Colour;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;
use std::thread;
use std::time;

extern crate clap;
use clap::{App, Arg};

use std::io::{Read, Write};
use std::net::TcpStream;

macro_rules! sleep {
    ($duration:expr) => {{
	let ten_millis = time::Duration::from_millis($duration);
	thread::sleep(ten_millis);
    }};
}

fn read_thinking_alternatives(filename: &str, storage_vec: &mut Vec<String>) {
    let file: std::fs::File;
    match File::open(filename) {
	Ok(fd) => file = fd,
	Err(err) => {
	    eprint!("Unable to open {}, {}", filename, err.to_string());
	    process::exit(-1);
	}
    };
    let reader = BufReader::new(file);

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
    let mut stream = TcpStream::connect("localhost:7878").unwrap();

    let mut buffer_string = String::new();

    let result = stream.read_to_string(&mut buffer_string);

    let answer_vec_test: Vec<String> = buffer_string
	.split('\n')
	.map(str::to_string)
	.filter(|e| e.len() > 0)
	.collect();

    for my_string in answer_vec_test {
	println!("Received: {}", my_string);
    }

    process::exit(0);

    let cmd_options = App::new("print_text_slow")
	.version("0.1")
	.author("BlauskaerM <blauskaerm@protonmail.ch>")
	.about(
	    "Print the source code of your projects and watch your computer think while doing it",
	)
	.arg(
	    Arg::with_name("FILE")
		.help("File to print")
		.required(true)
		.index(1),
	)
	.arg(
	    Arg::with_name("Phrases")
		.short("p")
		.long("phrase")
		.value_name("file-path")
		.help("File containing a list of phrases (default ./phrase)")
		.takes_value(true),
	)
	.arg(
	    Arg::with_name("Answers")
		.short("a")
		.long("answer")
		.value_name("file-path")
		.help("File containing a list of answers (default ./answer)")
		.takes_value(true),
	)
	.get_matches();

    let filename = cmd_options.value_of("FILE").unwrap_or("NOT FILE SELECTED");
    let phrases_file = cmd_options.value_of("Phrases").unwrap_or("phrase");
    let answers_file = cmd_options.value_of("Answers").unwrap_or("answer");

    let mut phrase: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();

    read_thinking_alternatives(phrases_file, &mut phrase);
    read_thinking_alternatives(answers_file, &mut answer);

    let source_file_content =
	fs::read_to_string(&filename).expect(format!("Unable to read file {}", &filename).as_str());

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

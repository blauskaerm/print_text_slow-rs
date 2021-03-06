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

use std::io::Read;
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

fn read_thinking_alternatives_tcp(server_str: &str,answer_vec: &mut Vec<String>, phrase_vec: &mut Vec<String>) {
    let mut stream = match TcpStream::connect(server_str) {
	Ok(server) => server,
	Err(err) => {
	    eprint!("Unable to connect to server {}, {}", server_str, err.to_string());
	    process::exit(-1);
	},
    };

    let mut buffer_string = String::new();
    let result = stream.read_to_string(&mut buffer_string);

    if result.unwrap() > 0 {
        for received_strings in buffer_string.split('\n') {
            if received_strings.len() > 0 {
                let received_strings_split: Vec<&str> = received_strings.split(';').collect();
                if received_strings_split.len() == 2 {
                    let type_s = received_strings_split[0];
                    let sentence = received_strings_split[1];

                    match type_s {
                        "answer" => {
                            answer_vec.push(String::from(sentence));
                        }
                        "phrase" => {
                            phrase_vec.push(String::from(sentence));
                        }
                        _ => {}
                    }
                }
            }
        }
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
    let cmd_options = App::new("print_text_slow")
        .version("0.2")
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
	.arg(
	    Arg::with_name("Server")
		.short("s")
		.long("server")
		.value_name("thinking-server")
		.help("Receive phrases and answers from a remove server")
		.takes_value(true),
	)
	.get_matches();

    let filename = cmd_options.value_of("FILE").unwrap_or("NOT FILE SELECTED");
    let phrases_file = cmd_options.value_of("Phrases").unwrap_or("phrase");
    let answers_file = cmd_options.value_of("Answers").unwrap_or("answer");
    let server = cmd_options.value_of("Server").unwrap_or("No server");

    let mut phrase: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();

    if server != "No server" {
	read_thinking_alternatives_tcp(server, &mut answer, &mut phrase);
    } else {
	read_thinking_alternatives(phrases_file, &mut phrase);
	read_thinking_alternatives(answers_file, &mut answer);
    }

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

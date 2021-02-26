use std::thread;

use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

fn read_file_to_client(mut stream: &TcpStream, category: String) {

    let file: std::fs::File;
    match File::open(&category) {
	Ok(fd) => file = fd,
	Err(err) => {
	    eprint!("Unable to open {}, {}", &category, err.to_string());
	    process::exit(-1);
	}
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
	let line = line.unwrap();
	let message_buffer = format!("{};{}\n", &category, line);
	stream.write(message_buffer.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}

fn handle_client(stream: TcpStream) {
    read_file_to_client(&stream, String::from("answer"));
    read_file_to_client(&stream, String::from("phrase"));
}

fn main() {
    const BIND_ADDR: &str = "localhost";
    const BIND_PORT: &str = "7878";

    let listener = TcpListener::bind(format!("{}:{}", BIND_ADDR, BIND_PORT)).unwrap();

    for stream in listener.incoming() {

	println!("Connection established!");

	let stream = stream.unwrap();
	thread::spawn(|| {
	    handle_client(stream);
	});

    }
}

use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

fn read_file_to_client(mut stream: &TcpStream, category: String) {
    //let filename =
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
	//storage_vec.push(String::from(line.unwrap()));
	//println!("Sending string: {}", line);
	let line = line.unwrap();
	println!("Sending: {}", &line);
	let message_buffer = format!("{};{}\n", &category, line);
	stream.write(message_buffer.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}

fn handle_client(stream: TcpStream) {
    // let mut string_vec: Vec<String> = Vec::new();
    // string_vec.push("Test".to_string());
    // string_vec.push("Test".to_string());
    // string_vec.push("Test".to_string());

    read_file_to_client(&stream, String::from("answer"));
    read_file_to_client(&stream, String::from("phrase"));

    // let mut buffer_string = String::new();
    // let result = stream.read(&mut buffer_string);
    // println!("I got: {}", buffer_string);

    // let filename = "answer";
    // let file: std::fs::File;
    // match File::open(filename) {
    //     Ok(fd) => file = fd,
    //     Err(err) => {
    //         eprint!("Unable to open {}, {}", filename, err.to_string());
    //         process::exit(-1);
    //     }
    // };
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     //storage_vec.push(String::from(line.unwrap()));
    //     //println!("Sending string: {}", line);
    //     let line = line.unwrap();
    //     println!("Sending: {}", &line);
    //     let message_buffer = format!("{}\n", line);
    //     stream.write(message_buffer.as_bytes()).unwrap();
    // }

    // Send EOF
    //stream.write(&[0x04]);
    //stream.flush().unwrap();
}

fn main() {
    const BIND_ADDR: &str = "localhost";
    const BIND_PORT: &str = "7878";

    let listener = TcpListener::bind(format!("{}:{}", BIND_ADDR, BIND_PORT)).unwrap();

    for stream in listener.incoming() {
	let stream = stream.unwrap();

	println!("Connection established!");

	handle_client(stream);
    }
}

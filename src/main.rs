use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*, Result, Lines};
use web_server::ThreadPool;
use web_server::shutdown;

mod home;
use home::home;
mod hello;
use hello::hello;

fn main() {
	let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming().take(2) {
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});

		if shutdown.lock().unwrap().len() == 1 {
			break;
		}
	}
	drop(pool);
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&stream);
	let mut response: String = String::from("");
	let mut path: &str = "";
	let mut req_type: &str ="";
	let lines = buf_reader.lines();

	let lines: Vec<String> = get_lines(lines).unwrap();

	if lines.len() == 0 {
		let content = String::from("Oh no, internal server error...");
		let length = content.len();
		response = format!("HTTP/1.1 500 INTERNAL SERVER\nContent-Length: {length}\n\n{content}");
	} else {
		let request_line = &lines[0];
		path = request_line.split_whitespace().nth(1).unwrap_or("/");
		req_type = request_line.split_whitespace().nth(0).unwrap_or("GET");
	}
	if path == "/" && req_type == "GET" {
		response = home();
	} else if path.starts_with("/hello?name=") && req_type == "GET" {
		response = hello(path);
	} else if path == "/shutdown" && req_type == "GET" {
		response = format!("HTTP/1.1 200\nContent-Length: 16\n\nShutting down...");
		shutdown.lock().unwrap().push(1)
	} else if response.len() == 0 {
		let content = format!("Oh Oh, looks like {path} does'nt exists.");
		let length: usize = content.len();
		response = format!("HTTP/1.1 404 NOT FOUND\nContent-Length: {length}\n\n{content}");
	}

	stream.write_all(response.as_bytes()).unwrap();
}

fn get_lines<B: std::io::BufRead>(linesor: Lines<B>) -> Result<Vec<String>> {
	let mut lines: Vec<String> = Vec::new();
	for line in linesor {
		let cur = line?;
		if cur.is_empty() {
			break;
		}
		lines.push(cur);
	}
	Ok(lines)
}

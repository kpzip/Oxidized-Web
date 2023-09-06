mod threadpool;
use crate::threadpool::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool: ThreadPool = ThreadPool::new(4).unwrap();

    //Main event loop
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        //println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut bad_request: bool = false;
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(some) => some,
            Err(_) => {
                bad_request = true;
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    let request_type: &String = http_request.first().unwrap();

    let (status, filename): (&str, &str) = match &request_type[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "test.html"),
        _ => match bad_request {
            true => ("HTTP/1.1 400 BAD REQUEST", ""),
            false => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        },
    };

    let contents: String = match fs::read_to_string(filename) {
        Ok(some) => some,
        Err(_) => String::new(),
    };

    respond(&mut stream, status.to_string(), contents);

    //println!("Request: {:#?}", http_request);
}

fn respond(stream: &mut TcpStream, status: String, contents: String) {
    let length: usize = contents.len();
    let response: String = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

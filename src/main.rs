mod resourcemanager;
mod threadpool;
use crate::threadpool::ThreadPool;
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
    let blank: String = String::new();
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

    //println!("Request: {:#?}", http_request);

    let request_type: &String = match http_request.first() {
        Some(req) => req,
        None => {
            bad_request = true;
            &blank
        }
    };

    let (mut status, filename): (&str, Option<&str>) = match &request_type[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", Some("html_default/test.html")),
        "GET /test.css HTTP/1.1" => ("HTTP/1.1 200 OK", Some("html_default/test.css")),
        "GET /test.js HTTP/1.1" => ("HTTP/1.1 200 OK", Some("html_default/test.js")),
        _ => match bad_request {
            true => ("HTTP/1.1 400 BAD REQUEST", None),
            false => ("HTTP/1.1 404 NOT FOUND", Some("html_default/404.html")),
        },
    };

    let contents: String = match filename {
        Some(file) => match resourcemanager::get_resource_data(file) {
            Ok(some) => some,
            Err(_) => {
                status = "HTTP/1.1 500 INTERNAL SERVER ERROR";
                String::new()
            }
        },
        None => String::new(),
    };

    respond(&mut stream, status.to_string(), contents);

    //println!("Request: {:#?}", http_request);
}

fn respond(stream: &mut TcpStream, status: String, contents: String) {
    let length: usize = contents.len();
    let response: String = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

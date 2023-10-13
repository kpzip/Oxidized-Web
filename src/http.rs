use std::io::prelude::*;
use std::{io::BufReader, net::TcpStream};

#[derive(Debug)]
pub struct HttpRequest {
    request: Vec<String>,
    is_valid: bool,
}

impl HttpRequest {
    pub fn new(contents: Vec<String>, is_valid: bool) -> HttpRequest {
        HttpRequest {
            request: contents,
            is_valid,
        }
    }

    pub fn from_stream(stream: &TcpStream) -> HttpRequest {
        let mut bad_request: bool = false;
        let request: Vec<String> = BufReader::new(stream)
            .lines()
            .map(|line| match line {
                Ok(s) => s,
                Err(_) => {
                    bad_request = true;
                    String::new()
                }
            })
            .take_while(|line| !line.is_empty())
            .collect();
        HttpRequest::new(request, bad_request)
    }

    pub fn first_line(&self) -> Option<&String> {
        self.request.first()
    }
}

pub fn respond(stream: &mut TcpStream, status: String, contents: String) {
    let length: usize = contents.len();
    let response: String = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

pub fn respond_empty(stream: &mut TcpStream, status: String) {
    respond(stream, status, String::new())
}

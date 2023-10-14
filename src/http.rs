use std::io::prelude::*;
use std::{io::BufReader, net::TcpStream};

#[derive(Debug)]
pub enum HttpRequest {
    GET(&'static str, String),
    POST(&'static str),
    INVALID,
}

impl HttpRequest {
    pub fn new(contents: Option<Vec<String>>) -> HttpRequest {
        match contents {
            Some(lines) => match lines.first() {
                Some(firstline) => match firstline.is_empty() {
                    false => {
                        if firstline[0..2] == *"GET" {
                            HttpRequest::GET((), ())
                        }
                        HttpRequest::POST(())
                    }
                    true => HttpRequest::INVALID,
                },
                None => HttpRequest::INVALID,
            },
            None => HttpRequest::INVALID,
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
        if bad_request {
            return HttpRequest::new(None);
        }
        HttpRequest::new(Some(request))
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

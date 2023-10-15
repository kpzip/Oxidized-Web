use std::io::prelude::*;
use std::{io::BufReader, net::TcpStream};

#[derive(Debug)]
#[allow(dead_code)]
pub enum HttpRequest {
    Get(String, String),
    Post(String),
    Invalid,
}

impl HttpRequest {
    pub fn new(contents: Option<Vec<String>>) -> HttpRequest {
        match contents {
            Some(lines) => match lines.first() {
                Some(firstline) => match firstline.is_empty() {
                    false => {
                        let words: Vec<&str> = firstline.split(' ').collect();
                        println!("Words: {:#?}", words);
                        match *match words.first() {
                            Some(s) => s,
                            None => return HttpRequest::Invalid,
                        } {
                            "GET" => HttpRequest::Get(
                                String::from(*words.get(2).unwrap()),
                                String::from(*words.get(1).unwrap()),
                            ),
                            _ => HttpRequest::Invalid,
                        }
                    }
                    true => HttpRequest::Invalid,
                },
                None => HttpRequest::Invalid,
            },
            None => HttpRequest::Invalid,
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
    let response: String = format!("{status}\r\nContent-Length: 0\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}

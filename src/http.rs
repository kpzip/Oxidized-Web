use std::io::prelude::*;
use std::{io::BufReader, net::TcpStream};

#[derive(Debug)]
#[allow(dead_code, non_camel_case_types, upper_case_acronyms)]
pub enum HttpRequest {
    GET(HttpBody),
    POST(HttpBody),
    INVALID,
}

#[derive(Debug)]
pub enum HttpVersion {
    HTTP0_9,
    HTTP1,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

impl HttpVersion {
    fn from_string(v: &str) -> HttpVersion {
        match v {
            "HTTP/0.9" => HttpVersion::HTTP0_9,
            "HTTP/1.0" => HttpVersion::HTTP1,
            "HTTP/1.1" => HttpVersion::HTTP1_1,
            "HTTP/2" => HttpVersion::HTTP2,
            "HTTP/3" => HttpVersion::HTTP3,
            _ => HttpVersion::HTTP1_1,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HttpBody {
    version: HttpVersion,
    path: String,
    fields: Vec<HttpRequestField>,
    contents: String,
}

impl HttpBody {
    pub fn new(
        version: HttpVersion,
        path: String,
        fields: Vec<HttpRequestField>,
        contents: String,
    ) -> HttpBody {
        HttpBody {
            version,
            path,
            fields,
            contents,
        }
    }

    pub fn get_version(&self) -> &HttpVersion {
        &self.version
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum HttpRequestField {
    Content_Length(usize),
    User_Agent(String),
    Host(String),
    Content_Type(ContentType),
}

impl HttpRequestField {
    pub fn from_string(field: &String) -> Option<HttpRequestField> {
        let field: Vec<&str> = field.split_whitespace().collect();
        if let Some(field_name) = field.first() {
            match *field_name {
                "Content-Length" => {
                    return Some(HttpRequestField::Content_Length(
                        field.get(1).unwrap_or(&"0").parse().unwrap_or(0),
                    ))
                }
                _ => return None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    XML,
    JSON,
    TEXT,
}

impl HttpRequest {
    pub fn new(contents: Vec<String>) -> HttpRequest {
        let mut fields: Vec<HttpRequestField> = Vec::new();
        let mut iter = contents.iter();
        let head: String = match iter.next() {
            Some(s) => s.clone(),
            None => return HttpRequest::INVALID,
        };
        let head_components: Vec<&str> = head.split_whitespace().collect();
        let path = String::from(match head_components.get(1) {
            Some(s) => *s,
            None => return HttpRequest::INVALID,
        });
        let version = HttpVersion::from_string(match head_components.get(2) {
            Some(s) => s,
            None => return HttpRequest::INVALID,
        });

        let mut capacity: usize = 0;
        let mut s: String;
        loop {
            if let Some(e) = iter.next() {
                s = e.clone();
            } else {
                break;
            }
            if s == "\r\n" {
                break;
            }
            if let Some(f) = HttpRequestField::from_string(&s) {
                if let HttpRequestField::Content_Length(l) = f {
                    capacity = l;
                }
                fields.push(f);
            }
        }
        //
        //TODO validate fields
        //
        let mut is_after: bool = false;
        let actual_content_length: usize = contents
            .iter()
            .filter(|line| -> bool {
                if *line == "\r\n" {
                    is_after = true;
                    return false;
                }
                is_after
            })
            .map(|line| -> usize { line.chars().count() })
            .sum();
        if capacity != actual_content_length {
            return HttpRequest::INVALID;
        }

        let mut contents: String = String::with_capacity(capacity);
        loop {
            if let Some(e) = iter.next() {
                s = e.clone();
            } else {
                break;
            }
            contents.push_str(&s);
        }

        let body: HttpBody = HttpBody::new(version, path, fields, contents);

        match *head_components.get(0).unwrap_or(&"INVALID") {
            "GET" => return HttpRequest::GET(body),
            "POST" => return HttpRequest::POST(body),
            _ => return HttpRequest::INVALID,
        }

        /*
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
        }*/
    }

    pub fn from_stream(stream: &TcpStream) -> HttpRequest {
        let request: Vec<String> = BufReader::new(stream)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        HttpRequest::new(request)
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

#[macro_use]
mod macros;
mod http;
mod resourcemanager;
mod threadpool;
use crate::http::HttpRequest;
use crate::http::HttpRequest::*;
use crate::threadpool::ThreadPool;
use std::io::ErrorKind;
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
    let http_request: HttpRequest = HttpRequest::from_stream(&stream);

    println!("Request: {:#?}", http_request);

    match http_request {
        Get(_, resource) => {
            let filename: String = match resource.as_str() {
                "/" => String::from("html_default/test.html"),
                path => {
                    let mut name: String = String::from("html_default");
                    name.push_str(path);
                    name
                }
            };
            match resourcemanager::get_resource_data(filename) {
                Ok(some) => {
                    http::respond(&mut stream, String::from(responsify!(200)), some);
                }
                Err(e) => {
                    if e.is::<std::io::Error>() {
                        http::respond(
                            &mut stream,
                            String::from(responsify!(404)),
                            resourcemanager::get_resource_data(String::from(
                                "html_default/404.html",
                            ))
                            .unwrap(),
                        );
                    } else {
                        http::respond_empty(&mut stream, String::from(responsify!(500)));
                    }
                }
            }
        }
        Post(_) => (),
        Invalid => {
            http::respond_empty(&mut stream, String::from(responsify!(400)));
        }
    }
    /*
    let request_type: String = match http_request.first_line() {
        Some(req) => req.clone(),
        None => {}
    };

    let (mut status, filename): (&str, &str) = match &request_type[..] {
        "GET / HTTP/1.1" => (responsify!(200), "html_default/test.html"),
        "GET /test.css HTTP/1.1" => (responsify!(200), "html_default/test.css"),
        "GET /test.js HTTP/1.1" => (responsify!(200), "html_default/test.js"),
        _ => (responsify!(404), "html_default/404.html"),
    };

    let contents: String = match resourcemanager::get_resource_data(filename) {
        Ok(some) => some,
        Err(_) => {
            status = responsify!(500);
            String::new()
        }
    };

    http::respond(&mut stream, status.to_string(), contents);
    */
}

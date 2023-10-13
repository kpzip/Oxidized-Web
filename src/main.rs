#[macro_use]
mod macros;
mod http;
mod resourcemanager;
mod threadpool;
use crate::http::HttpRequest;
use crate::threadpool::ThreadPool;
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

    let request_type: String = match http_request.first_line() {
        Some(req) => req.clone(),
        None => {
            http::respond_empty(&mut stream, String::from(responsify!(400)));
            return;
        }
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
}

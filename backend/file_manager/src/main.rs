use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
use parser::http_request::HttpRequest;
use logger::logger::Logger;
use enums::enums::HttpRequestMethod;

mod logger;
mod enums;
mod parser;

// CONSTS 
const IP_ADDRESS: &str = "127.0.0.1";
const PORT:       &str = "7000";

// UTILITY
fn construct_url() -> String {
    format!("{}:{}", IP_ADDRESS, PORT)
}

fn construct_response(status: &str, contents: &str, content_type: &str, content_length: usize) -> String {
    format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", status, content_type, content_length, contents)
}

// MAIN
fn main() {
    let listener = match TcpListener::bind(construct_url()) {
        Ok(tcpl) => tcpl,
        Err(e) => panic!("Failed to bind tcp listener. {:?}", e),
    };

    for res_stream in listener.incoming() {
        Logger::info(format!("Connection estublished at, PORT: {PORT:#?}").as_str());
        match res_stream {
            Ok(tcp_stream) => handle_connection(tcp_stream),
            Err(e) => panic!("Failed to get stream from listener. {:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::construct(request);
    Logger::debug(format!("Request: {http_request:#?}").as_str());

    // TODO: CREATE ROUTING MODULE
    let response = match http_request.method {
        HttpRequestMethod::GET => {
            let status = "HTTP/1.1 200 OK";
            let contents = "Test project";
            let content_type = "text/plain";
            let content_length = contents.len();
            construct_response(status, contents, content_type, content_length)
        },
        _ => {
            let status = "HTTP/1.1 200 OK";
            let contents = "404 Not Found";
            let content_type = "text/plain";
            let content_length = contents.len();
            construct_response(status, contents, content_type, content_length)
        }
    };
    match stream.write_all(response.as_bytes()) {
        Err(e) => panic!("Failed to write response. {:?}", e),
        _ => {},
    }
}
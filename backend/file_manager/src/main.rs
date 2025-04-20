use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use config::{constants::TOTAL_ACTIVE_THREADS, utility::{construct_response, construct_url}};
use logger::app_logger::Logger;

use library::tp::ThreadPool;
use parser::http_request::HttpRequest;
use enums::app_enums::HttpRequestMethod;

mod logger;
mod enums;
mod parser;
mod config;
mod library;

// MAIN
fn main() {
    let url: String = construct_url();
    let pool = ThreadPool::new(TOTAL_ACTIVE_THREADS);
    let listener = match TcpListener::bind(url.clone()) {
        Ok(tcpl) => tcpl,
        Err(e) => {
            Logger::error("Failed to bind tcp listener", None);
            panic!("{:?}", e);
        },
    };

    Logger::info(format!("Connection estublished at, HOST: {url}").as_str());
    for res_stream in listener.incoming() {
        match res_stream {
            Err(e) => Logger::error("Failed to get stream from listener", Some(Box::new(e))),
            Ok(tcp_stream) => pool.execute(|| connection_handler(tcp_stream)),
        }
    }
}

fn connection_handler(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::construct(request);
    Logger::debug(format!("Request [method]: {}", http_request.method).as_str());

    // TODO: CREATE ROUTING MODULE
    let http_response = match http_request.method {
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

    Logger::debug("Send response to client");
    match stream.write_all(http_response.as_bytes()) {
        Err(e) => Logger::error("Failed to write response to client", Some(Box::new(e))),
        _ => {},
    }
}
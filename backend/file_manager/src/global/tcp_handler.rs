use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use crate::config::{constants::{ASYNC_ROUTING_TABLE, THREADING_ROUTING_TABLE, TOTAL_ACTIVE_THREADS}, utility::{construct_response, construct_url}};
use crate::logger::app_logger::Logger;

use crate::library::tp::ThreadPool;
use crate::parser::http_request::HttpRequest;
use crate::enums::app_enums::HttpRequestMethod;

pub struct TcpHandler {
  pub url: String,
  pub pool: ThreadPool,
  pub listener: TcpListener,
}

impl TcpHandler {
  pub fn new() -> Self {
    TcpHandler {
      url: construct_url(),
      pool: ThreadPool::new(TOTAL_ACTIVE_THREADS),
      listener: match TcpListener::bind(construct_url()) {
        Ok(tcpl) => tcpl,
        Err(e) => {
            Logger::error("Failed to bind tcp listener", None);
            panic!("{:?}", e);
        },
      }
    }
  } 
}

impl TcpHandler {
  pub fn listen(&self) {
    Logger::info(format!("Connection estublished at, HOST: {}", self.url).as_str());
    for res_stream in self.listener.incoming() {
      match res_stream {
        Err(e) => Logger::error("Failed to get stream from listener", Some(Box::new(e))),
        Ok(tcp_stream) => {
          // TODO: BETTER TO MAKE IT ASYNC OR MANAGE BY SEPARATE THREAD
          let http_request = TcpHandler::build_http_request(&tcp_stream);
          if let Some(route) = THREADING_ROUTING_TABLE.iter().find(|p| **p == http_request.path) {
            self.execute(&http_request, &tcp_stream, route);
          } else if let Some(route) = ASYNC_ROUTING_TABLE.iter().find(|p| **p == http_request.path) {
            // TODO: ASYNC ROUTE UNDER CONSTRUCTION
            Logger::debug("ASYNC REQUEST IS UNDER CONSTUCTION!");
            Logger::debug(route);
          }
        },
      }
    }
  }
}

impl TcpHandler {
  fn execute(&self, http_request: &HttpRequest, stream: &TcpStream, route: &str) {
    let mut st = stream.try_clone().expect("Failed to clone mutable TCP stream!");
    let hr = http_request.clone();
    let rt = String::from(route);

    self.pool.execute(move || {
      let http_response = TcpHandler::build_http_response(&hr, rt);
      TcpHandler::reply(http_response, &mut st);
    });
  }
}

impl TcpHandler {
  fn build_http_request(stream: &TcpStream) -> HttpRequest {
    Logger::debug("Creating HTTP request from stream");
    let buf_reader = BufReader::new(stream);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    HttpRequest::construct(request)
  }

  fn build_http_response(http_request: &HttpRequest, route: String) -> String {
    Logger::debug("Creating HTTP response for client");

    let http_version = "HTTP/1.1";
    let http_status;
    let http_message;

    let content_type = "text/json";
    let content_length;
    let contents;

    match http_request.method { 
      HttpRequestMethod::GET => {
        match route.as_str() {
          "/" => {
            http_status = "200";
            http_message = "Ok";

            contents = "{ \"test\": \"test\" }";
            content_length = contents.len();
          }
          _ => {
            http_status = "404";
            http_message = "Not Found";

            contents = "{ \"test\": \"Not Found\" }";
            content_length = contents.len();
          }
        }
      },
      _ => {
        http_status = "404";
        http_message = "NOT FOUND";
         
        contents = "{ \"test\": \"Method Not Allowed\" }";
        content_length = contents.len();
      }
    };

    construct_response(
      format!("{} {} {}", http_version, http_status, http_message).as_str(),
      contents, 
      content_type, 
      content_length
    )
  }

  fn reply(http_response: String, stream: &mut TcpStream) {
    Logger::debug("Sending response to client");
    match stream.write_all(http_response.as_bytes()) {
        Err(e) => Logger::error("Failed to write response to client", Some(Box::new(e))),
        _ => {},
    }
  }
}

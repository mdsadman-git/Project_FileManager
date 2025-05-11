use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::{config::{constants::{ASYNC_ROUTING_TABLE, TOTAL_ACTIVE_THREADS}, utility::construct_app_url}, router::router_handler::RouterHandler};
use logger_main::Logger;

use crate::library::tp::ThreadPool;
use crate::parser::http_request::HttpRequest;

pub struct TcpHandler {
  pub url: String,
  pub pool: ThreadPool,
  pub listener: TcpListener,
  
  router_handler: RouterHandler
}

impl TcpHandler {
  pub fn new() -> Self {
    TcpHandler {
      url: construct_app_url(),
      pool: ThreadPool::new(TOTAL_ACTIVE_THREADS),
      listener: match TcpListener::bind(construct_app_url()) {
        Ok(tcpl) => tcpl,
        Err(e) => {
            Logger::error("Failed to bind tcp listener", None);
            panic!("{:?}", e);
        },
      },

      router_handler: RouterHandler::new(),
    }
  } 
}

impl TcpHandler {
  pub fn listen(&self) {
    Logger::info(format!("Connection estublished at, Host: {}", self.url));

    for res_stream in self.listener.incoming() {
      match res_stream {
        Err(e) => Logger::error("Failed to get stream from listener", Some(Box::new(e))),
        Ok(tcp_stream) => {
          // TODO: BETTER TO MAKE IT ASYNC OR MANAGE BY SEPARATE THREAD
          let http_request = TcpHandler::parse_tcp_stream(&tcp_stream);

          if let Some(&path) = ASYNC_ROUTING_TABLE.iter().find(|p| **p == http_request.path) {
            // TODO: ASYNC ROUTE UNDER CONSTRUCTION
            Logger::debug("ASYNC REQUEST IS UNDER CONSTUCTION!");
            Logger::debug(path);
            continue;
          } 

          Logger::debug(format!("Thread handling Http Request, Path: {}", &http_request.path));
          self.execute(&http_request, &tcp_stream, &http_request.path);
        },
      }
    }
  }
}

impl TcpHandler {
  fn execute(&self, http_request: &HttpRequest, tcp_stream: &TcpStream, path: &str) {
    let mut stream = tcp_stream.try_clone().expect("Failed to clone mutable TCP stream!");
    let http_response = self.router_handler.exec(&http_request.method, path).clone();
    self.pool.execute(move || TcpHandler::reply_to_client(http_response(), &mut stream));
  }
}

trait TcpHandlerTrait {
  fn parse_tcp_stream(stream: &TcpStream) -> HttpRequest;
  fn reply_to_client(http_response: String, stream: &mut TcpStream);
}

impl TcpHandlerTrait for TcpHandler {
  fn parse_tcp_stream(stream: &TcpStream) -> HttpRequest {
    Logger::debug("Creating HTTP request from stream");
    let buf_reader = BufReader::new(stream);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    HttpRequest::construct(request)
  }

  fn reply_to_client(http_response: String, stream: &mut TcpStream) {
    Logger::debug("Sending response to client");
    match stream.write_all(http_response.as_bytes()) {
        Err(e) => Logger::error("Failed to write response to client", Some(Box::new(e))),
        _ => {},
    }
  }
}

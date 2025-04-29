use global::tcp_handler::TcpHandler;

mod logger;
mod enums;
mod parser;
mod config;
mod library;
mod global;
mod router;

// MAIN
fn main() {
  TcpHandler::new().listen();
}

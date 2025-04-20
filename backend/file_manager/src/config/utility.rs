use crate::config::constants::{HOST_DEFAULT_PORT, HOST_IP_ADDRESS, LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME};

pub fn construct_url() -> String {
    format!("{}:{}", HOST_IP_ADDRESS, HOST_DEFAULT_PORT)
}

pub fn construct_response(status: &str, contents: &str, content_type: &str, content_length: usize) -> String {
    format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", status, content_type, content_length, contents)
}

pub fn construct_logger_dtf() -> String {
  format!("{} {}", LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME)
}
use crate::config::utility::construct_response;

pub struct Get;

impl Get {
  pub fn home() -> String {
    let http_version = "HTTP/1.1";
    let http_status = "200";
    let http_message = "Ok";
    let content_type = "text/json";
    let contents = "{ \"test\": \"test\" }";
    let content_length = contents.len();

    construct_response(
      format!("{} {} {}", http_version, http_status, http_message).as_str(),
      contents, 
      content_type, 
      content_length
    )
  } 
}
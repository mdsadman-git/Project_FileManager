use crate::config::utility::construct_response;

pub struct Extra;

impl Extra {
  pub fn not_found() -> String {
    let http_version = "HTTP/1.1";
    let http_status = "404";
    let http_message = "Not Found";
    let content_type = "text/json";
    let contents = "{ \"test\": \"Not Found\" }";
    let content_length = contents.len();

    construct_response(
      format!("{} {} {}", http_version, http_status, http_message).as_str(),
      contents, 
      content_type, 
      content_length
    )
  }

  pub fn method_not_allowed() -> String {
    let http_version = "HTTP/1.1";
    let http_status = "405";
    let http_message = "Not Allowed";
    let content_type = "text/json";
    let contents = "{ \"test\": \"Method Not Allowed\" }";
    let content_length = contents.len();

    construct_response(
      format!("{} {} {}", http_version, http_status, http_message).as_str(),
      contents, 
      content_type, 
      content_length
    )
  } 
}
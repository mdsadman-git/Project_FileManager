use crate::parser::http_response::HttpResponse;

pub struct Extra;

impl Extra {
  pub fn not_found() -> String {
    let contents = r#"{ "test": "Not Found" }"#;
    let http_response = HttpResponse::new("404", "Not Found", contents);
    http_response.construct()
  }

  pub fn method_not_allowed() -> String {
    let contents = r#"{ "test": "Method Not Allowed" }"#;
    let http_response = HttpResponse::new("405", "Not Allowed", contents);
    http_response.construct()
  } 
}
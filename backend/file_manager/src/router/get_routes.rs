use crate::parser::http_response::HttpResponse;

pub struct Get;

impl Get {
  pub fn home() -> String {
    let contents = r#"{ "test": "test" }"#;
    let http_response = HttpResponse::new("200", "Ok", contents);
    http_response.construct()
  } 
}

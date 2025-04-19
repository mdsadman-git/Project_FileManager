#[derive(Debug)]
pub enum HttpRequestMethod {
  GET, POST, UPDATE, DELETE, NONE,
}

impl HttpRequestMethod {
  pub fn from(method: String) -> HttpRequestMethod {
    match method.as_str() {
      "GET" => HttpRequestMethod::GET,
      "POST" => HttpRequestMethod::POST,
      "UPDATE" => HttpRequestMethod::UPDATE,
      "DELETE" => HttpRequestMethod::DELETE,
      _ => HttpRequestMethod::NONE,
    }
  }
}
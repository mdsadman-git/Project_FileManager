#[derive(Debug, Clone)]
pub enum HttpRequestMethod {
  GET, POST, UPDATE, DELETE, NONE,
}

impl HttpRequestMethod {
  pub fn from(method: String) -> HttpRequestMethod {
    match method.as_str() {
      "GET"     => HttpRequestMethod::GET,
      "POST"    => HttpRequestMethod::POST,
      "UPDATE"  => HttpRequestMethod::UPDATE,
      "DELETE"  => HttpRequestMethod::DELETE,
      _         => HttpRequestMethod::NONE,
    }
  }
}

impl HttpRequestMethod {
  fn as_string(&self) -> String {
    match self {
      HttpRequestMethod::GET      => String::from("GET"),
      HttpRequestMethod::POST     => String::from("POST"),
      HttpRequestMethod::UPDATE   => String::from("UPDATE"),
      HttpRequestMethod::DELETE   => String::from("DELETE"),
      HttpRequestMethod::NONE     => String::from("NONE"),
    }
  }
}

impl std::fmt::Display for HttpRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("HttpRequest-Method {{ {} }}", self.as_string()))
    }
}
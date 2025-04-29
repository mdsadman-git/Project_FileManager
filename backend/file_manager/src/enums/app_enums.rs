#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
  DELETE, GET, NONE, POST, UPDATE,
}

impl HttpMethod {
  pub fn from(method: String) -> HttpMethod {
    match method.as_str() {
      "GET"     => HttpMethod::GET,
      "POST"    => HttpMethod::POST,
      "UPDATE"  => HttpMethod::UPDATE,
      "DELETE"  => HttpMethod::DELETE,
      _         => HttpMethod::NONE,
    }
  }
}

impl HttpMethod {
  pub fn as_string(&self) -> String {
    match self {
      HttpMethod::GET      => String::from("GET"),
      HttpMethod::POST     => String::from("POST"),
      HttpMethod::UPDATE   => String::from("UPDATE"),
      HttpMethod::DELETE   => String::from("DELETE"),
      HttpMethod::NONE     => String::from("NONE"),
    }
  }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("HttpRequest-Method {{ {} }}", self.as_string()))
    }
}
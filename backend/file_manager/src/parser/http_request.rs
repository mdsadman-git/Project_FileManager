use crate::enums::app_enums::HttpRequestMethod;

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpRequest {
  pub method: HttpRequestMethod,
  pub path: String,
  pub http_version: String,
  pub host: String,
  pub user_agent: String,
  pub accept: String,
  pub accept_language: String,
  pub accept_encoding: Vec<String>,
  pub connection: String,
  pub upgrade_insecure_requests: String,
  pub sec_fetch_dest: String,
  pub sec_fetch_mode: String,
  pub sec_fetch_site: String,
  pub priority: String,
}

struct HttpRequestParser {
  request: Vec<String>,
  temp: String,
}

impl HttpRequest {
  pub fn construct(request: Vec<String>) -> Self {
    let mut parser = HttpRequestParser::new(request);
    HttpRequest {
      method: HttpRequestMethod::from(parser.parse_line(0, 0).1),
      path: parser.parse_line(0, 1).1,
      http_version: parser.parse_line(0, 2).1,
      host: parser.parse_colon( 1).1,
      user_agent: parser.parse_colon( 2).1,
      accept: parser.parse_colon(3).1,
      accept_language: parser.parse_colon(4).1,
      accept_encoding: parser.parse_colon(5).0.separate_with(','),
      connection: parser.parse_colon(6).1,
      upgrade_insecure_requests: parser.parse_colon(7).1,
      sec_fetch_dest: parser.parse_colon(8).1,
      sec_fetch_mode: parser.parse_colon(9).1,
      sec_fetch_site: parser.parse_colon(10).1,
      priority: parser.parse_colon(11).1,
    }
  }
}

impl HttpRequestParser {
  fn new(request: Vec<String>) -> Self {
    Self { request, temp: String::new() }
  }

  fn separate_with(&self, c: char) -> Vec<String> {
    if self.temp.is_empty() {
      panic!("Value not found at temp! Separation failed");
    }

    self.temp.split(c).map(|x| String::from(x.trim())).collect()
  }

  fn parse_line(&self, index: usize, at: usize) -> (&Self, String) {
    let value = self.request.get(index);
    return match value {
      Some(r_value) => (self, r_value.split_whitespace().nth(at).unwrap().to_owned()),
      None => (self, String::new()), 
    };
  }

  fn parse_colon(&mut self, index: usize) -> (&Self, String) {
    let value = self.request.get(index);
    return match value {
      Some(v_result) => {
        let mut result = String::new();
        let mut is_concat = false;
        for c in v_result.chars() {
          if c != ':' && !is_concat {
            is_concat = true;
            continue;
          }

          result.push(c);
        }

        self.temp = result.clone();
        (self, result)
      },
      None => (self, String::new()),
    }
  }
}
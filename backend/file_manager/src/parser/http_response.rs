pub struct HttpResponse {
  pub version: String,
  pub status: String,
  pub message: String,
  pub content_type: String,
  pub content_length: usize,
  pub contents: String,
}

impl HttpResponse {
  pub fn new(status: impl Into<String>, message: impl Into<String>, contents: impl Into<String>) -> Self {
    HttpResponse::init("HTTP/1.1", status, message, "application/json", contents)
  }

  pub fn init(
    version: impl Into<String>, 
    status: impl Into<String>, 
    message: impl Into<String>, 
    content_type: impl Into<String>, 
    contents: impl Into<String>
  ) -> Self {
    let c: String = contents.into().clone();

    Self { 
      version: version.into(), 
      status: status.into(), 
      message: message.into(), 
      content_type: content_type.into(), 
      content_length: c.len(),
      contents: c.into(),
    }
  }
}

impl HttpResponse {
  pub fn construct(&self) -> String {
      format!(
        "{} {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", 
        self.version,
        self.status,
        self.message,
        self.content_type, 
        self.content_length, 
        self.contents
      )
  }
}
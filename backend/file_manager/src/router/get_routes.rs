use crate::{library::json::{Json, JsonBuilder}, parser::http_response::HttpResponse};

pub struct Get;

impl Get {
  pub fn home() -> String {
    let mut json_builder = Json::builder();
    json_builder.put("path", "home");
    json_builder.put("method", "get");
    json_builder.put("number", 123);
    json_builder.put("is_alright", true);

    let contents = json_builder.build();
    let http_response = HttpResponse::new("200", "Ok", contents);
    http_response.construct()
  } 
}

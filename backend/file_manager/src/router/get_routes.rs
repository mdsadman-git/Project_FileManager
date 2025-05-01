use crate::{library::json::{Json, JsonBuilder, JsonNull}, parser::http_response::HttpResponse};

pub struct Get;

impl Get {
  pub fn home() -> String {
    let mut json_object = Json::object();
    json_object.insert("path", "home");
    json_object.insert("method", "get");
    json_object.insert("number", 123);
    json_object.insert("is_alright", true);
    json_object.insert("is_null", JsonNull::new());


    let contents = Json::build(json_object);
    let http_response = HttpResponse::new("200", "Ok", contents);
    http_response.construct()
  } 
}

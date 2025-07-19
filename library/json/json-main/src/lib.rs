use crate::builder::{object::JsonObject, array::JsonArray, main::JsonBuilder};
use crate::parser::parser::JsonParser;

pub mod builder;
pub mod parser;
pub mod ast;

pub struct Json;

impl JsonBuilder for Json {
  fn object() -> JsonObject {
    JsonObject::new()
  }

  fn array() -> JsonArray {
    JsonArray::new()
  }
}

impl JsonParser for  Json {
    
}
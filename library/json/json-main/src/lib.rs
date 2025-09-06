use crate::builder::main::JsonBuilder;
use crate::builder::{object::JsonBuilderObject, array::JsonBuilderArray};
use crate::parser::main::JsonParser;
use crate::parser::parser::JsonTextParser;
use crate::types::types::{JsonTypeArray, JsonTypeObject};

pub mod builder;
pub mod parser;
pub mod ast;
pub mod types;

pub struct Json {
  jtp: JsonTextParser 
}

impl JsonBuilder for Json {
  fn builder_object() -> JsonBuilderObject {
    JsonBuilderObject::new()
  }

  fn builder_array() -> JsonBuilderArray {
    JsonBuilderArray::new()
  }

  // TODO: NEED TO WORK FOR THIS GLOBAL TYPES TO BUILD INTO JSON TEXT
  fn object() -> JsonTypeObject {
    JsonTypeObject::new()
  }
  
  // TODO: NEED TO WORK FOR THIS GLOBAL TYPES TO BUILD INTO JSON TEXT
  fn array() -> JsonTypeArray {
    JsonTypeArray::new()
  }
}

impl JsonParser for Json {
  fn parse(&mut self) -> &mut Self {
    self.jtp.parse();
    self
  }
  
  fn parser(json: String) -> Self {
    Self { jtp: JsonTextParser::new(json) }
  }
  
  fn get<T: 'static>(&mut self) -> &T {
    self.jtp.get()
  }
  
  fn get_mut<T: 'static>(&mut self) -> &mut T {
    self.jtp.get_mut()
  }
}

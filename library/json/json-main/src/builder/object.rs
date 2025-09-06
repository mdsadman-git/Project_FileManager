use std::collections::HashMap;
use logger_main::Logger;

use crate::builder::value::JsonBuilderValue;

#[derive(Debug, Clone)]
pub struct JsonBuilderObject {
  pub object: HashMap<String, JsonBuilderValue>,
}

impl JsonBuilderObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }
}

impl JsonBuilderObject {
  pub fn insert(&mut self, k: impl Into<String>, v: impl Into<JsonBuilderValue>) -> &mut Self {
    let (json_key , json_value) = (k.into(), v.into());
    Logger::debug(format!("Append Value | {} => {}", json_key, json_value));
    self.object.insert(json_key, json_value);
    self
  }

  pub fn remove(&mut self, k: impl Into<String>) -> &mut Self {
    let json_key = k.into();
    let json_value = self.object.remove(&json_key);
    Logger::debug(format!("Delete Value | {} => {}", json_key, json_value.unwrap()));
    self
  }

  pub fn get(&mut self, k: impl Into<String>) -> Option<&JsonBuilderValue> {
    self.object.get(&k.into())
  }

  pub fn get_mut(&mut self, k: impl Into<String>) -> Option<&mut JsonBuilderValue> {
    self.object.get_mut(&k.into())
  }
}

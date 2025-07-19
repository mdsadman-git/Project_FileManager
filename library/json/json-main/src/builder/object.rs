use std::collections::HashMap;
use logger_main::Logger;

use crate::builder::value::JsonValue;

// TODO: CHANGE THE HASH MAP TO CUSTOM DS

#[derive(Debug)]
pub struct JsonObject {
  pub object: HashMap<String, JsonValue>,
}

impl JsonObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }
}

impl JsonObject {
  pub fn insert(&mut self, k: impl Into<String>, v: impl Into<JsonValue>) -> &mut Self {
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

  pub fn get(&mut self, k: impl Into<String>) -> Option<&JsonValue> {
    self.object.get(&k.into())
  }
}

use std::{collections::HashMap, fmt::Display, vec};

use crate::{hashmap, logger::app_logger::Logger};

pub struct Json {
  json_map: Option<HashMap<String, String>>,
}

pub trait JsonParser {
  fn perser() -> impl JsonParser;
  fn parse(&self, json: String);
}

impl JsonParser for Json {
  fn perser() -> impl JsonParser {
    Self { json_map: Option::None }
  }

  fn parse(&self, json: String) {
    todo!()
  }
}

pub trait JsonBuilder {
  fn builder() -> impl JsonBuilder;
  fn put(&mut self, key: impl Into<String>, value: impl Into<JsonValue>);
  fn build(&self) -> String;
}

impl JsonBuilder for Json {
  fn builder() -> impl JsonBuilder {
    Self { json_map: Option::Some(HashMap::new()) }
  }

  fn put(&mut self, key: impl Into<String>, value: impl Into<JsonValue>) {
    let json_key = key.into();
    let json_value: JsonValue = value.into();
    Logger::debug(format!("New Value | {} => {}", json_key, json_value));

    if !json_value.dt.is_primitive() {
      return;
    }


    self.json_map.as_mut().unwrap().insert(format!(r#""{}""#, json_key), json_value.value);
  }

  fn build(&self) -> String {
    let map = self.json_map.as_ref().expect("Failed to achieve json map!");
    let mut result = Vec::new();
    for e in map {
      result.push(format!("{}:{}", e.0, e.1));
    }

    format!("{{ {} }}", result.join(","))
  }
}

#[derive(Debug, PartialEq, Eq)]
enum JsonType {
  Number, String, Boolean, Null, Object, Array
}

impl Display for JsonType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{: <16}", format!("JsonType.{:?}", self).as_str()).as_str())
  }
}

impl JsonType {
  fn is_primitive(self) -> bool {
    self == JsonType::Number || self == JsonType::String || self == JsonType::Boolean || self == JsonType::Null
  }
}

#[derive(Debug)]
pub struct JsonValue {
  dt: JsonType,
  value: String,
}

impl Display for JsonValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("JsonValue {{ dt: {}, value: {} }}", self.dt, self.value).as_str())
  }
}

struct JsonTypeNull;
struct JsonTypeObject {
  object: HashMap<String, String>,
}
struct JsonTypeArray {
  array: Vec<JsonTypeObject>,
}

// ARRAY
impl Into<JsonValue> for JsonTypeArray {
  fn into(self) -> JsonValue {
    let mut v = Vec::new();
    for e in self.array {
      let json_object: JsonValue = e.into();
      v.push(format!("{}", json_object.value));
    }

    JsonValue { dt: JsonType::Array, value: format!("[{}]", v.join(",")) }
  }
}

// OBJECT 
impl Into<JsonValue> for JsonTypeObject {
  fn into(self) -> JsonValue {
    let mut v = Vec::new();
    for e in self.object {
      let json_value: JsonValue = e.1.into();
      v.push(format!("\"{}\": {}", e.0, json_value.value)); 
    } 

    JsonValue { dt: JsonType::Object, value: format!("{{ {} }}", v.join(",")) }
  }
}

// NULL
impl Into<JsonValue> for JsonTypeNull {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Null, value: format!("null") }
  }
}

// BOOLEAN 
impl Into<JsonValue> for bool {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Boolean, value: format!("{}", self) }
  }
}

// STRING 
impl Into<JsonValue> for &str {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("\"{}\"", self) }
  }
}

impl Into<JsonValue> for String {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("\"{}\"", self) }
  }
}

// NUMBER - F
impl Into<JsonValue> for f64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for f32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

// NUMBER - I
impl Into<JsonValue> for i128 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i16 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i8 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

// NUMBER - U
impl Into<JsonValue> for u128 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u16 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u8 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

#[cfg(test)]
mod tests {
  use super::{Json, JsonBuilder};

  #[test]
  fn json_value_test() {
    let mut json_builder = Json::builder();
    json_builder.put("key1", "string1");
    json_builder.put("key2", 123);
    json_builder.put("key3", true);

    let json = json_builder.build();
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }
}
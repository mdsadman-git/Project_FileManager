use std::{collections::HashMap, fmt::Display};

use logger_main::Logger;

pub struct Json;

#[allow(dead_code)]
pub trait JsonParser {
  fn perser() -> impl JsonParser;
  fn parse(&self, json: String);
}

impl JsonParser for Json {
  fn perser() -> impl JsonParser {
    Self { }
  }

  fn parse(&self, json: String) {
    todo!()
  }
}

#[allow(dead_code)]
pub trait JsonBuilder {
  fn object() -> JsonObject;
  fn array() -> JsonArray;
  fn build(json_container: impl Into<JsonContainer>) -> String {
    (json_container.into() as JsonContainer).result
  }
}

impl JsonBuilder for Json {
  fn object() -> JsonObject {
    JsonObject::new()
  }

  fn array() -> JsonArray {
    JsonArray::new()
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

pub struct JsonNull;

impl JsonNull {
  pub fn new() -> Self { 
    Self {} 
  }
}

pub struct JsonContainer {
  result: String,
}

impl Into<JsonContainer> for JsonObject {
  fn into(self) -> JsonContainer {
    let mut result = Vec::new(); 
    for e in self.object.iter() {
      result.push(format!(r#""{}":{}"#, *e.0, *e.1));
    }

    JsonContainer { result: format!("{{{}}}", result.join(",")) }
  }
}

impl Into<JsonContainer> for JsonArray {
  fn into(self) -> JsonContainer {
    let json_array: JsonValue = self.into();
    JsonContainer { result: json_array.value }
  }
}

pub struct JsonObject {
  object: HashMap<String, String>,
}

impl JsonObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }

pub fn insert(&mut self, k: impl Into<String>, v: impl Into<JsonValue>) -> &mut Self {
    let (json_key , json_value) = (k.into(), v.into());
    Logger::debug(format!("Append Value | {} => {}", json_key, json_value));
    self.object.insert(json_key, json_value.value);
    self
  }

  pub fn remove(&mut self, k: impl Into<String>) -> &mut Self {
    let json_key = k.into();
    let json_value = self.object.remove(&json_key);
    Logger::debug(format!("Delete Value | {} => {}", json_key, json_value.unwrap_or(String::new())));
    self
  }
}

pub struct JsonArray {
  array: Vec<JsonObject>,
}

impl JsonArray {
  fn new() -> Self {
    Self { array: Vec::new() }
  }

  fn append(&mut self, o: JsonObject) -> &mut Self {
    self.array.push(o);
    self
  }

  fn remove(&mut self, index: usize) -> &mut Self {
    let _ = self.array.remove(index);
    self
  }
}

// ARRAY
impl Into<JsonValue> for JsonArray {
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
impl Into<JsonValue> for JsonObject {
  fn into(self) -> JsonValue {
    let mut s = String::new();
    for (i, (k, v)) in self.object.iter().enumerate() {
      s.push_str(format!("\"{}\":{}", k, v).as_str());
      if i < self.object.len() - 1 { s.push(',') };
    } 

    JsonValue { dt: JsonType::Object, value: format!("{{{}}}", s) }
  }
}

// NULL
impl Into<JsonValue> for JsonNull {
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
  use crate::library::json::JsonNull;

use super::{Json, JsonBuilder};

  #[test]
  fn json_array_test() {
    let mut json_object_2 = Json::object();
    json_object_2.insert("key1", "string2");
    json_object_2.insert("key2", 456);
    json_object_2.insert("key3", false);
    json_object_2.insert("key4", JsonNull::new());

    let mut json_object_3 = Json::object();
    json_object_3.insert("key1", "string2");
    json_object_3.insert("key2", 456);
    json_object_3.insert("key3", false);
    json_object_3.insert("key4", JsonNull::new());

    let mut json_array = Json::array();
    json_array.append(json_object_2).append(json_object_3);

    let json = Json::build(json_array);
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }

  #[test]
  fn json_object_test() {
    let mut json_object_1 = Json::object();
    json_object_1.insert("key1", "string1");
    json_object_1.insert("key2", 456);
    json_object_1.insert("key3", false);
    json_object_1.insert("key4", JsonNull::new());

    let mut json_object_2 = Json::object();
    json_object_2.insert("key1", "string2");
    json_object_2.insert("key2", 456);
    json_object_2.insert("key3", false);
    json_object_2.insert("key4", JsonNull::new());

    let mut json_object_3 = Json::object();
    json_object_3.insert("key1", "string2");
    json_object_3.insert("key2", 456);
    json_object_3.insert("key3", false);
    json_object_3.insert("key4", JsonNull::new());

    let mut json_array = Json::array();
    json_array.append(json_object_2);
    json_array.append(json_object_3);

    let mut json_object = Json::object();
    json_object
      .insert("key1", "string0")
      .insert("key2", 123)
      .insert("key3", true)
      .insert("key4", JsonNull::new())
      .insert("object_1", json_object_1)
      .insert("array", json_array);

    let json = Json::build(json_object);
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }
}
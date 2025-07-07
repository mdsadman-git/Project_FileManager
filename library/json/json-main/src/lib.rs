use std::{collections::HashMap, fmt::Display};

use logger_main::Logger;

pub struct Json;

mod ast;

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

impl JsonValue {
  pub fn to_i32(&self) -> i32 {
    self.panic_when_invalid_type(JsonType::Number);
    self.value.parse().expect(format!("Unable to parse value! Value: '{:?}'", self.value).as_str())
  }

  pub fn to_i64(&self) -> i64 {
    self.panic_when_invalid_type(JsonType::Number);
    self.value.parse().expect(format!("Unable to parse value! Value: '{:?}'", self.value).as_str())
  }
}

impl JsonValue { // Panics
  fn panic_when_invalid_type(&self, json_type: JsonType) {
    if self.dt != json_type {
      panic!("Invalid json datatype! JsonType: {}", self.dt)
    }
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
      match e.1.dt {
        JsonType::String => result.push(format!(r#""{}":"{}""#, *e.0, e.1.value)),
        _                => result.push(format!(r#""{}":{}"#, *e.0, e.1.value)),
      }
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

#[derive(Debug)]
pub struct JsonObject {
  object: HashMap<String, JsonValue>,
}

impl JsonObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }

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

pub struct JsonArray {
  array: Vec<JsonValue>,
}

impl JsonArray {
  pub fn new() -> Self {
    Self { array: Vec::new() }
  }

  pub fn append(&mut self, o: impl Into<JsonValue>) -> &mut Self {
    self.array.push(o.into());
    self
  }

  pub fn remove(&mut self, index: usize) -> &mut Self {
    let _ = self.array.remove(index);
    self
  }
}

pub trait JsonBlock {}
impl JsonBlock for JsonObject {}
impl JsonBlock for JsonArray {}

// VECTOR 
impl <T: Into<JsonValue>> Into<JsonValue> for Vec<T> {
  fn into(self) -> JsonValue {
    let mut v: Vec<String> = Vec::new();
    for e in self {
      let json_value: JsonValue = e.try_into().expect("Unknown type for Json!");
      match json_value.dt {
        JsonType::String => v.push(format!(r#""{}""#, json_value.value)),
        _ => v.push(format!("{}", json_value.value)),
      }
    }

    JsonValue { dt: JsonType::Array, value: format!("[{}]", v.join(",")) }
  }
}

// ARRAY
impl Into<JsonValue> for JsonArray {
  fn into(self) -> JsonValue {
    let mut v = Vec::new();
    for e in self.array {
      let json_object: JsonValue = e.into();
      match json_object.dt {
        JsonType::String => v.push(format!(r#""{}""#, json_object.value)),
        _ => v.push(format!("{}", json_object.value))
      }
    }

    JsonValue { dt: JsonType::Array, value: format!("[{}]", v.join(",")) }
  }
}

// OBJECT 
impl Into<JsonValue> for JsonObject {
  fn into(self) -> JsonValue {
    let mut s = String::new();
    for (i, (k, v)) in self.object.iter().enumerate() {
      match v.dt {
        JsonType::String => s.push_str(format!(r#""{}":"{}""#, k, v.value).as_str()),
        _ => s.push_str(format!(r#""{}":{}"#, k, v.value).as_str()),
      }
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

impl Into<bool> for &JsonValue {
  fn into(self) -> bool {
    if self.dt != JsonType::Boolean { panic!("Json Type must be boolean! Found: {}", self.dt); }
    self.value.parse::<bool>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<bool>> for &JsonValue {
  fn into(self) -> Option<bool> {
    match self.dt { 
      JsonType::Boolean => 
        return Option::Some(self.value.parse::<bool>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be boolean! Found: {}", self.dt); 
  }
}

// STRING 
impl Into<JsonValue> for &str {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for String {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("{}", self) }
  }
}

impl Into<String> for &JsonValue {
  fn into(self) -> String {
    if self.dt != JsonType::String { panic!("Json Type must be string! Found: {}", self.dt); }
    self.value.parse::<String>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<String>> for &JsonValue {
  fn into(self) -> Option<String> {
    match self.dt {
      JsonType::String => 
        return Option::Some(self.value.parse::<String>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be string! Found: {}", self.dt); 
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

impl Into<i32> for &JsonValue {
  fn into(self) -> i32 {
    if self.dt != JsonType::Number { panic!("Json Type must be number! Found: {}", self.dt); }
    self.value.parse::<i32>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<i32>> for &JsonValue {
  fn into(self) -> Option<i32> {
    match self.dt {
      JsonType::Number =>  
        return Option::Some(self.value.parse::<i32>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be number! Found: {}", self.dt); 
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

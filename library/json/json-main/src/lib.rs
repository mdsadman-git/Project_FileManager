use std::{collections::HashMap, fmt::Display};

use logger_main::Logger;

pub struct Json;

mod ast;

// LOCAL MACROS 
macro_rules! ty_into_jv { // Type Into JsonValue
  ($_type:ty, $dt:path) => {
    impl Into<JsonValue> for $_type {
      fn into(self) -> JsonValue {
        JsonValue { dt: $dt, value: format!("{}", self) }
      }
    }
  };
}

macro_rules! rjv_into_ty {
  ($_type:ty) => {
    impl Into<$_type> for &JsonValue {
      fn into(self) -> $_type {
        if self.dt != JsonType::Number { panic!("Json Type must be number! Found: {}", self.dt); }
        self.value.parse::<$_type>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
      }
    }

    impl Into<Option<$_type>> for &JsonValue {
      fn into(self) -> Option<$_type> {
        match self.dt {
          JsonType::Number =>  
            return Option::Some(self.value.parse::<$_type>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
          JsonType::Null => 
            return Option::None,
          _ => {}
        }
      
        panic!("Json Type must be number! Found: {}", self.dt); 
      }
    }
  };
}
// LOCAL MACROS 

#[allow(dead_code)]
pub trait JsonBuilder {
  fn object() -> JsonObject;
  fn array() -> JsonArray;
  // fn container(); // TODO: THIS THE THIRD OPTION | A OBJECT BASED LINKED LIST
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

#[derive(Debug)]
pub struct JsonNull;

impl JsonNull {
  pub fn new() -> Self { 
    Self {} 
  }
}

impl Display for JsonNull {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("null")
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

// NullType
ty_into_jv!(JsonNull, JsonType::Null);

// BoolType Into JsonValue
ty_into_jv!(bool, JsonType::Boolean);

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

// StringType Into JsonValue 
ty_into_jv!(&str, JsonType::String);
ty_into_jv!(String, JsonType::String);

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

// NumericType Into JsonValue
ty_into_jv!(f64, JsonType::Number);
ty_into_jv!(f32, JsonType::Number);
ty_into_jv!(i128, JsonType::Number);
ty_into_jv!(i64, JsonType::Number);
ty_into_jv!(i32, JsonType::Number);
ty_into_jv!(i16, JsonType::Number);
ty_into_jv!(i8, JsonType::Number);
ty_into_jv!(u128, JsonType::Number);
ty_into_jv!(u64, JsonType::Number);
ty_into_jv!(u32, JsonType::Number);
ty_into_jv!(u16, JsonType::Number);
ty_into_jv!(u8, JsonType::Number);

// &JsonValue Into NumericType
rjv_into_ty!(f64);
rjv_into_ty!(f32);
rjv_into_ty!(i128);
rjv_into_ty!(i64);
rjv_into_ty!(i32);
rjv_into_ty!(i16);
rjv_into_ty!(i8);
rjv_into_ty!(u128);
rjv_into_ty!(u64);
rjv_into_ty!(u32);
rjv_into_ty!(u16);
rjv_into_ty!(u8);

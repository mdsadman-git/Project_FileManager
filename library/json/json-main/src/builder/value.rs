use crate::builder::{array::JsonBuilderArray, object::JsonBuilderObject, types::JsonType};

#[derive(Debug, Clone)]
pub struct JsonBuilderValue {
  pub dt: JsonType,
  pub value: String,
}

impl JsonBuilderValue {
  pub fn to_i32(&self) -> i32 {
    self.panic_when_invalid_type(JsonType::Number);
    self.value.parse().expect(format!("Unable to parse value! Value: '{:?}'", self.value).as_str())
  }

  pub fn to_i64(&self) -> i64 {
    self.panic_when_invalid_type(JsonType::Number);
    self.value.parse().expect(format!("Unable to parse value! Value: '{:?}'", self.value).as_str())
  }

  pub fn to_object(&self) -> JsonBuilderObject {
    self.panic_when_invalid_type(JsonType::Object);
    self.into()
  }

  pub fn to_array(&self) -> JsonBuilderArray {
    self.panic_when_invalid_type(JsonType::Array);
    self.into()
  }
}

// Display Trait
impl std::fmt::Display for JsonBuilderValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("JsonValue {{ dt: {}, value: {} }}", self.dt, self.value).as_str())
  }
}

// Panics
impl JsonBuilderValue {
  fn panic_when_invalid_type(&self, json_type: JsonType) {
    if self.dt != json_type {
      panic!("Invalid json datatype! JsonType: {}", self.dt)
    }
  }
}
use crate::builder::types::JsonType;

#[derive(Debug)]
pub struct JsonValue {
  pub dt: JsonType,
  pub value: String,
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

// Display Trait
impl std::fmt::Display for JsonValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("JsonValue {{ dt: {}, value: {} }}", self.dt, self.value).as_str())
  }
}

// Panics
impl JsonValue {
  fn panic_when_invalid_type(&self, json_type: JsonType) {
    if self.dt != json_type {
      panic!("Invalid json datatype! JsonType: {}", self.dt)
    }
  }
}
#[derive(Debug, PartialEq, Eq)]
pub enum JsonType { Number, String, Boolean, Null, Object, Array }

impl JsonType {
  pub fn is_primitive(self) -> bool {
    self == JsonType::Number || self == JsonType::String || self == JsonType::Boolean || self == JsonType::Null
  }
}

// Display Trait
impl std::fmt::Display for JsonType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{: <16}", format!("JsonType.{:?}", self).as_str()).as_str())
  }
}

// Custom Types - Null

#[derive(Debug)]
pub struct JsonNull;

impl JsonNull {
  pub fn new() -> Self { 
    Self {} 
  }
}

// Display Trait
impl std::fmt::Display for JsonNull {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("null")
  }
}
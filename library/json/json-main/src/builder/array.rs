use crate::builder::value::JsonValue;

#[derive(Debug)]
pub struct JsonArray {
  pub array: Vec<JsonValue>,
}

impl JsonArray {
  pub fn new() -> Self {
    Self { array: Vec::new() }
  }
}

impl JsonArray {
  pub fn append(&mut self, o: impl Into<JsonValue>) -> &mut Self {
    self.array.push(o.into());
    self
  }

  pub fn remove(&mut self, index: usize) -> &mut Self {
    let _ = self.array.remove(index);
    self
  }
}
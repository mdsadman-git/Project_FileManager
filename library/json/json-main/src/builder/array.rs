use crate::builder::value::JsonBuilderValue;

#[derive(Debug, Clone)]
pub struct JsonBuilderArray {
  pub array: Vec<JsonBuilderValue>,
}

impl JsonBuilderArray {
  pub fn new() -> Self {
    Self { array: Vec::new() }
  }
}

impl JsonBuilderArray {
  pub fn append(&mut self, o: impl Into<JsonBuilderValue>) -> &mut Self {
    self.array.push(o.into());
    self
  }

  pub fn remove(&mut self, index: usize) -> &mut Self {
    let _ = self.array.remove(index);
    self
  }
}
use crate::builder::object::JsonObject;
use crate::builder::array::JsonArray;
use crate::builder::types::JsonType;
use crate::builder::value::JsonValue;

pub struct JsonComponent {
  pub(in crate::builder) result: String,
}

// Into Trait
impl Into<JsonComponent> for JsonObject {
  fn into(self) -> JsonComponent {
    let mut result = Vec::new(); 
    for e in self.object.iter() {
      match e.1.dt {
        JsonType::String => result.push(format!(r#""{}":"{}""#, *e.0, e.1.value)),
        _                => result.push(format!(r#""{}":{}"#, *e.0, e.1.value)),
      }
    }

    JsonComponent { result: format!("{{{}}}", result.join(",")) }
  }
}

impl Into<JsonComponent> for JsonArray {
  fn into(self) -> JsonComponent {
    let v: JsonValue = self.into();
    JsonComponent { result: v.value }
  }
}

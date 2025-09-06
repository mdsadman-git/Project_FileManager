use crate::builder::object::JsonBuilderObject;
use crate::builder::array::JsonBuilderArray;
use crate::builder::types::JsonType;
use crate::builder::value::JsonBuilderValue;

pub struct JsonBuilderComponent {
  pub(in crate) result: String,
}

impl Into<JsonBuilderComponent> for JsonBuilderObject {
  fn into(self) -> JsonBuilderComponent {
    let mut result = Vec::new(); 
    for e in self.object.iter() {
      match e.1.dt {
        JsonType::String => result.push(format!(r#""{}":"{}""#, *e.0, e.1.value)),
        _                => result.push(format!(r#""{}":{}"#, *e.0, e.1.value)),
      }
    }

    JsonBuilderComponent { result: format!("{{{}}}", result.join(",")) }
  }
}

impl Into<JsonBuilderComponent> for JsonBuilderArray {
  fn into(self) -> JsonBuilderComponent {
    let v: JsonBuilderValue = self.into();
    JsonBuilderComponent { result: v.value }
  }
}

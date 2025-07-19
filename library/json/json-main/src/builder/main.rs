use crate::builder::component::JsonComponent;
use crate::builder::array::JsonArray;
use crate::builder::object::JsonObject;

pub trait JsonBuilder {
  fn object() -> JsonObject;
  fn array() -> JsonArray;
  fn build(jc: impl Into<JsonComponent>) -> String {
    (jc.into() as JsonComponent).result
  }
}

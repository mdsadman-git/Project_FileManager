use crate::builder::component::JsonBuilderComponent;
use crate::builder::array::JsonBuilderArray;
use crate::builder::object::JsonBuilderObject;
use crate::types::types::{JsonTypeArray, JsonTypeObject};

pub trait JsonBuilder {
  fn builder_object() -> JsonBuilderObject;
  fn builder_array() -> JsonBuilderArray;

  fn object() -> JsonTypeObject;
  fn array() -> JsonTypeArray;

  fn build(jc: impl Into<JsonBuilderComponent>) -> String {
    (jc.into() as JsonBuilderComponent).result
  }
}

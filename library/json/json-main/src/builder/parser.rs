use std::any::Any;

use crate::{ast::{parser::{
  ArrayExpression, ArrayLiteral, BooleanLiteral, KeyLiteral, LiteralValueTrait, NullLiteral, 
  NumericLiteral, ObjectExpression, ObjectLiteral, StringLiteral, ValueLiteral
}, traverser::{JsonTraverser, JsonTraverserTrait}}, builder::{array::JsonBuilderArray, object::JsonBuilderObject, types::JsonBuilderNull}};

macro_rules! entity_impl {
  ($($identifier:ident),*) => {
    $(
      impl JsonEntity for $identifier {
        fn debug_str(&self) -> String {
          format!("{:?}", self) 
        }

        fn to_ref(&self) -> &dyn Any {
          self
        }

        fn to_mut(&mut self) -> &mut dyn Any {
          self
        }
      }
    )*
  };
}

trait JsonEntity {
  fn debug_str(&self) -> String;
  fn to_ref(&self) -> &dyn Any;
  fn to_mut(&mut self) -> &mut dyn Any;
}

impl std::fmt::Debug for dyn JsonEntity + 'static {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.debug_str())
  }
}

entity_impl!(JsonBuilderObject, JsonBuilderArray);

pub(crate) struct JsonBuilderParser {
  json: String,
  container: JsonContainer,
}

impl JsonBuilderParser {
  pub(crate) fn new(json: String) -> Self {
    Self { json, container: JsonContainer::new() }
  }    
}

impl JsonBuilderParser {
  pub(crate) fn get<T: 'static>(&mut self) -> &T {
    if let Some(v) = self.container.get_last() {
      return v.to_ref().downcast_ref::<T>().take().unwrap();
    }

    panic!("Json Parser Exception! Unknown type found while getting data.");
  } 

  pub(crate) fn parse(&mut self) -> &mut Self {
    let mut traverser = JsonTraverser::new(self.json.clone());
    traverser.setup(Box::new(self)).parse();
    self
  }
}

impl JsonTraverserTrait for JsonBuilderParser {
  fn object_expression_main(&mut self, _: &mut ObjectExpression, _: &mut Vec<String>, _: usize) {
    self.container.push_entity(Box::new(JsonBuilderObject::new()));
  }

  fn object_expression_after(&mut self, tracker: &mut Vec<String>, level: usize) {
    let last = self.container.pop_entity();
    if let Some(mut entity) = last {
      if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
        item.insert(tracker.get(level).unwrap(), entity.to_mut().downcast_mut::<JsonBuilderObject>().unwrap().clone());
      } else
      if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
        item.append(entity.to_mut().downcast_mut::<JsonBuilderArray>().unwrap().clone());
      }
    }
  }

  fn array_expression_main(&mut self, _: &mut ArrayExpression, _: &mut Vec<String>, _: usize) {
    self.container.push_entity(Box::new(JsonBuilderArray::new()));
  }

  fn array_expression_after(&mut self, tracker: &mut Vec<String>, level: usize) {
    let last = self.container.pop_entity();
    if let Some(mut entity) = last {
      if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
        item.insert(tracker.get(level).unwrap(), entity.to_mut().downcast_mut::<JsonBuilderObject>().unwrap().clone());
      } else
      if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
        item.append(entity.to_mut().downcast_mut::<JsonBuilderArray>().unwrap().clone());
      }
    }
  }

  fn object_literal_before(&mut self, _: &mut ObjectLiteral, _: &mut Vec<String>, _: usize) {
    // PASS
  }

  fn object_literal_after(&mut self, _: &mut ObjectLiteral, tracker: &mut Vec<String>, _: usize) {
    tracker.pop();
  }

  fn array_literal(&mut self, _: &mut ArrayLiteral, _: &mut Vec<String>, _: usize) {
    // PASS
  }

  fn key_literal(&mut self, _: &mut KeyLiteral, _: &mut Vec<String>, _: usize) {
    // PASS
  }

  fn value_literal(&mut self, _: &mut ValueLiteral, _: &mut Vec<String>, _: usize) {
    // PASS
  }

  fn string_literal(&mut self, v: &mut StringLiteral, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      if tracker.get(level).is_none() {
        tracker.push(v.get()); 
      } else { 
        item.insert(
          tracker.get(level).unwrap(), 
          v.get()
        );
      }
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn boolean_literal(&mut self, v: &mut BooleanLiteral, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    } 
  }

  fn null_literal(&mut self, _: &mut NullLiteral, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        JsonBuilderNull::new()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(JsonBuilderNull::new());
    }
  }

  fn numeric_literal_u8(&mut self, v: &mut NumericLiteral<u8>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_u16(&mut self, v: &mut NumericLiteral<u16>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_u32(&mut self, v: &mut NumericLiteral<u32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_u64(&mut self, v: &mut NumericLiteral<u64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_i8(&mut self, v: &mut NumericLiteral<i8>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_i16(&mut self, v: &mut NumericLiteral<i16>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_i32(&mut self, v: &mut NumericLiteral<i32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_i64(&mut self, v: &mut NumericLiteral<i64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_f32(&mut self, v: &mut NumericLiteral<f32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }

  fn numeric_literal_f64(&mut self, v: &mut NumericLiteral<f64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderObject>() {
      item.insert(
        tracker.get(level).unwrap(),
        v.get()
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonBuilderArray>() {
      item.append(v.get());
    }
  }
}

#[derive(Debug)]
struct JsonContainer {
  jev: Vec<Box<dyn JsonEntity>>
}

impl JsonContainer {
  fn new() -> Self {
    Self { jev: Vec::new() }
  } 
}

impl JsonContainer {
  fn get_last(&mut self) -> Option<&Box<dyn JsonEntity>> {
    self.jev.last()
  }

  fn get_last_mut(&mut self) -> Option<&mut Box<dyn JsonEntity>> {
    self.jev.last_mut()
  }

  fn get_last_mut_generic<T: 'static>(&mut self) -> Option<&mut T> {
    if let Some(je) = self.get_last_mut() {
      return je.to_mut().downcast_mut::<T>();
    }

    None
  }

  fn push_entity(&mut self, je: Box<dyn JsonEntity>) {
    self.jev.push(je);
  }

  fn pop_entity(&mut self) -> Option<Box<dyn JsonEntity>> {
    if self.jev.len() > 1 {
      return self.jev.pop();
    }

    None
  }
}

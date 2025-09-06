use crate::{ast::{parser::{
  ArrayExpression, ArrayLiteral, BooleanLiteral, KeyLiteral, 
  LiteralValueTrait, NullLiteral, NumericLiteral, ObjectExpression,
  ObjectLiteral, StringLiteral, ValueLiteral
}, traverser::{JsonTraverser, JsonTraverserTrait}}, 
types::types::{JsonType, JsonTypeArray, JsonTypeBoolean, JsonTypeNull, JsonTypeNumeric, JsonTypeObject, JsonTypeString}};

pub struct JsonTextParser {
  container: JsonContainer,
  json: String,
}

impl JsonTextParser {
  pub fn new(json: String) -> Self {
    Self { json, container: JsonContainer::new() }
  }    
}

impl JsonTextParser {
  pub fn get<T: 'static>(&mut self) -> &T {
    if let Some(v) = self.container.get_last() {
      return v.to_ref().downcast_ref::<T>().take().unwrap();
    }

    panic!("Json Parser Exception! Unknown type found while getting data.");
  } 

  pub fn get_mut<T: 'static>(&mut self) -> &mut T {
    if let Some(v) = self.container.get_last_mut() {
      return v.to_mut().downcast_mut::<T>().take().unwrap();
    }

    panic!("Json Parser Exception! Unknown type found while getting data.");
  } 

  pub(crate) fn parse(&mut self) {
    let mut traverser = JsonTraverser::new(self.json.as_str());
    traverser.setup(Box::new(self)).parse();
  }
}

impl JsonTraverserTrait for JsonTextParser {
  fn object_expression_main(&mut self, _: &mut ObjectExpression, _: &mut Vec<String>, _: usize) {
    self.container.push_entity(Box::new(JsonTypeObject::new()));
  }

  fn object_expression_after(&mut self, tracker: &mut Vec<String>, level: usize) {
    let last = self.container.pop_entity();
    if let Some(entity) = last {
      if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
        item.add(tracker.get(level).unwrap(), entity);
      } else
      if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
        item.add(entity);
      }
    }
  }

  fn array_expression_main(&mut self, _: &mut ArrayExpression, _: &mut Vec<String>, _: usize) {
    self.container.push_entity(Box::new(JsonTypeArray::new()));
  }

  fn array_expression_after(&mut self, tracker: &mut Vec<String>, level: usize) {
    let last = self.container.pop_entity();
    if let Some(entity) = last {
      if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
        item.add(tracker.get(level).unwrap(), entity);
      } else
      if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
        item.add(entity);
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
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      if tracker.get(level).is_none() {
        tracker.push(v.get()); 
      } else { 
        item.add(
          tracker.get(level).unwrap(), 
          Box::new(JsonTypeString::new(v.get()))
        );
      }
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeString::new(v.get())));
    }
  }

  fn boolean_literal(&mut self, v: &mut BooleanLiteral, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeBoolean::new(v.get()))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeBoolean::new(v.get())));
    } 
  }

  fn null_literal(&mut self, _: &mut NullLiteral, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNull::new())
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNull::new()));
    }
  }

  fn numeric_literal_u8(&mut self, v: &mut NumericLiteral<u8>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as usize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as usize)));
    }
  }

  fn numeric_literal_u16(&mut self, v: &mut NumericLiteral<u16>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as usize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as usize)));
    }
  }

  fn numeric_literal_u32(&mut self, v: &mut NumericLiteral<u32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as usize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as usize)));
    }
  }

  fn numeric_literal_u64(&mut self, v: &mut NumericLiteral<u64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as usize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as usize)));
    }
  }

  fn numeric_literal_i8(&mut self, v: &mut NumericLiteral<i8>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as isize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as isize)));
    }
  }

  fn numeric_literal_i16(&mut self, v: &mut NumericLiteral<i16>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as isize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as isize)));
    }
  }

  fn numeric_literal_i32(&mut self, v: &mut NumericLiteral<i32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as isize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as isize)));
    }
  }

  fn numeric_literal_i64(&mut self, v: &mut NumericLiteral<i64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as isize))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as isize)));
    }
  }

  fn numeric_literal_f32(&mut self, v: &mut NumericLiteral<f32>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as f32))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as f32)));
    }
  }

  fn numeric_literal_f64(&mut self, v: &mut NumericLiteral<f64>, tracker: &mut Vec<String>, level: usize) {
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeObject>() {
      item.add(
        tracker.get(level).unwrap(),
        Box::new(JsonTypeNumeric::new(v.get() as f64))
      );
    } else 
    if let Some(item) = self.container.get_last_mut_generic::<JsonTypeArray>() {
      item.add(Box::new(JsonTypeNumeric::new(v.get() as f64)));
    }
  }
}

#[derive(Debug)]
struct JsonContainer {
  jev: Vec<Box<dyn JsonType>>
}

impl JsonContainer {
  fn new() -> Self {
    Self { jev: Vec::new() }
  } 
}

impl JsonContainer {
  fn get_last(&mut self) -> Option<&Box<dyn JsonType>> {
    self.jev.last()
  }

  fn get_last_mut(&mut self) -> Option<&mut Box<dyn JsonType>> {
    self.jev.last_mut()
  }

  fn get_last_mut_generic<T: 'static>(&mut self) -> Option<&mut T> {
    if let Some(je) = self.get_last_mut() {
      return je.to_mut().downcast_mut::<T>();
    }

    None
  }

  fn push_entity(&mut self, je: Box<dyn JsonType>) {
    self.jev.push(je);
  }

  fn pop_entity(&mut self) -> Option<Box<dyn JsonType>> {
    if self.jev.len() > 1 {
      return self.jev.pop();
    }

    None
  }
}

#[cfg(test)]
mod tests {

use crate::{parser::parser::JsonTextParser, types::types::{JsonTypeArray, JsonTypeObject}};

  #[test]
  fn test_5() {
    let json = String::from(r#"
      [
        {
          "x1": "999",
          "x2": 1.1,
          "x3": true,
        },
        1, 
        {
          "obj0": "LOL",
          "obj1": "LOL LOP",
        },
        true,
        null,
        {
          "item1": 999,
          "item2": false,
          "item3": true,
        },
        [
          "array 1",
          "array 2",
          "array 3",
          "array 4",
          "array 5",
        ],
      ]
    "#);

    let mut jp = JsonTextParser::new(json);
    jp.parse();

    let jo = jp.get_mut::<JsonTypeArray>();
    println!("JSON ARRAY: {:?}", jo)
  }

  #[test]
  fn test_4() {
    let json = String::from(r#"
      {
        "k1": true,
        "k2": null,
        "x1": "string value",
        "x2": -123.001,
      }
    "#);

    let mut jp = JsonTextParser::new(json);
    jp.parse();

    let jo = jp.get_mut::<JsonTypeObject>();
    println!("GET: K1: {:?}", jo.get("k1"));
    println!("GET: K2: {:?}", jo.get("k2"));
    println!("GET: X1: {:?}", jo.get("x1"));
    println!("GET: X2: {:?}", jo.get("x2"));

    let k1: bool = jo.get("k1").into();
    let k2: Option<String> = jo.get("k2").into();
    let x1: String = jo.get("x1").into();
    let x2: f32 = jo.get("x2").into();
    println!("VALUE: K1: {:?}", k1);
    println!("VALUE: K2: {:?}", k2);
    println!("VALUE: X1: {:?}", x1);
    println!("VALUE: X2: {:?}", x2);
  }

  #[test]
  fn test_3() {
    let json = String::from(r#"
      {
        "key1": true,
        "key2": {
          "x1": "string",
          "x2": false,
          "x3": {
            "f1": "Json Inside",
          }
        },
      }
    "#);

    let mut jp = JsonTextParser::new(json);
    jp.parse();

    let jo = jp.get_mut::<JsonTypeObject>();
    println!("JSON TYPE OBJECT: {:?}", jo);

    let key1: bool = jo.get("key1").into();
    let mut key2: JsonTypeObject  = jo.get("key2").into();
    let x1: String = key2.get("x1").into();
    let x2: bool = key2.get("x2").into();
    let mut x3: JsonTypeObject = key2.get("x3").into();
    let f1: String = x3.get("f1").into();
    println!("RESULT: {}", key1);
    println!("RESULT: {}", x1);
    println!("RESULT: {}", x2);
    println!("RESULT: {}", f1);
  }

  #[test]
  fn test_2() {
    let json = String::from(r#"
      {
        "k1": true,
        "k2": null,
        "x1": "string",
        "x2": 999,
      }
    "#);

    let mut jp = JsonTextParser::new(json);
    jp.parse();

    let jo = jp.get_mut::<JsonTypeObject>();
    println!("K1: {:?}", jo.get("k1"));
    println!("K2: {:?}", jo.get("k2"));
    println!("X1: {:?}", jo.get("x1"));
    println!("X2: {:?}", jo.get("x2"));

    let k1: bool = jo.get("k1").into();
    let k2: Option<String> = jo.get("k2").into();
    let x1: String = jo.get("x1").into();
    let x2: usize = jo.get("x2").into();
    println!("K1: {:?}", k1);
    println!("K2: {:?}", k2);
    println!("X1: {:?}", x1);
    println!("X2: {:?}", x2);
  }

  #[test]
  fn test_1() {
    let json = String::from(r#"
      {
        "key4": {
          "k1": true,
          "k2": {
            "k3": "v3",
            "k2": 123,
          },
          "k3": null,
        },
        "key3": null,
        "key2": {
          "x1": "string"
        },
        "key1": 999,
        "key0": [
          1, 
          {
            "obj0": "LOL"
          },
          true,
          null
        ]
      }
    "#);

    let mut jp = JsonTextParser::new(json);
    jp.parse();

    let jo = jp.get_mut::<JsonTypeObject>();
    println!("JSON OBJECT: {:?}", jo)
  }
}


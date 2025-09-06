use std::{any::{Any, TypeId}, collections::HashMap, fmt::Debug, rc::Rc};

// MACRO RULES 
macro_rules! general_type_impl {
  ($($identifier:ident),*) => {
    $(
      impl JsonType for $identifier {
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

macro_rules! generic_type_impl {
  ($identifier:tt,$($type:ty),*) => {
    $(
      impl JsonType for $identifier<$type> {
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

general_type_impl!(
  JsonTypeObject, JsonTypeArray, JsonTypeNull, JsonTypeBoolean, JsonTypeString
);

generic_type_impl!(
  JsonTypeNumeric, usize, isize, f32, f64
);
// MACRO RULES

// INTO TRAIT
macro_rules! numeric_into_impl {
  ($($type:ty),*) => {
    $(
      impl Into<$type> for &Box<dyn JsonType> {
        fn into(self) -> $type {
          self.to_ref().downcast_ref::<JsonTypeNumeric<$type>>().unwrap().value.to_owned()
        }
      }

      impl Into<$type> for &mut Box<dyn JsonType> {
        fn into(self) -> $type {
          self.to_mut().downcast_mut::<JsonTypeNumeric<$type>>().unwrap().value.to_owned()
        }
      }

      impl Into<Option<$type>> for &mut Box<dyn JsonType> {
        fn into(self) -> Option<$type> {
          if let Some(result) = self.to_ref().downcast_ref::<JsonTypeNumeric<$type>>() {
            return Some(result.value.to_owned());
          }

          None
        }
      }
    )*
  }
}

numeric_into_impl!(isize, usize, f32, f64);

impl Into<JsonTypeObject> for &Box<dyn JsonType> {
  fn into(self) -> JsonTypeObject {
    self.to_ref().downcast_ref::<JsonTypeObject>().unwrap().to_owned()
  }
}

impl Into<JsonTypeArray> for &mut Box<dyn JsonType> {
  fn into(self) -> JsonTypeArray {
    self.to_mut().downcast_mut::<JsonTypeArray>().unwrap().to_owned()
  }
}

impl Into<Option<isize>> for JsonTypeNull {
  fn into(self) -> Option<isize> {
    None
  }
}

impl Into<Option<String>> for JsonTypeNull {
  fn into(self) -> Option<String> {
    None
  }
}

impl Into<Option<bool>> for JsonTypeNull {
  fn into(self) -> Option<bool> {
    None
  }
}

impl Into<bool> for &Box<dyn JsonType> {
  fn into(self) -> bool {
    self.to_ref().downcast_ref::<JsonTypeBoolean>().unwrap().value.to_owned()
  }
}

impl Into<bool> for &mut Box<dyn JsonType> {
  fn into(self) -> bool {
    self.to_mut().downcast_mut::<JsonTypeBoolean>().unwrap().value.to_owned()
  }
}

impl Into<Option<bool>> for &mut Box<dyn JsonType> {
  fn into(self) -> Option<bool> {
    if let Some(result) = self.to_ref().downcast_ref::<JsonTypeBoolean>() {
      return Some(result.value.to_owned());
    }

    None
  }
}

impl Into<String> for &Box<dyn JsonType> {
  fn into(self) -> String {
    self.to_ref().downcast_ref::<JsonTypeString>().unwrap().value.to_owned()
  }
}

impl Into<String> for &mut Box<dyn JsonType> {
  fn into(self) -> String {
    self.to_mut().downcast_mut::<JsonTypeString>().unwrap().value.to_owned()
  }
}

impl Into<Option<String>> for &Box<dyn JsonType> {
  fn into(self) -> Option<String> {
    if let Some(result) = self.to_ref().downcast_ref::<JsonTypeString>() {
      return Some(result.value.to_owned());
    }

    None
  }
}

impl Into<Option<String>> for &mut Box<dyn JsonType> {
  fn into(self) -> Option<String> {
    if let Some(result) = self.to_mut().downcast_mut::<JsonTypeString>() {
      return Some(result.value.to_owned());
    }

    None
  }
}
// INTO TRAIT

pub trait JsonType {
  fn debug_str(&self) -> String;
  fn to_ref(&self) -> &dyn Any;
  fn to_mut(&mut self) -> &mut dyn Any;
}

impl std::fmt::Debug for dyn JsonType + 'static {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.debug_str())
  }
}

#[derive(Debug, Clone)]
pub struct JsonTypeObject {
  object: HashMap<String, Rc<Box<dyn JsonType>>>,
}

impl JsonTypeObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }
}

impl JsonTypeObject {
  pub fn add(&mut self, key: impl Into<String>, value: Box<dyn JsonType>) {
    self.object.insert(key.into(), Rc::new(value));
  }

  pub fn get(&mut self, key: impl Into<String>) -> &Box<dyn JsonType> {
    self.object.get(&key.into()).unwrap()
  }

  // TODO: ADD to_* FUNCTIONS
}

#[derive(Debug, Clone)]
pub struct JsonTypeArray {
  array: Vec<Rc<Box<dyn JsonType>>>,
}

impl JsonTypeArray {
  pub fn new() -> Self {
    Self { array: Vec::new() }
  }
}

impl JsonTypeArray {
  pub fn add(&mut self, value: Box<dyn JsonType>) {
    self.array.push(Rc::new(value));
  }
}

#[derive(Debug, Clone)]
pub struct JsonTypeNull;

impl JsonTypeNull {
  pub fn new() -> Self {
    Self { }
  }
}

#[derive(Debug, Clone)]
pub struct JsonTypeString {
  value: String,
}

impl JsonTypeString {
  pub fn new(value: impl Into<String>) -> Self {
    Self { value: value.into() }
  }
}

#[derive(Debug, Clone)]
pub struct JsonTypeBoolean {
  value: bool
}

impl JsonTypeBoolean {
  pub fn new(value: bool) -> Self {
    Self { value }
  }
}

#[derive(Debug, Clone)]
pub struct JsonTypeNumeric<T> {
  value: T
}

impl <T: Clone + Debug + 'static> JsonTypeNumeric<T> {
  pub fn new(value: T) -> Self {
    let supported_num_types = vec![
      TypeId::of::<usize>(),
      TypeId::of::<isize>(), 
      TypeId::of::<f32>(), 
      TypeId::of::<f64>(),
    ];

    if !supported_num_types.contains(&value.type_id()) {
      panic!("Json Parser Exception! Not a valid 'NUMERIC' type!")
    }

    Self { value }
  }
}

#[derive(Debug)]
pub enum JsonContainerType {
  Object, Array, None 
}

#[derive(Debug)]
pub struct JsonContainer {
  jct: JsonContainerType,
  jev: Vec<Box<dyn JsonType>>
}

impl JsonContainer {
  pub fn new() -> Self {
    Self { jct: JsonContainerType::None, jev: Vec::new() }
  } 
}

impl JsonContainer {
  pub fn update_jct(&mut self, jct: JsonContainerType) {
    self.jct = jct;
  }

  pub fn get_last(&mut self) -> Option<&Box<dyn JsonType>> {
    self.jev.last()
  }

  pub fn get_last_mut(&mut self) -> Option<&mut Box<dyn JsonType>> {
    self.jev.last_mut()
  }

  pub fn get_last_mut_generic<T: 'static>(&mut self) -> Option<&mut T> {
    if let Some(je) = self.get_last_mut() {
      return je.to_mut().downcast_mut::<T>();
    }

    None
  }

  pub fn push_entity(&mut self, je: Box<dyn JsonType>) {
    self.jev.push(je);
  }

  pub fn pop_entity(&mut self) -> Option<Box<dyn JsonType>> {
    if self.jev.len() > 1 {
      return self.jev.pop();
    }

    None
  }
}


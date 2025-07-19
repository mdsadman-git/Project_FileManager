use crate::builder::array::JsonArray;
use crate::builder::object::JsonObject;
use crate::builder::value::JsonValue;
use crate::builder::types::{JsonNull, JsonType};

// Macro - Type into JsonValue
macro_rules! ty_into_jv { 
  ($_type:ty, $dt:path) => {
    impl Into<JsonValue> for $_type {
      fn into(self) -> JsonValue {
        JsonValue { dt: $dt, value: format!("{}", self) }
      }
    }
  };
}

// VectorType
impl <T: Into<JsonValue>> Into<JsonValue> for Vec<T> {
  fn into(self) -> JsonValue {
    let mut v: Vec<String> = Vec::new();
    for e in self {
      let json_value: JsonValue = e.try_into().expect("Unknown type for Json!");
      match json_value.dt {
        JsonType::String => v.push(format!(r#""{}""#, json_value.value)),
        _ => v.push(format!("{}", json_value.value)),
      }
    }

    JsonValue { dt: JsonType::Array, value: format!("[{}]", v.join(",")) }
  }
}

// ObjectType
impl Into<JsonValue> for JsonObject {
  fn into(self) -> JsonValue {
    let mut s = String::new();
    for (i, (k, v)) in self.object.iter().enumerate() {
      match v.dt {
        JsonType::String => s.push_str(format!(r#""{}":"{}""#, k, v.value).as_str()),
        _ => s.push_str(format!(r#""{}":{}"#, k, v.value).as_str()),
      }
      if i < self.object.len() - 1 { s.push(',') };
    } 

    JsonValue { dt: JsonType::Object, value: format!("{{{}}}", s) }
  }
}

// ArrayType
impl Into<JsonValue> for JsonArray {
  fn into(self) -> JsonValue {
    let mut v = Vec::new();
    for e in self.array {
      let json_object: JsonValue = e.into();
      match json_object.dt {
        JsonType::String => v.push(format!(r#""{}""#, json_object.value)),
        _ => v.push(format!("{}", json_object.value))
      }
    }

    JsonValue { dt: JsonType::Array, value: format!("[{}]", v.join(",")) }
  }
}

// NullType
ty_into_jv!(JsonNull, JsonType::Null);

// BoolType Into JsonValue
ty_into_jv!(bool, JsonType::Boolean);

// StringType Into JsonValue 
ty_into_jv!(&str, JsonType::String);
ty_into_jv!(String, JsonType::String);

// NumericType Into JsonValue
ty_into_jv!(f64, JsonType::Number);
ty_into_jv!(f32, JsonType::Number);
ty_into_jv!(i128, JsonType::Number);
ty_into_jv!(i64, JsonType::Number);
ty_into_jv!(i32, JsonType::Number);
ty_into_jv!(i16, JsonType::Number);
ty_into_jv!(i8, JsonType::Number);
ty_into_jv!(u128, JsonType::Number);
ty_into_jv!(u64, JsonType::Number);
ty_into_jv!(u32, JsonType::Number);
ty_into_jv!(u16, JsonType::Number);
ty_into_jv!(u8, JsonType::Number);

// Macro - Ref JsonValue into Type
macro_rules! rjv_into_ty {
  ($_type:ty) => {
    impl Into<$_type> for &JsonValue {
      fn into(self) -> $_type {
        if self.dt != JsonType::Number { panic!("Json Type must be number! Found: {}", self.dt); }
        self.value.parse::<$_type>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
      }
    }

    impl Into<Option<$_type>> for &JsonValue {
      fn into(self) -> Option<$_type> {
        match self.dt {
          JsonType::Number =>  
            return Option::Some(self.value.parse::<$_type>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
          JsonType::Null => 
            return Option::None,
          _ => {}
        }
      
        panic!("Json Type must be number! Found: {}", self.dt); 
      }
    }
  };
}

// &JsonValue Into NumericType
impl Into<bool> for &JsonValue {
  fn into(self) -> bool {
    if self.dt != JsonType::Boolean { panic!("Json Type must be boolean! Found: {}", self.dt); }
    self.value.parse::<bool>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<bool>> for &JsonValue {
  fn into(self) -> Option<bool> {
    match self.dt { 
      JsonType::Boolean => 
        return Option::Some(self.value.parse::<bool>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be boolean! Found: {}", self.dt); 
  }
}

impl Into<String> for &JsonValue {
  fn into(self) -> String {
    if self.dt != JsonType::String { panic!("Json Type must be string! Found: {}", self.dt); }
    self.value.parse::<String>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<String>> for &JsonValue {
  fn into(self) -> Option<String> {
    match self.dt {
      JsonType::String => 
        return Option::Some(self.value.parse::<String>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be string! Found: {}", self.dt); 
  }
}

rjv_into_ty!(f64);
rjv_into_ty!(f32);
rjv_into_ty!(i128);
rjv_into_ty!(i64);
rjv_into_ty!(i32);
rjv_into_ty!(i16);
rjv_into_ty!(i8);
rjv_into_ty!(u128);
rjv_into_ty!(u64);
rjv_into_ty!(u32);
rjv_into_ty!(u16);
rjv_into_ty!(u8);
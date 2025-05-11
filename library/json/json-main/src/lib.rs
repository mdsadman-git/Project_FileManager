use std::{collections::HashMap, fmt::Display, ops::Deref, rc::Rc, sync::Arc};

use logger_main::Logger;

pub struct Json;

#[allow(dead_code)]
pub trait JsonBuilder {
  fn object() -> JsonObject;
  fn array() -> JsonArray;
  fn build(json_container: impl Into<JsonContainer>) -> String {
    (json_container.into() as JsonContainer).result
  }
}

impl JsonBuilder for Json {
  fn object() -> JsonObject {
    JsonObject::new()
  }

  fn array() -> JsonArray {
    JsonArray::new()
  }
}

#[derive(Debug)]
pub struct JsonValue {
  dt: JsonType,
  value: String,
}

impl Display for JsonValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("JsonValue {{ dt: {}, value: {} }}", self.dt, self.value).as_str())
  }
}

impl JsonValue {
  fn to_i32(&self) -> i32 {
    if self.dt != JsonType::Number {
      panic!("")
    }

    1
  }
}

#[derive(Debug, PartialEq, Eq)]
enum JsonType {
  Number, String, Boolean, Null, Object, Array
}

impl Display for JsonType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{: <16}", format!("JsonType.{:?}", self).as_str()).as_str())
  }
}

impl JsonType {
  fn is_primitive(self) -> bool {
    self == JsonType::Number || self == JsonType::String || self == JsonType::Boolean || self == JsonType::Null
  }
}

pub struct JsonNull;

impl JsonNull {
  pub fn new() -> Self { 
    Self {} 
  }
}

pub struct JsonContainer {
  result: String,
}

impl Into<JsonContainer> for JsonObject {
  fn into(self) -> JsonContainer {
    let mut result = Vec::new(); 
    for e in self.object.iter() {
      match e.1.dt {
        JsonType::String => result.push(format!(r#""{}":"{}""#, *e.0, e.1.value)),
        _                => result.push(format!(r#""{}":{}"#, *e.0, e.1.value)),
      }
    }

    JsonContainer { result: format!("{{{}}}", result.join(",")) }
  }
}

impl Into<JsonContainer> for JsonArray {
  fn into(self) -> JsonContainer {
    let json_array: JsonValue = self.into();
    JsonContainer { result: json_array.value }
  }
}

#[derive(Debug)]
pub struct JsonObject {
  object: HashMap<String, JsonValue>,
}

impl JsonObject {
  pub fn new() -> Self {
    Self { object: HashMap::new() }
  }

  pub fn insert(&mut self, k: impl Into<String>, v: impl Into<JsonValue>) -> &mut Self {
    let (json_key , json_value) = (k.into(), v.into());
    Logger::debug(format!("Append Value | {} => {}", json_key, json_value));
    self.object.insert(json_key, json_value);
    self
  }

  pub fn remove(&mut self, k: impl Into<String>) -> &mut Self {
    let json_key = k.into();
    let json_value = self.object.remove(&json_key);
    Logger::debug(format!("Delete Value | {} => {}", json_key, json_value.unwrap()));
    self
  }

  pub fn get(&mut self, k: impl Into<String>) -> Option<&JsonValue> {
    self.object.get(&k.into())
  }
}

pub struct JsonArray {
  array: Vec<JsonValue>,
}

impl JsonArray {
  pub fn new() -> Self {
    Self { array: Vec::new() }
  }

  pub fn append(&mut self, o: impl Into<JsonValue>) -> &mut Self {
    self.array.push(o.into());
    self
  }

  pub fn remove(&mut self, index: usize) -> &mut Self {
    let _ = self.array.remove(index);
    self
  }
}

pub trait JsonBlock {}
impl JsonBlock for JsonObject {}
impl JsonBlock for JsonArray {}

// VECTOR 
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

// ARRAY
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

// OBJECT 
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

// NULL
impl Into<JsonValue> for JsonNull {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Null, value: format!("null") }
  }
}

// BOOLEAN 
impl Into<JsonValue> for bool {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Boolean, value: format!("{}", self) }
  }
}

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

// STRING 
impl Into<JsonValue> for &str {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for String {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::String, value: format!("{}", self) }
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

// NUMBER - F
impl Into<JsonValue> for f64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for f32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

// NUMBER - I
impl Into<JsonValue> for i128 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<i32> for &JsonValue {
  fn into(self) -> i32 {
    if self.dt != JsonType::Number { panic!("Json Type must be number! Found: {}", self.dt); }
    self.value.parse::<i32>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())
  }
}

impl Into<Option<i32>> for &JsonValue {
  fn into(self) -> Option<i32> {
    match self.dt {
      JsonType::Number =>  
        return Option::Some(self.value.parse::<i32>().expect(format!("Failed to parse value. Value: {}", self.value).as_str())),
      JsonType::Null => 
        return Option::None,
      _ => {}
    }

    panic!("Json Type must be number! Found: {}", self.dt); 
  }
}

impl Into<JsonValue> for i16 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for i8 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

// NUMBER - U
impl Into<JsonValue> for u128 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u64 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u32 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u16 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

impl Into<JsonValue> for u8 {
  fn into(self) -> JsonValue {
    JsonValue { dt: JsonType::Number, value: format!("{}", self) }
  }
}

// WORKING...
#[allow(dead_code)]
pub trait JsonParser {
  fn perser() -> impl JsonParser;
  fn parse(&self, json: String);
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
  ObjectStart, 
  ObjectEnd, 
  ArrayStart, 
  ArrayEnd, 
  ContentSeparator, 
  ElementSeparator, 
  JsonKey, 
  JsonValue,
}

impl Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{:<20}", format!("{:?}", self)))
  }
}

#[derive(Debug)]
pub struct Token {
  pub tt: TokenType,
  pub value: String,
  pub quoted: Option<bool>,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Token {{ {:?} {:?} {} }}", self.tt, self.quoted, self.value))
  }
}

trait QuotedToken {
  fn new(tt: TokenType, value: impl Into<String>, quoted: Option<bool>) -> Self;
}

trait BasicToken {
  fn new(tt: TokenType, value: impl Into<String>) -> Self;
}

impl QuotedToken for Token {
  fn new(tt: TokenType, value: impl Into<String>, quoted: Option<bool>) -> Self {
    Self { tt, value: value.into(), quoted }
  }
}

impl BasicToken for Token {
  fn new(tt: TokenType, value: impl Into<String>) -> Self {
    Self { tt, value: value.into(), quoted: None }
  }
}

pub struct Lexer<'a> {
  iterator: Box<dyn Iterator<Item = char> + 'a>,
  tokens: Vec<Token>,
  is_quoted: bool,

  object_track: Vec<bool>,
  key_track: Vec<bool>,
}

impl<'a> Lexer<'a> {
  pub fn new(json: &'a str) -> Self {
    Self {
      iterator: Box::new(json.chars()), 
      tokens: Vec::new(),
      is_quoted: false, 

      object_track: Vec::new(),
      key_track: Vec::new(),
    }

  }
}

impl<'a> Lexer<'a> {
  pub fn tokenize(&mut self) -> &Vec<Token> {
    loop { if !self.next() { break; } }
    &self.tokens
  }

  fn next(&mut self) -> bool {
    if let Some(mut c) = self.iterator.next() {
      'label: loop {
        if c == ' ' || c == '\n' || c == '\t' {
          break 'label; 
        }

        if self.is_quoted || (c != '"' && self.key_track.len() > 0 && !*self.key_track.last().unwrap()) {
          self.add_content(&mut c);
        }

        if c == '"' {
          self.is_quoted = !self.is_quoted;
        } else if c == '{' && !self.is_quoted {
          self.object_track.push(true);
          self.key_track.push(true);
          self.tokens.push(BasicToken::new(TokenType::ObjectStart, c));
        } else if c == '}' && !self.is_quoted {
          self.object_track.pop();
          self.key_track.pop();
          self.tokens.push(BasicToken::new(TokenType::ObjectEnd, c));
        } else if c == '[' && !self.is_quoted {
          self.object_track.push(false);
          self.key_track.push(false);
          self.tokens.push(BasicToken::new(TokenType::ArrayStart, c));
        } else if c == ']' && !self.is_quoted {
          self.object_track.pop();
          self.key_track.pop();
          self.tokens.push(BasicToken::new(TokenType::ArrayEnd, c));
        } else if c == ':' && !self.is_quoted {
          self.key_track.pop();
          self.key_track.push(false);
          self.tokens.push(BasicToken::new(TokenType::ContentSeparator, c));
        } else if c == ',' && !self.is_quoted {
          self.key_track.pop();
          self.key_track.push(*self.object_track.last().unwrap());
          self.tokens.push(BasicToken::new(TokenType::ElementSeparator, c));
        }

        break 'label;
      }

      return true;
    } 

    false
  }

  fn add_content(&mut self, c: &mut char) {
    let checkers = [',','}','{',']','[',' ','\n','\t'];
    if checkers.contains(c) { return; }

    let mut content = String::from(*c);
    let mut is_escape = false;
    let is_quoted = self.is_quoted;
    while let Some(q) = self.iterator.next() {
        if q == '"' && !is_escape {
          self.is_quoted = !self.is_quoted;
          break;
        } else if !self.is_quoted && !is_escape && checkers.contains(&q) {
          *c = q;
          break;
        } else if q == '\\' && !is_escape {
          is_escape = true;
        } else {
          is_escape = false;
        }

        content.push(q);
    }

    if *self.key_track.last().unwrap() {
      self.tokens.push(BasicToken::new(TokenType::JsonKey, content));
    } else {
      self.tokens.push(QuotedToken::new(TokenType::JsonValue, content, Some(is_quoted)));
    }
  }
}

pub struct ObjectParser<'a> {
  tokens: &'a Vec<Token>,
  objects: Vec<JsonObject>,
}

impl <'a> ObjectParser<'a> {
  pub fn new(tokens: &'a Vec<Token>) -> Self {
    Self { tokens: tokens, objects: vec![JsonObject::new()] }
  }
}

impl <'a> ObjectParser<'a> {
  pub fn parse(&mut self) {
    let mut iter = self.tokens.iter().enumerate();
    let mut key: &String = &String::new();

    while let Some((i, token)) = iter.next() {
      if i == 0 {
        if token.tt == TokenType::ObjectStart {
          continue;
        } else {
          panic!("Invalid token given! Token: {}", token)
        }
      } 

      if token.tt == TokenType::JsonKey {
        key = &token.value;
      } else if token.tt == TokenType::JsonValue {
        match self.objects.last_mut() {
            Some(json_object) => {
                let k = key.clone();
                let v = token.value.clone();
                json_object.insert(k, v);
            }
            _ => panic!("Unable to get last object!"),  
        } 
      }
    }
  }

  pub fn print(&mut self) {
    for e in &self.objects {
      println!("Object: {:?}", e);
    }
  }
}
// WORKING...

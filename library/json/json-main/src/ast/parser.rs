use core::str;
use std::{any::{Any, TypeId}, fmt::Debug, ops::AddAssign};

use super::lexer::{Token, TokenType, ValueOf};

// LOCAL CUSTOM TYPES
trait FnVec {
  fn free_last(&mut self);
}

impl FnVec for Vec<Option<Box<dyn Expression>>> {
  fn free_last(&mut self) {
    let last = self.last_mut().unwrap();
    *last = None
  }
}
// LOCAL CUSTOM TYPES

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeType {
  ObjectExpression, 
  ArrayExpression,
  ObjectLiteral,
  ArrayLiteral,
}

trait Expression {
  fn debug_str(&self) -> String;
  fn to_ref(&self) -> &dyn Any;
  fn to_mut(&mut self) -> &mut dyn Any;
  fn ex_clone(&self) -> Box<dyn Expression>;
}

trait Literal : Expression {
  fn lit_clone(&self) -> Box<dyn Literal>;
}

impl Debug for dyn Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.debug_str())
  }
}

impl Debug for dyn Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.debug_str())
  }
}

#[derive(Debug, Clone)]
struct ObjectExpression {
  lit_objects: Vec<ObjectLiteral>,
}

impl Expression for ObjectExpression {
  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
      Box::new(self.clone())
  }
}

impl Literal for ObjectExpression {
    fn lit_clone(&self) -> Box<dyn Literal> {
      Box::new(self.clone())
    }
}

impl ObjectExpression {
  pub fn new() -> Self {
    Self { lit_objects: Vec::new() }
  }
}

impl ObjectExpression {
  pub fn add(&mut self, lit_object: ObjectLiteral) {
    self.lit_objects.push(lit_object);
  }
}

#[derive(Debug, Clone)]
struct ArrayExpression {
  lit_elements: Vec<ArrayLiteral>,
}

impl Expression for ArrayExpression {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl Literal for ArrayExpression {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl ArrayExpression {
  fn new() -> Self {
    Self { lit_elements: Vec::new() }
  }
}

impl ArrayExpression {
  pub fn add(&mut self, lit_element: ArrayLiteral) {
    self.lit_elements.push(lit_element);
  }
}

#[derive(Debug, Clone)]
struct StringLiteral {
  value: String,
}

impl Expression for StringLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl Literal for StringLiteral {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl StringLiteral {
  pub fn new(value: String) -> Self {
    Self { value }
  }
}

#[derive(Debug, Clone)]
struct NumericLiteral<T: Clone> {
  value: T,
}

impl <T: Clone + Debug + 'static> Expression for NumericLiteral<T> {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl <T: Clone + Debug + 'static> Literal for NumericLiteral<T> {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl <T: Clone + 'static> NumericLiteral<T> {
  pub fn new(value: T) -> Self {
    let supported_num_types = vec![
      TypeId::of::<u8>(),
      TypeId::of::<u16>(), 
      TypeId::of::<u32>(), 
      TypeId::of::<u64>(), 
      TypeId::of::<i8>(), 
      TypeId::of::<i16>(), 
      TypeId::of::<i32>(), 
      TypeId::of::<i64>(), 
      TypeId::of::<f32>(), 
      TypeId::of::<f64>(),
    ];

    if !supported_num_types.contains(&value.type_id()) {
      panic!("Not a valid 'NUMERIC' type!")
    }

    Self { value }
  }
}

#[derive(Debug, Clone)]
struct BooleanLiteral {
  value: bool,
}

impl Expression for BooleanLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl Literal for BooleanLiteral {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl BooleanLiteral {
  pub fn new(value: bool) -> Self {
    Self { value }
  }
}

#[derive(Debug, Clone)]
struct NullLiteral {}

impl Expression for NullLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone()) 
  }
}

impl Literal for NullLiteral {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl NullLiteral {
  pub fn new() -> Self {
    Self { }
  }
}

#[derive(Debug, Clone)]
struct ObjectKey {
  lit_string: StringLiteral,
}

impl Expression for ObjectKey {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl ObjectKey {
  pub fn new(lit_string: StringLiteral) -> Self {
    Self { lit_string }
  }
}

#[derive(Debug)]
struct ValueLiteral {
  literal: Box<dyn Literal>
}

impl Clone for ValueLiteral {
  fn clone(&self) -> Self {
    Self { literal: self.literal.lit_clone() }
  }
}

impl Expression for ValueLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl ValueLiteral {
  pub fn new(literal: Box<dyn Literal>) -> Self {
    Self { literal }
  }
}

trait ValueLiteralInjector {
  fn inject(&mut self, value: ValueLiteral);
}

#[derive(Debug, Clone)]
struct ObjectLiteral {
  key: Option<ObjectKey>,
  value: Option<ValueLiteral>,
}

impl Expression for ObjectLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl Literal for ObjectLiteral {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl ObjectLiteral {
  pub fn new() -> Self {
    Self { key: Option::None, value: Option::None }
  }
}

impl ValueLiteralInjector for ObjectLiteral {
  fn inject(&mut self, value: ValueLiteral) {
    self.value = Some(value);
  }
}

impl ObjectLiteral {
  pub fn key(&mut self, key: ObjectKey) {
    self.key = Some(key);
  }

  pub fn value(&mut self, value: ValueLiteral) {
    self.value = Some(value);
  }
}

#[derive(Debug, Clone)]
struct ArrayLiteral {
  value: Option<ValueLiteral>,
}

impl Expression for ArrayLiteral {

  fn debug_str(&self) -> String {
    format!("{:?}", self) 
  }

  fn to_ref(&self) -> &dyn Any {
    self
  }

  fn to_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn ex_clone(&self) -> Box<dyn Expression> {
    Box::new(self.clone())
  }
}

impl Literal for ArrayLiteral {
  fn lit_clone(&self) -> Box<dyn Literal> {
    Box::new(self.clone())
  }
}

impl ValueLiteralInjector for ArrayLiteral {
  fn inject(&mut self, value: ValueLiteral) {
    self.value = Some(value);
  }
}

impl ArrayLiteral {
  pub fn init(value: ValueLiteral) -> Self {
    Self { value: Some(value) }
  }

  pub fn new() -> Self {
    Self { value: None }
  }
}

#[derive(Debug)]
struct Node {
  node_type: NodeType,
  root: Box<dyn Expression>,
}

impl Node {
  fn new(node_type: NodeType, root: Box<dyn Expression>) -> Self {
    Self { node_type, root }
  }
}

impl Node {
  fn is_object_expression(&self) -> bool {
    self.node_type == NodeType::ObjectExpression
  }

  fn is_array_expression(&self) -> bool {
    self.node_type == NodeType::ArrayExpression
  }
}

pub struct JsonParser<'a> {
  tokens: &'a Vec<Token>,
  index: usize,
  nstack: Vec<Node>,
  lstack: Vec<Option<Box<dyn Expression>>>,
}

impl <'a> JsonParser<'a> {
  pub fn new(tokens: &'a Vec<Token>) -> Self {
    Self { tokens, index: 1, lstack: vec![None], nstack: match (tokens.get(0), tokens.get(tokens.len() - 1)) {
      (Some(start), Some(end)) if start.tt == TokenType::ObjectStart && end.tt == TokenType::ObjectEnd => {
        vec![Node::new(NodeType::ObjectExpression, Box::new(ObjectExpression::new()))]
      }
      (Some(start), Some(end)) if start.tt == TokenType::ArrayStart && end.tt == TokenType::ArrayEnd => {
        vec![Node::new(NodeType::ArrayExpression, Box::new(ArrayExpression::new()))]
      }
      _ => panic!("Parser Exception: Invalid Json! Json is not an object or an array type!")
    }}
  }

  pub fn parse(&mut self) {
    if self.is_last() || self.nstack.is_empty() { return; }

    let current = self.tokens.get(self.index);
    if let None = current { return; }

    match current.unwrap() {
      token if token.tt == TokenType::JsonKey => {
        if self.lstack.len() > 0 {
          if let Some(_) = self.lstack.last().unwrap() {
            panic!("Parser Exception: Last expression should be none!");
          }
        }

        if let Some(n) = self.nstack.last() {
          if n.node_type != NodeType::ObjectExpression {
            panic!("Parser Exception: Invalid type for token 'JsonKey'! '{:?}'", n.type_id());
          }
        }

        if self.lstack.len() > 0 {
          self.lstack.pop();
        }

        self.lstack.push(Some(Box::new(ObjectLiteral::new())));
        let le = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut().downcast_mut::<ObjectLiteral>().unwrap();
        let literal = StringLiteral::new(token.value());
        let object_key = ObjectKey::new(literal);
        le.key(object_key);
      }
      token if token.tt == TokenType::ContentSeparator => {

      }
      token if token.tt == TokenType::JsonValue => {
        match token.vof {
          ValueOf::Object => if let None = self.lstack.last() {
            panic!("Parser Exception: Last expression should not be none!");
          }, 
          ValueOf::Array => {
            self.lstack.pop();
            self.lstack.push(Some(Box::new(ArrayLiteral::new())));
          },
          _ => panic!("Parser Exception: Unknown value type is given!")
        }

        if let Some(n) = self.nstack.last() {
          if !n.is_object_expression() && !n.is_array_expression() {
            panic!("Parser Exception: Invalid type for token 'JsonValue'!");
          }
        }

        if let None = token.quoted {
          panic!("Parser Exception: Token quoted type is undefined for 'JsonValue'")
        }

        let ls = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut();
        let lx: Box<&mut dyn ValueLiteralInjector> = match ls.downcast_mut::<ObjectLiteral>() {
            Some(v) => Box::new(v),
            None => match ls.downcast_mut::<ArrayLiteral>() {
                Some(v) => Box::new(v),
                None => panic!("Parser Exception: Invalid literal type given! Only Array or Object is valid literal type."),
            },
        };

        match token.quoted.unwrap() {
          true => {
            let lit_value = ValueLiteral::new(Box::new(StringLiteral::new(token.value())));
            lx.inject(lit_value);
          }
          false => {
            match &token.value {
              v if v == "true" || v == "false" => {
                let lit_value = ValueLiteral::new(Box::new(BooleanLiteral::new(v.parse::<bool>().unwrap())));
                lx.inject(lit_value);
              }
              v if v == "null" => {
                let lit_value = ValueLiteral::new(Box::new(NullLiteral::new()));
                lx.inject(lit_value);
              }
              _  => {
                let tv: Vec<&str> = token.value.matches(char::is_numeric).collect();
                let lit_value: ValueLiteral; 
                if tv.len() != token.value.len() {
                  panic!("Parsing Exception: Unknown 'JsonValue' found, {}", token.value);
                }

                if let Ok(p) = token.value.parse::<u8>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<u16>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<u32>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<u64>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<i8>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<i16>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<i32>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else
                if let Ok(p) = token.value.parse::<i64>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else 
                if let Ok(p) = token.value.parse::<f32>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else 
                if let Ok(p) = token.value.parse::<f64>() {
                  lit_value = ValueLiteral::new(Box::new(NumericLiteral::new(p)));
                } else {
                  panic!("Parsing Exception: Unknown numeric value provided, {}", token.value);
                }

                lx.inject(lit_value);
              }
            }
          }
        }

        let node = self.nstack.last_mut().unwrap();
        match node.node_type {
            NodeType::ObjectExpression => {
              let node_root = node.root.to_mut().downcast_mut::<ObjectExpression>().unwrap();
              let ls = ls.downcast_mut::<ObjectLiteral>().unwrap();
              node_root.add(ls.clone());
            },
            NodeType::ArrayExpression => {
              let node_root = node.root.to_mut().downcast_mut::<ArrayExpression>().unwrap();
              let ls = ls.downcast_mut::<ArrayLiteral>().unwrap();
              node_root.add(ls.clone());
            },
            _ => {}
        }

        self.lstack.free_last();
      }
      token if token.tt == TokenType::ElementSeparator => {

      }
      token if token.tt == TokenType::ObjectStart => {
        if let Some(n) = self.nstack.last() {
          if n.is_object_expression() && n.is_array_expression() {
            panic!("Parser Exception: Invalid type for token 'JsonValue'! It must be an Object or Array expression");
          }
        }

        let nn = Node::new(NodeType::ObjectExpression, Box::new(ObjectExpression::new()));
        self.nstack.push(nn);
        self.lstack.push(None);
      }
      token if token.tt == TokenType::ObjectEnd => {
        if let Some(ln) = self.nstack.pop() {
          self.lstack.pop();

          let vl = ln.root.to_ref().downcast_ref::<ObjectExpression>().unwrap().clone();
          let node = self.nstack.last_mut().unwrap();
          match node.node_type {
            NodeType::ObjectExpression => {
              let le = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut().downcast_mut::<ObjectLiteral>().unwrap();
              le.value(ValueLiteral::new(Box::new(vl)));

              let nr = node.root.to_mut().downcast_mut::<ObjectExpression>().unwrap();
              nr.add(le.clone());
            },
            NodeType::ArrayExpression => {
              let nr = node.root.to_mut().downcast_mut::<ArrayExpression>().unwrap();
              nr.add(ArrayLiteral::init(ValueLiteral::new(Box::new(vl))));
            },
            _ => {} // TODO: ADD PANIC IF NECESSARY
          }

          self.lstack.free_last();
        }
      }
      token if token.tt == TokenType::ArrayStart => {
        if let Some(n) = self.nstack.last() {
          if n.is_object_expression() && n.is_array_expression() {
            panic!("Parser Exception: Invalid type for token 'JsonValue'! It must be an Object or Array expression");
          }
        }

        let nn = Node::new(NodeType::ArrayExpression, Box::new(ArrayExpression::new()));
        self.nstack.push(nn);
        self.lstack.push(None);
      }
      token if token.tt == TokenType::ArrayEnd => {
        if let Some(ln) = self.nstack.pop() {
          self.lstack.pop();

          let vl = ln.root.to_ref().downcast_ref::<ArrayExpression>().unwrap().clone();
          let node = self.nstack.last_mut().unwrap();
          match node.node_type {
            NodeType::ObjectExpression => {
              let le = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut().downcast_mut::<ObjectLiteral>().unwrap();
              le.value(ValueLiteral::new(Box::new(vl)));

              let nr = node.root.to_mut().downcast_mut::<ObjectExpression>().unwrap();
              nr.add(le.clone());
            },
            NodeType::ArrayExpression => {
              let nr = node.root.to_mut().downcast_mut::<ArrayExpression>().unwrap();
              nr.add(ArrayLiteral::init(ValueLiteral::new(Box::new(vl))));
            },
            _ => {} // TODO: ADD PANIC IF NECESSARY
          }

          self.lstack.free_last();
        }
      }
      _ => {} 
    }
    
    self.advance(); 
    self.parse();
  }
}

impl <'a> JsonParser<'a> {
  fn advance(&mut self) {
    self.index.add_assign(1);
  }

  fn is_last(&self) -> bool {
    self.index == self.tokens.len() - 1
  }
}

impl <'a> JsonParser<'a> {
  fn print(&self) {
    println!("NODE: {:?}", self.nstack.last())
  }
}

// TESTS
#[cfg(test)]
mod tests {
  use crate::ast::lexer::Lexer;
  use logger_main::Logger;
  use super::JsonParser;

  #[test]
  fn jp_array_test() {
    let json = String::from(r#"
      [
        {
          "key4": null,
          "key3": true,
          "key1": "string0",
          "key2": 123
        },
        {
          "key41": "1",
          "key31": 2,
          "key11": true,
          "key21": null
        }, 
        {
          "array1": ["1", 2, null, true],
          "array2": [["55", 55, null, false]],
        }
      ]
    "#);

    let mut lexer = Lexer::new(json.as_str());
    let tokens = lexer.tokenize();

    Logger::info("Tokenized List");
    for e in tokens {
      println!("Token - {} {:<15} '{}'", e.tt, format!("{:?}", e.quoted), e.value);
    }

    let mut json_parser = JsonParser::new(tokens);
    json_parser.parse();
    json_parser.print();
  }

  #[test]
  fn jp_test() {
    let json = String::from(r#"
      {
        "key4": null,
        "key3": true,
        "key1": "string0",
        "key2": 123
      }
    "#);

    let mut lexer = Lexer::new(json.as_str());
    let tokens = lexer.tokenize();

    Logger::info("Tokenized List");
    for e in tokens {
      println!("Token - {} {:<15} '{}'", e.tt, format!("{:?}", e.quoted), e.value);
    }

    let mut json_parser = JsonParser::new(tokens);
    json_parser.parse();
    json_parser.print();
  }

  #[test]
  fn json_object_parser_test() {
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
      }
    "#);

    let mut lexer = Lexer::new(json.as_str());
    let tokens = lexer.tokenize();
    Logger::info("Tokenized List");
    for e in tokens {
      println!("Token - {} {:<15} '{}'", e.tt, format!("{:?}", e.quoted), e.value);
    }

    let mut json_parser = JsonParser::new(tokens);
    json_parser.parse();
    json_parser.print();
  }

  #[test]
  fn json_parser_test() {
    let json = String::from(r#"
      {
        "array_1": [
          {
            "key1": "string2",
            "key2": 456,
            "key3": false,
            "key4": null
          },
          {
            "key3": false,
            "key4": null,
            "key1": "string2",
            "key2": 456
          }
        ],
        "key4": null,
        "array_2": [
          [
            1,
            2,
            3
          ],
          [
            "1",
            "2",
            "3"
          ],
          [
            3.1415,
            100,
            "Hello",
            false,
            null
          ]
        ],
        "key3": true,
        "object_1": {
          "key2": 456,
          "key4": null,
          "key1": "string1",
          "key3": false
        },
        "key1": "string0",
        "key2": 123
      }
    "#);
    let mut lexer = Lexer::new(json.as_str());
    let v = lexer.tokenize();
    Logger::info("Tokenized List");
    for e in v {
      println!("Token - {} {:<15} '{}'", e.tt, format!("{:?}", e.quoted), e.value);
    }
  }
}
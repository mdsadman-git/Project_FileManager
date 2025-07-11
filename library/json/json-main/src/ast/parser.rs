use core::str;
use std::{any::{Any, TypeId}, fmt::Debug, ops::AddAssign};

use super::lexer::{Token, TokenType, ValueOf};

// LOCAL CUSTOM TYPES
trait FnVec {
  fn free_last(&mut self);
  fn switch_last(&mut self, v: Option<Box<dyn Expression>>);
  fn dc_mut<T: Any>(&mut self) -> &mut T;
}

impl FnVec for Vec<Option<Box<dyn Expression>>> {
  fn free_last(&mut self) {
    let last = self.last_mut().unwrap();
    *last = None
  }

  fn switch_last(&mut self, v: Option<Box<dyn Expression>>) {
    if self.len() > 0 { 
      self.pop();
    }

    self.push(v);
  }
  
  fn dc_mut<T: Any>(&mut self) -> &mut T {
    self.last_mut().unwrap().as_mut().unwrap().to_mut().downcast_mut::<T>().unwrap()
  }
}
// LOCAL CUSTOM TYPES

// LOCAL MACROS
macro_rules! expression_impl {
  ($identifier:ident) => {
    impl Expression for $identifier {
      fn debug_str(&self) -> String {
        format!("{:?}", self) 
      }

      fn ident_name(&self) -> String { 
        format!("{}", stringify!($identifier))
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
  };
}

macro_rules! literal_impl {
  ($identifier:ident) => {
    impl Literal for $identifier {
      fn lit_clone(&self) -> Box<dyn Literal> {
        Box::new(self.clone())
      }
    }
  };
}

macro_rules! expression_gn_impl {
  ($identifier:ident) => {
    impl <T: Clone + Debug + 'static> Expression for $identifier<T> {
      fn debug_str(&self) -> String {
        format!("{:?}", self) 
      }

      fn ident_name(&self) -> String { 
        format!("{}", stringify!($identifier))
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
  };
}

macro_rules! literal_gn_impl {
  ($identifier:ident) => {
    impl <T: Clone + Debug + 'static> Literal for $identifier<T> {
      fn lit_clone(&self) -> Box<dyn Literal> {
        Box::new(self.clone())
      }
    }
  };
}
// LOCAL MACROS

// MACRO DECLARATION
expression_impl!(ObjectExpression);
literal_impl!(ObjectExpression);
expression_impl!(ArrayExpression);
literal_impl!(ArrayExpression);
expression_impl!(StringLiteral);

literal_impl!(StringLiteral);
expression_impl!(NullLiteral);
literal_impl!(NullLiteral);
expression_impl!(BooleanLiteral);
literal_impl!(BooleanLiteral);
expression_gn_impl!(NumericLiteral);
literal_gn_impl!(NumericLiteral);
expression_impl!(ObjectLiteral);
expression_impl!(ArrayLiteral);

expression_impl!(KeyLiteral);
expression_impl!(ValueLiteral);
// MACRO DECLARATION

trait Expression {
  fn debug_str(&self) -> String;
  fn ident_name(&self) -> String;
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

impl StringLiteral {
  pub fn new(value: String) -> Self {
    Self { value }
  }
}

#[derive(Debug, Clone)]
struct NumericLiteral<T: Clone + Debug + 'static> {
  value: T,
}

impl <T: Clone + Debug + 'static> NumericLiteral<T> {
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

impl BooleanLiteral {
  pub fn new(value: bool) -> Self {
    Self { value }
  }

  fn is_bool(v: impl Into<String>) -> bool {
    let v: String = v.into();
    v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("false")
  }
}

#[derive(Debug, Clone)]
struct NullLiteral;

impl NullLiteral {
  pub fn new() -> Self {
    Self {}
  }

  fn is_null(v: impl Into<String>) -> bool {
    let v: String = v.into();
    v.eq_ignore_ascii_case("null")
  }
}

#[derive(Debug, Clone)]
struct KeyLiteral {
  lit_string: StringLiteral,
}

impl KeyLiteral {
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
  key: Option<KeyLiteral>,
  value: Option<ValueLiteral>,
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
  pub fn key(&mut self, key: KeyLiteral) {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ExpressionType {
  Object, Array,
}

#[derive(Debug)]
struct Node {
  et: ExpressionType,
  root: Box<dyn Expression>,
}

impl Node {
  fn new(node_type: ExpressionType, root: Box<dyn Expression>) -> Self {
    Self { et: node_type, root }
  }
}

impl Node {
  fn is_object_expression(&self) -> bool {
    self.et == ExpressionType::Object
  }

  fn is_array_expression(&self) -> bool {
    self.et == ExpressionType::Array
  }
}

pub struct JsonParser<'a> {
  tokens: &'a Vec<Token>,
  token: Option<&'a Token>,
  index: usize,
  nstack: Vec<Node>,
  lstack: Vec<Option<Box<dyn Expression>>>,
}

impl <'a> JsonParser<'a> {
  fn advance(&mut self) {
    self.index.add_assign(1);
  }

  fn next_token(&mut self) -> bool {
    self.token = self.tokens.get(self.index);
    self.token.is_none()
  }

  fn is_last(&self) -> bool {
    self.index == self.tokens.len() - 1
  }

  fn validate(&mut self, ct: &Token) {
    match ct.tt {
      TokenType::ObjectEnd | TokenType::ArrayEnd => {
        if self.nstack.last().is_none() {
          panic!("Parser Exception: Last element of node stack must not none!");
        }
      },
      TokenType::JsonKey => {
        if self.lstack.last().is_none() {
          panic!("Parser Exception: Last expression should be none!");
        }

        if self.nstack.last().is_none() {
          panic!("Parser Exception: Node not found for further execution!");
        }

        if !self.nstack.last().unwrap().is_object_expression() {
          panic!("Parser Exception: Invalid node type for token 'JsonKey'!");
        }
      },
      TokenType::JsonValue => {
        match ct.vof {
          ValueOf::Array => {},
          ValueOf::Object => if let None = self.lstack.last() {
            panic!("Parser Exception: Last expression should not be none!");
          }, 
          _ => panic!("Parser Exception: Unknown value type is given!")
        }

        if self.nstack.last().is_none() {
            panic!("Parser Exception: No node found to proceed!");
        }

        if self.nstack.last().is_some_and(|n| !n.is_array_expression() && !n.is_object_expression()) {
          panic!("Parser Exception: Invalid type for token 'JsonValue'!");
        }

        if let None = ct.quoted {
          panic!("Parser Exception: Token quoted type is undefined for 'JsonValue'")
        }
      },
      _ => {}
    };
  }
}

impl <'a> JsonParser<'a> {
  pub fn new(tokens: &'a Vec<Token>) -> Self {
    Self { tokens, index: 1, token: None, lstack: vec![None], nstack: match (tokens.get(0), tokens.get(tokens.len() - 1)) {
      (Some(start), Some(end)) if start.tt == TokenType::ObjectStart && end.tt == TokenType::ObjectEnd => {
        vec![Node::new(ExpressionType::Object, Box::new(ObjectExpression::new()))]
      }
      (Some(start), Some(end)) if start.tt == TokenType::ArrayStart && end.tt == TokenType::ArrayEnd => {
        vec![Node::new(ExpressionType::Array, Box::new(ArrayExpression::new()))]
      }
      _ => panic!("Parser Exception: Invalid Json! Json is not an object or an array type!")
    }}
  }

  pub fn parse(&mut self) {
    if self.is_last() || self.nstack.is_empty() || self.next_token() { return; }

    match self.token.unwrap() {
      token if token.tt == TokenType::JsonKey => {
        self.validate(&token);
        self.lstack.switch_last(Some(Box::new(ObjectLiteral::new())));
        self.lstack.dc_mut::<ObjectLiteral>().key(KeyLiteral::new(StringLiteral::new(token.value())));
      }
      token if token.tt == TokenType::JsonValue => {
        self.validate(&token);

        if token.vof == ValueOf::Array {
          self.lstack.pop();
          self.lstack.push(Some(Box::new(ArrayLiteral::new())));
        }

        let ls = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut();
        let lx: Box<&mut dyn ValueLiteralInjector> = match ls.downcast_mut::<ObjectLiteral>() {
            Some(v) => Box::new(v),
            None => match ls.downcast_mut::<ArrayLiteral>() {
                Some(v) => Box::new(v),
                None => panic!("Parser Exception: Invalid literal type found! Only Array or Object is valid literal type."),
            },
        };

        match token.quoted.unwrap() {
          true => lx.inject(ValueLiteral::new(Box::new(StringLiteral::new(token.value())))),
          false => {
            match &token.value {
              v if BooleanLiteral::is_bool(v) => lx.inject(ValueLiteral::new(Box::new(BooleanLiteral::new(v.parse::<bool>().unwrap())))),
              v if NullLiteral::is_null(v) => lx.inject(ValueLiteral::new(Box::new(NullLiteral::new()))),
              _  => {
                if let Ok(p) = token.value.parse::<u8>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<u16>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<u32>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<u64>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<i8>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<i16>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<i32>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else
                if let Ok(p) = token.value.parse::<i64>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else 
                if let Ok(p) = token.value.parse::<f32>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else 
                if let Ok(p) = token.value.parse::<f64>() {
                  lx.inject(ValueLiteral::new(Box::new(NumericLiteral::new(p))));
                } else {
                  panic!("Parsing Exception: Unknown numeric value provided, {}", token.value);
                }
              }
            }
          }
        }

        if let Some(node) = self.nstack.last_mut() {
          if let Some(root) = Some(node.root.to_mut()) {
            match node.et {
                ExpressionType::Object => {
                  root.downcast_mut::<ObjectExpression>().unwrap().add(ls.downcast_mut::<ObjectLiteral>().unwrap().clone());
                },
                ExpressionType::Array => {
                  root.downcast_mut::<ArrayExpression>().unwrap().add(ls.downcast_mut::<ArrayLiteral>().unwrap().clone());
                },
                _ => panic!("Parser Exception: Unsupported parent node type found!"),
            }
          }

          self.lstack.free_last();
        } else {
          panic!("Parser Exception: No parent node found to store the value!");
        }
      }
      token if token.tt == TokenType::ObjectStart || token.tt == TokenType::ArrayStart => {
        self.lstack.push(None);
        self.nstack.push(match token.tt {
            TokenType::ObjectStart => Node::new(ExpressionType::Object, Box::new(ObjectExpression::new())),
            TokenType::ArrayStart => Node::new(ExpressionType::Array, Box::new(ArrayExpression::new())),
            _ => panic!("Parser Exception: Unknown type found!")
        });
      }
      token if token.tt == TokenType::ObjectEnd || token.tt == TokenType::ArrayEnd => {
        self.validate(&token);
        let ln = self.nstack.pop().unwrap();
        self.lstack.pop();

        let node = self.nstack.last_mut().unwrap();
        let vl: Box<dyn Literal> = match token.tt {
            TokenType::ObjectEnd => Box::new(ln.root.to_ref().downcast_ref::<ObjectExpression>().unwrap().clone()),
            TokenType::ArrayEnd => Box::new(ln.root.to_ref().downcast_ref::<ArrayExpression>().unwrap().clone()),
            _ => panic!("Parser Exception: Unknown type found!")
        };
        match node.et {
          ExpressionType::Object => {
            let le = self.lstack.last_mut().unwrap().as_mut().unwrap().to_mut().downcast_mut::<ObjectLiteral>().unwrap();
            le.value(ValueLiteral::new(vl));

            let nr = node.root.to_mut().downcast_mut::<ObjectExpression>().unwrap();
            nr.add(le.clone());
          },
          ExpressionType::Array => {
            let nr = node.root.to_mut().downcast_mut::<ArrayExpression>().unwrap();
            nr.add(ArrayLiteral::init(ValueLiteral::new(vl)));
          },
          _ => panic!("Parser Exception: Invalid node type has given! Node Type: {:?}", node.et),
        }

        self.lstack.free_last();
      }
      _ => {} 
    }
    
    self.advance(); 
    self.parse();
  }
}

impl <'a> JsonParser<'a> {
  fn print(&self) {
    println!("-- Json Parser Tree --");
    fn fun(ol: Box<&dyn Expression>, space_count: usize, space_size: usize) {
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<ObjectExpression>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        v.lit_objects.iter().for_each(|lo| {
          fun(Box::new(lo), space_count + space_size, space_size);
        });
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<ArrayExpression>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        v.lit_elements.iter().for_each(|le| {
          fun(Box::new(le), space_count + space_size, space_size);
        });
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<ObjectLiteral>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        fun(Box::new(v.key.as_ref().unwrap()), space_count + space_size, space_size);
        fun(Box::new(v.value.as_ref().unwrap()), space_count + space_size, space_size);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<ArrayLiteral>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        fun(Box::new(v.value.as_ref().unwrap()), space_count + space_size, space_size);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<KeyLiteral>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        fun(Box::new(&v.lit_string), space_count + space_size, space_size);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<StringLiteral>() {
        println!("{} {}: '{}'", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NullLiteral>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<BooleanLiteral>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u8>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u16>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u32>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<i8>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<i16>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<i32>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<f32>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
        println!("{} {}: {}", " ".repeat(space_count), v.ident_name(), v.value);
      } else 
      if let Some(v) = ol.as_ref().to_ref().downcast_ref::<ValueLiteral>() {
        println!("{} {}", " ".repeat(space_count), v.ident_name());
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<StringLiteral>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NullLiteral>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<BooleanLiteral>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u8>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u16>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u32>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<i8>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<i16>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<i32>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<f32>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<NumericLiteral<u64>>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<ObjectLiteral>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<ArrayLiteral>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<ObjectExpression>() {
          fun(Box::new(c), space_count + space_size, space_size);
        } else 
        if let Some(c) = v.literal.as_ref().to_ref().downcast_ref::<ArrayExpression>() {
          fun(Box::new(c), space_count + space_size, space_size);
        }
      }
    }

    if let Some(ln) = self.nstack.last() {
      let _space_size: usize = 3;

      if ln.is_object_expression() {
        let root = ln.root.as_ref().to_ref().downcast_ref::<ObjectExpression>().unwrap();
        println!("{}", root.ident_name());
        root.lit_objects.iter().for_each(|lo| {
          fun(Box::new(lo), _space_size, _space_size);
        });
      } else if ln.is_array_expression() {
        let root = ln.root.as_ref().to_ref().downcast_ref::<ArrayExpression>().unwrap();
        println!("{}", root.ident_name());
        root.lit_elements.iter().for_each(|le| {
          fun(Box::new(le), _space_size, _space_size);
        });
      } 
    }
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
    let tokens = lexer.tokenize();
    Logger::info("Tokenized List");
    for e in tokens {
      println!("Token - {} {:<15} '{}'", e.tt, format!("{:?}", e.quoted), e.value);
    }

    let mut json_parser = JsonParser::new(tokens);
    json_parser.parse();
    json_parser.print();
  }
}
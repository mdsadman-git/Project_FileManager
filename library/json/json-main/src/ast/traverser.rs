use crate::ast::{laxer::Laxer, parser::{
  ArrayExpression, ArrayLiteral, BooleanLiteral, Expression, KeyLiteral, NullLiteral, 
  NumericLiteral, ObjectExpression, ObjectLiteral, Parser, StringLiteral, ValueLiteral
}};

pub trait JsonTraverserTrait {
  fn object_expression_main(&mut self, v: &mut ObjectExpression, tracker: &mut Vec<String>, level: usize);
  fn object_expression_after(&mut self, tracker: &mut Vec<String>, level: usize);
  fn array_expression_main(&mut self, v: &mut ArrayExpression, tracker: &mut Vec<String>, level: usize);
  fn array_expression_after(&mut self, tracker: &mut Vec<String>, level: usize);
  fn object_literal_before(&mut self, v: &mut ObjectLiteral, tracker: &mut Vec<String>, level: usize);
  fn object_literal_after(&mut self, v: &mut ObjectLiteral, tracker: &mut Vec<String>, level: usize);
  fn array_literal(&mut self, v: &mut ArrayLiteral, tracker: &mut Vec<String>, level: usize);
  fn key_literal(&mut self, v: &mut KeyLiteral, tracker: &mut Vec<String>, level: usize);
  fn value_literal(&mut self, v: &mut ValueLiteral, tracker: &mut Vec<String>, level: usize);
  fn string_literal(&mut self, v: &mut StringLiteral, tracker: &mut Vec<String>, level: usize);
  fn boolean_literal(&mut self, v: &mut BooleanLiteral, tracker: &mut Vec<String>, level: usize);
  fn null_literal(&mut self, v: &mut NullLiteral, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_u8(&mut self, v: &mut NumericLiteral<u8>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_u16(&mut self, v: &mut NumericLiteral<u16>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_u32(&mut self, v: &mut NumericLiteral<u32>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_u64(&mut self, v: &mut NumericLiteral<u64>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_i8(&mut self, v: &mut NumericLiteral<i8>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_i16(&mut self, v: &mut NumericLiteral<i16>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_i32(&mut self, v: &mut NumericLiteral<i32>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_i64(&mut self, v: &mut NumericLiteral<i64>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_f32(&mut self, v: &mut NumericLiteral<f32>, tracker: &mut Vec<String>, level: usize);
  fn numeric_literal_f64(&mut self, v: &mut NumericLiteral<f64>, tracker: &mut Vec<String>, level: usize);
}

pub struct JsonTraverser<'a> {
  json: String,
  tracker: Vec<String>,
  traverser: Option<Box<&'a mut dyn JsonTraverserTrait>>,
}

impl <'a> JsonTraverser<'a> {
  pub fn new(json: impl Into<String>) -> Self {
    Self { json: json.into(), traverser: None, tracker: vec![String::new()] }
  }    
}

impl <'a> JsonTraverser<'a> {
  pub fn setup(&mut self, traverser: Box<&'a mut dyn JsonTraverserTrait>) -> &mut Self {
    self.traverser = Some(traverser);
    self
  }

  pub fn parse(&mut self) {
    let mut lexer = Laxer::new(&self.json);
    let mut parser = Parser::execute(lexer.tokenize());
    let expression = parser.node().root();
    if let Some(v) = expression.to_mut().downcast_mut::<ObjectExpression>() {
      self.traverse(Box::new(v), 0);
    } else 
    if let Some(v) = expression.to_mut().downcast_mut::<ArrayExpression>() {
      self.traverse(Box::new(v), 0);
    } else {
      panic!("Json Parser Exception! Unknown type found while traversing!")
    }
  }

  fn traverse(&mut self, ol: Box<&mut dyn Expression>, level: usize) {
    if let Some(v) = ol.to_mut().downcast_mut::<ObjectExpression>() {
      self.traverser.as_mut().unwrap().object_expression_main(v, &mut self.tracker, level);
      v.lit_objects.iter_mut().for_each(|lo| {
        self.traverse(Box::new(lo), level + 1);
      });
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<ArrayExpression>() {
      self.traverser.as_mut().unwrap().array_expression_main(v, &mut self.tracker, level);
      v.lit_elements.iter_mut().for_each(|le| {
        self.traverse(Box::new(le), level);
      });
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<ObjectLiteral>() {
      self.traverser.as_mut().unwrap().object_literal_before(v, &mut self.tracker, level);
      self.traverse(Box::new(v.key.as_mut().unwrap()), level);
      self.traverse(Box::new(v.value.as_mut().unwrap()), level);
      self.traverser.as_mut().unwrap().object_literal_after(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<ArrayLiteral>() {
      self.traverser.as_mut().unwrap().array_literal(v, &mut self.tracker, level);
      self.traverse(Box::new(v.value.as_mut().unwrap()), level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<KeyLiteral>() {
      self.traverser.as_mut().unwrap().key_literal(v, &mut self.tracker, level);
      self.traverse(Box::new(&mut v.lit_string), level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<StringLiteral>() {
      self.traverser.as_mut().unwrap().string_literal(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NullLiteral>() {
      self.traverser.as_mut().unwrap().null_literal(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<BooleanLiteral>() {
      self.traverser.as_mut().unwrap().boolean_literal(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<u8>>() {
      self.traverser.as_mut().unwrap().numeric_literal_u8(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<u16>>() {
      self.traverser.as_mut().unwrap().numeric_literal_u16(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<u32>>() {
      self.traverser.as_mut().unwrap().numeric_literal_u32(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<u64>>() {
      self.traverser.as_mut().unwrap().numeric_literal_u64(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<i8>>() {
      self.traverser.as_mut().unwrap().numeric_literal_i8(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<i16>>() {
      self.traverser.as_mut().unwrap().numeric_literal_i16(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<i32>>() {
      self.traverser.as_mut().unwrap().numeric_literal_i32(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<i64>>() {
      self.traverser.as_mut().unwrap().numeric_literal_i64(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<f32>>() {
      self.traverser.as_mut().unwrap().numeric_literal_f32(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<NumericLiteral<f64>>() {
      self.traverser.as_mut().unwrap().numeric_literal_f64(v, &mut self.tracker, level);
    } else 
    if let Some(v) = ol.to_mut().downcast_mut::<ValueLiteral>() {
      if let Some(c) = v.literal.to_mut().downcast_mut::<StringLiteral>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NullLiteral>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<BooleanLiteral>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u8>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u16>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u32>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u64>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<i8>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<i16>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<i32>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u64>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<f32>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<NumericLiteral<u64>>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<ObjectLiteral>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<ArrayLiteral>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<ObjectExpression>() {
        self.traverse(Box::new(c), level);
      } else 
      if let Some(c) = v.literal.to_mut().downcast_mut::<ArrayExpression>() {
        self.traverse(Box::new(c), level);
      }
    }

    if ol.to_mut().downcast_mut::<ObjectExpression>().is_some() {
      self.traverser.as_mut().unwrap().object_expression_after(&mut self.tracker, level);
    } else 
    if ol.to_mut().downcast_mut::<ArrayExpression>().is_some() {
      self.traverser.as_mut().unwrap().array_expression_after(&mut self.tracker, level);
    }
  }
}

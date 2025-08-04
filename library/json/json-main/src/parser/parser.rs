use alds_lib::list::fire::FireList;

use crate::ast::{lexer::Lexer, parser::{ArrayExpression, ObjectExpression, Parser}};

struct JsonParser {
  json: String,
  jc: JsonContainer,
}

impl JsonParser {
  fn new(json: String) -> Self {
    Self { json, jc: JsonContainer::new() }
  }    
}

impl JsonParser {
  fn parse(&mut self) {
    let mut lexer = Lexer::new(&self.json);
    let parser = Parser::execute(lexer.tokenize());
    let expression = parser.node().root();
    if let Some(v) = expression.to_ref().downcast_ref::<ObjectExpression>() {
      self.jc.update_jct(JsonContainerType::Object);
    } else 
    if let Some(v) = expression.to_ref().downcast_ref::<ArrayExpression>() {
      self.jc.update_jct(JsonContainerType::Array);
    } else {
      panic!("Json Parser Exception! Unknown type found while traversing!")
    }
  }
}

enum JsonContainerType {
  Object, Array, None 
}

struct JsonContainer {
  jct: JsonContainerType,
  entities: FireList<String>,
}

impl JsonContainer {
  fn new() -> Self {
    Self { jct: JsonContainerType::None, entities: FireList::new() }
  } 
}

impl JsonContainer {
  fn update_jct(&mut self, jct: JsonContainerType) {
    self.jct = jct;
  }
}

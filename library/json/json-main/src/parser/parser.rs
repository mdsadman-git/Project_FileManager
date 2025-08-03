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
  jct: JsonContainerType
}

impl JsonContainer {
  fn new() -> JsonContainer {
    Self { jct: JsonContainerType::None }
  } 
}

impl JsonContainer {
  fn update_jct(&mut self, jct: JsonContainerType) {
    self.jct = jct;
  }
}


// TODO: NEED TO MOVE THIS DS TO ALDS_LIB PROJECT

use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Indicator<T> = Option<Rc<RefCell<Box<FireNode<T>>>>>;

struct FireList<T> where T: Debug {
  size: usize,
  // head: Indicator<T>,
  // tail: Indicator<T>,
  head: Indicator<T>,
  tail: Indicator<T>,
}

struct FireNode<T> where T: Debug {
  value: T,
  // next: Indicator<T>,
  // prev: Indicator<T>,
  prev: Indicator<T>,
  next: Indicator<T>,
}

impl <T: Debug> FireNode<T> {
  fn new(value: T) -> Self {
    Self { value, next: None, prev: None }
  }

  // fn init(value: T, prev: Indicator<T>) -> Self {
  //   Self { value, next: None, prev }
  // }
}

impl <T: Debug> FireNode<T> {
  fn update_value(&mut self, value: T) {
    self.value = value;
  }

  // fn update_next(&mut self, next: Indicator<T>) {
  //   self.next = next;
  // }

  // fn update_prev(&mut self, prev: Indicator<T>) {
  //   self.prev = prev;
  // }

  fn update_next(&mut self, next: Indicator<T>) {
    self.next = next;
  }

  fn update_prev(&mut self, prev: Indicator<T>) {
    self.prev = prev;
  }
}

impl <T: Debug> FireList<T> {
  pub fn new() -> Self {
    Self { size: 0, head: None, tail: None }
  }
}

impl <T: Debug> FireList<T> {
  pub fn size(&self) -> usize {
    self.size
  }

  pub fn link(&mut self, value: T) -> &mut Self {
    let node = RefCell::new(Box::new(FireNode::new(value)));
    if self.size == 0 {
      self.head = Some(Rc::new(node));
      self.tail = self.head.clone();
    } else {
      let last = self.tail.take();
      node.borrow_mut().update_prev(last.clone());
      self.tail = Some(Rc::new(node));
      last.unwrap().borrow_mut().update_next(self.tail.clone()); 
    }

    self.size += 1;
    self
  }


  pub fn unlink(&mut self, index: usize) -> &Self {
    if index > self.size { panic!("Unlinking Exception! Index not found inside FireList [Index: {}, Size: {}]", index, self.size) }
    if self.head.is_none() || self.tail.is_none() { return self }
    match index {
      0 => {
        let curr = self.head.take().unwrap();
        let next = curr.borrow_mut().next.take();
        if let Some(next) = next {
          next.borrow_mut().update_prev(None);
          self.head = Some(next);
        } else {
          self.head = None;
          self.tail = None;
        }
      } 
      i if i == self.size - 1 => {
        let curr = self.tail.take().unwrap();
        let prev = curr.borrow_mut().prev.take();
        if let Some(prev) = prev {
          prev.borrow_mut().update_next(None);
          self.tail = Some(prev);
        } else {
          self.head = None;
          self.tail = None;
        }
      }
      _ => {
        let current = &mut self.head.as_ref();
        let current = self.finder(current, 0, index);
        if let Some(curr) = current {
          let prev = curr.borrow_mut().prev.take().unwrap();
          let next = curr.borrow_mut().next.take().unwrap();
          prev.borrow_mut().update_next(Some(next.clone()));
          next.borrow_mut().update_prev(Some(prev.clone()));
        }
      }
    }

    self
  }

  pub fn print(&self) {
    println!("Print: From Head -> Tail");
    let mut clone = self.head.clone();
    while let Some(v) = clone {
      let b = v.as_ref().borrow();
      print!("{:?} -> ", &(*b).value);
      clone = v.borrow().next.clone();
    }
    println!("()");

    println!("Print: From Tail -> Head");
    let mut clone = self.tail.clone();
    while let Some(v) = clone {
      let b = v.as_ref().borrow();
      print!("{:?} -> ", &(*b).value);
      clone = v.borrow().prev.clone();
    }
    println!("()");
  }
}

impl <T: Debug> FireList<T> {
  fn finder(&self, current: &mut Option<&Rc<RefCell<Box<FireNode<T>>>>>, counter: usize, index: usize) -> Option<Rc<RefCell<Box<FireNode<T>>>>> {
    if counter == self.size || current.is_none() { return None; }
    if counter == index { return Some(current.take().unwrap().clone()); }
    let binding = current.unwrap().as_ref().borrow();
    let next = &mut binding.next.as_ref();
    return self.finder(next, counter + 1, index);
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::parser::FireList;

  #[test]
  fn ll_link_test() {
    let mut fl = FireList::<String>::new();

    println!("Linking Test!");
    for i in 0..10 { fl.link(format!("{}", i)); }
    fl.print();

    println!("Unlinking Test!");
    for e in vec![3, 4, 5, 0, fl.size() - 1] {
      fl.unlink(e);
      fl.print();
    }
  }
}
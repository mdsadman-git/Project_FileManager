use std::{cell::{Ref, RefCell}, fmt::Debug, ops::DerefMut, rc::Rc};

use logger_main::Logger;

type Indicator<T> = Option<Rc<RefCell<FireNode<T>>>>;

pub struct FireList<T> where T: Debug {
  size: usize,
  head: Indicator<T>,
  tail: Indicator<T>,
}

pub struct FireNode<T> where T: Debug {
  value: T,
  prev: Indicator<T>,
  next: Indicator<T>,
}

impl <T: Debug> FireNode<T> {
  fn new(value: T) -> Self {
    Self { value, next: None, prev: None }
  }
}

impl <T: Debug> FireNode<T> {
  pub fn get(&self) -> &T {
    &self.value
  }

  pub fn get_mut(&mut self) -> &mut T {
    &mut self.value
  }

  pub fn next(&self) -> &Indicator<T> {
    &self.next
  }

  pub fn prev(&self) -> &Indicator<T> {
    &self.prev
  }
}

impl <T: Debug> FireNode<T> {
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

  // TODO: RETHINK ABOUT THE GET DATA RETURN TYPE
  pub fn get<'a>(&mut self, index: usize) -> Indicator<T> {
    let current = &mut self.head.as_ref();
    let current = self.item_finder(current, 0, index);
    if current.is_none() { return None; }
    current
  }

  // TODO: CREATE THE ITERATOR AND ENUMERATOR FOR FIRE LIST

  pub fn link(&mut self, value: T) -> &mut Self {
    let node = RefCell::new(FireNode::new(value));
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
        let current = self.item_finder(current, 0, index);
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
}

impl <T: Debug> FireList<T> {
  pub fn print(&self) {
    Logger::console("Print: From Head -> Tail");
    let mut clone = self.head.clone();
    while let Some(v) = clone {
      let b = v.as_ref().borrow();
      print!("{:?} -> ", &(*b).value);
      clone = v.borrow().next.clone();
    }
    Logger::console("()");

    Logger::console("Print: From Tail -> Head");
    let mut clone = self.tail.clone();
    while let Some(v) = clone {
      let b = v.as_ref().borrow();
      print!("{:?} -> ", &(*b).value);
      clone = v.borrow().prev.clone();
    }
    Logger::console("()");
  }
}

impl <T: Debug> FireList<T> {
  fn item_finder(&self, current: &mut Option<&Rc<RefCell<FireNode<T>>>>, counter: usize, index: usize) -> Option<Rc<RefCell<FireNode<T>>>> {
    if counter == self.size || current.is_none() { return None; }
    if counter == index { return Some(current.take().unwrap().clone()); }
    let binding = current.unwrap().as_ref().borrow();
    let next = &mut binding.next.as_ref();
    return self.item_finder(next, counter + 1, index);
  }
}

#[cfg(test)]
mod tests {
  use logger_main::Logger;

  use crate::list::fire::FireList;

  #[test]
  fn link_unlink_test() {
    let mut fl = FireList::<String>::new();

    Logger::console("Linking Test!");
    for i in 0..10 { fl.link(format!("{}", i)); }
    fl.print();

    Logger::console("Unlinking Test!");
    for e in vec![3, 4, 5, 0, fl.size() - 1] {
      fl.unlink(e);
      fl.print();
    }
  }

  #[test]
  fn looping_test() {
    let mut fl = FireList::<String>::new();

    Logger::console("Linking Test!");
    for i in 0..10 { fl.link(format!("{}", i)); }
    fl.print();

    let f = fl.get(2);
    println!("{:?}", f.unwrap().borrow().get());
  }
}

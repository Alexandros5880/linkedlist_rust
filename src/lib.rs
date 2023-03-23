#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Weak;
use std::{fmt::Display, rc::Rc, cell::RefCell};
use std::fmt::Debug;

// Node
#[derive(Debug)]
struct Node<T>
  where T: Display + Debug + Copy
{
  pub prev: Option<Weak<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
  pub data: T
}

impl<T> Node<T>
  where T: Display + Debug + Copy
{
  pub fn new(data: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Node {
      prev: None,
      next: None,
      data
    }))
  }
}

// Linked List
#[derive(Debug)]
pub struct LinkedList<T>
  where T: Display + Debug + Copy
{
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
  pub len: usize,
  index: usize
}

impl<T> LinkedList<T>
  where T: Display + Debug + Copy
{

  pub fn new() -> LinkedList<T> {
    LinkedList {
      head: None,
      tail: None,
      len: 0,
      index: 0
    }
  }

  pub fn push_front(&mut self, elem: T) {
    self.len += 1;
    let new_head = Node::new(elem);
    match self.head.take() {
        Some(old_head) => {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head.clone()));
            new_head.borrow_mut().next = Some(old_head.to_owned());
            self.head = Some(new_head);
        }
        None => {
            self.tail = Some(new_head.clone());
            self.head = Some(new_head);
        }
    }
  }

  pub fn push_back(&mut self, elem: T) {
    self.len += 1;
    let new_tail = Node::new(elem);
    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new_tail.to_owned());
        new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail.clone()));
        self.tail = Some(new_tail);
      }
      None => {
        self.head = Some(new_tail.clone());
        self.tail = Some(new_tail);
      }
    }
  }

  pub fn delete_by_index(&mut self, index: usize) {
    let node: Option<Rc<RefCell<Node<T>>>>;
    if index == 0 {
      node = self.head.clone();
    } else if index == self.len - 1 {
      node = self.tail.clone();
    } else if index <= self.len / 2 {
      let index = index + 1;
      node = self.get_by_index_from_head(self.head.clone(), index, 0);
    } else {
      let index = index + 1;
      node = self.get_by_index_from_tail(self.tail.clone(), index, self.len + 1);
    }
    
    let unwraped = node.unwrap();

    let prev = unwraped.borrow().prev.clone();
    let prev = if prev.is_none() {
      unwraped.clone()
    } else {
      prev.unwrap().upgrade().unwrap()
    };
    let mut prev_mut = prev.borrow_mut();

    let next = unwraped.borrow().next.clone();
    let next = if next.is_none() {
      unwraped.clone()
    } else {
      next.unwrap()
    };
    let mut next_mut = next.borrow_mut();
    
    prev_mut.next = Some(next.to_owned());
    next_mut.prev = Some(Rc::downgrade(&prev.to_owned()));
    
    self.len -= 1;
  }

  pub fn get_by_index(&self, index: usize) -> Option<T> {
    let node: Option<Rc<RefCell<Node<T>>>>;
    if index == 0 {
      node = self.head.clone();
    } else if index == self.len-1 {
      node = self.tail.clone();
    } else if index <= self.len / 2 {
      let index = index + 1;
      node = self.get_by_index_from_head(self.head.clone(), index, 0);
    } else {
      let index = index + 1;
      node = self.get_by_index_from_tail(self.tail.clone(), index, self.len + 1);
    }

    let result = match node {
      Some(data) => Some(data.borrow().data.clone()),
      None => None
    };

    result.to_owned()
  }

  fn get_by_index_local(&self, index: usize) -> Option<T> {
    let node: Option<Rc<RefCell<Node<T>>>>;

    if index == 0 {
      node = self.head.clone();
    } else if index == self.len {
      node = self.tail.clone();
    } else if index <= self.len / 2 {
      node = self.get_by_index_from_head(self.head.clone(), index, 0);
    } else {
      node = self.get_by_index_from_tail(self.tail.clone(), index, self.len + 1);
    }

    let result = match node {
      Some(data) => Some(data.borrow().data.clone()),
      None => None
    };

    result
  }

  fn get_by_index_from_head(&self, node: Option<Rc<RefCell<Node<T>>>>, index: usize, global_index: usize) -> Option<Rc<RefCell<Node<T>>>> {
    let n = node.clone();
    let global_index = global_index + 1;
    if node.is_some() {
      if index == global_index {
        n
      } else {
        let next = n.unwrap().borrow().next.clone();
        self.get_by_index_from_head(next, index, global_index)
      }
    } else {
      None
    }
  }

  fn get_by_index_from_tail(&self, node: Option<Rc<RefCell<Node<T>>>>, index: usize, global_index: usize) -> Option<Rc<RefCell<Node<T>>>> {
    let n = node.clone();
    let global_index = global_index - 1;
    if node.is_some() {
      if index == global_index {
        n
      } else {
        let prev = n.unwrap().borrow().prev.clone();
        if prev.is_some() {
          let prev = prev.unwrap().upgrade();
          self.get_by_index_from_tail(prev, index, global_index)
        } else {
          None
        }
      }
    } else {
      None
    }
  }

}

// Iterator
impl<T> Iterator for LinkedList<T>
  where T: Display + Debug + Copy
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.len {
      return None
    }
    self.index += 1;
    self.get_by_index_local(self.index)
  }

}

#[cfg(test)]
mod tests {
  use crate::LinkedList;

  #[test]
  fn push_back_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    assert_eq!(list.len, 8);
  }

  #[test]
  fn push_front_works() {
    let mut list = LinkedList::new();
    list.push_front(0);
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);
    list.push_front(6);
    list.push_front(7);

    assert_eq!(list.len, 8);
  }

  #[test]
  fn delete_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    list.delete_by_index(3);
    assert_eq!(list.len, 7);

    let data = list.get_by_index(3);
    assert_eq!(data.is_some(), true);
    let unwraped = data.unwrap();
    assert_eq!(unwraped, 4);
  }

  #[test]
  fn get_by_index_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    let data = list.get_by_index(4);
    assert_eq!(data.is_some(), true);
    let unwraped = data.unwrap();
    assert_eq!(unwraped, 4);

    let data = list.get_by_index(9);
    assert_eq!(data.is_none(), true);
  }

  #[test]
  fn iterator_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    let mut index = 0;
    for item in list {
      assert_eq!(item, index);
      index += 1;
    }
  }

}
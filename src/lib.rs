#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Weak;
use std::{fmt::Display, rc::Rc, cell::RefCell};
use std::fmt::Debug;
use std::cmp::PartialEq;

// Node
#[derive(Debug)]
struct Node<T>
  where T: Display + Debug + Copy + PartialEq
{
  pub prev: Option<Weak<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
  pub data: T
}

impl<T> Node<T>
  where T: Display + Debug + Copy + PartialEq
{
  pub fn new(data: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Node {
      prev: None,
      next: None,
      data
    }))
  }
}

/// # My LinkedList
// Linked List
#[derive(Debug)]
pub struct LinkedList<T>
  where T: Display + Debug + Copy + PartialEq
{
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
  pub len: usize,
  index: usize
}

impl<T> LinkedList<T>
  where T: Display + Debug + Copy + PartialEq
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

  pub fn get_by_value(&self, value: &T) -> Option<T> {
    let node = self.get_by_value_from_head(self.head.clone(), value);
    let result = match node {
      Some(data) => Some(data.borrow().data.clone()),
      None => None
    };
    result.to_owned()
  }

  pub fn delete_by_value(&mut self, value: &T) {
    let node = self.get_by_value_from_head(self.head.clone(), value);
    
    if node.is_some() {
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
    
  }

  /// Executes a closure to every node
  pub fn excecute_to_all(&self, f: fn(&mut T)) {
    self.recursive(self.head.to_owned(), f);
  }

  /// Private functions 
  fn recursive(&self, node: Option<Rc<RefCell<Node<T>>>>, f: fn(&mut T)) -> bool {
    let result = match node {
      Some(node) => {
        let mut bnode = node.borrow_mut();
        let data = &mut bnode.data;
        f(data);
        self.recursive(bnode.next.to_owned(), f)
      },
      None => false
    };
    result
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

  fn get_by_value_from_head(&self, node: Option<Rc<RefCell<Node<T>>>>, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
    let n = node.clone();
    if node.is_some() {
      let val = node.unwrap().borrow().data;
      if val == *value {
        n
      } else {
        let next = n.unwrap().borrow().next.clone();
        self.get_by_value_from_head(next, value)
      }
    } else {
      None
    }
  }

  fn get_by_value_from_tail(&self, node: Option<Rc<RefCell<Node<T>>>>, value: &T) -> Option<Rc<RefCell<Node<T>>>> {
    let n = node.clone();
    if node.is_some() {
      let val = node.unwrap().borrow().data;
      if val == *value {
        n
      } else {
        let prev = n.unwrap().borrow().prev.clone();
        if prev.is_some() {
          let prev = prev.unwrap().upgrade();
          self.get_by_value_from_tail(prev, value)
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
  where T: Display + Debug + Copy + PartialEq
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
  fn delete_by_index_works() {
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

  #[test]
  fn recursive_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    list.excecute_to_all(|data| {
      *data = *data * 2;
    });

    let item0 = list.get_by_index(0).unwrap();
    let item1 = list.get_by_index(1).unwrap();
    let item2 = list.get_by_index(2).unwrap();
    let item3 = list.get_by_index(3).unwrap();
    let item4 = list.get_by_index(4).unwrap();
    let item5 = list.get_by_index(5).unwrap();
    let item6 = list.get_by_index(6).unwrap();
    let item7 = list.get_by_index(7).unwrap();

    assert_eq!(item0, 0);
    assert_eq!(item1, 2);
    assert_eq!(item2, 4);
    assert_eq!(item3, 6);
    assert_eq!(item4, 8);
    assert_eq!(item5, 10);
    assert_eq!(item6, 12);
    assert_eq!(item7, 14);

  }

  #[test]
  fn get_by_value_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    list.excecute_to_all(|data| {
      *data = *data * 2;
    });

    let val = list.get_by_value(&10);

    assert!(val.is_some());

    assert_eq!(val.unwrap(), 10);
  }

  #[test]
  fn delete_by_value_works() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);
    list.push_back(7);

    list.excecute_to_all(|data| {
      *data = *data * 2;
    });

    list.delete_by_value(&10);

    assert_eq!(list.len, 7);
  }

}
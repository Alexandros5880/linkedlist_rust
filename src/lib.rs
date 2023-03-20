use std::rc::Weak;
use std::{fmt::Display, rc::Rc, cell::RefCell};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T>
  where T: Display + Debug 
{
  pub previus: Option<Weak<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
  pub data: T
}

#[derive(Debug)]
pub struct LinkedList<T: Display + Debug> {
  pub root: Rc<RefCell<Node<T>>>
}

impl<T: Display + Debug> LinkedList<T> {

  pub fn new(data: T) -> LinkedList<T> {
    let node = Node {
      previus: None,
      next: None,
      data
    };
    LinkedList {
      root: Rc::new(RefCell::new(node))
    }
  }

  pub fn Push(&mut self, data: T) {

    let next = Rc::from(RefCell::new(Node {
      previus: None,
      next: None,
      data
    }));

    let tail = self.Tail(Rc::clone(&self.root));
    
    next.borrow_mut().previus = Some(Rc::downgrade(&tail));
    tail.borrow_mut().next = Some(next);
  }

  fn Tail(&self, node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    match &node.borrow().next {
        Some(node) => self.Tail(Rc::clone(&node)),
        None => Rc::clone(&node)
    }
  }

  fn Head(&self, node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    match &node.borrow().previus {
        Some(node_weak) => {
          match node_weak.upgrade() {
            Some(node_rc) => self.Head(Rc::clone(&node_rc)),
            None => Rc::clone(&node)
          }
        },
        None => Rc::clone(&node)
    }
  }

}

// impl<T: Display + Debug> Iterator for LinkedList<T> {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

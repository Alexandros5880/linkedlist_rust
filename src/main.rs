use linkedlist::LinkedList;

pub fn main() {
  let mut list = LinkedList::new(1);
  list.Push(2);
  list.Push(3);
  list.Push(4);
  list.Push(5);
  list.Push(6);
  list.Push(7);

  println!("List: {:#?}", &list);
}
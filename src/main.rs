use linkedlist::LinkedList;

pub fn main() {
  let mut list = LinkedList::new();
  list.push_back(0);
  list.push_back(1);
  list.push_back(2);
  list.push_back(3);
  list.push_back(4);
  list.push_back(5);
  list.push_back(6);
  list.push_back(7);

  // println!("{:#?}", list);

  for item in list {
    println!("Item: {:#?}", item);
  }

}
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

  println!("{:#?}", list);

  let index = 7;
  let data = list.get_by_index(index);
  match data {
    Some(data) => println!("list[{}]: {:?}", &index, data),
    None => println!("Out of Index!")
  }
  


  // let data = list.get_by_index(4);
  // assert_eq!(data.is_some(), true);
  // let unwraped = data.unwrap();
  // println!("\n\n{}\n\n", unwraped);

  // for item in list {
  //   println!("Item: {:#?}", item);
  // }
}
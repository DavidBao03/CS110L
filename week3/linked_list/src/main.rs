use linked_list::LinkedList;

use crate::linked_list::ComputeNorm;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}

    let mut list_string: LinkedList<String> = LinkedList::new();
    assert!(list_string.is_empty());
    assert_eq!(list_string.get_size(), 0);
    for i in 1..12 {
        list_string.push_front(format!("Here's string {}\n", i));
    }

    println!("{}", list_string);
    println!("list size: {}", list_string.get_size());
    println!("top element: {}", list_string.pop_front().unwrap());
    println!("{}", list_string);
    println!("size: {}", list_string.get_size());

    let mut list_clone = list_string.clone();
    println!("cloned list: {}", list_clone);

    println!(
        "cloned list is equal to origin list: {}",
        list_clone == list_string
    );
    list_clone.pop_front();
    println!("After pop: {}", list_clone == list_string);

    println!("======Test for Iter======");
    for val in &list_clone {
        println!("{}", val);
    }

    println!("{}", list_clone);

    // for val in list_clone {
    //     println!("{}", val);
    // }

    // println!("{}", list_clone);

    let mut list_f64 = LinkedList::new();
    list_f64.push_front(3.0);
    list_f64.push_front(4.0);

    println!("Nore of the list: {}", list_f64.compute_nore());
    println!("{}", list_f64);
}

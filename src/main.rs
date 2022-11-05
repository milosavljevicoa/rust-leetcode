use std::collections::LinkedList;

use crate::medium::add_two_numbers;

pub mod medium;

fn main() {
    println!("{:?}", add_two_numbers::func(LinkedList::from([2,4,3]), LinkedList::from([5,6,4])));
    println!("{:?}", add_two_numbers::improved_func(LinkedList::from([2,4,3]), LinkedList::from([5,6,4])));
}

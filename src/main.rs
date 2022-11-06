use std::collections::LinkedList;
use crate::medium::{add_two_numbers, longest_substring_without_repeating_characters};

pub mod medium;

fn main() {
    // println!("{:?}", add_two_numbers::func(LinkedList::from([2,4,3]), LinkedList::from([5,6,4])));
    // println!("{:?}", add_two_numbers::improved_func(LinkedList::from([2,4,3]), LinkedList::from([5,6,4])));
    
    println!("{:?}", longest_substring_without_repeating_characters::func(String::from("abcabcbb")));
    println!("{:?}", longest_substring_without_repeating_characters::func(String::from("bbbbb")));
    println!("{:?}", longest_substring_without_repeating_characters::func(String::from("pwwkew")));
}

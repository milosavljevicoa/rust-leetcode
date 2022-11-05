use std::collections::LinkedList;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
//

pub fn improved_func(l1: LinkedList<u32>, l2: LinkedList<u32>) -> LinkedList<u32> {
    let (mut iter1, mut iter2) = (l1.iter(), l2.iter());
    let mut carry = 0;
    let mut sum: LinkedList<u32> = LinkedList::new();

    loop {
        let num1 = iter1.next().unwrap_or(&0);
        let num2 = iter2.next().unwrap_or(&0);
        let result = carry + num1 + num2;
        if result == 0 {
            break;
        }
        carry = result / 10;
        sum.push_back(result % 10);
    };

    sum
}


pub fn func(l1: LinkedList<u32>, l2: LinkedList<u32>) -> LinkedList<u32> {
    let num1 = get_num(&l1);
    let num2 = get_num(&l2);

    let mut num_sum = num1 + num2;

    let mut num_list: LinkedList<u32> = LinkedList::new();

    while num_sum != 0 {
        let test = num_sum % 10;
        num_list.push_back(test);
        num_sum = num_sum / 10;
    }

    num_list
}

fn get_num(list: &LinkedList<u32>) -> u32 {
    list
        .iter()
        .enumerate()
        .fold(0, |acc, (i, num)| {
            let next = num * (10u32.pow(i as u32));
            acc + next
        })
}

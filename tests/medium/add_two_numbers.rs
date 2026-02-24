// 2. Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/
use crate::shared::list_node_box::{create_node_list, ListNode};
struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut t1 = l1;
        let mut t2 = l2;

        let mut extra = 0;
        let mut values: Vec<i32> = Vec::new();
        while t1.is_some() || t2.is_some() {
            let (v1, n1) = if let Some(b1) = t1.as_mut() {
                (
                    b1.val,
                    if let Some(next) = b1.next.as_ref() {
                        Some(Box::clone(&next))
                    } else {
                        None
                    },
                )
            } else {
                (0, None)
            };

            let (v2, n2) = if let Some(b2) = t2.as_mut() {
                (
                    b2.val,
                    if let Some(next) = b2.next.as_ref() {
                        Some(Box::clone(&next))
                    } else {
                        None
                    },
                )
            } else {
                (0, None)
            };

            let additional = (v1 + v2 + extra) / 10;
            values.push(v1 + v2 + extra - additional * 10);
            extra = additional;

            t1 = n1;
            t2 = n2;
        }

        if extra > 0 {
            values.push(extra);
        }

        create_node_list(&values)
    }
}

#[test]
fn test_add_tw_numbers() {
    let l1 = create_node_list(&[2, 4, 3]);
    let l2 = create_node_list(&[5, 6, 4]);
    println!("l1: {:?}", &l1);
    println!("l2: {:?}", &l2);
    let result = Solution::add_two_numbers(l1, l2);
    let expected = create_node_list(&[7, 0, 8]);
    print!("expected: {:?}", &expected);
    assert_eq!(expected, result);

    let l1 = create_node_list(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = create_node_list(&[9, 9, 9, 9]);
    println!("l1: {:?}", &l1);
    println!("l2: {:?}", &l2);
    let result = Solution::add_two_numbers(l1, l2);
    let expected = create_node_list(&[8, 9, 9, 9, 0, 0, 0, 1]);
    print!("expected: {:?}", &expected);
    assert_eq!(expected, result);
}

#[test]
fn test_create_node_list() {
    let result = create_node_list(&[2, 3, 4]);
    let expected = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    assert_eq!(expected, result);
}

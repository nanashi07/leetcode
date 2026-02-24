// 21. Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut l1 = &mut list1;
        let mut l2 = &mut list2;

        let mut r: Option<Box<ListNode>> = None;
        let mut rx = &mut r;

        while l1.is_some() || l2.is_some() {
            let mut n: Option<Box<ListNode>> = None;

            if l2.is_some()
                && ((l1.is_some() && l1.as_mut().unwrap().val > l2.as_mut().unwrap().val)
                    || l1.is_none())
            {
                let v = l2.as_mut().unwrap().val;
                l2 = &mut (l2.as_mut().unwrap().next);

                n = Some(Box::new(ListNode::new(v)));
            } else if l1.is_some()
                && ((l2.is_some() && l1.as_mut().unwrap().val <= l2.as_mut().unwrap().val)
                    || l2.is_none())
            {
                let v = l1.as_mut().unwrap().val;
                l1 = &mut (l1.as_mut().unwrap().next);

                n = Some(Box::new(ListNode::new(v)));
            }

            if rx.is_none() {
                *rx = n;
            } else {
                rx.as_mut().unwrap().next = n;
                rx = &mut (rx.as_mut().unwrap().next);
            }
        }

        r
    }
}

#[test]
fn test_merge_two_lists() {
    let list1 = create_node_list(&[1, 2, 4]);
    let list2 = create_node_list(&[1, 3, 4]);
    let expected = create_node_list(&[1, 1, 2, 3, 4, 4]);
    let result = Solution::merge_two_lists(list1, list2);
    assert_eq!(expected, result);

    let list1 = create_node_list(&[]);
    let list2 = create_node_list(&[]);
    let expected = create_node_list(&[]);
    let result = Solution::merge_two_lists(list1, list2);
    assert_eq!(expected, result);

    let list1 = create_node_list(&[]);
    let list2 = create_node_list(&[0]);
    let expected = create_node_list(&[0]);
    let result = Solution::merge_two_lists(list1, list2);
    assert_eq!(expected, result);

    let list1 = create_node_list(&[1]);
    let list2 = create_node_list(&[]);
    let expected = create_node_list(&[1]);
    let result = Solution::merge_two_lists(list1, list2);
    assert_eq!(expected, result);
}

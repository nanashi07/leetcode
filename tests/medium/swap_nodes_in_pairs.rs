// # 24. Swap Nodes in Pairs
// https://leetcode.com/problems/swap-nodes-in-pairs/
use crate::shared::{create_node_list, ListNode};
struct Solution;

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
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        if head.is_none() || head.as_mut().unwrap().next.is_none() {
            return head;
        }

        Solution::shift(
            &mut head.as_mut().unwrap().next.as_mut().unwrap().next.take(),
            &mut head.as_mut().unwrap().next.take(),
            &mut head.take(),
        )
    }

    fn shift(
        c: &mut Option<Box<ListNode>>,
        b: &mut Option<Box<ListNode>>,
        a: &mut Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if c.is_none() {
            if b.is_none() {
                a.take()
            } else {
                b.as_mut().unwrap().next = a.take();
                b.take()
            }
        } else {
            a.as_mut().unwrap().next = Solution::shift(
                &mut c
                    .as_mut()
                    .unwrap()
                    .next
                    .as_mut()
                    .map_or(None, |x| x.next.take()),
                &mut c.as_mut().unwrap().next.take(),
                &mut c.take(),
            );

            b.as_mut().unwrap().next = a.take();

            b.take()
        }
    }
}

#[test]
fn test_swap_pairs() {
    let head = create_node_list(&[1, 2, 3, 4]);
    let expected = create_node_list(&[2, 1, 4, 3]);
    let result = Solution::swap_pairs(head);
    assert_eq!(expected, result);

    let head = create_node_list(&[]);
    let expected = create_node_list(&[]);
    let result = Solution::swap_pairs(head);
    assert_eq!(expected, result);

    let head = create_node_list(&[1]);
    let expected = create_node_list(&[1]);
    let result = Solution::swap_pairs(head);
    assert_eq!(expected, result);

    let head = create_node_list(&[1, 2, 3]);
    let expected = create_node_list(&[2, 1, 3]);
    let result = Solution::swap_pairs(head);
    assert_eq!(expected, result);
}

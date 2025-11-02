// 25. Reverse Nodes in k-Group
// https://leetcode.com/problems/reverse-nodes-in-k-group/
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let mut it = &mut head;
        let mut size = 1;

        while size < k && it.is_some() {
            it = &mut it.as_mut().unwrap().next;
            size = size + 1;
        }

        // size is less than k, return head directly
        if it.is_none() {
            return head;
        }

        let remain = it.as_mut().unwrap().next.take();

        // reverse left, size = k
        let mut left = Solution::reverse(&mut None, &mut head);

        if remain.is_none() {
            return left;
        }

        // do next reverse k
        let mut right = Solution::reverse_k_group(remain, k);

        // concat left and right
        let mut it = &mut left;
        while it.is_some() {
            if it.as_mut().unwrap().next.is_none() {
                it.as_mut().unwrap().next = right.take();
                return left;
            } else {
                it = &mut it.as_mut().unwrap().next;
            }
        }

        left
    }

    fn reverse(
        a: &mut Option<Box<ListNode>>,
        b: &mut Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if b.is_none() {
            a.take()
        } else {
            let mut next = b.as_mut().unwrap().next.take();
            b.as_mut().unwrap().next = a.take();
            Solution::reverse(b, &mut next)
        }
    }
}

#[test]
fn test_reverse_k_group() {
    let head = create_node_list(&[1, 2, 3, 4, 5]);
    let k = 2;
    let expected = create_node_list(&[2, 1, 4, 3, 5]);
    let result = Solution::reverse_k_group(head, k);
    assert_eq!(expected, result);

    let head = create_node_list(&[1, 2, 3, 4, 5]);
    let k = 3;
    let expected = create_node_list(&[3, 2, 1, 4, 5]);
    let result = Solution::reverse_k_group(head, k);
    assert_eq!(expected, result);
}

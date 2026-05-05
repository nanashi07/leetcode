// 61. Rotate List
// https://leetcode.com/problems/rotate-list/

use crate::shared::list_node_box::ListNode;

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotate_list::Solution;
    use crate::shared::list_node_box::create_node_list;

    #[test]
    fn test_rotate_right_1() {
        let head = create_node_list(&[1, 2, 3, 4, 5]);
        let k = 2;
        let output = create_node_list(&[4, 5, 1, 2, 3]);
        assert_eq!(output, Solution::rotate_right(head, k));
    }

    #[test]
    fn test_rotate_right_2() {
        let head = create_node_list(&[0, 1, 2]);
        let k = 4;
        let output = create_node_list(&[2, 0, 1]);
        assert_eq!(output, Solution::rotate_right(head, k));
    }
}
